//! Error types for S-expression parsing and conversion.
//!
//! This module defines the error types used throughout the sexpr module:
//!
//! - [`ParseError`] - Errors that occur during text-to-AST parsing
//! - [`ConvertError`] - Errors that occur during AST-to-IR conversion

use thiserror::Error;

use super::ast::Sexpr;

/// Errors that can occur during S-expression parsing.
///
/// These errors are produced by the nom-based parser when converting
/// text input into the untyped [`Sexpr`] AST.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParseError {
    /// Reached end of input when more content was expected.
    #[error("Unexpected end of input")]
    UnexpectedEof,

    /// Encountered an unexpected character during parsing.
    #[error("Unexpected character: '{0}'")]
    UnexpectedChar(char),

    /// A string literal was not properly closed.
    #[error("Unclosed string literal")]
    UnclosedString,

    /// A list was not properly closed with ')'.
    #[error("Unclosed list (missing ')')")]
    UnclosedList,

    /// An invalid escape sequence was found in a string.
    #[error("Invalid escape sequence: \\{0}")]
    InvalidEscape(char),

    /// A general parse error from nom.
    #[error("Parse error: {0}")]
    Nom(String),

    /// Trailing content after a complete expression.
    #[error("Unexpected trailing content: {0}")]
    TrailingContent(String),
}

/// Errors that can occur during IR conversion.
///
/// These errors are produced when converting between the untyped [`Sexpr`] AST
/// and typed IR structures via the [`ToSexpr`](super::traits::ToSexpr) and
/// [`FromSexpr`](super::traits::FromSexpr) traits.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ConvertError {
    /// Expected a different type of S-expression.
    #[error("Expected {expected}, found {found}")]
    TypeMismatch {
        /// The expected type description
        expected: &'static str,
        /// A description of what was actually found
        found: String,
    },

    /// A required field was not present.
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// An unknown keyword was encountered.
    #[error("Unknown keyword: :{0}")]
    UnknownKeyword(String),

    /// A value was invalid for the given field.
    #[error("Invalid value for {field}: {value}")]
    InvalidValue {
        /// The field name
        field: &'static str,
        /// The invalid value as a string
        value: String,
    },

    /// Expected a list with a specific head symbol.
    #[error("Expected list with head '{0}'")]
    ExpectedHead(&'static str),

    /// An invalid enum variant was specified.
    #[error("Invalid enum variant: {0}")]
    InvalidVariant(String),

    /// An error with additional context.
    #[error("{context}: {source}")]
    Context {
        /// Description of where the error occurred
        context: String,
        /// The underlying error
        #[source]
        source: Box<ConvertError>,
    },
}

impl ConvertError {
    /// Create a type mismatch error from an expected type and actual Sexpr.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::{Sexpr, ConvertError};
    ///
    /// let sexpr = Sexpr::Integer(42);
    /// let err = ConvertError::type_mismatch("string", &sexpr);
    /// assert!(err.to_string().contains("Expected string"));
    /// ```
    pub fn type_mismatch(expected: &'static str, found: &Sexpr) -> Self {
        Self::TypeMismatch {
            expected,
            found: describe_sexpr(found),
        }
    }

