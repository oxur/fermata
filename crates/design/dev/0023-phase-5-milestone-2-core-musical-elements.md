# Phase 5 — Milestone 2: Core Musical Elements

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Pitch parsing, duration parsing, note compilation, rest compilation
> **Depends On:** Milestone 1 (Foundation)

---

## Overview

This milestone implements the core musical element parsers and compilers:

- **Pitch parsing** — `c4`, `f#5`, `bb3`, `cn4`, microtones
- **Duration parsing** — `:q`, `:quarter`, `:crotchet`, dots
- **Note compilation** — `(note c4 :q)` → IR Note
- **Rest compilation** — `(rest :h)` → IR Rest Note

---

## Task 1: Pitch Parsing (`src/lang/pitch.rs`)

Parse scientific pitch notation into Fermata AST and compile to IR.

```rust
//! Pitch parsing for Fermata syntax.
//!
//! Supports scientific pitch notation:
//! - `c4` — middle C
//! - `f#5` — F sharp, octave 5
//! - `bb3` — B double-flat, octave 3
//! - `cn4` — C natural (explicit)
//! - `c+4` — C quarter-sharp (microtone)

use crate::ir::pitch::{Pitch, Step};
use crate::ir::common::{Octave, Semitones};
use crate::sexpr::Sexpr;
use super::ast::{FermataPitch, PitchStep, PitchAlter};
use super::error::{CompileError, CompileResult};

/// Parse a pitch string like "c4", "f#5", "bb3"
pub fn parse_pitch(s: &str) -> CompileResult<Pitch> {
    let fermata_pitch = parse_pitch_str(s)?;
    compile_pitch(&fermata_pitch)
}

/// Parse pitch string to Fermata AST
pub fn parse_pitch_str(s: &str) -> CompileResult<FermataPitch> {
    let s = s.trim().to_lowercase();
    if s.is_empty() {
        return Err(CompileError::InvalidPitch("empty pitch string".to_string()));
    }

    let mut chars = s.chars().peekable();

    // Parse step (required)
    let step_char = chars.next()
        .ok_or_else(|| CompileError::InvalidPitch(s.clone()))?;
    let step = parse_step(step_char)?;

    // Parse alteration (optional)
    let mut alter = None;
    while let Some(&ch) = chars.peek() {
        match ch {
            '#' => {
                chars.next();
                alter = Some(match alter {
                    None => PitchAlter::Sharp,
                    Some(PitchAlter::Sharp) => PitchAlter::DoubleSharp,
                    _ => return Err(CompileError::InvalidPitch(format!("invalid alteration in '{}'", s))),
                });
            }
            'b' if !chars.clone().skip(1).next().map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                chars.next();
                alter = Some(match alter {
                    None => PitchAlter::Flat,
                    Some(PitchAlter::Flat) => PitchAlter::DoubleFlat,
                    _ => return Err(CompileError::InvalidPitch(format!("invalid alteration in '{}'", s))),
                });
            }
            'x' => {
                chars.next();
                alter = Some(PitchAlter::DoubleSharp);
            }
            'n' => {
                chars.next();
                alter = Some(PitchAlter::Natural);
            }
            '+' => {
                chars.next();
                alter = Some(match alter {
                    None => PitchAlter::QuarterSharp,
                    Some(PitchAlter::Sharp) => PitchAlter::ThreeQuarterSharp,
                    _ => return Err(CompileError::InvalidPitch(format!("invalid microtone in '{}'", s))),
                });
            }
            'd' if !chars.clone().skip(1).next().map(|c| c.is_ascii_digit()).unwrap_or(false) => {
                chars.next();
                alter = Some(match alter {
                    None => PitchAlter::QuarterFlat,
                    Some(PitchAlter::Flat) => PitchAlter::ThreeQuarterFlat,
                    _ => return Err(CompileError::InvalidPitch(format!("invalid microtone in '{}'", s))),
                });
            }
            _ => break,
        }
    }

    // Parse octave (required)
    let octave_str: String = chars.collect();
    let octave = octave_str.parse::<u8>()
        .map_err(|_| CompileError::InvalidPitch(format!("invalid octave in '{}': '{}'", s, octave_str)))?;

    if octave > 9 {
        return Err(CompileError::InvalidPitch(format!("octave {} out of range (0-9)", octave)));
    }

    Ok(FermataPitch { step, alter, octave })
}

fn parse_step(ch: char) -> CompileResult<PitchStep> {
    match ch.to_ascii_uppercase() {
        'C' => Ok(PitchStep::C),
        'D' => Ok(PitchStep::D),
        'E' => Ok(PitchStep::E),
        'F' => Ok(PitchStep::F),
        'G' => Ok(PitchStep::G),
        'A' => Ok(PitchStep::A),
        'B' => Ok(PitchStep::B),
        _ => Err(CompileError::InvalidPitch(format!("invalid step '{}'", ch))),
    }
}

