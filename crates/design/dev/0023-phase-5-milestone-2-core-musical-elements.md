# Phase 5 — Milestone 2: Core Musical Elements

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Pitch parsing, duration parsing, note compilation, rest compilation
> **Depends On:** Milestone 1 (Foundation)
> **Estimated Implementation Time:** 3-4 hours

---

## Overview

This milestone implements the core musical element parsers and compilers:

- **Pitch parsing** — `c4`, `f#5`, `bb3`, `cn4`, microtones
- **Duration parsing** — `:q`, `:quarter`, `:crotchet`, dots
- **Note compilation** — `(note c4 :q)` → IR Note
- **Rest compilation** — `(rest :h)` → IR Rest Note

---

## Task 1: Pitch Parsing (`src/fermata/pitch.rs`)

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

use crate::ir::pitch::{Pitch, Step, Octave, Semitones};
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
            'b' if chars.clone().skip(1).next().map(|c| c.is_ascii_digit()).unwrap_or(false) == false => {
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

    let alter = pitch.alter.as_ref().map(|a| Semitones(a.to_semitones() as f32));

    Ok(Pitch {
        step,
        alter,
        octave: Octave(pitch.octave),
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
        assert_eq!(p.alter, Some(Semitones(1.0)));
        assert_eq!(p.octave, Octave(5));
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

## Task 2: Duration Parsing (`src/fermata/duration.rs`)

Parse duration keywords and compile to IR.

```rust
//! Duration parsing for Fermata syntax.
//!
//! Supports multiple naming conventions:
//! - Short: `:w`, `:h`, `:q`, `:8`, `:16`, `:32`, `:64`
//! - Long: `:whole`, `:half`, `:quarter`, `:eighth`, `:sixteenth`
//! - British: `:semibreve`, `:minim`, `:crotchet`, `:quaver`, `:semiquaver`
//! - Dots: `:q.` (dotted), `:h..` (double-dotted)

use crate::ir::duration::{NoteType, NoteTypeValue, Dot, PositiveDivisions};
use crate::sexpr::Sexpr;
use super::ast::{FermataDuration, DurationBase};
use super::error::{CompileError, CompileResult};
use super::defaults::DEFAULT_DIVISIONS;

/// Parse a duration keyword like ":q", ":quarter", ":h."
pub fn parse_duration(s: &str) -> CompileResult<FermataDuration> {
    let s = s.trim();

    // Handle keyword prefix
    let s = if s.starts_with(':') { &s[1..] } else { s };

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

        // Long forms (American)
        "whole" => Ok(DurationBase::Whole),
        "half" => Ok(DurationBase::Half),
        "quarter" => Ok(DurationBase::Quarter),
        "eighth" => Ok(DurationBase::Eighth),
        "sixteenth" => Ok(DurationBase::Sixteenth),
        "thirty-second" | "thirtysecond" => Ok(DurationBase::ThirtySecond),
        "sixty-fourth" | "sixtyfourth" => Ok(DurationBase::SixtyFourth),

        // British forms
        "semibreve" => Ok(DurationBase::Whole),
        "minim" => Ok(DurationBase::Minim),
        "crotchet" => Ok(DurationBase::Quarter),
        "quaver" => Ok(DurationBase::Eighth),
        "semiquaver" => Ok(DurationBase::Sixteenth),
        "demisemiquaver" => Ok(DurationBase::ThirtySecond),
        "hemidemisemiquaver" => Ok(DurationBase::SixtyFourth),

        // Breve and long
        "breve" | "double-whole" => Ok(DurationBase::Breve),
        "long" | "longa" => Ok(DurationBase::Long),
        "maxima" => Ok(DurationBase::Maxima),

        _ => Err(CompileError::InvalidDuration(s.to_string())),
    }
}

/// Compile Fermata duration to IR NoteType
pub fn compile_duration_type(dur: &FermataDuration) -> CompileResult<NoteType> {
    let value = match dur.base {
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
    };

    Ok(NoteType { value, size: None })
}

/// Compile duration to IR dots vector
pub fn compile_dots(dur: &FermataDuration) -> Vec<Dot> {
    (0..dur.dots).map(|_| Dot::default()).collect()
}

/// Compile duration to divisions value
pub fn compile_duration_divisions(dur: &FermataDuration) -> PositiveDivisions {
    use super::defaults::duration_to_divisions;

    let note_type = match dur.base {
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
    };

    PositiveDivisions(duration_to_divisions(note_type, dur.dots))
}

/// Parse duration from S-expression keyword
pub fn parse_duration_sexpr(sexpr: &Sexpr) -> CompileResult<FermataDuration> {
    match sexpr {
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
        assert_eq!(divs.0, DEFAULT_DIVISIONS);

        let d = parse_duration(":h").unwrap();
        let divs = compile_duration_divisions(&d);
        assert_eq!(divs.0, DEFAULT_DIVISIONS * 2);

        let d = parse_duration(":q.").unwrap();
        let divs = compile_duration_divisions(&d);
        assert_eq!(divs.0, DEFAULT_DIVISIONS + DEFAULT_DIVISIONS / 2);
    }

    #[test]
    fn test_invalid_durations() {
        assert!(parse_duration(":xyz").is_err());
        assert!(parse_duration("").is_err());
    }
}
```

**Note:** Add `Minim` alias to the AST `DurationBase` enum if not already present (it's an alias for `Half` in British terminology).

---

## Task 3: Note Compilation (`src/fermata/note.rs`)

Compile note S-expressions to IR.

```rust
//! Note compilation for Fermata syntax.
//!
//! Compiles `(note c4 :q)` to IR Note structure.

use crate::ir::note::{Note, NoteContent, FullNote};
use crate::ir::pitch::{Pitch, PitchRestUnpitched};
use crate::ir::duration::{NoteType, Dot, PositiveDivisions};
use crate::ir::beam::{Stem, StemValue};
use crate::sexpr::Sexpr;
use super::ast::{FermataNote, FermataPitch, FermataDuration, StemDirection};
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
    let mut lyric = None;

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
                "staccato" => articulations.push(super::ast::Articulation::Staccato),
                "accent" => articulations.push(super::ast::Articulation::Accent),
                "tenuto" => articulations.push(super::ast::Articulation::Tenuto),
                "marcato" => articulations.push(super::ast::Articulation::StrongAccent),
                "fermata" => articulations.push(super::ast::Articulation::Fermata),
                // Ornament shorthands
                "trill" => ornaments.push(super::ast::Ornament::Trill),
                "mordent" => ornaments.push(super::ast::Ornament::Mordent),
                "turn" => ornaments.push(super::ast::Ornament::Turn),
                _ => {
                    // Unknown keyword - skip value if present
                    if i < args.len() && !args[i].is_keyword() {
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

fn parse_u32(sexpr: &Sexpr) -> CompileResult<u32> {
    match sexpr {
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

fn parse_start_stop(sexpr: &Sexpr) -> CompileResult<crate::ir::common::StartStop> {
    match sexpr.as_symbol() {
        Some("start") => Ok(crate::ir::common::StartStop::Start),
        Some("stop") => Ok(crate::ir::common::StartStop::Stop),
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

    let stem = note.stem.as_ref().map(|s| compile_stem(s));

    Ok(Note {
        content,
        voice: Some(note.voice.map(|v| v.to_string()).unwrap_or_else(|| DEFAULT_VOICE.to_string())),
        r#type: Some(note_type),
        dots,
        accidental: None,  // Derived from pitch in Phase 5 extension
        time_modification: None,
        stem,
        staff: note.staff,
        beams: Vec::new(),  // Auto-beaming TBD
        notations: compile_notations(note)?,
        lyrics: Vec::new(),  // Lyric compilation TBD
        notehead: None,
        instrument: None,
    })
}

fn compile_ties(tie: &Option<crate::ir::common::StartStop>) -> Vec<crate::ir::note::Tie> {
    tie.as_ref().map(|t| vec![crate::ir::note::Tie {
        r#type: *t,
        time_only: None,
    }]).unwrap_or_default()
}

fn compile_stem(dir: &StemDirection) -> Stem {
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

fn compile_notations(note: &FermataNote) -> CompileResult<Vec<crate::ir::notation::Notations>> {
    use crate::ir::notation::*;

    let mut content = Vec::new();

    // Compile articulations
    if !note.articulations.is_empty() {
        let arts: Vec<ArticulationContent> = note.articulations.iter()
            .filter_map(|a| match a {
                super::ast::Articulation::Staccato => Some(ArticulationContent::Staccato(Staccato::default())),
                super::ast::Articulation::Accent => Some(ArticulationContent::Accent(Accent::default())),
                super::ast::Articulation::Tenuto => Some(ArticulationContent::Tenuto(Tenuto::default())),
                super::ast::Articulation::StrongAccent => Some(ArticulationContent::StrongAccent(StrongAccent::default())),
                super::ast::Articulation::Fermata => None, // Fermata goes elsewhere
                _ => None,
            })
            .collect();

        if !arts.is_empty() {
            content.push(NotationContent::Articulations(Articulations { content: arts }));
        }
    }

    // Compile fermata separately
    for art in &note.articulations {
        if matches!(art, super::ast::Articulation::Fermata) {
            content.push(NotationContent::Fermata(Fermata::default()));
        }
    }

    // Compile ornaments
    if !note.ornaments.is_empty() {
        let orns: Vec<OrnamentWithAccidentals> = note.ornaments.iter()
            .map(|o| {
                let ornament = match o {
                    super::ast::Ornament::Trill => OrnamentContent::TrillMark(TrillMark::default()),
                    super::ast::Ornament::Mordent => OrnamentContent::Mordent(Mordent::default()),
                    super::ast::Ornament::Turn => OrnamentContent::Turn(Turn::default()),
                    _ => OrnamentContent::TrillMark(TrillMark::default()),
                };
                OrnamentWithAccidentals {
                    ornament,
                    accidental_marks: Vec::new(),
                }
            })
            .collect();

        content.push(NotationContent::Ornaments(Ornaments { content: orns }));
    }

    // Compile slur
    if let Some(action) = &note.slur {
        content.push(NotationContent::Slur(Slur {
            r#type: *action,
            number: Some(1),
            ..Default::default()
        }));
    }

    // Compile tied (visual)
    if let Some(action) = &note.tie {
        content.push(NotationContent::Tied(Tied {
            r#type: *action,
            ..Default::default()
        }));
    }

    if content.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(vec![Notations { content }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_simple_note() {
        let sexpr = parse("(note c4 :q)").unwrap();
        let note = compile_note(&sexpr).unwrap();

        assert!(matches!(note.content, NoteContent::Regular { .. }));
        assert_eq!(note.r#type.as_ref().unwrap().value, crate::ir::duration::NoteTypeValue::Quarter);
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
}
```

---

## Task 4: Rest Compilation

Add rest handling to `src/fermata/note.rs`:

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
                    if i < args.len() && !args[i].is_keyword() {
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
    use crate::ir::pitch::PitchRestUnpitched;

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
        content,
        voice: Some(rest.voice.map(|v| v.to_string()).unwrap_or_else(|| DEFAULT_VOICE.to_string())),
        r#type: Some(note_type),
        dots,
        accidental: None,
        time_modification: None,
        stem: None,
        staff: rest.staff,
        beams: Vec::new(),
        notations: Vec::new(),
        lyrics: Vec::new(),
        notehead: None,
        instrument: None,
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
}
```

---

## Task 5: Integration Tests

Create `tests/fermata_core_elements.rs`:

```rust
//! Integration tests for core musical elements.

use fermata::fermata::compiler::{compile_note_str, compile_pitch_str};
use fermata::ir::pitch::{Step, Octave, Semitones};
use fermata::ir::duration::NoteTypeValue;
use fermata::ir::note::NoteContent;

#[test]
fn test_pitch_c4() {
    let pitch = compile_pitch_str("c4").unwrap();
    assert_eq!(pitch.step, Step::C);
    assert_eq!(pitch.octave, Octave(4));
    assert_eq!(pitch.alter, None);
}

#[test]
fn test_pitch_f_sharp_5() {
    let pitch = compile_pitch_str("f#5").unwrap();
    assert_eq!(pitch.step, Step::F);
    assert_eq!(pitch.octave, Octave(5));
    assert_eq!(pitch.alter, Some(Semitones(1.0)));
}

#[test]
fn test_pitch_b_flat_3() {
    let pitch = compile_pitch_str("bb3").unwrap();
    assert_eq!(pitch.step, Step::B);
    assert_eq!(pitch.octave, Octave(3));
    assert_eq!(pitch.alter, Some(Semitones(-1.0)));
}

#[test]
fn test_note_quarter() {
    let note = compile_note_str("(note c4 :q)").unwrap();

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
    let note = compile_note_str("(note d5 :h.)").unwrap();

    assert_eq!(note.r#type.as_ref().unwrap().value, NoteTypeValue::Half);
    assert_eq!(note.dots.len(), 1);
}

#[test]
fn test_note_all_durations() {
    // Verify all duration forms compile
    let durations = [
        ":w", ":h", ":q", ":8", ":16", ":32",
        ":whole", ":half", ":quarter", ":eighth",
        ":semibreve", ":minim", ":crotchet", ":quaver",
    ];

    for dur in durations {
        let source = format!("(note c4 {})", dur);
        let result = compile_note_str(&source);
        assert!(result.is_ok(), "Failed to compile duration {}: {:?}", dur, result);
    }
}

#[test]
fn test_rest_basic() {
    let note = compile_note_str("(rest :q)").unwrap();

    match note.content {
        NoteContent::Regular { full_note, .. } => {
            use fermata::ir::pitch::PitchRestUnpitched;
            assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
        }
        _ => panic!("Expected regular note content"),
    }
}
```

---

## Acceptance Criteria

1. ✅ Pitch parsing handles all cases: `c4`, `f#5`, `bb3`, `cn4`, `c+4`, `cx4`
2. ✅ Duration parsing handles all aliases: `:q`, `:quarter`, `:crotchet`
3. ✅ Dotted durations work: `:q.`, `:h..`
4. ✅ `(note c4 :q)` compiles to valid IR Note
5. ✅ `(rest :h)` compiles to valid IR Rest Note
6. ✅ Optional kwargs work: `:voice`, `:staff`, `:stem`
7. ✅ Articulation/ornament shorthands work: `:staccato`, `:trill`
8. ✅ All unit and integration tests pass

---

## Implementation Notes

1. **Case insensitivity** — Pitch steps are case-insensitive (`C4` = `c4`)

2. **Ambiguity with 'b'** — `b4` is pitch B, `bb4` is B-flat; parser must look ahead

3. **Default voice** — Use `"1"` as default voice string

4. **Duration divisions** — Use `DEFAULT_DIVISIONS` (960) as base unit

5. **Missing types** — You may need to add `Default` impls to some IR notation types

---

*Next: Milestone 3 — Compound Structures (Chords, Tuplets, Grace Notes)*
