//! Note and rest compilation for Fermata syntax.
//!
//! This module handles compiling note and rest S-expressions into IR Note types.

use crate::ir::beam::{Stem, StemValue};
use crate::ir::common::{
    EmptyPlacement, Position, StartStop, StartStopContinue, YesNo,
};
use crate::ir::notation::{
    ArticulationElement, Articulations, Mordent, NotationContent, Notations,
    OrnamentElement, OrnamentWithAccidentals, Ornaments, Slur, StrongAccent, Tied, Turn,
};
use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched, Rest, Tie};
use crate::lang::ast::{
    Articulation, FermataDuration, FermataNote, FermataRest, Ornament, StemDirection,
};
use crate::lang::defaults::DEFAULT_DIVISIONS;
use crate::lang::duration::{compile_dots, compile_duration_divisions_with, compile_duration_type};
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::pitch::{compile_pitch, parse_pitch_str};
use crate::sexpr::Sexpr;

/// Compile a note S-expression into an IR Note.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::note::compile_note;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(note c4 :q)")?;
/// let note = compile_note(&sexpr)?;
/// ```
pub fn compile_note(sexpr: &Sexpr) -> CompileResult<Note> {
    match sexpr {
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidNote("empty note list".to_string()));
            }

            // Check for 'note' head
            if !items[0].is_symbol("note") {
                return Err(CompileError::InvalidNote(
                    format!("expected 'note', got {:?}", items[0])
                ));
            }

            let fermata_note = parse_note_form(&items[1..])?;
            compile_fermata_note(&fermata_note)
        }
        _ => Err(CompileError::InvalidNote(
            format!("expected note list, got {:?}", sexpr)
        )),
    }
}

/// Parse note arguments from S-expression items.
///
/// Expected format: `pitch duration [keywords...]`
/// - pitch: "c4", "f#5", etc.
/// - duration: :q, :h, :w, :8, etc.
/// - keywords: :voice N, :staff N, :stem up/down, :tie start/stop, etc.
pub fn parse_note_form(items: &[Sexpr]) -> CompileResult<FermataNote> {
    if items.is_empty() {
        return Err(CompileError::InvalidNote("note requires pitch".to_string()));
    }

    // First item is pitch
    let pitch = parse_pitch_str(items[0].as_symbol().ok_or_else(|| {
        CompileError::InvalidNote(format!("expected pitch symbol, got {:?}", items[0]))
    })?)?;

    // Second item is duration (if present and is a keyword/symbol)
    let (duration, remaining_start) = if items.len() > 1 {
        if let Some(dur_str) = items[1].as_keyword().or_else(|| items[1].as_symbol()) {
            // Check if it's a duration keyword (starts with a duration char or is a duration word)
            if is_duration_keyword(dur_str) {
                (crate::lang::duration::parse_duration(dur_str)?, 2)
            } else {
                (FermataDuration::default(), 1)
            }
        } else {
            (FermataDuration::default(), 1)
        }
    } else {
        (FermataDuration::default(), 1)
    };

    // Parse remaining keyword arguments
    let mut voice: Option<u32> = None;
    let mut staff: Option<u32> = None;
    let mut stem: Option<StemDirection> = None;
    let mut tie: Option<StartStop> = None;
    let mut slur: Option<StartStop> = None;
    let mut articulations: Vec<Articulation> = Vec::new();
    let mut ornaments: Vec<Ornament> = Vec::new();

    let mut i = remaining_start;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "voice" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidNote("missing :voice value".to_string()));
                    }
                    voice = Some(parse_u32(&items[i + 1])?);
                    i += 2;
                }
                "staff" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidNote("missing :staff value".to_string()));
                    }
                    staff = Some(parse_u32(&items[i + 1])?);
                    i += 2;
                }
                "stem" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidNote("missing :stem value".to_string()));
                    }
                    stem = Some(parse_stem(&items[i + 1])?);
                    i += 2;
                }
                "tie" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidNote("missing :tie value".to_string()));
                    }
                    tie = Some(parse_start_stop(&items[i + 1])?);
                    i += 2;
                }
                "slur" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidNote("missing :slur value".to_string()));
                    }
                    slur = Some(parse_start_stop(&items[i + 1])?);
                    i += 2;
                }
                // Articulations as flags
                "staccato" => {
                    articulations.push(Articulation::Staccato);
                    i += 1;
                }
                "accent" => {
                    articulations.push(Articulation::Accent);
                    i += 1;
                }
                "tenuto" => {
                    articulations.push(Articulation::Tenuto);
                    i += 1;
                }
                "marcato" => {
                    articulations.push(Articulation::StrongAccent);
                    i += 1;
                }
                "staccatissimo" => {
                    articulations.push(Articulation::Staccatissimo);
                    i += 1;
                }
                "spiccato" => {
                    articulations.push(Articulation::Spiccato);
                    i += 1;
                }
                // Ornaments as flags
                "trill" => {
                    ornaments.push(Ornament::Trill);
                    i += 1;
                }
                "mordent" => {
                    ornaments.push(Ornament::Mordent);
                    i += 1;
                }
                "turn" => {
                    ornaments.push(Ornament::Turn);
                    i += 1;
                }
                _ => {
                    // Unknown keyword - skip it (or could error)
                    i += 1;
                }
            }
        } else {
            // Skip non-keyword items
            i += 1;
        }
    }

    Ok(FermataNote {
        pitch,
        duration,
        voice,
        staff,
        stem,
        articulations,
        ornaments,
        tie,
        slur,
        lyric: None,
    })
}

