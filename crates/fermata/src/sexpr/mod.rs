//! S-expression read/print support for music scores.
//!
//! This module provides functionality to convert between the internal
//! representation (IR) of music scores and S-expression (Lisp-like) syntax.
//!
//! # Overview
//!
//! S-expressions provide a simple, uniform syntax for representing structured
//! data. This format is particularly well-suited for music notation because:
//!
//! - Hierarchical structure maps naturally to musical elements
//! - Human-readable and editable
//! - Easy to parse and generate
//! - Concise compared to XML
//!
//! # Modules
//!
//! - [`ast`] - Untyped S-expression AST and [`ListBuilder`]
//! - [`error`] - Error types for parsing and conversion
//! - [`parser`] - nom-based parser (text -> AST)
//! - [`traits`] - [`ToSexpr`] and [`FromSexpr`] conversion traits
//! - [`convert`] - IR type conversions
//!
//! # Parsing S-expressions
//!
//! ```
//! use fermata::sexpr::{parse, Sexpr};
//!
//! let sexpr = parse("(note :pitch C4)").unwrap();
//! assert!(sexpr.is_list());
//! ```
//!
//! # Building S-expressions
//!
//! ```
//! use fermata::sexpr::{ListBuilder, Sexpr};
//!
//! let sexpr = ListBuilder::new("pitch")
//!     .kwarg("step", &"C".to_string())
//!     .kwarg("octave", &4i32)
//!     .build();
//! ```
//!
//! # Converting IR Types
//!
//! ```
//! use fermata::ir::common::YesNo;
//! use fermata::sexpr::{ToSexpr, FromSexpr, Sexpr};
//!
//! let yes = YesNo::Yes;
//! let sexpr = yes.to_sexpr();
//! assert_eq!(sexpr, Sexpr::symbol("yes"));
//!
//! let parsed = YesNo::from_sexpr(&sexpr).unwrap();
//! assert_eq!(parsed, YesNo::Yes);
//! ```
//!
//! # Output Format
//!
//! The S-expression output uses the following conventions:
//!
//! - **Type names** become kebab-case symbols: `ScorePartwise` -> `score-partwise`
//! - **Field names** become keywords: `part_name` -> `:part-name`
//! - **Booleans** use Lisp convention: `#t` / `#f` or `t` / `nil`
//! - **Strings** are quoted: `"Piano"`
//! - **Optional fields** are omitted when `None`
//! - **Empty vectors** are omitted
//!
//! # Pretty Printing
//!
//! Use [`PrintOptions`] to control formatting:
//!
//! ```
//! use fermata::sexpr::PrintOptions;
//!
//! // Default: 2-space indent, 80 char width
//! let pretty = PrintOptions::default();
//!
//! // Compact: minimal whitespace
//! let compact = PrintOptions {
//!     compact: true,
//!     ..Default::default()
//! };
//!
//! // Custom indent
//! let tabs = PrintOptions {
//!     indent: "\t".to_string(),
//!     ..Default::default()
//! };
//! ```

mod ast;
pub mod convert;
pub mod error;
pub mod parser;
mod printer;
pub mod traits;

// Re-export core types
pub use ast::{ListBuilder, Sexpr};
pub use error::{ConvertError, ConvertResult, ParseError, ParseResult};
pub use parser::{parse, parse_all};
pub use traits::{FromSexpr, ToSexpr};

// Note: print_sexpr and related functions are defined later in this file
// and are public functions in the crate::sexpr module.

use crate::ir::ScorePartwise;

/// Convenience function: Convert an IR value to an S-expression string.
///
/// This converts the value to an S-expression AST and then formats it
/// as a string using the default pretty-printer settings.
///
/// # Examples
///
/// ```
/// use fermata::ir::common::YesNo;
/// use fermata::sexpr::to_sexpr_string;
///
/// let s = to_sexpr_string(&YesNo::Yes);
/// assert_eq!(s, "yes");
/// ```
pub fn to_sexpr_string<T: ToSexpr>(value: &T) -> String {
    print_sexpr(&value.to_sexpr())
}

/// Print an S-expression AST to a string.
///
/// This function converts the [`Sexpr`] AST to a formatted string.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, print_sexpr};
///
/// let sexpr = Sexpr::list(vec![
///     Sexpr::symbol("note"),
///     Sexpr::keyword("pitch"),
///     Sexpr::symbol("C4"),
/// ]);
/// let s = print_sexpr(&sexpr);
/// assert_eq!(s, "(note :pitch C4)");
/// ```
pub fn print_sexpr(sexpr: &Sexpr) -> String {
    print_sexpr_internal(sexpr)
}

