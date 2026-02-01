# Phase 5 — Milestone 4: Attributes & Directions

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Key signatures, time signatures, clefs, dynamics, tempo, articulations
> **Depends On:** Milestone 2 (Core Elements)
> **Estimated Implementation Time:** 3-4 hours

---

## Overview

This milestone implements musical attributes and directions:

- **Key signatures** — `(key c :major)`, `(key g :major)`, `(key f :minor)`
- **Time signatures** — `(time 4 4)`, `(time 6 8)`, `(time :common)`
- **Clefs** — `(clef :treble)`, `(clef :bass)`, `(clef :alto)`
- **Dynamics** — `(ff)`, `(pp)`, `(cresc)`, `(dim)`
- **Tempo** — `(tempo :q 120)`, `(tempo "Allegro" :q 120)`
- **Articulations** — `(staccato)`, `(accent)`, `(fermata)`

---

## Task 1: Key Signature Compilation (`src/fermata/attributes.rs`)

```rust
//! Attribute compilation for Fermata syntax.
//!
//! Compiles key signatures, time signatures, clefs, and transposes.

use crate::ir::attributes::{
    Key, KeyContent, TraditionalKey, NonTraditionalKey, KeyStep,
    Time, TimeContent, TimeMeasured, TimeSignature, TimeSymbol,
    Clef, ClefSign,
    Transpose,
};
use crate::ir::common::Mode;
use crate::sexpr::Sexpr;
use super::ast::{KeySpec, TimeSpec, ClefSpec, PitchStep, Mode as FermataMode};
use super::error::{CompileError, CompileResult};

// ============ KEY SIGNATURE ============

/// Compile a key signature S-expression.
///
/// Syntax:
/// - `(key c :major)` — C major (0 fifths)
/// - `(key g :major)` — G major (1 fifth)
/// - `(key a :minor)` — A minor (0 fifths, minor mode)
/// - `(key d :dorian)` — D dorian
pub fn compile_key(sexpr: &Sexpr) -> CompileResult<Key> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("key list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("key") {
        return Err(CompileError::UnknownForm("expected (key ...)".to_string()));
    }

    let key_spec = parse_key_form(&list[1..])?;
    compile_key_spec(&key_spec)
}

fn parse_key_form(args: &[Sexpr]) -> CompileResult<KeySpec> {
    if args.is_empty() {
        return Err(CompileError::MissingField("root"));
    }

    // First arg: root note (c, g, d, etc.)
    let root_str = args[0].as_symbol()
        .ok_or_else(|| CompileError::type_mismatch("root note", format!("{:?}", args[0])))?;

    let root = parse_key_root(root_str)?;

    // Second arg: mode (required)
    if args.len() < 2 {
        return Err(CompileError::MissingField("mode"));
    }

    let mode_str = args[1].as_keyword()
        .or_else(|| args[1].as_symbol())
        .ok_or_else(|| CompileError::type_mismatch("mode", format!("{:?}", args[1])))?;

    let mode = parse_mode(mode_str)?;

    Ok(KeySpec { root, mode })
}

fn parse_key_root(s: &str) -> CompileResult<PitchStep> {
    match s.to_lowercase().chars().next() {
        Some('c') => Ok(PitchStep::C),
        Some('d') => Ok(PitchStep::D),
        Some('e') => Ok(PitchStep::E),
        Some('f') => Ok(PitchStep::F),
        Some('g') => Ok(PitchStep::G),
        Some('a') => Ok(PitchStep::A),
        Some('b') => Ok(PitchStep::B),
        _ => Err(CompileError::InvalidKey(format!("invalid root: {}", s))),
    }
}

fn parse_mode(s: &str) -> CompileResult<FermataMode> {
    match s.to_lowercase().as_str() {
        "major" => Ok(FermataMode::Major),
        "minor" => Ok(FermataMode::Minor),
        "dorian" => Ok(FermataMode::Dorian),
        "phrygian" => Ok(FermataMode::Phrygian),
        "lydian" => Ok(FermataMode::Lydian),
        "mixolydian" => Ok(FermataMode::Mixolydian),
        "aeolian" => Ok(FermataMode::Aeolian),
        "ionian" => Ok(FermataMode::Ionian),
        "locrian" => Ok(FermataMode::Locrian),
        _ => Err(CompileError::InvalidKey(format!("invalid mode: {}", s))),
    }
}

fn compile_key_spec(spec: &KeySpec) -> CompileResult<Key> {
    let fifths = compute_fifths(spec.root, &spec.mode);
    let mode = match spec.mode {
        FermataMode::Major => Some(Mode::Major),
        FermataMode::Minor => Some(Mode::Minor),
        FermataMode::Dorian => Some(Mode::Dorian),
        FermataMode::Phrygian => Some(Mode::Phrygian),
        FermataMode::Lydian => Some(Mode::Lydian),
        FermataMode::Mixolydian => Some(Mode::Mixolydian),
        FermataMode::Aeolian => Some(Mode::Aeolian),
        FermataMode::Ionian => Some(Mode::Ionian),
        FermataMode::Locrian => Some(Mode::Locrian),
    };

    Ok(Key {
        content: KeyContent::Traditional(TraditionalKey {
            cancel: None,
            fifths,
            mode,
        }),
        number: None,
        print_object: None,
    })
}

/// Compute fifths value for a key signature.
///
/// | Root | Major | Minor | Dorian |
/// |------|-------|-------|--------|
/// | C    | 0     | -3    | -2     |
/// | G    | 1     | -2    | -1     |
/// | D    | 2     | -1    | 0      |
/// | A    | 3     | 0     | 1      |
/// | E    | 4     | 1     | 2      |
/// | B    | 5     | 2     | 3      |
/// | F    | -1    | -4    | -3     |
fn compute_fifths(root: PitchStep, mode: &FermataMode) -> i8 {
    // Major key fifths
    let major_fifths = match root {
        PitchStep::C => 0,
        PitchStep::G => 1,
        PitchStep::D => 2,
        PitchStep::A => 3,
        PitchStep::E => 4,
        PitchStep::B => 5,
        PitchStep::F => -1,
    };

    // Adjust for mode (relative to major)
    let mode_offset = match mode {
        FermataMode::Major | FermataMode::Ionian => 0,
        FermataMode::Minor | FermataMode::Aeolian => -3,
        FermataMode::Dorian => -2,
        FermataMode::Phrygian => -4,
        FermataMode::Lydian => 1,
        FermataMode::Mixolydian => -1,
        FermataMode::Locrian => -5,
    };

    major_fifths + mode_offset
}

// ============ TIME SIGNATURE ============

/// Compile a time signature S-expression.
///
/// Syntax:
/// - `(time 4 4)` — 4/4 time
/// - `(time 6 8)` — 6/8 time
/// - `(time :common)` — Common time (4/4 with C symbol)
/// - `(time :cut)` — Cut time (2/2 with cut-C symbol)
pub fn compile_time(sexpr: &Sexpr) -> CompileResult<Time> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("time list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("time") {
        return Err(CompileError::UnknownForm("expected (time ...)".to_string()));
    }

    let time_spec = parse_time_form(&list[1..])?;
    compile_time_spec(&time_spec)
}

fn parse_time_form(args: &[Sexpr]) -> CompileResult<TimeSpec> {
    if args.is_empty() {
        return Err(CompileError::MissingField("beats or symbol"));
    }

    // Check for keyword forms
    if let Some(key) = args[0].as_keyword() {
        return match key {
            "common" => Ok(TimeSpec::Common),
            "cut" => Ok(TimeSpec::Cut),
            "senza-misura" | "senza" => Ok(TimeSpec::SenzaMisura),
            _ => Err(CompileError::InvalidTime(format!("unknown time symbol: {}", key))),
        };
    }

    // Numeric form: (time beats beat-type)
    let beats = parse_u8(&args[0])?;

    if args.len() < 2 {
        return Err(CompileError::MissingField("beat-type"));
    }

    let beat_type = parse_u8(&args[1])?;

    Ok(TimeSpec::Simple { beats, beat_type })
}

fn parse_u8(sexpr: &Sexpr) -> CompileResult<u8> {
    match sexpr {
        Sexpr::Symbol(s) => s.parse::<u8>()
            .map_err(|_| CompileError::type_mismatch("integer", s.clone())),
        _ => Err(CompileError::type_mismatch("integer", format!("{:?}", sexpr))),
    }
}

fn compile_time_spec(spec: &TimeSpec) -> CompileResult<Time> {
    match spec {
        TimeSpec::Simple { beats, beat_type } => {
            Ok(Time {
                content: TimeContent::Measured(TimeMeasured {
                    signatures: vec![TimeSignature {
                        beats: beats.to_string(),
                        beat_type: beat_type.to_string(),
                    }],
                    interchangeable: None,
                }),
                symbol: None,
                separator: None,
                ..Default::default()
            })
        }
        TimeSpec::Common => {
            Ok(Time {
                content: TimeContent::Measured(TimeMeasured {
                    signatures: vec![TimeSignature {
                        beats: "4".to_string(),
                        beat_type: "4".to_string(),
                    }],
                    interchangeable: None,
                }),
                symbol: Some(TimeSymbol::Common),
                separator: None,
                ..Default::default()
            })
        }
        TimeSpec::Cut => {
            Ok(Time {
                content: TimeContent::Measured(TimeMeasured {
                    signatures: vec![TimeSignature {
                        beats: "2".to_string(),
                        beat_type: "2".to_string(),
                    }],
                    interchangeable: None,
                }),
                symbol: Some(TimeSymbol::Cut),
                separator: None,
                ..Default::default()
            })
        }
        TimeSpec::SenzaMisura => {
            Ok(Time {
                content: TimeContent::SenzaMisura(None),
                symbol: None,
                separator: None,
                ..Default::default()
            })
        }
    }
}

// ============ CLEF ============

/// Compile a clef S-expression.
///
/// Syntax:
/// - `(clef :treble)` — G clef on line 2
/// - `(clef :bass)` — F clef on line 4
/// - `(clef :alto)` — C clef on line 3
/// - `(clef :tenor)` — C clef on line 4
/// - `(clef :treble-8vb)` — Treble with octave down
pub fn compile_clef(sexpr: &Sexpr) -> CompileResult<Clef> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("clef list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("clef") {
        return Err(CompileError::UnknownForm("expected (clef ...)".to_string()));
    }

    if list.len() < 2 {
        return Err(CompileError::MissingField("clef type"));
    }

    let clef_name = list[1].as_keyword()
        .or_else(|| list[1].as_symbol())
        .ok_or_else(|| CompileError::type_mismatch("clef name", format!("{:?}", list[1])))?;

    let clef_spec = parse_clef_name(clef_name)?;
    compile_clef_spec(&clef_spec)
}

fn parse_clef_name(name: &str) -> CompileResult<ClefSpec> {
    match name.to_lowercase().as_str() {
        "treble" | "g" => Ok(ClefSpec::Treble),
        "bass" | "f" => Ok(ClefSpec::Bass),
        "alto" | "c" => Ok(ClefSpec::Alto),
        "tenor" => Ok(ClefSpec::Tenor),
        "treble-8vb" | "treble8vb" | "g-8vb" => Ok(ClefSpec::Treble8vb),
        "treble-8va" | "treble8va" | "g-8va" => Ok(ClefSpec::Treble8va),
        "bass-8vb" | "bass8vb" | "f-8vb" => Ok(ClefSpec::Bass8vb),
        "bass-8va" | "bass8va" | "f-8va" => Ok(ClefSpec::Bass8va),
        "percussion" | "perc" => Ok(ClefSpec::Percussion),
        "tab" => Ok(ClefSpec::Tab),
        _ => Err(CompileError::InvalidClef(name.to_string())),
    }
}

fn compile_clef_spec(spec: &ClefSpec) -> CompileResult<Clef> {
    let (sign, line, octave_change) = match spec {
        ClefSpec::Treble => (ClefSign::G, Some(2), None),
        ClefSpec::Bass => (ClefSign::F, Some(4), None),
        ClefSpec::Alto => (ClefSign::C, Some(3), None),
        ClefSpec::Tenor => (ClefSign::C, Some(4), None),
        ClefSpec::Treble8vb => (ClefSign::G, Some(2), Some(-1)),
        ClefSpec::Treble8va => (ClefSign::G, Some(2), Some(1)),
        ClefSpec::Bass8vb => (ClefSign::F, Some(4), Some(-1)),
        ClefSpec::Bass8va => (ClefSign::F, Some(4), Some(1)),
        ClefSpec::Percussion => (ClefSign::Percussion, None, None),
        ClefSpec::Tab => (ClefSign::Tab, Some(5), None),
        ClefSpec::Custom { sign, line, octave_change } => {
            let clef_sign = match sign.to_ascii_uppercase() {
                'G' => ClefSign::G,
                'F' => ClefSign::F,
                'C' => ClefSign::C,
                _ => return Err(CompileError::InvalidClef(format!("invalid sign: {}", sign))),
            };
            (clef_sign, Some(*line), *octave_change)
        }
    };

    Ok(Clef {
        sign,
        line,
        octave_change,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_key_c_major() {
        let sexpr = parse("(key c :major)").unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, 0);
            assert_eq!(tk.mode, Some(Mode::Major));
        }
    }

    #[test]
    fn test_key_g_major() {
        let sexpr = parse("(key g :major)").unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, 1);
        }
    }

    #[test]
    fn test_key_a_minor() {
        let sexpr = parse("(key a :minor)").unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, 0); // A minor = C major's relative minor
            assert_eq!(tk.mode, Some(Mode::Minor));
        }
    }

    #[test]
    fn test_time_4_4() {
        let sexpr = parse("(time 4 4)").unwrap();
        let time = compile_time(&sexpr).unwrap();

        if let TimeContent::Measured(tm) = &time.content {
            assert_eq!(tm.signatures[0].beats, "4");
            assert_eq!(tm.signatures[0].beat_type, "4");
        }
    }

    #[test]
    fn test_time_common() {
        let sexpr = parse("(time :common)").unwrap();
        let time = compile_time(&sexpr).unwrap();

        assert_eq!(time.symbol, Some(TimeSymbol::Common));
    }

    #[test]
    fn test_clef_treble() {
        let sexpr = parse("(clef :treble)").unwrap();
        let clef = compile_clef(&sexpr).unwrap();

        assert_eq!(clef.sign, ClefSign::G);
        assert_eq!(clef.line, Some(2));
    }

    #[test]
    fn test_clef_bass() {
        let sexpr = parse("(clef :bass)").unwrap();
        let clef = compile_clef(&sexpr).unwrap();

        assert_eq!(clef.sign, ClefSign::F);
        assert_eq!(clef.line, Some(4));
    }
}
```

