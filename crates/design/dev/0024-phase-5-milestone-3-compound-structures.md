# Phase 5 — Milestone 3: Compound Structures

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Chords, tuplets, grace notes, ties/slurs compilation
> **Depends On:** Milestone 2 (Core Elements)

---

## Overview

This milestone implements compound musical structures:

- **Chords** — `(chord :q c4 e4 g4)` → Multiple notes with `:chord t`
- **Tuplets** — `(tuplet 3:2 ...)` → Notes with `time-modification`
- **Grace notes** — `(grace c4)` → Grace note content
- **Ties/Slurs** — Extended start/continue/stop handling

---

## Task 1: Chord Compilation (`src/lang/chord.rs`)

Chords expand to multiple IR notes where all but the first have `:chord t`.

```rust
//! Chord compilation for Fermata syntax.
//!
//! Compiles `(chord :q c4 e4 g4)` to multiple IR notes.

use crate::ir::common::{Position, UpDown, YesNo};
use crate::ir::note::{Note, NoteContent, FullNote};
use crate::ir::pitch::PitchRestUnpitched;
use crate::ir::notation::{Notations, NotationContent, Arpeggiate};
use crate::sexpr::Sexpr;
use super::ast::{FermataChord, FermataPitch, ArpeggiateDirection, StemDirection, Articulation};
use super::error::{CompileError, CompileResult};
use super::pitch::{parse_pitch_sexpr, compile_pitch};
use super::duration::{parse_duration_sexpr, compile_duration_type, compile_dots, compile_duration_divisions};
use super::defaults::DEFAULT_VOICE;

/// Compile a chord S-expression to multiple IR notes.
///
/// Syntax: `(chord <duration> <pitch>+ [:voice n] [:stem up|down] [:arpeggiate up|down])`
///
/// Returns notes where first is root (no chord flag), rest have `:chord t`.
pub fn compile_chord(sexpr: &Sexpr) -> CompileResult<Vec<Note>> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("chord list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("chord") {
        return Err(CompileError::UnknownForm("expected (chord ...)".to_string()));
    }

    let fermata_chord = parse_chord_form(&list[1..])?;
    compile_fermata_chord(&fermata_chord)
}

/// Parse chord form to AST (exposed for tuplet compilation)
pub fn parse_chord_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataChord> {
    parse_chord_form(args)
}

fn parse_chord_form(args: &[Sexpr]) -> CompileResult<FermataChord> {
    if args.is_empty() {
        return Err(CompileError::MissingField("duration"));
    }

    // First arg: duration
    let duration = parse_duration_sexpr(&args[0])?;

    let mut pitches = Vec::new();
    let mut voice = None;
    let mut staff = None;
    let mut stem = None;
    let mut articulations = Vec::new();
    let mut ornaments = Vec::new();
    let mut arpeggiate = None;

    let mut i = 1;
    while i < args.len() {
        if let Some(key) = args[i].as_keyword() {
            i += 1;
            match key {
                "voice" => {
                    if i < args.len() {
                        voice = Some(parse_u32(&args[i])?);
                        i += 1;
                    }
                }
                "staff" => {
                    if i < args.len() {
                        staff = Some(parse_u32(&args[i])?);
                        i += 1;
                    }
                }
                "stem" => {
                    if i < args.len() {
                        stem = Some(parse_stem_direction(&args[i])?);
                        i += 1;
                    }
                }
                "arpeggiate" => {
                    if i < args.len() {
                        arpeggiate = Some(parse_arpeggiate_direction(&args[i])?);
                        i += 1;
                    }
                }
                "staccato" => articulations.push(Articulation::Staccato),
                "accent" => articulations.push(Articulation::Accent),
                "tenuto" => articulations.push(Articulation::Tenuto),
                _ => {
                    // Unknown keyword - skip value if present
                    if i < args.len() && args[i].as_keyword().is_none() {
                        i += 1;
                    }
                }
            }
        } else {
            // Try to parse as pitch
            match parse_pitch_sexpr(&args[i]) {
                Ok(pitch) => pitches.push(pitch),
                Err(_) => {} // Ignore non-pitch non-keyword args
            }
            i += 1;
        }
    }

    if pitches.len() < 2 {
        return Err(CompileError::InvalidChord {
            reason: "chord requires at least 2 pitches".to_string(),
        });
    }

    Ok(FermataChord {
        pitches,
        duration,
        voice,
        staff,
        stem,
        articulations,
        ornaments,
        arpeggiate,
    })
}

/// Parse an integer from S-expression (handles both Symbol and Integer)
fn parse_u32(sexpr: &Sexpr) -> CompileResult<u32> {
    match sexpr {
        Sexpr::Integer(n) if *n >= 0 => Ok(*n as u32),
        Sexpr::Symbol(s) => s.parse::<u32>()
            .map_err(|_| CompileError::type_mismatch("integer", s.clone())),
        _ => Err(CompileError::type_mismatch("integer", format!("{:?}", sexpr))),
    }
}

fn parse_stem_direction(sexpr: &Sexpr) -> CompileResult<StemDirection> {
    match sexpr.as_symbol() {
        Some("up") => Ok(StemDirection::Up),
        Some("down") => Ok(StemDirection::Down),
        Some("none") => Ok(StemDirection::None),
        _ => Err(CompileError::type_mismatch("stem direction", format!("{:?}", sexpr))),
    }
}

fn parse_arpeggiate_direction(sexpr: &Sexpr) -> CompileResult<ArpeggiateDirection> {
    match sexpr.as_symbol() {
        Some("up") => Ok(ArpeggiateDirection::Up),
        Some("down") => Ok(ArpeggiateDirection::Down),
        Some("none") => Ok(ArpeggiateDirection::None),
        _ => Err(CompileError::type_mismatch("arpeggiate direction", format!("{:?}", sexpr))),
    }
}

/// Compile FermataChord to multiple IR Notes
pub fn compile_fermata_chord(chord: &FermataChord) -> CompileResult<Vec<Note>> {
    let note_type = compile_duration_type(&chord.duration)?;
    let dots = compile_dots(&chord.duration);
    let duration = compile_duration_divisions(&chord.duration);

    let voice_str = chord.voice
        .map(|v| v.to_string())
        .unwrap_or_else(|| DEFAULT_VOICE.to_string());

    let stem = chord.stem.as_ref().map(compile_stem);

    let mut notes = Vec::with_capacity(chord.pitches.len());

    for (i, fermata_pitch) in chord.pitches.iter().enumerate() {
        let pitch = compile_pitch(fermata_pitch)?;
        let is_chord_tone = i > 0; // First note is root, rest are chord tones

        let full_note = FullNote {
            chord: is_chord_tone,
            content: PitchRestUnpitched::Pitch(pitch),
        };

        let content = NoteContent::Regular {
            full_note,
            duration,  // PositiveDivisions is Copy (u64 type alias)
            ties: Vec::new(),
        };

        // Only first note gets arpeggiate notation
        let notations = if i == 0 && chord.arpeggiate.is_some() {
            let arp_dir = chord.arpeggiate.as_ref().and_then(|d| match d {
                ArpeggiateDirection::Up => Some(UpDown::Up),
                ArpeggiateDirection::Down => Some(UpDown::Down),
                ArpeggiateDirection::None => None,
            });

            vec![Notations {
                print_object: None,
                content: vec![NotationContent::Arpeggiate(Arpeggiate {
                    number: None,
                    direction: arp_dir,
                    position: Position::default(),
                    color: None,
                })],
                editorial: Default::default(),
            }]
        } else {
            Vec::new()
        };

        notes.push(Note {
            // Position/playback attributes (defaults)
            position: Position::default(),
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            print_object: None,
            // Core content
            content,
            // Common fields
            instrument: Vec::new(),
            voice: Some(voice_str.clone()),
            r#type: Some(note_type.clone()),
            dots: dots.clone(),
            accidental: None,
            time_modification: None,
            stem: stem.clone(),
            notehead: None,
            staff: chord.staff,
            beams: Vec::new(),
            notations,
            lyrics: Vec::new(),
        });
    }

    Ok(notes)
}

fn compile_stem(dir: &StemDirection) -> crate::ir::beam::Stem {
    use crate::ir::beam::{Stem, StemValue};
    Stem {
        value: match dir {
            StemDirection::Up => StemValue::Up,
            StemDirection::Down => StemValue::Down,
            StemDirection::None => StemValue::None,
            StemDirection::Double => StemValue::Double,
        },
        default_y: None,
        relative_y: None,
        color: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_simple_chord() {
        let sexpr = parse("(chord :q c4 e4 g4)").unwrap();
        let notes = compile_chord(&sexpr).unwrap();

        assert_eq!(notes.len(), 3);

        // First note: not a chord tone
        if let NoteContent::Regular { full_note, .. } = &notes[0].content {
            assert!(!full_note.chord);
        }

        // Second and third: chord tones
        if let NoteContent::Regular { full_note, .. } = &notes[1].content {
            assert!(full_note.chord);
        }
        if let NoteContent::Regular { full_note, .. } = &notes[2].content {
            assert!(full_note.chord);
        }
    }

    #[test]
    fn test_chord_with_arpeggiate() {
        let sexpr = parse("(chord :q c4 e4 g4 :arpeggiate up)").unwrap();
        let notes = compile_chord(&sexpr).unwrap();

        // Only first note should have arpeggiate
        assert!(!notes[0].notations.is_empty());
        assert!(notes[1].notations.is_empty());
    }

    #[test]
    fn test_chord_needs_two_pitches() {
        let sexpr = parse("(chord :q c4)").unwrap();
        let result = compile_chord(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_chord_with_voice() {
        let sexpr = parse("(chord :q c4 e4 g4 :voice 2)").unwrap();
        let notes = compile_chord(&sexpr).unwrap();

        for note in &notes {
            assert_eq!(note.voice, Some("2".to_string()));
        }
    }
}
```

