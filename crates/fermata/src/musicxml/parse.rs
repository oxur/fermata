//! MusicXML parsing module.
//!
//! This module provides the main parsing functionality for MusicXML documents.
//! It converts XML into the IR types defined in the `ir` module.
//!
//! # Example
//!
//! ```ignore
//! use fermata::musicxml::parse_score;
//!
//! let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <score-partwise version="4.0">
//!   <part-list>
//!     <score-part id="P1"><part-name>Piano</part-name></score-part>
//!   </part-list>
//!   <part id="P1"><measure number="1"/></part>
//! </score-partwise>"#;
//!
//! let score = parse_score(xml)?;
//! ```

use quick_xml::events::Event;

use super::ParseError;
use super::reader::{XmlReader, element_name};
use crate::ir::common::Editorial;
use crate::ir::measure::Measure;
use crate::ir::part::{PartList, PartListElement, PartName, ScorePart};
use crate::ir::score::ScorePartwise;
use crate::ir::{Part, PrintStyle};

/// Parse a MusicXML document from a string.
///
/// This is the main entry point for parsing MusicXML. It handles the XML
/// declaration, DOCTYPE, and root element, then delegates to the appropriate
/// parsing function based on the document type.
///
/// Currently only `score-partwise` documents are supported.
///
/// # Arguments
///
/// * `xml` - The MusicXML document as a string
///
/// # Returns
///
/// A `Result` containing the parsed `ScorePartwise` or a `ParseError`
///
/// # Errors
///
/// Returns an error if:
/// - The XML is malformed
/// - Required elements or attributes are missing
/// - The document uses `score-timewise` (not yet supported)
/// - References are undefined (e.g., part ID not in part-list)
pub fn parse_score(xml: &str) -> Result<ScorePartwise, ParseError> {
    let mut reader = XmlReader::new(xml);

    // Skip XML declaration and DOCTYPE
    loop {
        match reader.next_event()? {
            Event::Decl(_) | Event::DocType(_) | Event::Comment(_) | Event::PI(_) => continue,
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "score-partwise" => {
                        let version = reader
                            .get_optional_attr(e.attributes(), "version")?
                            .or_else(|| Some("4.0".to_string()));
                        return parse_score_partwise(&mut reader, version);
                    }
                    "score-timewise" => {
                        return Err(ParseError::other(
                            "score-timewise documents are not yet supported",
                            Some(reader.position()),
                        ));
                    }
                    _ => {
                        return Err(ParseError::unexpected_element(
                            &name,
                            "document",
                            reader.position(),
                        ));
                    }
                }
            }
            Event::Eof => {
                return Err(ParseError::other(
                    "unexpected end of document before score element",
                    Some(reader.position()),
                ));
            }
            _ => continue,
        }
    }
}