---

## Task 2: Direction Compilation (`src/fermata/direction.rs`)

```rust
//! Direction compilation for Fermata syntax.
//!
//! Compiles dynamics, tempo, wedges, and other directions.

use crate::ir::direction::{
    Direction, DirectionType, DirectionTypeContent,
    Dynamics, DynamicsContent,
    Wedge, WedgeType,
    Metronome, MetronomeContent, PerMinute, BeatUnit,
    Words,
};
use crate::ir::common::{StartStop, AboveBelow};
use crate::ir::duration::NoteTypeValue;
use crate::sexpr::Sexpr;
use super::ast::{DynamicMark, TempoMark, FermataDirection, DurationBase};
use super::error::{CompileError, CompileResult};

// ============ DYNAMICS ============

/// Compile a dynamic marking.
///
/// Syntax:
/// - `(ff)` or `(dynamic ff)` — Fortissimo
/// - `(pp)` — Pianissimo
/// - `(mf)` — Mezzo-forte
/// - `(cresc)` or `(crescendo)` — Start crescendo
/// - `(dim)` or `(diminuendo)` — Start diminuendo
pub fn compile_dynamic(sexpr: &Sexpr) -> CompileResult<Direction> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("dynamic list", format!("{:?}", sexpr)))?;

    let head = list.first().and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::UnknownForm("expected dynamic symbol".to_string()))?;

    // Try to parse as direct dynamic
    if let Ok(dynamic) = parse_dynamic_name(head) {
        return compile_dynamic_mark(&dynamic);
    }

    // Handle (dynamic <name>) form
    if head == "dynamic" && list.len() >= 2 {
        let name = list[1].as_symbol()
            .ok_or_else(|| CompileError::type_mismatch("dynamic name", format!("{:?}", list[1])))?;
        let dynamic = parse_dynamic_name(name)?;
        return compile_dynamic_mark(&dynamic);
    }

    Err(CompileError::InvalidDynamic(head.to_string()))
}

fn parse_dynamic_name(name: &str) -> CompileResult<DynamicMark> {
    match name.to_lowercase().as_str() {
        // Standard dynamics
        "pppppp" => Ok(DynamicMark::PPPPPP),
        "ppppp" => Ok(DynamicMark::PPPPP),
        "pppp" => Ok(DynamicMark::PPPP),
        "ppp" => Ok(DynamicMark::PPP),
        "pp" => Ok(DynamicMark::PP),
        "p" => Ok(DynamicMark::P),
        "mp" => Ok(DynamicMark::MP),
        "mf" => Ok(DynamicMark::MF),
        "f" => Ok(DynamicMark::F),
        "ff" => Ok(DynamicMark::FF),
        "fff" => Ok(DynamicMark::FFF),
        "ffff" => Ok(DynamicMark::FFFF),
        "fffff" => Ok(DynamicMark::FFFFF),
        "ffffff" => Ok(DynamicMark::FFFFFF),

        // Combined dynamics
        "fp" => Ok(DynamicMark::FP),
        "sf" => Ok(DynamicMark::SF),
        "sfp" => Ok(DynamicMark::SFP),
        "sfpp" => Ok(DynamicMark::SFPP),
        "sfz" => Ok(DynamicMark::SFZ),
        "sffz" => Ok(DynamicMark::SFFZ),
        "sfzp" => Ok(DynamicMark::SFZP),
        "fz" => Ok(DynamicMark::FZ),
        "pf" => Ok(DynamicMark::PF),
        "rf" | "rfz" => Ok(DynamicMark::RFZ),

        // Niente
        "n" | "niente" => Ok(DynamicMark::N),

        // Wedges
        "cresc" | "crescendo" => Ok(DynamicMark::Crescendo(StartStop::Start)),
        "dim" | "diminuendo" | "decresc" | "decrescendo" =>
            Ok(DynamicMark::Diminuendo(StartStop::Start)),
        "cresc-stop" | "crescendo-stop" => Ok(DynamicMark::Crescendo(StartStop::Stop)),
        "dim-stop" | "diminuendo-stop" | "decresc-stop" =>
            Ok(DynamicMark::Diminuendo(StartStop::Stop)),

        _ => Err(CompileError::InvalidDynamic(name.to_string())),
    }
}

fn compile_dynamic_mark(mark: &DynamicMark) -> CompileResult<Direction> {
    let content = match mark {
        // Wedges
        DynamicMark::Crescendo(action) => {
            DirectionTypeContent::Wedge(Wedge {
                r#type: WedgeType::Crescendo,
                number: None,
                niente: None,
                ..Default::default()
            })
        }
        DynamicMark::Diminuendo(action) => {
            DirectionTypeContent::Wedge(Wedge {
                r#type: WedgeType::Diminuendo,
                number: None,
                niente: None,
                ..Default::default()
            })
        }

        // Standard dynamics
        _ => {
            let dyn_content = match mark {
                DynamicMark::PPPPPP => DynamicsContent::Pppppp,
                DynamicMark::PPPPP => DynamicsContent::Ppppp,
                DynamicMark::PPPP => DynamicsContent::Pppp,
                DynamicMark::PPP => DynamicsContent::Ppp,
                DynamicMark::PP => DynamicsContent::Pp,
                DynamicMark::P => DynamicsContent::P,
                DynamicMark::MP => DynamicsContent::Mp,
                DynamicMark::MF => DynamicsContent::Mf,
                DynamicMark::F => DynamicsContent::F,
                DynamicMark::FF => DynamicsContent::Ff,
                DynamicMark::FFF => DynamicsContent::Fff,
                DynamicMark::FFFF => DynamicsContent::Ffff,
                DynamicMark::FFFFF => DynamicsContent::Fffff,
                DynamicMark::FFFFFF => DynamicsContent::Ffffff,
                DynamicMark::FP => DynamicsContent::Fp,
                DynamicMark::SF => DynamicsContent::Sf,
                DynamicMark::SFP => DynamicsContent::Sfp,
                DynamicMark::SFPP => DynamicsContent::Sfpp,
                DynamicMark::SFZ => DynamicsContent::Sfz,
                DynamicMark::SFFZ => DynamicsContent::Sffz,
                DynamicMark::SFZP => DynamicsContent::Sfzp,
                DynamicMark::FZ => DynamicsContent::Fz,
                DynamicMark::PF => DynamicsContent::Pf,
                DynamicMark::RFZ => DynamicsContent::Rfz,
                DynamicMark::N => DynamicsContent::N,
                _ => return Err(CompileError::InvalidDynamic("unexpected variant".to_string())),
            };

            DirectionTypeContent::Dynamics(Dynamics {
                content: vec![dyn_content],
                ..Default::default()
            })
        }
    };

    Ok(Direction {
        direction_types: vec![DirectionType { content }],
        placement: Some(AboveBelow::Below),
        staff: None,
        voice: None,
        ..Default::default()
    })
}

// ============ TEMPO ============

/// Compile a tempo marking.
///
/// Syntax:
/// - `(tempo :q 120)` — Quarter = 120
/// - `(tempo "Allegro" :q 120)` — "Allegro" with quarter = 120
/// - `(tempo "Adagio")` — Text only
pub fn compile_tempo(sexpr: &Sexpr) -> CompileResult<Direction> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("tempo list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("tempo") {
        return Err(CompileError::UnknownForm("expected (tempo ...)".to_string()));
    }

    let tempo_mark = parse_tempo_form(&list[1..])?;
    compile_tempo_mark(&tempo_mark)
}

fn parse_tempo_form(args: &[Sexpr]) -> CompileResult<TempoMark> {
    let mut text = None;
    let mut beat_unit = None;
    let mut beat_unit_dots = 0u8;
    let mut per_minute = None;

    let mut i = 0;
    while i < args.len() {
        match &args[i] {
            Sexpr::String(s) => {
                text = Some(s.clone());
                i += 1;
            }
            Sexpr::Symbol(s) if s.starts_with(':') => {
                // Duration keyword
                let dur_str = &s[1..];
                let (base, dots) = parse_duration_for_tempo(dur_str)?;
                beat_unit = Some(base);
                beat_unit_dots = dots;
                i += 1;

                // Next should be BPM
                if i < args.len() {
                    if let Some(bpm) = args[i].as_symbol().and_then(|s| s.parse::<u32>().ok()) {
                        per_minute = Some(bpm);
                        i += 1;
                    }
                }
            }
            Sexpr::Symbol(s) => {
                // Could be BPM or text
                if let Ok(bpm) = s.parse::<u32>() {
                    per_minute = Some(bpm);
                } else {
                    text = Some(s.clone());
                }
                i += 1;
            }
            _ => i += 1,
        }
    }

    Ok(TempoMark {
        text,
        beat_unit,
        beat_unit_dots,
        per_minute,
    })
}

fn parse_duration_for_tempo(s: &str) -> CompileResult<(DurationBase, u8)> {
    let dot_count = s.chars().rev().take_while(|&c| c == '.').count() as u8;
    let base_str = &s[..s.len() - dot_count as usize];

    let base = match base_str.to_lowercase().as_str() {
        "w" | "whole" | "1" => DurationBase::Whole,
        "h" | "half" | "2" => DurationBase::Half,
        "q" | "quarter" | "4" => DurationBase::Quarter,
        "e" | "eighth" | "8" => DurationBase::Eighth,
        "s" | "sixteenth" | "16" => DurationBase::Sixteenth,
        _ => return Err(CompileError::InvalidDuration(s.to_string())),
    };

    Ok((base, dot_count))
}

fn compile_tempo_mark(mark: &TempoMark) -> CompileResult<Direction> {
    let mut direction_types = Vec::new();

    // Add text if present
    if let Some(text) = &mark.text {
        direction_types.push(DirectionType {
            content: DirectionTypeContent::Words(Words {
                value: text.clone(),
                ..Default::default()
            }),
        });
    }

    // Add metronome if beat unit is present
    if let Some(beat_unit) = &mark.beat_unit {
        let bu = match beat_unit {
            DurationBase::Whole => BeatUnit::Whole,
            DurationBase::Half => BeatUnit::Half,
            DurationBase::Quarter => BeatUnit::Quarter,
            DurationBase::Eighth => BeatUnit::Eighth,
            DurationBase::Sixteenth => BeatUnit::N16th,
            _ => BeatUnit::Quarter,
        };

        let pm = mark.per_minute.map(|bpm| PerMinute {
            value: bpm.to_string(),
        });

        if let Some(per_minute) = pm {
            direction_types.push(DirectionType {
                content: DirectionTypeContent::Metronome(Metronome {
                    content: MetronomeContent::PerMinute {
                        beat_unit: bu,
                        beat_unit_dots: mark.beat_unit_dots,
                        per_minute,
                    },
                    ..Default::default()
                }),
            });
        }
    }

    Ok(Direction {
        direction_types,
        placement: Some(AboveBelow::Above),
        staff: None,
        voice: None,
        ..Default::default()
    })
}

// ============ OTHER DIRECTIONS ============

/// Compile rehearsal mark, segno, coda, etc.
pub fn compile_direction(sexpr: &Sexpr) -> CompileResult<Direction> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("direction list", format!("{:?}", sexpr)))?;

    let head = list.first().and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::UnknownForm("expected direction head".to_string()))?;

    match head {
        "rehearsal" => compile_rehearsal(&list[1..]),
        "segno" => compile_segno(),
        "coda" => compile_coda(),
        "pedal" => compile_pedal(&list[1..]),
        "words" | "text" => compile_words(&list[1..]),
        _ => Err(CompileError::UnknownForm(head.to_string())),
    }
}

fn compile_rehearsal(args: &[Sexpr]) -> CompileResult<Direction> {
    let text = args.first()
        .and_then(|s| s.as_string().or_else(|| s.as_symbol()))
        .unwrap_or("A")
        .to_string();

    Ok(Direction {
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Rehearsal(vec![
                crate::ir::direction::FormattedText {
                    value: text,
                    ..Default::default()
                }
            ]),
        }],
        placement: Some(AboveBelow::Above),
        ..Default::default()
    })
}

fn compile_segno() -> CompileResult<Direction> {
    Ok(Direction {
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Segno(crate::ir::direction::Segno::default()),
        }],
        placement: Some(AboveBelow::Above),
        ..Default::default()
    })
}

fn compile_coda() -> CompileResult<Direction> {
    Ok(Direction {
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Coda(crate::ir::direction::Coda::default()),
        }],
        placement: Some(AboveBelow::Above),
        ..Default::default()
    })
}

fn compile_pedal(args: &[Sexpr]) -> CompileResult<Direction> {
    use crate::ir::direction::{Pedal, PedalType};

    let action = args.first()
        .and_then(|s| s.as_keyword().or_else(|| s.as_symbol()))
        .map(|s| match s {
            "start" => PedalType::Start,
            "stop" => PedalType::Stop,
            "change" => PedalType::Change,
            _ => PedalType::Start,
        })
        .unwrap_or(PedalType::Start);

    Ok(Direction {
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Pedal(Pedal {
                r#type: action,
                ..Default::default()
            }),
        }],
        placement: Some(AboveBelow::Below),
        ..Default::default()
    })
}

fn compile_words(args: &[Sexpr]) -> CompileResult<Direction> {
    let text = args.first()
        .and_then(|s| s.as_string().or_else(|| s.as_symbol()))
        .ok_or(CompileError::MissingField("text"))?
        .to_string();

    Ok(Direction {
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Words(Words {
                value: text,
                ..Default::default()
            }),
        }],
        placement: None,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_dynamic_ff() {
        let sexpr = parse("(ff)").unwrap();
        let dir = compile_dynamic(&sexpr).unwrap();

        assert!(!dir.direction_types.is_empty());
    }

    #[test]
    fn test_dynamic_crescendo() {
        let sexpr = parse("(cresc)").unwrap();
        let dir = compile_dynamic(&sexpr).unwrap();

        if let DirectionTypeContent::Wedge(w) = &dir.direction_types[0].content {
            assert!(matches!(w.r#type, WedgeType::Crescendo));
        } else {
            panic!("Expected wedge");
        }
    }

    #[test]
    fn test_tempo_quarter_120() {
        let sexpr = parse("(tempo :q 120)").unwrap();
        let dir = compile_tempo(&sexpr).unwrap();

        // Should have metronome
        assert!(!dir.direction_types.is_empty());
    }

    #[test]
    fn test_tempo_with_text() {
        let sexpr = parse(r#"(tempo "Allegro" :q 120)"#).unwrap();
        let dir = compile_tempo(&sexpr).unwrap();

        // Should have words and metronome
        assert!(dir.direction_types.len() >= 2);
    }
}
```

