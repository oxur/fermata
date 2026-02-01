//! Duration parsing and compilation.
//!
//! This module provides functions for parsing duration keywords and symbols
//! (e.g., :q, :quarter, :h., :w) and compiling them to the IR representation.

use crate::ir::common::PositiveDivisions;
use crate::ir::duration::{Dot, NoteType, NoteTypeValue};
use crate::lang::ast::{DurationBase, FermataDuration};
use crate::lang::defaults::DEFAULT_DIVISIONS;
use crate::lang::error::{CompileError, CompileResult};
use crate::sexpr::Sexpr;

/// Parse a duration keyword to FermataDuration.
///
/// Supported formats:
/// - Short forms: `:q`, `:h`, `:w`, `:8`, `:16`, `:32`, `:64`, `:128`, `:256`, `:512`, `:1024`
/// - Full names: `:quarter`, `:half`, `:whole`, `:eighth`
/// - British names: `:crotchet`, `:minim`, `:semibreve`, `:quaver`
/// - With dots: `:q.`, `:h..`, `:quarter.`
///
/// # Examples
///
/// ```
/// use fermata::lang::duration::parse_duration;
///
/// let dur = parse_duration("q").unwrap();
/// assert_eq!(dur.dots, 0);
///
/// let dotted = parse_duration("q.").unwrap();
/// assert_eq!(dotted.dots, 1);
/// ```
pub fn parse_duration(s: &str) -> CompileResult<FermataDuration> {
    let s = s.trim();
    if s.is_empty() {
        return Err(CompileError::InvalidDuration("empty duration string".to_string()));
    }

    // Strip leading colon if present (keyword syntax)
    let s = s.strip_prefix(':').unwrap_or(s);

    // Count and strip trailing dots
    let dot_count = s.chars().rev().take_while(|&c| c == '.').count();
    let base_str = &s[..s.len() - dot_count];

    if base_str.is_empty() {
        return Err(CompileError::InvalidDuration(
            "duration cannot be only dots".to_string()
        ));
    }

    let base = parse_duration_base(base_str)?;

    Ok(FermataDuration {
        base,
        dots: dot_count as u8,
    })
}

/// Parse a base duration string to a DurationBase.
///
/// Supports:
/// - Short forms: `q`, `h`, `w`, `8`, `16`, `32`, `64`, `128`, `256`, `512`, `1024`
/// - Full names: `quarter`, `half`, `whole`, `eighth`, `sixteenth`
/// - British names: `crotchet`, `minim`, `semibreve`, `quaver`, `semiquaver`
pub fn parse_duration_base(s: &str) -> CompileResult<DurationBase> {
    match s.to_lowercase().as_str() {
        // American short forms
        "w" | "whole" | "semibreve" => Ok(DurationBase::Whole),
        "h" | "half" | "minim" => Ok(DurationBase::Half),
        "q" | "quarter" | "crotchet" => Ok(DurationBase::Quarter),
        "8" | "eighth" | "quaver" => Ok(DurationBase::Eighth),
        "16" | "sixteenth" | "semiquaver" => Ok(DurationBase::Sixteenth),
        "32" | "thirty-second" | "thirtysecond" | "demisemiquaver" => Ok(DurationBase::ThirtySecond),
        "64" | "sixty-fourth" | "sixtyfourth" | "hemidemisemiquaver" => Ok(DurationBase::SixtyFourth),
        "128" | "one-twenty-eighth" | "onetwentyeighth" => Ok(DurationBase::OneTwentyEighth),
        "256" | "two-fifty-sixth" | "twofiftysixth" => Ok(DurationBase::TwoFiftySixth),
        "512" | "five-twelfth" | "fivetwelfth" => Ok(DurationBase::FiveTwelfth),
        "1024" | "one-thousand-twenty-fourth" | "onethousandtwentyfourth" => Ok(DurationBase::OneThousandTwentyFourth),

        // Long durations
        "breve" | "double-whole" | "doublewhole" => Ok(DurationBase::Breve),
        "long" | "longa" => Ok(DurationBase::Long),
        "maxima" => Ok(DurationBase::Maxima),

        _ => Err(CompileError::InvalidDuration(
            format!("unknown duration '{}', expected q, h, w, 8, 16, 32, etc.", s)
        )),
    }
}

