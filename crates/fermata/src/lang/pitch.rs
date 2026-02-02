//! Pitch parsing and compilation.
//!
//! This module provides functions for parsing pitch strings (e.g., "c4", "f#5", "bb3")
//! and compiling them to the IR representation.

use crate::ir::common::Semitones;
use crate::ir::pitch::{Pitch as IrPitch, Step as IrStep};
use crate::lang::ast::{FermataPitch, PitchAlter, PitchStep};
use crate::lang::error::{CompileError, CompileResult};
use crate::sexpr::Sexpr;

/// Parse a pitch string directly to IR Pitch.
///
/// Supported formats:
/// - `c4` - C natural in octave 4
/// - `f#5` - F sharp in octave 5
/// - `bb3` - B flat in octave 3
/// - `cn4` - C natural (explicit) in octave 4
/// - `c+4` - C quarter-sharp in octave 4
/// - `cd4` - C quarter-flat in octave 4
/// - `cx4` - C double-sharp in octave 4
///
/// # Examples
///
/// ```
/// use fermata::lang::pitch::parse_pitch;
///
/// let pitch = parse_pitch("c4").unwrap();
/// assert_eq!(pitch.octave, 4);
/// ```
pub fn parse_pitch(s: &str) -> CompileResult<IrPitch> {
    let fermata_pitch = parse_pitch_str(s)?;
    compile_pitch(&fermata_pitch)
}

/// Parse a pitch string to a FermataPitch AST node.
///
/// See `parse_pitch` for supported formats.
pub fn parse_pitch_str(s: &str) -> CompileResult<FermataPitch> {
    let s = s.trim();
    if s.is_empty() {
        return Err(CompileError::InvalidPitch("empty pitch string".to_string()));
    }

    let mut chars = s.chars().peekable();

    // Parse the step (first character)
    let step_char = chars
        .next()
        .ok_or_else(|| CompileError::InvalidPitch("expected pitch letter".to_string()))?;
    let step = parse_step(step_char)?;

    // Collect remaining characters to analyze
    let remaining: String = chars.collect();

    if remaining.is_empty() {
        return Err(CompileError::InvalidPitch(format!(
            "missing octave in pitch '{}'",
            s
        )));
    }

    // Find where the octave number starts (first digit)
    let octave_pos = remaining.chars().position(|c| c.is_ascii_digit());

    if octave_pos.is_none() {
        return Err(CompileError::InvalidPitch(format!(
            "missing octave number in pitch '{}'",
            s
        )));
    }

    let octave_pos = octave_pos.unwrap();
    let alter_str = &remaining[..octave_pos];
    let octave_str = &remaining[octave_pos..];

    // Parse alteration
    let alter = if alter_str.is_empty() {
        None
    } else {
        Some(parse_alter(alter_str)?)
    };

    // Parse octave
    let octave: u8 = octave_str.parse().map_err(|_| {
        CompileError::InvalidPitch(format!("invalid octave '{}' in pitch '{}'", octave_str, s))
    })?;

    // Validate octave range (0-9 is the MusicXML standard)
    if octave > 9 {
        return Err(CompileError::InvalidPitch(format!(
            "octave {} out of range (0-9) in pitch '{}'",
            octave, s
        )));
    }

    Ok(FermataPitch {
        step,
        alter,
        octave,
    })
}

/// Parse a single character to a PitchStep.
pub fn parse_step(c: char) -> CompileResult<PitchStep> {
    match c.to_ascii_lowercase() {
        'c' => Ok(PitchStep::C),
        'd' => Ok(PitchStep::D),
        'e' => Ok(PitchStep::E),
        'f' => Ok(PitchStep::F),
        'g' => Ok(PitchStep::G),
        'a' => Ok(PitchStep::A),
        'b' => Ok(PitchStep::B),
        _ => Err(CompileError::InvalidPitch(format!(
            "invalid pitch letter '{}', expected a-g",
            c
        ))),
    }
}

