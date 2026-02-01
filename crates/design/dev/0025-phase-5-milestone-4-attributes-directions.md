# Phase 5 — Milestone 4: Attributes & Directions

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Key signatures, time signatures, clefs, dynamics, tempo, articulations
> **Depends On:** Milestone 2 (Core Elements)

---

## Overview

This milestone implements musical attributes and directions:

- **Key signatures** — `(key c :major)`, `(key g :major)`, `(key f :minor)`, `(key f# :major)`
- **Time signatures** — `(time 4 4)`, `(time 6 8)`, `(time :common)`
- **Clefs** — `(clef :treble)`, `(clef :bass)`, `(clef :alto)`
- **Dynamics** — `(ff)`, `(pp)`, `(cresc)`, `(dim)`
- **Tempo** — `(tempo :q 120)`, `(tempo "Allegro" :q 120)`
- **Articulations** — `(staccato)`, `(accent)`, `(fermata)`

---

## Task 1: Key Signature Compilation (`src/lang/attributes.rs`)

```rust
//! Attribute compilation for Fermata syntax.
//!
//! Compiles key signatures, time signatures, clefs, and transposes.

use crate::ir::attributes::{
    Key, KeyContent, TraditionalKey,
    Time, TimeContent, TimeSignature, TimeSymbol,
    Clef, ClefSign,
};
use crate::ir::common::{Mode, YesNo};
use crate::sexpr::Sexpr;
use super::ast::{KeySpec, TimeSpec, ClefSpec, PitchStep, PitchAlter, Mode as FermataMode};
use super::error::{CompileError, CompileResult};

// ============ KEY SIGNATURE ============

/// Compile a key signature S-expression.
///
/// Syntax:
/// - `(key c :major)` — C major (0 fifths)
/// - `(key g :major)` — G major (1 fifth)
/// - `(key f# :major)` — F# major (6 sharps)
/// - `(key bb :minor)` — Bb minor (5 flats)
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

    // First arg: root note (c, g, d, f#, bb, etc.)
    let root_str = args[0].as_symbol()
        .ok_or_else(|| CompileError::type_mismatch("root note", format!("{:?}", args[0])))?;

    let (root, root_alter) = parse_key_root(root_str)?;

    // Second arg: mode (required)
    if args.len() < 2 {
        return Err(CompileError::MissingField("mode"));
    }

    let mode_str = args[1].as_keyword()
        .or_else(|| args[1].as_symbol())
        .ok_or_else(|| CompileError::type_mismatch("mode", format!("{:?}", args[1])))?;

    let mode = parse_mode(mode_str)?;

    Ok(KeySpec { root, root_alter, mode })
}