/// Compile a DurationBase to IR NoteType.
pub fn compile_duration_type(base: &DurationBase) -> NoteType {
    let value = match base {
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
    };

    NoteType { value, size: None }
}

/// Compile a dot count to a Vec<Dot>.
pub fn compile_dots(count: u8) -> Vec<Dot> {
    (0..count).map(|_| Dot::default()).collect()
}

/// Compile a FermataDuration to a divisions value.
///
/// Uses the default divisions per quarter note (typically 1) to calculate
/// the absolute duration in divisions.
///
/// # Duration calculation with dots
///
/// A dotted note adds half of its value for each dot:
/// - Quarter (1.0) with 1 dot = 1.0 + 0.5 = 1.5
/// - Quarter (1.0) with 2 dots = 1.0 + 0.5 + 0.25 = 1.75
/// - Half (2.0) with 1 dot = 2.0 + 1.0 = 3.0
pub fn compile_duration_divisions(duration: &FermataDuration) -> PositiveDivisions {
    compile_duration_divisions_with(duration, DEFAULT_DIVISIONS as u64)
}

/// Compile a FermataDuration to a divisions value with explicit divisions per quarter.
pub fn compile_duration_divisions_with(
    duration: &FermataDuration,
    divisions_per_quarter: PositiveDivisions,
) -> PositiveDivisions {
    // Base duration relative to quarter note
    let base_quarters = match duration.base {
        DurationBase::Maxima => 32.0,  // 8 whole notes = 32 quarters
        DurationBase::Long => 16.0,    // 4 whole notes = 16 quarters
        DurationBase::Breve => 8.0,    // 2 whole notes = 8 quarters
        DurationBase::Whole => 4.0,    // 4 quarters
        DurationBase::Half => 2.0,     // 2 quarters
        DurationBase::Quarter => 1.0,  // 1 quarter
        DurationBase::Eighth => 0.5,
        DurationBase::Sixteenth => 0.25,
        DurationBase::ThirtySecond => 0.125,
        DurationBase::SixtyFourth => 0.0625,
        DurationBase::OneTwentyEighth => 0.03125,
        DurationBase::TwoFiftySixth => 0.015625,
        DurationBase::FiveTwelfth => 0.0078125,
        DurationBase::OneThousandTwentyFourth => 0.00390625,
    };

    // Apply dot multiplier: each dot adds half of the previous value
    // dot_multiplier = 1 + 1/2 + 1/4 + ... = 2 - (1/2)^dots
    let dot_multiplier = if duration.dots == 0 {
        1.0
    } else {
        let mut multiplier = 1.0;
        let mut add = 0.5;
        for _ in 0..duration.dots {
            multiplier += add;
            add *= 0.5;
        }
        multiplier
    };

    let total_quarters = base_quarters * dot_multiplier;
    let divisions = total_quarters * divisions_per_quarter as f64;

    // Round to nearest integer (should be exact for standard durations)
    divisions.round() as PositiveDivisions
}

/// Parse a duration from an S-expression.
///
/// Supported formats:
/// - `:q` (keyword) - Quarter note
/// - `(duration :base quarter :dots 1)` - Dotted quarter
/// - `q.` (symbol) - Dotted quarter shorthand
pub fn parse_duration_sexpr(sexpr: &Sexpr) -> CompileResult<FermataDuration> {
    match sexpr {
        // Keyword form: :q, :h, :w, :q., :h..
        Sexpr::Keyword(s) => parse_duration(s),

        // Symbol form: q, h, w, q., h..
        Sexpr::Symbol(s) => parse_duration(s),

        // Full form: (duration :base quarter :dots 1)
        Sexpr::List(items) => {
            if items.is_empty() {
                return Err(CompileError::InvalidDuration("empty duration list".to_string()));
            }

            // Check for 'duration' head
            if !items[0].is_symbol("duration") {
                return Err(CompileError::InvalidDuration(
                    format!("expected 'duration', got {:?}", items[0])
                ));
            }

            let mut base: Option<DurationBase> = None;
            let mut dots: u8 = 0;

            // Parse keyword arguments
            let mut i = 1;
            while i < items.len() {
                if let Some(kw) = items[i].as_keyword() {
                    if i + 1 >= items.len() {
                        return Err(CompileError::InvalidDuration(
                            format!("missing value for keyword :{}", kw)
                        ));
                    }

                    match kw {
                        "base" | "type" => {
                            base = Some(parse_duration_base_sexpr(&items[i + 1])?);
                        }
                        "dots" => {
                            dots = parse_dots_sexpr(&items[i + 1])?;
                        }
                        _ => {
                            return Err(CompileError::InvalidDuration(
                                format!("unknown duration keyword :{}", kw)
                            ));
                        }
                    }
                    i += 2;
                } else {
                    return Err(CompileError::InvalidDuration(
                        format!("expected keyword, got {:?}", items[i])
                    ));
                }
            }

            // Validate required fields
            let base = base.ok_or_else(|| {
                CompileError::InvalidDuration("missing :base in duration".to_string())
            })?;

            Ok(FermataDuration { base, dots })
        }

        _ => Err(CompileError::InvalidDuration(
            format!("expected duration keyword or list, got {:?}", sexpr)
        )),
    }
}