/// Parse an alteration string to a PitchAlter.
fn parse_alter(s: &str) -> CompileResult<PitchAlter> {
    match s {
        "#" | "s" => Ok(PitchAlter::Sharp),
        "b" => Ok(PitchAlter::Flat),
        "##" | "x" => Ok(PitchAlter::DoubleSharp),
        "bb" => Ok(PitchAlter::DoubleFlat),
        "n" => Ok(PitchAlter::Natural),
        "+" => Ok(PitchAlter::QuarterSharp),
        "d" => Ok(PitchAlter::QuarterFlat),
        "+#" | "#+" => Ok(PitchAlter::ThreeQuarterSharp),
        "db" | "bd" => Ok(PitchAlter::ThreeQuarterFlat),
        _ => Err(CompileError::InvalidPitch(format!(
            "invalid alteration '{}', expected #, b, ##, x, bb, n, +, or d",
            s
        ))),
    }
}

/// Compile a FermataPitch to an IR Pitch.
pub fn compile_pitch(pitch: &FermataPitch) -> CompileResult<IrPitch> {
    let step = compile_step(&pitch.step);
    let alter: Option<Semitones> = pitch.alter.as_ref().map(|a| a.to_semitones());
    let octave = pitch.octave;

    Ok(IrPitch {
        step,
        alter,
        octave,
    })
}

/// Compile a PitchStep to an IR Step.
fn compile_step(step: &PitchStep) -> IrStep {
    match step {
        PitchStep::C => IrStep::C,
        PitchStep::D => IrStep::D,
        PitchStep::E => IrStep::E,
        PitchStep::F => IrStep::F,
        PitchStep::G => IrStep::G,
        PitchStep::A => IrStep::A,
        PitchStep::B => IrStep::B,
    }
}

/// Parse a pitch from an S-expression.
///
/// Supported formats:
/// - `(pitch :step C :octave 4)` - C4
/// - `(pitch :step F :alter 1 :octave 5)` - F#5
/// - `(pitch :step B :alter -1 :octave 3)` - Bb3
/// - `c4` (symbol) - Shorthand form
pub fn parse_pitch_sexpr(sexpr: &Sexpr) -> CompileResult<FermataPitch> {
    match sexpr {
        // Shorthand: symbol like "c4" or "f#5"
        Sexpr::Symbol(s) => parse_pitch_str(s),

        // Full form: (pitch :step C :octave 4 [:alter N])
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidPitch("empty pitch list".to_string()));
            }

            // Check for 'pitch' head
            if !items[0].is_symbol("pitch") {
                return Err(CompileError::InvalidPitch(format!(
                    "expected 'pitch', got {:?}",
                    items[0]
                )));
            }

            let mut step: Option<PitchStep> = None;
            let mut alter: Option<PitchAlter> = None;
            let mut octave: Option<u8> = None;

            // Parse keyword arguments
            let mut i = 1;
            while i < items.len() {
                if let Some(kw) = items[i].as_keyword() {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidPitch(format!(
                            "missing value for keyword :{}",
                            kw
                        )));
                    }

                    match kw {
                        "step" => {
                            step = Some(parse_step_sexpr(&items[i + 1])?);
                        }
                        "alter" => {
                            alter = Some(parse_alter_sexpr(&items[i + 1])?);
                        }
                        "octave" => {
                            octave = Some(parse_octave_sexpr(&items[i + 1])?);
                        }
                        _ => {
                            return Err(CompileError::InvalidPitch(format!(
                                "unknown pitch keyword :{}",
                                kw
                            )));
                        }
                    }
                    i += 2;
                } else {
                    return Err(CompileError::InvalidPitch(format!(
                        "expected keyword, got {:?}",
                        items[i]
                    )));
                }
            }

            // Validate required fields
            let step = step
                .ok_or_else(|| CompileError::InvalidPitch("missing :step in pitch".to_string()))?;
            let octave = octave.ok_or_else(|| {
                CompileError::InvalidPitch("missing :octave in pitch".to_string())
            })?;

            Ok(FermataPitch {
                step,
                alter,
                octave,
            })
        }

        _ => Err(CompileError::InvalidPitch(format!(
            "expected pitch symbol or list, got {:?}",
            sexpr
        ))),
    }
}