/// Check if a string looks like a duration keyword.
fn is_duration_keyword(s: &str) -> bool {
    let s = s.trim_start_matches(':');
    let s = s.trim_end_matches('.');
    matches!(
        s.to_lowercase().as_str(),
        "q" | "h" | "w" | "8" | "16" | "32" | "64" | "128" | "256" | "512" | "1024"
            | "quarter" | "half" | "whole" | "eighth" | "sixteenth"
            | "crotchet" | "minim" | "semibreve" | "quaver" | "semiquaver"
            | "breve" | "long" | "maxima"
    )
}

/// Compile a FermataNote to an IR Note.
pub fn compile_fermata_note(note: &FermataNote) -> CompileResult<Note> {
    let ir_pitch = compile_pitch(&note.pitch)?;
    let divisions = DEFAULT_DIVISIONS as u64;

    // Build ties
    let ties = compile_ties(note.tie);

    // Build notations
    let notations = compile_notations(note)?;

    Ok(Note {
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
                content: PitchRestUnpitched::Pitch(ir_pitch),
            },
            duration: compile_duration_divisions_with(&note.duration, divisions),
            ties,
        },
        instrument: vec![],
        voice: note.voice.map(|v| v.to_string()),
        r#type: Some(compile_duration_type(&note.duration.base)),
        dots: compile_dots(note.duration.dots),
        accidental: None,
        time_modification: None,
        stem: note.stem.map(compile_stem_ir),
        notehead: None,
        staff: note.staff.map(|s| s as u16),
        beams: vec![],
        notations,
        lyrics: vec![],
    })
}

/// Parse a u32 from an S-expression (Integer or Symbol).
pub fn parse_u32(sexpr: &Sexpr) -> CompileResult<u32> {
    match sexpr {
        Sexpr::Integer(n) => {
            if *n < 0 {
                return Err(CompileError::InvalidNote(
                    format!("expected positive integer, got {}", n)
                ));
            }
            Ok(*n as u32)
        }
        Sexpr::Symbol(s) => s.parse().map_err(|_| {
            CompileError::InvalidNote(format!("invalid number '{}'", s))
        }),
        _ => Err(CompileError::InvalidNote(
            format!("expected integer, got {:?}", sexpr)
        )),
    }
}

/// Parse stem direction from an S-expression.
pub fn parse_stem(sexpr: &Sexpr) -> CompileResult<StemDirection> {
    let s = sexpr.as_symbol().or_else(|| sexpr.as_keyword()).ok_or_else(|| {
        CompileError::InvalidNote(format!("expected stem symbol, got {:?}", sexpr))
    })?;

    match s.to_lowercase().as_str() {
        "up" => Ok(StemDirection::Up),
        "down" => Ok(StemDirection::Down),
        "none" => Ok(StemDirection::None),
        "double" => Ok(StemDirection::Double),
        _ => Err(CompileError::InvalidNote(
            format!("invalid stem direction '{}', expected up, down, none, or double", s)
        )),
    }
}