    /// Wrap this error with additional context.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ConvertError;
    ///
    /// let err = ConvertError::MissingField("step");
    /// let with_context = err.with_context("parsing Pitch");
    /// assert!(with_context.to_string().contains("parsing Pitch"));
    /// ```
    pub fn with_context(self, context: impl Into<String>) -> Self {
        Self::Context {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Create an invalid value error.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ConvertError;
    ///
    /// let err = ConvertError::invalid_value("step", "X");
    /// assert!(err.to_string().contains("Invalid value for step: X"));
    /// ```
    pub fn invalid_value(field: &'static str, value: impl Into<String>) -> Self {
        Self::InvalidValue {
            field,
            value: value.into(),
        }
    }
}

/// Describe an S-expression for error messages.
fn describe_sexpr(sexpr: &Sexpr) -> String {
    match sexpr {
        Sexpr::Symbol(s) => format!("symbol '{}'", s),
        Sexpr::Keyword(k) => format!("keyword ':{}'", k),
        Sexpr::String(s) => {
            if s.len() > 20 {
                format!("string \"{}...\"", &s[..17])
            } else {
                format!("string \"{}\"", s)
            }
        }
        Sexpr::Integer(i) => format!("integer {}", i),
        Sexpr::Float(f) => format!("float {}", f),
        Sexpr::Bool(b) => format!("boolean {}", if *b { "#t" } else { "#f" }),
        Sexpr::Nil => "nil".to_string(),
        Sexpr::List(items) => {
            if items.is_empty() {
                "empty list ()".to_string()
            } else if let Some(head) = items.first().and_then(|s| s.as_symbol()) {
                format!("list with head '{}'", head)
            } else {
                format!("list with {} items", items.len())
            }
        }
    }
}

/// Result type for parsing operations.
pub type ParseResult<T> = Result<T, ParseError>;

/// Result type for conversion operations.
pub type ConvertResult<T> = Result<T, ConvertError>;

#[cfg(test)]
mod tests {
    use super::*;

    // === ParseError Tests ===

    #[test]
    fn test_parse_error_unexpected_eof_display() {
        let err = ParseError::UnexpectedEof;
        assert_eq!(err.to_string(), "Unexpected end of input");
    }

    #[test]
    fn test_parse_error_unexpected_char_display() {
        let err = ParseError::UnexpectedChar('@');
        assert_eq!(err.to_string(), "Unexpected character: '@'");
    }

    #[test]
    fn test_parse_error_unclosed_string_display() {
        let err = ParseError::UnclosedString;
        assert_eq!(err.to_string(), "Unclosed string literal");
    }

    #[test]
    fn test_parse_error_unclosed_list_display() {
        let err = ParseError::UnclosedList;
        assert_eq!(err.to_string(), "Unclosed list (missing ')')");
    }

    #[test]
    fn test_parse_error_invalid_escape_display() {
        let err = ParseError::InvalidEscape('x');
        assert_eq!(err.to_string(), "Invalid escape sequence: \\x");
    }

    #[test]
    fn test_parse_error_nom_display() {
        let err = ParseError::Nom("some nom error".to_string());
        assert_eq!(err.to_string(), "Parse error: some nom error");
    }

    #[test]
    fn test_parse_error_trailing_content_display() {
        let err = ParseError::TrailingContent("extra stuff".to_string());
        assert_eq!(err.to_string(), "Unexpected trailing content: extra stuff");
    }

    #[test]
    fn test_parse_error_clone() {
        let err = ParseError::UnexpectedChar('!');
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    // === ConvertError Tests ===

    #[test]
    fn test_convert_error_type_mismatch_symbol() {
        let sexpr = Sexpr::Symbol("foo".to_string());
        let err = ConvertError::type_mismatch("string", &sexpr);
        let msg = err.to_string();
        assert!(msg.contains("Expected string"));
        assert!(msg.contains("symbol 'foo'"));
    }

    #[test]
    fn test_convert_error_type_mismatch_keyword() {
        let sexpr = Sexpr::Keyword("bar".to_string());
        let err = ConvertError::type_mismatch("list", &sexpr);
        let msg = err.to_string();
        assert!(msg.contains("keyword ':bar'"));
    }

    #[test]
    fn test_convert_error_type_mismatch_string() {
        let sexpr = Sexpr::String("hello world".to_string());
        let err = ConvertError::type_mismatch("integer", &sexpr);
        let msg = err.to_string();
        assert!(msg.contains("string \"hello world\""));
    }

    #[test]
    fn test_convert_error_type_mismatch_long_string() {
        let sexpr =
            Sexpr::String("this is a very long string that should be truncated".to_string());
        let err = ConvertError::type_mismatch("integer", &sexpr);
        let msg = err.to_string();
        assert!(msg.contains("..."));
    }

    #[test]
    fn test_convert_error_type_mismatch_integer() {
        let sexpr = Sexpr::Integer(42);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("integer 42"));
    }

    #[test]
    fn test_convert_error_type_mismatch_float() {
        let sexpr = Sexpr::Float(3.14);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("float 3.14"));
    }

    #[test]
    fn test_convert_error_type_mismatch_bool_true() {
        let sexpr = Sexpr::Bool(true);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("boolean #t"));
    }

    #[test]
    fn test_convert_error_type_mismatch_bool_false() {
        let sexpr = Sexpr::Bool(false);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("boolean #f"));
    }

    #[test]
    fn test_convert_error_type_mismatch_nil() {
        let sexpr = Sexpr::Nil;
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("nil"));
    }

    #[test]
    fn test_convert_error_type_mismatch_empty_list() {
        let sexpr = Sexpr::List(vec![]);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("empty list ()"));
    }

    #[test]
    fn test_convert_error_type_mismatch_list_with_head() {
        let sexpr = Sexpr::List(vec![Sexpr::Symbol("note".to_string())]);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("list with head 'note'"));
    }

    #[test]
    fn test_convert_error_type_mismatch_list_without_symbol_head() {
        let sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(2)]);
        let err = ConvertError::type_mismatch("string", &sexpr);
        assert!(err.to_string().contains("list with 2 items"));
    }

    #[test]
    fn test_convert_error_missing_field_display() {
        let err = ConvertError::MissingField("step");
        assert_eq!(err.to_string(), "Missing required field: step");
    }

    #[test]
    fn test_convert_error_unknown_keyword_display() {
        let err = ConvertError::UnknownKeyword("foo".to_string());
        assert_eq!(err.to_string(), "Unknown keyword: :foo");
    }

    #[test]
    fn test_convert_error_invalid_value_display() {
        let err = ConvertError::InvalidValue {
            field: "octave",
            value: "eleven".to_string(),
        };
        assert_eq!(err.to_string(), "Invalid value for octave: eleven");
    }

    #[test]
    fn test_convert_error_invalid_value_helper() {
        let err = ConvertError::invalid_value("step", "X");
        assert_eq!(err.to_string(), "Invalid value for step: X");
    }

    #[test]
    fn test_convert_error_expected_head_display() {
        let err = ConvertError::ExpectedHead("pitch");
        assert_eq!(err.to_string(), "Expected list with head 'pitch'");
    }

    #[test]
    fn test_convert_error_invalid_variant_display() {
        let err = ConvertError::InvalidVariant("unknown".to_string());
        assert_eq!(err.to_string(), "Invalid enum variant: unknown");
    }

    #[test]
    fn test_convert_error_with_context() {
        let inner = ConvertError::MissingField("step");
        let err = inner.with_context("parsing Pitch");
        let msg = err.to_string();
        assert!(msg.contains("parsing Pitch"));
        assert!(msg.contains("Missing required field: step"));
    }

    #[test]
    fn test_convert_error_clone() {
        let err = ConvertError::MissingField("step");
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_convert_error_context_clone() {
        let inner = ConvertError::MissingField("step");
        let err = inner.with_context("test");
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }
}