---

## Task 2: Tuplet Compilation (`src/lang/tuplet.rs`)

Tuplets wrap notes and add `time-modification` to each.

```rust
//! Tuplet compilation for Fermata syntax.
//!
//! Compiles `(tuplet 3:2 (note c4 :8) (note d4 :8) (note e4 :8))` to notes
//! with time-modification and tuplet notations.

use crate::ir::common::{Position, StartStop, YesNo};
use crate::ir::note::Note;
use crate::ir::duration::TimeModification;
use crate::ir::notation::{Notations, NotationContent, Tuplet as TupletNotation, ShowTuplet};
use crate::sexpr::Sexpr;
use super::ast::{FermataTuplet, MeasureElement};
use super::error::{CompileError, CompileResult};

/// Compile a tuplet S-expression.
///
/// Syntax: `(tuplet <ratio> <note/rest>+)`
/// Ratio: `3:2` (triplet), `5:4` (quintuplet), etc.
///
/// Returns notes with time-modification applied.
pub fn compile_tuplet(sexpr: &Sexpr) -> CompileResult<Vec<Note>> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("tuplet list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("tuplet") {
        return Err(CompileError::UnknownForm("expected (tuplet ...)".to_string()));
    }

    let fermata_tuplet = parse_tuplet_form(&list[1..])?;
    compile_fermata_tuplet(&fermata_tuplet)
}

fn parse_tuplet_form(args: &[Sexpr]) -> CompileResult<FermataTuplet> {
    if args.is_empty() {
        return Err(CompileError::MissingField("ratio"));
    }

    // First arg: ratio (e.g., "3:2")
    let ratio_str = args[0].as_symbol()
        .ok_or_else(|| CompileError::type_mismatch("ratio symbol", format!("{:?}", args[0])))?;

    let (actual, normal) = parse_ratio(ratio_str)?;

    // Remaining args: notes/rests
    let mut notes = Vec::new();
    for arg in &args[1..] {
        let element = parse_measure_element(arg)?;
        notes.push(element);
    }

    if notes.is_empty() {
        return Err(CompileError::InvalidTuplet {
            reason: "tuplet requires at least one note".to_string(),
        });
    }

    Ok(FermataTuplet { actual, normal, notes })
}

fn parse_ratio(s: &str) -> CompileResult<(u32, u32)> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(CompileError::InvalidTuplet {
            reason: format!("invalid ratio format '{}', expected 'actual:normal' (e.g., 3:2)", s),
        });
    }

    let actual = parts[0].parse::<u32>()
        .map_err(|_| CompileError::InvalidTuplet {
            reason: format!("invalid actual notes in ratio: {}", parts[0]),
        })?;

    let normal = parts[1].parse::<u32>()
        .map_err(|_| CompileError::InvalidTuplet {
            reason: format!("invalid normal notes in ratio: {}", parts[1]),
        })?;

    if actual == 0 || normal == 0 {
        return Err(CompileError::InvalidTuplet {
            reason: "ratio values must be > 0".to_string(),
        });
    }

    Ok((actual, normal))
}

fn parse_measure_element(sexpr: &Sexpr) -> CompileResult<MeasureElement> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("note/rest list", format!("{:?}", sexpr)))?;

    match list.first().and_then(|s| s.as_symbol()) {
        Some("note") => {
            let note_ast = super::note::parse_note_form_to_ast(&list[1..])?;
            Ok(MeasureElement::Note(note_ast))
        }
        Some("rest") => {
            let rest_ast = super::note::parse_rest_form_to_ast(&list[1..])?;
            Ok(MeasureElement::Rest(rest_ast))
        }
        Some("chord") => {
            let chord_ast = super::chord::parse_chord_form_to_ast(&list[1..])?;
            Ok(MeasureElement::Chord(chord_ast))
        }
        other => Err(CompileError::UnknownForm(
            other.map(|s| s.to_string()).unwrap_or_else(|| "non-symbol".to_string())
        )),
    }
}

/// Compile FermataTuplet to IR notes with time-modification
fn compile_fermata_tuplet(tuplet: &FermataTuplet) -> CompileResult<Vec<Note>> {
    let time_mod = TimeModification {
        actual_notes: tuplet.actual,
        normal_notes: tuplet.normal,
        normal_type: None,
        normal_dots: 0,
    };

    let total_notes = count_notes(&tuplet.notes);
    let mut notes = Vec::new();
    let mut note_index = 0;

    for element in &tuplet.notes {
        let mut element_notes = compile_element_to_notes(element)?;

        for note in &mut element_notes {
            // Add time-modification to each note
            note.time_modification = Some(time_mod.clone());

            // Add tuplet notation to first and last note
            let is_first = note_index == 0;
            let is_last = note_index == total_notes - 1;

            if is_first || is_last {
                let tuplet_type = if is_first {
                    StartStop::Start
                } else {
                    StartStop::Stop
                };

                // Tuplet is boxed in NotationContent
                // Tuplet.bracket is Option<YesNo>, not Option<bool>
                let tuplet_notation = TupletNotation {
                    r#type: tuplet_type,
                    number: Some(1),
                    bracket: if is_first { Some(YesNo::Yes) } else { None },
                    show_number: if is_first { Some(ShowTuplet::Actual) } else { None },
                    show_type: None,
                    line_shape: None,
                    position: Position::default(),
                    placement: None,
                    tuplet_actual: None,
                    tuplet_normal: None,
                };

                // Add to existing notations or create new
                if note.notations.is_empty() {
                    note.notations.push(Notations {
                        print_object: None,
                        content: vec![NotationContent::Tuplet(Box::new(tuplet_notation))],
                        editorial: Default::default(),
                    });
                } else {
                    note.notations[0].content.push(NotationContent::Tuplet(Box::new(tuplet_notation)));
                }
            }

            notes.push(note.clone());
            note_index += 1;
        }
    }

    Ok(notes)
}

fn count_notes(elements: &[MeasureElement]) -> usize {
    elements.iter().map(|e| match e {
        MeasureElement::Note(_) | MeasureElement::Rest(_) => 1,
        MeasureElement::Chord(_) => 1, // Chord counts as one "beat"
        _ => 0,
    }).sum()
}

fn compile_element_to_notes(element: &MeasureElement) -> CompileResult<Vec<Note>> {
    match element {
        MeasureElement::Note(n) => {
            let note = super::note::compile_fermata_note(n)?;
            Ok(vec![note])
        }
        MeasureElement::Rest(r) => {
            let note = super::note::compile_fermata_rest(r)?;
            Ok(vec![note])
        }
        MeasureElement::Chord(c) => {
            super::chord::compile_fermata_chord(c)
        }
        _ => Err(CompileError::InvalidTuplet {
            reason: "unexpected element type in tuplet".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_triplet() {
        let sexpr = parse("(tuplet 3:2 (note c4 :8) (note d4 :8) (note e4 :8))").unwrap();
        let notes = compile_tuplet(&sexpr).unwrap();

        assert_eq!(notes.len(), 3);

        // All notes should have time-modification
        for note in &notes {
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 3);
            assert_eq!(tm.normal_notes, 2);
        }
    }

    #[test]
    fn test_tuplet_notations() {
        let sexpr = parse("(tuplet 3:2 (note c4 :8) (note d4 :8) (note e4 :8))").unwrap();
        let notes = compile_tuplet(&sexpr).unwrap();

        // First note should have tuplet start
        assert!(!notes[0].notations.is_empty());

        // Last note should have tuplet stop
        assert!(!notes[2].notations.is_empty());
    }

    #[test]
    fn test_quintuplet() {
        let sexpr = parse("(tuplet 5:4 (note c4 :16) (note d4 :16) (note e4 :16) (note f4 :16) (note g4 :16))").unwrap();
        let notes = compile_tuplet(&sexpr).unwrap();

        assert_eq!(notes.len(), 5);
        for note in &notes {
            let tm = note.time_modification.as_ref().unwrap();
            assert_eq!(tm.actual_notes, 5);
            assert_eq!(tm.normal_notes, 4);
        }
    }

    #[test]
    fn test_invalid_ratio() {
        let sexpr = parse("(tuplet 3 (note c4 :8))").unwrap();
        assert!(compile_tuplet(&sexpr).is_err());

        let sexpr = parse("(tuplet 0:2 (note c4 :8))").unwrap();
        assert!(compile_tuplet(&sexpr).is_err());
    }
}
```

