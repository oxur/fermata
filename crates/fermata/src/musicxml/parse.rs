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
    Attributes, Barline, Cancel, Clef, ClefSign, Ending, Key, KeyContent, Mode, Repeat, Time,
    TimeContent, TimeSignature, TraditionalKey,
};
use crate::ir::beam::{Beam, Notehead, Stem};
use crate::ir::common::{Editorial, Font, Position, WavyLine, YesNo};
use crate::ir::direction::{Coda, Segno};
use crate::ir::duration::{Dot, NoteType, TimeModification};
use crate::ir::measure::Measure;
use crate::ir::notation::{Fermata, FermataShape};
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
    let mut notations: Vec<crate::ir::notation::Notations> = Vec::new();

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
                        notations.push(parse_notations(reader, &e)?);
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
                    "unpitched" => {
                        unpitched = Some(Unpitched::default());
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
        notations,
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

/// Parse a direction element.
///
/// Direction elements contain musical directions like dynamics, wedges (crescendo/diminuendo),
/// metronome markings, pedal markings, rehearsal marks, segno, coda, and other text instructions.
fn parse_direction(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Direction, ParseError> {
    use crate::ir::direction::Direction;

    // Parse direction attributes
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let directive = reader
        .get_optional_attr(start.attributes(), "directive")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut direction_types = Vec::new();
    let mut offset = None;
    let mut voice = None;
    let mut staff = None;
    let mut sound = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "direction-type" => {
                        let dt = parse_direction_type(reader)?;
                        direction_types.push(dt);
                    }
                    "offset" => {
                        offset = Some(parse_offset(reader, &e)?);
                    }
                    "voice" => {
                        voice = Some(reader.read_text("voice")?);
                    }
                    "staff" => {
                        staff = Some(reader.read_text_as("staff")?);
                    }
                    "sound" => {
                        sound = Some(parse_sound(reader, &e)?);
                    }
                    "footnote" | "level" => {
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
                    "offset" => {
                        offset = Some(parse_offset_from_empty(&e, reader)?);
                    }
                    "sound" => {
                        sound = Some(parse_sound_from_empty(&e, reader)?);
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in direction",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Direction {
        placement,
        directive,
        direction_types,
        offset,
        voice,
        staff,
        sound,
    })
}

/// Parse a direction-type element.
fn parse_direction_type(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::direction::DirectionType, ParseError> {
    use crate::ir::direction::{DirectionType, DirectionTypeContent};

    let mut content: Option<DirectionTypeContent> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "dynamics" => {
                        content = Some(DirectionTypeContent::Dynamics(parse_dynamics(reader, &e)?));
                    }
                    "wedge" => {
                        content = Some(DirectionTypeContent::Wedge(parse_wedge(reader, &e)?));
                    }
                    "words" => {
                        let words = parse_words(reader, &e)?;
                        // Words can appear multiple times in direction-type
                        if let Some(DirectionTypeContent::Words(ref mut vec)) = content {
                            vec.push(words);
                        } else {
                            content = Some(DirectionTypeContent::Words(vec![words]));
                        }
                    }
                    "metronome" => {
                        content = Some(DirectionTypeContent::Metronome(parse_metronome(
                            reader, &e,
                        )?));
                    }
                    "rehearsal" => {
                        let text = parse_rehearsal(reader, &e)?;
                        if let Some(DirectionTypeContent::Rehearsal(ref mut vec)) = content {
                            vec.push(text);
                        } else {
                            content = Some(DirectionTypeContent::Rehearsal(vec![text]));
                        }
                    }
                    "segno" => {
                        let segno = parse_segno(reader, &e)?;
                        if let Some(DirectionTypeContent::Segno(ref mut vec)) = content {
                            vec.push(segno);
                        } else {
                            content = Some(DirectionTypeContent::Segno(vec![segno]));
                        }
                    }
                    "coda" => {
                        let coda = parse_coda(reader, &e)?;
                        if let Some(DirectionTypeContent::Coda(ref mut vec)) = content {
                            vec.push(coda);
                        } else {
                            content = Some(DirectionTypeContent::Coda(vec![coda]));
                        }
                    }
                    "pedal" => {
                        content = Some(DirectionTypeContent::Pedal(parse_pedal(reader, &e)?));
                    }
                    "octave-shift" => {
                        content = Some(DirectionTypeContent::OctaveShift(parse_octave_shift(
                            reader, &e,
                        )?));
                    }
                    "dashes" => {
                        content = Some(DirectionTypeContent::Dashes(parse_dashes(reader, &e)?));
                    }
                    "bracket" => {
                        content = Some(DirectionTypeContent::Bracket(parse_bracket(reader, &e)?));
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "dynamics" => {
                        content = Some(DirectionTypeContent::Dynamics(parse_dynamics_from_empty(
                            &e, reader,
                        )?));
                    }
                    "wedge" => {
                        content = Some(DirectionTypeContent::Wedge(parse_wedge_from_empty(
                            &e, reader,
                        )?));
                    }
                    "segno" => {
                        let segno = parse_segno_from_empty(&e, reader)?;
                        if let Some(DirectionTypeContent::Segno(ref mut vec)) = content {
                            vec.push(segno);
                        } else {
                            content = Some(DirectionTypeContent::Segno(vec![segno]));
                        }
                    }
                    "coda" => {
                        let coda = parse_coda_from_empty(&e, reader)?;
                        if let Some(DirectionTypeContent::Coda(ref mut vec)) = content {
                            vec.push(coda);
                        } else {
                            content = Some(DirectionTypeContent::Coda(vec![coda]));
                        }
                    }
                    "pedal" => {
                        content = Some(DirectionTypeContent::Pedal(parse_pedal_from_empty(
                            &e, reader,
                        )?));
                    }
                    "octave-shift" => {
                        content = Some(DirectionTypeContent::OctaveShift(
                            parse_octave_shift_from_empty(&e, reader)?,
                        ));
                    }
                    "dashes" => {
                        content = Some(DirectionTypeContent::Dashes(parse_dashes_from_empty(
                            &e, reader,
                        )?));
                    }
                    "bracket" => {
                        content = Some(DirectionTypeContent::Bracket(parse_bracket_from_empty(
                            &e, reader,
                        )?));
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in direction-type",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    // If no content was parsed, return an empty dynamics as default
    let content = content.unwrap_or_else(|| {
        DirectionTypeContent::Dynamics(crate::ir::direction::Dynamics::default())
    });

    Ok(DirectionType { content })
}

/// Parse a dynamics element.
fn parse_dynamics(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Dynamics, ParseError> {
    use crate::ir::direction::Dynamics;

    let print_style = parse_print_style_attrs(start, reader)?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    let mut content = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                let elem = parse_dynamic_element(&name, reader)?;
                if let Some(e) = elem {
                    content.push(e);
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                let elem = parse_dynamic_element_from_empty(&name);
                if let Some(e) = elem {
                    content.push(e);
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in dynamics",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Dynamics {
        content,
        print_style,
        placement,
    })
}

/// Parse a dynamics element from an empty tag.
fn parse_dynamics_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Dynamics, ParseError> {
    use crate::ir::direction::Dynamics;

    let print_style = parse_print_style_attrs(start, reader)?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    Ok(Dynamics {
        content: vec![],
        print_style,
        placement,
    })
}

/// Parse a dynamic element by name.
fn parse_dynamic_element(
    name: &str,
    reader: &mut XmlReader<'_>,
) -> Result<Option<crate::ir::direction::DynamicElement>, ParseError> {
    use crate::ir::direction::DynamicElement;

    let elem = match name {
        "p" => Some(DynamicElement::P),
        "pp" => Some(DynamicElement::PP),
        "ppp" => Some(DynamicElement::PPP),
        "pppp" => Some(DynamicElement::PPPP),
        "ppppp" => Some(DynamicElement::PPPPP),
        "pppppp" => Some(DynamicElement::PPPPPP),
        "f" => Some(DynamicElement::F),
        "ff" => Some(DynamicElement::FF),
        "fff" => Some(DynamicElement::FFF),
        "ffff" => Some(DynamicElement::FFFF),
        "fffff" => Some(DynamicElement::FFFFF),
        "ffffff" => Some(DynamicElement::FFFFFF),
        "mp" => Some(DynamicElement::MP),
        "mf" => Some(DynamicElement::MF),
        "sf" => Some(DynamicElement::SF),
        "sfp" => Some(DynamicElement::SFP),
        "sfpp" => Some(DynamicElement::SFPP),
        "fp" => Some(DynamicElement::FP),
        "rf" => Some(DynamicElement::RF),
        "rfz" => Some(DynamicElement::RFZ),
        "sfz" => Some(DynamicElement::SFZ),
        "sffz" => Some(DynamicElement::SFFZ),
        "fz" => Some(DynamicElement::FZ),
        "n" => Some(DynamicElement::N),
        "pf" => Some(DynamicElement::PF),
        "sfzp" => Some(DynamicElement::SFZP),
        "other-dynamics" => {
            let text = reader.read_text("other-dynamics")?;
            return Ok(Some(DynamicElement::OtherDynamics(text)));
        }
        _ => None,
    };

    // Skip to end of element for standard dynamics
    if elem.is_some() {
        reader.skip_element(name)?;
    }

    Ok(elem)
}

/// Parse a dynamic element from an empty tag.
fn parse_dynamic_element_from_empty(name: &str) -> Option<crate::ir::direction::DynamicElement> {
    use crate::ir::direction::DynamicElement;

    match name {
        "p" => Some(DynamicElement::P),
        "pp" => Some(DynamicElement::PP),
        "ppp" => Some(DynamicElement::PPP),
        "pppp" => Some(DynamicElement::PPPP),
        "ppppp" => Some(DynamicElement::PPPPP),
        "pppppp" => Some(DynamicElement::PPPPPP),
        "f" => Some(DynamicElement::F),
        "ff" => Some(DynamicElement::FF),
        "fff" => Some(DynamicElement::FFF),
        "ffff" => Some(DynamicElement::FFFF),
        "fffff" => Some(DynamicElement::FFFFF),
        "ffffff" => Some(DynamicElement::FFFFFF),
        "mp" => Some(DynamicElement::MP),
        "mf" => Some(DynamicElement::MF),
        "sf" => Some(DynamicElement::SF),
        "sfp" => Some(DynamicElement::SFP),
        "sfpp" => Some(DynamicElement::SFPP),
        "fp" => Some(DynamicElement::FP),
        "rf" => Some(DynamicElement::RF),
        "rfz" => Some(DynamicElement::RFZ),
        "sfz" => Some(DynamicElement::SFZ),
        "sffz" => Some(DynamicElement::SFFZ),
        "fz" => Some(DynamicElement::FZ),
        "n" => Some(DynamicElement::N),
        "pf" => Some(DynamicElement::PF),
        "sfzp" => Some(DynamicElement::SFZP),
        _ => None,
    }
}

/// Parse a wedge element.
fn parse_wedge(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Wedge, ParseError> {
    let wedge = parse_wedge_from_empty(start, reader)?;
    reader.skip_element("wedge")?;
    Ok(wedge)
}

/// Parse a wedge element from an empty tag.
fn parse_wedge_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Wedge, ParseError> {
    use crate::ir::direction::Wedge;

    let type_str = reader.get_attr(start.attributes(), "type", "wedge")?;
    let r#type = values::parse_wedge_type(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let spread = reader.get_optional_attr_as::<f64>(start.attributes(), "spread")?;
    let niente = reader
        .get_optional_attr(start.attributes(), "niente")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Wedge {
        r#type,
        number,
        spread,
        niente,
        line_type,
        position,
        color,
    })
}

/// Parse a metronome element.
fn parse_metronome(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Metronome, ParseError> {
    use crate::ir::direction::{Metronome, MetronomeContent, PerMinute};
    use crate::ir::duration::NoteTypeValue;

    let parentheses = reader
        .get_optional_attr(start.attributes(), "parentheses")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    let mut beat_unit: Option<NoteTypeValue> = None;
    let mut beat_unit_dots = 0u32;
    let mut per_minute_value: Option<String> = None;
    let mut right_beat_unit: Option<NoteTypeValue> = None;
    let mut right_beat_unit_dots = 0u32;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "beat-unit" => {
                        let text = reader.read_text("beat-unit")?;
                        let value = values::parse_note_type_value(&text, reader.position())?;
                        if beat_unit.is_none() {
                            beat_unit = Some(value);
                        } else {
                            right_beat_unit = Some(value);
                        }
                    }
                    "beat-unit-dot" => {
                        if right_beat_unit.is_none() {
                            beat_unit_dots += 1;
                        } else {
                            right_beat_unit_dots += 1;
                        }
                        reader.skip_element("beat-unit-dot")?;
                    }
                    "per-minute" => {
                        per_minute_value = Some(reader.read_text("per-minute")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "beat-unit-dot" {
                    if right_beat_unit.is_none() {
                        beat_unit_dots += 1;
                    } else {
                        right_beat_unit_dots += 1;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in metronome",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    // Determine content type
    let content = if let Some(right_unit) = right_beat_unit {
        MetronomeContent::BeatEquation {
            left_unit: beat_unit.unwrap_or(NoteTypeValue::Quarter),
            left_dots: beat_unit_dots,
            right_unit,
            right_dots: right_beat_unit_dots,
        }
    } else if let Some(pm) = per_minute_value {
        MetronomeContent::PerMinute {
            beat_unit: beat_unit.unwrap_or(NoteTypeValue::Quarter),
            beat_unit_dots,
            per_minute: PerMinute {
                value: pm,
                font: Font::default(),
            },
        }
    } else {
        // Default to quarter = 120 if nothing specified
        MetronomeContent::PerMinute {
            beat_unit: NoteTypeValue::Quarter,
            beat_unit_dots: 0,
            per_minute: PerMinute {
                value: "120".to_string(),
                font: Font::default(),
            },
        }
    };

    Ok(Metronome {
        parentheses,
        content,
        print_style,
    })
}

/// Parse a words element.
fn parse_words(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Words, ParseError> {
    use crate::ir::direction::Words;

    let print_style = parse_print_style_attrs(start, reader)?;
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let lang = reader.get_optional_attr(start.attributes(), "xml:lang")?;

    let value = reader.read_text("words")?;

    Ok(Words {
        value,
        print_style,
        justify,
        lang,
    })
}

/// Parse a rehearsal element.
fn parse_rehearsal(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::common::FormattedText, ParseError> {
    use crate::ir::common::FormattedText;

    let print_style = parse_print_style_attrs(start, reader)?;
    let lang = reader.get_optional_attr(start.attributes(), "xml:lang")?;
    let value = reader.read_text("rehearsal")?;

    Ok(FormattedText {
        value,
        print_style,
        lang,
    })
}

/// Parse a pedal element.
fn parse_pedal(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Pedal, ParseError> {
    let pedal = parse_pedal_from_empty(start, reader)?;
    reader.skip_element("pedal")?;
    Ok(pedal)
}

/// Parse a pedal element from an empty tag.
fn parse_pedal_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Pedal, ParseError> {
    use crate::ir::direction::Pedal;

    let type_str = reader.get_attr(start.attributes(), "type", "pedal")?;
    let r#type = values::parse_pedal_type(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let line = reader
        .get_optional_attr(start.attributes(), "line")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let sign = reader
        .get_optional_attr(start.attributes(), "sign")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let abbreviated = reader
        .get_optional_attr(start.attributes(), "abbreviated")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    Ok(Pedal {
        r#type,
        number,
        line,
        sign,
        abbreviated,
        print_style,
    })
}

/// Parse an octave-shift element.
fn parse_octave_shift(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::OctaveShift, ParseError> {
    let shift = parse_octave_shift_from_empty(start, reader)?;
    reader.skip_element("octave-shift")?;
    Ok(shift)
}

/// Parse an octave-shift element from an empty tag.
fn parse_octave_shift_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::OctaveShift, ParseError> {
    use crate::ir::direction::OctaveShift;

    let type_str = reader.get_attr(start.attributes(), "type", "octave-shift")?;
    let r#type = values::parse_up_down_stop_continue(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let size = reader.get_optional_attr_as::<u8>(start.attributes(), "size")?;
    let position = parse_position_attrs(start, reader)?;

    Ok(OctaveShift {
        r#type,
        number,
        size,
        position,
    })
}

/// Parse an offset element.
fn parse_offset(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Offset, ParseError> {
    use crate::ir::direction::Offset;

    let sound = reader
        .get_optional_attr(start.attributes(), "sound")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let value = reader.read_text_as("offset")?;

    Ok(Offset { value, sound })
}

/// Parse an offset element from an empty tag.
fn parse_offset_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Offset, ParseError> {
    use crate::ir::direction::Offset;

    let sound = reader
        .get_optional_attr(start.attributes(), "sound")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Offset { value: 0, sound })
}

/// Parse a sound element.
fn parse_sound(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Sound, ParseError> {
    let sound = parse_sound_from_empty(start, reader)?;
    reader.skip_element("sound")?;
    Ok(sound)
}

/// Parse a sound element from an empty tag.
fn parse_sound_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Sound, ParseError> {
    use crate::ir::direction::Sound;

    let tempo = reader.get_optional_attr_as::<f64>(start.attributes(), "tempo")?;
    let dynamics = reader.get_optional_attr_as::<f64>(start.attributes(), "dynamics")?;
    let dacapo = reader
        .get_optional_attr(start.attributes(), "dacapo")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let segno = reader.get_optional_attr(start.attributes(), "segno")?;
    let dalsegno = reader.get_optional_attr(start.attributes(), "dalsegno")?;
    let coda = reader.get_optional_attr(start.attributes(), "coda")?;
    let tocoda = reader.get_optional_attr(start.attributes(), "tocoda")?;
    let divisions = reader.get_optional_attr_as::<i64>(start.attributes(), "divisions")?;
    let forward_repeat = reader
        .get_optional_attr(start.attributes(), "forward-repeat")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let fine = reader.get_optional_attr(start.attributes(), "fine")?;
    let time_only = reader.get_optional_attr(start.attributes(), "time-only")?;
    let pizzicato = reader
        .get_optional_attr(start.attributes(), "pizzicato")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Sound {
        tempo,
        dynamics,
        dacapo,
        segno,
        dalsegno,
        coda,
        tocoda,
        divisions,
        forward_repeat,
        fine,
        time_only,
        pizzicato,
    })
}

/// Parse a dashes element.
fn parse_dashes(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Dashes, ParseError> {
    let dashes = parse_dashes_from_empty(start, reader)?;
    reader.skip_element("dashes")?;
    Ok(dashes)
}

/// Parse a dashes element from an empty tag.
fn parse_dashes_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Dashes, ParseError> {
    use crate::ir::direction::Dashes;

    let type_str = reader.get_attr(start.attributes(), "type", "dashes")?;
    let r#type = values::parse_start_stop_continue(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Dashes {
        r#type,
        number,
        position,
        color,
    })
}

/// Parse a bracket element.
fn parse_bracket(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::direction::Bracket, ParseError> {
    let bracket = parse_bracket_from_empty(start, reader)?;
    reader.skip_element("bracket")?;
    Ok(bracket)
}

/// Parse a bracket element from an empty tag.
fn parse_bracket_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::direction::Bracket, ParseError> {
    use crate::ir::direction::{Bracket, LineEnd};

    let type_str = reader.get_attr(start.attributes(), "type", "bracket")?;
    let r#type = values::parse_start_stop_continue(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let line_end_str = reader.get_attr(start.attributes(), "line-end", "bracket")?;
    let line_end = match line_end_str.as_str() {
        "up" => LineEnd::Up,
        "down" => LineEnd::Down,
        "both" => LineEnd::Both,
        "arrow" => LineEnd::Arrow,
        "none" => LineEnd::None,
        _ => {
            return Err(ParseError::invalid_value(
                "line-end",
                &line_end_str,
                reader.position(),
            ));
        }
    };
    let end_length = reader.get_optional_attr_as::<f64>(start.attributes(), "end-length")?;
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Bracket {
        r#type,
        number,
        line_end,
        end_length,
        line_type,
        position,
        color,
    })
}

// === Notations Parsing ===

/// Parse a notations element.
///
/// Notations contain musical notation elements like slurs, ties, tuplets,
/// articulations, ornaments, technical markings, fermatas, and arpeggios.
fn parse_notations(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Notations, ParseError> {
    use crate::ir::notation::{NotationContent, Notations};

    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut content: Vec<NotationContent> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "tied" => {
                        content.push(NotationContent::Tied(parse_tied(reader, &e)?));
                    }
                    "slur" => {
                        content.push(NotationContent::Slur(parse_slur(reader, &e)?));
                    }
                    "tuplet" => {
                        content.push(NotationContent::Tuplet(Box::new(parse_tuplet(reader, &e)?)));
                    }
                    "ornaments" => {
                        content.push(NotationContent::Ornaments(Box::new(parse_ornaments(
                            reader,
                        )?)));
                    }
                    "technical" => {
                        content.push(NotationContent::Technical(Box::new(parse_technical(
                            reader,
                        )?)));
                    }
                    "articulations" => {
                        content.push(NotationContent::Articulations(Box::new(
                            parse_articulations(reader)?,
                        )));
                    }
                    "dynamics" => {
                        content.push(NotationContent::Dynamics(Box::new(parse_dynamics(
                            reader, &e,
                        )?)));
                    }
                    "fermata" => {
                        content.push(NotationContent::Fermata(parse_fermata_notation(
                            reader, &e,
                        )?));
                    }
                    "arpeggiate" => {
                        content.push(NotationContent::Arpeggiate(parse_arpeggiate(reader, &e)?));
                    }
                    "non-arpeggiate" => {
                        content.push(NotationContent::NonArpeggiate(parse_non_arpeggiate(
                            reader, &e,
                        )?));
                    }
                    "glissando" => {
                        content.push(NotationContent::Glissando(parse_glissando(reader, &e)?));
                    }
                    "slide" => {
                        content.push(NotationContent::Slide(parse_slide(reader, &e)?));
                    }
                    "accidental-mark" => {
                        content.push(NotationContent::AccidentalMark(parse_accidental_mark(
                            reader, &e,
                        )?));
                    }
                    "footnote" | "level" | "other-notation" => {
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
                    "tied" => {
                        content.push(NotationContent::Tied(parse_tied_from_empty(&e, reader)?));
                    }
                    "slur" => {
                        content.push(NotationContent::Slur(parse_slur_from_empty(&e, reader)?));
                    }
                    "tuplet" => {
                        content.push(NotationContent::Tuplet(Box::new(parse_tuplet_from_empty(
                            &e, reader,
                        )?)));
                    }
                    "fermata" => {
                        content.push(NotationContent::Fermata(parse_fermata_notation_from_empty(
                            &e, reader,
                        )?));
                    }
                    "arpeggiate" => {
                        content.push(NotationContent::Arpeggiate(parse_arpeggiate_from_empty(
                            &e, reader,
                        )?));
                    }
                    "non-arpeggiate" => {
                        content.push(NotationContent::NonArpeggiate(
                            parse_non_arpeggiate_from_empty(&e, reader)?,
                        ));
                    }
                    "accidental-mark" => {
                        content.push(NotationContent::AccidentalMark(
                            parse_accidental_mark_from_empty(&e, reader)?,
                        ));
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in notations",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Notations {
        print_object,
        content,
        editorial: crate::ir::common::Editorial::default(),
    })
}

/// Parse a tied element (visual notation, not playback).
fn parse_tied(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Tied, ParseError> {
    let tied = parse_tied_from_empty(start, reader)?;
    reader.skip_element("tied")?;
    Ok(tied)
}

/// Parse a tied element from an empty tag.
fn parse_tied_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Tied, ParseError> {
    use crate::ir::notation::Tied;

    let type_str = reader.get_attr(start.attributes(), "type", "tied")?;
    let r#type = values::parse_start_stop_continue(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let orientation = reader
        .get_optional_attr(start.attributes(), "orientation")?
        .map(|s| values::parse_over_under(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Tied {
        r#type,
        number,
        line_type,
        placement,
        orientation,
        position,
        color,
    })
}

/// Parse a slur element.
fn parse_slur(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Slur, ParseError> {
    let slur = parse_slur_from_empty(start, reader)?;
    reader.skip_element("slur")?;
    Ok(slur)
}

/// Parse a slur element from an empty tag.
fn parse_slur_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Slur, ParseError> {
    use crate::ir::notation::Slur;

    let type_str = reader.get_attr(start.attributes(), "type", "slur")?;
    let r#type = values::parse_start_stop_continue(&type_str, reader.position())?;
    let number = reader
        .get_optional_attr_as::<u8>(start.attributes(), "number")?
        .unwrap_or(1);
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let orientation = reader
        .get_optional_attr(start.attributes(), "orientation")?
        .map(|s| values::parse_over_under(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Slur {
        r#type,
        number,
        line_type,
        placement,
        orientation,
        position,
        color,
    })
}

/// Parse a tuplet element.
fn parse_tuplet(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Tuplet, ParseError> {
    use crate::ir::notation::{Tuplet, TupletPortion};

    let type_str = reader.get_attr(start.attributes(), "type", "tuplet")?;
    let r#type = values::parse_start_stop(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let bracket = reader
        .get_optional_attr(start.attributes(), "bracket")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let show_number = reader
        .get_optional_attr(start.attributes(), "show-number")?
        .map(|s| values::parse_show_tuplet(&s, reader.position()))
        .transpose()?;
    let show_type = reader
        .get_optional_attr(start.attributes(), "show-type")?
        .map(|s| values::parse_show_tuplet(&s, reader.position()))
        .transpose()?;
    let line_shape = reader
        .get_optional_attr(start.attributes(), "line-shape")?
        .map(|s| values::parse_line_shape(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    let mut tuplet_actual: Option<TupletPortion> = None;
    let mut tuplet_normal: Option<TupletPortion> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "tuplet-actual" => {
                        tuplet_actual = Some(parse_tuplet_portion(reader)?);
                    }
                    "tuplet-normal" => {
                        tuplet_normal = Some(parse_tuplet_portion(reader)?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in tuplet",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Tuplet {
        r#type,
        number,
        bracket,
        show_number,
        show_type,
        line_shape,
        placement,
        position,
        tuplet_actual,
        tuplet_normal,
    })
}

/// Parse a tuplet element from an empty tag.
fn parse_tuplet_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Tuplet, ParseError> {
    use crate::ir::notation::Tuplet;

    let type_str = reader.get_attr(start.attributes(), "type", "tuplet")?;
    let r#type = values::parse_start_stop(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let bracket = reader
        .get_optional_attr(start.attributes(), "bracket")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let show_number = reader
        .get_optional_attr(start.attributes(), "show-number")?
        .map(|s| values::parse_show_tuplet(&s, reader.position()))
        .transpose()?;
    let show_type = reader
        .get_optional_attr(start.attributes(), "show-type")?
        .map(|s| values::parse_show_tuplet(&s, reader.position()))
        .transpose()?;
    let line_shape = reader
        .get_optional_attr(start.attributes(), "line-shape")?
        .map(|s| values::parse_line_shape(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(Tuplet {
        r#type,
        number,
        bracket,
        show_number,
        show_type,
        line_shape,
        placement,
        position,
        tuplet_actual: None,
        tuplet_normal: None,
    })
}

/// Parse a tuplet-actual or tuplet-normal portion.
fn parse_tuplet_portion(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::notation::TupletPortion, ParseError> {
    use crate::ir::notation::TupletPortion;

    use crate::ir::notation::{TupletDot, TupletNumber, TupletType};

    let mut tuplet_number: Option<TupletNumber> = None;
    let mut tuplet_type: Option<TupletType> = None;
    let mut tuplet_dots: Vec<TupletDot> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "tuplet-number" => {
                        let value: u32 = reader.read_text_as("tuplet-number")?;
                        tuplet_number = Some(TupletNumber {
                            value,
                            font: Font::default(),
                            color: None,
                        });
                    }
                    "tuplet-type" => {
                        let text = reader.read_text("tuplet-type")?;
                        let value = values::parse_note_type_value(&text, reader.position())?;
                        tuplet_type = Some(TupletType {
                            value,
                            font: Font::default(),
                            color: None,
                        });
                    }
                    "tuplet-dot" => {
                        tuplet_dots.push(TupletDot {
                            font: Font::default(),
                            color: None,
                        });
                        reader.skip_element("tuplet-dot")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "tuplet-dot" {
                    tuplet_dots.push(TupletDot {
                        font: Font::default(),
                        color: None,
                    });
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in tuplet portion",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(TupletPortion {
        tuplet_number,
        tuplet_type,
        tuplet_dots,
    })
}

/// Parse an ornaments element.
///
/// In MusicXML, ornaments can be followed by accidental-mark elements that apply to them.
/// For simplicity, we collect all ornaments and accidental marks, and associate accidental
/// marks with the preceding ornament (if any), or create standalone ornament entries.
fn parse_ornaments(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::notation::Ornaments, ParseError> {
    use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

    let mut content: Vec<OrnamentWithAccidentals> = Vec::new();
    let mut pending_accidental_marks: Vec<crate::ir::notation::AccidentalMark> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "trill-mark" => {
                        // If we had pending accidental marks and a previous ornament, attach them
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::TrillMark(parse_empty_trill_sound(
                                reader, &e,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Turn(parse_turn(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "delayed-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::DelayedTurn(parse_turn(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "inverted-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::InvertedTurn(parse_turn(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "delayed-inverted-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::DelayedInvertedTurn(parse_turn(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "vertical-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::VerticalTurn(parse_empty_trill_sound(
                                reader, &e,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "inverted-vertical-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::InvertedVerticalTurn(
                                parse_empty_trill_sound(reader, &e)?,
                            ),
                            accidental_marks: vec![],
                        });
                    }
                    "mordent" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Mordent(parse_mordent(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "inverted-mordent" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::InvertedMordent(parse_mordent(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "shake" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Shake(parse_empty_trill_sound(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "wavy-line" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::WavyLine(parse_wavy_line(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "schleifer" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Schleifer(
                                crate::ir::common::EmptyPlacement::default(),
                            ),
                            accidental_marks: vec![],
                        });
                        reader.skip_element("schleifer")?;
                    }
                    "tremolo" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Tremolo(parse_tremolo(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "haydn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Haydn(parse_empty_trill_sound(reader, &e)?),
                            accidental_marks: vec![],
                        });
                    }
                    "accidental-mark" => {
                        pending_accidental_marks.push(parse_accidental_mark(reader, &e)?);
                    }
                    "other-ornament" => {
                        reader.skip_element("other-ornament")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "trill-mark" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::TrillMark(
                                parse_empty_trill_sound_from_empty(&e, reader)?,
                            ),
                            accidental_marks: vec![],
                        });
                    }
                    "turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Turn(parse_turn_from_empty(&e, reader)?),
                            accidental_marks: vec![],
                        });
                    }
                    "delayed-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::DelayedTurn(parse_turn_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "inverted-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::InvertedTurn(parse_turn_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "delayed-inverted-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::DelayedInvertedTurn(parse_turn_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "vertical-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::VerticalTurn(
                                parse_empty_trill_sound_from_empty(&e, reader)?,
                            ),
                            accidental_marks: vec![],
                        });
                    }
                    "inverted-vertical-turn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::InvertedVerticalTurn(
                                parse_empty_trill_sound_from_empty(&e, reader)?,
                            ),
                            accidental_marks: vec![],
                        });
                    }
                    "mordent" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Mordent(parse_mordent_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "inverted-mordent" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::InvertedMordent(parse_mordent_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "shake" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Shake(parse_empty_trill_sound_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "wavy-line" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::WavyLine(parse_wavy_line_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "schleifer" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Schleifer(parse_empty_placement_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "tremolo" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Tremolo(parse_tremolo_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "haydn" => {
                        if !pending_accidental_marks.is_empty() && !content.is_empty() {
                            if let Some(last) = content.last_mut() {
                                last.accidental_marks.append(&mut pending_accidental_marks);
                            }
                        }
                        content.push(OrnamentWithAccidentals {
                            ornament: OrnamentElement::Haydn(parse_empty_trill_sound_from_empty(
                                &e, reader,
                            )?),
                            accidental_marks: vec![],
                        });
                    }
                    "accidental-mark" => {
                        pending_accidental_marks
                            .push(parse_accidental_mark_from_empty(&e, reader)?);
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in ornaments",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    // Attach any remaining accidental marks to the last ornament
    if !pending_accidental_marks.is_empty() && !content.is_empty() {
        if let Some(last) = content.last_mut() {
            last.accidental_marks.append(&mut pending_accidental_marks);
        }
    }

    Ok(Ornaments { content })
}

/// Parse a technical element container.
fn parse_technical(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::notation::Technical, ParseError> {
    use crate::ir::notation::{Technical, TechnicalElement};

    let mut content: Vec<TechnicalElement> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "up-bow" => {
                        content.push(TechnicalElement::UpBow(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "down-bow" => {
                        content.push(TechnicalElement::DownBow(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "harmonic" => {
                        content.push(TechnicalElement::Harmonic(parse_harmonic(reader, &e)?));
                    }
                    "open-string" => {
                        content.push(TechnicalElement::OpenString(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "thumb-position" => {
                        content.push(TechnicalElement::ThumbPosition(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "fingering" => {
                        content.push(TechnicalElement::Fingering(parse_fingering(reader, &e)?));
                    }
                    "pluck" => {
                        content.push(TechnicalElement::Pluck(parse_pluck(reader, &e)?));
                    }
                    "double-tongue" => {
                        content.push(TechnicalElement::DoubleTongue(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "triple-tongue" => {
                        content.push(TechnicalElement::TripleTongue(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "stopped" => {
                        content.push(TechnicalElement::Stopped(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "snap-pizzicato" => {
                        content.push(TechnicalElement::SnapPizzicato(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "fret" => {
                        content.push(TechnicalElement::Fret(parse_fret(reader, &e)?));
                    }
                    "string" => {
                        content.push(TechnicalElement::String(parse_string_number(reader, &e)?));
                    }
                    "hammer-on" => {
                        content.push(TechnicalElement::HammerOn(parse_hammer_pull(reader, &e)?));
                    }
                    "pull-off" => {
                        content.push(TechnicalElement::PullOff(parse_hammer_pull(reader, &e)?));
                    }
                    "tap" => {
                        content.push(TechnicalElement::Tap(parse_tap(reader, &e)?));
                    }
                    "heel" => {
                        content.push(TechnicalElement::Heel(parse_heel_toe(reader, &e)?));
                    }
                    "toe" => {
                        content.push(TechnicalElement::Toe(parse_heel_toe(reader, &e)?));
                    }
                    "fingernails" => {
                        content.push(TechnicalElement::Fingernails(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "bend" => {
                        content.push(TechnicalElement::Bend(parse_bend(reader, &e)?));
                    }
                    "hole" => {
                        reader.skip_element("hole")?;
                    }
                    "arrow" => {
                        reader.skip_element("arrow")?;
                    }
                    "handbell" => {
                        reader.skip_element("handbell")?;
                    }
                    "brass-bend" => {
                        content.push(TechnicalElement::BrassBend(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "flip" => {
                        content.push(TechnicalElement::Flip(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "smear" => {
                        content.push(TechnicalElement::Smear(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "open" => {
                        content.push(TechnicalElement::Open(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "half-muted" => {
                        content.push(TechnicalElement::HalfMuted(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "golpe" => {
                        content.push(TechnicalElement::Golpe(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "other-technical" => {
                        reader.skip_element("other-technical")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "up-bow" => {
                        content.push(TechnicalElement::UpBow(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    "down-bow" => {
                        content.push(TechnicalElement::DownBow(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    "harmonic" => {
                        content.push(TechnicalElement::Harmonic(parse_harmonic_from_empty(
                            &e, reader,
                        )?));
                    }
                    "open-string" => {
                        content.push(TechnicalElement::OpenString(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "thumb-position" => {
                        content.push(TechnicalElement::ThumbPosition(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "double-tongue" => {
                        content.push(TechnicalElement::DoubleTongue(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "triple-tongue" => {
                        content.push(TechnicalElement::TripleTongue(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "stopped" => {
                        content.push(TechnicalElement::Stopped(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    "snap-pizzicato" => {
                        content.push(TechnicalElement::SnapPizzicato(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "fingernails" => {
                        content.push(TechnicalElement::Fingernails(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "brass-bend" => {
                        content.push(TechnicalElement::BrassBend(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "flip" => {
                        content.push(TechnicalElement::Flip(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    "smear" => {
                        content.push(TechnicalElement::Smear(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    "open" => {
                        content.push(TechnicalElement::Open(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    "half-muted" => {
                        content.push(TechnicalElement::HalfMuted(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "golpe" => {
                        content.push(TechnicalElement::Golpe(parse_empty_placement_from_empty(
                            &e, reader,
                        )?));
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in technical",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Technical { content })
}

/// Parse an articulations element container.
fn parse_articulations(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::notation::Articulations, ParseError> {
    use crate::ir::notation::{ArticulationElement, Articulations};

    let mut content: Vec<ArticulationElement> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "accent" => {
                        content.push(ArticulationElement::Accent(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "strong-accent" => {
                        content.push(ArticulationElement::StrongAccent(parse_strong_accent(
                            reader, &e,
                        )?));
                    }
                    "staccato" => {
                        content.push(ArticulationElement::Staccato(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "tenuto" => {
                        content.push(ArticulationElement::Tenuto(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "detached-legato" => {
                        content.push(ArticulationElement::DetachedLegato(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "staccatissimo" => {
                        content.push(ArticulationElement::Staccatissimo(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "spiccato" => {
                        content.push(ArticulationElement::Spiccato(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "scoop" => {
                        content.push(ArticulationElement::Scoop(parse_empty_line(reader, &e)?));
                    }
                    "plop" => {
                        content.push(ArticulationElement::Plop(parse_empty_line(reader, &e)?));
                    }
                    "doit" => {
                        content.push(ArticulationElement::Doit(parse_empty_line(reader, &e)?));
                    }
                    "falloff" => {
                        content.push(ArticulationElement::Falloff(parse_empty_line(reader, &e)?));
                    }
                    "breath-mark" => {
                        content.push(ArticulationElement::BreathMark(parse_breath_mark(
                            reader, &e,
                        )?));
                    }
                    "caesura" => {
                        content.push(ArticulationElement::Caesura(parse_caesura(reader, &e)?));
                    }
                    "stress" => {
                        content.push(ArticulationElement::Stress(parse_empty_placement_element(
                            reader, &e,
                        )?));
                    }
                    "unstress" => {
                        content.push(ArticulationElement::Unstress(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "soft-accent" => {
                        content.push(ArticulationElement::SoftAccent(
                            parse_empty_placement_element(reader, &e)?,
                        ));
                    }
                    "other-articulation" => {
                        reader.skip_element("other-articulation")?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "accent" => {
                        content.push(ArticulationElement::Accent(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "strong-accent" => {
                        content.push(ArticulationElement::StrongAccent(
                            parse_strong_accent_from_empty(&e, reader)?,
                        ));
                    }
                    "staccato" => {
                        content.push(ArticulationElement::Staccato(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "tenuto" => {
                        content.push(ArticulationElement::Tenuto(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "detached-legato" => {
                        content.push(ArticulationElement::DetachedLegato(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "staccatissimo" => {
                        content.push(ArticulationElement::Staccatissimo(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "spiccato" => {
                        content.push(ArticulationElement::Spiccato(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "scoop" => {
                        content.push(ArticulationElement::Scoop(parse_empty_line_from_empty(
                            &e, reader,
                        )?));
                    }
                    "plop" => {
                        content.push(ArticulationElement::Plop(parse_empty_line_from_empty(
                            &e, reader,
                        )?));
                    }
                    "doit" => {
                        content.push(ArticulationElement::Doit(parse_empty_line_from_empty(
                            &e, reader,
                        )?));
                    }
                    "falloff" => {
                        content.push(ArticulationElement::Falloff(parse_empty_line_from_empty(
                            &e, reader,
                        )?));
                    }
                    "breath-mark" => {
                        content.push(ArticulationElement::BreathMark(
                            parse_breath_mark_from_empty(&e, reader)?,
                        ));
                    }
                    "caesura" => {
                        content.push(ArticulationElement::Caesura(parse_caesura_from_empty(
                            &e, reader,
                        )?));
                    }
                    "stress" => {
                        content.push(ArticulationElement::Stress(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "unstress" => {
                        content.push(ArticulationElement::Unstress(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    "soft-accent" => {
                        content.push(ArticulationElement::SoftAccent(
                            parse_empty_placement_from_empty(&e, reader)?,
                        ));
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in articulations",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Articulations { content })
}

/// Parse a fermata element in notation context.
fn parse_fermata_notation(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Fermata, ParseError> {
    use crate::ir::notation::Fermata;

    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_upright_inverted(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    let shape_text = reader.read_optional_text("fermata")?;
    let shape = shape_text
        .map(|s| values::parse_fermata_shape(&s, reader.position()))
        .transpose()?;

    Ok(Fermata {
        shape,
        r#type,
        print_style,
    })
}

/// Parse a fermata element from an empty tag.
fn parse_fermata_notation_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Fermata, ParseError> {
    use crate::ir::notation::Fermata;

    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_upright_inverted(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    Ok(Fermata {
        shape: None,
        r#type,
        print_style,
    })
}

/// Parse an arpeggiate element.
fn parse_arpeggiate(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Arpeggiate, ParseError> {
    let arp = parse_arpeggiate_from_empty(start, reader)?;
    reader.skip_element("arpeggiate")?;
    Ok(arp)
}

/// Parse an arpeggiate element from an empty tag.
fn parse_arpeggiate_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Arpeggiate, ParseError> {
    use crate::ir::notation::Arpeggiate;

    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let direction = reader
        .get_optional_attr(start.attributes(), "direction")?
        .map(|s| values::parse_up_down(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Arpeggiate {
        number,
        direction,
        position,
        color,
    })
}

/// Parse a non-arpeggiate element.
fn parse_non_arpeggiate(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::NonArpeggiate, ParseError> {
    let na = parse_non_arpeggiate_from_empty(start, reader)?;
    reader.skip_element("non-arpeggiate")?;
    Ok(na)
}

/// Parse a non-arpeggiate element from an empty tag.
fn parse_non_arpeggiate_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::NonArpeggiate, ParseError> {
    use crate::ir::notation::NonArpeggiate;

    let type_str = reader.get_attr(start.attributes(), "type", "non-arpeggiate")?;
    let r#type = values::parse_top_bottom(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(NonArpeggiate {
        r#type,
        number,
        position,
        color,
    })
}

/// Parse a glissando element.
fn parse_glissando(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Glissando, ParseError> {
    use crate::ir::notation::Glissando;

    let type_str = reader.get_attr(start.attributes(), "type", "glissando")?;
    let r#type = values::parse_start_stop(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    let text = reader.read_optional_text("glissando")?;

    Ok(Glissando {
        r#type,
        number,
        line_type,
        text,
        position,
    })
}

/// Parse a slide element.
fn parse_slide(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Slide, ParseError> {
    use crate::ir::notation::Slide;

    let type_str = reader.get_attr(start.attributes(), "type", "slide")?;
    let r#type = values::parse_start_stop(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    let text = reader.read_optional_text("slide")?;

    Ok(Slide {
        r#type,
        number,
        line_type,
        text,
        position,
    })
}

/// Parse an accidental-mark element.
fn parse_accidental_mark(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::AccidentalMark, ParseError> {
    use crate::ir::notation::AccidentalMark;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    let text = reader.read_text("accidental-mark")?;
    let value = values::parse_accidental_value(&text, reader.position())?;

    Ok(AccidentalMark {
        value,
        placement,
        print_style,
    })
}

/// Parse an accidental-mark element from an empty tag.
fn parse_accidental_mark_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::AccidentalMark, ParseError> {
    use crate::ir::common::AccidentalValue;
    use crate::ir::notation::AccidentalMark;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    // Default to natural for empty accidental-mark (unusual but handle gracefully)
    Ok(AccidentalMark {
        value: AccidentalValue::Natural,
        placement,
        print_style,
    })
}

// === Helper functions for notation parsing ===

/// Parse an empty placement element.
fn parse_empty_placement_element(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::common::EmptyPlacement, ParseError> {
    let ep = parse_empty_placement_from_empty(start, reader)?;
    reader.skip_element(&element_name(start))?;
    Ok(ep)
}

/// Parse an empty placement from an empty element.
fn parse_empty_placement_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::common::EmptyPlacement, ParseError> {
    use crate::ir::common::EmptyPlacement;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(EmptyPlacement {
        placement,
        position,
    })
}

/// Parse a turn element.
fn parse_turn(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Turn, ParseError> {
    let turn = parse_turn_from_empty(start, reader)?;
    reader.skip_element(&element_name(start))?;
    Ok(turn)
}

/// Parse a turn from an empty element.
fn parse_turn_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Turn, ParseError> {
    use crate::ir::notation::Turn;

    let slash = reader
        .get_optional_attr(start.attributes(), "slash")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let start_note = reader
        .get_optional_attr(start.attributes(), "start-note")?
        .map(|s| values::parse_start_note(&s, reader.position()))
        .transpose()?;
    let trill_step = reader
        .get_optional_attr(start.attributes(), "trill-step")?
        .map(|s| values::parse_trill_step(&s, reader.position()))
        .transpose()?;
    let two_note_turn = reader
        .get_optional_attr(start.attributes(), "two-note-turn")?
        .map(|s| values::parse_two_note_turn(&s, reader.position()))
        .transpose()?;
    let accelerate = reader
        .get_optional_attr(start.attributes(), "accelerate")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let beats = reader.get_optional_attr_as::<f64>(start.attributes(), "beats")?;
    let second_beat = reader.get_optional_attr_as::<f64>(start.attributes(), "second-beat")?;
    let last_beat = reader.get_optional_attr_as::<f64>(start.attributes(), "last-beat")?;

    Ok(Turn {
        slash,
        placement,
        position,
        start_note,
        trill_step,
        two_note_turn,
        accelerate,
        beats,
        second_beat,
        last_beat,
    })
}

/// Parse an empty trill sound element.
fn parse_empty_trill_sound(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::EmptyTrillSound, ParseError> {
    let ets = parse_empty_trill_sound_from_empty(start, reader)?;
    reader.skip_element(&element_name(start))?;
    Ok(ets)
}

/// Parse empty trill sound from an empty element.
fn parse_empty_trill_sound_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::EmptyTrillSound, ParseError> {
    use crate::ir::notation::EmptyTrillSound;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let start_note = reader
        .get_optional_attr(start.attributes(), "start-note")?
        .map(|s| values::parse_start_note(&s, reader.position()))
        .transpose()?;
    let trill_step = reader
        .get_optional_attr(start.attributes(), "trill-step")?
        .map(|s| values::parse_trill_step(&s, reader.position()))
        .transpose()?;
    let two_note_turn = reader
        .get_optional_attr(start.attributes(), "two-note-turn")?
        .map(|s| values::parse_two_note_turn(&s, reader.position()))
        .transpose()?;
    let accelerate = reader
        .get_optional_attr(start.attributes(), "accelerate")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let beats = reader.get_optional_attr_as::<f64>(start.attributes(), "beats")?;
    let second_beat = reader.get_optional_attr_as::<f64>(start.attributes(), "second-beat")?;
    let last_beat = reader.get_optional_attr_as::<f64>(start.attributes(), "last-beat")?;

    Ok(EmptyTrillSound {
        placement,
        position,
        start_note,
        trill_step,
        two_note_turn,
        accelerate,
        beats,
        second_beat,
        last_beat,
    })
}

/// Parse a mordent element.
fn parse_mordent(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Mordent, ParseError> {
    let m = parse_mordent_from_empty(start, reader)?;
    reader.skip_element(&element_name(start))?;
    Ok(m)
}

/// Parse a mordent from an empty element.
fn parse_mordent_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Mordent, ParseError> {
    use crate::ir::notation::Mordent;

    let long = reader
        .get_optional_attr(start.attributes(), "long")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let approach = reader
        .get_optional_attr(start.attributes(), "approach")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let departure = reader
        .get_optional_attr(start.attributes(), "departure")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(Mordent {
        long,
        approach,
        departure,
        placement,
        position,
        start_note: None,
        trill_step: None,
        two_note_turn: None,
        accelerate: None,
        beats: None,
        second_beat: None,
        last_beat: None,
    })
}

/// Parse a tremolo element.
fn parse_tremolo(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Tremolo, ParseError> {
    use crate::ir::notation::Tremolo;

    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_tremolo_type(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    let value_text = reader.read_optional_text("tremolo")?;
    let value = value_text.and_then(|s| s.parse().ok()).unwrap_or(3);

    Ok(Tremolo {
        value,
        r#type,
        placement,
        position,
    })
}

/// Parse a tremolo from an empty element.
fn parse_tremolo_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Tremolo, ParseError> {
    use crate::ir::notation::Tremolo;

    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_tremolo_type(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(Tremolo {
        value: 3,
        r#type,
        placement,
        position,
    })
}

/// Parse a strong-accent element.
fn parse_strong_accent(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::StrongAccent, ParseError> {
    let sa = parse_strong_accent_from_empty(start, reader)?;
    reader.skip_element("strong-accent")?;
    Ok(sa)
}

/// Parse a strong-accent from an empty element.
fn parse_strong_accent_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::StrongAccent, ParseError> {
    use crate::ir::notation::StrongAccent;

    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_up_down(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(StrongAccent {
        r#type,
        placement,
        position,
    })
}

/// Parse an empty-line element (for scoop, plop, doit, falloff).
fn parse_empty_line(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::EmptyLine, ParseError> {
    let el = parse_empty_line_from_empty(start, reader)?;
    reader.skip_element(&element_name(start))?;
    Ok(el)
}

/// Parse an empty-line from an empty element.
fn parse_empty_line_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::EmptyLine, ParseError> {
    use crate::ir::notation::EmptyLine;

    let line_shape = reader
        .get_optional_attr(start.attributes(), "line-shape")?
        .map(|s| values::parse_line_shape(&s, reader.position()))
        .transpose()?;
    let line_type = reader
        .get_optional_attr(start.attributes(), "line-type")?
        .map(|s| values::parse_line_type(&s, reader.position()))
        .transpose()?;
    let line_length = reader
        .get_optional_attr(start.attributes(), "line-length")?
        .map(|s| values::parse_line_length(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(EmptyLine {
        line_shape,
        line_type,
        line_length,
        placement,
        position,
    })
}

/// Parse a breath-mark element.
fn parse_breath_mark(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::BreathMark, ParseError> {
    use crate::ir::notation::{BreathMark, BreathMarkValue};

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    let value_text = reader.read_optional_text("breath-mark")?;
    let value = value_text
        .map(|s| values::parse_breath_mark_value(&s, reader.position()))
        .transpose()?
        .unwrap_or(BreathMarkValue::Empty);

    Ok(BreathMark {
        value,
        placement,
        position,
    })
}

/// Parse a breath-mark from an empty element.
fn parse_breath_mark_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::BreathMark, ParseError> {
    use crate::ir::notation::{BreathMark, BreathMarkValue};

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(BreathMark {
        value: BreathMarkValue::Empty,
        placement,
        position,
    })
}

/// Parse a caesura element.
fn parse_caesura(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Caesura, ParseError> {
    use crate::ir::notation::{Caesura, CaesuraValue};

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    let value_text = reader.read_optional_text("caesura")?;
    let value = value_text
        .map(|s| values::parse_caesura_value(&s, reader.position()))
        .transpose()?
        .unwrap_or(CaesuraValue::Normal);

    Ok(Caesura {
        value,
        placement,
        position,
    })
}

/// Parse a caesura from an empty element.
fn parse_caesura_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Caesura, ParseError> {
    use crate::ir::notation::{Caesura, CaesuraValue};

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;

    Ok(Caesura {
        value: CaesuraValue::Normal,
        placement,
        position,
    })
}

/// Parse a fingering element.
fn parse_fingering(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Fingering, ParseError> {
    use crate::ir::notation::Fingering;

    let substitution = reader
        .get_optional_attr(start.attributes(), "substitution")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let alternate = reader
        .get_optional_attr(start.attributes(), "alternate")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    let value = reader.read_text("fingering")?;

    Ok(Fingering {
        value,
        substitution,
        alternate,
        placement,
        print_style,
    })
}

/// Parse a fret element.
fn parse_fret(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Fret, ParseError> {
    use crate::ir::notation::Fret;

    let color = reader.get_optional_attr(start.attributes(), "color")?;
    let value = reader.read_text_as("fret")?;

    Ok(Fret {
        value,
        font: Font::default(),
        color,
    })
}

/// Parse a string element (as number).
fn parse_string_number(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::StringNumber, ParseError> {
    use crate::ir::notation::StringNumber;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    let value = reader.read_text_as("string")?;

    Ok(StringNumber {
        value,
        placement,
        print_style,
    })
}

/// Parse a hammer-on or pull-off element.
fn parse_hammer_pull(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::HammerPull, ParseError> {
    use crate::ir::notation::HammerPull;

    let type_str = reader.get_attr(start.attributes(), "type", "hammer-on/pull-off")?;
    let r#type = values::parse_start_stop(&type_str, reader.position())?;
    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    let value = reader
        .read_optional_text(&element_name(start))?
        .unwrap_or_default();

    Ok(HammerPull {
        value,
        r#type,
        number,
        placement,
    })
}

/// Parse a heel or toe element.
fn parse_heel_toe(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::HeelToe, ParseError> {
    use crate::ir::notation::HeelToe;

    let substitution = reader
        .get_optional_attr(start.attributes(), "substitution")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    reader.skip_element(&element_name(start))?;

    Ok(HeelToe {
        substitution,
        placement,
    })
}

/// Parse a pluck element.
fn parse_pluck(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Pluck, ParseError> {
    use crate::ir::notation::Pluck;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;

    let value = reader.read_text(&element_name(start))?;

    Ok(Pluck { value, placement })
}

/// Parse a tap element.
fn parse_tap(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Tap, ParseError> {
    use crate::ir::notation::Tap;

    let hand = reader
        .get_optional_attr(start.attributes(), "hand")?
        .map(|s| values::parse_tap_hand(&s, reader.position()))
        .transpose()?;

    let value = reader.read_text(&element_name(start))?;

    Ok(Tap { value, hand })
}

/// Parse a harmonic element.
fn parse_harmonic(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Harmonic, ParseError> {
    use crate::ir::notation::Harmonic;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let mut natural = false;
    let mut artificial = false;
    let mut base_pitch = false;
    let mut touching_pitch = false;
    let mut sounding_pitch = false;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "natural" => natural = true,
                    "artificial" => artificial = true,
                    "base-pitch" => base_pitch = true,
                    "touching-pitch" => touching_pitch = true,
                    "sounding-pitch" => sounding_pitch = true,
                    _ => {}
                }
                reader.skip_element(&name)?;
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "natural" => natural = true,
                    "artificial" => artificial = true,
                    "base-pitch" => base_pitch = true,
                    "touching-pitch" => touching_pitch = true,
                    "sounding-pitch" => sounding_pitch = true,
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in harmonic",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Harmonic {
        natural,
        artificial,
        base_pitch,
        touching_pitch,
        sounding_pitch,
        placement,
        print_object,
    })
}

/// Parse a harmonic from an empty element.
fn parse_harmonic_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::notation::Harmonic, ParseError> {
    use crate::ir::notation::Harmonic;

    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Harmonic {
        natural: false,
        artificial: false,
        base_pitch: false,
        touching_pitch: false,
        sounding_pitch: false,
        placement,
        print_object,
    })
}

/// Parse a bend element.
fn parse_bend(
    reader: &mut XmlReader<'_>,
    _start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::notation::Bend, ParseError> {
    use crate::ir::notation::{Bend, BendRelease};

    let mut bend_alter: Option<f64> = None;
    let mut pre_bend = false;
    let mut release: Option<BendRelease> = None;
    let mut with_bar: Option<String> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "bend-alter" => {
                        bend_alter = Some(reader.read_text_as("bend-alter")?);
                    }
                    "pre-bend" => {
                        pre_bend = true;
                        reader.skip_element("pre-bend")?;
                    }
                    "release" => {
                        release = Some(BendRelease::Early);
                        reader.skip_element("release")?;
                    }
                    "with-bar" => {
                        with_bar = Some(reader.read_text("with-bar")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "pre-bend" => pre_bend = true,
                    "release" => release = Some(BendRelease::Early),
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in bend", reader.position()));
            }
            _ => {}
        }
    }

    Ok(Bend {
        bend_alter: bend_alter.unwrap_or(0.0),
        pre_bend,
        release,
        with_bar,
    })
}

/// Parse a barline element.
///
/// Barline elements describe bar lines at the end or within measures.
/// They contain optional bar-style, repeat, ending, segno, coda, fermata,
/// and wavy-line elements.
fn parse_barline(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Barline, ParseError> {
    // Parse location attribute
    let location = reader
        .get_optional_attr(start.attributes(), "location")?
        .map(|s| values::parse_right_left_middle(&s, reader.position()))
        .transpose()?;

    let mut bar_style = None;
    let mut wavy_line = None;
    let mut segno = None;
    let mut coda = None;
    let mut fermatas = Vec::new();
    let mut ending = None;
    let mut repeat = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "bar-style" => {
                        bar_style = Some(parse_bar_style_element(reader)?);
                    }
                    "wavy-line" => {
                        wavy_line = Some(parse_wavy_line(reader, &e)?);
                    }
                    "segno" => {
                        segno = Some(parse_segno(reader, &e)?);
                    }
                    "coda" => {
                        coda = Some(parse_coda(reader, &e)?);
                    }
                    "fermata" => {
                        fermatas.push(parse_fermata_from_barline(reader, &e)?);
                    }
                    "ending" => {
                        ending = Some(parse_ending(reader, &e)?);
                    }
                    "repeat" => {
                        repeat = Some(parse_repeat(reader, &e)?);
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
                    "segno" => {
                        segno = Some(parse_segno_from_empty(&e, reader)?);
                    }
                    "coda" => {
                        coda = Some(parse_coda_from_empty(&e, reader)?);
                    }
                    "fermata" => {
                        fermatas.push(parse_fermata_from_empty(&e, reader)?);
                    }
                    "repeat" => {
                        repeat = Some(parse_repeat_from_empty(&e, reader)?);
                    }
                    "wavy-line" => {
                        wavy_line = Some(parse_wavy_line_from_empty(&e, reader)?);
                    }
                    "ending" => {
                        ending = Some(parse_ending_from_empty(&e, reader)?);
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in barline",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Barline {
        location,
        bar_style,
        editorial: Editorial::default(),
        wavy_line,
        segno,
        coda,
        fermatas,
        ending,
        repeat,
    })
}

/// Parse a bar-style element.
///
/// Returns the BarStyle enum value parsed from the element text.
fn parse_bar_style_element(
    reader: &mut XmlReader<'_>,
) -> Result<crate::ir::attributes::BarStyle, ParseError> {
    let text = reader.read_text("bar-style")?;
    values::parse_bar_style(&text, reader.position())
}

/// Parse a repeat element.
fn parse_repeat(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Repeat, ParseError> {
    let direction_str = reader.get_attr(start.attributes(), "direction", "repeat")?;
    let direction = values::parse_backward_forward(&direction_str, reader.position())?;

    let times = reader.get_optional_attr_as::<u32>(start.attributes(), "times")?;

    let winged = reader
        .get_optional_attr(start.attributes(), "winged")?
        .map(|s| values::parse_winged(&s, reader.position()))
        .transpose()?;

    // Skip to end of element
    reader.skip_element("repeat")?;

    Ok(Repeat {
        direction,
        times,
        winged,
    })
}

/// Parse a repeat element from an empty tag.
fn parse_repeat_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Repeat, ParseError> {
    let direction_str = reader.get_attr(start.attributes(), "direction", "repeat")?;
    let direction = values::parse_backward_forward(&direction_str, reader.position())?;

    let times = reader.get_optional_attr_as::<u32>(start.attributes(), "times")?;

    let winged = reader
        .get_optional_attr(start.attributes(), "winged")?
        .map(|s| values::parse_winged(&s, reader.position()))
        .transpose()?;

    Ok(Repeat {
        direction,
        times,
        winged,
    })
}

/// Parse an ending element (volta bracket).
fn parse_ending(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Ending, ParseError> {
    let type_str = reader.get_attr(start.attributes(), "type", "ending")?;
    let r#type = values::parse_start_stop_discontinue(&type_str, reader.position())?;

    let number = reader.get_attr(start.attributes(), "number", "ending")?;

    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let end_length = reader.get_optional_attr_as::<f64>(start.attributes(), "end-length")?;
    let text_x = reader.get_optional_attr_as::<f64>(start.attributes(), "text-x")?;
    let text_y = reader.get_optional_attr_as::<f64>(start.attributes(), "text-y")?;

    // Read optional text content
    let text = reader.read_optional_text("ending")?;

    Ok(Ending {
        r#type,
        number,
        text,
        print_object,
        end_length,
        text_x,
        text_y,
    })
}

/// Parse an ending element from an empty element.
fn parse_ending_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Ending, ParseError> {
    let type_str = reader.get_attr(start.attributes(), "type", "ending")?;
    let r#type = values::parse_start_stop_discontinue(&type_str, reader.position())?;

    let number = reader.get_attr(start.attributes(), "number", "ending")?;

    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    let end_length = reader.get_optional_attr_as::<f64>(start.attributes(), "end-length")?;
    let text_x = reader.get_optional_attr_as::<f64>(start.attributes(), "text-x")?;
    let text_y = reader.get_optional_attr_as::<f64>(start.attributes(), "text-y")?;

    Ok(Ending {
        r#type,
        number,
        text: None, // Empty element has no text content
        print_object,
        end_length,
        text_x,
        text_y,
    })
}

/// Parse a segno element.
fn parse_segno(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Segno, ParseError> {
    let print_style = parse_print_style_attrs(start, reader)?;
    let smufl = reader.get_optional_attr(start.attributes(), "smufl")?;

    // Skip to end of element
    reader.skip_element("segno")?;

    Ok(Segno { print_style, smufl })
}

/// Parse a segno element from an empty tag.
fn parse_segno_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Segno, ParseError> {
    let print_style = parse_print_style_attrs(start, reader)?;
    let smufl = reader.get_optional_attr(start.attributes(), "smufl")?;

    Ok(Segno { print_style, smufl })
}

/// Parse a coda element.
fn parse_coda(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Coda, ParseError> {
    let print_style = parse_print_style_attrs(start, reader)?;
    let smufl = reader.get_optional_attr(start.attributes(), "smufl")?;

    // Skip to end of element
    reader.skip_element("coda")?;

    Ok(Coda { print_style, smufl })
}

/// Parse a coda element from an empty tag.
fn parse_coda_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Coda, ParseError> {
    let print_style = parse_print_style_attrs(start, reader)?;
    let smufl = reader.get_optional_attr(start.attributes(), "smufl")?;

    Ok(Coda { print_style, smufl })
}

/// Parse a fermata element in barline context.
fn parse_fermata_from_barline(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Fermata, ParseError> {
    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_upright_inverted(&s, reader.position()))
        .transpose()?;

    let print_style = parse_print_style_attrs(start, reader)?;

    // Read optional shape content
    let shape = reader.read_optional_text("fermata")?.and_then(|s| {
        if s.is_empty() {
            Some(FermataShape::Normal)
        } else {
            values::parse_fermata_shape(&s, 0).ok()
        }
    });

    Ok(Fermata {
        shape,
        r#type,
        print_style,
    })
}

/// Parse a fermata element from an empty tag.
fn parse_fermata_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Fermata, ParseError> {
    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_upright_inverted(&s, reader.position()))
        .transpose()?;

    let print_style = parse_print_style_attrs(start, reader)?;

    Ok(Fermata {
        shape: Some(FermataShape::Normal),
        r#type,
        print_style,
    })
}

/// Parse a wavy-line element.
fn parse_wavy_line(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<WavyLine, ParseError> {
    let type_str = reader.get_attr(start.attributes(), "type", "wavy-line")?;
    let r#type = values::parse_start_stop_continue(&type_str, reader.position())?;

    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let position = parse_position_attrs(start, reader)?;

    // Skip to end of element
    reader.skip_element("wavy-line")?;

    Ok(WavyLine {
        r#type,
        number,
        position,
    })
}

/// Parse a wavy-line element from an empty tag.
fn parse_wavy_line_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<WavyLine, ParseError> {
    let type_str = reader.get_attr(start.attributes(), "type", "wavy-line")?;
    let r#type = values::parse_start_stop_continue(&type_str, reader.position())?;

    let number = reader.get_optional_attr_as::<u8>(start.attributes(), "number")?;
    let position = parse_position_attrs(start, reader)?;

    Ok(WavyLine {
        r#type,
        number,
        position,
    })
}

/// Parse print-style attributes from an element.
fn parse_print_style_attrs(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<PrintStyle, ParseError> {
    let position = parse_position_attrs(start, reader)?;

    let color = reader.get_optional_attr(start.attributes(), "color")?;

    // Font attributes are typically not present on these elements, use defaults
    let font = Font::default();

    Ok(PrintStyle {
        position,
        font,
        color,
    })
}

/// Parse position attributes from an element.
fn parse_position_attrs(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Position, ParseError> {
    let default_x = reader.get_optional_attr_as::<f64>(start.attributes(), "default-x")?;
    let default_y = reader.get_optional_attr_as::<f64>(start.attributes(), "default-y")?;
    let relative_x = reader.get_optional_attr_as::<f64>(start.attributes(), "relative-x")?;
    let relative_y = reader.get_optional_attr_as::<f64>(start.attributes(), "relative-y")?;

    Ok(Position {
        default_x,
        default_y,
        relative_x,
        relative_y,
    })
}

/// Parse a backup element.
///
/// Backup elements move the cursor backward in time within a measure.
/// This is essential for multi-voice music where voices share the same staff.
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

/// Parse a forward element.
///
/// Forward elements move the cursor forward in time within a measure.
/// This is used to create space between notes in a voice.
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

    // =======================================================================
    // Additional tests for uncovered paths
    // =======================================================================

    #[test]
    fn test_parse_score_unexpected_element() {
        let xml = r#"<?xml version="1.0"?>
            <unknown-root>
            </unknown-root>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        if let Err(ParseError::UnexpectedElement { element, .. }) = result {
            assert_eq!(element, "unknown-root");
        } else {
            panic!("Expected UnexpectedElement error");
        }
    }

    #[test]
    fn test_parse_score_with_comments() {
        let xml = r#"<?xml version="1.0"?>
            <!-- This is a comment before the root -->
            <score-partwise>
                <!-- Another comment -->
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
        assert_eq!(score.parts.len(), 1);
    }

    #[test]
    fn test_parse_score_with_movement_number() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <movement-number>1</movement-number>
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
        assert_eq!(score.movement_number, Some("1".to_string()));
    }

    #[test]
    fn test_parse_score_without_version() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
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
        // Default version should be "4.0" when not specified
        assert_eq!(score.version, Some("4.0".to_string()));
    }

    #[test]
    fn test_parse_score_with_work_element() {
        // Work element is skipped but should not cause errors
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <work>
                    <work-number>Op. 1</work-number>
                    <work-title>Sonata</work-title>
                </work>
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
        assert!(score.work.is_none()); // Currently skipped
    }

    #[test]
    fn test_parse_score_with_identification_element() {
        // Identification element is skipped but should not cause errors
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <identification>
                    <creator type="composer">Bach</creator>
                </identification>
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
        assert!(score.identification.is_none()); // Currently skipped
    }

    #[test]
    fn test_parse_score_with_defaults_element() {
        // Defaults element is skipped but should not cause errors
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults>
                    <scaling>
                        <millimeters>7</millimeters>
                        <tenths>40</tenths>
                    </scaling>
                </defaults>
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
        assert!(score.defaults.is_none()); // Currently skipped
    }

    #[test]
    fn test_parse_score_with_credit_elements() {
        // Credit elements are skipped but should not cause errors
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <credit page="1">
                    <credit-words>Title</credit-words>
                </credit>
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
        assert!(score.credits.is_empty()); // Currently skipped
    }

    #[test]
    fn test_parse_score_with_empty_defaults() {
        // Empty defaults element should be handled
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <defaults/>
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
        assert!(score.defaults.is_none());
    }

    #[test]
    fn test_parse_score_with_unknown_element() {
        // Unknown elements should be skipped for forward compatibility
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <future-element>Some content</future-element>
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
        assert_eq!(score.parts.len(), 1);
    }

    #[test]
    fn test_parse_empty_measure() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
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
        assert_eq!(score.parts[0].measures[0].number, "1");
        assert!(score.parts[0].measures[0].content.is_empty());
    }

    #[test]
    fn test_parse_measure_with_non_controlling() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1" non-controlling="yes">
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        assert_eq!(score.parts[0].measures[0].non_controlling, Some(YesNo::Yes));
    }

    #[test]
    fn test_parse_part_group_with_barline() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-name>Strings</group-name>
                        <group-barline>yes</group-barline>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
            assert!(pg.group_barline.is_some());
        } else {
            panic!("Expected PartGroup");
        }
    }

    #[test]
    fn test_parse_part_group_mensurstrich() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-barline>Mensurstrich</group-barline>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
            use crate::ir::part::GroupBarlineValue;
            assert_eq!(
                pg.group_barline.as_ref().unwrap().value,
                GroupBarlineValue::Mensurstrich
            );
        } else {
            panic!("Expected PartGroup");
        }
    }

    #[test]
    fn test_parse_part_group_symbol_values() {
        let symbols = ["none", "brace", "line", "bracket", "square"];
        for symbol in symbols {
            let xml = format!(
                r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <part-group type="start" number="1">
                            <group-symbol>{}</group-symbol>
                        </part-group>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                        <part-group type="stop" number="1"/>
                    </part-list>
                    <part id="P1"><measure number="1"/></part>
                </score-partwise>"#,
                symbol
            );

            let score = parse_score(&xml).unwrap();
            if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
                assert!(pg.group_symbol.is_some(), "Failed for symbol: {}", symbol);
            } else {
                panic!("Expected PartGroup");
            }
        }
    }

    #[test]
    fn test_parse_part_group_invalid_symbol() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-symbol>invalid</group-symbol>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_group_invalid_barline() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-barline>invalid</group-barline>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_part_group_empty_symbol() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-symbol/>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
            use crate::ir::attributes::GroupSymbolValue;
            assert_eq!(
                pg.group_symbol.as_ref().unwrap().value,
                GroupSymbolValue::None
            );
        } else {
            panic!("Expected PartGroup");
        }
    }

    #[test]
    fn test_parse_part_group_with_group_time() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <part-group type="start" number="1">
                        <group-time/>
                    </part-group>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                    <part-group type="stop" number="1"/>
                </part-list>
                <part id="P1"><measure number="1"/></part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let PartListElement::PartGroup(pg) = &score.part_list.content[0] {
            assert!(pg.group_time.is_some());
        } else {
            panic!("Expected PartGroup");
        }
    }

    #[test]
    fn test_parse_score_part_with_empty_part_name() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name/>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
            assert_eq!(sp.part_name.value, "");
        } else {
            panic!("Expected ScorePart");
        }
    }

    #[test]
    fn test_parse_score_part_with_group() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Violin</part-name>
                        <group>Strings</group>
                        <group>Orchestra</group>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1"/>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
            assert_eq!(sp.group.len(), 2);
            assert_eq!(sp.group[0], "Strings");
            assert_eq!(sp.group[1], "Orchestra");
        } else {
            panic!("Expected ScorePart");
        }
    }

    #[test]
    fn test_parse_unpitched_note() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Drums</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <unpitched>
                                <display-step>E</display-step>
                                <display-octave>4</display-octave>
                            </unpitched>
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
                if let PitchRestUnpitched::Unpitched(u) = &full_note.content {
                    assert_eq!(u.display_step, Some(crate::ir::pitch::Step::E));
                    assert_eq!(u.display_octave, Some(4));
                } else {
                    panic!("Expected Unpitched");
                }
            } else {
                panic!("Expected Regular note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_empty_unpitched_note() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Drums</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <note>
                            <unpitched/>
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
                if let PitchRestUnpitched::Unpitched(u) = &full_note.content {
                    assert!(u.display_step.is_none());
                    assert!(u.display_octave.is_none());
                } else {
                    panic!("Expected Unpitched");
                }
            }
        }
    }

    #[test]
    fn test_parse_rest_with_display_position() {
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
                            <rest>
                                <display-step>B</display-step>
                                <display-octave>4</display-octave>
                            </rest>
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
                if let PitchRestUnpitched::Rest(r) = &full_note.content {
                    assert_eq!(r.display_step, Some(crate::ir::pitch::Step::B));
                    assert_eq!(r.display_octave, Some(4));
                } else {
                    panic!("Expected Rest");
                }
            }
        }
    }

    #[test]
    fn test_parse_note_with_notehead() {
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
                            <notehead>diamond</notehead>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(note.notehead.is_some());
            use crate::ir::beam::NoteheadValue;
            assert_eq!(
                note.notehead.as_ref().unwrap().value,
                NoteheadValue::Diamond
            );
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_cue_note() {
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
                            <cue/>
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
            if let NoteContent::Cue {
                full_note,
                duration,
            } = &note.content
            {
                assert_eq!(*duration, 4);
                if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                    assert_eq!(p.step, crate::ir::pitch::Step::C);
                } else {
                    panic!("Expected Pitch");
                }
            } else {
                panic!("Expected Cue note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_key_with_cancel() {
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
                                <cancel>-2</cancel>
                                <fifths>1</fifths>
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
            if let KeyContent::Traditional(tk) = &attrs.keys[0].content {
                assert!(tk.cancel.is_some());
                assert_eq!(tk.cancel.as_ref().unwrap().fifths, -2);
            } else {
                panic!("Expected Traditional key");
            }
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_time_senza_misura() {
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
                                <senza-misura/>
                            </time>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            if let TimeContent::SenzaMisura(_) = &attrs.times[0].content {
                // Success
            } else {
                panic!("Expected SenzaMisura time");
            }
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_clef_octave_change() {
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
                                <clef-octave-change>-1</clef-octave-change>
                            </clef>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.clefs[0].octave_change, Some(-1));
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_attributes_staves() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Piano</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <attributes>
                            <divisions>4</divisions>
                            <staves>2</staves>
                        </attributes>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(attrs.staves, Some(2));
        } else {
            panic!("Expected Attributes");
        }
    }

    #[test]
    fn test_parse_forward_without_optional_elements() {
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
                            <duration>4</duration>
                        </forward>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Forward(forward) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(forward.duration, 4);
            assert!(forward.voice.is_none());
            assert!(forward.staff.is_none());
        } else {
            panic!("Expected Forward");
        }
    }

    #[test]
    fn test_parse_all_clef_signs() {
        let signs = ["G", "F", "C", "percussion", "TAB", "jianpu", "none"];
        for sign in signs {
            let xml = format!(
                r#"<?xml version="1.0"?>
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
                                    <sign>{}</sign>
                                </clef>
                            </attributes>
                        </measure>
                    </part>
                </score-partwise>"#,
                sign
            );

            let score = parse_score(&xml).unwrap();
            if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
                &score.parts[0].measures[0].content[0]
            {
                assert!(!attrs.clefs.is_empty(), "Failed for sign: {}", sign);
            } else {
                panic!("Expected Attributes for sign: {}", sign);
            }
        }
    }

    #[test]
    fn test_parse_all_mode_values() {
        let modes = [
            "major",
            "minor",
            "dorian",
            "phrygian",
            "lydian",
            "mixolydian",
            "aeolian",
            "ionian",
            "locrian",
        ];
        for mode in modes {
            let xml = format!(
                r#"<?xml version="1.0"?>
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
                                    <fifths>0</fifths>
                                    <mode>{}</mode>
                                </key>
                            </attributes>
                        </measure>
                    </part>
                </score-partwise>"#,
                mode
            );

            let score = parse_score(&xml).unwrap();
            if let crate::ir::measure::MusicDataElement::Attributes(attrs) =
                &score.parts[0].measures[0].content[0]
            {
                if let KeyContent::Traditional(tk) = &attrs.keys[0].content {
                    assert!(tk.mode.is_some(), "Failed for mode: {}", mode);
                } else {
                    panic!("Expected Traditional key for mode: {}", mode);
                }
            } else {
                panic!("Expected Attributes for mode: {}", mode);
            }
        }
    }

    #[test]
    fn test_parse_note_without_type() {
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
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(note.r#type.is_none());
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_grace_note_with_steal_time() {
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
                            <grace steal-time-previous="50" steal-time-following="25"/>
                            <pitch>
                                <step>D</step>
                                <octave>4</octave>
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
                assert_eq!(grace.steal_time_previous, Some(50.0));
                assert_eq!(grace.steal_time_following, Some(25.0));
            } else {
                panic!("Expected Grace note");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_double_dotted_note() {
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
                            <duration>7</duration>
                            <type>quarter</type>
                            <dot/>
                            <dot/>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(note.dots.len(), 2);
        } else {
            panic!("Expected Note");
        }
    }

    // =======================================================================
    // Multi-Voice Tests (Task 3.3)
    // =======================================================================

    #[test]
    fn test_parse_two_voice_measure_with_backup() {
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
                        <note>
                            <pitch>
                                <step>E</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>16</duration>
                            <voice>1</voice>
                            <type>whole</type>
                        </note>
                        <backup>
                            <duration>16</duration>
                        </backup>
                        <note>
                            <pitch>
                                <step>C</step>
                                <octave>3</octave>
                            </pitch>
                            <duration>16</duration>
                            <voice>2</voice>
                            <type>whole</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        let content = &score.parts[0].measures[0].content;
        assert_eq!(content.len(), 4); // attributes, note, backup, note

        // Check first note is voice 1
        if let crate::ir::measure::MusicDataElement::Note(note) = &content[1] {
            assert_eq!(note.voice, Some("1".to_string()));
        } else {
            panic!("Expected Note at index 1");
        }

        // Check backup element
        if let crate::ir::measure::MusicDataElement::Backup(backup) = &content[2] {
            assert_eq!(backup.duration, 16);
        } else {
            panic!("Expected Backup at index 2");
        }

        // Check second note is voice 2
        if let crate::ir::measure::MusicDataElement::Note(note) = &content[3] {
            assert_eq!(note.voice, Some("2".to_string()));
        } else {
            panic!("Expected Note at index 3");
        }
    }

    #[test]
    fn test_parse_forward_element_with_voice_and_staff() {
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

    #[test]
    fn test_parse_voice_assignment_preserved() {
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
                        </note>
                        <note>
                            <pitch>
                                <step>D</step>
                                <octave>4</octave>
                            </pitch>
                            <duration>4</duration>
                            <voice>1</voice>
                            <type>quarter</type>
                        </note>
                        <backup>
                            <duration>8</duration>
                        </backup>
                        <note>
                            <pitch>
                                <step>G</step>
                                <octave>3</octave>
                            </pitch>
                            <duration>8</duration>
                            <voice>2</voice>
                            <type>half</type>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        let content = &score.parts[0].measures[0].content;

        // Verify voice assignments are preserved
        let mut voice_1_count = 0;
        let mut voice_2_count = 0;

        for element in content {
            if let crate::ir::measure::MusicDataElement::Note(note) = element {
                match note.voice.as_deref() {
                    Some("1") => voice_1_count += 1,
                    Some("2") => voice_2_count += 1,
                    _ => {}
                }
            }
        }

        assert_eq!(voice_1_count, 2, "Expected 2 notes in voice 1");
        assert_eq!(voice_2_count, 1, "Expected 1 note in voice 2");
    }

    // =======================================================================
    // Barline Tests (Task 3.4)
    // =======================================================================

    #[test]
    fn test_parse_barline_simple_forward_repeat() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <bar-style>heavy-light</bar-style>
                            <repeat direction="forward"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(
                barline.location,
                Some(crate::ir::common::RightLeftMiddle::Left)
            );
            assert_eq!(
                barline.bar_style,
                Some(crate::ir::attributes::BarStyle::HeavyLight)
            );
            assert!(barline.repeat.is_some());
            let repeat = barline.repeat.as_ref().unwrap();
            assert_eq!(
                repeat.direction,
                crate::ir::common::BackwardForward::Forward
            );
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_backward_repeat() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <bar-style>light-heavy</bar-style>
                            <repeat direction="backward" times="2"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(
                barline.location,
                Some(crate::ir::common::RightLeftMiddle::Right)
            );
            assert_eq!(
                barline.bar_style,
                Some(crate::ir::attributes::BarStyle::LightHeavy)
            );
            let repeat = barline.repeat.as_ref().unwrap();
            assert_eq!(
                repeat.direction,
                crate::ir::common::BackwardForward::Backward
            );
            assert_eq!(repeat.times, Some(2));
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_volta_first_ending() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <ending number="1" type="start">1.</ending>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(barline.ending.is_some());
            let ending = barline.ending.as_ref().unwrap();
            assert_eq!(
                ending.r#type,
                crate::ir::common::StartStopDiscontinue::Start
            );
            assert_eq!(ending.number, "1");
            assert_eq!(ending.text, Some("1.".to_string()));
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_volta_second_ending() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <ending number="2" type="start">2.</ending>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            let ending = barline.ending.as_ref().unwrap();
            assert_eq!(ending.number, "2");
            assert_eq!(ending.text, Some("2.".to_string()));
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_ending_stop() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <ending number="1" type="stop"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            let ending = barline.ending.as_ref().unwrap();
            assert_eq!(ending.r#type, crate::ir::common::StartStopDiscontinue::Stop);
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_ending_discontinue() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <ending number="1" type="discontinue"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            let ending = barline.ending.as_ref().unwrap();
            assert_eq!(
                ending.r#type,
                crate::ir::common::StartStopDiscontinue::Discontinue
            );
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_with_segno() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <segno/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(barline.segno.is_some());
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_with_coda() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="left">
                            <coda/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(barline.coda.is_some());
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_with_fermata() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <fermata type="upright"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(barline.fermatas.len(), 1);
            assert_eq!(
                barline.fermatas[0].r#type,
                Some(crate::ir::common::UprightInverted::Upright)
            );
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_all_bar_styles() {
        let styles = [
            ("regular", crate::ir::attributes::BarStyle::Regular),
            ("dotted", crate::ir::attributes::BarStyle::Dotted),
            ("dashed", crate::ir::attributes::BarStyle::Dashed),
            ("heavy", crate::ir::attributes::BarStyle::Heavy),
            ("light-light", crate::ir::attributes::BarStyle::LightLight),
            ("light-heavy", crate::ir::attributes::BarStyle::LightHeavy),
            ("heavy-light", crate::ir::attributes::BarStyle::HeavyLight),
            ("heavy-heavy", crate::ir::attributes::BarStyle::HeavyHeavy),
            ("tick", crate::ir::attributes::BarStyle::Tick),
            ("short", crate::ir::attributes::BarStyle::Short),
            ("none", crate::ir::attributes::BarStyle::None),
        ];

        for (style_str, expected_style) in styles {
            let xml = format!(
                r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <barline>
                                <bar-style>{}</bar-style>
                            </barline>
                        </measure>
                    </part>
                </score-partwise>"#,
                style_str
            );

            let score = parse_score(&xml).unwrap();
            if let crate::ir::measure::MusicDataElement::Barline(barline) =
                &score.parts[0].measures[0].content[0]
            {
                assert_eq!(
                    barline.bar_style,
                    Some(expected_style),
                    "Failed for style: {}",
                    style_str
                );
            } else {
                panic!("Expected Barline for style: {}", style_str);
            }
        }
    }

    #[test]
    fn test_parse_barline_repeat_with_winged() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="right">
                            <repeat direction="backward" winged="curved"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            let repeat = barline.repeat.as_ref().unwrap();
            assert_eq!(repeat.winged, Some(crate::ir::attributes::Winged::Curved));
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_location_middle() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline location="middle">
                            <bar-style>dashed</bar-style>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(
                barline.location,
                Some(crate::ir::common::RightLeftMiddle::Middle)
            );
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_with_wavy_line() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <wavy-line type="start" number="1"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(barline.wavy_line.is_some());
            let wavy = barline.wavy_line.as_ref().unwrap();
            assert_eq!(wavy.r#type, crate::ir::common::StartStopContinue::Start);
            assert_eq!(wavy.number, Some(1));
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_empty_repeat() {
        // Test parsing repeat as an empty element
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <repeat direction="forward"/>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            assert!(barline.repeat.is_some());
        } else {
            panic!("Expected Barline");
        }
    }

    #[test]
    fn test_parse_barline_ending_with_attributes() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <barline>
                            <ending number="1, 2" type="start" end-length="30" text-x="5" text-y="-10">1, 2.</ending>
                        </barline>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Barline(barline) =
            &score.parts[0].measures[0].content[0]
        {
            let ending = barline.ending.as_ref().unwrap();
            assert_eq!(ending.number, "1, 2");
            assert_eq!(ending.text, Some("1, 2.".to_string()));
            assert_eq!(ending.end_length, Some(30.0));
            assert_eq!(ending.text_x, Some(5.0));
            assert_eq!(ending.text_y, Some(-10.0));
        } else {
            panic!("Expected Barline");
        }
    }

    // =======================================================================
    // Direction Tests (Milestone 4, Task 4.1-4.3)
    // =======================================================================

    #[test]
    fn test_parse_direction_with_dynamics_f() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction placement="below">
                            <direction-type>
                                <dynamics><f/></dynamics>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Direction(dir) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(dir.placement, Some(crate::ir::common::AboveBelow::Below));
            assert_eq!(dir.direction_types.len(), 1);
            if let crate::ir::direction::DirectionTypeContent::Dynamics(d) =
                &dir.direction_types[0].content
            {
                assert_eq!(d.content.len(), 1);
                assert_eq!(d.content[0], crate::ir::direction::DynamicElement::F);
            } else {
                panic!("Expected Dynamics content");
            }
        } else {
            panic!("Expected Direction");
        }
    }

    #[test]
    fn test_parse_direction_with_wedge_crescendo() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <wedge type="crescendo"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Direction(dir) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::direction::DirectionTypeContent::Wedge(w) =
                &dir.direction_types[0].content
            {
                assert_eq!(w.r#type, crate::ir::direction::WedgeType::Crescendo);
            } else {
                panic!("Expected Wedge content");
            }
        } else {
            panic!("Expected Direction");
        }
    }

    #[test]
    fn test_parse_direction_with_metronome() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <metronome>
                                    <beat-unit>quarter</beat-unit>
                                    <per-minute>120</per-minute>
                                </metronome>
                            </direction-type>
                            <sound tempo="120"/>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Direction(dir) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::direction::DirectionTypeContent::Metronome(m) =
                &dir.direction_types[0].content
            {
                if let crate::ir::direction::MetronomeContent::PerMinute { per_minute, .. } =
                    &m.content
                {
                    assert_eq!(per_minute.value, "120");
                } else {
                    panic!("Expected PerMinute content");
                }
            } else {
                panic!("Expected Metronome content");
            }
            // Check sound element
            assert!(dir.sound.is_some());
            assert_eq!(dir.sound.as_ref().unwrap().tempo, Some(120.0));
        } else {
            panic!("Expected Direction");
        }
    }

    #[test]
    fn test_parse_direction_with_words() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <words>cresc.</words>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Direction(dir) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::direction::DirectionTypeContent::Words(w) =
                &dir.direction_types[0].content
            {
                assert_eq!(w.len(), 1);
                assert_eq!(w[0].value, "cresc.");
            } else {
                panic!("Expected Words content");
            }
        } else {
            panic!("Expected Direction");
        }
    }

    #[test]
    fn test_parse_direction_with_pedal() {
        let xml = r#"<?xml version="1.0"?>
            <score-partwise>
                <part-list>
                    <score-part id="P1">
                        <part-name>Test</part-name>
                    </score-part>
                </part-list>
                <part id="P1">
                    <measure number="1">
                        <direction>
                            <direction-type>
                                <pedal type="start" line="yes"/>
                            </direction-type>
                        </direction>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Direction(dir) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::direction::DirectionTypeContent::Pedal(p) =
                &dir.direction_types[0].content
            {
                assert_eq!(p.r#type, crate::ir::direction::PedalType::Start);
                assert_eq!(p.line, Some(YesNo::Yes));
            } else {
                panic!("Expected Pedal content");
            }
        } else {
            panic!("Expected Direction");
        }
    }

    // =======================================================================
    // Notations Tests (Milestone 4, Task 4.4-4.5)
    // =======================================================================

    #[test]
    fn test_parse_note_with_tied_notation() {
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
                            <notations>
                                <tied type="start"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            assert_eq!(note.notations.len(), 1);
            assert_eq!(note.notations[0].content.len(), 1);
            if let crate::ir::notation::NotationContent::Tied(t) = &note.notations[0].content[0] {
                assert_eq!(t.r#type, crate::ir::common::StartStopContinue::Start);
            } else {
                panic!("Expected Tied notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_slur() {
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
                            <notations>
                                <slur type="start" number="1"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Slur(s) = &note.notations[0].content[0] {
                assert_eq!(s.r#type, crate::ir::common::StartStopContinue::Start);
                assert_eq!(s.number, 1);
            } else {
                panic!("Expected Slur notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_articulations() {
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
                            <notations>
                                <articulations>
                                    <staccato/>
                                    <accent/>
                                </articulations>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Articulations(a) =
                &note.notations[0].content[0]
            {
                assert_eq!(a.content.len(), 2);
            } else {
                panic!("Expected Articulations notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_ornaments_trill() {
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
                            <notations>
                                <ornaments>
                                    <trill-mark/>
                                </ornaments>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Ornaments(o) =
                &note.notations[0].content[0]
            {
                assert_eq!(o.content.len(), 1);
                if let crate::ir::notation::OrnamentElement::TrillMark(_) = &o.content[0].ornament {
                    // Success
                } else {
                    panic!("Expected TrillMark ornament");
                }
            } else {
                panic!("Expected Ornaments notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_fermata() {
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
                            <notations>
                                <fermata type="upright"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Fermata(f) = &note.notations[0].content[0]
            {
                assert_eq!(f.r#type, Some(crate::ir::common::UprightInverted::Upright));
            } else {
                panic!("Expected Fermata notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_tuplet() {
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
                            <notations>
                                <tuplet type="start" bracket="yes"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Tuplet(t) = &note.notations[0].content[0] {
                assert_eq!(t.r#type, crate::ir::common::StartStop::Start);
                assert_eq!(t.bracket, Some(YesNo::Yes));
            } else {
                panic!("Expected Tuplet notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_technical_fingering() {
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
                            <notations>
                                <technical>
                                    <fingering>1</fingering>
                                </technical>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Technical(t) =
                &note.notations[0].content[0]
            {
                assert_eq!(t.content.len(), 1);
                if let crate::ir::notation::TechnicalElement::Fingering(f) = &t.content[0] {
                    assert_eq!(f.value, "1");
                } else {
                    panic!("Expected Fingering technical");
                }
            } else {
                panic!("Expected Technical notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_note_with_arpeggiate() {
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
                            <notations>
                                <arpeggiate direction="up"/>
                            </notations>
                        </note>
                    </measure>
                </part>
            </score-partwise>"#;

        let score = parse_score(xml).unwrap();
        if let crate::ir::measure::MusicDataElement::Note(note) =
            &score.parts[0].measures[0].content[0]
        {
            if let crate::ir::notation::NotationContent::Arpeggiate(a) =
                &note.notations[0].content[0]
            {
                assert_eq!(a.direction, Some(crate::ir::common::UpDown::Up));
            } else {
                panic!("Expected Arpeggiate notation");
            }
        } else {
            panic!("Expected Note");
        }
    }

    #[test]
    fn test_parse_all_dynamics() {
        let dynamics = [
            "p", "pp", "ppp", "pppp", "ppppp", "pppppp", "f", "ff", "fff", "ffff", "fffff",
            "ffffff", "mp", "mf", "sf", "sfp", "sfpp", "fp", "rf", "rfz", "sfz", "sffz", "fz", "n",
            "pf", "sfzp",
        ];
        for d in dynamics {
            let xml = format!(
                r#"<?xml version="1.0"?>
                <score-partwise>
                    <part-list>
                        <score-part id="P1">
                            <part-name>Test</part-name>
                        </score-part>
                    </part-list>
                    <part id="P1">
                        <measure number="1">
                            <direction>
                                <direction-type>
                                    <dynamics><{}/></dynamics>
                                </direction-type>
                            </direction>
                        </measure>
                    </part>
                </score-partwise>"#,
                d
            );

            let result = parse_score(&xml);
            assert!(
                result.is_ok(),
                "Failed to parse dynamics: {} - {:?}",
                d,
                result.err()
            );
        }
    }
}