/// Compile Fermata pitch to IR Pitch
pub fn compile_pitch(pitch: &FermataPitch) -> CompileResult<Pitch> {
    let step = match pitch.step {
        PitchStep::C => Step::C,
        PitchStep::D => Step::D,
        PitchStep::E => Step::E,
        PitchStep::F => Step::F,
        PitchStep::G => Step::G,
        PitchStep::A => Step::A,
        PitchStep::B => Step::B,
    };

    // Semitones and Octave are type aliases (f64 and u8), not newtypes
    let alter: Option<Semitones> = pitch.alter.as_ref().map(|a| a.to_semitones());

    Ok(Pitch {
        step,
        alter,
        octave: pitch.octave,
    })
}

/// Parse a pitch from S-expression
pub fn parse_pitch_sexpr(sexpr: &Sexpr) -> CompileResult<FermataPitch> {
    match sexpr {
        Sexpr::Symbol(s) if !s.starts_with(':') => parse_pitch_str(s),
        _ => Err(CompileError::type_mismatch("pitch symbol", format!("{:?}", sexpr))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pitches() {
        let p = parse_pitch_str("c4").unwrap();
        assert_eq!(p.step, PitchStep::C);
        assert_eq!(p.octave, 4);
        assert_eq!(p.alter, None);

        let p = parse_pitch_str("a3").unwrap();
        assert_eq!(p.step, PitchStep::A);
        assert_eq!(p.octave, 3);
    }

    #[test]
    fn test_parse_sharps() {
        let p = parse_pitch_str("f#5").unwrap();
        assert_eq!(p.step, PitchStep::F);
        assert_eq!(p.octave, 5);
        assert_eq!(p.alter, Some(PitchAlter::Sharp));

        let p = parse_pitch_str("c##4").unwrap();
        assert_eq!(p.alter, Some(PitchAlter::DoubleSharp));

        let p = parse_pitch_str("cx4").unwrap();
        assert_eq!(p.alter, Some(PitchAlter::DoubleSharp));
    }

    #[test]
    fn test_parse_flats() {
        let p = parse_pitch_str("bb3").unwrap();
        assert_eq!(p.step, PitchStep::B);
        assert_eq!(p.octave, 3);
        assert_eq!(p.alter, Some(PitchAlter::Flat));

        let p = parse_pitch_str("ebb2").unwrap();
        assert_eq!(p.alter, Some(PitchAlter::DoubleFlat));
    }

    #[test]
    fn test_parse_natural() {
        let p = parse_pitch_str("cn4").unwrap();
        assert_eq!(p.step, PitchStep::C);
        assert_eq!(p.alter, Some(PitchAlter::Natural));
    }

    #[test]
    fn test_parse_microtones() {
        let p = parse_pitch_str("c+4").unwrap();
        assert_eq!(p.alter, Some(PitchAlter::QuarterSharp));

        let p = parse_pitch_str("cd4").unwrap();
        assert_eq!(p.alter, Some(PitchAlter::QuarterFlat));
    }

    #[test]
    fn test_compile_to_ir() {
        let p = parse_pitch("f#5").unwrap();
        assert_eq!(p.step, Step::F);
        // Semitones is a type alias (f64), not a newtype
        assert_eq!(p.alter, Some(1.0));
        // Octave is a type alias (u8), not a newtype
        assert_eq!(p.octave, 5);
    }

    #[test]
    fn test_invalid_pitches() {
        assert!(parse_pitch_str("").is_err());
        assert!(parse_pitch_str("x4").is_err());  // invalid step
        assert!(parse_pitch_str("c").is_err());   // missing octave
        assert!(parse_pitch_str("c10").is_err()); // octave out of range
    }
}
```

---

## Task 2: Duration Parsing (`src/lang/duration.rs`)

Parse duration keywords and compile to IR.

```rust
//! Duration parsing for Fermata syntax.
//!
//! Supports multiple naming conventions:
//! - Short: `:w`, `:h`, `:q`, `:8`, `:16`, `:32`, `:64`, `:128`, `:256`, `:512`, `:1024`
//! - Long: `:whole`, `:half`, `:quarter`, `:eighth`, `:sixteenth`
//! - British: `:semibreve`, `:minim`, `:crotchet`, `:quaver`, `:semiquaver`
//! - Dots: `:q.` (dotted), `:h..` (double-dotted)

use crate::ir::common::PositiveDivisions;
use crate::ir::duration::{NoteType, NoteTypeValue, Dot};
use crate::sexpr::Sexpr;
use super::ast::{FermataDuration, DurationBase};
use super::error::{CompileError, CompileResult};
use super::defaults::DEFAULT_DIVISIONS;

/// Parse a duration keyword like ":q", ":quarter", ":h."
pub fn parse_duration(s: &str) -> CompileResult<FermataDuration> {
    let s = s.trim();

    // Handle keyword prefix
    let s = if s.starts_with(':') { &s[1..] } else { s };

    if s.is_empty() {
        return Err(CompileError::InvalidDuration("empty duration".to_string()));
    }

    // Count trailing dots
    let dot_count = s.chars().rev().take_while(|&c| c == '.').count() as u8;
    let base_str = &s[..s.len() - dot_count as usize];

    let base = parse_duration_base(base_str)?;

    Ok(FermataDuration { base, dots: dot_count })
}

fn parse_duration_base(s: &str) -> CompileResult<DurationBase> {
    match s.to_lowercase().as_str() {
        // Short forms
        "w" | "1" => Ok(DurationBase::Whole),
        "h" | "2" => Ok(DurationBase::Half),
        "q" | "4" => Ok(DurationBase::Quarter),
        "e" | "8" => Ok(DurationBase::Eighth),
        "s" | "16" => Ok(DurationBase::Sixteenth),
        "32" => Ok(DurationBase::ThirtySecond),
        "64" => Ok(DurationBase::SixtyFourth),
        "128" => Ok(DurationBase::OneTwentyEighth),
        "256" => Ok(DurationBase::TwoFiftySixth),
        "512" => Ok(DurationBase::FiveTwelfth),
        "1024" => Ok(DurationBase::OneThousandTwentyFourth),

        // Long forms (American)
        "whole" => Ok(DurationBase::Whole),
        "half" => Ok(DurationBase::Half),
        "quarter" => Ok(DurationBase::Quarter),
        "eighth" => Ok(DurationBase::Eighth),
        "sixteenth" => Ok(DurationBase::Sixteenth),
        "thirty-second" | "thirtysecond" => Ok(DurationBase::ThirtySecond),
        "sixty-fourth" | "sixtyfourth" => Ok(DurationBase::SixtyFourth),
        "one-twenty-eighth" | "onetwentyeighth" => Ok(DurationBase::OneTwentyEighth),
        "two-fifty-sixth" | "twofiftysixth" => Ok(DurationBase::TwoFiftySixth),
        "five-twelfth" | "fivetwelfth" => Ok(DurationBase::FiveTwelfth),
        "one-thousand-twenty-fourth" | "onethousandtwentyfourth" => Ok(DurationBase::OneThousandTwentyFourth),

        // British forms (minim maps to Half, not a separate variant)
        "semibreve" => Ok(DurationBase::Whole),
        "minim" => Ok(DurationBase::Half),
        "crotchet" => Ok(DurationBase::Quarter),
        "quaver" => Ok(DurationBase::Eighth),
        "semiquaver" => Ok(DurationBase::Sixteenth),
        "demisemiquaver" => Ok(DurationBase::ThirtySecond),
        "hemidemisemiquaver" => Ok(DurationBase::SixtyFourth),
        "semihemidemisemiquaver" => Ok(DurationBase::OneTwentyEighth),
        "demisemihemidemisemiquaver" => Ok(DurationBase::TwoFiftySixth),

        // Breve and long
        "breve" | "double-whole" => Ok(DurationBase::Breve),
        "long" | "longa" => Ok(DurationBase::Long),
        "maxima" => Ok(DurationBase::Maxima),

        _ => Err(CompileError::InvalidDuration(s.to_string())),
    }
}

/// Compile Fermata duration to IR NoteType
pub fn compile_duration_type(dur: &FermataDuration) -> CompileResult<NoteType> {
    let value = duration_base_to_note_type_value(dur.base);
    Ok(NoteType { value, size: None })
}

/// Convert DurationBase to NoteTypeValue
fn duration_base_to_note_type_value(base: DurationBase) -> NoteTypeValue {
    match base {
        DurationBase::Maxima => NoteTypeValue::Maxima,
        DurationBase::Long => NoteTypeValue::Long,
        DurationBase::Breve => NoteTypeValue::Breve,
        DurationBase::Whole => NoteTypeValue::Whole,
        DurationBase::Half => NoteTypeValue::Half,
        DurationBase::Quarter => NoteTypeValue::Quarter,
        DurationBase::Eighth => NoteTypeValue::Eighth,
        DurationBase::Sixteenth => NoteTypeValue::N16th,
        DurationBase::ThirtySecond => NoteTypeValue::N32nd,
        DurationBase::SixtyFourth => NoteTypeValue::N64th,
        DurationBase::OneTwentyEighth => NoteTypeValue::N128th,
        DurationBase::TwoFiftySixth => NoteTypeValue::N256th,
        DurationBase::FiveTwelfth => NoteTypeValue::N512th,
        DurationBase::OneThousandTwentyFourth => NoteTypeValue::N1024th,
    }
}

/// Compile duration to IR dots vector
pub fn compile_dots(dur: &FermataDuration) -> Vec<Dot> {
    (0..dur.dots).map(|_| Dot::default()).collect()
}

/// Compile duration to divisions value
/// PositiveDivisions is a type alias (u64), not a newtype
pub fn compile_duration_divisions(dur: &FermataDuration) -> PositiveDivisions {
    use super::defaults::duration_to_divisions;

    let note_type = duration_base_to_note_type_value(dur.base);
    duration_to_divisions(note_type, dur.dots)
}

/// Parse duration from S-expression keyword
pub fn parse_duration_sexpr(sexpr: &Sexpr) -> CompileResult<FermataDuration> {
    match sexpr {
        Sexpr::Keyword(k) => parse_duration(k),
        Sexpr::Symbol(s) if s.starts_with(':') => parse_duration(s),
        Sexpr::Symbol(s) => parse_duration(s),
        _ => Err(CompileError::type_mismatch("duration keyword", format!("{:?}", sexpr))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_short_durations() {
        let d = parse_duration(":q").unwrap();
        assert_eq!(d.base, DurationBase::Quarter);
        assert_eq!(d.dots, 0);

        let d = parse_duration(":h").unwrap();
        assert_eq!(d.base, DurationBase::Half);

        let d = parse_duration(":8").unwrap();
        assert_eq!(d.base, DurationBase::Eighth);
    }

    #[test]
    fn test_parse_long_durations() {
        let d = parse_duration(":quarter").unwrap();
        assert_eq!(d.base, DurationBase::Quarter);

        let d = parse_duration(":whole").unwrap();
        assert_eq!(d.base, DurationBase::Whole);
    }

    #[test]
    fn test_parse_british_durations() {
        let d = parse_duration(":crotchet").unwrap();
        assert_eq!(d.base, DurationBase::Quarter);

        let d = parse_duration(":quaver").unwrap();
        assert_eq!(d.base, DurationBase::Eighth);

        let d = parse_duration(":semibreve").unwrap();
        assert_eq!(d.base, DurationBase::Whole);

        // Minim maps to Half (not a separate variant)
        let d = parse_duration(":minim").unwrap();
        assert_eq!(d.base, DurationBase::Half);
    }

    #[test]
    fn test_parse_dotted_durations() {
        let d = parse_duration(":q.").unwrap();
        assert_eq!(d.base, DurationBase::Quarter);
        assert_eq!(d.dots, 1);

        let d = parse_duration(":h..").unwrap();
        assert_eq!(d.base, DurationBase::Half);
        assert_eq!(d.dots, 2);
    }

    #[test]
    fn test_parse_very_short_durations() {
        let d = parse_duration(":512").unwrap();
        assert_eq!(d.base, DurationBase::FiveTwelfth);

        let d = parse_duration(":1024").unwrap();
        assert_eq!(d.base, DurationBase::OneThousandTwentyFourth);
    }

    #[test]
    fn test_compile_to_ir() {
        let d = parse_duration(":q").unwrap();
        let note_type = compile_duration_type(&d).unwrap();
        assert_eq!(note_type.value, NoteTypeValue::Quarter);

        let d = parse_duration(":q.").unwrap();
        let dots = compile_dots(&d);
        assert_eq!(dots.len(), 1);
    }

    #[test]
    fn test_duration_divisions() {
        let d = parse_duration(":q").unwrap();
        let divs = compile_duration_divisions(&d);
        // PositiveDivisions is a type alias, so compare directly
        assert_eq!(divs, DEFAULT_DIVISIONS);

        let d = parse_duration(":h").unwrap();
        let divs = compile_duration_divisions(&d);
        assert_eq!(divs, DEFAULT_DIVISIONS * 2);

        let d = parse_duration(":q.").unwrap();
        let divs = compile_duration_divisions(&d);
        assert_eq!(divs, DEFAULT_DIVISIONS + DEFAULT_DIVISIONS / 2);
    }

    #[test]
    fn test_invalid_durations() {
        assert!(parse_duration(":xyz").is_err());
        assert!(parse_duration("").is_err());
    }
}
```

---

## Task 3: Note Compilation (`src/lang/note.rs`)

Compile note S-expressions to IR.

```rust
//! Note compilation for Fermata syntax.
//!
//! Compiles `(note c4 :q)` to IR Note structure.

use crate::ir::common::{Position, PositiveDivisions, StartStop, StartStopContinue, EmptyPlacement};
use crate::ir::note::{Note, NoteContent, FullNote, Tie};
use crate::ir::pitch::PitchRestUnpitched;
use crate::ir::duration::{NoteType, Dot};
use crate::ir::beam::{Stem, StemValue};
use crate::ir::notation::{
    Notations, NotationContent, Articulations, ArticulationElement, StrongAccent,
    Ornaments, OrnamentWithAccidentals, OrnamentElement, EmptyTrillSound, Mordent, Turn,
    Slur, Tied, Fermata,
};
use crate::sexpr::Sexpr;
use super::ast::{FermataNote, FermataDuration, StemDirection, Articulation, Ornament};
use super::error::{CompileError, CompileResult};
use super::pitch::{parse_pitch_sexpr, compile_pitch};
use super::duration::{parse_duration_sexpr, compile_duration_type, compile_dots, compile_duration_divisions};
use super::defaults::DEFAULT_VOICE;

/// Parse and compile a note S-expression
///
/// Syntax: `(note <pitch> <duration> [:voice n] [:staff n] [:stem up|down] ...)`
pub fn compile_note(sexpr: &Sexpr) -> CompileResult<Note> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("note list", format!("{:?}", sexpr)))?;

    // Verify head
    if list.first().and_then(|s| s.as_symbol()) != Some("note") {
        return Err(CompileError::UnknownForm("expected (note ...)".to_string()));
    }

    // Parse the note form
    let fermata_note = parse_note_form(&list[1..])?;

    // Compile to IR
    compile_fermata_note(&fermata_note)
}

/// Parse note form arguments into Fermata AST
fn parse_note_form(args: &[Sexpr]) -> CompileResult<FermataNote> {
    if args.is_empty() {
        return Err(CompileError::MissingField("pitch"));
    }

    // First positional arg: pitch (required)
    let pitch = parse_pitch_sexpr(&args[0])?;

    // Second positional arg: duration (required)
    if args.len() < 2 {
        return Err(CompileError::MissingField("duration"));
    }
    let duration = parse_duration_sexpr(&args[1])?;

    // Parse keyword arguments
    let mut voice = None;
    let mut staff = None;
    let mut stem = None;
    let mut articulations = Vec::new();
    let mut ornaments = Vec::new();
    let mut tie = None;
    let mut slur = None;
    let mut fermata = None;
    let lyric = None;

    let mut i = 2;
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
                        stem = Some(parse_stem(&args[i])?);
                        i += 1;
                    }
                }
                "tie" => {
                    if i < args.len() {
                        tie = Some(parse_start_stop(&args[i])?);
                        i += 1;
                    }
                }
                "slur" => {
                    if i < args.len() {
                        slur = Some(parse_start_stop(&args[i])?);
                        i += 1;
                    }
                }
                // Articulation shorthands
                "staccato" => articulations.push(Articulation::Staccato),
                "accent" => articulations.push(Articulation::Accent),
                "tenuto" => articulations.push(Articulation::Tenuto),
                "marcato" => articulations.push(Articulation::StrongAccent),
                // Fermata is separate (not an Articulation in our AST)
                "fermata" => fermata = Some(super::ast::FermataMark::default()),
                // Ornament shorthands
                "trill" => ornaments.push(Ornament::Trill),
                "mordent" => ornaments.push(Ornament::Mordent),
                "turn" => ornaments.push(Ornament::Turn),
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
        lyric,
    })
}