/// Parse a duration base from an S-expression.
fn parse_duration_base_sexpr(sexpr: &Sexpr) -> CompileResult<DurationBase> {
    match sexpr {
        Sexpr::Symbol(s) => parse_duration_base(s),
        Sexpr::Keyword(s) => parse_duration_base(s),
        Sexpr::Integer(n) => {
            // Numeric form: 1 = whole, 2 = half, 4 = quarter, 8 = eighth, etc.
            match *n {
                1 => Ok(DurationBase::Whole),
                2 => Ok(DurationBase::Half),
                4 => Ok(DurationBase::Quarter),
                8 => Ok(DurationBase::Eighth),
                16 => Ok(DurationBase::Sixteenth),
                32 => Ok(DurationBase::ThirtySecond),
                64 => Ok(DurationBase::SixtyFourth),
                128 => Ok(DurationBase::OneTwentyEighth),
                256 => Ok(DurationBase::TwoFiftySixth),
                512 => Ok(DurationBase::FiveTwelfth),
                1024 => Ok(DurationBase::OneThousandTwentyFourth),
                _ => Err(CompileError::InvalidDuration(
                    format!("invalid numeric duration {}, expected 1, 2, 4, 8, etc.", n)
                )),
            }
        }
        _ => Err(CompileError::InvalidDuration(
            format!("expected duration base symbol or number, got {:?}", sexpr)
        )),
    }
}

