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
use super::values;
use crate::ir::attributes::{
    Attributes, Cancel, Clef, ClefSign, Key, KeyContent, Mode, Time, TimeContent, TimeSignature,
    TraditionalKey,
};
use crate::ir::beam::{Beam, Notehead, Stem};
use crate::ir::common::{Editorial, Font, Position, YesNo};
use crate::ir::duration::{Dot, NoteType, TimeModification};
use crate::ir::measure::Measure;
use crate::ir::note::{
    Accidental, FullNote, Grace, Note, NoteContent, PitchRestUnpitched, Rest, Tie,
};
use crate::ir::part::{PartList, PartListElement, PartName, ScorePart};
use crate::ir::pitch::{Pitch, Unpitched};
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

/// Parse a note element.
///
/// Notes are the fundamental music content in MusicXML. They can be:
/// - Regular notes (with pitch and duration)
/// - Grace notes (no duration, steal time from surrounding notes)
/// - Cue notes (small notes used as cues)
/// - Chord notes (share duration with previous note)
/// - Rests (no pitch)
fn parse_note(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Note, ParseError> {
    // Parse note attributes
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let dynamics = reader.get_optional_attr_as::<f64>(start.attributes(), "dynamics")?;
    let end_dynamics = reader.get_optional_attr_as::<f64>(start.attributes(), "end-dynamics")?;
    let attack = reader.get_optional_attr_as::<i64>(start.attributes(), "attack")?;
    let release = reader.get_optional_attr_as::<i64>(start.attributes(), "release")?;
    let pizzicato = reader
        .get_optional_attr(start.attributes(), "pizzicato")?
        .map(|s| s == "yes");

    // State for building the note
    let mut grace: Option<Grace> = None;
    let mut is_cue = false;
    let mut is_chord = false;
    let mut pitch: Option<Pitch> = None;
    let mut rest: Option<Rest> = None;
    let mut unpitched: Option<Unpitched> = None;
    let mut duration: Option<u64> = None;
    let mut ties: Vec<Tie> = Vec::new();
    let mut voice: Option<String> = None;
    let mut note_type: Option<NoteType> = None;
    let mut dots: Vec<Dot> = Vec::new();
    let mut accidental: Option<Accidental> = None;
    let mut time_modification: Option<TimeModification> = None;
    let mut stem: Option<Stem> = None;
    let mut notehead: Option<Notehead> = None;
    let mut staff: Option<u16> = None;
    let mut beams: Vec<Beam> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "grace" => {
                        grace = Some(parse_grace(reader, &e)?);
                    }
                    "cue" => {
                        is_cue = true;
                        reader.skip_element("cue")?;
                    }
                    "chord" => {
                        is_chord = true;
                        reader.skip_element("chord")?;
                    }
                    "pitch" => {
                        pitch = Some(parse_pitch(reader)?);
                    }
                    "rest" => {
                        rest = Some(parse_rest(reader, &e)?);
                    }
                    "unpitched" => {
                        unpitched = Some(parse_unpitched(reader)?);
                    }
                    "duration" => {
                        duration = Some(reader.read_text_as("duration")?);
                    }
                    "tie" => {
                        ties.push(parse_tie(reader, &e)?);
                    }
                    "voice" => {
                        voice = Some(reader.read_text("voice")?);
                    }
                    "type" => {
                        note_type = Some(parse_note_type(reader, &e)?);
                    }
                    "dot" => {
                        dots.push(parse_dot(reader, &e)?);
                    }
                    "accidental" => {
                        accidental = Some(parse_accidental(reader, &e)?);
                    }
                    "time-modification" => {
                        time_modification = Some(parse_time_modification(reader)?);
                    }
                    "stem" => {
                        stem = Some(parse_stem(reader, &e)?);
                    }
                    "notehead" => {
                        notehead = Some(parse_notehead(reader, &e)?);
                    }
                    "staff" => {
                        staff = Some(reader.read_text_as("staff")?);
                    }
                    "beam" => {
                        beams.push(parse_beam(reader, &e)?);
                    }
                    "notations" => {
                        // TODO: Parse notations fully in a later milestone
                        reader.skip_element("notations")?;
                    }
                    "lyric" => {
                        // TODO: Parse lyrics fully in a later milestone
                        reader.skip_element("lyric")?;
                    }
                    "instrument" => {
                        // TODO: Parse instrument references
                        reader.skip_element("instrument")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "grace" => {
                        grace = Some(parse_grace_from_empty(&e, reader)?);
                    }
                    "cue" => {
                        is_cue = true;
                    }
                    "chord" => {
                        is_chord = true;
                    }
                    "rest" => {
                        rest = Some(parse_rest_from_empty(&e, reader)?);
                    }
                    "tie" => {
                        ties.push(parse_tie_from_empty(&e, reader)?);
                    }
                    "dot" => {
                        dots.push(parse_dot_from_empty(&e, reader)?);
                    }
                    _ => {
                        // Skip unknown empty elements
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in note", reader.position()));
            }
            _ => {}
        }
    }

    // Build the full note content
    let content = if let Some(p) = pitch {
        PitchRestUnpitched::Pitch(p)
    } else if let Some(r) = rest {
        PitchRestUnpitched::Rest(r)
    } else if let Some(u) = unpitched {
        PitchRestUnpitched::Unpitched(u)
    } else {
        // Default to rest if nothing specified
        PitchRestUnpitched::Rest(Rest::default())
    };

    let full_note = FullNote {
        chord: is_chord,
        content,
    };

    // Build the note content variant
    let note_content = if let Some(g) = grace {
        NoteContent::Grace {
            grace: g,
            full_note,
            ties,
        }
    } else if is_cue {
        NoteContent::Cue {
            full_note,
            duration: duration.unwrap_or(1),
        }
    } else {
        NoteContent::Regular {
            full_note,
            duration: duration.unwrap_or(1),
            ties,
        }
    };

    Ok(Note {
        position: Position::default(),
        dynamics,
        end_dynamics,
        attack,
        release,
        pizzicato,
        print_object,
        content: note_content,
        instrument: vec![],
        voice,
        r#type: note_type,
        dots,
        accidental,
        time_modification,
        stem,
        notehead,
        staff,
        beams,
        notations: vec![],
        lyrics: vec![],
    })
}

