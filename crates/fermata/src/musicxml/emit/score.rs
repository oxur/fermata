//! Score-level emission functions for MusicXML.
//!
//! This module contains the main entry point `emit_score` and functions for
//! emitting score headers, part lists, parts, and measures.

use crate::ir::part::{PartGroup, PartList, PartListElement, ScorePart};
use crate::ir::{Measure, MusicDataElement, Part, ScorePartwise};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::attributes::emit_attributes;
use super::barline::emit_barline;
use super::direction::emit_direction;
use super::note::emit_note;
use super::voice::{emit_backup, emit_forward};

/// Emit a complete MusicXML document from a ScorePartwise.
///
/// This function generates a complete MusicXML 4.0 partwise document including
/// the XML declaration, DOCTYPE, and all score content.
///
/// # Arguments
///
/// * `score` - The ScorePartwise IR to emit
///
/// # Returns
///
/// A `Result` containing the complete XML string or an `EmitError`
pub fn emit_score(score: &ScorePartwise) -> Result<String, EmitError> {
    let mut w = XmlWriter::new();

    w.write_header()
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // <score-partwise version="4.0">
    let mut root = ElementBuilder::new("score-partwise");
    if let Some(ref v) = score.version {
        root = root.attr("version", v);
    }
    w.write_start(root)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Score header elements (work, identification, defaults, credits, part-list)
    emit_score_header(&mut w, score)?;

    // Parts
    for part in &score.parts {
        emit_part(&mut w, part)?;
    }

    w.end_element("score-partwise")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.into_string()
        .map_err(|e| EmitError::XmlWrite(e.to_string()))
}

/// Emit the score header elements.
///
/// This includes work, movement-number, movement-title, identification,
/// defaults, credits, and part-list.
pub(crate) fn emit_score_header(w: &mut XmlWriter, score: &ScorePartwise) -> Result<(), EmitError> {
    // TODO: work
    // TODO: movement-number
    // TODO: movement-title
    // TODO: identification
    // TODO: defaults
    // TODO: credits

    emit_part_list(w, &score.part_list)?;
    Ok(())
}

