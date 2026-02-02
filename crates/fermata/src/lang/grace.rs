//! Grace note compilation for Fermata syntax.
//!
//! This module handles compiling grace note S-expressions into IR types.
//! Grace notes are ornamental notes that do not take up time in the measure.

use crate::ir::common::{Position, YesNo};
use crate::ir::note::{FullNote, Grace, Note, NoteContent, PitchRestUnpitched};
use crate::lang::ast::{FermataDuration, FermataGraceNote, FermataPitch};
use crate::lang::duration::{compile_dots, compile_duration_type};
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::pitch::{compile_pitch, parse_pitch_str};
use crate::sexpr::Sexpr;

/// Compile a grace note S-expression into an IR Note.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::grace::compile_grace_note;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(grace c4 :slash)")?;
/// let note = compile_grace_note(&sexpr)?;
/// ```
pub fn compile_grace_note(sexpr: &Sexpr) -> CompileResult<Note> {
    match sexpr {
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidNote("empty grace note list".to_string()));
            }

            // Check for 'grace' head
            if !items[0].is_symbol("grace") {
                return Err(CompileError::InvalidNote(
                    format!("expected 'grace', got {:?}", items[0])
                ));
            }

            let fermata_grace = parse_grace_form(&items[1..])?;
            compile_fermata_grace(&fermata_grace)
        }
        _ => Err(CompileError::InvalidNote(
            format!("expected grace note list, got {:?}", sexpr)
        )),
    }
}