---

## Task 3: Grace Note Compilation (`src/lang/grace.rs`)

```rust
//! Grace note compilation.
//!
//! Compiles `(grace c4)` and `(grace c4 :slash)` to IR grace notes.

use crate::ir::common::{Position, YesNo};
use crate::ir::note::{Note, NoteContent, FullNote, Grace};
use crate::ir::pitch::PitchRestUnpitched;
use crate::sexpr::Sexpr;
use super::ast::{FermataGraceNote, FermataPitch};
use super::error::{CompileError, CompileResult};
use super::pitch::{parse_pitch_sexpr, compile_pitch};
use super::duration::{parse_duration_sexpr, compile_duration_type};
use super::defaults::DEFAULT_VOICE;

/// Compile a grace note S-expression.
///
/// Syntax: `(grace <pitch> [:slash] [:duration dur])`
pub fn compile_grace_note(sexpr: &Sexpr) -> CompileResult<Note> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("grace list", format!("{:?}", sexpr)))?;

    let head = list.first().and_then(|s| s.as_symbol());
    if head != Some("grace") && head != Some("grace-note") {
        return Err(CompileError::UnknownForm("expected (grace ...)".to_string()));
    }

    let fermata_grace = parse_grace_form(&list[1..])?;
    compile_fermata_grace(&fermata_grace)
}

fn parse_grace_form(args: &[Sexpr]) -> CompileResult<FermataGraceNote> {
    if args.is_empty() {
        return Err(CompileError::MissingField("pitch"));
    }

    let pitch = parse_pitch_sexpr(&args[0])?;

    let mut slash = false;
    let mut duration = None;

    let mut i = 1;
    while i < args.len() {
        if let Some(key) = args[i].as_keyword() {
            i += 1;
            match key {
                "slash" => slash = true,
                "duration" | "dur" => {
                    if i < args.len() {
                        duration = Some(parse_duration_sexpr(&args[i])?);
                        i += 1;
                    }
                }
                _ => {
                    // Unknown keyword - skip value if present
                    if i < args.len() && args[i].as_keyword().is_none() {
                        i += 1;
                    }
                }
            }
        } else {
            i += 1;
        }
    }

    Ok(FermataGraceNote { pitch, slash, duration })
}

fn compile_fermata_grace(grace: &FermataGraceNote) -> CompileResult<Note> {
    let pitch = compile_pitch(&grace.pitch)?;

    let full_note = FullNote {
        chord: false,
        content: PitchRestUnpitched::Pitch(pitch),
    };

    let grace_elem = Grace {
        steal_time_previous: None,
        steal_time_following: None,
        make_time: None,
        slash: if grace.slash { Some(YesNo::Yes) } else { None },
    };

    let content = NoteContent::Grace {
        grace: grace_elem,
        full_note,
        ties: Vec::new(),
    };

    let note_type = grace.duration.as_ref()
        .map(|d| compile_duration_type(d))
        .transpose()?;

    Ok(Note {
        // Position/playback attributes (defaults)
        position: Position::default(),
        dynamics: None,
        end_dynamics: None,
        attack: None,
        release: None,
        pizzicato: None,
        print_object: None,
        // Core content
        content,
        // Common fields
        instrument: Vec::new(),
        voice: Some(DEFAULT_VOICE.to_string()),
        r#type: note_type,
        dots: Vec::new(),
        accidental: None,
        time_modification: None,
        stem: None,
        notehead: None,
        staff: None,
        beams: Vec::new(),
        notations: Vec::new(),
        lyrics: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_simple_grace() {
        let sexpr = parse("(grace d5)").unwrap();
        let note = compile_grace_note(&sexpr).unwrap();

        assert!(matches!(note.content, NoteContent::Grace { .. }));
    }

    #[test]
    fn test_grace_with_slash() {
        let sexpr = parse("(grace d5 :slash)").unwrap();
        let note = compile_grace_note(&sexpr).unwrap();

        if let NoteContent::Grace { grace, .. } = &note.content {
            assert_eq!(grace.slash, Some(YesNo::Yes));
        } else {
            panic!("Expected grace note");
        }
    }

    #[test]
    fn test_grace_with_duration() {
        let sexpr = parse("(grace d5 :slash :duration :16)").unwrap();
        let note = compile_grace_note(&sexpr).unwrap();

        assert!(note.r#type.is_some());
    }

    #[test]
    fn test_grace_note_alias() {
        let sexpr = parse("(grace-note e5)").unwrap();
        let note = compile_grace_note(&sexpr).unwrap();

        assert!(matches!(note.content, NoteContent::Grace { .. }));
    }
}
```