/// Emit the part-list element.
pub(crate) fn emit_part_list(w: &mut XmlWriter, part_list: &PartList) -> Result<(), EmitError> {
    w.start_element("part-list")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for element in &part_list.content {
        match element {
            PartListElement::ScorePart(sp) => emit_score_part(w, sp)?,
            PartListElement::PartGroup(pg) => emit_part_group(w, pg)?,
        }
    }

    w.end_element("part-list")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a score-part element.
pub(crate) fn emit_score_part(w: &mut XmlWriter, sp: &ScorePart) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("score-part").attr("id", &sp.id);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // part-name is required
    w.text_element("part-name", &sp.part_name.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // TODO: part-name-display
    // TODO: part-abbreviation
    // TODO: part-abbreviation-display
    // TODO: group
    // TODO: score-instrument
    // TODO: midi-device
    // TODO: midi-instrument

    w.end_element("score-part")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a part-group element (stub).
pub(crate) fn emit_part_group(w: &mut XmlWriter, _pg: &PartGroup) -> Result<(), EmitError> {
    // TODO: implement part-group emission
    // For now, this is a stub that does nothing
    let _ = w;
    Ok(())
}

/// Emit a part element.
pub(crate) fn emit_part(w: &mut XmlWriter, part: &Part) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("part").attr("id", &part.id);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for measure in &part.measures {
        emit_measure(w, measure)?;
    }

    w.end_element("part")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a measure element.
pub(crate) fn emit_measure(w: &mut XmlWriter, measure: &Measure) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("measure").attr("number", &measure.number);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for element in &measure.content {
        emit_music_data(w, element)?;
    }

    w.end_element("measure")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a music data element.
///
/// This handles all variants of the MusicDataElement enum:
/// - Note
/// - Backup
/// - Forward
/// - Direction
/// - Attributes
/// - Barline
pub(crate) fn emit_music_data(
    w: &mut XmlWriter,
    element: &MusicDataElement,
) -> Result<(), EmitError> {
    match element {
        MusicDataElement::Note(note) => emit_note(w, note),
        MusicDataElement::Backup(backup) => emit_backup(w, backup),
        MusicDataElement::Forward(forward) => emit_forward(w, forward),
        MusicDataElement::Direction(dir) => emit_direction(w, dir),
        MusicDataElement::Attributes(attrs) => emit_attributes(w, attrs),
        MusicDataElement::Barline(barline) => emit_barline(w, barline),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::PrintStyle;
    use crate::ir::attributes::{
        Attributes, Clef, ClefSign, Key, KeyContent, Mode, Time, TimeContent, TimeSignature,
        TraditionalKey,
    };
    use crate::ir::common::Editorial;
    use crate::ir::part::PartName;

    fn create_minimal_score() -> ScorePartwise {
        ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList {
                content: vec![PartListElement::ScorePart(ScorePart {
                    id: "P1".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Test Part".to_string(),
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
                })],
            },
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![Measure {
                    number: "1".to_string(),
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    content: vec![],
                }],
            }],
        }
    }

    #[test]
    fn test_emit_score_structure() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        // Check XML declaration
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

        // Check DOCTYPE
        assert!(xml.contains("<!DOCTYPE score-partwise"));

        // Check root element
        assert!(xml.contains("<score-partwise version=\"4.0\">"));
        assert!(xml.contains("</score-partwise>"));
    }

    #[test]
    fn test_emit_score_part_list() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<part-list>"));
        assert!(xml.contains("</part-list>"));
    }

    #[test]
    fn test_emit_score_part() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<score-part id=\"P1\">"));
        assert!(xml.contains("<part-name>Test Part</part-name>"));
        assert!(xml.contains("</score-part>"));
    }

    #[test]
    fn test_emit_part() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<part id=\"P1\">"));
        assert!(xml.contains("</part>"));
    }

    #[test]
    fn test_emit_measure() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("</measure>"));
    }

    #[test]
    fn test_emit_score_without_version() {
        let mut score = create_minimal_score();
        score.version = None;
        let xml = emit_score(&score).unwrap();

        // Should have <score-partwise> without version attribute
        assert!(xml.contains("<score-partwise>"));
        // Verify no version attribute on score-partwise (but version= exists in XML declaration)
        assert!(!xml.contains("<score-partwise version="));
    }

    #[test]
    fn test_emit_multiple_measures() {
        let mut score = create_minimal_score();
        score.parts[0].measures.push(Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("<measure number=\"2\">"));
    }

    #[test]
    fn test_emit_multiple_parts() {
        let mut score = create_minimal_score();

        // Add second part to part-list
        score
            .part_list
            .content
            .push(PartListElement::ScorePart(ScorePart {
                id: "P2".to_string(),
                identification: None,
                part_name: PartName {
                    value: "Second Part".to_string(),
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
            }));

        // Add second part
        score.parts.push(Part {
            id: "P2".to_string(),
            measures: vec![Measure {
                number: "1".to_string(),
                implicit: None,
                non_controlling: None,
                width: None,
                content: vec![],
            }],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<score-part id=\"P1\">"));
        assert!(xml.contains("<score-part id=\"P2\">"));
        assert!(xml.contains("<part id=\"P1\">"));
        assert!(xml.contains("<part id=\"P2\">"));
    }

    #[test]
    fn test_emit_music_data_with_empty_content() {
        // Test that empty measure content works
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        // Should have measure tags but no content between them
        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("</measure>"));
    }

    #[test]
    fn test_emit_score_with_attributes() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
                        mode: Some(Mode::Major),
                    }),
                    number: None,
                    print_object: None,
                }],
                times: vec![Time {
                    content: TimeContent::Measured {
                        signatures: vec![TimeSignature {
                            beats: "4".to_string(),
                            beat_type: "4".to_string(),
                        }],
                    },
                    number: None,
                    symbol: None,
                    print_object: None,
                }],
                staves: None,
                part_symbol: None,
                instruments: None,
                clefs: vec![Clef {
                    sign: ClefSign::G,
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<attributes>"));
        assert!(xml.contains("<divisions>4</divisions>"));
        assert!(xml.contains("<fifths>0</fifths>"));
        assert!(xml.contains("<mode>major</mode>"));
        assert!(xml.contains("</attributes>"));
    }

    // =======================================================================
    // Integration Tests: Full Score Scenarios
    // =======================================================================

    #[test]
    fn test_emit_twinkle_twinkle_first_phrase() {
        use crate::ir::common::Position;
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        // "Twinkle Twinkle Little Star" first phrase: C C G G A A G (half)
        let mut score = create_minimal_score();

        // Add attributes
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
                        mode: Some(Mode::Major),
                    }),
                    number: None,
                    print_object: None,
                }],
                times: vec![Time {
                    content: TimeContent::Measured {
                        signatures: vec![TimeSignature {
                            beats: "4".to_string(),
                            beat_type: "4".to_string(),
                        }],
                    },
                    number: None,
                    symbol: None,
                    print_object: None,
                }],
                staves: None,
                part_symbol: None,
                instruments: None,
                clefs: vec![Clef {
                    sign: ClefSign::G,
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        // Helper to create a quarter note
        let make_quarter = |step: Step| -> MusicDataElement {
            MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            }))
        };

        // Helper to create a half note
        let make_half = |step: Step| -> MusicDataElement {
            MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            }))
        };

        // Measure 1: C C G G
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::C));
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::C));
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::G));
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::G));

        // Measure 2: A A G (half)
        score.parts[0].measures.push(Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                make_quarter(Step::A),
                make_quarter(Step::A),
                make_half(Step::G),
            ],
        });

        let xml = emit_score(&score).unwrap();

        // Verify structure
        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("<measure number=\"2\">"));
        assert!(xml.contains("<divisions>4</divisions>"));
        assert!(xml.contains("<fifths>0</fifths>"));
        assert!(xml.contains("<mode>major</mode>"));
        assert!(xml.contains("<sign>G</sign>"));

        // Verify notes (should have C, C, G, G in measure 1)
        let c_count = xml.matches("<step>C</step>").count();
        let g_count = xml.matches("<step>G</step>").count();
        let a_count = xml.matches("<step>A</step>").count();

        assert_eq!(c_count, 2, "Should have 2 C notes");
        assert_eq!(g_count, 3, "Should have 3 G notes (2 quarters + 1 half)");
        assert_eq!(a_count, 2, "Should have 2 A notes");

        // Verify we have quarter and half notes
        assert!(xml.contains("<type>quarter</type>"));
        assert!(xml.contains("<type>half</type>"));
    }

    #[test]
    fn test_emit_multi_voice_with_backup() {
        use crate::ir::beam::{Stem, StemValue};
        use crate::ir::common::Position;
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};
        use crate::ir::voice::Backup;

        // Two voices: Voice 1 has C4 half, D4 half; Voice 2 has E3 half, F3 half
        let mut score = create_minimal_score();

        // Add attributes
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
                        mode: Some(Mode::Major),
                    }),
                    number: None,
                    print_object: None,
                }],
                times: vec![Time {
                    content: TimeContent::Measured {
                        signatures: vec![TimeSignature {
                            beats: "4".to_string(),
                            beat_type: "4".to_string(),
                        }],
                    },
                    number: None,
                    symbol: None,
                    print_object: None,
                }],
                staves: None,
                part_symbol: None,
                instruments: None,
                clefs: vec![Clef {
                    sign: ClefSign::G,
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        // Voice 1: C4 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Up,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Voice 1: D4 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::D,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Up,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Backup to start of measure for voice 2
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Backup(Backup {
                duration: 16, // Full measure
                editorial: Editorial::default(),
            }));

        // Voice 2: E3 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::E,
                            alter: None,
                            octave: 3,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("2".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Down,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Voice 2: F3 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::F,
                            alter: None,
                            octave: 3,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("2".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Down,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify two voices
        assert!(xml.contains("<voice>1</voice>"));
        assert!(xml.contains("<voice>2</voice>"));

        // Verify backup
        assert!(xml.contains("<backup>"));
        assert!(xml.contains("<duration>16</duration>"));

        // Verify stem directions
        assert!(xml.contains("<stem>up</stem>"));
        assert!(xml.contains("<stem>down</stem>"));

        // Verify all pitches are present
        assert!(xml.contains("<step>C</step>"));
        assert!(xml.contains("<step>D</step>"));
        assert!(xml.contains("<step>E</step>"));
        assert!(xml.contains("<step>F</step>"));
    }

    #[test]
    fn test_emit_repeat_with_volta_brackets() {
        use crate::ir::attributes::{BarStyle, Barline, Ending, Repeat};
        use crate::ir::common::{BackwardForward, Position, RightLeftMiddle, StartStopDiscontinue};
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();

        // Measure 1: Start repeat
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Barline(Box::new(Barline {
                location: Some(RightLeftMiddle::Left),
                bar_style: Some(BarStyle::HeavyLight),
                editorial: Editorial::default(),
                wavy_line: None,
                segno: None,
                coda: None,
                fermatas: vec![],
                ending: None,
                repeat: Some(Repeat {
                    direction: BackwardForward::Forward,
                    times: None,
                    winged: None,
                }),
            })));

        // Add a note to measure 1
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 16,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Whole,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Measure 2: First ending with backward repeat
        score.parts[0].measures.push(Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                // First ending start
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Left),
                    bar_style: None,
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Start,
                        number: "1".to_string(),
                        text: Some("1.".to_string()),
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: None,
                })),
                // A whole note
                MusicDataElement::Note(Box::new(Note {
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
                            content: PitchRestUnpitched::Pitch(Pitch {
                                step: Step::D,
                                alter: None,
                                octave: 4,
                            }),
                        },
                        duration: 16,
                        ties: vec![],
                    },
                    instrument: vec![],
                    voice: Some("1".to_string()),
                    r#type: Some(NoteType {
                        value: NoteTypeValue::Whole,
                        size: None,
                    }),
                    dots: vec![],
                    accidental: None,
                    time_modification: None,
                    stem: None,
                    notehead: None,
                    staff: None,
                    beams: vec![],
                    notations: vec![],
                    lyrics: vec![],
                })),
                // End of first ending with backward repeat
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Right),
                    bar_style: Some(BarStyle::LightHeavy),
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Stop,
                        number: "1".to_string(),
                        text: None,
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: Some(Repeat {
                        direction: BackwardForward::Backward,
                        times: None,
                        winged: None,
                    }),
                })),
            ],
        });

        // Measure 3: Second ending
        score.parts[0].measures.push(Measure {
            number: "3".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                // Second ending start
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Left),
                    bar_style: None,
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Start,
                        number: "2".to_string(),
                        text: Some("2.".to_string()),
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: None,
                })),
                // E whole note
                MusicDataElement::Note(Box::new(Note {
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
                            content: PitchRestUnpitched::Pitch(Pitch {
                                step: Step::E,
                                alter: None,
                                octave: 4,
                            }),
                        },
                        duration: 16,
                        ties: vec![],
                    },
                    instrument: vec![],
                    voice: Some("1".to_string()),
                    r#type: Some(NoteType {
                        value: NoteTypeValue::Whole,
                        size: None,
                    }),
                    dots: vec![],
                    accidental: None,
                    time_modification: None,
                    stem: None,
                    notehead: None,
                    staff: None,
                    beams: vec![],
                    notations: vec![],
                    lyrics: vec![],
                })),
                // End of second ending (discontinue - no line at end)
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Right),
                    bar_style: Some(BarStyle::LightHeavy),
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Discontinue,
                        number: "2".to_string(),
                        text: None,
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: None,
                })),
            ],
        });

        let xml = emit_score(&score).unwrap();

        // Verify forward repeat
        assert!(xml.contains("<repeat direction=\"forward\"/>"));

        // Verify backward repeat
        assert!(xml.contains("<repeat direction=\"backward\"/>"));

        // Verify first ending
        assert!(xml.contains("<ending number=\"1\" type=\"start\">1.</ending>"));
        assert!(xml.contains("<ending number=\"1\" type=\"stop\"/>"));

        // Verify second ending
        assert!(xml.contains("<ending number=\"2\" type=\"start\">2.</ending>"));
        assert!(xml.contains("<ending number=\"2\" type=\"discontinue\"/>"));

        // Verify bar styles
        assert!(xml.contains("<bar-style>heavy-light</bar-style>"));
        assert!(xml.contains("<bar-style>light-heavy</bar-style>"));
    }

    #[test]
    fn test_emit_direction_and_notations_integration() {
        use crate::ir::common::AboveBelow;
        use crate::ir::common::{EmptyPlacement, Position, StartStopContinue};
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics, Wedge,
            WedgeType,
        };
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::notation::{
            ArticulationElement, Articulations, NotationContent, Notations, Slur,
        };
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();

        // Add attributes
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
                        mode: Some(Mode::Major),
                    }),
                    number: None,
                    print_object: None,
                }],
                times: vec![Time {
                    content: TimeContent::Measured {
                        signatures: vec![TimeSignature {
                            beats: "4".to_string(),
                            beat_type: "4".to_string(),
                        }],
                    },
                    number: None,
                    symbol: None,
                    print_object: None,
                }],
                staves: None,
                part_symbol: None,
                instruments: None,
                clefs: vec![Clef {
                    sign: ClefSign::G,
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        // Add forte direction
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![DynamicElement::F],
                        print_style: PrintStyle::default(),
                        placement: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        // Add crescendo start
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: WedgeType::Crescendo,
                        number: Some(1),
                        spread: None,
                        niente: None,
                        line_type: None,
                        position: Position::default(),
                        color: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        // Add a note with slur and staccato
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
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
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![Notations {
                    print_object: None,
                    content: vec![
                        NotationContent::Slur(Slur {
                            r#type: StartStopContinue::Start,
                            number: 1,
                            line_type: None,
                            position: Position::default(),
                            placement: Some(AboveBelow::Above),
                            orientation: None,
                            color: None,
                        }),
                        NotationContent::Articulations(Box::new(Articulations {
                            content: vec![ArticulationElement::Staccato(EmptyPlacement {
                                placement: Some(AboveBelow::Above),
                                position: Position::default(),
                            })],
                        })),
                    ],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify structure
        assert!(xml.contains("<direction placement=\"below\">"));
        assert!(xml.contains("<dynamics>"));
        assert!(xml.contains("<f/>"));
        assert!(xml.contains("<wedge type=\"crescendo\""));
        assert!(xml.contains("<notations>"));
        assert!(xml.contains("<slur type=\"start\""));
        assert!(xml.contains("<articulations>"));
        assert!(xml.contains("<staccato"));
    }
}