/// Parse a score-partwise element.
///
/// A score-partwise contains:
/// - Optional work, movement-number, movement-title
/// - Optional identification
/// - Optional defaults
/// - Zero or more credit elements
/// - Required part-list
/// - One or more part elements
fn parse_score_partwise(
    reader: &mut XmlReader<'_>,
    version: Option<String>,
) -> Result<ScorePartwise, ParseError> {
    let mut score = ScorePartwise {
        version,
        work: None,
        movement_number: None,
        movement_title: None,
        identification: None,
        defaults: None,
        credits: vec![],
        part_list: PartList { content: vec![] },
        parts: vec![],
    };

    let mut found_part_list = false;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "work" => {
                        // TODO: Parse work element
                        reader.skip_element("work")?;
                    }
                    "movement-number" => {
                        score.movement_number = Some(reader.read_text("movement-number")?);
                    }
                    "movement-title" => {
                        score.movement_title = Some(reader.read_text("movement-title")?);
                    }
                    "identification" => {
                        // TODO: Parse identification element
                        reader.skip_element("identification")?;
                    }
                    "defaults" => {
                        // TODO: Parse defaults element
                        reader.skip_element("defaults")?;
                    }
                    "credit" => {
                        // TODO: Parse credit element
                        reader.skip_element("credit")?;
                    }
                    "part-list" => {
                        score.part_list = parse_part_list(reader)?;
                        found_part_list = true;
                    }
                    "part" => {
                        if !found_part_list {
                            return Err(ParseError::missing_element(
                                "part-list",
                                "score-partwise",
                                reader.position(),
                            ));
                        }
                        let part = parse_part(reader, &e, &score.part_list)?;
                        score.parts.push(part);
                    }
                    _ => {
                        // Skip unknown elements for forward compatibility
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                // Handle empty elements (self-closing tags)
                match name.as_str() {
                    "defaults" | "identification" | "work" | "credit" => {
                        // Empty versions of these elements - just skip
                    }
                    _ => {
                        // Unknown empty element - skip for forward compatibility
                    }
                }
            }
            Event::End(_) => {
                // End of score-partwise
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in score-partwise",
                    reader.position(),
                ));
            }
            _ => {
                // Text, comments, etc. - skip
            }
        }
    }

    if !found_part_list {
        return Err(ParseError::missing_element(
            "part-list",
            "score-partwise",
            reader.position(),
        ));
    }

    Ok(score)
}

/// Parse a part-list element.
///
/// A part-list contains one or more of:
/// - part-group (start/stop)
/// - score-part (with id, part-name, and other optional elements)
fn parse_part_list(reader: &mut XmlReader<'_>) -> Result<PartList, ParseError> {
    let mut content = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "part-group" => {
                        let part_group = parse_part_group(reader, &e)?;
                        content.push(PartListElement::PartGroup(part_group));
                    }
                    "score-part" => {
                        let score_part = parse_score_part(reader, &e)?;
                        content.push(PartListElement::ScorePart(score_part));
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "part-group" => {
                        let part_group = parse_part_group_empty(&e, reader)?;
                        content.push(PartListElement::PartGroup(part_group));
                    }
                    _ => {
                        // Unknown empty element
                    }
                }
            }
            Event::End(_) => {
                // End of part-list
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in part-list",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(PartList { content })
}

