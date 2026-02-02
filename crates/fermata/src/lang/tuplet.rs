//! Tuplet compilation for Fermata syntax.
//!
//! This module handles compiling tuplet S-expressions into IR types.
//! A tuplet modifies the time value of notes, such as triplets (3 in the time of 2).

use crate::ir::common::{Position, StartStop, YesNo};
use crate::ir::duration::TimeModification;
use crate::ir::notation::{
    NotationContent, Notations, ShowTuplet, Tuplet, TupletNumber, TupletPortion,
};
use crate::ir::note::Note;
use crate::lang::ast::{FermataTuplet, MeasureElement};
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::note::parse_u32;
use crate::sexpr::Sexpr;

/// Compile a tuplet S-expression into a Vec<Note>.
///
/// The tuplet wraps a sequence of notes/rests/chords and applies a time modification
/// to each. Tuplet notation brackets are added to the first and last notes.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::tuplet::compile_tuplet;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(tuplet 3:2 (note c4 :8) (note d4 :8) (note e4 :8))")?;
/// let notes = compile_tuplet(&sexpr)?;
/// assert_eq!(notes.len(), 3);
/// ```
pub fn compile_tuplet(sexpr: &Sexpr) -> CompileResult<Vec<Note>> {
    match sexpr {
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidTuplet {
                    reason: "empty tuplet list".to_string(),
                });
            }

            // Check for 'tuplet' head
            if !items[0].is_symbol("tuplet") {
                return Err(CompileError::InvalidTuplet {
                    reason: format!("expected 'tuplet', got {:?}", items[0]),
                });
            }

            let fermata_tuplet = parse_tuplet_form(&items[1..])?;
            compile_fermata_tuplet(&fermata_tuplet)
        }
        _ => Err(CompileError::InvalidTuplet {
            reason: format!("expected tuplet list, got {:?}", sexpr),
        }),
    }
}

/// Parse tuplet arguments from S-expression items into a FermataTuplet AST.
///
/// Expected format: `ratio notes...`
/// - ratio: "3:2" or "3/2" or separate "3" "2" or just "3" (implies 3:2)
/// - notes: sequence of note, rest, or chord forms
pub fn parse_tuplet_form(items: &[Sexpr]) -> CompileResult<FermataTuplet> {
    if items.is_empty() {
        return Err(CompileError::InvalidTuplet {
            reason: "tuplet requires ratio".to_string(),
        });
    }

    // Parse the ratio (actual:normal)
    let (actual, normal, remaining_start) = parse_ratio(items)?;

    // Parse the notes/rests/chords inside the tuplet
    let mut notes: Vec<MeasureElement> = Vec::new();

    for item in &items[remaining_start..] {
        // Each item should be a list representing a note, rest, or chord
        match item {
            Sexpr::List(sub_items) if !sub_items.is_empty() => {
                if let Some(head) = sub_items[0].as_symbol() {
                    let element = match head {
                        "note" => {
                            let fermata_note = crate::lang::note::parse_note_form(&sub_items[1..])?;
                            MeasureElement::Note(fermata_note)
                        }
                        "rest" => {
                            let fermata_rest = crate::lang::note::parse_rest_form(&sub_items[1..])?;
                            MeasureElement::Rest(fermata_rest)
                        }
                        "chord" => {
                            let fermata_chord =
                                crate::lang::chord::parse_chord_form(&sub_items[1..])?;
                            MeasureElement::Chord(fermata_chord)
                        }
                        _ => {
                            return Err(CompileError::InvalidTuplet {
                                reason: format!(
                                    "unexpected element '{}' in tuplet, expected note, rest, or chord",
                                    head
                                ),
                            });
                        }
                    };
                    notes.push(element);
                } else {
                    return Err(CompileError::InvalidTuplet {
                        reason: format!("expected note/rest/chord form, got {:?}", item),
                    });
                }
            }
            _ => {
                return Err(CompileError::InvalidTuplet {
                    reason: format!("expected note/rest/chord list, got {:?}", item),
                });
            }
        }
    }

    if notes.is_empty() {
        return Err(CompileError::InvalidTuplet {
            reason: "tuplet requires at least one note".to_string(),
        });
    }

    Ok(FermataTuplet {
        actual,
        normal,
        notes,
    })
}