/// Parse a step from an S-expression (symbol like "C", "D", etc.).
fn parse_step_sexpr(sexpr: &Sexpr) -> CompileResult<PitchStep> {
    match sexpr {
        Sexpr::Symbol(s) => {
            if s.len() != 1 {
                return Err(CompileError::InvalidPitch(format!(
                    "step must be single letter, got '{}'",
                    s
                )));
            }
            parse_step(s.chars().next().unwrap())
        }
        _ => Err(CompileError::InvalidPitch(format!(
            "expected step symbol, got {:?}",
            sexpr
        ))),
    }
}

/// Parse an alteration from an S-expression (number or symbol).
fn parse_alter_sexpr(sexpr: &Sexpr) -> CompileResult<PitchAlter> {
    match sexpr {
        // Numeric form: 1 = sharp, -1 = flat, etc.
        Sexpr::Integer(n) => match *n {
            -2 => Ok(PitchAlter::DoubleFlat),
            -1 => Ok(PitchAlter::Flat),
            0 => Ok(PitchAlter::Natural),
            1 => Ok(PitchAlter::Sharp),
            2 => Ok(PitchAlter::DoubleSharp),
            _ => Err(CompileError::InvalidPitch(format!(
                "invalid alteration value {}, expected -2 to 2",
                n
            ))),
        },

        // Float form for microtones
        Sexpr::Float(f) => {
            let semitones = *f;
            if (semitones - (-2.0)).abs() < 0.01 {
                Ok(PitchAlter::DoubleFlat)
            } else if (semitones - (-1.5)).abs() < 0.01 {
                Ok(PitchAlter::ThreeQuarterFlat)
            } else if (semitones - (-1.0)).abs() < 0.01 {
                Ok(PitchAlter::Flat)
            } else if (semitones - (-0.5)).abs() < 0.01 {
                Ok(PitchAlter::QuarterFlat)
            } else if semitones.abs() < 0.01 {
                Ok(PitchAlter::Natural)
            } else if (semitones - 0.5).abs() < 0.01 {
                Ok(PitchAlter::QuarterSharp)
            } else if (semitones - 1.0).abs() < 0.01 {
                Ok(PitchAlter::Sharp)
            } else if (semitones - 1.5).abs() < 0.01 {
                Ok(PitchAlter::ThreeQuarterSharp)
            } else if (semitones - 2.0).abs() < 0.01 {
                Ok(PitchAlter::DoubleSharp)
            } else {
                Err(CompileError::InvalidPitch(format!(
                    "unsupported microtone alteration {}",
                    semitones
                )))
            }
        }

        // Symbol form: sharp, flat, natural, etc.
        Sexpr::Symbol(s) => match s.as_str() {
            "sharp" | "#" => Ok(PitchAlter::Sharp),
            "flat" | "b" => Ok(PitchAlter::Flat),
            "double-sharp" | "x" | "##" => Ok(PitchAlter::DoubleSharp),
            "double-flat" | "bb" => Ok(PitchAlter::DoubleFlat),
            "natural" | "n" => Ok(PitchAlter::Natural),
            "quarter-sharp" | "+" => Ok(PitchAlter::QuarterSharp),
            "quarter-flat" | "d" => Ok(PitchAlter::QuarterFlat),
            "three-quarter-sharp" => Ok(PitchAlter::ThreeQuarterSharp),
            "three-quarter-flat" => Ok(PitchAlter::ThreeQuarterFlat),
            _ => Err(CompileError::InvalidPitch(format!(
                "unknown alteration symbol '{}'",
                s
            ))),
        },

        _ => Err(CompileError::InvalidPitch(format!(
            "expected alteration number or symbol, got {:?}",
            sexpr
        ))),
    }
}