/// Parse a score-part element.
fn parse_score_part(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<ScorePart, ParseError> {
    let id = reader.get_attr(start.attributes(), "id", "score-part")?;

    let mut score_part = ScorePart {
        id,
        identification: None,
        part_name: PartName {
            value: String::new(),
            print_style: PrintStyle::default(),
            print_object: None,
            justify: None,
        },
        part_name_display: None,
        part_abbreviation: None,
        part_abbreviation_display: None,
        group: vec![],
        score_instruments: vec![],
        midi_devices: vec![],
        midi_instruments: vec![],
    };

    let mut found_part_name = false;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "identification" => {
                        // TODO: Parse identification
                        reader.skip_element("identification")?;
                    }
                    "part-name" => {
                        score_part.part_name.value = reader.read_text("part-name")?;
                        found_part_name = true;
                    }
                    "part-name-display" => {
                        // TODO: Parse part-name-display
                        reader.skip_element("part-name-display")?;
                    }
                    "part-abbreviation" => {
                        // TODO: Parse part-abbreviation
                        reader.skip_element("part-abbreviation")?;
                    }
                    "part-abbreviation-display" => {
                        // TODO: Parse part-abbreviation-display
                        reader.skip_element("part-abbreviation-display")?;
                    }
                    "group" => {
                        let group_name = reader.read_text("group")?;
                        score_part.group.push(group_name);
                    }
                    "score-instrument" => {
                        // TODO: Parse score-instrument
                        reader.skip_element("score-instrument")?;
                    }
                    "midi-device" => {
                        // TODO: Parse midi-device
                        reader.skip_element("midi-device")?;
                    }
                    "midi-instrument" => {
                        // TODO: Parse midi-instrument
                        reader.skip_element("midi-instrument")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "part-name" => {
                        // Empty part-name
                        score_part.part_name.value = String::new();
                        found_part_name = true;
                    }
                    _ => {
                        // Unknown empty element
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in score-part",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    if !found_part_name {
        return Err(ParseError::missing_element(
            "part-name",
            "score-part",
            reader.position(),
        ));
    }

    Ok(score_part)
}

/// Parse a part-group element.
fn parse_part_group(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::part::PartGroup, ParseError> {
    use crate::ir::attributes::GroupSymbolValue;
    use crate::ir::part::{GroupBarlineValue, GroupName, GroupSymbol, PartGroup};

    let type_attr = reader.get_attr(start.attributes(), "type", "part-group")?;
    let number = reader
        .get_optional_attr(start.attributes(), "number")?
        .unwrap_or_else(|| "1".to_string());

    let start_stop = super::values::parse_start_stop(&type_attr, reader.position())?;

    let mut part_group = PartGroup {
        r#type: start_stop,
        number: Some(number),
        group_name: None,
        group_name_display: None,
        group_abbreviation: None,
        group_abbreviation_display: None,
        group_symbol: None,
        group_barline: None,
        group_time: None,
        editorial: Editorial::default(),
    };

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "group-name" => {
                        let value = reader.read_text("group-name")?;
                        part_group.group_name = Some(GroupName {
                            value,
                            print_style: PrintStyle::default(),
                            justify: None,
                        });
                    }
                    "group-name-display" => {
                        reader.skip_element("group-name-display")?;
                    }
                    "group-abbreviation" => {
                        reader.skip_element("group-abbreviation")?;
                    }
                    "group-abbreviation-display" => {
                        reader.skip_element("group-abbreviation-display")?;
                    }
                    "group-symbol" => {
                        let value = reader.read_text("group-symbol")?;
                        let symbol_value = match value.as_str() {
                            "none" => GroupSymbolValue::None,
                            "brace" => GroupSymbolValue::Brace,
                            "line" => GroupSymbolValue::Line,
                            "bracket" => GroupSymbolValue::Bracket,
                            "square" => GroupSymbolValue::Square,
                            _ => {
                                return Err(ParseError::invalid_value(
                                    "group-symbol-value",
                                    &value,
                                    reader.position(),
                                ));
                            }
                        };
                        part_group.group_symbol = Some(GroupSymbol {
                            value: symbol_value,
                            position: crate::ir::common::Position::default(),
                            color: None,
                        });
                    }
                    "group-barline" => {
                        let value = reader.read_text("group-barline")?;
                        let barline_value = match value.as_str() {
                            "yes" => GroupBarlineValue::Yes,
                            "no" => GroupBarlineValue::No,
                            "Mensurstrich" => GroupBarlineValue::Mensurstrich,
                            _ => {
                                return Err(ParseError::invalid_value(
                                    "group-barline-value",
                                    &value,
                                    reader.position(),
                                ));
                            }
                        };
                        part_group.group_barline = Some(crate::ir::part::GroupBarline {
                            value: barline_value,
                            color: None,
                        });
                    }
                    "group-time" => {
                        reader.skip_element("group-time")?;
                    }
                    "footnote" | "level" => {
                        // Skip editorial elements for now
                        reader.skip_element(&name)?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "group-symbol" => {
                        // Empty group-symbol means "none"
                        part_group.group_symbol = Some(GroupSymbol {
                            value: GroupSymbolValue::None,
                            position: crate::ir::common::Position::default(),
                            color: None,
                        });
                    }
                    "group-time" => {
                        part_group.group_time = Some(());
                    }
                    _ => {}
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in part-group",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(part_group)
}

/// Parse an empty part-group element.
fn parse_part_group_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::part::PartGroup, ParseError> {
    use crate::ir::part::PartGroup;

    let type_attr = reader.get_attr(start.attributes(), "type", "part-group")?;
    let number = reader
        .get_optional_attr(start.attributes(), "number")?
        .unwrap_or_else(|| "1".to_string());

    let start_stop = super::values::parse_start_stop(&type_attr, reader.position())?;

    Ok(PartGroup {
        r#type: start_stop,
        number: Some(number),
        group_name: None,
        group_name_display: None,
        group_abbreviation: None,
        group_abbreviation_display: None,
        group_symbol: None,
        group_barline: None,
        group_time: None,
        editorial: Editorial::default(),
    })
}

/// Parse a part element.
fn parse_part(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
    part_list: &PartList,
) -> Result<Part, ParseError> {
    let id = reader.get_attr(start.attributes(), "id", "part")?;

    // Validate that the part ID exists in the part-list
    let id_exists = part_list.content.iter().any(|elem| match elem {
        PartListElement::ScorePart(sp) => sp.id == id,
        _ => false,
    });

    if !id_exists {
        return Err(ParseError::undefined_reference(
            "part",
            &id,
            reader.position(),
        ));
    }

    let mut measures = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "measure" => {
                        let measure = parse_measure(reader, &e)?;
                        measures.push(measure);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "measure" {
                    // Empty measure
                    let number = reader.get_attr(e.attributes(), "number", "measure")?;
                    measures.push(Measure {
                        number,
                        implicit: None,
                        non_controlling: None,
                        width: None,
                        content: vec![],
                    });
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in part", reader.position()));
            }
            _ => {}
        }
    }

    Ok(Part { id, measures })
}

/// Parse a measure element.
fn parse_measure(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Measure, ParseError> {
    let number = reader.get_attr(start.attributes(), "number", "measure")?;
    let implicit = reader
        .get_optional_attr(start.attributes(), "implicit")?
        .map(|s| super::values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let non_controlling = reader
        .get_optional_attr(start.attributes(), "non-controlling")?
        .map(|s| super::values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let width = reader.get_optional_attr_as::<f64>(start.attributes(), "width")?;

    let mut content = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "note" => {
                        let note = parse_note(reader, &e)?;
                        content.push(crate::ir::measure::MusicDataElement::Note(Box::new(note)));
                    }
                    "backup" => {
                        let backup = parse_backup(reader)?;
                        content.push(crate::ir::measure::MusicDataElement::Backup(backup));
                    }
                    "forward" => {
                        let forward = parse_forward(reader, &e)?;
                        content.push(crate::ir::measure::MusicDataElement::Forward(forward));
                    }
                    "direction" => {
                        let direction = parse_direction(reader, &e)?;
                        content.push(crate::ir::measure::MusicDataElement::Direction(Box::new(
                            direction,
                        )));
                    }
                    "attributes" => {
                        let attrs = parse_attributes(reader)?;
                        content.push(crate::ir::measure::MusicDataElement::Attributes(Box::new(
                            attrs,
                        )));
                    }
                    "barline" => {
                        let barline = parse_barline(reader, &e)?;
                        content.push(crate::ir::measure::MusicDataElement::Barline(Box::new(
                            barline,
                        )));
                    }
                    "harmony" => {
                        // TODO: Parse harmony
                        reader.skip_element("harmony")?;
                    }
                    "figured-bass" => {
                        // TODO: Parse figured-bass
                        reader.skip_element("figured-bass")?;
                    }
                    "print" => {
                        // TODO: Parse print
                        reader.skip_element("print")?;
                    }
                    "sound" => {
                        // TODO: Parse sound
                        reader.skip_element("sound")?;
                    }
                    "listening" => {
                        // TODO: Parse listening
                        reader.skip_element("listening")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "print" | "sound" | "listening" => {
                        // Empty versions - skip for now
                    }
                    _ => {}
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in measure",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Measure {
        number,
        implicit,
        non_controlling,
        width,
        content,
    })
}

// === Stub functions for elements that will be fully implemented in later milestones ===

/// Parse a note element (stub - will be fully implemented in Milestone 2).
fn parse_note(
    reader: &mut XmlReader<'_>,
    _start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::note::Note, ParseError> {
    use crate::ir::common::Position;
    use crate::ir::note::{FullNote, NoteContent, PitchRestUnpitched, Rest};

    // For now, just skip the note content and return a minimal note
    reader.skip_element("note")?;

    // Return a minimal rest note as a placeholder
    Ok(crate::ir::note::Note {
        position: Position::default(),
        dynamics: None,
        end_dynamics: None,
        attack: None,
        release: None,
        pizzicato: None,
        print_object: None,
        content: NoteContent::Regular {
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Rest(Rest::default()),
            },
            duration: 1,
            ties: vec![],
        },
        instrument: vec![],
        voice: None,
        r#type: None,
        dots: vec![],
        accidental: None,
        time_modification: None,
        stem: None,
        notehead: None,
        staff: None,
        beams: vec![],
        notations: vec![],
        lyrics: vec![],
    })
}

/// Parse an attributes element (stub - will be fully implemented in Milestone 2).
fn parse_attributes(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::attributes::Attributes, ParseError> {
    reader.skip_element("attributes")?;

    Ok(crate::ir::attributes::Attributes {
        editorial: Editorial::default(),
        divisions: None,
        keys: vec![],
        times: vec![],
        staves: None,
        part_symbol: None,
        instruments: None,
        clefs: vec![],
        staff_details: vec![],
        transpose: vec![],
        measure_styles: vec![],
    })
}

/// Parse a direction element (stub - will be fully implemented in Milestone 3).
fn parse_direction(
    reader: &mut XmlReader<'_>,
    _start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Direction, ParseError> {
    reader.skip_element("direction")?;

    Ok(crate::ir::direction::Direction {
        placement: None,
        directive: None,
        direction_types: vec![],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

/// Parse a barline element (stub - will be fully implemented in Milestone 3).
fn parse_barline(
    reader: &mut XmlReader<'_>,
    _start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::attributes::Barline, ParseError> {
    reader.skip_element("barline")?;

    Ok(crate::ir::attributes::Barline {
        location: None,
        bar_style: None,
        editorial: Editorial::default(),
        wavy_line: None,
        segno: None,
        coda: None,
        fermatas: vec![],
        ending: None,
        repeat: None,
    })
}

/// Parse a backup element (stub - will be fully implemented in Milestone 2).
fn parse_backup(reader: &mut XmlReader<'_>) -> Result<crate::ir::voice::Backup, ParseError> {
    let mut duration = 0u64;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "duration" => {
                        duration = reader.read_text_as("duration")?;
                    }
                    "footnote" | "level" => {
                        reader.skip_element(&name)?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in backup",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(crate::ir::voice::Backup {
        duration,
        editorial: Editorial::default(),
    })
}

/// Parse a forward element (stub - will be fully implemented in Milestone 2).
fn parse_forward(
    reader: &mut XmlReader<'_>,
    _start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::voice::Forward, ParseError> {
    let mut duration = 0u64;
    let mut voice = None;
    let mut staff = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "duration" => {
                        duration = reader.read_text_as("duration")?;
                    }
                    "voice" => {
                        voice = Some(reader.read_text("voice")?);
                    }
                    "staff" => {
                        staff = Some(reader.read_text_as("staff")?);
                    }
                    "footnote" | "level" => {
                        reader.skip_element(&name)?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in forward",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(crate::ir::voice::Forward {
        duration,
        voice,
        staff,
        editorial: Editorial::default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_score Tests ===

    #[test]
    fn test_parse_score_minimal() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <score-partwise version="4.0">
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.version, Some("4.0".to_string()));
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].id, "P1");
        assert_eq!(score.parts[0].measures.len(), 1);
        assert_eq!(score.parts[0].measures[0].number, "1");
    }

    #[test]
    fn test_parse_score_with_doctype() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
            <score-partwise version="4.0">
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.version, Some("4.0".to_string()));
    }

    #[test]
    fn test_parse_score_with_movement_title() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <movement-title>Symphony No. 5</movement-title>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.movement_title, Some("Symphony No. 5".to_string()));
    }

    #[test]
    fn test_parse_score_missing_part_list() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        if let Err(ParseError::MissingElement {
            element, parent, ..
        }) = result
        {
            assert_eq!(element, "part-list");
            assert_eq!(parent, "score-partwise");
        } else {
            panic!("Expected MissingElement error");
        }
    }

    #[test]
    fn test_parse_score_undefined_part_reference() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P2">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        if let Err(ParseError::UndefinedReference {
            reference_type, id, ..
        }) = result
        {
            assert_eq!(reference_type, "part");
            assert_eq!(id, "P2");
        } else {
            panic!("Expected UndefinedReference error");
        }
    }

    #[test]
    fn test_parse_score_timewise_not_supported() {
        let xml = r#"<?xml version="1.0"?>
            <score-timewise>
            </score-timewise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
    }

    // === Part List Tests ===

    #[test]
    fn test_parse_multiple_score_parts() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin I</part-name>
                    </score-part>
                    <score-part id="P2">
                        <part-name>Violin II</part-name>
                    </score-part>
                </part-list>
                <part id="P1"><measure number="1"/></part>
                <part id="P2"><measure number="1"/></part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.part_list.content.len(), 2);
        assert_eq!(score.parts.len(), 2);
    }

    #[test]
    fn test_parse_part_group() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-name>Strings</group-name>
                        <group-symbol>bracket</group-symbol>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.part_list.content.len(), 3);

        // First element should be part-group start
        if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
            assert_eq!(pg.r#type, crate::ir::common::StartStop::Start);
            assert_eq!(pg.number, Some("1".to_string()));
            assert!(pg.group_name.is_some());
        } else {
            panic!("Expected PartGroup");
        }
    }

    // === Measure Tests ===

    #[test]
    fn test_parse_measure_with_attributes() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1" implicit="yes" width="200">
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        let measure = &score.parts[0].measures[0];
        assert_eq!(measure.number, "1");
        assert_eq!(measure.implicit, Some(crate::ir::common::YesNo::Yes));
        assert_eq!(measure.width, Some(200.0));
    }

    #[test]
    fn test_parse_multiple_measures() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                    <measure number="2"/>
                    <measure number="3"/>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.parts[0].measures.len(), 3);
        assert_eq!(score.parts[0].measures[0].number, "1");
        assert_eq!(score.parts[0].measures[1].number, "2");
        assert_eq!(score.parts[0].measures[2].number, "3");
    }

    // === Backup/Forward Tests ===

    #[test]
    fn test_parse_backup() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <backup>
                            <duration>4</duration>
                        </backup>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.parts[0].measures[0].content.len(), 1);
        if let crate::ir::measure::MusicDataElement::Backup(backup) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(backup.duration, 4);
        } else {
            panic!("Expected Backup");
        }
    }

    #[test]
    fn test_parse_forward() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <forward>
                            <duration>8</duration>
                            <voice>2</voice>
                            <staff>1</staff>
                        </forward>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Forward(forward) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(forward.duration, 8);
            assert_eq!(forward.voice, Some("2".to_string()));
            assert_eq!(forward.staff, Some(1));
        } else {
            panic!("Expected Forward");
        }
    }

    // === Error Case Tests ===

    #[test]
    fn test_parse_empty_document() {
        let xml = "";
        let result = parse_score(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_measure_number() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure>
                    </measure>
                </part>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        if let Err(ParseError::MissingAttribute {
            attribute, element, ..
        }) = result
        {
            assert_eq!(attribute, "number");
            assert_eq!(element, "measure");
        } else {
            panic!("Expected MissingAttribute error");
        }
    }

    #[test]
    fn test_parse_missing_score_part_id() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part>
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_part_name() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                    </score-part>
                </part-list>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        if let Err(ParseError::MissingElement {
            element, parent, ..
        }) = result
        {
            assert_eq!(element, "part-name");
            assert_eq!(parent, "score-part");
        } else {
            panic!("Expected MissingElement error");
        }
    }
}
