//! Compilation error types for Fermata syntax.

use thiserror::Error;

use crate::sexpr::error::{ConvertError, ParseError};

/// Source location for error reporting
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SourceSpan {
    /// Starting byte offset in the source
    pub start: usize,
    /// Ending byte offset in the source
    pub end: usize,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

impl SourceSpan {
    /// Create a new span with byte offsets
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            line: 0,
            column: 0,
        }
    }

    /// Compute line/column from source text
    pub fn with_source(mut self, source: &str) -> Self {
        let mut line = 1;
        let mut column = 1;
        for (i, ch) in source.char_indices() {
            if i >= self.start {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        self.line = line;
        self.column = column;
        self
    }
}

/// Errors that can occur during Fermata compilation
#[derive(Debug, Error)]
pub enum CompileError {
    /// Parse error from the S-expression parser
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    /// Invalid pitch specification
    #[error("Invalid pitch: {0}")]
    InvalidPitch(String),

    /// Invalid duration specification
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    /// Invalid note specification
    #[error("Invalid note: {reason}")]
    InvalidNote {
        /// Description of why the note is invalid
        reason: String,
    },

    /// Invalid chord specification
    #[error("Invalid chord: {reason}")]
    InvalidChord {
        /// Description of why the chord is invalid
        reason: String,
    },

    /// Invalid tuplet specification
    #[error("Invalid tuplet: {reason}")]
    InvalidTuplet {
        /// Description of why the tuplet is invalid
        reason: String,
    },

    /// Invalid key signature
    #[error("Invalid key signature: {0}")]
    InvalidKey(String),

    /// Invalid time signature
    #[error("Invalid time signature: {0}")]
    InvalidTime(String),

    /// Invalid clef specification
    #[error("Invalid clef: {0}")]
    InvalidClef(String),

    /// Invalid dynamic marking
    #[error("Invalid dynamic: {0}")]
    InvalidDynamic(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// Unknown form (unrecognized S-expression structure)
    #[error("Unknown form: {0}")]
    UnknownForm(String),

    /// Type mismatch in S-expression
    #[error("Expected {expected}, found {found}")]
    TypeMismatch {
        /// What type was expected
        expected: &'static str,
        /// Description of what was actually found
        found: String,
    },

    /// Error during IR conversion
    #[error("IR conversion error: {0}")]
    IrConvert(#[from] ConvertError),

    /// Error with source span information attached
    #[error("{message}")]
    WithSpan {
        /// The error message
        message: String,
        /// Source location where the error occurred
        span: SourceSpan,
        /// The underlying error
        #[source]
        source: Box<CompileError>,
    },
}

impl CompileError {
    /// Attach source span information to this error
    pub fn with_span(self, span: SourceSpan) -> Self {
        CompileError::WithSpan {
            message: self.to_string(),
            span,
            source: Box::new(self),
        }
    }

    /// Create a type mismatch error
    pub fn type_mismatch(expected: &'static str, found: impl AsRef<str>) -> Self {
        CompileError::TypeMismatch {
            expected,
            found: found.as_ref().to_string(),
        }
    }
}

/// Result type for compilation operations
pub type CompileResult<T> = Result<T, CompileError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_error_invalid_pitch_display() {
        let err = CompileError::InvalidPitch("xyz".to_string());
        assert!(err.to_string().contains("xyz"));
    }

    #[test]
    fn test_compile_error_type_mismatch_display() {
        let err = CompileError::type_mismatch("pitch", "list");
        assert!(err.to_string().contains("pitch"));
        assert!(err.to_string().contains("list"));
    }

    #[test]
    fn test_compile_error_invalid_note_display() {
        let err = CompileError::InvalidNote {
            reason: "missing pitch".to_string(),
        };
        assert!(err.to_string().contains("missing pitch"));
    }

    #[test]
    fn test_compile_error_invalid_chord_display() {
        let err = CompileError::InvalidChord {
            reason: "no pitches".to_string(),
        };
        assert!(err.to_string().contains("no pitches"));
    }

    #[test]
    fn test_compile_error_invalid_tuplet_display() {
        let err = CompileError::InvalidTuplet {
            reason: "invalid ratio".to_string(),
        };
        assert!(err.to_string().contains("invalid ratio"));
    }

    #[test]
    fn test_compile_error_invalid_key_display() {
        let err = CompileError::InvalidKey("X major".to_string());
        assert!(err.to_string().contains("X major"));
    }

    #[test]
    fn test_compile_error_invalid_time_display() {
        let err = CompileError::InvalidTime("5/0".to_string());
        assert!(err.to_string().contains("5/0"));
    }

    #[test]
    fn test_compile_error_invalid_clef_display() {
        let err = CompileError::InvalidClef("unknown".to_string());
        assert!(err.to_string().contains("unknown"));
    }

    #[test]
    fn test_compile_error_invalid_dynamic_display() {
        let err = CompileError::InvalidDynamic("xxx".to_string());
        assert!(err.to_string().contains("xxx"));
    }

    #[test]
    fn test_compile_error_missing_field_display() {
        let err = CompileError::MissingField("pitch");
        assert!(err.to_string().contains("pitch"));
    }

    #[test]
    fn test_compile_error_unknown_form_display() {
        let err = CompileError::UnknownForm("(unknown-form)".to_string());
        assert!(err.to_string().contains("unknown-form"));
    }

    #[test]
    fn test_source_span_new() {
        let span = SourceSpan::new(5, 10);
        assert_eq!(span.start, 5);
        assert_eq!(span.end, 10);
        assert_eq!(span.line, 0);
        assert_eq!(span.column, 0);
    }

    #[test]
    fn test_source_span_default() {
        let span = SourceSpan::default();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 0);
        assert_eq!(span.line, 0);
        assert_eq!(span.column, 0);
    }

    #[test]
    fn test_source_span_with_source_first_line() {
        let source = "hello world";
        let span = SourceSpan::new(6, 11).with_source(source);
        assert_eq!(span.line, 1);
        assert_eq!(span.column, 7);
    }

    #[test]
    fn test_source_span_with_source_second_line() {
        let source = "line1\nline2\nline3";
        let span = SourceSpan::new(7, 12).with_source(source);
        assert_eq!(span.line, 2);
        assert_eq!(span.column, 2);
    }

    #[test]
    fn test_source_span_with_source_third_line() {
        let source = "a\nb\nc";
        let span = SourceSpan::new(4, 5).with_source(source);
        assert_eq!(span.line, 3);
        assert_eq!(span.column, 1);
    }

    #[test]
    fn test_source_span_with_source_at_newline() {
        let source = "line1\nline2";
        let span = SourceSpan::new(5, 6).with_source(source);
        assert_eq!(span.line, 1);
        assert_eq!(span.column, 6);
    }

    #[test]
    fn test_source_span_clone() {
        let span = SourceSpan::new(10, 20);
        let cloned = span.clone();
        assert_eq!(span, cloned);
    }

    #[test]
    fn test_compile_error_with_span() {
        let err = CompileError::InvalidPitch("bad".to_string());
        let span = SourceSpan::new(5, 10);
        let err_with_span = err.with_span(span);

        if let CompileError::WithSpan { span, .. } = err_with_span {
            assert_eq!(span.start, 5);
            assert_eq!(span.end, 10);
        } else {
            panic!("Expected WithSpan variant");
        }
    }

    #[test]
    fn test_compile_error_with_span_preserves_message() {
        let err = CompileError::InvalidPitch("test".to_string());
        let span = SourceSpan::new(0, 5);
        let err_with_span = err.with_span(span);
        assert!(err_with_span.to_string().contains("test"));
    }
}
