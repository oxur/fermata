//! Chord compilation for Fermata syntax.
//!
//! This module handles compiling chord S-expressions into IR types.
//! A chord is a group of notes sounding simultaneously, represented
//! in MusicXML as multiple Note elements where subsequent notes have
//! chord=true.

use crate::ir::beam::Stem;
use crate::ir::common::{Position, UpDown, YesNo};
use crate::ir::notation::{
    Arpeggiate, ArticulationElement, Articulations, NotationContent, Notations,
};
use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
use crate::lang::ast::{
    ArpeggiateDirection, Articulation, FermataChord, FermataDuration, FermataPitch, PitchStep,
    StemDirection,
};
use crate::lang::defaults::DEFAULT_DIVISIONS;
use crate::lang::duration::{compile_dots, compile_duration_divisions_with, compile_duration_type};
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::note::{compile_stem_ir, parse_stem, parse_u32};
use crate::lang::pitch::{compile_pitch, parse_pitch_str};
use crate::sexpr::Sexpr;

/// Compile a chord S-expression into a Vec<Note>.
///
/// A chord compiles to multiple IR Notes where the first note has chord=false
/// and subsequent notes have chord=true.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::chord::compile_chord;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(chord (c4 e4 g4) :q)")?;
/// let notes = compile_chord(&sexpr)?;
/// assert_eq!(notes.len(), 3);
/// ```
pub fn compile_chord(sexpr: &Sexpr) -> CompileResult<Vec<Note>> {
    match sexpr {
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidChord {
                    reason: "empty chord list".to_string(),
                });
            }

            // Check for 'chord' head
            if !items[0].is_symbol("chord") {
                return Err(CompileError::InvalidChord {
                    reason: format!("expected 'chord', got {:?}", items[0]),
                });
            }

            let fermata_chord = parse_chord_form(&items[1..])?;
            compile_fermata_chord(&fermata_chord)
        }
        _ => Err(CompileError::InvalidChord {
            reason: format!("expected chord list, got {:?}", sexpr),
        }),
    }
}

/// Parse chord arguments from S-expression items into a FermataChord AST.
///
/// Expected format: `(pitches...) duration [keywords...]`
/// - pitches: a list of pitch symbols like (c4 e4 g4)
/// - duration: :q, :h, :w, :8, etc.
/// - keywords: :voice N, :staff N, :stem up/down, :arpeggiate up/down/none, etc.
pub fn parse_chord_form(items: &[Sexpr]) -> CompileResult<FermataChord> {
    if items.is_empty() {
        return Err(CompileError::InvalidChord {
            reason: "chord requires pitches".to_string(),
        });
    }

    // First item should be a list of pitches
    let pitches = match &items[0] {
        Sexpr::List(pitch_items) => {
            if pitch_items.is_empty() {
                return Err(CompileError::InvalidChord {
                    reason: "chord requires at least one pitch".to_string(),
                });
            }
            let mut pitches = Vec::new();
            for pitch_item in pitch_items {
                let pitch_str = pitch_item.as_symbol().ok_or_else(|| CompileError::InvalidChord {
                    reason: format!("expected pitch symbol, got {:?}", pitch_item),
                })?;
                pitches.push(parse_pitch_str(pitch_str)?);
            }
            pitches
        }
        _ => {
            return Err(CompileError::InvalidChord {
                reason: format!("expected pitch list, got {:?}", items[0]),
            });
        }
    };

    // Second item is duration (if present and is a keyword/symbol)
    let (duration, remaining_start) = if items.len() > 1 {
        if let Some(dur_str) = items[1].as_keyword().or_else(|| items[1].as_symbol()) {
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
    let mut arpeggiate: Option<ArpeggiateDirection> = None;
    let mut articulations: Vec<Articulation> = Vec::new();

    let mut i = remaining_start;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "voice" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidChord {
                            reason: "missing :voice value".to_string(),
                        });
                    }
                    voice = Some(parse_u32(&items[i + 1])?);
                    i += 2;
                }
                "staff" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidChord {
                            reason: "missing :staff value".to_string(),
                        });
                    }
                    staff = Some(parse_u32(&items[i + 1])?);
                    i += 2;
                }
                "stem" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidChord {
                            reason: "missing :stem value".to_string(),
                        });
                    }
                    stem = Some(parse_stem(&items[i + 1])?);
                    i += 2;
                }
                "arpeggiate" => {
                    if i + 1 >= items.len() {
                        // Just :arpeggiate without a direction means default (None direction)
                        arpeggiate = Some(ArpeggiateDirection::None);
                        i += 1;
                    } else if let Some(dir_str) = items[i + 1].as_symbol().or_else(|| items[i + 1].as_keyword()) {
                        arpeggiate = Some(parse_arpeggiate_direction(dir_str)?);
                        i += 2;
                    } else {
                        arpeggiate = Some(ArpeggiateDirection::None);
                        i += 1;
                    }
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

    Ok(FermataChord {
        pitches,
        duration,
        voice,
        staff,
        stem,
        articulations,
        ornaments: vec![],
        arpeggiate,
    })
}