/// Parse an integer from S-expression (handles both Symbol and Integer)
fn parse_u32(sexpr: &Sexpr) -> CompileResult<u32> {
    match sexpr {
        Sexpr::Integer(n) => {
            if *n >= 0 {
                Ok(*n as u32)
            } else {
                Err(CompileError::type_mismatch("positive integer", n.to_string()))
            }
        }
        Sexpr::Symbol(s) => s.parse::<u32>()
            .map_err(|_| CompileError::type_mismatch("integer", s.clone())),
        _ => Err(CompileError::type_mismatch("integer", format!("{:?}", sexpr))),
    }
}

fn parse_stem(sexpr: &Sexpr) -> CompileResult<StemDirection> {
    match sexpr.as_symbol() {
        Some("up") => Ok(StemDirection::Up),
        Some("down") => Ok(StemDirection::Down),
        Some("none") => Ok(StemDirection::None),
        Some("double") => Ok(StemDirection::Double),
        _ => Err(CompileError::type_mismatch("stem direction (up/down/none/double)", format!("{:?}", sexpr))),
    }
}

fn parse_start_stop(sexpr: &Sexpr) -> CompileResult<StartStop> {
    match sexpr.as_symbol() {
        Some("start") => Ok(StartStop::Start),
        Some("stop") => Ok(StartStop::Stop),
        _ => Err(CompileError::type_mismatch("start/stop", format!("{:?}", sexpr))),
    }
}