/// Parse the tuplet ratio from the beginning of items.
///
/// Supports formats:
/// - "3:2" - colon-separated
/// - "3/2" - slash-separated
/// - Symbol("3"), Symbol("2") or Integer(3), Integer(2) - separate values
/// - Just "3" or Integer(3) - implies normal=2 for triplets
fn parse_ratio(items: &[Sexpr]) -> CompileResult<(u32, u32, usize)> {
    if items.is_empty() {
        return Err(CompileError::InvalidTuplet {
            reason: "missing ratio".to_string(),
        });
    }

    // Try to parse as a ratio string first (e.g., "3:2" or "3/2")
    if let Some(ratio_str) = items[0].as_symbol() {
        if ratio_str.contains(':') {
            let parts: Vec<&str> = ratio_str.split(':').collect();
            if parts.len() == 2 {
                let actual: u32 = parts[0].parse().map_err(|_| CompileError::InvalidTuplet {
                    reason: format!("invalid actual value in ratio '{}'", ratio_str),
                })?;
                let normal: u32 = parts[1].parse().map_err(|_| CompileError::InvalidTuplet {
                    reason: format!("invalid normal value in ratio '{}'", ratio_str),
                })?;
                return Ok((actual, normal, 1));
            }
        } else if ratio_str.contains('/') {
            let parts: Vec<&str> = ratio_str.split('/').collect();
            if parts.len() == 2 {
                let actual: u32 = parts[0].parse().map_err(|_| CompileError::InvalidTuplet {
                    reason: format!("invalid actual value in ratio '{}'", ratio_str),
                })?;
                let normal: u32 = parts[1].parse().map_err(|_| CompileError::InvalidTuplet {
                    reason: format!("invalid normal value in ratio '{}'", ratio_str),
                })?;
                return Ok((actual, normal, 1));
            }
        } else if let Ok(actual) = ratio_str.parse::<u32>() {
            // Just a number - check if next item is also a number
            if items.len() > 1 {
                if let Ok(normal) = parse_u32(&items[1]) {
                    return Ok((actual, normal, 2));
                }
            }
            // Just one number - assume triplet (3:2)
            let normal = if actual == 3 { 2 } else { actual - 1 };
            return Ok((actual, normal, 1));
        }
    }

    // Try as integer
    if let Sexpr::Integer(actual) = &items[0] {
        if *actual <= 0 {
            return Err(CompileError::InvalidTuplet {
                reason: "actual value must be positive".to_string(),
            });
        }
        let actual = *actual as u32;

        // Check if next item is the normal value
        if items.len() > 1 {
            if let Ok(normal) = parse_u32(&items[1]) {
                return Ok((actual, normal, 2));
            }
        }
        // Default normal value
        let normal = if actual == 3 { 2 } else { actual - 1 };
        return Ok((actual, normal, 1));
    }

    Err(CompileError::InvalidTuplet {
        reason: format!("expected ratio (e.g., 3:2), got {:?}", items[0]),
    })
}