/// Parse key root with optional accidental (e.g., "f#", "bb", "c")
fn parse_key_root(s: &str) -> CompileResult<(PitchStep, Option<PitchAlter>)> {
    let s = s.to_lowercase();
    let mut chars = s.chars();

    let step = match chars.next() {
        Some('c') => PitchStep::C,
        Some('d') => PitchStep::D,
        Some('e') => PitchStep::E,
        Some('f') => PitchStep::F,
        Some('g') => PitchStep::G,
        Some('a') => PitchStep::A,
        Some('b') => PitchStep::B,
        _ => return Err(CompileError::InvalidKey(format!("invalid root: {}", s))),
    };

    // Check for accidental
    let alter = match chars.next() {
        Some('#') => Some(PitchAlter::Sharp),
        Some('b') => Some(PitchAlter::Flat),
        Some('x') => Some(PitchAlter::DoubleSharp),
        None => None,
        Some(c) => return Err(CompileError::InvalidKey(format!("invalid accidental '{}' in root: {}", c, s))),
    };

    Ok((step, alter))
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
    let fifths = compute_fifths(spec.root, spec.root_alter.as_ref(), &spec.mode);
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
/// | F#   | 6     | 3     | 4      |
/// | Bb   | -2    | -5    | -4     |
fn compute_fifths(root: PitchStep, root_alter: Option<&PitchAlter>, mode: &FermataMode) -> i8 {
    // Major key fifths for natural roots
    let base_fifths = match root {
        PitchStep::C => 0,
        PitchStep::G => 1,
        PitchStep::D => 2,
        PitchStep::A => 3,
        PitchStep::E => 4,
        PitchStep::B => 5,
        PitchStep::F => -1,
    };

    // Adjust for root accidental (each sharp adds 7, each flat subtracts 7)
    let alter_offset = match root_alter {
        Some(PitchAlter::Sharp) => 7,
        Some(PitchAlter::DoubleSharp) => 14,
        Some(PitchAlter::Flat) => -7,
        Some(PitchAlter::DoubleFlat) => -14,
        _ => 0,
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

    base_fifths + alter_offset + mode_offset
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

/// Parse u8 from S-expression (handles both Symbol and Integer)
fn parse_u8(sexpr: &Sexpr) -> CompileResult<u8> {
    match sexpr {
        Sexpr::Integer(n) if *n >= 0 && *n <= 255 => Ok(*n as u8),
        Sexpr::Symbol(s) => s.parse::<u8>()
            .map_err(|_| CompileError::type_mismatch("integer", s.clone())),
        _ => Err(CompileError::type_mismatch("integer", format!("{:?}", sexpr))),
    }
}

fn compile_time_spec(spec: &TimeSpec) -> CompileResult<Time> {
    match spec {
        TimeSpec::Simple { beats, beat_type } => {
            // TimeContent::Measured is a struct variant, not a tuple variant
            Ok(Time {
                content: TimeContent::Measured {
                    signatures: vec![TimeSignature {
                        beats: beats.to_string(),
                        beat_type: beat_type.to_string(),
                    }],
                },
                number: None,
                symbol: None,
                print_object: None,
            })
        }
        TimeSpec::Common => {
            Ok(Time {
                content: TimeContent::Measured {
                    signatures: vec![TimeSignature {
                        beats: "4".to_string(),
                        beat_type: "4".to_string(),
                    }],
                },
                number: None,
                symbol: Some(TimeSymbol::Common),
                print_object: None,
            })
        }
        TimeSpec::Cut => {
            Ok(Time {
                content: TimeContent::Measured {
                    signatures: vec![TimeSignature {
                        beats: "2".to_string(),
                        beat_type: "2".to_string(),
                    }],
                },
                number: None,
                symbol: Some(TimeSymbol::Cut),
                print_object: None,
            })
        }
        TimeSpec::SenzaMisura => {
            // SenzaMisura takes a String, not Option<String>
            Ok(Time {
                content: TimeContent::SenzaMisura(String::new()),
                number: None,
                symbol: None,
                print_object: None,
            })
        }
        TimeSpec::Compound { signatures } => {
            // Compound time signatures like 2/4 + 3/8
            Ok(Time {
                content: TimeContent::Measured {
                    signatures: signatures.iter()
                        .map(|(beats, beat_type)| TimeSignature {
                            beats: beats.to_string(),
                            beat_type: beat_type.to_string(),
                        })
                        .collect(),
                },
                number: None,
                symbol: None,
                print_object: None,
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
        number: None,
        size: None,
        print_object: None,
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
    fn test_key_f_sharp_major() {
        let sexpr = parse("(key f# :major)").unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, 6); // F# major = 6 sharps
            assert_eq!(tk.mode, Some(Mode::Major));
        }
    }

    #[test]
    fn test_key_bb_minor() {
        let sexpr = parse("(key bb :minor)").unwrap();
        let key = compile_key(&sexpr).unwrap();

        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, -5); // Bb minor = 5 flats
            assert_eq!(tk.mode, Some(Mode::Minor));
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

        if let TimeContent::Measured { signatures } = &time.content {
            assert_eq!(signatures[0].beats, "4");
            assert_eq!(signatures[0].beat_type, "4");
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

## Task 2: Direction Compilation (`src/lang/direction.rs`)

```rust
//! Direction compilation for Fermata syntax.
//!
//! Compiles dynamics, tempo, wedges, and other directions.

use crate::ir::direction::{
    Direction, DirectionType, DirectionTypeContent,
    Dynamics, DynamicElement,
    Wedge, WedgeType,
    Metronome, MetronomeContent, PerMinute,
    Words, Segno, Coda, Pedal, PedalType,
    FormattedText,
};
use crate::ir::duration::NoteTypeValue;
use crate::ir::common::{AboveBelow, Position, PrintStyle, YesNo};
use crate::sexpr::Sexpr;
use super::ast::{DynamicMark, TempoMark, DurationBase};
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

        // Wedges (cresc/dim)
        "cresc" | "crescendo" => Ok(DynamicMark::Crescendo),
        "dim" | "diminuendo" | "decresc" | "decrescendo" => Ok(DynamicMark::Diminuendo),

        _ => Err(CompileError::InvalidDynamic(name.to_string())),
    }
}

fn compile_dynamic_mark(mark: &DynamicMark) -> CompileResult<Direction> {
    let content = match mark {
        // Wedges become DirectionTypeContent::Wedge
        DynamicMark::Crescendo => {
            DirectionTypeContent::Wedge(Wedge {
                r#type: WedgeType::Crescendo,
                number: None,
                spread: None,
                niente: None,
                line_type: None,
                position: Position::default(),
                color: None,
            })
        }
        DynamicMark::Diminuendo => {
            DirectionTypeContent::Wedge(Wedge {
                r#type: WedgeType::Diminuendo,
                number: None,
                spread: None,
                niente: None,
                line_type: None,
                position: Position::default(),
                color: None,
            })
        }

        // Standard dynamics become DirectionTypeContent::Dynamics
        // Note: DynamicElement, not DynamicsContent
        _ => {
            let dyn_element = match mark {
                DynamicMark::PPPPPP => DynamicElement::PPPPPP,
                DynamicMark::PPPPP => DynamicElement::PPPPP,
                DynamicMark::PPPP => DynamicElement::PPPP,
                DynamicMark::PPP => DynamicElement::PPP,
                DynamicMark::PP => DynamicElement::PP,
                DynamicMark::P => DynamicElement::P,
                DynamicMark::MP => DynamicElement::MP,
                DynamicMark::MF => DynamicElement::MF,
                DynamicMark::F => DynamicElement::F,
                DynamicMark::FF => DynamicElement::FF,
                DynamicMark::FFF => DynamicElement::FFF,
                DynamicMark::FFFF => DynamicElement::FFFF,
                DynamicMark::FFFFF => DynamicElement::FFFFF,
                DynamicMark::FFFFFF => DynamicElement::FFFFFF,
                DynamicMark::FP => DynamicElement::FP,
                DynamicMark::SF => DynamicElement::SF,
                DynamicMark::SFP => DynamicElement::SFP,
                DynamicMark::SFPP => DynamicElement::SFPP,
                DynamicMark::SFZ => DynamicElement::SFZ,
                DynamicMark::SFFZ => DynamicElement::SFFZ,
                DynamicMark::SFZP => DynamicElement::SFZP,
                DynamicMark::FZ => DynamicElement::FZ,
                DynamicMark::PF => DynamicElement::PF,
                DynamicMark::RFZ => DynamicElement::RFZ,
                DynamicMark::N => DynamicElement::N,
                _ => return Err(CompileError::InvalidDynamic("unexpected variant".to_string())),
            };

            // Dynamics has print_style field
            DirectionTypeContent::Dynamics(Dynamics {
                content: vec![dyn_element],
                print_style: PrintStyle::default(),
                placement: Some(AboveBelow::Below),
            })
        }
    };

    Ok(Direction {
        placement: Some(AboveBelow::Below),
        directive: None,
        direction_types: vec![DirectionType { content }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
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
    let mut beat_unit_dots = 0u32;  // Note: u32 not u8
    let mut per_minute = None;

    let mut i = 0;
    while i < args.len() {
        match &args[i] {
            Sexpr::String(s) => {
                text = Some(s.clone());
                i += 1;
            }
            Sexpr::Keyword(k) => {
                // Duration keyword (without leading :)
                let (base, dots) = parse_duration_for_tempo(k)?;
                beat_unit = Some(base);
                beat_unit_dots = dots as u32;
                i += 1;

                // Next should be BPM
                if i < args.len() {
                    if let Some(bpm) = parse_bpm(&args[i]) {
                        per_minute = Some(bpm);
                        i += 1;
                    }
                }
            }
            Sexpr::Symbol(s) if s.starts_with(':') => {
                // Duration keyword with : prefix
                let dur_str = &s[1..];
                let (base, dots) = parse_duration_for_tempo(dur_str)?;
                beat_unit = Some(base);
                beat_unit_dots = dots as u32;
                i += 1;

                // Next should be BPM
                if i < args.len() {
                    if let Some(bpm) = parse_bpm(&args[i]) {
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
            Sexpr::Integer(n) => {
                per_minute = Some(*n as u32);
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

fn parse_bpm(sexpr: &Sexpr) -> Option<u32> {
    match sexpr {
        Sexpr::Integer(n) if *n > 0 => Some(*n as u32),
        Sexpr::Symbol(s) => s.parse::<u32>().ok(),
        _ => None,
    }
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
    // Note: DirectionTypeContent::Words takes Vec<Words>
    if let Some(text) = &mark.text {
        direction_types.push(DirectionType {
            content: DirectionTypeContent::Words(vec![Words {
                value: text.clone(),
                print_style: PrintStyle::default(),
                justify: None,
                lang: None,
            }]),
        });
    }

    // Add metronome if beat unit is present
    if let Some(beat_unit) = &mark.beat_unit {
        // Use NoteTypeValue directly - there is no BeatUnit enum
        let bu = match beat_unit {
            DurationBase::Whole => NoteTypeValue::Whole,
            DurationBase::Half => NoteTypeValue::Half,
            DurationBase::Quarter => NoteTypeValue::Quarter,
            DurationBase::Eighth => NoteTypeValue::Eighth,
            DurationBase::Sixteenth => NoteTypeValue::N16th,
            _ => NoteTypeValue::Quarter,
        };

        if let Some(bpm) = mark.per_minute {
            let per_minute = PerMinute {
                value: bpm.to_string(),
            };

            direction_types.push(DirectionType {
                content: DirectionTypeContent::Metronome(Metronome {
                    parentheses: None,
                    content: MetronomeContent::PerMinute {
                        beat_unit: bu,
                        beat_unit_dots: mark.beat_unit_dots,  // Already u32
                        per_minute,
                    },
                    print_style: PrintStyle::default(),
                }),
            });
        }
    }

    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types,
        offset: None,
        voice: None,
        staff: None,
        sound: None,
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

    // DirectionTypeContent::Rehearsal takes Vec<FormattedText>
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Rehearsal(vec![FormattedText {
                value: text,
                print_style: PrintStyle::default(),
                enclosure: None,
                lang: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

fn compile_segno() -> CompileResult<Direction> {
    // DirectionTypeContent::Segno takes Vec<Segno>
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Segno(vec![Segno {
                print_style: PrintStyle::default(),
                smufl: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

fn compile_coda() -> CompileResult<Direction> {
    // DirectionTypeContent::Coda takes Vec<Coda>
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Coda(vec![Coda {
                print_style: PrintStyle::default(),
                smufl: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

fn compile_pedal(args: &[Sexpr]) -> CompileResult<Direction> {
    let action = args.first()
        .and_then(|s| s.as_keyword().or_else(|| s.as_symbol()))
        .map(|s| match s {
            "start" => PedalType::Start,
            "stop" => PedalType::Stop,
            "change" => PedalType::Change,
            "continue" => PedalType::Continue,
            _ => PedalType::Start,
        })
        .unwrap_or(PedalType::Start);

    Ok(Direction {
        placement: Some(AboveBelow::Below),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Pedal(Pedal {
                r#type: action,
                number: None,
                line: None,
                sign: None,
                abbreviated: None,
                print_style: PrintStyle::default(),
            }),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

fn compile_words(args: &[Sexpr]) -> CompileResult<Direction> {
    let text = args.first()
        .and_then(|s| s.as_string().or_else(|| s.as_symbol()))
        .ok_or(CompileError::MissingField("text"))?
        .to_string();

    // DirectionTypeContent::Words takes Vec<Words>
    Ok(Direction {
        placement: None,
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Words(vec![Words {
                value: text,
                print_style: PrintStyle::default(),
                justify: None,
                lang: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
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

    #[test]
    fn test_segno() {
        let sexpr = parse("(segno)").unwrap();
        let dir = compile_direction(&sexpr).unwrap();

        assert!(!dir.direction_types.is_empty());
        assert!(matches!(
            &dir.direction_types[0].content,
            DirectionTypeContent::Segno(_)
        ));
    }

    #[test]
    fn test_coda() {
        let sexpr = parse("(coda)").unwrap();
        let dir = compile_direction(&sexpr).unwrap();

        assert!(!dir.direction_types.is_empty());
        assert!(matches!(
            &dir.direction_types[0].content,
            DirectionTypeContent::Coda(_)
        ));
    }

    #[test]
    fn test_pedal_start() {
        let sexpr = parse("(pedal :start)").unwrap();
        let dir = compile_direction(&sexpr).unwrap();

        if let DirectionTypeContent::Pedal(p) = &dir.direction_types[0].content {
            assert!(matches!(p.r#type, PedalType::Start));
        } else {
            panic!("Expected pedal");
        }
    }
}
```

---

## Task 3: Integration Tests (inline)

Add comprehensive tests to `src/lang/attributes.rs`:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::sexpr::parser::parse;
    use crate::ir::attributes::{ClefSign, TimeSymbol, KeyContent};
    use crate::ir::common::Mode;

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
            ("(key f# :major)", 6),   // F# major
            ("(key c# :major)", 7),   // C# major
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
    fn test_flat_keys() {
        let keys = [
            ("(key bb :major)", -2),  // Bb major = 2 flats
            ("(key eb :major)", -3),  // Eb major = 3 flats
            ("(key ab :major)", -4),  // Ab major = 4 flats
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
            let result = crate::lang::direction::compile_dynamic(&sexpr);
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
            let result = crate::lang::direction::compile_tempo(&sexpr);
            assert!(result.is_ok(), "Failed to compile tempo: {}", source);
        }
    }
}
```

---

## Acceptance Criteria

1. ✅ `(key c :major)` compiles to fifths=0, mode=major
2. ✅ `(key g :major)` compiles to fifths=1
3. ✅ `(key f# :major)` compiles to fifths=6
4. ✅ `(key bb :minor)` compiles to fifths=-5, mode=minor
5. ✅ `(key a :minor)` compiles to fifths=0, mode=minor
6. ✅ All modes compile correctly: major, minor, dorian, etc.
7. ✅ `(time 4 4)` compiles to beats="4", beat-type="4"
8. ✅ `(time :common)` includes Common symbol
9. ✅ `(clef :treble)` compiles to G clef on line 2
10. ✅ `(clef :bass)` compiles to F clef on line 4
11. ✅ All dynamics compile: pp, p, mp, mf, f, ff, sf, etc.
12. ✅ Wedges compile: cresc, dim
13. ✅ `(tempo :q 120)` compiles to metronome
14. ✅ All tests pass

---

## Implementation Notes

1. **Key signature math** — The fifths value is computed from root + root_alter + mode offset. Sharps add 7, flats subtract 7.

2. **KeySpec includes `root_alter`** — For keys like F# major and Bb minor, the `root_alter` field captures the accidental.

3. **Mode strings** — Accept both keyword (`:major`) and symbol (`major`) forms

4. **Time struct** — Has NO `separator` field. `TimeContent::Measured` is a struct variant, not a tuple with `TimeMeasured` struct.

5. **TimeContent::SenzaMisura** — Takes `String`, not `Option<String>`

6. **Clef variants** — Support common names and octave-transposing variants

7. **Dynamic placement** — Default to `below` for dynamics, `above` for tempo

8. **DynamicElement not DynamicsContent** — The IR uses `DynamicElement` enum with UPPERCASE variants (e.g., `DynamicElement::FF`)

9. **Dynamics.print_style** — Required field, use `PrintStyle::default()`

10. **DirectionTypeContent variants take Vec** — `Words(Vec<Words>)`, `Rehearsal(Vec<FormattedText>)`, `Segno(Vec<Segno>)`, `Coda(Vec<Coda>)`

11. **No BeatUnit type** — Use `NoteTypeValue` directly for metronome beat units

12. **beat_unit_dots is u32** — Not u8

13. **All Direction structs have print_style** — `Words`, `Metronome`, `Pedal`, `Segno`, `Coda` all need `print_style: PrintStyle::default()`

14. **Wedge has position field** — And `spread`, `line_type`, `color` fields

15. **parse_u8 handles Integer** — Must handle both `Sexpr::Integer` and `Sexpr::Symbol`

---

*Next: Milestone 5 — Score Assembly & CLI*