/// Parse a pitch element.
fn parse_pitch(reader: &mut XmlReader<'_>) -> Result<Pitch, ParseError> {
    let mut step: Option<crate::ir::pitch::Step> = None;
    let mut alter: Option<f64> = None;
    let mut octave: Option<u8> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "step" => {
                        let step_text = reader.read_text("step")?;
                        step = Some(values::parse_step(&step_text, reader.position())?);
                    }
                    "alter" => {
                        alter = Some(reader.read_text_as("alter")?);
                    }
                    "octave" => {
                        octave = Some(reader.read_text_as("octave")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in pitch",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Pitch {
        step: step
            .ok_or_else(|| ParseError::missing_element("step", "pitch", reader.position()))?,
        alter,
        octave: octave
            .ok_or_else(|| ParseError::missing_element("octave", "pitch", reader.position()))?,
    })
}

/// Parse a rest element.
fn parse_rest(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Rest, ParseError> {
    let measure = reader
        .get_optional_attr(start.attributes(), "measure")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut display_step: Option<crate::ir::pitch::Step> = None;
    let mut display_octave: Option<u8> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "display-step" => {
                        let step_text = reader.read_text("display-step")?;
                        display_step = Some(values::parse_step(&step_text, reader.position())?);
                    }
                    "display-octave" => {
                        display_octave = Some(reader.read_text_as("display-octave")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in rest", reader.position()));
            }
            _ => {}
        }
    }

    Ok(Rest {
        measure,
        display_step,
        display_octave,
    })
}

/// Parse a rest from an empty element.
fn parse_rest_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Rest, ParseError> {
    let measure = reader
        .get_optional_attr(start.attributes(), "measure")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Rest {
        measure,
        display_step: None,
        display_octave: None,
    })
}

/// Parse an unpitched element.
fn parse_unpitched(reader: &mut XmlReader<'_>) -> Result<Unpitched, ParseError> {
    let mut display_step: Option<crate::ir::pitch::Step> = None;
    let mut display_octave: Option<u8> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "display-step" => {
                        let step_text = reader.read_text("display-step")?;
                        display_step = Some(values::parse_step(&step_text, reader.position())?);
                    }
                    "display-octave" => {
                        display_octave = Some(reader.read_text_as("display-octave")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in unpitched",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Unpitched {
        display_step,
        display_octave,
    })
}

