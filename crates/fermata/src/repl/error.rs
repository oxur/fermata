//! Error types for the Fermata REPL.

use thiserror::Error;

use crate::lang::error::CompileError;

/// Errors that can occur in the REPL.
#[derive(Debug, Error)]
pub enum ReplError {
    /// Error from the line editor (reedline).
    #[error("Line editor error: {0}")]
    Reedline(String),

    /// Compilation error from Fermata language.
    #[error("{0}")]
    Compile(#[from] CompileError),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic message error.
    #[error("{0}")]
    Message(String),
}

impl ReplError {
    /// Create a message error.
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }

    /// Create a reedline error.
    pub fn reedline(msg: impl Into<String>) -> Self {
        Self::Reedline(msg.into())
    }
}

/// Result type for REPL operations.
pub type ReplResult<T> = Result<T, ReplError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_error_message_display() {
        let err = ReplError::message("test error");
        assert_eq!(err.to_string(), "test error");
    }

    #[test]
    fn test_repl_error_reedline_display() {
        let err = ReplError::reedline("editor failed");
        assert!(err.to_string().contains("editor failed"));
    }

    #[test]
    fn test_repl_error_io_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let repl_err: ReplError = io_err.into();
        assert!(repl_err.to_string().contains("file not found"));
    }

    #[test]
    fn test_repl_error_compile_from() {
        let compile_err = CompileError::InvalidPitch("xyz".to_string());
        let repl_err: ReplError = compile_err.into();
        assert!(repl_err.to_string().contains("xyz"));
    }

    #[test]
    fn test_repl_error_debug() {
        let err = ReplError::message("debug test");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Message"));
    }
}
