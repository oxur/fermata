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
use crate::ir::common::{
    Editorial, Encoding, EncodingContent, Font, Identification, Position, Supports, TypedText,
    WavyLine, YesNo,
};
use crate::ir::direction::{Coda, Segno};
use crate::ir::duration::{Dot, NoteType, TimeModification};
use crate::ir::lyric::{Elision, Extend, Lyric, LyricContent, TextElementData};
use crate::ir::measure::Measure;
use crate::ir::notation::{Fermata, FermataShape};
use crate::ir::note::{
    Accidental, FullNote, Grace, Note, NoteContent, PitchRestUnpitched, Rest, Tie,
};
use crate::ir::part::{PartList, PartListElement, PartName, ScorePart};
use crate::ir::pitch::{Pitch, Unpitched};
use crate::ir::score::{
    Appearance, Credit, CreditContent, CreditImage, CreditWords, Defaults, Distance, Divider,
    LineWidth, LyricFont, LyricLanguage, NoteSize, Opus, PageLayout, PageMargins, Scaling,
    ScorePartwise, StaffLayout, SystemDividers, SystemLayout, SystemMargins, Work,
};
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
                        score.work = Some(parse_work(reader)?);
                    }
                    "movement-number" => {
                        score.movement_number = Some(reader.read_text("movement-number")?);
                    }
                    "movement-title" => {
                        score.movement_title = Some(reader.read_text("movement-title")?);
                    }
                    "identification" => {
                        score.identification = Some(parse_identification(reader)?);
                    }
                    "defaults" => {
                        score.defaults = Some(parse_defaults(reader)?);
                    }
                    "credit" => {
                        score.credits.push(parse_credit(reader, &e)?);
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
    let mut lyrics: Vec<Lyric> = Vec::new();

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
                        lyrics.push(parse_lyric(reader, &e)?);
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
        lyrics,
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

// =============================================================================
// Lyric Parsing (Phase 3 Milestone 5, Task 5.1)
// =============================================================================

/// Parse a lyric element.
///
/// Lyrics describe sung text associated with notes. They can include syllables,
/// elisions, extends (melisma lines), and special markers like laughing or humming.
fn parse_lyric(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Lyric, ParseError> {
    use crate::ir::lyric::{LyricExtension, Syllabic};

    let number = reader.get_optional_attr(start.attributes(), "number")?;
    let name = reader.get_optional_attr(start.attributes(), "name")?;
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    // Track what we're parsing
    let mut content: Option<LyricContent> = None;
    let mut end_line = false;
    let mut end_paragraph = false;

    // For building syllable content
    let mut syllabic: Option<Syllabic> = None;
    let mut text: Option<TextElementData> = None;
    let mut extensions: Vec<LyricExtension> = Vec::new();
    let mut extend: Option<Extend> = None;

    // For building extensions (elision + syllabic? + text sequences)
    let mut pending_elision: Option<Elision> = None;
    let mut pending_syllabic: Option<Syllabic> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let elem_name = element_name(&e);
                match elem_name.as_str() {
                    "syllabic" => {
                        let s = reader.read_text("syllabic")?;
                        let parsed = values::parse_syllabic(&s, reader.position())?;
                        // If we have a pending elision, this syllabic is for the extension
                        if pending_elision.is_some() {
                            pending_syllabic = Some(parsed);
                        } else {
                            syllabic = Some(parsed);
                        }
                    }
                    "text" => {
                        let parsed_text = parse_text_element_data(reader, &e)?;
                        // If we have a pending elision, this completes an extension
                        if let Some(elision) = pending_elision.take() {
                            extensions.push(LyricExtension {
                                elision,
                                syllabic: pending_syllabic.take(),
                                text: parsed_text,
                            });
                        } else {
                            text = Some(parsed_text);
                        }
                    }
                    "elision" => {
                        let elision_value =
                            reader.read_optional_text("elision")?.unwrap_or_default();
                        pending_elision = Some(Elision {
                            value: elision_value,
                            font: Font::default(),
                            color: None,
                        });
                    }
                    "extend" => {
                        extend = Some(parse_extend_element(reader, &e)?);
                    }
                    "laughing" => {
                        content = Some(LyricContent::Laughing);
                        reader.skip_element("laughing")?;
                    }
                    "humming" => {
                        content = Some(LyricContent::Humming);
                        reader.skip_element("humming")?;
                    }
                    "end-line" => {
                        end_line = true;
                        reader.skip_element("end-line")?;
                    }
                    "end-paragraph" => {
                        end_paragraph = true;
                        reader.skip_element("end-paragraph")?;
                    }
                    "footnote" | "level" => {
                        reader.skip_element(&elem_name)?;
                    }
                    _ => {
                        reader.skip_element(&elem_name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let elem_name = element_name(&e);
                match elem_name.as_str() {
                    "extend" => {
                        extend = Some(parse_extend_from_empty_element(&e, reader)?);
                    }
                    "laughing" => {
                        content = Some(LyricContent::Laughing);
                    }
                    "humming" => {
                        content = Some(LyricContent::Humming);
                    }
                    "end-line" => {
                        end_line = true;
                    }
                    "end-paragraph" => {
                        end_paragraph = true;
                    }
                    "elision" => {
                        // Empty elision element (uses space as separator)
                        pending_elision = Some(Elision {
                            value: String::new(),
                            font: Font::default(),
                            color: None,
                        });
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in lyric",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    // Determine content type
    let lyric_content = if let Some(c) = content {
        c
    } else if let Some(t) = text {
        LyricContent::Syllable {
            syllabic,
            text: t,
            extensions,
            extend,
        }
    } else if let Some(ext) = extend {
        LyricContent::ExtendOnly(ext)
    } else {
        // Empty syllable (unusual but valid)
        LyricContent::Syllable {
            syllabic: None,
            text: TextElementData {
                value: String::new(),
                font: Font::default(),
                color: None,
                lang: None,
            },
            extensions: vec![],
            extend: None,
        }
    };

    Ok(Lyric {
        number,
        name,
        justify,
        placement,
        print_object,
        content: lyric_content,
        end_line,
        end_paragraph,
    })
}

/// Parse a lyric element from an empty element (rare but possible).
#[allow(dead_code)]
fn parse_lyric_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Lyric, ParseError> {
    let number = reader.get_optional_attr(start.attributes(), "number")?;
    let name = reader.get_optional_attr(start.attributes(), "name")?;
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let placement = reader
        .get_optional_attr(start.attributes(), "placement")?
        .map(|s| values::parse_above_below(&s, reader.position()))
        .transpose()?;
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;

    Ok(Lyric {
        number,
        name,
        justify,
        placement,
        print_object,
        content: LyricContent::Syllable {
            syllabic: None,
            text: TextElementData {
                value: String::new(),
                font: Font::default(),
                color: None,
                lang: None,
            },
            extensions: vec![],
            extend: None,
        },
        end_line: false,
        end_paragraph: false,
    })
}

/// Parse text-element-data with formatting attributes.
fn parse_text_element_data(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<TextElementData, ParseError> {
    let font = parse_font_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;
    let lang = reader.get_optional_attr(start.attributes(), "xml:lang")?;

    let value = reader.read_text("text")?;

    Ok(TextElementData {
        value,
        font,
        color,
        lang,
    })
}

/// Parse font attributes from an element.
fn parse_font_attrs(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Font, ParseError> {
    let font_family = reader.get_optional_attr(start.attributes(), "font-family")?;
    use crate::ir::common::{FontStyle, FontWeight};

    let font_style = reader
        .get_optional_attr(start.attributes(), "font-style")?
        .and_then(|s| match s.as_str() {
            "normal" => Some(FontStyle::Normal),
            "italic" => Some(FontStyle::Italic),
            _ => None,
        });

    let font_weight = reader
        .get_optional_attr(start.attributes(), "font-weight")?
        .and_then(|s| match s.as_str() {
            "normal" => Some(FontWeight::Normal),
            "bold" => Some(FontWeight::Bold),
            _ => None,
        });

    // font-size can be a number or a CSS size keyword
    let font_size = reader
        .get_optional_attr(start.attributes(), "font-size")?
        .map(|s| values::parse_font_size(&s, reader.position()))
        .transpose()?;

    Ok(Font {
        font_family,
        font_style,
        font_size,
        font_weight,
    })
}

/// Parse an extend element (melisma line).
fn parse_extend_element(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Extend, ParseError> {
    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_start_stop_continue(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    reader.skip_element("extend")?;

    Ok(Extend {
        r#type,
        position,
        color,
    })
}

/// Parse an extend element from an empty element.
fn parse_extend_from_empty_element(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Extend, ParseError> {
    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_start_stop_continue(&s, reader.position()))
        .transpose()?;
    let position = parse_position_attrs(start, reader)?;
    let color = reader.get_optional_attr(start.attributes(), "color")?;

    Ok(Extend {
        r#type,
        position,
        color,
    })
}

// =============================================================================
// Score Header Parsing (Phase 3 Milestone 5, Task 5.4)
// =============================================================================

/// Parse a work element containing work-number, work-title, and opus.
fn parse_work(reader: &mut XmlReader<'_>) -> Result<Work, ParseError> {
    let mut work_number: Option<String> = None;
    let mut work_title: Option<String> = None;
    let mut opus: Option<Opus> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "work-number" => {
                        work_number = Some(reader.read_text("work-number")?);
                    }
                    "work-title" => {
                        work_title = Some(reader.read_text("work-title")?);
                    }
                    "opus" => {
                        opus = Some(parse_opus(reader, &e)?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name.as_str() == "opus" {
                    opus = Some(parse_opus_from_empty(&e, reader)?);
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml("unexpected EOF in work", reader.position()));
            }
            _ => {}
        }
    }

    Ok(Work {
        work_number,
        work_title,
        opus,
    })
}

/// Parse an opus element (link to opus document).
fn parse_opus(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Opus, ParseError> {
    let href = reader.get_attr(start.attributes(), "xlink:href", "opus")?;
    reader.skip_element("opus")?;
    Ok(Opus { href })
}

/// Parse an opus element from empty tag.
fn parse_opus_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Opus, ParseError> {
    let href = reader.get_attr(start.attributes(), "xlink:href", "opus")?;
    Ok(Opus { href })
}

/// Parse an identification element containing creators, rights, encoding, source, relations.
fn parse_identification(reader: &mut XmlReader<'_>) -> Result<Identification, ParseError> {
    let mut creators: Vec<TypedText> = Vec::new();
    let mut rights: Vec<TypedText> = Vec::new();
    let mut encoding: Option<Encoding> = None;
    let mut source: Option<String> = None;
    let mut relations: Vec<TypedText> = Vec::new();
    let mut miscellaneous: Option<crate::ir::common::Miscellaneous> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "creator" => {
                        let r#type = reader.get_optional_attr(e.attributes(), "type")?;
                        let value = reader.read_text("creator")?;
                        creators.push(TypedText { r#type, value });
                    }
                    "rights" => {
                        let r#type = reader.get_optional_attr(e.attributes(), "type")?;
                        let value = reader.read_text("rights")?;
                        rights.push(TypedText { r#type, value });
                    }
                    "encoding" => {
                        encoding = Some(parse_encoding(reader)?);
                    }
                    "source" => {
                        source = Some(reader.read_text("source")?);
                    }
                    "relation" => {
                        let r#type = reader.get_optional_attr(e.attributes(), "type")?;
                        let value = reader.read_text("relation")?;
                        relations.push(TypedText { r#type, value });
                    }
                    "miscellaneous" => {
                        miscellaneous = parse_miscellaneous(reader)?;
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in identification",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Identification {
        creators,
        rights,
        encoding,
        source,
        relations,
        miscellaneous,
    })
}

/// Parse a miscellaneous element containing miscellaneous-field elements.
fn parse_miscellaneous(
    reader: &mut XmlReader<'_>,
) -> Result<Option<crate::ir::common::Miscellaneous>, ParseError> {
    use crate::ir::common::{Miscellaneous, MiscellaneousField};
    let mut fields: Vec<MiscellaneousField> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "miscellaneous-field" => {
                        let field_name =
                            reader.get_attr(e.attributes(), "name", "miscellaneous-field")?;
                        let value = reader.read_text("miscellaneous-field")?;
                        fields.push(MiscellaneousField {
                            name: field_name,
                            value,
                        });
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in miscellaneous",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    if fields.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Miscellaneous { fields }))
    }
}

/// Parse an encoding element containing encoding-date, encoder, software, etc.
fn parse_encoding(reader: &mut XmlReader<'_>) -> Result<Encoding, ParseError> {
    let mut content: Vec<EncodingContent> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "encoding-date" => {
                        let value = reader.read_text("encoding-date")?;
                        content.push(EncodingContent::EncodingDate(value));
                    }
                    "encoder" => {
                        let r#type = reader.get_optional_attr(e.attributes(), "type")?;
                        let value = reader.read_text("encoder")?;
                        content.push(EncodingContent::Encoder(TypedText { r#type, value }));
                    }
                    "software" => {
                        let value = reader.read_text("software")?;
                        content.push(EncodingContent::Software(value));
                    }
                    "encoding-description" => {
                        let value = reader.read_text("encoding-description")?;
                        content.push(EncodingContent::EncodingDescription(value));
                    }
                    "supports" => {
                        let supports = parse_supports(reader, &e)?;
                        content.push(EncodingContent::Supports(supports));
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                if name == "supports" {
                    let supports = parse_supports_from_empty(&e, reader)?;
                    content.push(EncodingContent::Supports(supports));
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in encoding",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Encoding { content })
}

/// Parse a supports element.
fn parse_supports(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Supports, ParseError> {
    let element = reader.get_attr(start.attributes(), "element", "supports")?;
    let type_str = reader.get_attr(start.attributes(), "type", "supports")?;
    let r#type = values::parse_yes_no(&type_str, reader.position())?;
    let attribute = reader.get_optional_attr(start.attributes(), "attribute")?;
    let value = reader.get_optional_attr(start.attributes(), "value")?;

    reader.skip_element("supports")?;

    Ok(Supports {
        r#type,
        element,
        attribute,
        value,
    })
}

/// Parse a supports element from empty tag.
fn parse_supports_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Supports, ParseError> {
    let element = reader.get_attr(start.attributes(), "element", "supports")?;
    let type_str = reader.get_attr(start.attributes(), "type", "supports")?;
    let r#type = values::parse_yes_no(&type_str, reader.position())?;
    let attribute = reader.get_optional_attr(start.attributes(), "attribute")?;
    let value = reader.get_optional_attr(start.attributes(), "value")?;

    Ok(Supports {
        r#type,
        element,
        attribute,
        value,
    })
}

/// Parse a credit element containing credit information for score display.
fn parse_credit(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Credit, ParseError> {
    let page = reader.get_optional_attr_as::<u32>(start.attributes(), "page")?;

    let mut content: Vec<CreditContent> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "credit-type" => {
                        let value = reader.read_text("credit-type")?;
                        content.push(CreditContent::CreditType(value));
                    }
                    "credit-words" => {
                        let cw = parse_credit_words(reader, &e)?;
                        content.push(CreditContent::CreditWords(cw));
                    }
                    "credit-symbol" => {
                        let cs = parse_credit_symbol(reader, &e)?;
                        content.push(CreditContent::CreditSymbol(cs));
                    }
                    "credit-image" => {
                        let img = parse_credit_image(reader, &e)?;
                        content.push(CreditContent::CreditImage(img));
                    }
                    "link" | "bookmark" => {
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
                    "credit-words" => {
                        let cw = parse_credit_words_from_empty(&e, reader)?;
                        content.push(CreditContent::CreditWords(cw));
                    }
                    "credit-image" => {
                        let img = parse_credit_image_from_empty(&e, reader)?;
                        content.push(CreditContent::CreditImage(img));
                    }
                    "credit-symbol" => {
                        let cs = parse_credit_symbol_from_empty(&e, reader)?;
                        content.push(CreditContent::CreditSymbol(cs));
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in credit",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Credit { page, content })
}

/// Parse credit-symbol element.
fn parse_credit_symbol(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<crate::ir::score::CreditSymbol, ParseError> {
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let halign = reader
        .get_optional_attr(start.attributes(), "halign")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let valign = reader
        .get_optional_attr(start.attributes(), "valign")?
        .map(|s| values::parse_top_middle_bottom(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    let value = reader.read_text("credit-symbol")?;

    Ok(crate::ir::score::CreditSymbol {
        value,
        print_style,
        justify,
        halign,
        valign,
    })
}

/// Parse credit-symbol element from empty tag.
fn parse_credit_symbol_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<crate::ir::score::CreditSymbol, ParseError> {
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let halign = reader
        .get_optional_attr(start.attributes(), "halign")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let valign = reader
        .get_optional_attr(start.attributes(), "valign")?
        .map(|s| values::parse_top_middle_bottom(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    Ok(crate::ir::score::CreditSymbol {
        value: String::new(),
        print_style,
        justify,
        halign,
        valign,
    })
}

/// Parse credit-words element.
fn parse_credit_words(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<CreditWords, ParseError> {
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let halign = reader
        .get_optional_attr(start.attributes(), "halign")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let valign = reader
        .get_optional_attr(start.attributes(), "valign")?
        .map(|s| values::parse_top_middle_bottom(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;
    let lang = reader.get_optional_attr(start.attributes(), "xml:lang")?;

    let value = reader.read_text("credit-words")?;

    Ok(CreditWords {
        value,
        print_style,
        justify,
        halign,
        valign,
        lang,
    })
}

/// Parse credit-words element from empty tag.
fn parse_credit_words_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<CreditWords, ParseError> {
    let justify = reader
        .get_optional_attr(start.attributes(), "justify")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let halign = reader
        .get_optional_attr(start.attributes(), "halign")?
        .map(|s| values::parse_left_center_right(&s, reader.position()))
        .transpose()?;
    let valign = reader
        .get_optional_attr(start.attributes(), "valign")?
        .map(|s| values::parse_top_middle_bottom(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;
    let lang = reader.get_optional_attr(start.attributes(), "xml:lang")?;

    Ok(CreditWords {
        value: String::new(),
        print_style,
        justify,
        halign,
        valign,
        lang,
    })
}

/// Parse credit-image element.
fn parse_credit_image(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<CreditImage, ParseError> {
    let source = reader.get_attr(start.attributes(), "source", "credit-image")?;
    let r#type = reader.get_attr(start.attributes(), "type", "credit-image")?;
    let position = parse_position_attrs(start, reader)?;

    reader.skip_element("credit-image")?;

    Ok(CreditImage {
        source,
        r#type,
        position,
    })
}

/// Parse credit-image element from empty tag.
fn parse_credit_image_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<CreditImage, ParseError> {
    let source = reader.get_attr(start.attributes(), "source", "credit-image")?;
    let r#type = reader.get_attr(start.attributes(), "type", "credit-image")?;
    let position = parse_position_attrs(start, reader)?;

    Ok(CreditImage {
        source,
        r#type,
        position,
    })
}

/// Parse a defaults element containing page layout, system layout, etc.
fn parse_defaults(reader: &mut XmlReader<'_>) -> Result<Defaults, ParseError> {
    let mut scaling: Option<Scaling> = None;
    let mut page_layout: Option<PageLayout> = None;
    let mut system_layout: Option<SystemLayout> = None;
    let mut staff_layout: Vec<StaffLayout> = Vec::new();
    let mut appearance: Option<Appearance> = None;
    let mut music_font: Option<Font> = None;
    let mut word_font: Option<Font> = None;
    let mut lyric_fonts: Vec<LyricFont> = Vec::new();
    let mut lyric_languages: Vec<LyricLanguage> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "scaling" => {
                        scaling = Some(parse_scaling(reader)?);
                    }
                    "page-layout" => {
                        page_layout = Some(parse_page_layout(reader)?);
                    }
                    "system-layout" => {
                        system_layout = Some(parse_system_layout(reader)?);
                    }
                    "staff-layout" => {
                        staff_layout.push(parse_staff_layout(reader, &e)?);
                    }
                    "appearance" => {
                        appearance = Some(parse_appearance(reader)?);
                    }
                    "music-font" => {
                        music_font = Some(parse_font_element(reader, &e)?);
                    }
                    "word-font" => {
                        word_font = Some(parse_font_element(reader, &e)?);
                    }
                    "lyric-font" => {
                        lyric_fonts.push(parse_lyric_font(reader, &e)?);
                    }
                    "lyric-language" => {
                        lyric_languages.push(parse_lyric_language(reader, &e)?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "music-font" => {
                        music_font = Some(parse_font_attrs(&e, reader)?);
                    }
                    "word-font" => {
                        word_font = Some(parse_font_attrs(&e, reader)?);
                    }
                    "lyric-font" => {
                        lyric_fonts.push(parse_lyric_font_from_empty(&e, reader)?);
                    }
                    "lyric-language" => {
                        lyric_languages.push(parse_lyric_language_from_empty(&e, reader)?);
                    }
                    "staff-layout" => {
                        staff_layout.push(parse_staff_layout_from_empty(&e, reader)?);
                    }
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in defaults",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Defaults {
        scaling,
        page_layout,
        system_layout,
        staff_layout,
        appearance,
        music_font,
        word_font,
        lyric_fonts,
        lyric_languages,
    })
}

/// Parse a scaling element.
fn parse_scaling(reader: &mut XmlReader<'_>) -> Result<Scaling, ParseError> {
    let mut millimeters: Option<f64> = None;
    let mut tenths: Option<f64> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "millimeters" => {
                        millimeters = Some(reader.read_text_as("millimeters")?);
                    }
                    "tenths" => {
                        tenths = Some(reader.read_text_as("tenths")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in scaling",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Scaling {
        millimeters: millimeters.unwrap_or(7.2),
        tenths: tenths.unwrap_or(40.0),
    })
}

/// Parse a page-layout element.
fn parse_page_layout(reader: &mut XmlReader<'_>) -> Result<PageLayout, ParseError> {
    let mut page_height: Option<f64> = None;
    let mut page_width: Option<f64> = None;
    let mut page_margins: Vec<PageMargins> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "page-height" => {
                        page_height = Some(reader.read_text_as("page-height")?);
                    }
                    "page-width" => {
                        page_width = Some(reader.read_text_as("page-width")?);
                    }
                    "page-margins" => {
                        page_margins.push(parse_page_margins(reader, &e)?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in page-layout",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(PageLayout {
        page_height,
        page_width,
        page_margins,
    })
}

/// Parse a page-margins element.
fn parse_page_margins(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<PageMargins, ParseError> {
    let r#type = reader
        .get_optional_attr(start.attributes(), "type")?
        .map(|s| values::parse_margin_type(&s, reader.position()))
        .transpose()?;

    let mut left_margin: Option<f64> = None;
    let mut right_margin: Option<f64> = None;
    let mut top_margin: Option<f64> = None;
    let mut bottom_margin: Option<f64> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "left-margin" => {
                        left_margin = Some(reader.read_text_as("left-margin")?);
                    }
                    "right-margin" => {
                        right_margin = Some(reader.read_text_as("right-margin")?);
                    }
                    "top-margin" => {
                        top_margin = Some(reader.read_text_as("top-margin")?);
                    }
                    "bottom-margin" => {
                        bottom_margin = Some(reader.read_text_as("bottom-margin")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in page-margins",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(PageMargins {
        r#type,
        left: left_margin.unwrap_or(0.0),
        right: right_margin.unwrap_or(0.0),
        top: top_margin.unwrap_or(0.0),
        bottom: bottom_margin.unwrap_or(0.0),
    })
}

/// Parse a system-layout element.
fn parse_system_layout(reader: &mut XmlReader<'_>) -> Result<SystemLayout, ParseError> {
    let mut system_margins: Option<SystemMargins> = None;
    let mut system_distance: Option<f64> = None;
    let mut top_system_distance: Option<f64> = None;
    let mut system_dividers: Option<SystemDividers> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "system-margins" => {
                        system_margins = Some(parse_system_margins(reader)?);
                    }
                    "system-distance" => {
                        system_distance = Some(reader.read_text_as("system-distance")?);
                    }
                    "top-system-distance" => {
                        top_system_distance = Some(reader.read_text_as("top-system-distance")?);
                    }
                    "system-dividers" => {
                        system_dividers = Some(parse_system_dividers(reader)?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in system-layout",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(SystemLayout {
        system_margins,
        system_distance,
        top_system_distance,
        system_dividers,
    })
}

/// Parse a system-margins element.
fn parse_system_margins(reader: &mut XmlReader<'_>) -> Result<SystemMargins, ParseError> {
    let mut left_margin: Option<f64> = None;
    let mut right_margin: Option<f64> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "left-margin" => {
                        left_margin = Some(reader.read_text_as("left-margin")?);
                    }
                    "right-margin" => {
                        right_margin = Some(reader.read_text_as("right-margin")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in system-margins",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(SystemMargins {
        left: left_margin.unwrap_or(0.0),
        right: right_margin.unwrap_or(0.0),
    })
}

/// Parse a system-dividers element.
fn parse_system_dividers(reader: &mut XmlReader<'_>) -> Result<SystemDividers, ParseError> {
    let mut left_divider: Option<Divider> = None;
    let mut right_divider: Option<Divider> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "left-divider" => {
                        left_divider = Some(parse_divider(reader, &e)?);
                    }
                    "right-divider" => {
                        right_divider = Some(parse_divider(reader, &e)?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::Empty(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "left-divider" => left_divider = Some(parse_divider_from_empty(&e, reader)?),
                    "right-divider" => right_divider = Some(parse_divider_from_empty(&e, reader)?),
                    _ => {}
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in system-dividers",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(SystemDividers {
        left_divider,
        right_divider,
    })
}

/// Parse a divider element.
fn parse_divider(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Divider, ParseError> {
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    reader.skip_element("divider")?;

    Ok(Divider {
        print_object,
        print_style,
    })
}

/// Parse a divider element from empty tag.
fn parse_divider_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<Divider, ParseError> {
    let print_object = reader
        .get_optional_attr(start.attributes(), "print-object")?
        .map(|s| values::parse_yes_no(&s, reader.position()))
        .transpose()?;
    let print_style = parse_print_style_attrs(start, reader)?;

    Ok(Divider {
        print_object,
        print_style,
    })
}

/// Parse a staff-layout element.
fn parse_staff_layout(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<StaffLayout, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;
    let mut staff_distance: Option<f64> = None;

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "staff-distance" => {
                        staff_distance = Some(reader.read_text_as("staff-distance")?);
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in staff-layout",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(StaffLayout {
        number,
        staff_distance,
    })
}

/// Parse a staff-layout element from empty tag.
fn parse_staff_layout_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<StaffLayout, ParseError> {
    let number = reader.get_optional_attr_as::<u16>(start.attributes(), "number")?;

    Ok(StaffLayout {
        number,
        staff_distance: None,
    })
}

/// Parse an appearance element.
fn parse_appearance(reader: &mut XmlReader<'_>) -> Result<Appearance, ParseError> {
    use crate::ir::score::OtherAppearance;

    let mut line_widths: Vec<LineWidth> = Vec::new();
    let mut note_sizes: Vec<NoteSize> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();
    let mut other_appearances: Vec<OtherAppearance> = Vec::new();

    loop {
        let event = reader.next_event()?;
        match event {
            Event::Start(e) => {
                let name = element_name(&e);
                match name.as_str() {
                    "line-width" => {
                        let r#type = reader.get_attr(e.attributes(), "type", "line-width")?;
                        let value: f64 = reader.read_text_as("line-width")?;
                        line_widths.push(LineWidth { r#type, value });
                    }
                    "note-size" => {
                        let type_str = reader.get_attr(e.attributes(), "type", "note-size")?;
                        let r#type = values::parse_note_size_type(&type_str, reader.position())?;
                        let value: f64 = reader.read_text_as("note-size")?;
                        note_sizes.push(NoteSize { r#type, value });
                    }
                    "distance" => {
                        let r#type = reader.get_attr(e.attributes(), "type", "distance")?;
                        let value: f64 = reader.read_text_as("distance")?;
                        distances.push(Distance { r#type, value });
                    }
                    "other-appearance" => {
                        let r#type = reader.get_attr(e.attributes(), "type", "other-appearance")?;
                        let value = reader.read_text("other-appearance")?;
                        other_appearances.push(OtherAppearance { r#type, value });
                    }
                    _ => {
                        reader.skip_element(&name)?;
                    }
                }
            }
            Event::End(_) => break,
            Event::Eof => {
                return Err(ParseError::xml(
                    "unexpected EOF in appearance",
                    reader.position(),
                ));
            }
            _ => {}
        }
    }

    Ok(Appearance {
        line_widths,
        note_sizes,
        distances,
        other_appearances,
    })
}

/// Parse a font element (music-font, word-font).
fn parse_font_element(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<Font, ParseError> {
    let font = parse_font_attrs(start, reader)?;
    reader.skip_element("font")?;
    Ok(font)
}

/// Parse a lyric-font element.
fn parse_lyric_font(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<LyricFont, ParseError> {
    let number = reader.get_optional_attr(start.attributes(), "number")?;
    let name = reader.get_optional_attr(start.attributes(), "name")?;
    let font = parse_font_attrs(start, reader)?;

    reader.skip_element("lyric-font")?;

    Ok(LyricFont { number, name, font })
}

/// Parse a lyric-font element from empty tag.
fn parse_lyric_font_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<LyricFont, ParseError> {
    let number = reader.get_optional_attr(start.attributes(), "number")?;
    let name = reader.get_optional_attr(start.attributes(), "name")?;
    let font = parse_font_attrs(start, reader)?;

    Ok(LyricFont { number, name, font })
}

/// Parse a lyric-language element.
fn parse_lyric_language(
    reader: &mut XmlReader<'_>,
    start: &quick_xml::events::BytesStart<'_>,
) -> Result<LyricLanguage, ParseError> {
    let number = reader.get_optional_attr(start.attributes(), "number")?;
    let name = reader.get_optional_attr(start.attributes(), "name")?;
    let lang = reader.get_attr(start.attributes(), "xml:lang", "lyric-language")?;

    reader.skip_element("lyric-language")?;

    Ok(LyricLanguage { number, name, lang })
}

/// Parse a lyric-language element from empty tag.
fn parse_lyric_language_from_empty(
    start: &quick_xml::events::BytesStart<'_>,
    reader: &XmlReader<'_>,
) -> Result<LyricLanguage, ParseError> {
    let number = reader.get_optional_attr(start.attributes(), "number")?;
    let name = reader.get_optional_attr(start.attributes(), "name")?;
    let lang = reader.get_attr(start.attributes(), "xml:lang", "lyric-language")?;

    Ok(LyricLanguage { number, name, lang })
}

#[cfg(test)]
mod tests;