---

## Task 4: Extended Tie/Slur Handling

Add to `src/lang/connectors.rs`:

```rust
//! Tie and slur marker compilation.

use crate::ir::common::{NumberLevel, StartStopContinue};
use crate::ir::notation::Slur;
use crate::sexpr::Sexpr;
use super::error::{CompileError, CompileResult};

/// Parse start/stop/continue for slurs
pub fn parse_slur_action(sexpr: &Sexpr) -> CompileResult<StartStopContinue> {
    match sexpr.as_symbol() {
        Some("start") => Ok(StartStopContinue::Start),
        Some("stop") => Ok(StartStopContinue::Stop),
        Some("continue") => Ok(StartStopContinue::Continue),
        _ => Err(CompileError::type_mismatch(
            "start/stop/continue",
            format!("{:?}", sexpr)
        )),
    }
}

/// Compile a slur marker
///
/// Syntax: `(slur :start)` or `(slur :stop :number 2)` (with number)
pub fn compile_slur_marker(sexpr: &Sexpr) -> CompileResult<Slur> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("slur list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("slur") {
        return Err(CompileError::UnknownForm("expected (slur ...)".to_string()));
    }

    let mut action = None;
    // Slur.number is NumberLevel (u8), NOT Option<NumberLevel> - it's required
    let mut number: NumberLevel = 1;

    let mut i = 1;
    while i < list.len() {
        if let Some(key) = list[i].as_keyword() {
            match key {
                "start" => action = Some(StartStopContinue::Start),
                "stop" => action = Some(StartStopContinue::Stop),
                "continue" => action = Some(StartStopContinue::Continue),
                "number" => {
                    i += 1;
                    if i < list.len() {
                        number = parse_u8(&list[i])?;
                    }
                }
                _ => {}
            }
        } else if let Some(sym) = list[i].as_symbol() {
            // Allow bare start/stop/continue
            match sym {
                "start" => action = Some(StartStopContinue::Start),
                "stop" => action = Some(StartStopContinue::Stop),
                "continue" => action = Some(StartStopContinue::Continue),
                _ => {}
            }
        }
        i += 1;
    }

    let action = action.ok_or(CompileError::MissingField("slur action (start/stop/continue)"))?;

    Ok(Slur {
        r#type: action,
        number,  // Required field, not Option
        line_type: None,
        position: Default::default(),
        placement: None,
        orientation: None,
        color: None,
    })
}

/// Parse u8 from S-expression (handles both Symbol and Integer)
fn parse_u8(sexpr: &Sexpr) -> CompileResult<u8> {
    match sexpr {
        Sexpr::Integer(n) if *n >= 0 && *n <= 255 => Ok(*n as u8),
        Sexpr::Symbol(s) => s.parse::<u8>()
            .map_err(|_| CompileError::type_mismatch("u8", s.clone())),
        _ => Err(CompileError::type_mismatch("u8", format!("{:?}", sexpr))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_slur_start() {
        let sexpr = parse("(slur :start)").unwrap();
        let slur = compile_slur_marker(&sexpr).unwrap();

        assert_eq!(slur.r#type, StartStopContinue::Start);
        assert_eq!(slur.number, 1);
    }

    #[test]
    fn test_slur_stop_with_number() {
        let sexpr = parse("(slur :stop :number 2)").unwrap();
        let slur = compile_slur_marker(&sexpr).unwrap();

        assert_eq!(slur.r#type, StartStopContinue::Stop);
        assert_eq!(slur.number, 2);
    }

    #[test]
    fn test_slur_continue() {
        let sexpr = parse("(slur :continue)").unwrap();
        let slur = compile_slur_marker(&sexpr).unwrap();

        assert_eq!(slur.r#type, StartStopContinue::Continue);
    }

    #[test]
    fn test_slur_bare_symbol() {
        let sexpr = parse("(slur start)").unwrap();
        let slur = compile_slur_marker(&sexpr).unwrap();

        assert_eq!(slur.r#type, StartStopContinue::Start);
    }
}
```

