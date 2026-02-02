//! Attributes compilation for Fermata syntax.
//!
//! This module handles compiling attribute S-expressions (key, time, clef,
//! divisions, etc.) into IR types.

use crate::ir::attributes::Mode as IrMode;
use crate::ir::attributes::{
    Clef, ClefSign, Key, KeyContent, Time, TimeContent, TimeSignature, TimeSymbol, TraditionalKey,
};
use crate::sexpr::Sexpr;

use super::ast::{ClefSpec, KeySpec, Mode as FermataMode, PitchAlter, PitchStep, TimeSpec};
use super::error::{CompileError, CompileResult};

// =============================================================================
// Key Signature Compilation
// =============================================================================

/// Compile a key S-expression into an IR Key.
///
/// Supports forms like:
/// - `(key c :major)`
/// - `(key f# :major)`
/// - `(key bb :minor)`
/// - `(key d :dorian)`
pub fn compile_key(sexpr: &Sexpr) -> CompileResult<Key> {
    let args = sexpr
        .as_list()
        .ok_or_else(|| CompileError::type_mismatch("list", format!("{:?}", sexpr)))?;

    if args.is_empty() {
        return Err(CompileError::InvalidKey("empty key form".to_string()));
    }

    // First element should be the symbol "key"
    let head = args
        .first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::InvalidKey("expected 'key' symbol".to_string()))?;

    if head != "key" {
        return Err(CompileError::InvalidKey(format!(
            "expected 'key' form, got '{}'",
            head
        )));
    }

    let spec = parse_key_form(&args[1..])?;
    compile_key_spec(&spec)
}

/// Parse key arguments into a KeySpec.
///
/// Expected form: `<root> <mode>` where root is like "c", "f#", "bb"
/// and mode is a keyword like `:major`, `:minor`, `:dorian`.
pub fn parse_key_form(args: &[Sexpr]) -> CompileResult<KeySpec> {
    if args.len() < 2 {
        return Err(CompileError::InvalidKey(
            "key requires root and mode".to_string(),
        ));
    }

    // First argument: root (e.g., "c", "f#", "bb")
    let root_str = args[0]
        .as_symbol()
        .ok_or_else(|| CompileError::InvalidKey("expected root symbol".to_string()))?;

    let (root, root_alter) = parse_key_root(root_str)?;

    // Second argument: mode keyword (e.g., :major, :minor)
    let mode_str = args[1]
        .as_keyword()
        .ok_or_else(|| CompileError::InvalidKey("expected mode keyword".to_string()))?;

    let mode = parse_mode(mode_str)?;

    Ok(KeySpec {
        root,
        root_alter,
        mode,
    })
}

/// Parse a key root string into a pitch step and optional alteration.
///
/// Examples: "c" -> (C, None), "f#" -> (F, Some(Sharp)), "bb" -> (B, Some(Flat))
pub fn parse_key_root(s: &str) -> CompileResult<(PitchStep, Option<PitchAlter>)> {
    if s.is_empty() {
        return Err(CompileError::InvalidKey("empty root".to_string()));
    }

    let lower = s.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    // Parse the step (first character)
    let step = match chars[0] {
        'c' => PitchStep::C,
        'd' => PitchStep::D,
        'e' => PitchStep::E,
        'f' => PitchStep::F,
        'g' => PitchStep::G,
        'a' => PitchStep::A,
        'b' => PitchStep::B,
        _ => {
            return Err(CompileError::InvalidKey(format!(
                "invalid pitch step: {}",
                chars[0]
            )));
        }
    };

    // Parse optional alteration (rest of string)
    let alter = if chars.len() > 1 {
        let alter_str: String = chars[1..].iter().collect();
        Some(parse_alteration(&alter_str)?)
    } else {
        None
    };

    Ok((step, alter))
}

/// Parse an alteration string into a PitchAlter.
fn parse_alteration(s: &str) -> CompileResult<PitchAlter> {
    match s {
        "#" | "s" | "sharp" => Ok(PitchAlter::Sharp),
        "b" | "flat" => Ok(PitchAlter::Flat),
        "##" | "x" | "double-sharp" => Ok(PitchAlter::DoubleSharp),
        "bb" | "double-flat" => Ok(PitchAlter::DoubleFlat),
        "n" | "natural" => Ok(PitchAlter::Natural),
        _ => Err(CompileError::InvalidKey(format!(
            "invalid alteration: {}",
            s
        ))),
    }
}