/// Compile a FermataTuplet to a Vec<Note>.
///
/// Each note in the tuplet gets:
/// - A TimeModification specifying the actual:normal ratio
/// - First note gets Tuplet notation with type=Start
/// - Last note gets Tuplet notation with type=Stop
pub fn compile_fermata_tuplet(tuplet: &FermataTuplet) -> CompileResult<Vec<Note>> {
    let time_modification = TimeModification {
        actual_notes: tuplet.actual,
        normal_notes: tuplet.normal,
        normal_type: None,
        normal_dots: 0,
    };

    let mut all_notes: Vec<Note> = Vec::new();

    for (idx, element) in tuplet.notes.iter().enumerate() {
        let is_first = idx == 0;
        let is_last = idx == tuplet.notes.len() - 1;

        // Compile the element to notes
        let mut notes = compile_measure_element(element)?;

        // Apply time modification and tuplet notation to each note
        for note in &mut notes {
            // Apply time modification
            note.time_modification = Some(time_modification.clone());

            // Add tuplet notation for first and last
            if is_first || is_last {
                let tuplet_notation = create_tuplet_notation(
                    if is_first {
                        StartStop::Start
                    } else {
                        StartStop::Stop
                    },
                    tuplet.actual,
                    tuplet.normal,
                );

                // Add to existing notations or create new
                if note.notations.is_empty() {
                    note.notations.push(Notations {
                        print_object: None,
                        content: vec![NotationContent::Tuplet(Box::new(tuplet_notation))],
                        editorial: Default::default(),
                    });
                } else {
                    note.notations[0]
                        .content
                        .push(NotationContent::Tuplet(Box::new(tuplet_notation)));
                }
            }
        }

        all_notes.extend(notes);
    }

    Ok(all_notes)
}

/// Compile a MeasureElement to a Vec<Note>.
fn compile_measure_element(element: &MeasureElement) -> CompileResult<Vec<Note>> {
    match element {
        MeasureElement::Note(fermata_note) => {
            let note = crate::lang::note::compile_fermata_note(fermata_note)?;
            Ok(vec![note])
        }
        MeasureElement::Rest(fermata_rest) => {
            let note = crate::lang::note::compile_fermata_rest(fermata_rest)?;
            Ok(vec![note])
        }
        MeasureElement::Chord(fermata_chord) => {
            crate::lang::chord::compile_fermata_chord(fermata_chord)
        }
        _ => Err(CompileError::InvalidTuplet {
            reason: format!("unsupported element type in tuplet: {:?}", element),
        }),
    }
}