/// Parse a grace element.
fn parse_grace(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Grace, ParseError> {
    let steal_time_previous =
        reader.get_optional_attr_as::<f64>(start.attributes(), "steal-time-previous")?;
    let steal_time_following =
        reader.get_optional_attr_as::<f64>(start.attributes(), "steal-time-following")?;
    let make_time = reader.get_optional_attr_as::<i64>(start.attributes(), "make-time")?;
    let slash = reader
        .get_optional_attr(start.attributes(), "slash")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    // Skip any content (grace is usually empty)
    reader.skip_element("grace")?;

    Ok(Grace {
        steal_time_previous,
        steal_time_following,
        make_time,
        slash,
    })
}

/// Parse a grace element from an empty tag.
fn parse_grace_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Grace, ParseError> {
    let steal_time_previous =
        reader.get_optional_attr_as::<f64>(start.attributes(), "steal-time-previous")?;
    let steal_time_following =
        reader.get_optional_attr_as::<f64>(start.attributes(), "steal-time-following")?;
    let make_time = reader.get_optional_attr_as::<i64>(start.attributes(), "make-time")?;
    let slash = reader
        .get_optional_attr(start.attributes(), "slash")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Grace {
        steal_time_previous,
        steal_time_following,
        make_time,
        slash,
    })
}