---

## Task 3: Integration Tests

Create `tests/fermata_attributes.rs`:

```rust
//! Integration tests for attributes and directions.

use fermata::fermata::attributes::{compile_key, compile_time, compile_clef};
use fermata::fermata::direction::{compile_dynamic, compile_tempo};
use fermata::ir::attributes::{ClefSign, TimeSymbol, KeyContent};
use fermata::ir::common::Mode;
use fermata::sexpr::parser::parse;

#[test]
fn test_all_major_keys() {
    let keys = [
        ("(key c :major)", 0),
        ("(key g :major)", 1),
        ("(key d :major)", 2),
        ("(key a :major)", 3),
        ("(key e :major)", 4),
        ("(key b :major)", 5),
        ("(key f :major)", -1),
    ];

    for (source, expected_fifths) in keys {
        let sexpr = parse(source).unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, expected_fifths, "Key {} should have {} fifths", source, expected_fifths);
        }
    }
}

#[test]
fn test_minor_keys() {
    let keys = [
        ("(key a :minor)", 0),   // Relative to C major
        ("(key e :minor)", 1),   // Relative to G major
        ("(key d :minor)", -1),  // Relative to F major
    ];

    for (source, expected_fifths) in keys {
        let sexpr = parse(source).unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, expected_fifths);
            assert_eq!(tk.mode, Some(Mode::Minor));
        }
    }
}

#[test]
fn test_time_signatures() {
    let times = [
        "(time 4 4)",
        "(time 3 4)",
        "(time 6 8)",
        "(time 2 2)",
        "(time 12 8)",
        "(time :common)",
        "(time :cut)",
    ];

    for source in times {
        let sexpr = parse(source).unwrap();
        let result = compile_time(&sexpr);
        assert!(result.is_ok(), "Failed to compile: {}", source);
    }
}

#[test]
fn test_clefs() {
    let clefs = [
        ("(clef :treble)", ClefSign::G, Some(2)),
        ("(clef :bass)", ClefSign::F, Some(4)),
        ("(clef :alto)", ClefSign::C, Some(3)),
        ("(clef :tenor)", ClefSign::C, Some(4)),
    ];

    for (source, expected_sign, expected_line) in clefs {
        let sexpr = parse(source).unwrap();
        let clef = compile_clef(&sexpr).unwrap();

        assert_eq!(clef.sign, expected_sign, "Clef {} sign mismatch", source);
        assert_eq!(clef.line, expected_line, "Clef {} line mismatch", source);
    }
}

#[test]
fn test_all_dynamics() {
    let dynamics = [
        "(pppp)", "(ppp)", "(pp)", "(p)",
        "(mp)", "(mf)",
        "(f)", "(ff)", "(fff)", "(ffff)",
        "(sf)", "(sfz)", "(fp)",
        "(cresc)", "(dim)",
    ];

    for source in dynamics {
        let sexpr = parse(source).unwrap();
        let result = compile_dynamic(&sexpr);
        assert!(result.is_ok(), "Failed to compile dynamic: {}", source);
    }
}

#[test]
fn test_tempo_forms() {
    let tempos = [
        "(tempo :q 120)",
        "(tempo :h 60)",
        "(tempo :q. 80)",
        r#"(tempo "Allegro" :q 132)"#,
        r#"(tempo "Adagio")"#,
    ];

    for source in tempos {
        let sexpr = parse(source).unwrap();
        let result = compile_tempo(&sexpr);
        assert!(result.is_ok(), "Failed to compile tempo: {}", source);
    }
}
```