/// Parse a chord form from S-expression items directly to AST.
/// This is the public wrapper for external use.
pub fn parse_chord_form_to_ast(items: &[Sexpr]) -> CompileResult<FermataChord> {
    parse_chord_form(items)
}

/// Parse arpeggiate direction from a string.
fn parse_arpeggiate_direction(s: &str) -> CompileResult<ArpeggiateDirection> {
    match s.to_lowercase().as_str() {
        "up" => Ok(ArpeggiateDirection::Up),
        "down" => Ok(ArpeggiateDirection::Down),
        "none" | "" => Ok(ArpeggiateDirection::None),
        _ => Err(CompileError::InvalidChord {
            reason: format!("invalid arpeggiate direction '{}', expected up, down, or none", s),
        }),
    }
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

/// Compile a FermataChord to a Vec<Note>.
///
/// The first note in the chord has chord=false (it's the root).
/// Subsequent notes have chord=true.
pub fn compile_fermata_chord(chord: &FermataChord) -> CompileResult<Vec<Note>> {
    if chord.pitches.is_empty() {
        return Err(CompileError::InvalidChord {
            reason: "chord must have at least one pitch".to_string(),
        });
    }

    let divisions = DEFAULT_DIVISIONS as u64;
    let duration_divisions = compile_duration_divisions_with(&chord.duration, divisions);
    let note_type = Some(compile_duration_type(&chord.duration.base));
    let dots = compile_dots(chord.duration.dots);
    let stem_ir: Option<Stem> = chord.stem.map(compile_stem_ir);

    // Build articulations for all notes
    let articulations_content = compile_articulations(&chord.articulations);

    // Build arpeggiate notation (only for first note typically, but we put on all)
    let arpeggiate_notation = chord.arpeggiate.map(compile_arpeggiate);

    let mut notes = Vec::with_capacity(chord.pitches.len());

    for (idx, fermata_pitch) in chord.pitches.iter().enumerate() {
        let is_chord_note = idx > 0; // First note is NOT a chord note
        let ir_pitch = compile_pitch(fermata_pitch)?;

        // Build notations for this note
        let mut notations_content: Vec<NotationContent> = Vec::new();

        // Add articulations to all notes in the chord
        if let Some(ref arts) = articulations_content {
            notations_content.push(NotationContent::Articulations(Box::new(arts.clone())));
        }

        // Add arpeggiate to all notes (MusicXML convention)
        if let Some(ref arp) = arpeggiate_notation {
            notations_content.push(NotationContent::Arpeggiate(arp.clone()));
        }

        let notations = if notations_content.is_empty() {
            vec![]
        } else {
            vec![Notations {
                print_object: None,
                content: notations_content,
                editorial: Default::default(),
            }]
        };

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
                    chord: is_chord_note,
                    content: PitchRestUnpitched::Pitch(ir_pitch),
                },
                duration: duration_divisions,
                ties: vec![],
            },
            instrument: vec![],
            voice: chord.voice.map(|v| v.to_string()),
            r#type: note_type.clone(),
            dots: dots.clone(),
            accidental: None,
            time_modification: None,
            stem: stem_ir.clone(),
            notehead: None,
            staff: chord.staff.map(|s| s as u16),
            beams: vec![],
            notations,
            lyrics: vec![],
        };

        notes.push(note);
    }

    Ok(notes)
}

/// Compile articulations to IR Articulations.
fn compile_articulations(articulations: &[Articulation]) -> Option<Articulations> {
    if articulations.is_empty() {
        return None;
    }

    use crate::ir::common::EmptyPlacement;
    use crate::ir::notation::StrongAccent;

    let content: Vec<ArticulationElement> = articulations
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

    Some(Articulations { content })
}