/// Compile FermataNote to IR Note
pub fn compile_fermata_note(note: &FermataNote) -> CompileResult<Note> {
    let pitch = compile_pitch(&note.pitch)?;
    let note_type = compile_duration_type(&note.duration)?;
    let dots = compile_dots(&note.duration);
    let duration = compile_duration_divisions(&note.duration);

    let full_note = FullNote {
        chord: false,
        content: PitchRestUnpitched::Pitch(pitch),
    };

    let content = NoteContent::Regular {
        full_note,
        duration,
        ties: compile_ties(&note.tie),
    };

    let stem = note.stem.as_ref().map(compile_stem_ir);

    Ok(Note {
        // Position/playback attributes (defaults for user-facing syntax)
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
        voice: Some(note.voice.map(|v| v.to_string()).unwrap_or_else(|| DEFAULT_VOICE.to_string())),
        r#type: Some(note_type),
        dots,
        accidental: None,  // Derived from pitch in Phase 5 extension
        time_modification: None,
        stem,
        notehead: None,
        staff: note.staff,
        beams: Vec::new(),  // Auto-beaming TBD
        notations: compile_notations(note)?,
        lyrics: Vec::new(),  // Lyric compilation TBD
    })
}

fn compile_ties(tie: &Option<StartStop>) -> Vec<Tie> {
    tie.as_ref().map(|t| vec![Tie {
        r#type: *t,
        time_only: None,
    }]).unwrap_or_default()
}