/// Parse a dots value from an S-expression.
fn parse_dots_sexpr(sexpr: &Sexpr) -> CompileResult<u8> {
    match sexpr {
        Sexpr::Integer(n) => {
            if *n < 0 || *n > 4 {
                return Err(CompileError::InvalidDuration(
                    format!("dots {} out of range (0-4)", n)
                ));
            }
            Ok(*n as u8)
        }
        _ => Err(CompileError::InvalidDuration(
            format!("expected dots integer, got {:?}", sexpr)
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_duration_base tests ===

    #[test]
    fn test_parse_duration_base_short_forms() {
        assert_eq!(parse_duration_base("w").unwrap(), DurationBase::Whole);
        assert_eq!(parse_duration_base("h").unwrap(), DurationBase::Half);
        assert_eq!(parse_duration_base("q").unwrap(), DurationBase::Quarter);
        assert_eq!(parse_duration_base("8").unwrap(), DurationBase::Eighth);
        assert_eq!(parse_duration_base("16").unwrap(), DurationBase::Sixteenth);
        assert_eq!(parse_duration_base("32").unwrap(), DurationBase::ThirtySecond);
        assert_eq!(parse_duration_base("64").unwrap(), DurationBase::SixtyFourth);
        assert_eq!(parse_duration_base("128").unwrap(), DurationBase::OneTwentyEighth);
        assert_eq!(parse_duration_base("256").unwrap(), DurationBase::TwoFiftySixth);
        assert_eq!(parse_duration_base("512").unwrap(), DurationBase::FiveTwelfth);
        assert_eq!(parse_duration_base("1024").unwrap(), DurationBase::OneThousandTwentyFourth);
    }

    #[test]
    fn test_parse_duration_base_full_names() {
        assert_eq!(parse_duration_base("whole").unwrap(), DurationBase::Whole);
        assert_eq!(parse_duration_base("half").unwrap(), DurationBase::Half);
        assert_eq!(parse_duration_base("quarter").unwrap(), DurationBase::Quarter);
        assert_eq!(parse_duration_base("eighth").unwrap(), DurationBase::Eighth);
        assert_eq!(parse_duration_base("sixteenth").unwrap(), DurationBase::Sixteenth);
    }

    #[test]
    fn test_parse_duration_base_british_names() {
        assert_eq!(parse_duration_base("semibreve").unwrap(), DurationBase::Whole);
        assert_eq!(parse_duration_base("minim").unwrap(), DurationBase::Half);
        assert_eq!(parse_duration_base("crotchet").unwrap(), DurationBase::Quarter);
        assert_eq!(parse_duration_base("quaver").unwrap(), DurationBase::Eighth);
        assert_eq!(parse_duration_base("semiquaver").unwrap(), DurationBase::Sixteenth);
    }

    #[test]
    fn test_parse_duration_base_long_durations() {
        assert_eq!(parse_duration_base("breve").unwrap(), DurationBase::Breve);
        assert_eq!(parse_duration_base("long").unwrap(), DurationBase::Long);
        assert_eq!(parse_duration_base("maxima").unwrap(), DurationBase::Maxima);
    }

    #[test]
    fn test_parse_duration_base_case_insensitive() {
        assert_eq!(parse_duration_base("QUARTER").unwrap(), DurationBase::Quarter);
        assert_eq!(parse_duration_base("Quarter").unwrap(), DurationBase::Quarter);
        assert_eq!(parse_duration_base("WHOLE").unwrap(), DurationBase::Whole);
    }

    #[test]
    fn test_parse_duration_base_invalid() {
        assert!(parse_duration_base("invalid").is_err());
        assert!(parse_duration_base("").is_err());
        assert!(parse_duration_base("x").is_err());
    }

    // === parse_duration tests ===

    #[test]
    fn test_parse_duration_simple() {
        let dur = parse_duration("q").unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 0);
    }

    #[test]
    fn test_parse_duration_with_colon() {
        let dur = parse_duration(":q").unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 0);
    }

    #[test]
    fn test_parse_duration_dotted() {
        let dur = parse_duration("q.").unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 1);
    }

    #[test]
    fn test_parse_duration_double_dotted() {
        let dur = parse_duration("h..").unwrap();
        assert_eq!(dur.base, DurationBase::Half);
        assert_eq!(dur.dots, 2);
    }

    #[test]
    fn test_parse_duration_triple_dotted() {
        let dur = parse_duration("w...").unwrap();
        assert_eq!(dur.base, DurationBase::Whole);
        assert_eq!(dur.dots, 3);
    }

    #[test]
    fn test_parse_duration_full_name_dotted() {
        let dur = parse_duration("quarter.").unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 1);
    }

    #[test]
    fn test_parse_duration_british_name() {
        let dur = parse_duration("crotchet").unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
    }

    #[test]
    fn test_parse_duration_minim() {
        let dur = parse_duration("minim.").unwrap();
        assert_eq!(dur.base, DurationBase::Half);
        assert_eq!(dur.dots, 1);
    }

    #[test]
    fn test_parse_duration_empty() {
        assert!(parse_duration("").is_err());
    }

    #[test]
    fn test_parse_duration_only_dots() {
        assert!(parse_duration("...").is_err());
    }

    // === compile_duration_type tests ===

    #[test]
    fn test_compile_duration_type_quarter() {
        let note_type = compile_duration_type(&DurationBase::Quarter);
        assert_eq!(note_type.value, NoteTypeValue::Quarter);
        assert!(note_type.size.is_none());
    }

    #[test]
    fn test_compile_duration_type_half() {
        let note_type = compile_duration_type(&DurationBase::Half);
        assert_eq!(note_type.value, NoteTypeValue::Half);
    }

    #[test]
    fn test_compile_duration_type_whole() {
        let note_type = compile_duration_type(&DurationBase::Whole);
        assert_eq!(note_type.value, NoteTypeValue::Whole);
    }

    #[test]
    fn test_compile_duration_type_eighth() {
        let note_type = compile_duration_type(&DurationBase::Eighth);
        assert_eq!(note_type.value, NoteTypeValue::Eighth);
    }

    #[test]
    fn test_compile_duration_type_sixteenth() {
        let note_type = compile_duration_type(&DurationBase::Sixteenth);
        assert_eq!(note_type.value, NoteTypeValue::N16th);
    }

    #[test]
    fn test_compile_duration_type_all() {
        let cases = [
            (DurationBase::Maxima, NoteTypeValue::Maxima),
            (DurationBase::Long, NoteTypeValue::Long),
            (DurationBase::Breve, NoteTypeValue::Breve),
            (DurationBase::Whole, NoteTypeValue::Whole),
            (DurationBase::Half, NoteTypeValue::Half),
            (DurationBase::Quarter, NoteTypeValue::Quarter),
            (DurationBase::Eighth, NoteTypeValue::Eighth),
            (DurationBase::Sixteenth, NoteTypeValue::N16th),
            (DurationBase::ThirtySecond, NoteTypeValue::N32nd),
            (DurationBase::SixtyFourth, NoteTypeValue::N64th),
            (DurationBase::OneTwentyEighth, NoteTypeValue::N128th),
            (DurationBase::TwoFiftySixth, NoteTypeValue::N256th),
            (DurationBase::FiveTwelfth, NoteTypeValue::N512th),
            (DurationBase::OneThousandTwentyFourth, NoteTypeValue::N1024th),
        ];

        for (base, expected) in cases {
            let note_type = compile_duration_type(&base);
            assert_eq!(note_type.value, expected, "Failed for {:?}", base);
        }
    }

    // === compile_dots tests ===

    #[test]
    fn test_compile_dots_zero() {
        let dots = compile_dots(0);
        assert!(dots.is_empty());
    }

    #[test]
    fn test_compile_dots_one() {
        let dots = compile_dots(1);
        assert_eq!(dots.len(), 1);
        assert_eq!(dots[0], Dot::default());
    }

    #[test]
    fn test_compile_dots_two() {
        let dots = compile_dots(2);
        assert_eq!(dots.len(), 2);
    }

    #[test]
    fn test_compile_dots_three() {
        let dots = compile_dots(3);
        assert_eq!(dots.len(), 3);
    }

    // === compile_duration_divisions tests ===

    #[test]
    fn test_compile_duration_divisions_quarter() {
        let dur = FermataDuration {
            base: DurationBase::Quarter,
            dots: 0,
        };
        let divisions = compile_duration_divisions(&dur);
        // DEFAULT_DIVISIONS = 960, quarter = 960 divisions
        assert_eq!(divisions, DEFAULT_DIVISIONS as u64);
    }

    #[test]
    fn test_compile_duration_divisions_half() {
        let dur = FermataDuration {
            base: DurationBase::Half,
            dots: 0,
        };
        let divisions = compile_duration_divisions(&dur);
        // Half = 2 * quarter = 1920 divisions
        assert_eq!(divisions, DEFAULT_DIVISIONS as u64 * 2);
    }

    #[test]
    fn test_compile_duration_divisions_whole() {
        let dur = FermataDuration {
            base: DurationBase::Whole,
            dots: 0,
        };
        let divisions = compile_duration_divisions(&dur);
        // Whole = 4 * quarter = 3840 divisions
        assert_eq!(divisions, DEFAULT_DIVISIONS as u64 * 4);
    }

    #[test]
    fn test_compile_duration_divisions_dotted_quarter() {
        let dur = FermataDuration {
            base: DurationBase::Quarter,
            dots: 1,
        };
        // Dotted quarter = 1.5 * quarter = 1440 divisions
        // Actually with divisions=1, we need to increase resolution
        let divisions = compile_duration_divisions_with(&dur, 2);
        // Quarter = 2 divisions, dotted = 3 divisions
        assert_eq!(divisions, 3);
    }

    #[test]
    fn test_compile_duration_divisions_double_dotted_half() {
        let dur = FermataDuration {
            base: DurationBase::Half,
            dots: 2,
        };
        // Half = 2 quarters, with 2 dots: 2 + 1 + 0.5 = 3.5 quarters
        // With divisions=2: 7 divisions
        let divisions = compile_duration_divisions_with(&dur, 2);
        assert_eq!(divisions, 7);
    }

    #[test]
    fn test_compile_duration_divisions_eighth() {
        let dur = FermataDuration {
            base: DurationBase::Eighth,
            dots: 0,
        };
        // Eighth = 0.5 quarters, with divisions=2: 1 division
        let divisions = compile_duration_divisions_with(&dur, 2);
        assert_eq!(divisions, 1);
    }

    // === parse_duration_sexpr tests ===

    #[test]
    fn test_parse_duration_sexpr_keyword() {
        let sexpr = Sexpr::keyword("q");
        let dur = parse_duration_sexpr(&sexpr).unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 0);
    }

    #[test]
    fn test_parse_duration_sexpr_keyword_dotted() {
        let sexpr = Sexpr::keyword("q.");
        let dur = parse_duration_sexpr(&sexpr).unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 1);
    }

    #[test]
    fn test_parse_duration_sexpr_symbol() {
        let sexpr = Sexpr::symbol("h");
        let dur = parse_duration_sexpr(&sexpr).unwrap();
        assert_eq!(dur.base, DurationBase::Half);
    }

    #[test]
    fn test_parse_duration_sexpr_list() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("duration"),
            Sexpr::keyword("base"),
            Sexpr::symbol("quarter"),
            Sexpr::keyword("dots"),
            Sexpr::Integer(1),
        ]);
        let dur = parse_duration_sexpr(&sexpr).unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 1);
    }

    #[test]
    fn test_parse_duration_sexpr_list_numeric_base() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("duration"),
            Sexpr::keyword("base"),
            Sexpr::Integer(4),
            Sexpr::keyword("dots"),
            Sexpr::Integer(0),
        ]);
        let dur = parse_duration_sexpr(&sexpr).unwrap();
        assert_eq!(dur.base, DurationBase::Quarter);
    }

    #[test]
    fn test_parse_duration_sexpr_list_type_alias() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("duration"),
            Sexpr::keyword("type"),
            Sexpr::symbol("half"),
        ]);
        let dur = parse_duration_sexpr(&sexpr).unwrap();
        assert_eq!(dur.base, DurationBase::Half);
        assert_eq!(dur.dots, 0);
    }

    #[test]
    fn test_parse_duration_sexpr_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(parse_duration_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_duration_sexpr_missing_base() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("duration"),
            Sexpr::keyword("dots"),
            Sexpr::Integer(1),
        ]);
        assert!(parse_duration_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_duration_sexpr_invalid_type() {
        let sexpr = Sexpr::Integer(42);
        assert!(parse_duration_sexpr(&sexpr).is_err());
    }

    // === Additional tests ===

    #[test]
    fn test_parse_duration_all_short_forms() {
        let cases = [
            ("w", DurationBase::Whole),
            ("h", DurationBase::Half),
            ("q", DurationBase::Quarter),
            ("8", DurationBase::Eighth),
            ("16", DurationBase::Sixteenth),
            ("32", DurationBase::ThirtySecond),
            ("64", DurationBase::SixtyFourth),
            ("128", DurationBase::OneTwentyEighth),
            ("256", DurationBase::TwoFiftySixth),
            ("512", DurationBase::FiveTwelfth),
            ("1024", DurationBase::OneThousandTwentyFourth),
        ];

        for (input, expected) in cases {
            let dur = parse_duration(input).unwrap();
            assert_eq!(dur.base, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_parse_duration_with_keyword_prefix() {
        let cases = [
            (":q", DurationBase::Quarter),
            (":h", DurationBase::Half),
            (":w", DurationBase::Whole),
            (":8", DurationBase::Eighth),
            (":16", DurationBase::Sixteenth),
        ];

        for (input, expected) in cases {
            let dur = parse_duration(input).unwrap();
            assert_eq!(dur.base, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_compile_duration_divisions_with_high_resolution() {
        // Test with divisions=24 (common for complex rhythms)
        let dur = FermataDuration {
            base: DurationBase::Quarter,
            dots: 0,
        };
        let divisions = compile_duration_divisions_with(&dur, 24);
        assert_eq!(divisions, 24);

        // Eighth note
        let dur = FermataDuration {
            base: DurationBase::Eighth,
            dots: 0,
        };
        let divisions = compile_duration_divisions_with(&dur, 24);
        assert_eq!(divisions, 12);

        // Dotted quarter
        let dur = FermataDuration {
            base: DurationBase::Quarter,
            dots: 1,
        };
        let divisions = compile_duration_divisions_with(&dur, 24);
        assert_eq!(divisions, 36);
    }
}
