//! Note emission functions for MusicXML.
//!
//! This module handles the emission of note elements including pitch, rest,
//! grace notes, accidentals, beams, stems, noteheads, and related elements.

use crate::ir::beam::{Beam, Notehead, Stem};
use crate::ir::duration::TimeModification;
use crate::ir::lyric::{Elision, Extend, Lyric, LyricContent, LyricExtension, TextElementData};
use crate::ir::note::{
    Accidental, FullNote, Grace, Note, NoteContent, PitchRestUnpitched, Rest, Tie,
};
use crate::ir::pitch::{Pitch, Unpitched};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::helpers::{
    above_below_to_string, accidental_value_to_string, beam_value_to_string, fan_to_string,
    left_center_right_to_string, note_type_value_to_string, notehead_value_to_string,
    start_stop_continue_to_string, start_stop_to_string, stem_value_to_string, step_to_string,
    syllabic_to_string, yes_no_to_string,
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

    // lyric*
    for lyric in &note.lyrics {
        emit_lyric(w, lyric)?;
    }

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

/// Emit a lyric element.
///
/// Lyrics are attached to notes and contain syllables, extending lines, or special indicators.
/// The MusicXML structure is:
/// ```xml
/// <lyric number="1" placement="below">
///   <syllabic>begin</syllabic>
///   <text>Hap</text>
///   <extend type="start"/>
/// </lyric>
/// ```
pub(crate) fn emit_lyric(w: &mut XmlWriter, lyric: &Lyric) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("lyric");

    if let Some(ref number) = lyric.number {
        elem = elem.attr("number", number);
    }
    if let Some(ref name) = lyric.name {
        elem = elem.attr("name", name);
    }
    if let Some(ref justify) = lyric.justify {
        elem = elem.attr("justify", left_center_right_to_string(justify));
    }
    if let Some(ref placement) = lyric.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref print_object) = lyric.print_object {
        elem = elem.attr("print-object", yes_no_to_string(print_object));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    match &lyric.content {
        LyricContent::Syllable {
            syllabic,
            text,
            extensions,
            extend,
        } => {
            // syllabic?
            if let Some(syl) = syllabic {
                w.text_element("syllabic", syllabic_to_string(syl))
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }

            // text
            emit_text_element_data(w, text)?;

            // (elision, syllabic?, text)*
            for ext in extensions {
                emit_lyric_extension(w, ext)?;
            }

            // extend?
            if let Some(ext) = extend {
                emit_extend(w, ext)?;
            }
        }
        LyricContent::ExtendOnly(extend) => {
            emit_extend(w, extend)?;
        }
        LyricContent::Laughing => {
            w.empty_element("laughing")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        LyricContent::Humming => {
            w.empty_element("humming")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
    }

    // end-line?
    if lyric.end_line {
        w.empty_element("end-line")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // end-paragraph?
    if lyric.end_paragraph {
        w.empty_element("end-paragraph")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("lyric")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a text element for lyrics.
pub(crate) fn emit_text_element_data(
    w: &mut XmlWriter,
    text: &TextElementData,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("text");

    if let Some(ref lang) = text.lang {
        elem = elem.attr("xml:lang", lang);
    }
    if let Some(ref color) = text.color {
        elem = elem.attr("color", color);
    }
    // Font attributes could be added here if needed

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&text.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("text")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a lyric extension (elision + optional syllabic + text).
pub(crate) fn emit_lyric_extension(
    w: &mut XmlWriter,
    ext: &LyricExtension,
) -> Result<(), EmitError> {
    emit_elision(w, &ext.elision)?;

    if let Some(ref syl) = ext.syllabic {
        w.text_element("syllabic", syllabic_to_string(syl))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    emit_text_element_data(w, &ext.text)?;
    Ok(())
}

/// Emit an elision element.
pub(crate) fn emit_elision(w: &mut XmlWriter, elision: &Elision) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("elision");

    if let Some(ref color) = elision.color {
        elem = elem.attr("color", color);
    }
    // Font attributes could be added here if needed

    if elision.value.is_empty() {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(&elision.value)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("elision")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit an extend element for lyrics (melisma line).
pub(crate) fn emit_extend(w: &mut XmlWriter, extend: &Extend) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("extend");

    if let Some(ref ext_type) = extend.r#type {
        elem = elem.attr("type", start_stop_continue_to_string(ext_type));
    }
    if let Some(ref color) = extend.color {
        elem = elem.attr("color", color);
    }
    // Position attributes could be added here if needed

    w.empty_element_with_attrs(elem)
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

    // === Lyric Tests ===

    #[test]
    fn test_emit_lyric_single_syllable() {
        use crate::ir::lyric::{Lyric, LyricContent, Syllabic, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: Some(crate::ir::common::AboveBelow::Below),
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Single),
                text: TextElementData {
                    value: "love".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<lyric number=\"1\" placement=\"below\">"));
        assert!(xml.contains("<syllabic>single</syllabic>"));
        assert!(xml.contains("<text>love</text>"));
        assert!(xml.contains("</lyric>"));
    }

    #[test]
    fn test_emit_lyric_begin_syllable() {
        use crate::ir::lyric::{Lyric, LyricContent, Syllabic, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Begin),
                text: TextElementData {
                    value: "Hap".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<syllabic>begin</syllabic>"));
        assert!(xml.contains("<text>Hap</text>"));
    }

    #[test]
    fn test_emit_lyric_with_extend() {
        use crate::ir::common::StartStopContinue;
        use crate::ir::lyric::{Extend, Lyric, LyricContent, Syllabic, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::End),
                text: TextElementData {
                    value: "py".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: Some(Extend {
                    r#type: Some(StartStopContinue::Start),
                    position: Position::default(),
                    color: None,
                }),
            },
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<syllabic>end</syllabic>"));
        assert!(xml.contains("<text>py</text>"));
        assert!(xml.contains("<extend type=\"start\"/>"));
    }

    #[test]
    fn test_emit_lyric_extend_only() {
        use crate::ir::common::StartStopContinue;
        use crate::ir::lyric::{Extend, Lyric, LyricContent};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::ExtendOnly(Extend {
                r#type: Some(StartStopContinue::Continue),
                position: Position::default(),
                color: None,
            }),
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<lyric number=\"1\">"));
        assert!(xml.contains("<extend type=\"continue\"/>"));
        assert!(xml.contains("</lyric>"));
    }

    #[test]
    fn test_emit_lyric_laughing() {
        use crate::ir::lyric::{Lyric, LyricContent};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Laughing,
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<laughing/>"));
    }

    #[test]
    fn test_emit_lyric_humming() {
        use crate::ir::lyric::{Lyric, LyricContent};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Humming,
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<humming/>"));
    }

    #[test]
    fn test_emit_lyric_with_end_line() {
        use crate::ir::lyric::{Lyric, LyricContent, Syllabic, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::End),
                text: TextElementData {
                    value: "line".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: true,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<end-line/>"));
    }

    #[test]
    fn test_emit_lyric_with_elision() {
        use crate::ir::lyric::{
            Elision, Lyric, LyricContent, LyricExtension, Syllabic, TextElementData,
        };

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Begin),
                text: TextElementData {
                    value: "fa".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![LyricExtension {
                    elision: Elision {
                        value: " ".to_string(),
                        font: crate::ir::common::Font::default(),
                        color: None,
                    },
                    syllabic: Some(Syllabic::End),
                    text: TextElementData {
                        value: "la".to_string(),
                        font: crate::ir::common::Font::default(),
                        color: None,
                        lang: None,
                    },
                }],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<syllabic>begin</syllabic>"));
        assert!(xml.contains("<text>fa</text>"));
        assert!(xml.contains("<elision> </elision>"));
        assert!(xml.contains("<syllabic>end</syllabic>"));
        assert!(xml.contains("<text>la</text>"));
    }

    // =======================================================================
    // Additional tests for uncovered paths
    // =======================================================================

    #[test]
    fn test_emit_cue_note() {
        let mut w = XmlWriter::new();
        let note = Note {
            position: Position::default(),
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            print_object: None,
            content: NoteContent::Cue {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::E,
                        alter: None,
                        octave: 5,
                    }),
                },
                duration: 4,
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

        assert!(xml.contains("<cue/>"));
        assert!(xml.contains("<pitch>"));
        assert!(xml.contains("<step>E</step>"));
        assert!(xml.contains("<duration>4</duration>"));
    }

    #[test]
    fn test_emit_grace_note_with_all_attributes() {
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
                    steal_time_previous: Some(50.0),
                    steal_time_following: Some(25.0),
                    make_time: Some(10),
                    slash: Some(YesNo::No),
                },
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::A,
                        alter: None,
                        octave: 4,
                    }),
                },
                ties: vec![Tie {
                    r#type: StartStop::Start,
                    time_only: Some("1".to_string()),
                }],
            },
            instrument: vec![],
            voice: None,
            r#type: Some(NoteType {
                value: NoteTypeValue::N16th,
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

        assert!(xml.contains("slash=\"no\""));
        assert!(xml.contains("steal-time-previous=\"50\""));
        assert!(xml.contains("steal-time-following=\"25\""));
        assert!(xml.contains("make-time=\"10\""));
        assert!(xml.contains("<tie type=\"start\" time-only=\"1\"/>"));
    }

    #[test]
    fn test_emit_chord_note() {
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
                    chord: true,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::E,
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

        assert!(xml.contains("<chord/>"));
        assert!(xml.contains("<step>E</step>"));
    }

    #[test]
    fn test_emit_unpitched_note() {
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
                    content: PitchRestUnpitched::Unpitched(Unpitched {
                        display_step: Some(Step::E),
                        display_octave: Some(4),
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

        assert!(xml.contains("<unpitched>"));
        assert!(xml.contains("<display-step>E</display-step>"));
        assert!(xml.contains("<display-octave>4</display-octave>"));
        assert!(xml.contains("</unpitched>"));
    }

    #[test]
    fn test_emit_unpitched_note_empty() {
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
                    content: PitchRestUnpitched::Unpitched(Unpitched {
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

        assert!(xml.contains("<unpitched/>"));
    }

    #[test]
    fn test_emit_whole_measure_rest() {
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
                        measure: Some(YesNo::Yes),
                        display_step: None,
                        display_octave: None,
                    }),
                },
                duration: 16,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
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
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<rest measure=\"yes\"/>"));
    }

    #[test]
    fn test_emit_rest_with_display_step_only() {
        let mut w = XmlWriter::new();
        let rest = Rest {
            measure: None,
            display_step: Some(Step::C),
            display_octave: None,
        };

        emit_rest(&mut w, &rest).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<rest>"));
        assert!(xml.contains("<display-step>C</display-step>"));
        assert!(!xml.contains("<display-octave>"));
        assert!(xml.contains("</rest>"));
    }

    #[test]
    fn test_emit_accidental_with_all_attributes() {
        let mut w = XmlWriter::new();
        let acc = Accidental {
            value: AccidentalValue::DoubleSharp,
            cautionary: Some(YesNo::Yes),
            editorial: Some(YesNo::Yes),
            parentheses: Some(YesNo::Yes),
            bracket: Some(YesNo::No),
            size: None,
        };

        emit_accidental(&mut w, &acc).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("cautionary=\"yes\""));
        assert!(xml.contains("editorial=\"yes\""));
        assert!(xml.contains("parentheses=\"yes\""));
        assert!(xml.contains("bracket=\"no\""));
        assert!(xml.contains(">double-sharp</accidental>"));
    }

    #[test]
    fn test_emit_note_with_instrument() {
        use crate::ir::note::Instrument;

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
            instrument: vec![Instrument {
                id: "P1-I1".to_string(),
            }],
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
            staff: Some(1),
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<instrument id=\"P1-I1\"/>"));
        assert!(xml.contains("<staff>1</staff>"));
    }

    #[test]
    fn test_emit_beam_with_fan_and_color() {
        use crate::ir::beam::Fan;

        let mut w = XmlWriter::new();
        let beam = Beam {
            value: BeamValue::End,
            number: 2,
            fan: Some(Fan::Accel),
            color: Some("#FF0000".to_string()),
        };

        emit_beam(&mut w, &beam).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"2\""));
        assert!(xml.contains("fan=\"accel\""));
        assert!(xml.contains("color=\"#FF0000\""));
        assert!(xml.contains(">end</beam>"));
    }

    #[test]
    fn test_emit_stem_with_default_y_and_color() {
        let mut w = XmlWriter::new();
        let stem = Stem {
            value: StemValue::Down,
            default_y: Some(-50.0),
            color: Some("#0000FF".to_string()),
        };

        emit_stem(&mut w, &stem).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("default-y=\"-50\""));
        assert!(xml.contains("color=\"#0000FF\""));
        assert!(xml.contains(">down</stem>"));
    }

    #[test]
    fn test_emit_notehead_with_all_attributes() {
        let mut w = XmlWriter::new();
        let notehead = Notehead {
            value: NoteheadValue::X,
            filled: Some(YesNo::No),
            parentheses: Some(YesNo::Yes),
            font: crate::ir::common::Font::default(),
            color: Some("#00FF00".to_string()),
        };

        emit_notehead(&mut w, &notehead).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("filled=\"no\""));
        assert!(xml.contains("parentheses=\"yes\""));
        assert!(xml.contains("color=\"#00FF00\""));
        assert!(xml.contains(">x</notehead>"));
    }

    #[test]
    fn test_emit_time_modification_with_normal_dots() {
        let mut w = XmlWriter::new();
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: Some(NoteTypeValue::Quarter),
            normal_dots: 2,
        };

        emit_time_modification(&mut w, &tm).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<actual-notes>3</actual-notes>"));
        assert!(xml.contains("<normal-notes>2</normal-notes>"));
        assert!(xml.contains("<normal-type>quarter</normal-type>"));
        // Should have 2 normal-dot elements
        assert_eq!(xml.matches("<normal-dot/>").count(), 2);
    }

    #[test]
    fn test_emit_lyric_with_name_and_justify() {
        use crate::ir::common::LeftCenterRight;
        use crate::ir::lyric::{Lyric, LyricContent, Syllabic, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: Some("verse".to_string()),
            justify: Some(LeftCenterRight::Center),
            placement: None,
            print_object: Some(YesNo::Yes),
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Single),
                text: TextElementData {
                    value: "word".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("name=\"verse\""));
        assert!(xml.contains("justify=\"center\""));
        assert!(xml.contains("print-object=\"yes\""));
    }

    #[test]
    fn test_emit_lyric_with_end_paragraph() {
        use crate::ir::lyric::{Lyric, LyricContent, Syllabic, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Single),
                text: TextElementData {
                    value: "end".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: true,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<end-paragraph/>"));
    }

    #[test]
    fn test_emit_text_element_with_lang_and_color() {
        use crate::ir::lyric::TextElementData;

        let mut w = XmlWriter::new();
        let text = TextElementData {
            value: "Test".to_string(),
            font: crate::ir::common::Font::default(),
            color: Some("#123456".to_string()),
            lang: Some("en".to_string()),
        };

        emit_text_element_data(&mut w, &text).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("xml:lang=\"en\""));
        assert!(xml.contains("color=\"#123456\""));
        assert!(xml.contains(">Test</text>"));
    }

    #[test]
    fn test_emit_elision_empty() {
        use crate::ir::lyric::Elision;

        let mut w = XmlWriter::new();
        let elision = Elision {
            value: String::new(),
            font: crate::ir::common::Font::default(),
            color: None,
        };

        emit_elision(&mut w, &elision).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<elision/>"));
    }

    #[test]
    fn test_emit_elision_with_color() {
        use crate::ir::lyric::Elision;

        let mut w = XmlWriter::new();
        let elision = Elision {
            value: "-".to_string(),
            font: crate::ir::common::Font::default(),
            color: Some("#AABBCC".to_string()),
        };

        emit_elision(&mut w, &elision).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("color=\"#AABBCC\""));
        assert!(xml.contains(">-</elision>"));
    }

    #[test]
    fn test_emit_extend_with_color() {
        use crate::ir::common::StartStopContinue;
        use crate::ir::lyric::Extend;

        let mut w = XmlWriter::new();
        let extend = Extend {
            r#type: Some(StartStopContinue::Stop),
            position: Position::default(),
            color: Some("#DDEEFF".to_string()),
        };

        emit_extend(&mut w, &extend).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"stop\""));
        assert!(xml.contains("color=\"#DDEEFF\""));
    }

    #[test]
    fn test_emit_extend_without_type() {
        use crate::ir::lyric::Extend;

        let mut w = XmlWriter::new();
        let extend = Extend {
            r#type: None,
            position: Position::default(),
            color: None,
        };

        emit_extend(&mut w, &extend).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<extend/>"));
        assert!(!xml.contains("type="));
    }

    #[test]
    fn test_emit_lyric_extension_without_syllabic() {
        use crate::ir::lyric::{Elision, LyricExtension, TextElementData};

        let mut w = XmlWriter::new();
        let ext = LyricExtension {
            elision: Elision {
                value: " ".to_string(),
                font: crate::ir::common::Font::default(),
                color: None,
            },
            syllabic: None,
            text: TextElementData {
                value: "da".to_string(),
                font: crate::ir::common::Font::default(),
                color: None,
                lang: None,
            },
        };

        emit_lyric_extension(&mut w, &ext).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<elision> </elision>"));
        assert!(!xml.contains("<syllabic>"));
        assert!(xml.contains("<text>da</text>"));
    }

    #[test]
    fn test_emit_lyric_syllable_without_syllabic() {
        use crate::ir::lyric::{Lyric, LyricContent, TextElementData};

        let mut w = XmlWriter::new();
        let lyric = Lyric {
            number: None,
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: None,
                text: TextElementData {
                    value: "oh".to_string(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        emit_lyric(&mut w, &lyric).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<lyric>"));
        assert!(!xml.contains("<syllabic>"));
        assert!(xml.contains("<text>oh</text>"));
    }

    #[test]
    fn test_emit_note_with_double_dotted() {
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
                        step: Step::G,
                        alter: None,
                        octave: 4,
                    }),
                },
                duration: 7,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: Some(crate::ir::common::SymbolSize::Large),
            }),
            dots: vec![Dot::default(), Dot::default()],
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

        assert_eq!(xml.matches("<dot/>").count(), 2);
    }

    #[test]
    fn test_emit_grace_note_chord_with_tie() {
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
                    slash: None,
                },
                full_note: FullNote {
                    chord: true,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::B,
                        alter: Some(-1.0),
                        octave: 3,
                    }),
                },
                ties: vec![
                    Tie {
                        r#type: StartStop::Start,
                        time_only: None,
                    },
                    Tie {
                        r#type: StartStop::Stop,
                        time_only: None,
                    },
                ],
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
        };

        emit_note(&mut w, &note).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<grace/>"));
        assert!(xml.contains("<chord/>"));
        assert!(xml.contains("<alter>-1</alter>"));
        assert!(xml.contains("<tie type=\"start\"/>"));
        assert!(xml.contains("<tie type=\"stop\"/>"));
    }

    #[test]
    fn test_emit_all_note_type_values() {
        // Test emission of various note type values
        let test_cases = [
            (NoteTypeValue::Maxima, "maxima"),
            (NoteTypeValue::Long, "long"),
            (NoteTypeValue::Breve, "breve"),
            (NoteTypeValue::Whole, "whole"),
            (NoteTypeValue::Half, "half"),
            (NoteTypeValue::Quarter, "quarter"),
            (NoteTypeValue::Eighth, "eighth"),
            (NoteTypeValue::N16th, "16th"),
            (NoteTypeValue::N32nd, "32nd"),
            (NoteTypeValue::N64th, "64th"),
            (NoteTypeValue::N128th, "128th"),
            (NoteTypeValue::N256th, "256th"),
            (NoteTypeValue::N512th, "512th"),
            (NoteTypeValue::N1024th, "1024th"),
        ];

        for (value, expected) in test_cases {
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
                    duration: 1,
                    ties: vec![],
                },
                instrument: vec![],
                voice: None,
                r#type: Some(NoteType { value, size: None }),
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

            assert!(
                xml.contains(&format!("<type>{}</type>", expected)),
                "Failed for {:?}",
                value
            );
        }
    }

    #[test]
    fn test_emit_all_beam_values() {
        let test_cases = [
            (BeamValue::Begin, "begin"),
            (BeamValue::Continue, "continue"),
            (BeamValue::End, "end"),
            (BeamValue::ForwardHook, "forward hook"),
            (BeamValue::BackwardHook, "backward hook"),
        ];

        for (value, expected) in test_cases {
            let mut w = XmlWriter::new();
            let beam = Beam {
                value,
                number: 1,
                fan: None,
                color: None,
            };

            emit_beam(&mut w, &beam).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!(">{}</beam>", expected)),
                "Failed for {:?}",
                value
            );
        }
    }

    #[test]
    fn test_emit_all_stem_values() {
        let test_cases = [
            (StemValue::Up, "up"),
            (StemValue::Down, "down"),
            (StemValue::None, "none"),
            (StemValue::Double, "double"),
        ];

        for (value, expected) in test_cases {
            let mut w = XmlWriter::new();
            let stem = Stem {
                value,
                default_y: None,
                color: None,
            };

            emit_stem(&mut w, &stem).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!(">{}</stem>", expected)),
                "Failed for {:?}",
                value
            );
        }
    }
}