fn compile_stem_ir(dir: &StemDirection) -> Stem {
    let value = match dir {
        StemDirection::Up => StemValue::Up,
        StemDirection::Down => StemValue::Down,
        StemDirection::None => StemValue::None,
        StemDirection::Double => StemValue::Double,
    };
    Stem {
        value,
        default_y: None,
        relative_y: None,
        color: None,
    }
}

fn compile_notations(note: &FermataNote) -> CompileResult<Vec<Notations>> {
    let mut content = Vec::new();

    // Compile articulations
    // ArticulationElement uses EmptyPlacement for most articulations, not separate structs
    if !note.articulations.is_empty() {
        let arts: Vec<ArticulationElement> = note.articulations.iter()
            .map(|a| match a {
                Articulation::Staccato => ArticulationElement::Staccato(EmptyPlacement::default()),
                Articulation::Accent => ArticulationElement::Accent(EmptyPlacement::default()),
                Articulation::Tenuto => ArticulationElement::Tenuto(EmptyPlacement::default()),
                Articulation::StrongAccent => ArticulationElement::StrongAccent(StrongAccent::default()),
                Articulation::Staccatissimo => ArticulationElement::Staccatissimo(EmptyPlacement::default()),
                Articulation::Spiccato => ArticulationElement::Spiccato(EmptyPlacement::default()),
                Articulation::DetachedLegato => ArticulationElement::DetachedLegato(EmptyPlacement::default()),
            })
            .collect();

        if !arts.is_empty() {
            // Articulations is boxed in NotationContent
            content.push(NotationContent::Articulations(Box::new(Articulations { content: arts })));
        }
    }

    // Compile ornaments
    // OrnamentElement uses EmptyTrillSound for TrillMark, concrete structs for Mordent/Turn
    if !note.ornaments.is_empty() {
        let orns: Vec<OrnamentWithAccidentals> = note.ornaments.iter()
            .map(|o| {
                let ornament = match o {
                    Ornament::Trill => OrnamentElement::TrillMark(EmptyTrillSound::default()),
                    Ornament::Mordent => OrnamentElement::Mordent(Mordent::default()),
                    Ornament::InvertedMordent => OrnamentElement::InvertedMordent(Mordent::default()),
                    Ornament::Turn => OrnamentElement::Turn(Turn::default()),
                    Ornament::InvertedTurn => OrnamentElement::InvertedTurn(Turn::default()),
                    Ornament::DelayedTurn => OrnamentElement::DelayedTurn(Turn::default()),
                    Ornament::Shake => OrnamentElement::Shake(EmptyTrillSound::default()),
                };
                OrnamentWithAccidentals {
                    ornament,
                    accidental_marks: Vec::new(),
                }
            })
            .collect();

        // Ornaments is boxed in NotationContent
        content.push(NotationContent::Ornaments(Box::new(Ornaments { content: orns })));
    }

    // Compile slur
    // Slur.number is NumberLevel (u8), not Option<u8> - it's required
    if let Some(action) = &note.slur {
        content.push(NotationContent::Slur(Slur {
            r#type: start_stop_to_continue(*action),
            number: 1,  // Required field, not Option
            line_type: None,
            position: Default::default(),
            placement: None,
            orientation: None,
            color: None,
        }));
    }

    // Compile tied (visual)
    // Tied.r#type is StartStopContinue, not StartStop
    if let Some(action) = &note.tie {
        content.push(NotationContent::Tied(Tied {
            r#type: start_stop_to_continue(*action),
            number: None,
            line_type: None,
            position: Default::default(),
            placement: None,
            orientation: None,
            color: None,
        }));
    }

    if content.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(vec![Notations {
            print_object: None,
            content,
            editorial: Default::default(),
        }])
    }
}