fn print_sexpr_internal(sexpr: &Sexpr) -> String {
    match sexpr {
        Sexpr::Symbol(s) => s.clone(),
        Sexpr::Keyword(k) => format!(":{}", k),
        Sexpr::String(s) => format!("\"{}\"", escape_string(s)),
        Sexpr::Integer(i) => i.to_string(),
        Sexpr::Float(f) => format_float(*f),
        Sexpr::Bool(true) => "#t".to_string(),
        Sexpr::Bool(false) => "#f".to_string(),
        Sexpr::Nil => "nil".to_string(),
        Sexpr::List(items) => {
            if items.is_empty() {
                "()".to_string()
            } else {
                let inner: Vec<String> = items.iter().map(print_sexpr_internal).collect();
                format!("({})", inner.join(" "))
            }
        }
    }
}

fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

fn format_float(f: f64) -> String {
    if f.fract() == 0.0 {
        format!("{}.0", f as i64)
    } else {
        format!("{}", f)
    }
}

/// Convenience function: Parse an S-expression string into an IR value.
///
/// This parses the string into an S-expression AST and then converts it
/// to the target IR type.
///
/// # Examples
///
/// ```
/// use fermata::ir::common::YesNo;
/// use fermata::sexpr::from_sexpr_str;
///
/// let value: YesNo = from_sexpr_str("yes").unwrap();
/// assert_eq!(value, YesNo::Yes);
/// ```
///
/// # Errors
///
/// Returns an error if parsing fails or if the S-expression cannot be
/// converted to the target type.
pub fn from_sexpr_str<T: FromSexpr>(input: &str) -> Result<T, Box<dyn std::error::Error>> {
    let sexpr = parse(input)?;
    T::from_sexpr(&sexpr).map_err(Into::into)
}

/// Formatting options for S-expression output.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::PrintOptions;
///
/// // Default options: 2-space indent, 80 char max width
/// let default = PrintOptions::default();
///
/// // Compact mode: minimal whitespace
/// let compact = PrintOptions {
///     compact: true,
///     ..Default::default()
/// };
///
/// // Custom indentation
/// let custom = PrintOptions {
///     indent: "    ".to_string(), // 4 spaces
///     max_width: 100,
///     compact: false,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct PrintOptions {
    /// Indentation string (default: two spaces).
    ///
    /// This string is repeated for each nesting level.
    pub indent: String,

    /// Maximum line width before wrapping (default: 80).
    ///
    /// Note: This is a hint; some elements may exceed this width.
    pub max_width: usize,

    /// Print compact (minimal whitespace).
    ///
    /// When true, newlines and indentation are replaced with single spaces.
    pub compact: bool,
}

impl Default for PrintOptions {
    fn default() -> Self {
        Self {
            indent: "  ".to_string(),
            max_width: 80,
            compact: false,
        }
    }
}

/// Print an IR score as an S-expression string.
///
/// This uses default formatting options (2-space indent, 80 char width).
///
/// # Arguments
///
/// * `score` - The score to print
///
/// # Returns
///
/// A formatted S-expression string representing the score.
///
/// # Examples
///
/// ```ignore
/// use fermata::ir::{ScorePartwise, PartList, Part};
/// use fermata::sexpr::print_score;
///
/// let score = ScorePartwise { /* ... */ };
/// let sexpr = print_score(&score);
/// assert!(sexpr.starts_with("(score-partwise"));
/// ```
pub fn print_score(score: &ScorePartwise) -> String {
    printer::print_score(score, &PrintOptions::default())
}