/// Parse an octave from an S-expression.
fn parse_octave_sexpr(sexpr: &Sexpr) -> CompileResult<u8> {
    match sexpr {
        Sexpr::Integer(n) => {
            if *n < 0 || *n > 9 {
                return Err(CompileError::InvalidPitch(format!(
                    "octave {} out of range (0-9)",
                    n
                )));
            }
            Ok(*n as u8)
        }
        _ => Err(CompileError::InvalidPitch(format!(
            "expected octave integer, got {:?}",
            sexpr
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_step tests ===

    #[test]
    fn test_parse_step_c() {
        assert_eq!(parse_step('c').unwrap(), PitchStep::C);
        assert_eq!(parse_step('C').unwrap(), PitchStep::C);
    }

    #[test]
    fn test_parse_step_d() {
        assert_eq!(parse_step('d').unwrap(), PitchStep::D);
        assert_eq!(parse_step('D').unwrap(), PitchStep::D);
    }

    #[test]
    fn test_parse_step_e() {
        assert_eq!(parse_step('e').unwrap(), PitchStep::E);
    }

    #[test]
    fn test_parse_step_f() {
        assert_eq!(parse_step('f').unwrap(), PitchStep::F);
    }

    #[test]
    fn test_parse_step_g() {
        assert_eq!(parse_step('g').unwrap(), PitchStep::G);
    }

    #[test]
    fn test_parse_step_a() {
        assert_eq!(parse_step('a').unwrap(), PitchStep::A);
    }

    #[test]
    fn test_parse_step_b() {
        assert_eq!(parse_step('b').unwrap(), PitchStep::B);
    }

    #[test]
    fn test_parse_step_invalid() {
        assert!(parse_step('x').is_err());
        assert!(parse_step('h').is_err());
        assert!(parse_step('1').is_err());
    }

    // === parse_pitch_str tests ===

    #[test]
    fn test_parse_pitch_str_natural() {
        let pitch = parse_pitch_str("c4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, None);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_sharp() {
        let pitch = parse_pitch_str("f#5").unwrap();
        assert_eq!(pitch.step, PitchStep::F);
        assert_eq!(pitch.alter, Some(PitchAlter::Sharp));
        assert_eq!(pitch.octave, 5);
    }

    #[test]
    fn test_parse_pitch_str_flat() {
        let pitch = parse_pitch_str("bb3").unwrap();
        assert_eq!(pitch.step, PitchStep::B);
        assert_eq!(pitch.alter, Some(PitchAlter::Flat));
        assert_eq!(pitch.octave, 3);
    }

    #[test]
    fn test_parse_pitch_str_explicit_natural() {
        let pitch = parse_pitch_str("cn4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, Some(PitchAlter::Natural));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_quarter_sharp() {
        let pitch = parse_pitch_str("c+4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, Some(PitchAlter::QuarterSharp));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_quarter_flat() {
        let pitch = parse_pitch_str("cd4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, Some(PitchAlter::QuarterFlat));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_double_sharp() {
        let pitch = parse_pitch_str("cx4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, Some(PitchAlter::DoubleSharp));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_double_sharp_hash() {
        let pitch = parse_pitch_str("c##4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, Some(PitchAlter::DoubleSharp));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_double_flat() {
        let pitch = parse_pitch_str("ebb3").unwrap();
        assert_eq!(pitch.step, PitchStep::E);
        assert_eq!(pitch.alter, Some(PitchAlter::DoubleFlat));
        assert_eq!(pitch.octave, 3);
    }

    #[test]
    fn test_parse_pitch_str_uppercase() {
        let pitch = parse_pitch_str("C4").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_str_octave_0() {
        let pitch = parse_pitch_str("a0").unwrap();
        assert_eq!(pitch.step, PitchStep::A);
        assert_eq!(pitch.octave, 0);
    }

    #[test]
    fn test_parse_pitch_str_octave_9() {
        let pitch = parse_pitch_str("c9").unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.octave, 9);
    }

    #[test]
    fn test_parse_pitch_str_empty() {
        assert!(parse_pitch_str("").is_err());
    }

    #[test]
    fn test_parse_pitch_str_no_octave() {
        assert!(parse_pitch_str("c").is_err());
        assert!(parse_pitch_str("c#").is_err());
    }

    #[test]
    fn test_parse_pitch_str_invalid_step() {
        assert!(parse_pitch_str("x4").is_err());
    }

    #[test]
    fn test_parse_pitch_str_invalid_octave() {
        assert!(parse_pitch_str("c10").is_err());
    }

    // === compile_pitch tests ===

    #[test]
    fn test_compile_pitch_c4() {
        let fermata_pitch = FermataPitch {
            step: PitchStep::C,
            alter: None,
            octave: 4,
        };
        let ir_pitch = compile_pitch(&fermata_pitch).unwrap();
        assert_eq!(ir_pitch.step, IrStep::C);
        assert_eq!(ir_pitch.alter, None);
        assert_eq!(ir_pitch.octave, 4);
    }

    #[test]
    fn test_compile_pitch_f_sharp_5() {
        let fermata_pitch = FermataPitch {
            step: PitchStep::F,
            alter: Some(PitchAlter::Sharp),
            octave: 5,
        };
        let ir_pitch = compile_pitch(&fermata_pitch).unwrap();
        assert_eq!(ir_pitch.step, IrStep::F);
        assert_eq!(ir_pitch.alter, Some(1.0));
        assert_eq!(ir_pitch.octave, 5);
    }

    #[test]
    fn test_compile_pitch_bb3() {
        let fermata_pitch = FermataPitch {
            step: PitchStep::B,
            alter: Some(PitchAlter::Flat),
            octave: 3,
        };
        let ir_pitch = compile_pitch(&fermata_pitch).unwrap();
        assert_eq!(ir_pitch.step, IrStep::B);
        assert_eq!(ir_pitch.alter, Some(-1.0));
        assert_eq!(ir_pitch.octave, 3);
    }

    #[test]
    fn test_compile_pitch_quarter_sharp() {
        let fermata_pitch = FermataPitch {
            step: PitchStep::D,
            alter: Some(PitchAlter::QuarterSharp),
            octave: 4,
        };
        let ir_pitch = compile_pitch(&fermata_pitch).unwrap();
        assert_eq!(ir_pitch.step, IrStep::D);
        assert_eq!(ir_pitch.alter, Some(0.5));
    }

    #[test]
    fn test_compile_pitch_double_sharp() {
        let fermata_pitch = FermataPitch {
            step: PitchStep::G,
            alter: Some(PitchAlter::DoubleSharp),
            octave: 4,
        };
        let ir_pitch = compile_pitch(&fermata_pitch).unwrap();
        assert_eq!(ir_pitch.alter, Some(2.0));
    }

    // === parse_pitch tests (end-to-end) ===

    #[test]
    fn test_parse_pitch_c4() {
        let pitch = parse_pitch("c4").unwrap();
        assert_eq!(pitch.step, IrStep::C);
        assert_eq!(pitch.alter, None);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_f_sharp_5() {
        let pitch = parse_pitch("f#5").unwrap();
        assert_eq!(pitch.step, IrStep::F);
        assert_eq!(pitch.alter, Some(1.0));
        assert_eq!(pitch.octave, 5);
    }

    // === parse_pitch_sexpr tests ===

    #[test]
    fn test_parse_pitch_sexpr_symbol() {
        let sexpr = Sexpr::symbol("c4");
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_sexpr_symbol_with_sharp() {
        let sexpr = Sexpr::symbol("f#5");
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::F);
        assert_eq!(pitch.alter, Some(PitchAlter::Sharp));
        assert_eq!(pitch.octave, 5);
    }

    #[test]
    fn test_parse_pitch_sexpr_list_basic() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("C"),
            Sexpr::keyword("octave"),
            Sexpr::Integer(4),
        ]);
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::C);
        assert_eq!(pitch.alter, None);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_parse_pitch_sexpr_list_with_alter() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("F"),
            Sexpr::keyword("alter"),
            Sexpr::Integer(1),
            Sexpr::keyword("octave"),
            Sexpr::Integer(5),
        ]);
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::F);
        assert_eq!(pitch.alter, Some(PitchAlter::Sharp));
        assert_eq!(pitch.octave, 5);
    }

    #[test]
    fn test_parse_pitch_sexpr_list_with_flat() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("B"),
            Sexpr::keyword("alter"),
            Sexpr::Integer(-1),
            Sexpr::keyword("octave"),
            Sexpr::Integer(3),
        ]);
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::B);
        assert_eq!(pitch.alter, Some(PitchAlter::Flat));
        assert_eq!(pitch.octave, 3);
    }

    #[test]
    fn test_parse_pitch_sexpr_list_with_float_alter() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("D"),
            Sexpr::keyword("alter"),
            Sexpr::Float(0.5),
            Sexpr::keyword("octave"),
            Sexpr::Integer(4),
        ]);
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::D);
        assert_eq!(pitch.alter, Some(PitchAlter::QuarterSharp));
    }

    #[test]
    fn test_parse_pitch_sexpr_list_with_symbol_alter() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("G"),
            Sexpr::keyword("alter"),
            Sexpr::symbol("sharp"),
            Sexpr::keyword("octave"),
            Sexpr::Integer(4),
        ]);
        let pitch = parse_pitch_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, PitchStep::G);
        assert_eq!(pitch.alter, Some(PitchAlter::Sharp));
    }

    #[test]
    fn test_parse_pitch_sexpr_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(parse_pitch_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_pitch_sexpr_missing_step() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("octave"),
            Sexpr::Integer(4),
        ]);
        assert!(parse_pitch_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_pitch_sexpr_missing_octave() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("C"),
        ]);
        assert!(parse_pitch_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_pitch_sexpr_invalid_type() {
        let sexpr = Sexpr::Integer(42);
        assert!(parse_pitch_sexpr(&sexpr).is_err());
    }

    // === Additional edge case tests ===

    #[test]
    fn test_parse_pitch_str_all_steps() {
        for (c, expected) in [
            ('c', PitchStep::C),
            ('d', PitchStep::D),
            ('e', PitchStep::E),
            ('f', PitchStep::F),
            ('g', PitchStep::G),
            ('a', PitchStep::A),
            ('b', PitchStep::B),
        ] {
            let pitch_str = format!("{}4", c);
            let pitch = parse_pitch_str(&pitch_str).unwrap();
            assert_eq!(pitch.step, expected);
        }
    }

    #[test]
    fn test_parse_pitch_str_all_alterations() {
        let cases = [
            ("c#4", PitchAlter::Sharp),
            ("cb4", PitchAlter::Flat),
            ("c##4", PitchAlter::DoubleSharp),
            ("cx4", PitchAlter::DoubleSharp),
            ("cbb4", PitchAlter::DoubleFlat),
            ("cn4", PitchAlter::Natural),
            ("c+4", PitchAlter::QuarterSharp),
            ("cd4", PitchAlter::QuarterFlat),
        ];

        for (input, expected) in cases {
            let pitch = parse_pitch_str(input).unwrap();
            assert_eq!(pitch.alter, Some(expected), "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_compile_all_steps() {
        let steps = [
            (PitchStep::C, IrStep::C),
            (PitchStep::D, IrStep::D),
            (PitchStep::E, IrStep::E),
            (PitchStep::F, IrStep::F),
            (PitchStep::G, IrStep::G),
            (PitchStep::A, IrStep::A),
            (PitchStep::B, IrStep::B),
        ];

        for (fermata_step, ir_step) in steps {
            let pitch = FermataPitch {
                step: fermata_step,
                alter: None,
                octave: 4,
            };
            let ir_pitch = compile_pitch(&pitch).unwrap();
            assert_eq!(ir_pitch.step, ir_step);
        }
    }
}