/// Convert StartStop to StartStopContinue (for Tied and Slur)
fn start_stop_to_continue(ss: StartStop) -> StartStopContinue {
    match ss {
        StartStop::Start => StartStopContinue::Start,
        StartStop::Stop => StartStopContinue::Stop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;
    use crate::ir::duration::NoteTypeValue;

    #[test]
    fn test_simple_note() {
        let sexpr = parse("(note c4 :q)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert!(matches!(note.content, NoteContent::Regular { .. }));
        assert_eq!(note.r#type.as_ref().unwrap().value, NoteTypeValue::Quarter);
    }

    #[test]
    fn test_note_with_voice() {
        let sexpr = parse("(note d5 :h :voice 2)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert_eq!(note.voice, Some("2".to_string()));
    }

    #[test]
    fn test_note_with_stem() {
        let sexpr = parse("(note e4 :q :stem down)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert_eq!(note.stem.as_ref().unwrap().value, StemValue::Down);
    }

    #[test]
    fn test_note_with_articulation() {
        let sexpr = parse("(note c4 :q :staccato)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert!(!note.notations.is_empty());
    }

    #[test]
    fn test_note_with_tie() {
        let sexpr = parse("(note g4 :w :tie start)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        if let NoteContent::Regular { ties, .. } = &note.content {
            assert_eq!(ties.len(), 1);
        } else {
            panic!("Expected regular note");
        }
    }

    #[test]
    fn test_note_with_integer_voice() {
        // Test that Integer sexpr works for voice
        let sexpr = parse("(note c4 :q :voice 3)").unwrap();
        let note = compile_note(&sexpr).unwrap();
        assert_eq!(note.voice, Some("3".to_string()));
    }
}
```

---

## Task 4: Rest Compilation

Add rest handling to `src/lang/note.rs`:

```rust
/// Compile a rest S-expression
///
/// Syntax: `(rest <duration> [:voice n] [:measure])`
pub fn compile_rest(sexpr: &Sexpr) -> CompileResult<Note> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("rest list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("rest") {
        return Err(CompileError::UnknownForm("expected (rest ...)".to_string()));
    }

    let fermata_rest = parse_rest_form(&list[1..])?;
    compile_fermata_rest(&fermata_rest)
}

fn parse_rest_form(args: &[Sexpr]) -> CompileResult<super::ast::FermataRest> {
    if args.is_empty() {
        return Err(CompileError::MissingField("duration"));
    }

    let duration = parse_duration_sexpr(&args[0])?;

    let mut voice = None;
    let mut staff = None;
    let mut measure_rest = false;

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
                "measure" => {
                    measure_rest = true;
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

    Ok(super::ast::FermataRest {
        duration,
        voice,
        staff,
        measure_rest,
    })
}

fn compile_fermata_rest(rest: &super::ast::FermataRest) -> CompileResult<Note> {
    use crate::ir::note::Rest;

    let note_type = compile_duration_type(&rest.duration)?;
    let dots = compile_dots(&rest.duration);
    let duration = compile_duration_divisions(&rest.duration);

    let full_note = FullNote {
        chord: false,
        content: PitchRestUnpitched::Rest(Rest {
            display_step: None,
            display_octave: None,
            measure: if rest.measure_rest { Some(true) } else { None },
        }),
    };

    let content = NoteContent::Regular {
        full_note,
        duration,
        ties: Vec::new(),
    };

    Ok(Note {
        // Position/playback attributes (defaults for user-facing syntax)
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
        voice: Some(rest.voice.map(|v| v.to_string()).unwrap_or_else(|| DEFAULT_VOICE.to_string())),
        r#type: Some(note_type),
        dots,
        accidental: None,
        time_modification: None,
        stem: None,
        notehead: None,
        staff: rest.staff,
        beams: Vec::new(),
        notations: Vec::new(),
        lyrics: Vec::new(),
    })
}

#[cfg(test)]
mod rest_tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_simple_rest() {
        let sexpr = parse("(rest :h)").unwrap();
        let note = compile_rest(&sexpr).unwrap();

        if let NoteContent::Regular { full_note, .. } = &note.content {
            assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
        } else {
            panic!("Expected regular note content");
        }
    }

    #[test]
    fn test_measure_rest() {
        let sexpr = parse("(rest :w :measure)").unwrap();
        let note = compile_rest(&sexpr).unwrap();

        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Rest(r) = &full_note.content {
                assert_eq!(r.measure, Some(true));
            }
        }
    }

    #[test]
    fn test_rest_with_voice() {
        let sexpr = parse("(rest :q :voice 2)").unwrap();
        let note = compile_rest(&sexpr).unwrap();
        assert_eq!(note.voice, Some("2".to_string()));
    }
}
```

---

## Task 5: Additional Integration Tests (inline)

Add comprehensive tests to `src/lang/note.rs`:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::sexpr::parser::parse;
    use crate::ir::pitch::Step;
    use crate::ir::duration::NoteTypeValue;

    #[test]
    fn test_pitch_c4() {
        let pitch = parse_pitch("c4").unwrap();
        assert_eq!(pitch.step, Step::C);
        // Octave is type alias u8, not newtype
        assert_eq!(pitch.octave, 4);
        assert_eq!(pitch.alter, None);
    }

    #[test]
    fn test_pitch_f_sharp_5() {
        let pitch = parse_pitch("f#5").unwrap();
        assert_eq!(pitch.step, Step::F);
        assert_eq!(pitch.octave, 5);
        // Semitones is type alias f64, not newtype
        assert_eq!(pitch.alter, Some(1.0));
    }

    #[test]
    fn test_pitch_b_flat_3() {
        let pitch = parse_pitch("bb3").unwrap();
        assert_eq!(pitch.step, Step::B);
        assert_eq!(pitch.octave, 3);
        assert_eq!(pitch.alter, Some(-1.0));
    }

    #[test]
    fn test_note_quarter() {
        let sexpr = parse("(note c4 :q)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        match note.content {
            NoteContent::Regular { full_note, .. } => {
                assert!(!full_note.chord);
            }
            _ => panic!("Expected regular note"),
        }

        assert_eq!(note.r#type.as_ref().unwrap().value, NoteTypeValue::Quarter);
    }

    #[test]
    fn test_note_dotted_half() {
        let sexpr = parse("(note d5 :h.)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert_eq!(note.r#type.as_ref().unwrap().value, NoteTypeValue::Half);
        assert_eq!(note.dots.len(), 1);
    }

    #[test]
    fn test_note_all_durations() {
        // Verify all duration forms compile
        let durations = [
            ":w", ":h", ":q", ":8", ":16", ":32", ":64", ":128", ":256", ":512", ":1024",
            ":whole", ":half", ":quarter", ":eighth",
            ":semibreve", ":minim", ":crotchet", ":quaver",
        ];

        for dur in durations {
            let source = format!("(note c4 {})", dur);
            let sexpr = parse(&source).unwrap();
            let result = compile_note(&sexpr);
            assert!(result.is_ok(), "Failed to compile duration {}: {:?}", dur, result);
        }
    }

    #[test]
    fn test_rest_basic() {
        let sexpr = parse("(rest :q)").unwrap();
        let note = compile_rest(&sexpr).unwrap();

        match note.content {
            NoteContent::Regular { full_note, .. } => {
                assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
            }
            _ => panic!("Expected regular note content"),
        }
    }

    #[test]
    fn test_note_with_multiple_articulations() {
        let sexpr = parse("(note c4 :q :staccato :accent)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert!(!note.notations.is_empty());
        // Should have articulations content
        let notations = &note.notations[0];
        assert!(!notations.content.is_empty());
    }

    #[test]
    fn test_note_with_ornament() {
        let sexpr = parse("(note c4 :q :trill)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert!(!note.notations.is_empty());
    }
}
```

---

## Acceptance Criteria

1. ✅ Pitch parsing handles all cases: `c4`, `f#5`, `bb3`, `cn4`, `c+4`, `cx4`
2. ✅ Duration parsing handles all aliases: `:q`, `:quarter`, `:crotchet`, `:minim`
3. ✅ Dotted durations work: `:q.`, `:h..`
4. ✅ Extended durations work: `:512`, `:1024`
5. ✅ `(note c4 :q)` compiles to valid IR Note
6. ✅ `(rest :h)` compiles to valid IR Rest Note
7. ✅ Optional kwargs work: `:voice`, `:staff`, `:stem`
8. ✅ Articulation/ornament shorthands work: `:staccato`, `:trill`
9. ✅ All unit and integration tests pass

---

## Implementation Notes

1. **Type aliases** — `Semitones`, `Octave`, and `PositiveDivisions` are type aliases (not newtypes), so use values directly: `pitch.octave` not `Octave(pitch.octave)`

2. **Case insensitivity** — Pitch steps are case-insensitive (`C4` = `c4`)

3. **Ambiguity with 'b'** — `b4` is pitch B, `bb4` is B-flat; parser must look ahead

4. **British terminology** — `:minim` maps to `DurationBase::Half` (not a separate variant)

5. **Default voice** — Use `"1"` as default voice string

6. **Duration divisions** — Use `DEFAULT_DIVISIONS` (960) as base unit

7. **Articulation types** — `ArticulationElement` uses `EmptyPlacement` for most articulations, not separate structs

8. **Ornament types** — `OrnamentElement` uses `EmptyTrillSound` for trill marks, concrete structs for `Mordent`/`Turn`

9. **Boxing** — `NotationContent::Articulations` and `NotationContent::Ornaments` wrap boxed values

10. **Slur.number** — Is a required `NumberLevel` (u8), not `Option<u8>`

11. **Tied/Slur types** — Use `StartStopContinue`, not `StartStop`; convert with helper function

12. **Fermata handling** — Fermata is separate from Articulation in the AST (uses `FermataMark`)

13. **parse_u32** — Must handle both `Sexpr::Integer` and `Sexpr::Symbol` for flexibility

14. **Note struct has many fields** — The IR `Note` struct includes position/playback fields (`position`, `dynamics`, `end_dynamics`, `attack`, `release`, `pizzicato`, `print_object`) that should be set to defaults for user-facing syntax. The `instrument` field is `Vec<Instrument>`, not `Option`.

---

*Next: Milestone 3 — Compound Structures (Chords, Tuplets, Grace Notes)*
