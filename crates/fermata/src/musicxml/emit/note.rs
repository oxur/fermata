//! Note emission functions for MusicXML.
//!
//! This module handles the emission of note elements including pitch, rest,
//! grace notes, accidentals, beams, stems, noteheads, and related elements.

use crate::ir::beam::{Beam, Notehead, Stem};
use crate::ir::duration::TimeModification;
use crate::ir::note::{
    Accidental, FullNote, Grace, Note, NoteContent, PitchRestUnpitched, Rest, Tie,
};
use crate::ir::pitch::{Pitch, Unpitched};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::helpers::{
    accidental_value_to_string, beam_value_to_string, fan_to_string, note_type_value_to_string,
    notehead_value_to_string, start_stop_to_string, stem_value_to_string, step_to_string,
    yes_no_to_string,
};
use super::notation::emit_notations;

/// Emit a note element.
///
/// Elements are emitted in XSD order:
/// 1. grace OR cue OR (chord?, pitch/rest/unpitched)
/// 2. duration (for regular and cue notes)
/// 3. tie*
/// 4. instrument*
/// 5. editorial-voice (footnote, level, voice)
/// 6. type
/// 7. dot*
/// 8. accidental
/// 9. time-modification
/// 10. stem
/// 11. notehead
/// 12. staff
/// 13. beam* (0-8)
/// 14. notations*
/// 15. lyric*
pub(crate) fn emit_note(w: &mut XmlWriter, note: &Note) -> Result<(), EmitError> {
    w.start_element("note")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Handle the three content variants
    match &note.content {
        NoteContent::Grace {
            grace,
            full_note,
            ties,
        } => {
            emit_grace(w, grace)?;
            emit_full_note(w, full_note)?;
            for tie in ties {
                emit_tie(w, tie)?;
            }
        }
        NoteContent::Cue {
            full_note,
            duration,
        } => {
            w.empty_element("cue")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            emit_full_note(w, full_note)?;
            w.text_element("duration", &duration.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        NoteContent::Regular {
            full_note,
            duration,
            ties,
        } => {
            emit_full_note(w, full_note)?;
            w.text_element("duration", &duration.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            for tie in ties {
                emit_tie(w, tie)?;
            }
        }
    }

    // instrument*
    for inst in &note.instrument {
        let elem = ElementBuilder::new("instrument").attr("id", &inst.id);
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // voice
    if let Some(ref voice) = note.voice {
        w.text_element("voice", voice)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // type
    if let Some(ref note_type) = note.r#type {
        let elem = ElementBuilder::new("type");
        // size attribute if present
        if let Some(ref _size) = note_type.size {
            // size attribute would go here if needed
        }
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(note_type_value_to_string(&note_type.value))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("type")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // dot*
    for _dot in &note.dots {
        w.empty_element("dot")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // accidental
    if let Some(ref acc) = note.accidental {
        emit_accidental(w, acc)?;
    }

    // time-modification
    if let Some(ref tm) = note.time_modification {
        emit_time_modification(w, tm)?;
    }

    // stem
    if let Some(ref stem) = note.stem {
        emit_stem(w, stem)?;
    }

    // notehead
    if let Some(ref notehead) = note.notehead {
        emit_notehead(w, notehead)?;
    }

    // staff
    if let Some(staff) = note.staff {
        w.text_element("staff", &staff.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // beam* (up to 8 levels)
    for beam in &note.beams {
        emit_beam(w, beam)?;
    }

    // notations* - emit each Notations container
    for notations in &note.notations {
        emit_notations(w, notations)?;
    }

    // lyric* - skipped for now (Milestone 5)

    w.end_element("note")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a grace element.
pub(crate) fn emit_grace(w: &mut XmlWriter, grace: &Grace) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("grace");

    if let Some(ref slash) = grace.slash {
        elem = elem.attr("slash", yes_no_to_string(slash));
    }
    if let Some(stp) = grace.steal_time_previous {
        elem = elem.attr("steal-time-previous", &stp.to_string());
    }
    if let Some(stf) = grace.steal_time_following {
        elem = elem.attr("steal-time-following", &stf.to_string());
    }
    if let Some(mt) = grace.make_time {
        elem = elem.attr("make-time", &mt.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the full note content (chord flag + pitch/rest/unpitched).
pub(crate) fn emit_full_note(w: &mut XmlWriter, full_note: &FullNote) -> Result<(), EmitError> {
    // chord flag
    if full_note.chord {
        w.empty_element("chord")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // pitch, rest, or unpitched
    match &full_note.content {
        PitchRestUnpitched::Pitch(pitch) => emit_pitch(w, pitch)?,
        PitchRestUnpitched::Rest(rest) => emit_rest(w, rest)?,
        PitchRestUnpitched::Unpitched(unpitched) => emit_unpitched(w, unpitched)?,
    }

    Ok(())
}

/// Emit a pitch element.
pub(crate) fn emit_pitch(w: &mut XmlWriter, pitch: &Pitch) -> Result<(), EmitError> {
    w.start_element("pitch")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // step (required)
    w.text_element("step", step_to_string(&pitch.step))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // alter
    if let Some(alter) = pitch.alter {
        w.text_element("alter", &alter.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // octave (required)
    w.text_element("octave", &pitch.octave.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.end_element("pitch")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a rest element.
pub(crate) fn emit_rest(w: &mut XmlWriter, rest: &Rest) -> Result<(), EmitError> {
    let has_content = rest.display_step.is_some() || rest.display_octave.is_some();

    let mut elem = ElementBuilder::new("rest");
    if let Some(ref measure) = rest.measure {
        elem = elem.attr("measure", yes_no_to_string(measure));
    }

    if has_content {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        if let Some(ref step) = rest.display_step {
            w.text_element("display-step", step_to_string(step))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if let Some(octave) = rest.display_octave {
            w.text_element("display-octave", &octave.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        w.end_element("rest")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    Ok(())
}

/// Emit an unpitched element.
pub(crate) fn emit_unpitched(w: &mut XmlWriter, unpitched: &Unpitched) -> Result<(), EmitError> {
    let has_content = unpitched.display_step.is_some() || unpitched.display_octave.is_some();

    if has_content {
        w.start_element("unpitched")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        if let Some(ref step) = unpitched.display_step {
            w.text_element("display-step", step_to_string(step))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if let Some(octave) = unpitched.display_octave {
            w.text_element("display-octave", &octave.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        w.end_element("unpitched")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element("unpitched")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    Ok(())
}

/// Emit a tie element (playback, not visual).
pub(crate) fn emit_tie(w: &mut XmlWriter, tie: &Tie) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("tie").attr("type", start_stop_to_string(&tie.r#type));
    if let Some(ref time_only) = tie.time_only {
        elem = elem.attr("time-only", time_only);
    }
    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an accidental element.
pub(crate) fn emit_accidental(w: &mut XmlWriter, acc: &Accidental) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("accidental");
    if let Some(ref cautionary) = acc.cautionary {
        elem = elem.attr("cautionary", yes_no_to_string(cautionary));
    }
    if let Some(ref editorial) = acc.editorial {
        elem = elem.attr("editorial", yes_no_to_string(editorial));
    }
    if let Some(ref parentheses) = acc.parentheses {
        elem = elem.attr("parentheses", yes_no_to_string(parentheses));
    }
    if let Some(ref bracket) = acc.bracket {
        elem = elem.attr("bracket", yes_no_to_string(bracket));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(accidental_value_to_string(&acc.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("accidental")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a time-modification element (for tuplets).
pub(crate) fn emit_time_modification(
    w: &mut XmlWriter,
    tm: &TimeModification,
) -> Result<(), EmitError> {
    w.start_element("time-modification")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // actual-notes (required)
    w.text_element("actual-notes", &tm.actual_notes.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // normal-notes (required)
    w.text_element("normal-notes", &tm.normal_notes.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // normal-type
    if let Some(ref nt) = tm.normal_type {
        w.text_element("normal-type", note_type_value_to_string(nt))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // normal-dot*
    for _ in 0..tm.normal_dots {
        w.empty_element("normal-dot")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("time-modification")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a stem element.
pub(crate) fn emit_stem(w: &mut XmlWriter, stem: &Stem) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("stem");
    if let Some(dy) = stem.default_y {
        elem = elem.attr("default-y", &dy.to_string());
    }
    if let Some(ref color) = stem.color {
        elem = elem.attr("color", color);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(stem_value_to_string(&stem.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("stem")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a notehead element.
pub(crate) fn emit_notehead(w: &mut XmlWriter, notehead: &Notehead) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("notehead");
    if let Some(ref filled) = notehead.filled {
        elem = elem.attr("filled", yes_no_to_string(filled));
    }
    if let Some(ref parentheses) = notehead.parentheses {
        elem = elem.attr("parentheses", yes_no_to_string(parentheses));
    }
    if let Some(ref color) = notehead.color {
        elem = elem.attr("color", color);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(notehead_value_to_string(&notehead.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("notehead")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a beam element.
pub(crate) fn emit_beam(w: &mut XmlWriter, beam: &Beam) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("beam").attr("number", &beam.number.to_string());
    if let Some(ref fan) = beam.fan {
        elem = elem.attr("fan", fan_to_string(fan));
    }
    if let Some(ref color) = beam.color {
        elem = elem.attr("color", color);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(beam_value_to_string(&beam.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("beam")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::beam::{BeamValue, NoteheadValue, StemValue};
    use crate::ir::common::{AccidentalValue, Position, StartStop, YesNo};
    use crate::ir::duration::{Dot, NoteType, NoteTypeValue};
    use crate::ir::pitch::Step;

    #[test]
    fn test_emit_note_c4_quarter() {
        let mut w = XmlWriter::new();
        let note = Note {
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
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<note>"));
        assert!(xml.contains("<pitch>"));
        assert!(xml.contains("<step>C</step>"));
        assert!(xml.contains("<octave>4</octave>"));
        assert!(xml.contains("</pitch>"));
        assert!(xml.contains("<duration>4</duration>"));
        assert!(xml.contains("<voice>1</voice>"));
        assert!(xml.contains("<type>quarter</type>"));
        assert!(xml.contains("</note>"));
    }

    #[test]
    fn test_emit_note_with_accidental() {
        let mut w = XmlWriter::new();
        let note = Note {
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
                        alter: Some(1.0),
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
            accidental: Some(Accidental {
                value: AccidentalValue::Sharp,
                cautionary: None,
                editorial: None,
                parentheses: None,
                bracket: None,
                size: None,
            }),
            time_modification: None,
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<step>F</step>"));
        assert!(xml.contains("<alter>1</alter>"));
        assert!(xml.contains("<accidental>sharp</accidental>"));
    }

    #[test]
    fn test_emit_note_with_tie() {
        let mut w = XmlWriter::new();
        let note = Note {
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
                ties: vec![Tie {
                    r#type: StartStop::Start,
                    time_only: None,
                }],
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
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tie type=\"start\"/>"));
    }

    #[test]
    fn test_emit_note_with_triplet_time_modification() {
        let mut w = XmlWriter::new();
        let note = Note {
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
                duration: 2,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: Some(NoteType {
                value: NoteTypeValue::Eighth,
                size: None,
            }),
            dots: vec![],
            accidental: None,
            time_modification: Some(TimeModification {
                actual_notes: 3,
                normal_notes: 2,
                normal_type: Some(NoteTypeValue::Eighth),
                normal_dots: 0,
            }),
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<time-modification>"));
        assert!(xml.contains("<actual-notes>3</actual-notes>"));
        assert!(xml.contains("<normal-notes>2</normal-notes>"));
        assert!(xml.contains("<normal-type>eighth</normal-type>"));
        assert!(xml.contains("</time-modification>"));
    }

    #[test]
    fn test_emit_note_with_beam() {
        let mut w = XmlWriter::new();
        let note = Note {
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
                duration: 2,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: Some(NoteType {
                value: NoteTypeValue::Eighth,
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
            beams: vec![Beam {
                value: BeamValue::Begin,
                number: 1,
                fan: None,
                color: None,
            }],
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<stem>up</stem>"));
        assert!(xml.contains("<beam number=\"1\">begin</beam>"));
    }

    #[test]
    fn test_emit_note_with_notehead() {
        let mut w = XmlWriter::new();
        let note = Note {
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
            notehead: Some(Notehead {
                value: NoteheadValue::Diamond,
                filled: Some(YesNo::Yes),
                parentheses: None,
                font: crate::ir::common::Font::default(),
                color: None,
            }),
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<notehead filled=\"yes\">diamond</notehead>"));
    }

    #[test]
    fn test_emit_rest() {
        let mut w = XmlWriter::new();
        let note = Note {
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
                    content: PitchRestUnpitched::Rest(Rest {
                        measure: None,
                        display_step: None,
                        display_octave: None,
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
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<rest/>"));
        assert!(xml.contains("<duration>4</duration>"));
        assert!(xml.contains("<type>quarter</type>"));
    }

    #[test]
    fn test_emit_dotted_note() {
        let mut w = XmlWriter::new();
        let note = Note {
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
                duration: 6,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
            dots: vec![Dot::default()],
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<type>quarter</type>"));
        assert!(xml.contains("<dot/>"));
    }

    #[test]
    fn test_emit_grace_note() {
        let mut w = XmlWriter::new();
        let note = Note {
            position: Position::default(),
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            print_object: None,
            content: NoteContent::Grace {
                grace: Grace {
                    steal_time_previous: None,
                    steal_time_following: None,
                    make_time: None,
                    slash: Some(YesNo::Yes),
                },
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::D,
                        alter: None,
                        octave: 4,
                    }),
                },
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: Some(NoteType {
                value: NoteTypeValue::Eighth,
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
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<grace slash=\"yes\"/>"));
        assert!(xml.contains("<step>D</step>"));
        // Grace notes have no duration
        assert!(!xml.contains("<duration>"));
    }

    #[test]
    fn test_emit_pitch() {
        let mut w = XmlWriter::new();
        let pitch = Pitch {
            step: Step::G,
            alter: Some(-1.0),
            octave: 5,
        };

        emit_pitch(&mut w, &pitch).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pitch>"));
        assert!(xml.contains("<step>G</step>"));
        assert!(xml.contains("<alter>-1</alter>"));
        assert!(xml.contains("<octave>5</octave>"));
        assert!(xml.contains("</pitch>"));
    }

    #[test]
    fn test_emit_rest_with_display() {
        let mut w = XmlWriter::new();
        let rest = Rest {
            measure: None,
            display_step: Some(Step::B),
            display_octave: Some(4),
        };

        emit_rest(&mut w, &rest).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<rest>"));
        assert!(xml.contains("<display-step>B</display-step>"));
        assert!(xml.contains("<display-octave>4</display-octave>"));
        assert!(xml.contains("</rest>"));
    }
}