/// Parse a mode keyword string into a FermataMode.
pub fn parse_mode(s: &str) -> CompileResult<FermataMode> {
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
        _ => Err(CompileError::InvalidKey(format!("unknown mode: {}", s))),
    }
}

/// Compile a KeySpec into an IR Key.
pub fn compile_key_spec(spec: &KeySpec) -> CompileResult<Key> {
    let fifths = compute_fifths(spec.root, spec.root_alter.as_ref(), &spec.mode);

    let ir_mode = match spec.mode {
        FermataMode::Major => IrMode::Major,
        FermataMode::Minor => IrMode::Minor,
        FermataMode::Dorian => IrMode::Dorian,
        FermataMode::Phrygian => IrMode::Phrygian,
        FermataMode::Lydian => IrMode::Lydian,
        FermataMode::Mixolydian => IrMode::Mixolydian,
        FermataMode::Aeolian => IrMode::Aeolian,
        FermataMode::Ionian => IrMode::Ionian,
        FermataMode::Locrian => IrMode::Locrian,
    };

    Ok(Key {
        content: KeyContent::Traditional(TraditionalKey {
            cancel: None,
            fifths,
            mode: Some(ir_mode),
        }),
        number: None,
        print_object: None,
    })
}

/// Compute the circle of fifths position for a key.
///
/// The calculation:
/// 1. Start with the base fifths for the root in major mode
/// 2. Add 7 for sharp, subtract 7 for flat
/// 3. Adjust for mode
///
/// Base fifths (major mode):
/// - C = 0, G = 1, D = 2, A = 3, E = 4, B = 5, F# = 6
/// - F = -1, Bb = -2, Eb = -3, Ab = -4, Db = -5, Gb = -6, Cb = -7
///
/// Mode adjustments (relative to major):
/// - Ionian = 0 (same as major)
/// - Dorian = -2
/// - Phrygian = -4
/// - Lydian = +1
/// - Mixolydian = -1
/// - Aeolian = -3 (same as minor)
/// - Locrian = -5
/// - Minor = -3
pub fn compute_fifths(root: PitchStep, root_alter: Option<&PitchAlter>, mode: &FermataMode) -> i8 {
    // Base fifths for natural roots in major mode
    let base = match root {
        PitchStep::C => 0,
        PitchStep::G => 1,
        PitchStep::D => 2,
        PitchStep::A => 3,
        PitchStep::E => 4,
        PitchStep::B => 5,
        PitchStep::F => -1,
    };

    // Adjust for alteration (sharp adds 7, flat subtracts 7)
    let alter_adjustment = match root_alter {
        Some(PitchAlter::Sharp) => 7,
        Some(PitchAlter::Flat) => -7,
        Some(PitchAlter::DoubleSharp) => 14,
        Some(PitchAlter::DoubleFlat) => -14,
        _ => 0,
    };

    // Mode adjustment
    let mode_adjustment = match mode {
        FermataMode::Major | FermataMode::Ionian => 0,
        FermataMode::Minor | FermataMode::Aeolian => -3,
        FermataMode::Dorian => -2,
        FermataMode::Phrygian => -4,
        FermataMode::Lydian => 1,
        FermataMode::Mixolydian => -1,
        FermataMode::Locrian => -5,
    };

    base + alter_adjustment + mode_adjustment
}

// =============================================================================
// Time Signature Compilation
// =============================================================================

/// Compile a time S-expression into an IR Time.
///
/// Supports forms like:
/// - `(time 4 4)` - simple time signature
/// - `(time :common)` - common time (4/4 with C symbol)
/// - `(time :cut)` - cut time (2/2 with cut C symbol)
pub fn compile_time(sexpr: &Sexpr) -> CompileResult<Time> {
    let args = sexpr
        .as_list()
        .ok_or_else(|| CompileError::type_mismatch("list", format!("{:?}", sexpr)))?;

    if args.is_empty() {
        return Err(CompileError::InvalidTime("empty time form".to_string()));
    }

    // First element should be the symbol "time"
    let head = args
        .first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::InvalidTime("expected 'time' symbol".to_string()))?;

    if head != "time" {
        return Err(CompileError::InvalidTime(format!(
            "expected 'time' form, got '{}'",
            head
        )));
    }

    let spec = parse_time_form(&args[1..])?;
    compile_time_spec(&spec)
}