/// Parse grace note arguments from S-expression items into a FermataGraceNote AST.
///
/// Expected format: `pitch [keywords...]`
/// - pitch: "c4", "f#5", etc.
/// - keywords: :slash, :duration :8, etc.
pub fn parse_grace_form(items: &[Sexpr]) -> CompileResult<FermataGraceNote> {
    if items.is_empty() {
        return Err(CompileError::InvalidNote("grace note requires pitch".to_string()));
    }

    // First item is pitch
    let pitch = parse_pitch_str(items[0].as_symbol().ok_or_else(|| {
        CompileError::InvalidNote(format!("expected pitch symbol, got {:?}", items[0]))
    })?)?;

    // Parse remaining keyword arguments
    let mut slash = false;
    let mut duration: Option<FermataDuration> = None;

    let mut i = 1;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "slash" => {
                    slash = true;
                    i += 1;
                }
                "duration" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidNote("missing :duration value".to_string()));
                    }
                    // Parse the duration value
                    if let Some(dur_str) = items[i + 1].as_keyword().or_else(|| items[i + 1].as_symbol()) {
                        duration = Some(crate::lang::duration::parse_duration(dur_str)?);
                    } else {
                        return Err(CompileError::InvalidNote(
                            format!("expected duration keyword, got {:?}", items[i + 1])
                        ));
                    }
                    i += 2;
                }
                // Also accept duration keywords directly (e.g., :8, :16)
                _ if is_duration_keyword(kw) => {
                    duration = Some(crate::lang::duration::parse_duration(kw)?);
                    i += 1;
                }
                _ => {
                    // Unknown keyword - skip it
                    i += 1;
                }
            }
        } else if let Some(sym) = items[i].as_symbol() {
            // Check if it's a duration symbol
            if is_duration_keyword(sym) {
                duration = Some(crate::lang::duration::parse_duration(sym)?);
            }
            i += 1;
        } else {
            // Skip non-keyword items
            i += 1;
        }
    }

    Ok(FermataGraceNote {
        pitch,
        slash,
        duration,
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

/// Compile a FermataGraceNote to an IR Note.
pub fn compile_fermata_grace(grace_note: &FermataGraceNote) -> CompileResult<Note> {
    let ir_pitch = compile_pitch(&grace_note.pitch)?;

    // Build the Grace struct
    let grace = Grace {
        steal_time_previous: None,
        steal_time_following: None,
        make_time: None,
        slash: if grace_note.slash { Some(YesNo::Yes) } else { None },
    };

    // Determine note type from duration if provided
    let note_type = grace_note.duration.as_ref().map(|d| compile_duration_type(&d.base));
    let dots = grace_note.duration.as_ref().map(|d| compile_dots(d.dots)).unwrap_or_default();

    Ok(Note {
        position: Position::default(),
        dynamics: None,
        end_dynamics: None,
        attack: None,
        release: None,
        pizzicato: None,
        print_object: None,
        content: NoteContent::Grace {
            grace,
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Pitch(ir_pitch),
            },
            ties: vec![],
        },
        instrument: vec![],
        voice: None,
        r#type: note_type,
        dots,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::pitch::Step as IrStep;
    use crate::lang::ast::{DurationBase, PitchStep};

    // === is_duration_keyword tests ===

    #[test]
    fn test_is_duration_keyword_short_forms() {
        assert!(is_duration_keyword("8"));
        assert!(is_duration_keyword("16"));
        assert!(is_duration_keyword("32"));
    }

    #[test]
    fn test_is_duration_keyword_full_names() {
        assert!(is_duration_keyword("eighth"));
        assert!(is_duration_keyword("sixteenth"));
    }

    #[test]
    fn test_is_duration_keyword_with_colon() {
        assert!(is_duration_keyword(":8"));
        assert!(is_duration_keyword(":16"));
    }

    #[test]
    fn test_is_duration_keyword_not_duration() {
        assert!(!is_duration_keyword("slash"));
        assert!(!is_duration_keyword("duration"));
    }

    // === parse_grace_form tests ===

    #[test]
    fn test_parse_grace_form_simple() {
        let items = vec![Sexpr::symbol("c4")];
        let grace = parse_grace_form(&items).unwrap();
        assert_eq!(grace.pitch.step, PitchStep::C);
        assert_eq!(grace.pitch.octave, 4);
        assert!(!grace.slash);
        assert!(grace.duration.is_none());
    }

    #[test]
    fn test_parse_grace_form_with_slash() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("slash"),
        ];
        let grace = parse_grace_form(&items).unwrap();
        assert!(grace.slash);
    }

    #[test]
    fn test_parse_grace_form_with_duration() {
        let items = vec![
            Sexpr::symbol("c4"),
            Sexpr::keyword("duration"),
            Sexpr::keyword("16"),
        ];
        let grace = parse_grace_form(&items).unwrap();
        assert!(grace.duration.is_some());
        let dur = grace.duration.unwrap();
        assert_eq!(dur.base, DurationBase::Sixteenth);
    }

    #[test]
    fn test_parse_grace_form_with_direct_duration() {
        let items = vec![
            Sexpr::symbol("d5"),
            Sexpr::keyword("8"),
        ];
        let grace = parse_grace_form(&items).unwrap();
        assert!(grace.duration.is_some());
        let dur = grace.duration.unwrap();
        assert_eq!(dur.base, DurationBase::Eighth);
    }

    #[test]
    fn test_parse_grace_form_with_slash_and_duration() {
        let items = vec![
            Sexpr::symbol("e4"),
            Sexpr::keyword("slash"),
            Sexpr::keyword("16"),
        ];
        let grace = parse_grace_form(&items).unwrap();
        assert!(grace.slash);
        assert!(grace.duration.is_some());
    }

    #[test]
    fn test_parse_grace_form_with_sharp() {
        let items = vec![
            Sexpr::symbol("f#5"),
            Sexpr::keyword("slash"),
        ];
        let grace = parse_grace_form(&items).unwrap();
        assert_eq!(grace.pitch.step, PitchStep::F);
        assert!(grace.pitch.alter.is_some());
    }

    #[test]
    fn test_parse_grace_form_empty() {
        let items: Vec<Sexpr> = vec![];
        assert!(parse_grace_form(&items).is_err());
    }

    // === compile_grace_note tests ===

    #[test]
    fn test_compile_grace_note_simple() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("grace"),
            Sexpr::symbol("c4"),
        ]);
        let note = compile_grace_note(&sexpr).unwrap();

        if let NoteContent::Grace { grace, full_note, .. } = &note.content {
            assert!(grace.slash.is_none());
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::C);
                assert_eq!(p.octave, 4);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Grace");
        }
    }

    #[test]
    fn test_compile_grace_note_with_slash() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("grace"),
            Sexpr::symbol("d4"),
            Sexpr::keyword("slash"),
        ]);
        let note = compile_grace_note(&sexpr).unwrap();

        if let NoteContent::Grace { grace, .. } = &note.content {
            assert_eq!(grace.slash, Some(YesNo::Yes));
        } else {
            panic!("Expected Grace");
        }
    }

    #[test]
    fn test_compile_grace_note_with_duration() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("grace"),
            Sexpr::symbol("e4"),
            Sexpr::keyword("16"),
        ]);
        let note = compile_grace_note(&sexpr).unwrap();

        assert!(note.r#type.is_some());
        let note_type = note.r#type.unwrap();
        assert_eq!(note_type.value, crate::ir::duration::NoteTypeValue::N16th);
    }

    #[test]
    fn test_compile_grace_note_with_sharp() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("grace"),
            Sexpr::symbol("g#5"),
        ]);
        let note = compile_grace_note(&sexpr).unwrap();

        if let NoteContent::Grace { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::G);
                assert_eq!(p.alter, Some(1.0));
                assert_eq!(p.octave, 5);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Grace");
        }
    }

    #[test]
    fn test_compile_grace_note_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(compile_grace_note(&sexpr).is_err());
    }

    #[test]
    fn test_compile_grace_note_wrong_head() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
        ]);
        assert!(compile_grace_note(&sexpr).is_err());
    }

    #[test]
    fn test_compile_grace_note_not_list() {
        let sexpr = Sexpr::symbol("grace");
        assert!(compile_grace_note(&sexpr).is_err());
    }

    // === compile_fermata_grace tests ===

    #[test]
    fn test_compile_fermata_grace_basic() {
        let grace_note = FermataGraceNote {
            pitch: FermataPitch { step: PitchStep::C, alter: None, octave: 4 },
            slash: false,
            duration: None,
        };

        let note = compile_fermata_grace(&grace_note).unwrap();

        if let NoteContent::Grace { grace, full_note, .. } = &note.content {
            assert!(grace.slash.is_none());
            assert!(!full_note.chord);
        } else {
            panic!("Expected Grace");
        }
    }

    #[test]
    fn test_compile_fermata_grace_with_slash() {
        let grace_note = FermataGraceNote {
            pitch: FermataPitch { step: PitchStep::D, alter: None, octave: 4 },
            slash: true,
            duration: None,
        };

        let note = compile_fermata_grace(&grace_note).unwrap();

        if let NoteContent::Grace { grace, .. } = &note.content {
            assert_eq!(grace.slash, Some(YesNo::Yes));
        } else {
            panic!("Expected Grace");
        }
    }

    #[test]
    fn test_compile_fermata_grace_with_duration() {
        let grace_note = FermataGraceNote {
            pitch: FermataPitch { step: PitchStep::E, alter: None, octave: 4 },
            slash: true,
            duration: Some(FermataDuration {
                base: DurationBase::Sixteenth,
                dots: 0,
            }),
        };

        let note = compile_fermata_grace(&grace_note).unwrap();

        assert!(note.r#type.is_some());
        let note_type = note.r#type.unwrap();
        assert_eq!(note_type.value, crate::ir::duration::NoteTypeValue::N16th);
    }

    #[test]
    fn test_compile_fermata_grace_with_dotted_duration() {
        let grace_note = FermataGraceNote {
            pitch: FermataPitch { step: PitchStep::F, alter: None, octave: 4 },
            slash: false,
            duration: Some(FermataDuration {
                base: DurationBase::Eighth,
                dots: 1,
            }),
        };

        let note = compile_fermata_grace(&grace_note).unwrap();

        assert!(note.r#type.is_some());
        assert_eq!(note.dots.len(), 1);
    }
}