/// Compile arpeggiate direction to IR Arpeggiate.
fn compile_arpeggiate(direction: ArpeggiateDirection) -> Arpeggiate {
    let ir_direction = match direction {
        ArpeggiateDirection::Up => Some(UpDown::Up),
        ArpeggiateDirection::Down => Some(UpDown::Down),
        ArpeggiateDirection::None => None,
    };

    Arpeggiate {
        number: None,
        direction: ir_direction,
        position: Position::default(),
        color: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::pitch::Step as IrStep;

    // === parse_arpeggiate_direction tests ===

    #[test]
    fn test_parse_arpeggiate_direction_up() {
        assert_eq!(parse_arpeggiate_direction("up").unwrap(), ArpeggiateDirection::Up);
        assert_eq!(parse_arpeggiate_direction("UP").unwrap(), ArpeggiateDirection::Up);
    }

    #[test]
    fn test_parse_arpeggiate_direction_down() {
        assert_eq!(parse_arpeggiate_direction("down").unwrap(), ArpeggiateDirection::Down);
    }

    #[test]
    fn test_parse_arpeggiate_direction_none() {
        assert_eq!(parse_arpeggiate_direction("none").unwrap(), ArpeggiateDirection::None);
        assert_eq!(parse_arpeggiate_direction("").unwrap(), ArpeggiateDirection::None);
    }

    #[test]
    fn test_parse_arpeggiate_direction_invalid() {
        assert!(parse_arpeggiate_direction("sideways").is_err());
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
        assert!(!is_duration_keyword("arpeggiate"));
    }

    // === parse_chord_form tests ===

    #[test]
    fn test_parse_chord_form_simple() {
        let items = vec![
            Sexpr::list(vec![
                Sexpr::symbol("c4"),
                Sexpr::symbol("e4"),
                Sexpr::symbol("g4"),
            ]),
            Sexpr::keyword("q"),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert_eq!(chord.pitches.len(), 3);
        assert_eq!(chord.pitches[0].step, PitchStep::C);
        assert_eq!(chord.pitches[1].step, PitchStep::E);
        assert_eq!(chord.pitches[2].step, PitchStep::G);
    }

    #[test]
    fn test_parse_chord_form_with_voice() {
        let items = vec![
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("voice"),
            Sexpr::Integer(1),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert_eq!(chord.voice, Some(1));
    }

    #[test]
    fn test_parse_chord_form_with_staff() {
        let items = vec![
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("g4")]),
            Sexpr::keyword("h"),
            Sexpr::keyword("staff"),
            Sexpr::Integer(2),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert_eq!(chord.staff, Some(2));
    }

    #[test]
    fn test_parse_chord_form_with_stem() {
        let items = vec![
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("stem"),
            Sexpr::symbol("up"),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert_eq!(chord.stem, Some(StemDirection::Up));
    }

    #[test]
    fn test_parse_chord_form_with_arpeggiate() {
        let items = vec![
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4"), Sexpr::symbol("g4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("arpeggiate"),
            Sexpr::symbol("up"),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert_eq!(chord.arpeggiate, Some(ArpeggiateDirection::Up));
    }

    #[test]
    fn test_parse_chord_form_with_staccato() {
        let items = vec![
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("staccato"),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert!(chord.articulations.contains(&Articulation::Staccato));
    }

    #[test]
    fn test_parse_chord_form_with_accent() {
        let items = vec![
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("accent"),
        ];
        let chord = parse_chord_form(&items).unwrap();
        assert!(chord.articulations.contains(&Articulation::Accent));
    }

    #[test]
    fn test_parse_chord_form_empty() {
        let items: Vec<Sexpr> = vec![];
        assert!(parse_chord_form(&items).is_err());
    }

    #[test]
    fn test_parse_chord_form_empty_pitches() {
        let items = vec![Sexpr::list(vec![])];
        assert!(parse_chord_form(&items).is_err());
    }

    // === compile_chord tests ===

    #[test]
    fn test_compile_chord_simple() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("chord"),
            Sexpr::list(vec![
                Sexpr::symbol("c4"),
                Sexpr::symbol("e4"),
                Sexpr::symbol("g4"),
            ]),
            Sexpr::keyword("q"),
        ]);
        let notes = compile_chord(&sexpr).unwrap();

        assert_eq!(notes.len(), 3);

        // First note should NOT be a chord note
        if let NoteContent::Regular { full_note, .. } = &notes[0].content {
            assert!(!full_note.chord);
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::C);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Regular");
        }

        // Second note SHOULD be a chord note
        if let NoteContent::Regular { full_note, .. } = &notes[1].content {
            assert!(full_note.chord);
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::E);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Regular");
        }

        // Third note SHOULD be a chord note
        if let NoteContent::Regular { full_note, .. } = &notes[2].content {
            assert!(full_note.chord);
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::G);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_chord_with_sharps() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("chord"),
            Sexpr::list(vec![
                Sexpr::symbol("c#4"),
                Sexpr::symbol("f#4"),
            ]),
            Sexpr::keyword("h"),
        ]);
        let notes = compile_chord(&sexpr).unwrap();

        assert_eq!(notes.len(), 2);

        if let NoteContent::Regular { full_note, .. } = &notes[0].content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, IrStep::C);
                assert_eq!(p.alter, Some(1.0));
            }
        }
    }

    #[test]
    fn test_compile_chord_with_voice_and_staff() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("chord"),
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("voice"),
            Sexpr::Integer(2),
            Sexpr::keyword("staff"),
            Sexpr::Integer(1),
        ]);
        let notes = compile_chord(&sexpr).unwrap();

        for note in &notes {
            assert_eq!(note.voice, Some("2".to_string()));
            assert_eq!(note.staff, Some(1));
        }
    }

    #[test]
    fn test_compile_chord_with_arpeggiate() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("chord"),
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4"), Sexpr::symbol("g4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("arpeggiate"),
            Sexpr::symbol("up"),
        ]);
        let notes = compile_chord(&sexpr).unwrap();

        // All notes should have arpeggiate notation
        for note in &notes {
            let has_arpeggiate = note.notations.iter().any(|n| {
                n.content.iter().any(|c| matches!(c, NotationContent::Arpeggiate(_)))
            });
            assert!(has_arpeggiate);
        }
    }

    #[test]
    fn test_compile_chord_with_articulations() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("chord"),
            Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
            Sexpr::keyword("q"),
            Sexpr::keyword("staccato"),
            Sexpr::keyword("accent"),
        ]);
        let notes = compile_chord(&sexpr).unwrap();

        // All notes should have articulations
        for note in &notes {
            let has_articulations = note.notations.iter().any(|n| {
                n.content.iter().any(|c| matches!(c, NotationContent::Articulations(_)))
            });
            assert!(has_articulations);
        }
    }

    #[test]
    fn test_compile_chord_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(compile_chord(&sexpr).is_err());
    }

    #[test]
    fn test_compile_chord_wrong_head() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
        ]);
        assert!(compile_chord(&sexpr).is_err());
    }

    #[test]
    fn test_compile_chord_not_list() {
        let sexpr = Sexpr::symbol("chord");
        assert!(compile_chord(&sexpr).is_err());
    }

    // === compile_fermata_chord tests ===

    #[test]
    fn test_compile_fermata_chord_basic() {
        let chord = FermataChord {
            pitches: vec![
                FermataPitch { step: PitchStep::C, alter: None, octave: 4 },
                FermataPitch { step: PitchStep::E, alter: None, octave: 4 },
            ],
            duration: FermataDuration::default(),
            voice: Some(1),
            staff: Some(1),
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            arpeggiate: None,
        };

        let notes = compile_fermata_chord(&chord).unwrap();
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].voice, Some("1".to_string()));
        assert_eq!(notes[0].staff, Some(1));
    }

    #[test]
    fn test_compile_fermata_chord_empty_pitches() {
        let chord = FermataChord {
            pitches: vec![],
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            arpeggiate: None,
        };

        assert!(compile_fermata_chord(&chord).is_err());
    }

    // === compile_articulations tests ===

    #[test]
    fn test_compile_articulations_empty() {
        let result = compile_articulations(&[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_compile_articulations_staccato() {
        let result = compile_articulations(&[Articulation::Staccato]);
        assert!(result.is_some());
        let arts = result.unwrap();
        assert_eq!(arts.content.len(), 1);
    }

    #[test]
    fn test_compile_articulations_multiple() {
        let result = compile_articulations(&[Articulation::Staccato, Articulation::Accent]);
        assert!(result.is_some());
        let arts = result.unwrap();
        assert_eq!(arts.content.len(), 2);
    }

    // === compile_arpeggiate tests ===

    #[test]
    fn test_compile_arpeggiate_up() {
        let arp = compile_arpeggiate(ArpeggiateDirection::Up);
        assert_eq!(arp.direction, Some(UpDown::Up));
    }

    #[test]
    fn test_compile_arpeggiate_down() {
        let arp = compile_arpeggiate(ArpeggiateDirection::Down);
        assert_eq!(arp.direction, Some(UpDown::Down));
    }

    #[test]
    fn test_compile_arpeggiate_none() {
        let arp = compile_arpeggiate(ArpeggiateDirection::None);
        assert!(arp.direction.is_none());
    }
}