/// Parse time arguments into a TimeSpec.
pub fn parse_time_form(args: &[Sexpr]) -> CompileResult<TimeSpec> {
    if args.is_empty() {
        return Err(CompileError::InvalidTime(
            "time signature requires arguments".to_string(),
        ));
    }

    // Check for keyword forms (:common, :cut)
    if let Some(kw) = args[0].as_keyword() {
        match kw {
            "common" => return Ok(TimeSpec::Common),
            "cut" => return Ok(TimeSpec::Cut),
            "senza-misura" | "senza" => return Ok(TimeSpec::SenzaMisura),
            _ => {
                return Err(CompileError::InvalidTime(format!(
                    "unknown time keyword: {}",
                    kw
                )));
            }
        }
    }

    // Parse numeric time signature (beats beat-type)
    if args.len() < 2 {
        return Err(CompileError::InvalidTime(
            "time signature requires beats and beat-type".to_string(),
        ));
    }

    let beats = args[0]
        .as_integer()
        .ok_or_else(|| CompileError::InvalidTime("beats must be an integer".to_string()))?;

    let beat_type = args[1]
        .as_integer()
        .ok_or_else(|| CompileError::InvalidTime("beat-type must be an integer".to_string()))?;

    if beats <= 0 || beats > 255 {
        return Err(CompileError::InvalidTime(format!(
            "beats out of range: {}",
            beats
        )));
    }

    if beat_type <= 0 || beat_type > 255 {
        return Err(CompileError::InvalidTime(format!(
            "beat-type out of range: {}",
            beat_type
        )));
    }

    Ok(TimeSpec::Simple {
        beats: beats as u8,
        beat_type: beat_type as u8,
    })
}

/// Compile a TimeSpec into an IR Time.
pub fn compile_time_spec(spec: &TimeSpec) -> CompileResult<Time> {
    match spec {
        TimeSpec::Simple { beats, beat_type } => Ok(Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: beats.to_string(),
                    beat_type: beat_type.to_string(),
                }],
            },
            number: None,
            symbol: None,
            print_object: None,
        }),
        TimeSpec::Compound { signatures } => Ok(Time {
            content: TimeContent::Measured {
                signatures: signatures
                    .iter()
                    .map(|(b, bt)| TimeSignature {
                        beats: b.to_string(),
                        beat_type: bt.to_string(),
                    })
                    .collect(),
            },
            number: None,
            symbol: None,
            print_object: None,
        }),
        TimeSpec::Common => Ok(Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Common),
            print_object: None,
        }),
        TimeSpec::Cut => Ok(Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "2".to_string(),
                    beat_type: "2".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Cut),
            print_object: None,
        }),
        TimeSpec::SenzaMisura => Ok(Time {
            content: TimeContent::SenzaMisura(String::new()),
            number: None,
            symbol: None,
            print_object: None,
        }),
    }
}

// =============================================================================
// Clef Compilation
// =============================================================================

/// Compile a clef S-expression into an IR Clef.
///
/// Supports forms like:
/// - `(clef :treble)`
/// - `(clef :bass)`
/// - `(clef :alto)`
/// - `(clef :treble-8vb)`
pub fn compile_clef(sexpr: &Sexpr) -> CompileResult<Clef> {
    let args = sexpr
        .as_list()
        .ok_or_else(|| CompileError::type_mismatch("list", format!("{:?}", sexpr)))?;

    if args.is_empty() {
        return Err(CompileError::InvalidClef("empty clef form".to_string()));
    }

    // First element should be the symbol "clef"
    let head = args
        .first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::InvalidClef("expected 'clef' symbol".to_string()))?;

    if head != "clef" {
        return Err(CompileError::InvalidClef(format!(
            "expected 'clef' form, got '{}'",
            head
        )));
    }

    if args.len() < 2 {
        return Err(CompileError::InvalidClef(
            "clef requires a type".to_string(),
        ));
    }

    // Second argument: clef type keyword
    let clef_name = args[1]
        .as_keyword()
        .ok_or_else(|| CompileError::InvalidClef("expected clef type keyword".to_string()))?;

    let spec = parse_clef_name(clef_name)?;
    compile_clef_spec(&spec)
}

/// Parse a clef name keyword into a ClefSpec.
pub fn parse_clef_name(name: &str) -> CompileResult<ClefSpec> {
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
        _ => Err(CompileError::InvalidClef(format!(
            "unknown clef type: {}",
            name
        ))),
    }
}