/// Print an IR score with custom formatting options.
///
/// # Arguments
///
/// * `score` - The score to print
/// * `options` - Formatting options (indentation, width, compact mode)
///
/// # Returns
///
/// A formatted S-expression string representing the score.
///
/// # Examples
///
/// ```ignore
/// use fermata::ir::{ScorePartwise, PartList, Part};
/// use fermata::sexpr::{print_score_with_options, PrintOptions};
///
/// let score = ScorePartwise { /* ... */ };
/// let options = PrintOptions { compact: true, ..Default::default() };
/// let sexpr = print_score_with_options(&score, &options);
/// assert!(!sexpr.contains('\n'));
/// ```
pub fn print_score_with_options(score: &ScorePartwise, options: &PrintOptions) -> String {
    printer::print_score(score, options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{
        common::PrintStyle,
        part::{Part, PartList, PartListElement, PartName, ScorePart},
        measure::Measure,
        score::ScorePartwise,
    };

    // Helper function to create a minimal score for testing
    fn minimal_score() -> ScorePartwise {
        ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        }
    }

    // Helper function to create a score with a part
    fn score_with_part() -> ScorePartwise {
        ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList {
                content: vec![PartListElement::ScorePart(ScorePart {
                    id: "P1".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Piano".to_string(),
                        print_style: PrintStyle::default(),
                        print_object: None,
                        justify: None,
                    },
                    part_name_display: None,
                    part_abbreviation: None,
                    part_abbreviation_display: None,
                    group: vec![],
                    score_instruments: vec![],
                    midi_devices: vec![],
                    midi_instruments: vec![],
                })],
            },
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![],
            }],
        }
    }

    // Helper function to create a score with a measure
    fn score_with_measure() -> ScorePartwise {
        ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList {
                content: vec![PartListElement::ScorePart(ScorePart {
                    id: "P1".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Piano".to_string(),
                        print_style: PrintStyle::default(),
                        print_object: None,
                        justify: None,
                    },
                    part_name_display: None,
                    part_abbreviation: None,
                    part_abbreviation_display: None,
                    group: vec![],
                    score_instruments: vec![],
                    midi_devices: vec![],
                    midi_instruments: vec![],
                })],
            },
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![Measure {
                    number: "1".to_string(),
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    content: vec![],
                }],
            }],
        }
    }

    // === PrintOptions Tests ===

    #[test]
    fn test_print_options_default() {
        let options = PrintOptions::default();
        assert_eq!(options.indent, "  ");
        assert_eq!(options.max_width, 80);
        assert!(!options.compact);
    }

    #[test]
    fn test_print_options_clone() {
        let options = PrintOptions {
            indent: "\t".to_string(),
            max_width: 120,
            compact: true,
        };
        let cloned = options.clone();
        assert_eq!(options.indent, cloned.indent);
        assert_eq!(options.max_width, cloned.max_width);
        assert_eq!(options.compact, cloned.compact);
    }

    #[test]
    fn test_print_options_debug() {
        let options = PrintOptions::default();
        let debug = format!("{:?}", options);
        assert!(debug.contains("PrintOptions"));
        assert!(debug.contains("indent"));
        assert!(debug.contains("max_width"));
        assert!(debug.contains("compact"));
    }

    // === print_score Tests ===

    #[test]
    fn test_print_score_minimal() {
        let score = minimal_score();
        let sexpr = print_score(&score);

        assert!(sexpr.starts_with("(score-partwise"));
        assert!(sexpr.contains(":version \"4.0\""));
        assert!(sexpr.ends_with(')'));
    }

    #[test]
    fn test_print_score_with_part_list() {
        let score = score_with_part();
        let sexpr = print_score(&score);

        assert!(sexpr.contains("(part-list"));
        assert!(sexpr.contains("(score-part :id \"P1\""));
        assert!(sexpr.contains("(part-name \"Piano\")"));
        assert!(sexpr.contains("(part :id \"P1\""));
    }

    #[test]
    fn test_print_score_with_measure() {
        let score = score_with_measure();
        let sexpr = print_score(&score);

        assert!(sexpr.contains("(measure :number \"1\""));
    }

    // === print_score_with_options Tests ===

    #[test]
    fn test_print_score_compact() {
        let score = score_with_part();

        let options = PrintOptions {
            compact: true,
            ..Default::default()
        };

        let sexpr = print_score_with_options(&score, &options);

        // Compact mode should not have newlines
        assert!(!sexpr.contains('\n'));
        // But should still have all the content
        assert!(sexpr.contains("(score-partwise"));
        assert!(sexpr.contains(":version \"4.0\""));
    }

    #[test]
    fn test_print_score_pretty() {
        let score = score_with_part();

        let options = PrintOptions::default();
        let sexpr = print_score_with_options(&score, &options);

        // Pretty mode should have newlines
        assert!(sexpr.contains('\n'));
        // And proper indentation
        assert!(sexpr.contains("  (part-list"));
    }

    #[test]
    fn test_print_score_custom_indent() {
        let score = score_with_part();

        let options = PrintOptions {
            indent: "\t".to_string(),
            ..Default::default()
        };

        let sexpr = print_score_with_options(&score, &options);

        // Should use tabs for indentation (printer uses both newline_indent + function indent)
        assert!(sexpr.contains("\t(part-list"));
        // Verify the output uses tabs, not spaces, for indentation
        assert!(sexpr.contains('\t'));
        assert!(!sexpr.lines().skip(1).any(|l| l.starts_with("  ")));
    }

    // === Balanced Parentheses Tests ===

    #[test]
    fn test_balanced_parens_minimal() {
        let score = minimal_score();
        let sexpr = print_score(&score);

        let open = sexpr.chars().filter(|&c| c == '(').count();
        let close = sexpr.chars().filter(|&c| c == ')').count();
        assert_eq!(open, close, "Parentheses should be balanced");
    }

    #[test]
    fn test_balanced_parens_with_content() {
        let score = score_with_measure();
        let sexpr = print_score(&score);

        let open = sexpr.chars().filter(|&c| c == '(').count();
        let close = sexpr.chars().filter(|&c| c == ')').count();
        assert_eq!(open, close, "Parentheses should be balanced");
    }

    // === Sexpr Re-export Tests ===

    #[test]
    fn test_sexpr_reexport() {
        // Verify Sexpr is accessible through the module
        let sym = Sexpr::symbol("note");
        assert!(sym.is_symbol("note"));

        let kw = Sexpr::keyword("step");
        assert!(kw.is_keyword("step"));

        let list = Sexpr::list(vec![sym, kw]);
        assert!(list.is_list());
    }

    // === print_sexpr Tests ===

    #[test]
    fn test_print_sexpr_symbol() {
        let sexpr = Sexpr::symbol("foo");
        assert_eq!(print_sexpr(&sexpr), "foo");
    }

    #[test]
    fn test_print_sexpr_keyword() {
        let sexpr = Sexpr::keyword("step");
        assert_eq!(print_sexpr(&sexpr), ":step");
    }

    #[test]
    fn test_print_sexpr_string() {
        let sexpr = Sexpr::String("hello".to_string());
        assert_eq!(print_sexpr(&sexpr), "\"hello\"");
    }

    #[test]
    fn test_print_sexpr_string_with_escapes() {
        let sexpr = Sexpr::String("line\nbreak".to_string());
        assert_eq!(print_sexpr(&sexpr), "\"line\\nbreak\"");
    }

    #[test]
    fn test_print_sexpr_integer() {
        let sexpr = Sexpr::Integer(42);
        assert_eq!(print_sexpr(&sexpr), "42");
    }

    #[test]
    fn test_print_sexpr_float() {
        let sexpr = Sexpr::Float(3.14);
        assert_eq!(print_sexpr(&sexpr), "3.14");
    }

    #[test]
    fn test_print_sexpr_float_whole() {
        let sexpr = Sexpr::Float(5.0);
        assert_eq!(print_sexpr(&sexpr), "5.0");
    }

    #[test]
    fn test_print_sexpr_bool_true() {
        let sexpr = Sexpr::Bool(true);
        assert_eq!(print_sexpr(&sexpr), "#t");
    }

    #[test]
    fn test_print_sexpr_bool_false() {
        let sexpr = Sexpr::Bool(false);
        assert_eq!(print_sexpr(&sexpr), "#f");
    }

    #[test]
    fn test_print_sexpr_nil() {
        let sexpr = Sexpr::Nil;
        assert_eq!(print_sexpr(&sexpr), "nil");
    }

    #[test]
    fn test_print_sexpr_empty_list() {
        let sexpr = Sexpr::List(vec![]);
        assert_eq!(print_sexpr(&sexpr), "()");
    }

    #[test]
    fn test_print_sexpr_simple_list() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::keyword("pitch"),
            Sexpr::symbol("C4"),
        ]);
        assert_eq!(print_sexpr(&sexpr), "(note :pitch C4)");
    }

    #[test]
    fn test_print_sexpr_nested_list() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::keyword("pitch"),
            Sexpr::list(vec![
                Sexpr::symbol("pitch"),
                Sexpr::keyword("step"),
                Sexpr::symbol("C"),
            ]),
        ]);
        assert_eq!(print_sexpr(&sexpr), "(note :pitch (pitch :step C))");
    }

    // === to_sexpr_string Tests ===

    #[test]
    fn test_to_sexpr_string_yesno() {
        use crate::ir::common::YesNo;
        assert_eq!(to_sexpr_string(&YesNo::Yes), "yes");
        assert_eq!(to_sexpr_string(&YesNo::No), "no");
    }

    // === from_sexpr_str Tests ===

    #[test]
    fn test_from_sexpr_str_yesno() {
        use crate::ir::common::YesNo;
        let yes: YesNo = from_sexpr_str("yes").unwrap();
        assert_eq!(yes, YesNo::Yes);
    }

    #[test]
    fn test_from_sexpr_str_parse_error() {
        use crate::ir::common::YesNo;
        let result: Result<YesNo, _> = from_sexpr_str("(unclosed");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_sexpr_str_convert_error() {
        use crate::ir::common::YesNo;
        let result: Result<YesNo, _> = from_sexpr_str("maybe");
        assert!(result.is_err());
    }

    // === Round Trip Tests ===

    #[test]
    fn test_round_trip_via_string() {
        use crate::ir::common::StartStop;
        let original = StartStop::Start;
        let s = to_sexpr_string(&original);
        let parsed: StartStop = from_sexpr_str(&s).unwrap();
        assert_eq!(original, parsed);
    }
}