/// Parse a tie element.
fn parse_tie(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Tie, ParseError> {
    let type_attr = reader.get_attr(start.attributes(), "type", "tie")?;
    let r#type = values::parse_start_stop(&type_attr, reader.position())?;
    let time_only = reader.get_optional_attr(start.attributes(), "time-only")?;

    // Skip any content
    reader.skip_element("tie")?;

    Ok(Tie { r#type, time_only })
}

/// Parse a tie element from an empty tag.
fn parse_tie_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Tie, ParseError> {
    let type_attr = reader.get_attr(start.attributes(), "type", "tie")?;
    let r#type = values::parse_start_stop(&type_attr, reader.position())?;
    let time_only = reader.get_optional_attr(start.attributes(), "time-only")?;

    Ok(Tie { r#type, time_only })
}

/// Parse a note type element.
fn parse_note_type(
    reader: &mut XmlReader<'_>,
    _start: &quick_xml::events::BytesStart<'_>,
) -> Result<NoteType, ParseError> {
    let text = reader.read_text("type")?;
    let value = values::parse_note_type_value(&text, reader.position())?;

    Ok(NoteType { value, size: None })
}

/// Parse a dot element.
fn parse_dot(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Dot, ParseError> {
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    // Skip any content
    reader.skip_element("dot")?;

    Ok(Dot {
        placement,
        position: Position::default(),
    })
}

/// Parse a dot element from an empty tag.
fn parse_dot_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Dot, ParseError> {
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    Ok(Dot {
        placement,
        position: Position::default(),
    })
}

/// Parse an accidental element.
fn parse_accidental(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Accidental, ParseError> {
    let cautionary = reader
        .get_optional_attr(start.attributes(), "cautionary")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let editorial = reader
        .get_optional_attr(start.attributes(), "editorial")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let parentheses = reader
        .get_optional_attr(start.attributes(), "parentheses")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let bracket = reader
        .get_optional_attr(start.attributes(), "bracket")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let text = reader.read_text("accidental")?;
    let value = values::parse_accidental_value(&text, reader.position())?;

    Ok(Accidental {
        value,
        cautionary,
        editorial,
        parentheses,
        bracket,
        size: None,
    })
}

/// Parse a time-modification element.
fn parse_time_modification(reader: &mut XmlReader<'_>) -> Result<TimeModification, ParseError> {
    let mut actual_notes: Option<u32> = None;
    let mut normal_notes: Option<u32> = None;
    let mut normal_type: Option<crate::ir::duration::NoteTypeValue> = None;
    let mut normal_dots: u32 = 0;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "actual-notes" => {
                        actual_notes = Some(reader.read_text_as("actual-notes")?);
                    }
                    "normal-notes" => {
                        normal_notes = Some(reader.read_text_as("normal-notes")?);
                    }
                    "normal-type" => {
                        let type_text = reader.read_text("normal-type")?;
                        normal_type = Some(values::parse_note_type_value(
                            &type_text,
                            reader.position(),
                        )?);
                    }
                    "normal-dot" => {
                        normal_dots += 1;
                        reader.skip_element("normal-dot")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "normal-dot" {
                    normal_dots += 1;
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in time-modification",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(TimeModification {
        actual_notes: actual_notes.unwrap_or(3),
        normal_notes: normal_notes.unwrap_or(2),
        normal_type,
        normal_dots,
    })
}

/// Parse a stem element.
fn parse_stem(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Stem, ParseError> {
    let default_y = reader.get_optional_attr_as::<f64>(start.attributes(), "default-y")?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    let text = reader.read_text("stem")?;
    let value = values::parse_stem_value(&text, reader.position())?;

    Ok(Stem {
        value,
        default_y,
        color,
    })
}

/// Parse a notehead element.
fn parse_notehead(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Notehead, ParseError> {
    let filled = reader
        .get_optional_attr(start.attributes(), "filled")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let parentheses = reader
        .get_optional_attr(start.attributes(), "parentheses")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    let text = reader.read_text("notehead")?;
    let value = values::parse_notehead_value(&text, reader.position())?;

    Ok(Notehead {
        value,
        filled,
        parentheses,
        font: Font::default(),
        color,
    })
}

/// Parse a beam element.
fn parse_beam(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Beam, ParseError> {
    let number: u8 = reader
        .get_optional_attr_as(start.attributes(), "number")?
        .unwrap_or(1);
    let fan = reader
        .get_optional_attr(start.attributes(), "fan")?
        .map(|s| values::parse_fan(&s, reader.position()))
        .transpose()?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    let text = reader.read_text("beam")?;
    let value = values::parse_beam_value(&text, reader.position())?;

    Ok(Beam {
        value,
        number,
        fan,
        color,
    })
}

/// Parse an attributes element.
///
/// Attributes contain information about key signature, time signature, clef,
/// divisions, and other musical notation symbols that affect how subsequent
/// notes are interpreted and displayed.
fn parse_attributes(reader: &mut XmlReader<'_>) -> Result<Attributes, ParseError> {
    let mut attrs = Attributes::default();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "divisions" => {
                        attrs.divisions = Some(reader.read_text_as("divisions")?);
                    }
                    "key" => {
                        let key = parse_key(reader, &e)?;
                        attrs.keys.push(key);
                    }
                    "time" => {
                        let time = parse_time(reader, &e)?;
                        attrs.times.push(time);
                    }
                    "staves" => {
                        attrs.staves = Some(reader.read_text_as("staves")?);
                    }
                    "part-symbol" => {
                        // TODO: Parse part-symbol fully
                        reader.skip_element("part-symbol")?;
                    }
                    "instruments" => {
                        attrs.instruments = Some(reader.read_text_as("instruments")?);
                    }
                    "clef" => {
                        let clef = parse_clef(reader, &e)?;
                        attrs.clefs.push(clef);
                    }
                    "staff-details" => {
                        // TODO: Parse staff-details fully
                        reader.skip_element("staff-details")?;
                    }
                    "transpose" => {
                        let transpose = parse_transpose(reader, &e)?;
                        attrs.transpose.push(transpose);
                    }
                    "measure-style" => {
                        // TODO: Parse measure-style fully
                        reader.skip_element("measure-style")?;
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
                    "clef" => {
                        let clef = parse_clef_from_empty(&e, reader)?;
                        attrs.clefs.push(clef);
                    }
                    _ => {
                        // Skip unknown empty elements
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in attributes",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(attrs)
}

/// Parse a key element.
fn parse_key(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Key, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut fifths: Option<i8> = None;
    let mut mode: Option<Mode> = None;
    let mut cancel: Option<Cancel> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "cancel" => {
                        // Get location attribute before reading text content
                        let location = reader
                            .get_optional_attr(e.attributes(), "location")?
                            .map(|s| values::parse_cancel_location(&s, reader.position()))
                            .transpose()?;
                        let cancel_fifths: i8 = reader.read_text_as("cancel")?;
                        cancel = Some(Cancel {
                            fifths: cancel_fifths,
                            location,
                        });
                    }
                    "fifths" => {
                        fifths = Some(reader.read_text_as("fifths")?);
                    }
                    "mode" => {
                        let mode_text = reader.read_text("mode")?;
                        mode = Some(values::parse_mode(&mode_text, reader.position())?);
                    }
                    "key-step" | "key-alter" | "key-accidental" => {
                        // TODO: Support non-traditional keys
                        reader.skip_element(&name)?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in key", reader.position()));
            }
            _ => {}
        }
    }

    // For traditional keys, fifths is required
    let content = if let Some(f) = fifths {
        KeyContent::Traditional(TraditionalKey {
            cancel,
            fifths: f,
            mode,
        })
    } else {
        // Default to C major if no key content specified
        KeyContent::Traditional(TraditionalKey {
            cancel: None,
            fifths: 0,
            mode: None,
        })
    };

    Ok(Key {
        content,
        number,
        print_object,
    })
}

/// Parse a time element.
fn parse_time(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Time, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;
    let symbol = reader
        .get_optional_attr(start.attributes(), "symbol")?
        .map(|s| values::parse_time_symbol(&s, reader.position()))
        .transpose()?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut signatures = Vec::new();
    let mut current_beats: Option<String> = None;
    let mut senza_misura: Option<String> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "beats" => {
                        current_beats = Some(reader.read_text("beats")?);
                    }
                    "beat-type" => {
                        let beat_type = reader.read_text("beat-type")?;
                        if let Some(beats) = current_beats.take() {
                            signatures.push(TimeSignature { beats, beat_type });
                        }
                    }
                    "senza-misura" => {
                        senza_misura = Some(reader.read_text("senza-misura")?);
                    }
                    "interchangeable" => {
                        // TODO: Parse interchangeable time signatures
                        reader.skip_element("interchangeable")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "senza-misura" {
                    senza_misura = Some(String::new());
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in time", reader.position()));
            }
            _ => {}
        }
    }

    let content = if let Some(text) = senza_misura {
        TimeContent::SenzaMisura(text)
    } else {
        TimeContent::Measured { signatures }
    };

    Ok(Time {
        content,
        number,
        symbol,
        print_object,
    })
}

/// Parse a clef element.
fn parse_clef(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Clef, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut sign: Option<ClefSign> = None;
    let mut line: Option<u8> = None;
    let mut octave_change: Option<i8> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "sign" => {
                        let sign_text = reader.read_text("sign")?;
                        sign = Some(values::parse_clef_sign(&sign_text, reader.position())?);
                    }
                    "line" => {
                        line = Some(reader.read_text_as("line")?);
                    }
                    "clef-octave-change" => {
                        octave_change = Some(reader.read_text_as("clef-octave-change")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in clef", reader.position()));
            }
            _ => {}
        }
    }

    Ok(Clef {
        sign: sign.unwrap_or(ClefSign::G),
        line,
        octave_change,
        number,
        size: None,
        print_object,
    })
}

/// Parse a clef from an empty element.
fn parse_clef_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Clef, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Clef {
        sign: ClefSign::G,
        line: None,
        octave_change: None,
        number,
        size: None,
        print_object,
    })
}

/// Parse a transpose element.
fn parse_transpose(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::attributes::Transpose, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;

    let mut diatonic: Option<i32> = None;
    let mut chromatic: i32 = 0;
    let mut octave_change: Option<i32> = None;
    let mut double: Option<YesNo> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "diatonic" => {
                        diatonic = Some(reader.read_text_as("diatonic")?);
                    }
                    "chromatic" => {
                        chromatic = reader.read_text_as("chromatic")?;
                    }
                    "octave-change" => {
                        octave_change = Some(reader.read_text_as("octave-change")?);
                    }
                    "double" => {
                        // Empty element means "yes"
                        double = Some(YesNo::Yes);
                        reader.skip_element("double")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "double" {
                    double = Some(YesNo::Yes);
                }
            }
            Event::End(_) => {
                break;
            }
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in transpose",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(crate::ir::attributes::Transpose {
        number,
        diatonic,
        chromatic,
        octave_change,
        double,
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
    use crate::ir::attributes::TimeSymbol;

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

    // === Attributes Tests ===

    #[test]
    fn test_parse_attributes_divisions() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.divisions, Some(4));
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_attributes_key_signature() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <key>
                                <fifths>2</fifths>
                                <mode>major</mode>
                            </key>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.keys.len(), 1);
            if let KeyContent::Traditional(tk) = &attrs.keys[0].content {
                assert_eq!(tk.fifths, 2);
                assert_eq!(tk.mode, Some(Mode::Major));
            } else {
                panic!("Expected Traditional key");
            }
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_attributes_time_signature() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <time>
                                <beats>4</beats>
                                <beat-type>4</beat-type>
                            </time>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.times.len(), 1);
            if let TimeContent::Measured { signatures } = &attrs.times[0].content {
                assert_eq!(signatures.len(), 1);
                assert_eq!(signatures[0].beats, "4");
                assert_eq!(signatures[0].beat_type, "4");
            } else {
                panic!("Expected Measured time");
            }
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_attributes_clef() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <clef>
                                <sign>G</sign>
                                <line>2</line>
                            </clef>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.clefs.len(), 1);
            assert_eq!(attrs.clefs[0].sign, ClefSign::G);
            assert_eq!(attrs.clefs[0].line, Some(2));
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_attributes_complete() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                            <key>
                                <fifths>-1</fifths>
                                <mode>major</mode>
                            </key>
                            <time symbol="common">
                                <beats>4</beats>
                                <beat-type>4</beat-type>
                            </time>
                            <clef>
                                <sign>F</sign>
                                <line>4</line>
                            </clef>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.divisions, Some(4));
            assert_eq!(attrs.keys.len(), 1);
            assert_eq!(attrs.times.len(), 1);
            assert_eq!(attrs.times[0].symbol, Some(TimeSymbol::Common));
            assert_eq!(attrs.clefs.len(), 1);
            assert_eq!(attrs.clefs[0].sign, ClefSign::F);
        } else {
            panic!("Expected Attributes");
        }
    }

    // === Note Tests ===

    #[test]
    fn test_parse_note_pitched() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let NoteContent::Regular {
                full_note,
                duration,
                ..
            } = &note.content
            {
                assert_eq!(*duration, 4);
                if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                    assert_eq!(p.step, crate::ir::pitch::Step::C);
                    assert_eq!(p.octave, 4);
                } else {
                    panic!("Expected Pitch");
                }
            } else {
                panic!("Expected Regular note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_accidental() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>F</step>
                                <alter>1</alter>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <accidental>sharp</accidental>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(note.accidental.is_some());
            assert_eq!(
                note.accidental.as_ref().unwrap().value,
                crate::ir::common::AccidentalValue::Sharp
            );
            if let NoteContent::Regular { full_note, .. } = &note.content {
                if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                    assert_eq!(p.alter, Some(1.0));
                } else {
                    panic!("Expected Pitch");
                }
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_rest() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <rest/>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let NoteContent::Regular { full_note, .. } = &note.content {
                assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
            } else {
                panic!("Expected Regular note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_whole_measure_rest() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <rest measure="yes"/>
                            <duration>16</duration>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let NoteContent::Regular { full_note, .. } = &note.content {
                if let PitchRestUnpitched::Rest(r) = &full_note.content {
                    assert_eq!(r.measure, Some(YesNo::Yes));
                } else {
                    panic!("Expected Rest");
                }
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_chord_note() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                        <note>
                            <chord/>
                            <pitch>
                                <step>E</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.parts[0].measures[0].content.len(), 2);
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[1]
        {
            if let NoteContent::Regular { full_note, .. } = &note.content {
                assert!(full_note.chord);
            } else {
                panic!("Expected Regular note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_grace_note() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <grace slash="yes"/>
                            <pitch>
                                <step>D</step>
                                <octave>5</octave>
                            </pitch>
                            <type>eighth</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let NoteContent::Grace { grace, .. } = &note.content {
                assert_eq!(grace.slash, Some(YesNo::Yes));
            } else {
                panic!("Expected Grace note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_beam() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>1</duration>
                            <type>eighth</type>
                            <beam number="1">begin</beam>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(note.beams.len(), 1);
            assert_eq!(note.beams[0].value, crate::ir::beam::BeamValue::Begin);
            assert_eq!(note.beams[0].number, 1);
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_stem() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <type>quarter</type>
                            <stem>up</stem>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(note.stem.is_some());
            assert_eq!(
                note.stem.as_ref().unwrap().value,
                crate::ir::beam::StemValue::Up
            );
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_tie() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <tie type="start"/>
                            <type>quarter</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let NoteContent::Regular { ties, .. } = &note.content {
                assert_eq!(ties.len(), 1);
                assert_eq!(ties[0].r#type, crate::ir::common::StartStop::Start);
            } else {
                panic!("Expected Regular note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_dots() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>6</duration>
                            <type>quarter</type>
                            <dot/>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(note.dots.len(), 1);
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_time_modification() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>2</duration>
                            <type>eighth</type>
                            <time-modification>
                                <actual-notes>3</actual-notes>
                                <normal-notes>2</normal-notes>
                            </time-modification>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(note.time_modification.is_some());
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 3);
            assert_eq!(tm.normal_notes, 2);
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_voice_and_staff() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <voice>1</voice>
                            <type>quarter</type>
                            <staff>1</staff>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(note.voice, Some("1".to_string()));
            assert_eq!(note.staff, Some(1));
        } else {
            panic!("Expected Note");
        }
    }
}