/// Compile a ClefSpec into an IR Clef.
pub fn compile_clef_spec(spec: &ClefSpec) -> CompileResult<Clef> {
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
        ClefSpec::Custom {
            sign,
            line,
            octave_change,
        } => {
            let clef_sign = match sign {
                'G' | 'g' => ClefSign::G,
                'F' | 'f' => ClefSign::F,
                'C' | 'c' => ClefSign::C,
                _ => {
                    return Err(CompileError::InvalidClef(format!(
                        "unknown clef sign: {}",
                        sign
                    )));
                }
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
    use crate::sexpr::parse;

    // =============================================================================
    // Key Signature Tests
    // =============================================================================

    mod key_tests {
        use super::*;

        #[test]
        fn test_compile_key_c_major() {
            let sexpr = parse("(key c :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Major));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_g_major() {
            let sexpr = parse("(key g :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 1);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_d_major() {
            let sexpr = parse("(key d :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 2);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_a_major() {
            let sexpr = parse("(key a :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 3);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_e_major() {
            let sexpr = parse("(key e :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 4);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_b_major() {
            let sexpr = parse("(key b :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 5);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_f_major() {
            let sexpr = parse("(key f :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -1);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_f_sharp_major() {
            let sexpr = parse("(key f# :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 6);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_c_sharp_major() {
            let sexpr = parse("(key c# :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 7);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_bb_major() {
            let sexpr = parse("(key bb :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -2);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_eb_major() {
            let sexpr = parse("(key eb :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -3);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_ab_major() {
            let sexpr = parse("(key ab :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -4);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_db_major() {
            let sexpr = parse("(key db :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -5);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_gb_major() {
            let sexpr = parse("(key gb :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -6);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_cb_major() {
            let sexpr = parse("(key cb :major)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -7);
            } else {
                panic!("Expected Traditional key");
            }
        }

        // Minor keys
        #[test]
        fn test_compile_key_a_minor() {
            let sexpr = parse("(key a :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Minor));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_e_minor() {
            let sexpr = parse("(key e :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 1);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_d_minor() {
            let sexpr = parse("(key d :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -1);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_b_minor() {
            let sexpr = parse("(key b :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 2);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_f_sharp_minor() {
            let sexpr = parse("(key f# :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 3);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_c_minor() {
            let sexpr = parse("(key c :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -3);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_g_minor() {
            let sexpr = parse("(key g :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -2);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_c_sharp_minor() {
            let sexpr = parse("(key c# :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 4);
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_f_minor() {
            let sexpr = parse("(key f :minor)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, -4);
            } else {
                panic!("Expected Traditional key");
            }
        }

        // Modal keys
        #[test]
        fn test_compile_key_d_dorian() {
            let sexpr = parse("(key d :dorian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Dorian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_e_phrygian() {
            let sexpr = parse("(key e :phrygian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Phrygian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_f_lydian() {
            let sexpr = parse("(key f :lydian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Lydian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_g_mixolydian() {
            let sexpr = parse("(key g :mixolydian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Mixolydian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_a_aeolian() {
            let sexpr = parse("(key a :aeolian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Aeolian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_c_ionian() {
            let sexpr = parse("(key c :ionian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Ionian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        #[test]
        fn test_compile_key_b_locrian() {
            let sexpr = parse("(key b :locrian)").unwrap();
            let key = compile_key(&sexpr).unwrap();
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
                assert_eq!(tk.mode, Some(IrMode::Locrian));
            } else {
                panic!("Expected Traditional key");
            }
        }

        // Error cases
        #[test]
        fn test_compile_key_invalid_empty() {
            let sexpr = parse("(key)").unwrap();
            assert!(compile_key(&sexpr).is_err());
        }

        #[test]
        fn test_compile_key_invalid_missing_mode() {
            let sexpr = parse("(key c)").unwrap();
            assert!(compile_key(&sexpr).is_err());
        }

        #[test]
        fn test_compile_key_invalid_root() {
            let sexpr = parse("(key x :major)").unwrap();
            assert!(compile_key(&sexpr).is_err());
        }

        #[test]
        fn test_compile_key_invalid_mode() {
            let sexpr = parse("(key c :unknown)").unwrap();
            assert!(compile_key(&sexpr).is_err());
        }

        // Helper function tests
        #[test]
        fn test_parse_key_root_c() {
            let (step, alter) = parse_key_root("c").unwrap();
            assert_eq!(step, PitchStep::C);
            assert!(alter.is_none());
        }

        #[test]
        fn test_parse_key_root_f_sharp() {
            let (step, alter) = parse_key_root("f#").unwrap();
            assert_eq!(step, PitchStep::F);
            assert_eq!(alter, Some(PitchAlter::Sharp));
        }

        #[test]
        fn test_parse_key_root_bb() {
            let (step, alter) = parse_key_root("bb").unwrap();
            assert_eq!(step, PitchStep::B);
            assert_eq!(alter, Some(PitchAlter::Flat));
        }

        #[test]
        fn test_compute_fifths_major_keys() {
            assert_eq!(compute_fifths(PitchStep::C, None, &FermataMode::Major), 0);
            assert_eq!(compute_fifths(PitchStep::G, None, &FermataMode::Major), 1);
            assert_eq!(compute_fifths(PitchStep::D, None, &FermataMode::Major), 2);
            assert_eq!(compute_fifths(PitchStep::F, None, &FermataMode::Major), -1);
        }

        #[test]
        fn test_compute_fifths_with_sharps() {
            assert_eq!(
                compute_fifths(PitchStep::F, Some(&PitchAlter::Sharp), &FermataMode::Major),
                6
            );
            assert_eq!(
                compute_fifths(PitchStep::C, Some(&PitchAlter::Sharp), &FermataMode::Major),
                7
            );
        }

        #[test]
        fn test_compute_fifths_with_flats() {
            assert_eq!(
                compute_fifths(PitchStep::B, Some(&PitchAlter::Flat), &FermataMode::Major),
                -2
            );
            assert_eq!(
                compute_fifths(PitchStep::E, Some(&PitchAlter::Flat), &FermataMode::Major),
                -3
            );
        }

        #[test]
        fn test_compute_fifths_minor_keys() {
            assert_eq!(compute_fifths(PitchStep::A, None, &FermataMode::Minor), 0);
            assert_eq!(compute_fifths(PitchStep::E, None, &FermataMode::Minor), 1);
            assert_eq!(compute_fifths(PitchStep::D, None, &FermataMode::Minor), -1);
        }
    }

    // =============================================================================
    // Time Signature Tests
    // =============================================================================

    mod time_tests {
        use super::*;

        #[test]
        fn test_compile_time_4_4() {
            let sexpr = parse("(time 4 4)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures.len(), 1);
                assert_eq!(signatures[0].beats, "4");
                assert_eq!(signatures[0].beat_type, "4");
            } else {
                panic!("Expected Measured time");
            }
            assert!(time.symbol.is_none());
        }

        #[test]
        fn test_compile_time_3_4() {
            let sexpr = parse("(time 3 4)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "3");
                assert_eq!(signatures[0].beat_type, "4");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_6_8() {
            let sexpr = parse("(time 6 8)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "6");
                assert_eq!(signatures[0].beat_type, "8");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_2_2() {
            let sexpr = parse("(time 2 2)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "2");
                assert_eq!(signatures[0].beat_type, "2");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_5_4() {
            let sexpr = parse("(time 5 4)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "5");
                assert_eq!(signatures[0].beat_type, "4");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_7_8() {
            let sexpr = parse("(time 7 8)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "7");
                assert_eq!(signatures[0].beat_type, "8");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_common() {
            let sexpr = parse("(time :common)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            assert_eq!(time.symbol, Some(TimeSymbol::Common));
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "4");
                assert_eq!(signatures[0].beat_type, "4");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_cut() {
            let sexpr = parse("(time :cut)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            assert_eq!(time.symbol, Some(TimeSymbol::Cut));
            if let TimeContent::Measured { signatures } = &time.content {
                assert_eq!(signatures[0].beats, "2");
                assert_eq!(signatures[0].beat_type, "2");
            } else {
                panic!("Expected Measured time");
            }
        }

        #[test]
        fn test_compile_time_senza_misura() {
            let sexpr = parse("(time :senza-misura)").unwrap();
            let time = compile_time(&sexpr).unwrap();
            if let TimeContent::SenzaMisura(text) = &time.content {
                assert_eq!(text, "");
            } else {
                panic!("Expected SenzaMisura time");
            }
        }

        // Error cases
        #[test]
        fn test_compile_time_invalid_empty() {
            let sexpr = parse("(time)").unwrap();
            assert!(compile_time(&sexpr).is_err());
        }

        #[test]
        fn test_compile_time_invalid_missing_beat_type() {
            let sexpr = parse("(time 4)").unwrap();
            assert!(compile_time(&sexpr).is_err());
        }

        #[test]
        fn test_compile_time_invalid_unknown_keyword() {
            let sexpr = parse("(time :unknown)").unwrap();
            assert!(compile_time(&sexpr).is_err());
        }
    }

    // =============================================================================
    // Clef Tests
    // =============================================================================

    mod clef_tests {
        use super::*;

        #[test]
        fn test_compile_clef_treble() {
            let sexpr = parse("(clef :treble)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::G);
            assert_eq!(clef.line, Some(2));
            assert!(clef.octave_change.is_none());
        }

        #[test]
        fn test_compile_clef_bass() {
            let sexpr = parse("(clef :bass)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::F);
            assert_eq!(clef.line, Some(4));
            assert!(clef.octave_change.is_none());
        }

        #[test]
        fn test_compile_clef_alto() {
            let sexpr = parse("(clef :alto)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::C);
            assert_eq!(clef.line, Some(3));
            assert!(clef.octave_change.is_none());
        }

        #[test]
        fn test_compile_clef_tenor() {
            let sexpr = parse("(clef :tenor)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::C);
            assert_eq!(clef.line, Some(4));
            assert!(clef.octave_change.is_none());
        }

        #[test]
        fn test_compile_clef_treble_8vb() {
            let sexpr = parse("(clef :treble-8vb)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::G);
            assert_eq!(clef.line, Some(2));
            assert_eq!(clef.octave_change, Some(-1));
        }

        #[test]
        fn test_compile_clef_treble_8va() {
            let sexpr = parse("(clef :treble-8va)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::G);
            assert_eq!(clef.line, Some(2));
            assert_eq!(clef.octave_change, Some(1));
        }

        #[test]
        fn test_compile_clef_bass_8vb() {
            let sexpr = parse("(clef :bass-8vb)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::F);
            assert_eq!(clef.line, Some(4));
            assert_eq!(clef.octave_change, Some(-1));
        }

        #[test]
        fn test_compile_clef_bass_8va() {
            let sexpr = parse("(clef :bass-8va)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::F);
            assert_eq!(clef.line, Some(4));
            assert_eq!(clef.octave_change, Some(1));
        }

        #[test]
        fn test_compile_clef_percussion() {
            let sexpr = parse("(clef :percussion)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::Percussion);
            assert!(clef.line.is_none());
            assert!(clef.octave_change.is_none());
        }

        #[test]
        fn test_compile_clef_tab() {
            let sexpr = parse("(clef :tab)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::Tab);
            assert_eq!(clef.line, Some(5));
            assert!(clef.octave_change.is_none());
        }

        // Alternative spellings
        #[test]
        fn test_compile_clef_g() {
            let sexpr = parse("(clef :g)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::G);
        }

        #[test]
        fn test_compile_clef_f() {
            let sexpr = parse("(clef :f)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::F);
        }

        #[test]
        fn test_compile_clef_c() {
            let sexpr = parse("(clef :c)").unwrap();
            let clef = compile_clef(&sexpr).unwrap();
            assert_eq!(clef.sign, ClefSign::C);
        }

        // Error cases
        #[test]
        fn test_compile_clef_invalid_empty() {
            let sexpr = parse("(clef)").unwrap();
            assert!(compile_clef(&sexpr).is_err());
        }

        #[test]
        fn test_compile_clef_invalid_unknown() {
            let sexpr = parse("(clef :unknown)").unwrap();
            assert!(compile_clef(&sexpr).is_err());
        }

        // ClefSpec compile tests
        #[test]
        fn test_compile_clef_spec_custom() {
            let spec = ClefSpec::Custom {
                sign: 'G',
                line: 2,
                octave_change: Some(-1),
            };
            let clef = compile_clef_spec(&spec).unwrap();
            assert_eq!(clef.sign, ClefSign::G);
            assert_eq!(clef.line, Some(2));
            assert_eq!(clef.octave_change, Some(-1));
        }

        #[test]
        fn test_compile_clef_spec_custom_invalid_sign() {
            let spec = ClefSpec::Custom {
                sign: 'X',
                line: 2,
                octave_change: None,
            };
            assert!(compile_clef_spec(&spec).is_err());
        }
    }
}