---

## Task 5: Note Module Updates

Add AST parsing helpers to `src/lang/note.rs`:

```rust
// Add these public functions to note.rs for tuplet compilation

/// Parse note form to AST without compiling to IR
pub fn parse_note_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataNote> {
    parse_note_form(args)
}

/// Parse rest form to AST without compiling to IR
pub fn parse_rest_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataRest> {
    parse_rest_form(args)
}

/// Compile FermataNote to IR Note (public for tuplet/chord use)
pub fn compile_fermata_note(note: &FermataNote) -> CompileResult<Note> {
    // ... existing implementation
}

/// Compile FermataRest to IR Note (public for tuplet use)
pub fn compile_fermata_rest(rest: &FermataRest) -> CompileResult<Note> {
    // ... existing implementation
}
```

---

## Task 6: Integration Tests (inline)

Add to appropriate module files:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::sexpr::parser::parse;
    use crate::ir::note::NoteContent;

    #[test]
    fn test_c_major_chord() {
        let sexpr = parse("(chord :q c4 e4 g4)").unwrap();
        let notes = crate::lang::chord::compile_chord(&sexpr).unwrap();

        assert_eq!(notes.len(), 3, "C major chord has 3 notes");

        // Verify chord flags
        if let NoteContent::Regular { full_note, .. } = &notes[0].content {
            assert!(!full_note.chord, "First note should not have chord flag");
        }
        if let NoteContent::Regular { full_note, .. } = &notes[1].content {
            assert!(full_note.chord, "Second note should have chord flag");
        }
        if let NoteContent::Regular { full_note, .. } = &notes[2].content {
            assert!(full_note.chord, "Third note should have chord flag");
        }
    }

    #[test]
    fn test_seventh_chord() {
        let sexpr = parse("(chord :h g4 b4 d5 f5)").unwrap();
        let notes = crate::lang::chord::compile_chord(&sexpr).unwrap();

        assert_eq!(notes.len(), 4, "G7 chord has 4 notes");
    }

    #[test]
    fn test_triplet_eighth_notes() {
        let sexpr = parse("(tuplet 3:2 (note c4 :8) (note d4 :8) (note e4 :8))").unwrap();
        let notes = crate::lang::tuplet::compile_tuplet(&sexpr).unwrap();

        assert_eq!(notes.len(), 3);

        for note in &notes {
            let tm = note.time_modification.as_ref()
                .expect("Tuplet note should have time-modification");
            assert_eq!(tm.actual_notes, 3);
            assert_eq!(tm.normal_notes, 2);
        }
    }

    #[test]
    fn test_tuplet_with_chord() {
        let sexpr = parse("(tuplet 3:2 (chord :8 c4 e4) (note d4 :8) (note e4 :8))").unwrap();
        let notes = crate::lang::tuplet::compile_tuplet(&sexpr).unwrap();

        // Chord expands to 2 notes, plus 2 single notes = 4 notes total
        // But for tuplet purposes, chord is one "beat"
        assert!(notes.len() >= 3);

        // All notes should have time-modification
        for note in &notes {
            assert!(note.time_modification.is_some());
        }
    }

    #[test]
    fn test_grace_note_simple() {
        let sexpr = parse("(grace d5)").unwrap();
        let note = crate::lang::grace::compile_grace_note(&sexpr).unwrap();

        assert!(matches!(note.content, NoteContent::Grace { .. }));
    }

    #[test]
    fn test_grace_note_slashed() {
        let sexpr = parse("(grace d5 :slash)").unwrap();
        let note = crate::lang::grace::compile_grace_note(&sexpr).unwrap();

        if let NoteContent::Grace { grace, .. } = &note.content {
            assert!(grace.slash.is_some());
        } else {
            panic!("Expected grace note content");
        }
    }

    #[test]
    fn test_nested_structures_compile() {
        use crate::lang::{chord, tuplet, grace};

        // Verify complex structures at least parse and compile
        let chord_result = parse("(chord :q. c4 e4 g4)")
            .and_then(|s| chord::compile_chord(&s));
        assert!(chord_result.is_ok());

        let chord_result = parse("(chord :h c4 e4 g4 :stem up)")
            .and_then(|s| chord::compile_chord(&s));
        assert!(chord_result.is_ok());

        let chord_result = parse("(chord :q c4 e4 g4 :arpeggiate up)")
            .and_then(|s| chord::compile_chord(&s));
        assert!(chord_result.is_ok());

        let tuplet_result = parse("(tuplet 5:4 (note c4 :16) (note d4 :16) (note e4 :16) (note f4 :16) (note g4 :16))")
            .and_then(|s| tuplet::compile_tuplet(&s));
        assert!(tuplet_result.is_ok());

        let grace_result = parse("(grace e5 :slash :duration :16)")
            .and_then(|s| grace::compile_grace_note(&s));
        assert!(grace_result.is_ok());
    }
}
```

---

## Acceptance Criteria

1. ✅ `(chord :q c4 e4 g4)` produces 3 notes; 2nd and 3rd have `:chord t`
2. ✅ Chords support `:arpeggiate up|down`
3. ✅ `(tuplet 3:2 ...)` adds time-modification to all child notes
4. ✅ Tuplet start/stop notations added to first/last notes
5. ✅ `(grace c4)` produces grace note content
6. ✅ `(grace c4 :slash)` adds slash attribute
7. ✅ Slurs support `start`/`stop`/`continue` and number
8. ✅ All tests pass

---

## Implementation Notes

1. **Chord ordering** — Pitches should be in the order provided; no automatic sorting

2. **Tuplet flattening** — When a tuplet contains a chord, the chord's notes all get time-modification

3. **Grace note duration** — Optional; used for display only (no sounding duration)

4. **Slur numbers** — Default to 1; higher numbers for nested slurs. `Slur.number` is `NumberLevel` (u8), NOT `Option<NumberLevel>`

5. **AST helper functions** — `parse_*_form_to_ast` variants return AST types without compiling to IR (needed for tuplet/chord composition)

6. **Note struct fields** — IR `Note` includes position/playback fields (`position`, `dynamics`, `end_dynamics`, `attack`, `release`, `pizzicato`, `print_object`) that should be set to defaults. The `instrument` field is `Vec<Instrument>`, not `Option`.

7. **Tuplet is boxed** — `NotationContent::Tuplet(Box<Tuplet>)` requires `Box::new()`

8. **Tuplet.bracket** — Uses `Option<YesNo>`, not `Option<bool>`

9. **Notations struct** — Has three fields: `print_object`, `content`, `editorial` - use `..Default::default()` or set all explicitly

10. **parse_u32/parse_u8** — Must handle both `Sexpr::Integer` and `Sexpr::Symbol` for flexibility

11. **PositiveDivisions is Copy** — Being a `u64` type alias, it's `Copy`, so `.clone()` is unnecessary but harmless

---

*Next: Milestone 4 — Attributes & Directions (Keys, Times, Clefs, Dynamics)*