/// Parse start/stop from an S-expression.
pub fn parse_start_stop(sexpr: &Sexpr) -> CompileResult<StartStop> {
    let s = sexpr.as_symbol().or_else(|| sexpr.as_keyword()).ok_or_else(|| {
        CompileError::InvalidNote(format!("expected start/stop symbol, got {:?}", sexpr))
    })?;

    match s.to_lowercase().as_str() {
        "start" => Ok(StartStop::Start),
        "stop" => Ok(StartStop::Stop),
        _ => Err(CompileError::InvalidNote(
            format!("invalid start/stop '{}', expected start or stop", s)
        )),
    }
}

/// Compile tie option to Vec<Tie>.
pub fn compile_ties(tie: Option<StartStop>) -> Vec<Tie> {
    match tie {
        Some(ss) => vec![Tie {
            r#type: ss,
            time_only: None,
        }],
        None => vec![],
    }
}

/// Compile stem direction to IR Stem.
pub fn compile_stem_ir(direction: StemDirection) -> Stem {
    let value = match direction {
        StemDirection::Up => StemValue::Up,
        StemDirection::Down => StemValue::Down,
        StemDirection::None => StemValue::None,
        StemDirection::Double => StemValue::Double,
    };

    Stem {
        value,
        default_y: None,
        color: None,
    }
}

/// Convert StartStop to StartStopContinue (for Tied notation).
pub fn start_stop_to_continue(ss: StartStop) -> StartStopContinue {
    match ss {
        StartStop::Start => StartStopContinue::Start,
        StartStop::Stop => StartStopContinue::Stop,
    }
}

/// Compile notations from a FermataNote.
pub fn compile_notations(note: &FermataNote) -> CompileResult<Vec<Notations>> {
    let mut content: Vec<NotationContent> = Vec::new();

    // Add tied notation if tie is present
    if let Some(tie) = note.tie {
        content.push(NotationContent::Tied(Tied {
            r#type: start_stop_to_continue(tie),
            number: None,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        }));
    }

    // Add slur notation if slur is present
    if let Some(slur) = note.slur {
        content.push(NotationContent::Slur(Slur {
            r#type: start_stop_to_continue(slur),
            number: 1, // Default slur number
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        }));
    }

    // Add articulations
    if !note.articulations.is_empty() {
        let articulation_elements: Vec<ArticulationElement> = note
            .articulations
            .iter()
            .map(|a| match a {
                Articulation::Staccato => ArticulationElement::Staccato(EmptyPlacement::default()),
                Articulation::Staccatissimo => ArticulationElement::Staccatissimo(EmptyPlacement::default()),
                Articulation::Spiccato => ArticulationElement::Spiccato(EmptyPlacement::default()),
                Articulation::Accent => ArticulationElement::Accent(EmptyPlacement::default()),
                Articulation::StrongAccent => ArticulationElement::StrongAccent(StrongAccent::default()),
                Articulation::Tenuto => ArticulationElement::Tenuto(EmptyPlacement::default()),
                Articulation::DetachedLegato => ArticulationElement::DetachedLegato(EmptyPlacement::default()),
                Articulation::BreathMark => {
                    // BreathMark requires a value, using default
                    ArticulationElement::BreathMark(crate::ir::notation::BreathMark {
                        value: crate::ir::notation::BreathMarkValue::Comma,
                        placement: None,
                        position: Position::default(),
                    })
                }
                Articulation::Caesura => {
                    ArticulationElement::Caesura(crate::ir::notation::Caesura {
                        value: crate::ir::notation::CaesuraValue::Normal,
                        placement: None,
                        position: Position::default(),
                    })
                }
            })
            .collect();

        content.push(NotationContent::Articulations(Box::new(Articulations {
            content: articulation_elements,
        })));
    }

    // Add ornaments
    if !note.ornaments.is_empty() {
        let ornament_elements: Vec<OrnamentWithAccidentals> = note
            .ornaments
            .iter()
            .map(|o| {
                let ornament = match o {
                    Ornament::Trill => OrnamentElement::TrillMark(
                        crate::ir::notation::EmptyTrillSound::default()
                    ),
                    Ornament::Mordent => OrnamentElement::Mordent(Mordent::default()),
                    Ornament::InvertedMordent => OrnamentElement::InvertedMordent(Mordent::default()),
                    Ornament::Turn => OrnamentElement::Turn(Turn::default()),
                    Ornament::InvertedTurn => OrnamentElement::InvertedTurn(Turn::default()),
                    Ornament::DelayedTurn => OrnamentElement::DelayedTurn(Turn::default()),
                    Ornament::Shake => OrnamentElement::Shake(
                        crate::ir::notation::EmptyTrillSound::default()
                    ),
                    Ornament::Tremolo(marks) => OrnamentElement::Tremolo(
                        crate::ir::notation::Tremolo {
                            value: *marks,
                            r#type: None,
                            placement: None,
                            position: Position::default(),
                        }
                    ),
                };
                OrnamentWithAccidentals {
                    ornament,
                    accidental_marks: vec![],
                }
            })
            .collect();

        content.push(NotationContent::Ornaments(Box::new(Ornaments {
            content: ornament_elements,
        })));
    }

    // Return notations if we have any content
    if content.is_empty() {
        Ok(vec![])
    } else {
        Ok(vec![Notations {
            print_object: None,
            content,
            editorial: Default::default(),
        }])
    }
}