---

## Acceptance Criteria

1. ✅ `(key c :major)` compiles to fifths=0, mode=major
2. ✅ `(key g :major)` compiles to fifths=1
3. ✅ `(key a :minor)` compiles to fifths=0, mode=minor
4. ✅ All modes compile correctly: major, minor, dorian, etc.
5. ✅ `(time 4 4)` compiles to beats="4", beat-type="4"
6. ✅ `(time :common)` includes Common symbol
7. ✅ `(clef :treble)` compiles to G clef on line 2
8. ✅ `(clef :bass)` compiles to F clef on line 4
9. ✅ All dynamics compile: pp, p, mp, mf, f, ff, sf, etc.
10. ✅ Wedges compile: cresc, dim
11. ✅ `(tempo :q 120)` compiles to metronome
12. ✅ All tests pass

---

## Implementation Notes

1. **Key signature math** — The fifths value is computed from root + mode offset

2. **Mode strings** — Accept both keyword (`:major`) and symbol (`major`) forms

3. **Time symbol** — Common and cut time set both the signature AND the symbol attribute

4. **Clef variants** — Support common names and octave-transposing variants

5. **Dynamic placement** — Default to `below` for dynamics, `above` for tempo

6. **Missing IR types** — You may need to add `Default` impls to various IR types

---

*Next: Milestone 5 — Score Assembly & CLI*