/// Create a Tuplet notation element.
fn create_tuplet_notation(r#type: StartStop, actual: u32, normal: u32) -> Tuplet {
    Tuplet {
        r#type,
        number: Some(1), // Default tuplet number
        bracket: Some(YesNo::Yes),
        show_number: Some(ShowTuplet::Actual),
        show_type: None,
        line_shape: None,
        position: Position::default(),
        placement: None,
        tuplet_actual: Some(TupletPortion {
            tuplet_number: Some(TupletNumber {
                value: actual,
                font: Default::default(),
                color: None,
            }),
            tuplet_type: None,
            tuplet_dots: vec![],
        }),
        tuplet_normal: Some(TupletPortion {
            tuplet_number: Some(TupletNumber {
                value: normal,
                font: Default::default(),
                color: None,
            }),
            tuplet_type: None,
            tuplet_dots: vec![],
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::note::NoteContent;
    use crate::lang::ast::{FermataDuration, FermataNote, FermataPitch, FermataRest, PitchStep};

    // === parse_ratio tests ===

    #[test]
    fn test_parse_ratio_colon_format() {
        let items = vec![Sexpr::symbol("3:2")];
        let (actual, normal, start) = parse_ratio(&items).unwrap();
        assert_eq!(actual, 3);
        assert_eq!(normal, 2);
        assert_eq!(start, 1);
    }

    #[test]
    fn test_parse_ratio_slash_format() {
        let items = vec![Sexpr::symbol("5/4")];
        let (actual, normal, start) = parse_ratio(&items).unwrap();
        assert_eq!(actual, 5);
        assert_eq!(normal, 4);
        assert_eq!(start, 1);
    }

    #[test]
    fn test_parse_ratio_separate_symbols() {
        let items = vec![Sexpr::symbol("3"), Sexpr::symbol("2")];
        let (actual, normal, start) = parse_ratio(&items).unwrap();
        assert_eq!(actual, 3);
        assert_eq!(normal, 2);
        assert_eq!(start, 2);
    }

    #[test]
    fn test_parse_ratio_integers() {
        let items = vec![Sexpr::Integer(3), Sexpr::Integer(2)];
        let (actual, normal, start) = parse_ratio(&items).unwrap();
        assert_eq!(actual, 3);
        assert_eq!(normal, 2);
        assert_eq!(start, 2);
    }

    #[test]
    fn test_parse_ratio_single_triplet() {
        let items = vec![Sexpr::symbol("3")];
        let (actual, normal, start) = parse_ratio(&items).unwrap();
        assert_eq!(actual, 3);
        assert_eq!(normal, 2); // Default for triplet
        assert_eq!(start, 1);
    }

    #[test]
    fn test_parse_ratio_single_quintuplet() {
        let items = vec![Sexpr::symbol("5")];
        let (actual, normal, start) = parse_ratio(&items).unwrap();
        assert_eq!(actual, 5);
        assert_eq!(normal, 4); // Default: n-1
        assert_eq!(start, 1);
    }

    #[test]
    fn test_parse_ratio_empty() {
        let items: Vec<Sexpr> = vec![];
        assert!(parse_ratio(&items).is_err());
    }

    #[test]
    fn test_parse_ratio_invalid() {
        let items = vec![Sexpr::symbol("invalid")];
        assert!(parse_ratio(&items).is_err());
    }

    // === parse_tuplet_form tests ===

    #[test]
    fn test_parse_tuplet_form_simple() {
        let items = vec![
            Sexpr::symbol("3:2"),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("c4"),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("d4"),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("e4"),
                Sexpr::keyword("8"),
            ]),
        ];
        let tuplet = parse_tuplet_form(&items).unwrap();
        assert_eq!(tuplet.actual, 3);
        assert_eq!(tuplet.normal, 2);
        assert_eq!(tuplet.notes.len(), 3);
    }

    #[test]
    fn test_parse_tuplet_form_with_rest() {
        let items = vec![
            Sexpr::symbol("3:2"),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("c4"),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![Sexpr::symbol("rest"), Sexpr::keyword("8")]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("e4"),
                Sexpr::keyword("8"),
            ]),
        ];
        let tuplet = parse_tuplet_form(&items).unwrap();
        assert_eq!(tuplet.notes.len(), 3);
    }

    #[test]
    fn test_parse_tuplet_form_empty() {
        let items: Vec<Sexpr> = vec![];
        assert!(parse_tuplet_form(&items).is_err());
    }

    #[test]
    fn test_parse_tuplet_form_no_notes() {
        let items = vec![Sexpr::symbol("3:2")];
        assert!(parse_tuplet_form(&items).is_err());
    }

    // === compile_tuplet tests ===

    #[test]
    fn test_compile_tuplet_triplet() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("tuplet"),
            Sexpr::symbol("3:2"),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("c4"),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("d4"),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("e4"),
                Sexpr::keyword("8"),
            ]),
        ]);
        let notes = compile_tuplet(&sexpr).unwrap();

        assert_eq!(notes.len(), 3);

        // All notes should have time modification
        for note in &notes {
            assert!(note.time_modification.is_some());
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 3);
            assert_eq!(tm.normal_notes, 2);
        }

        // First note should have start tuplet notation
        let has_tuplet_start = notes[0].notations.iter().any(|n| {
            n.content.iter().any(|c| {
                if let NotationContent::Tuplet(t) = c {
                    t.r#type == StartStop::Start
                } else {
                    false
                }
            })
        });
        assert!(has_tuplet_start);

        // Last note should have stop tuplet notation
        let has_tuplet_stop = notes[2].notations.iter().any(|n| {
            n.content.iter().any(|c| {
                if let NotationContent::Tuplet(t) = c {
                    t.r#type == StartStop::Stop
                } else {
                    false
                }
            })
        });
        assert!(has_tuplet_stop);
    }

    #[test]
    fn test_compile_tuplet_quintuplet() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("tuplet"),
            Sexpr::symbol("5:4"),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("c4"),
                Sexpr::keyword("16"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("d4"),
                Sexpr::keyword("16"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("e4"),
                Sexpr::keyword("16"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("f4"),
                Sexpr::keyword("16"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("g4"),
                Sexpr::keyword("16"),
            ]),
        ]);
        let notes = compile_tuplet(&sexpr).unwrap();

        assert_eq!(notes.len(), 5);

        for note in &notes {
            assert!(note.time_modification.is_some());
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 5);
            assert_eq!(tm.normal_notes, 4);
        }
    }

    #[test]
    fn test_compile_tuplet_with_chord() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("tuplet"),
            Sexpr::symbol("3:2"),
            Sexpr::list(vec![
                Sexpr::symbol("chord"),
                Sexpr::list(vec![Sexpr::symbol("c4"), Sexpr::symbol("e4")]),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("d4"),
                Sexpr::keyword("8"),
            ]),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol("e4"),
                Sexpr::keyword("8"),
            ]),
        ]);
        let notes = compile_tuplet(&sexpr).unwrap();

        // Chord expands to 2 notes, so total is 4
        assert_eq!(notes.len(), 4);

        // All notes should have time modification
        for note in &notes {
            assert!(note.time_modification.is_some());
        }
    }

    #[test]
    fn test_compile_tuplet_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(compile_tuplet(&sexpr).is_err());
    }

    #[test]
    fn test_compile_tuplet_wrong_head() {
        let sexpr = Sexpr::list(vec![Sexpr::symbol("note"), Sexpr::symbol("c4")]);
        assert!(compile_tuplet(&sexpr).is_err());
    }

    #[test]
    fn test_compile_tuplet_not_list() {
        let sexpr = Sexpr::symbol("tuplet");
        assert!(compile_tuplet(&sexpr).is_err());
    }

    // === compile_fermata_tuplet tests ===

    #[test]
    fn test_compile_fermata_tuplet_basic() {
        let tuplet = FermataTuplet {
            actual: 3,
            normal: 2,
            notes: vec![
                MeasureElement::Note(FermataNote {
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
                }),
                MeasureElement::Note(FermataNote {
                    pitch: FermataPitch {
                        step: PitchStep::D,
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
                }),
            ],
        };

        let notes = compile_fermata_tuplet(&tuplet).unwrap();
        assert_eq!(notes.len(), 2);

        // Both should have time modification
        for note in &notes {
            assert!(note.time_modification.is_some());
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 3);
            assert_eq!(tm.normal_notes, 2);
        }
    }

    #[test]
    fn test_compile_fermata_tuplet_with_rest() {
        let tuplet = FermataTuplet {
            actual: 3,
            normal: 2,
            notes: vec![
                MeasureElement::Note(FermataNote {
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
                }),
                MeasureElement::Rest(FermataRest {
                    duration: FermataDuration::default(),
                    voice: None,
                    staff: None,
                    measure_rest: false,
                }),
                MeasureElement::Note(FermataNote {
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
                    ornaments: vec![],
                    tie: None,
                    slur: None,
                    lyric: None,
                }),
            ],
        };

        let notes = compile_fermata_tuplet(&tuplet).unwrap();
        assert_eq!(notes.len(), 3);

        // Middle note (rest) should also have time modification
        assert!(notes[1].time_modification.is_some());
    }

    // === create_tuplet_notation tests ===

    #[test]
    fn test_create_tuplet_notation_start() {
        let notation = create_tuplet_notation(StartStop::Start, 3, 2);
        assert_eq!(notation.r#type, StartStop::Start);
        assert_eq!(notation.bracket, Some(YesNo::Yes));

        let actual = notation.tuplet_actual.as_ref().unwrap();
        assert_eq!(actual.tuplet_number.as_ref().unwrap().value, 3);

        let normal = notation.tuplet_normal.as_ref().unwrap();
        assert_eq!(normal.tuplet_number.as_ref().unwrap().value, 2);
    }

    #[test]
    fn test_create_tuplet_notation_stop() {
        let notation = create_tuplet_notation(StartStop::Stop, 5, 4);
        assert_eq!(notation.r#type, StartStop::Stop);

        let actual = notation.tuplet_actual.as_ref().unwrap();
        assert_eq!(actual.tuplet_number.as_ref().unwrap().value, 5);
    }
}