// === Rest compilation ===

/// Compile a rest S-expression into an IR Note.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::note::compile_rest;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(rest :q)")?;
/// let note = compile_rest(&sexpr)?;
/// ```
pub fn compile_rest(sexpr: &Sexpr) -> CompileResult<Note> {
    match sexpr {
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidRest("empty rest list".to_string()));
            }

            // Check for 'rest' head
            if !items[0].is_symbol("rest") {
                return Err(CompileError::InvalidRest(
                    format!("expected 'rest', got {:?}", items[0])
                ));
            }

            let fermata_rest = parse_rest_form(&items[1..])?;
            compile_fermata_rest(&fermata_rest)
        }
        _ => Err(CompileError::InvalidRest(
            format!("expected rest list, got {:?}", sexpr)
        )),
    }
}

/// Parse rest arguments from S-expression items.
///
/// Expected format: `duration [keywords...]`
/// - duration: :q, :h, :w, :8, etc.
/// - keywords: :voice N, :staff N, :measure
pub fn parse_rest_form(items: &[Sexpr]) -> CompileResult<FermataRest> {
    // First item is duration (if present and is a keyword/symbol)
    let (duration, remaining_start) = if !items.is_empty() {
        if let Some(dur_str) = items[0].as_keyword().or_else(|| items[0].as_symbol()) {
            if is_duration_keyword(dur_str) {
                (crate::lang::duration::parse_duration(dur_str)?, 1)
            } else {
                (FermataDuration::default(), 0)
            }
        } else {
            (FermataDuration::default(), 0)
        }
    } else {
        (FermataDuration::default(), 0)
    };

    // Parse remaining keyword arguments
    let mut voice: Option<u32> = None;
    let mut staff: Option<u32> = None;
    let mut measure_rest = false;

    let mut i = remaining_start;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "voice" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidRest("missing :voice value".to_string()));
                    }
                    voice = Some(parse_u32(&items[i + 1])?);
                    i += 2;
                }
                "staff" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidRest("missing :staff value".to_string()));
                    }
                    staff = Some(parse_u32(&items[i + 1])?);
                    i += 2;
                }
                "measure" => {
                    measure_rest = true;
                    i += 1;
                }
                _ => {
                    // Unknown keyword - skip it
                    i += 1;
                }
            }
        } else {
            // Skip non-keyword items
            i += 1;
        }
    }

    Ok(FermataRest {
        duration,
        voice,
        staff,
        measure_rest,
    })
}

/// Compile a FermataRest to an IR Note (with Rest content).
pub fn compile_fermata_rest(rest: &FermataRest) -> CompileResult<Note> {
    let divisions = DEFAULT_DIVISIONS as u64;

    let ir_rest = Rest {
        measure: if rest.measure_rest { Some(YesNo::Yes) } else { None },
        display_step: None,
        display_octave: None,
    };

    Ok(Note {
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
                content: PitchRestUnpitched::Rest(ir_rest),
            },
            duration: compile_duration_divisions_with(&rest.duration, divisions),
            ties: vec![],
        },
        instrument: vec![],
        voice: rest.voice.map(|v| v.to_string()),
        r#type: Some(compile_duration_type(&rest.duration.base)),
        dots: compile_dots(rest.duration.dots),
        accidental: None,
        time_modification: None,
        stem: None,
        notehead: None,
        staff: rest.staff.map(|s| s as u16),
        beams: vec![],
        notations: vec![],
        lyrics: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::pitch::Step as IrStep;
    use crate::lang::ast::{FermataPitch, PitchStep};

    // === parse_u32 tests ===

    #[test]
    fn test_parse_u32_integer() {
        assert_eq!(parse_u32(&Sexpr::Integer(42)).unwrap(), 42);
        assert_eq!(parse_u32(&Sexpr::Integer(0)).unwrap(), 0);
        assert_eq!(parse_u32(&Sexpr::Integer(1)).unwrap(), 1);
    }

    #[test]
    fn test_parse_u32_symbol() {
        assert_eq!(parse_u32(&Sexpr::symbol("123")).unwrap(), 123);
        assert_eq!(parse_u32(&Sexpr::symbol("1")).unwrap(), 1);
    }

    #[test]
    fn test_parse_u32_negative() {
        assert!(parse_u32(&Sexpr::Integer(-1)).is_err());
    }

    #[test]
    fn test_parse_u32_invalid_symbol() {
        assert!(parse_u32(&Sexpr::symbol("abc")).is_err());
    }

    // === parse_stem tests ===

    #[test]
    fn test_parse_stem_up() {
        assert_eq!(parse_stem(&Sexpr::symbol("up")).unwrap(), StemDirection::Up);
        assert_eq!(parse_stem(&Sexpr::symbol("UP")).unwrap(), StemDirection::Up);
    }

    #[test]
    fn test_parse_stem_down() {
        assert_eq!(parse_stem(&Sexpr::symbol("down")).unwrap(), StemDirection::Down);
    }

    #[test]
    fn test_parse_stem_none() {
        assert_eq!(parse_stem(&Sexpr::symbol("none")).unwrap(), StemDirection::None);
    }

    #[test]
    fn test_parse_stem_double() {
        assert_eq!(parse_stem(&Sexpr::symbol("double")).unwrap(), StemDirection::Double);
    }

    #[test]
    fn test_parse_stem_invalid() {
        assert!(parse_stem(&Sexpr::symbol("sideways")).is_err());
    }

    // === parse_start_stop tests ===

    #[test]
    fn test_parse_start_stop_start() {
        assert_eq!(parse_start_stop(&Sexpr::symbol("start")).unwrap(), StartStop::Start);
    }

    #[test]
    fn test_parse_start_stop_stop() {
        assert_eq!(parse_start_stop(&Sexpr::symbol("stop")).unwrap(), StartStop::Stop);
    }

    #[test]
    fn test_parse_start_stop_invalid() {
        assert!(parse_start_stop(&Sexpr::symbol("continue")).is_err());
    }

    // === compile_ties tests ===

    #[test]
    fn test_compile_ties_none() {
        let ties = compile_ties(None);
        assert!(ties.is_empty());
    }

    #[test]
    fn test_compile_ties_start() {
        let ties = compile_ties(Some(StartStop::Start));
        assert_eq!(ties.len(), 1);
        assert_eq!(ties[0].r#type, StartStop::Start);
    }

    #[test]
    fn test_compile_ties_stop() {
        let ties = compile_ties(Some(StartStop::Stop));
        assert_eq!(ties.len(), 1);
        assert_eq!(ties[0].r#type, StartStop::Stop);
    }

    // === compile_stem_ir tests ===

    #[test]
    fn test_compile_stem_ir_up() {
        let stem = compile_stem_ir(StemDirection::Up);
        assert_eq!(stem.value, StemValue::Up);
    }

    #[test]
    fn test_compile_stem_ir_down() {
        let stem = compile_stem_ir(StemDirection::Down);
        assert_eq!(stem.value, StemValue::Down);
    }

    #[test]
    fn test_compile_stem_ir_none() {
        let stem = compile_stem_ir(StemDirection::None);
        assert_eq!(stem.value, StemValue::None);
    }

    #[test]
    fn test_compile_stem_ir_double() {
        let stem = compile_stem_ir(StemDirection::Double);
        assert_eq!(stem.value, StemValue::Double);
    }

    // === start_stop_to_continue tests ===

    #[test]
    fn test_start_stop_to_continue_start() {
        assert_eq!(start_stop_to_continue(StartStop::Start), StartStopContinue::Start);
    }

    #[test]
    fn test_start_stop_to_continue_stop() {
        assert_eq!(start_stop_to_continue(StartStop::Stop), StartStopContinue::Stop);
    }

    // === is_duration_keyword tests ===

    #[test]
    fn test_is_duration_keyword_short_forms() {
        assert!(is_duration_keyword("q"));
        assert!(is_duration_keyword("h"));
        assert!(is_duration_keyword("w"));
        assert!(is_duration_keyword("8"));
        assert!(is_duration_keyword("16"));
    }

    #[test]
    fn test_is_duration_keyword_full_names() {
        assert!(is_duration_keyword("quarter"));
        assert!(is_duration_keyword("half"));
        assert!(is_duration_keyword("whole"));
    }

    #[test]
    fn test_is_duration_keyword_with_dots() {
        assert!(is_duration_keyword("q."));
        assert!(is_duration_keyword("h.."));
    }

    #[test]
    fn test_is_duration_keyword_with_colon() {
        assert!(is_duration_keyword(":q"));
        assert!(is_duration_keyword(":quarter"));
    }

    #[test]
    fn test_is_duration_keyword_not_duration() {
        assert!(!is_duration_keyword("voice"));
        assert!(!is_duration_keyword("staff"));
        assert!(!is_duration_keyword("staccato"));
    }

    // === parse_note_form tests ===

    #[test]
    fn test_parse_note_form_simple() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert_eq!(note.pitch.step, PitchStep::C);
        assert_eq!(note.pitch.octave, 4);
    }

    #[test]
    fn test_parse_note_form_with_voice() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("voice"),
            Sexpr::Integer(1),
        ];
        let note = parse_note_form(&items).unwrap();
        assert_eq!(note.voice, Some(1));
    }

    #[test]
    fn test_parse_note_form_with_staff() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("staff"),
            Sexpr::Integer(2),
        ];
        let note = parse_note_form(&items).unwrap();
        assert_eq!(note.staff, Some(2));
    }

    #[test]
    fn test_parse_note_form_with_stem() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("stem"),
            Sexpr::symbol("up"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert_eq!(note.stem, Some(StemDirection::Up));
    }

    #[test]
    fn test_parse_note_form_with_tie() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("tie"),
            Sexpr::symbol("start"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert_eq!(note.tie, Some(StartStop::Start));
    }

    #[test]
    fn test_parse_note_form_with_slur() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("slur"),
            Sexpr::symbol("start"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert_eq!(note.slur, Some(StartStop::Start));
    }

    #[test]
    fn test_parse_note_form_with_staccato() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("staccato"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert!(note.articulations.contains(&Articulation::Staccato));
    }

    #[test]
    fn test_parse_note_form_with_accent() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("accent"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert!(note.articulations.contains(&Articulation::Accent));
    }

    #[test]
    fn test_parse_note_form_with_trill() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("trill"),
        ];
        let note = parse_note_form(&items).unwrap();
        assert!(note.ornaments.contains(&Ornament::Trill));
    }

    #[test]
    fn test_parse_note_form_empty() {
        let items: Vec<Sexpr> = vec![];
        assert!(parse_note_form(&items).is_err());
    }

    // === compile_note tests ===

    #[test]
    fn test_compile_note_simple() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
        ]);
        let note = compile_note(&sexpr).unwrap();

        if let NoteContent::Regular { full_note, duration, .. } = &note.content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::C);
                assert_eq!(p.octave, 4);
            } else {
                panic!("Expected Pitch");
            }
            assert_eq!(*duration, DEFAULT_DIVISIONS as u64); // Quarter note
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_note_with_sharp() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("f#5"),
            Sexpr::keyword("h"),
        ]);
        let note = compile_note(&sexpr).unwrap();

        if let NoteContent::Regular { full_note, duration, .. } = &note.content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::F);
                assert_eq!(p.alter, Some(1.0));
                assert_eq!(p.octave, 5);
            } else {
                panic!("Expected Pitch");
            }
            assert_eq!(*duration, DEFAULT_DIVISIONS as u64 * 2); // Half note
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_note_with_voice_and_staff() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("voice"),
            Sexpr::Integer(2),
            Sexpr::keyword("staff"),
            Sexpr::Integer(1),
        ]);
        let note = compile_note(&sexpr).unwrap();
        assert_eq!(note.voice, Some("2".to_string()));
        assert_eq!(note.staff, Some(1));
    }

    #[test]
    fn test_compile_note_with_stem() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("stem"),
            Sexpr::symbol("down"),
        ]);
        let note = compile_note(&sexpr).unwrap();
        assert!(note.stem.is_some());
        assert_eq!(note.stem.as_ref().unwrap().value, StemValue::Down);
    }

    #[test]
    fn test_compile_note_with_tie() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("tie"),
            Sexpr::symbol("start"),
        ]);
        let note = compile_note(&sexpr).unwrap();

        if let NoteContent::Regular { ties, .. } = &note.content {
            assert_eq!(ties.len(), 1);
            assert_eq!(ties[0].r#type, StartStop::Start);
        } else {
            panic!("Expected Regular");
        }

        // Also check notations for tied
        assert!(!note.notations.is_empty());
    }

    #[test]
    fn test_compile_note_with_articulations() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
            Sexpr::keyword("q"),
            Sexpr::keyword("staccato"),
            Sexpr::keyword("accent"),
        ]);
        let note = compile_note(&sexpr).unwrap();

        assert!(!note.notations.is_empty());
        // Check that articulations are in the notations
        let has_articulations = note.notations.iter().any(|n| {
            n.content.iter().any(|c| matches!(c, NotationContent::Articulations(_)))
        });
        assert!(has_articulations);
    }

    #[test]
    fn test_compile_note_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(compile_note(&sexpr).is_err());
    }

    #[test]
    fn test_compile_note_wrong_head() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("rest"),
            Sexpr::keyword("q"),
        ]);
        assert!(compile_note(&sexpr).is_err());
    }

    #[test]
    fn test_compile_note_not_list() {
        let sexpr = Sexpr::symbol("note");
        assert!(compile_note(&sexpr).is_err());
    }

    // === parse_rest_form tests ===

    #[test]
    fn test_parse_rest_form_simple() {
        let items = vec![
            Sexpr::keyword("q"),
        ];
        let rest = parse_rest_form(&items).unwrap();
        assert!(!rest.measure_rest);
    }

    #[test]
    fn test_parse_rest_form_with_voice() {
        let items = vec![
            Sexpr::keyword("q"),
            Sexpr::keyword("voice"),
            Sexpr::Integer(1),
        ];
        let rest = parse_rest_form(&items).unwrap();
        assert_eq!(rest.voice, Some(1));
    }

    #[test]
    fn test_parse_rest_form_with_staff() {
        let items = vec![
            Sexpr::keyword("q"),
            Sexpr::keyword("staff"),
            Sexpr::Integer(2),
        ];
        let rest = parse_rest_form(&items).unwrap();
        assert_eq!(rest.staff, Some(2));
    }

    #[test]
    fn test_parse_rest_form_measure_rest() {
        let items = vec![
            Sexpr::keyword("w"),
            Sexpr::keyword("measure"),
        ];
        let rest = parse_rest_form(&items).unwrap();
        assert!(rest.measure_rest);
    }

    #[test]
    fn test_parse_rest_form_empty() {
        let items: Vec<Sexpr> = vec![];
        let rest = parse_rest_form(&items).unwrap();
        // Should have default duration
        assert!(!rest.measure_rest);
    }

    // === compile_rest tests ===

    #[test]
    fn test_compile_rest_simple() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("rest"),
            Sexpr::keyword("q"),
        ]);
        let note = compile_rest(&sexpr).unwrap();

        if let NoteContent::Regular { full_note, duration, .. } = &note.content {
            if let PitchRestUnpitched::Rest(r) = &full_note.content {
                assert!(r.measure.is_none());
            } else {
                panic!("Expected Rest");
            }
            assert_eq!(*duration, DEFAULT_DIVISIONS as u64); // Quarter rest
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_rest_half() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("rest"),
            Sexpr::keyword("h"),
        ]);
        let note = compile_rest(&sexpr).unwrap();

        if let NoteContent::Regular { duration, .. } = &note.content {
            assert_eq!(*duration, DEFAULT_DIVISIONS as u64 * 2); // Half rest
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_rest_with_voice() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("rest"),
            Sexpr::keyword("q"),
            Sexpr::keyword("voice"),
            Sexpr::Integer(2),
        ]);
        let note = compile_rest(&sexpr).unwrap();
        assert_eq!(note.voice, Some("2".to_string()));
    }

    #[test]
    fn test_compile_rest_with_staff() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("rest"),
            Sexpr::keyword("q"),
            Sexpr::keyword("staff"),
            Sexpr::Integer(1),
        ]);
        let note = compile_rest(&sexpr).unwrap();
        assert_eq!(note.staff, Some(1));
    }

    #[test]
    fn test_compile_rest_measure() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("rest"),
            Sexpr::keyword("w"),
            Sexpr::keyword("measure"),
        ]);
        let note = compile_rest(&sexpr).unwrap();

        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Rest(r) = &full_note.content {
                assert_eq!(r.measure, Some(YesNo::Yes));
            } else {
                panic!("Expected Rest");
            }
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_rest_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(compile_rest(&sexpr).is_err());
    }

    #[test]
    fn test_compile_rest_wrong_head() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
        ]);
        assert!(compile_rest(&sexpr).is_err());
    }

    #[test]
    fn test_compile_rest_not_list() {
        let sexpr = Sexpr::symbol("rest");
        assert!(compile_rest(&sexpr).is_err());
    }

    // === compile_fermata_note tests ===

    #[test]
    fn test_compile_fermata_note_basic() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::C,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: Some(1),
            staff: Some(1),
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            tie: None,
            slur: None,
            lyric: None,
        };

        let note = compile_fermata_note(&fermata_note).unwrap();
        assert_eq!(note.voice, Some("1".to_string()));
        assert_eq!(note.staff, Some(1));
    }

    #[test]
    fn test_compile_fermata_note_with_articulations() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::D,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![Articulation::Staccato, Articulation::Accent],
            ornaments: vec![],
            tie: None,
            slur: None,
            lyric: None,
        };

        let note = compile_fermata_note(&fermata_note).unwrap();
        assert!(!note.notations.is_empty());
    }

    #[test]
    fn test_compile_fermata_note_with_ornaments() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::E,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![],
            ornaments: vec![Ornament::Trill],
            tie: None,
            slur: None,
            lyric: None,
        };

        let note = compile_fermata_note(&fermata_note).unwrap();
        assert!(!note.notations.is_empty());

        let has_ornaments = note.notations.iter().any(|n| {
            n.content.iter().any(|c| matches!(c, NotationContent::Ornaments(_)))
        });
        assert!(has_ornaments);
    }

    // === compile_fermata_rest tests ===

    #[test]
    fn test_compile_fermata_rest_basic() {
        let fermata_rest = FermataRest {
            duration: FermataDuration::default(),
            voice: Some(1),
            staff: Some(1),
            measure_rest: false,
        };

        let note = compile_fermata_rest(&fermata_rest).unwrap();
        assert_eq!(note.voice, Some("1".to_string()));
        assert_eq!(note.staff, Some(1));

        if let NoteContent::Regular { full_note, .. } = &note.content {
            assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_fermata_rest_measure() {
        let fermata_rest = FermataRest {
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            measure_rest: true,
        };

        let note = compile_fermata_rest(&fermata_rest).unwrap();

        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Rest(r) = &full_note.content {
                assert_eq!(r.measure, Some(YesNo::Yes));
            } else {
                panic!("Expected Rest");
            }
        } else {
            panic!("Expected Regular");
        }
    }

    // === compile_notations tests ===

    #[test]
    fn test_compile_notations_empty() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::C,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            tie: None,
            slur: None,
            lyric: None,
        };

        let notations = compile_notations(&fermata_note).unwrap();
        assert!(notations.is_empty());
    }

    #[test]
    fn test_compile_notations_with_tie() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::C,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            tie: Some(StartStop::Start),
            slur: None,
            lyric: None,
        };

        let notations = compile_notations(&fermata_note).unwrap();
        assert!(!notations.is_empty());

        let has_tied = notations[0].content.iter().any(|c| matches!(c, NotationContent::Tied(_)));
        assert!(has_tied);
    }

    #[test]
    fn test_compile_notations_with_slur() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::C,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            tie: None,
            slur: Some(StartStop::Start),
            lyric: None,
        };

        let notations = compile_notations(&fermata_note).unwrap();
        assert!(!notations.is_empty());

        let has_slur = notations[0].content.iter().any(|c| matches!(c, NotationContent::Slur(_)));
        assert!(has_slur);
    }

    #[test]
    fn test_compile_notations_with_multiple() {
        let fermata_note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::C,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![Articulation::Staccato],
            ornaments: vec![Ornament::Trill],
            tie: Some(StartStop::Start),
            slur: Some(StartStop::Start),
            lyric: None,
        };

        let notations = compile_notations(&fermata_note).unwrap();
        assert!(!notations.is_empty());

        // Should have tied, slur, articulations, and ornaments
        let content = &notations[0].content;
        assert!(content.iter().any(|c| matches!(c, NotationContent::Tied(_))));
        assert!(content.iter().any(|c| matches!(c, NotationContent::Slur(_))));
        assert!(content.iter().any(|c| matches!(c, NotationContent::Articulations(_))));
        assert!(content.iter().any(|c| matches!(c, NotationContent::Ornaments(_))));
    }
}
