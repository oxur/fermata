//! Display formatting for REPL output.

use owo_colors::OwoColorize;

use crate::ir::score::ScorePartwise;
use crate::lang::error::CompileError;
use crate::sexpr::{ToSexpr, print_sexpr};

/// Format a successful evaluation result.
pub fn format_eval_result(score: &ScorePartwise, use_colors: bool) -> String {
    // For now, show the S-expression representation
    let sexpr = score.to_sexpr();
    let output = print_sexpr(&sexpr);

    if use_colors {
        format!("{}", output.green())
    } else {
        output
    }
}

/// Format a compilation error.
pub fn format_compile_error(error: &CompileError, use_colors: bool) -> String {
    let msg = error.to_string();

    if use_colors {
        format!("{}: {}", "Error".red(), msg)
    } else {
        format!("Error: {}", msg)
    }
}

/// Format a general message (info level).
pub fn format_info(message: &str, use_colors: bool) -> String {
    if use_colors {
        format!("{}", message.cyan())
    } else {
        message.to_string()
    }
}

/// Format a warning message.
pub fn format_warning(message: &str, use_colors: bool) -> String {
    if use_colors {
        format!("{}: {}", "Warning".yellow(), message)
    } else {
        format!("Warning: {}", message)
    }
}

/// Format the REPL banner shown at startup.
pub fn format_banner(use_colors: bool) -> String {
    let version = crate::VERSION;
    let banner = format!(
        r#"Fermata {} - Interactive music notation
Type :help for help, :quit to exit.
"#,
        version
    );

    if use_colors {
        format!("{}", banner.bold())
    } else {
        banner
    }
}

/// Format a chat message (stub for Phase 6b).
pub fn format_chat_stub(kind: &str, message: &str, use_colors: bool) -> String {
    let output = format!("[Chat {}]: {}", kind, message);

    if use_colors {
        format!("{}", output.dimmed())
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::part::PartList;

    // Helper to create a minimal ScorePartwise for testing
    fn test_score() -> ScorePartwise {
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

    #[test]
    fn test_format_eval_result_no_colors() {
        let score = test_score();
        let result = format_eval_result(&score, false);
        assert!(result.contains("score"));
    }

    #[test]
    fn test_format_eval_result_with_colors() {
        let score = test_score();
        let result = format_eval_result(&score, true);
        // Should contain the output (color codes vary by terminal)
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_compile_error_no_colors() {
        let error = CompileError::InvalidPitch("xyz".to_string());
        let result = format_compile_error(&error, false);
        assert!(result.contains("Error:"));
        assert!(result.contains("xyz"));
    }

    #[test]
    fn test_format_compile_error_with_colors() {
        let error = CompileError::InvalidPitch("xyz".to_string());
        let result = format_compile_error(&error, true);
        assert!(result.contains("xyz"));
    }

    #[test]
    fn test_format_info_no_colors() {
        let result = format_info("test message", false);
        assert_eq!(result, "test message");
    }

    #[test]
    fn test_format_info_with_colors() {
        let result = format_info("test message", true);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_format_warning_no_colors() {
        let result = format_warning("something happened", false);
        assert!(result.contains("Warning:"));
        assert!(result.contains("something happened"));
    }

    #[test]
    fn test_format_warning_with_colors() {
        let result = format_warning("something happened", true);
        assert!(result.contains("something happened"));
    }

    #[test]
    fn test_format_banner_no_colors() {
        let result = format_banner(false);
        assert!(result.contains("Fermata"));
        assert!(result.contains(":help"));
        assert!(result.contains(":quit"));
    }

    #[test]
    fn test_format_banner_with_colors() {
        let result = format_banner(true);
        assert!(result.contains("Fermata"));
    }

    #[test]
    fn test_format_chat_stub_no_colors() {
        let result = format_chat_stub("say", "hello world", false);
        assert!(result.contains("[Chat say]"));
        assert!(result.contains("hello world"));
    }

    #[test]
    fn test_format_chat_stub_with_colors() {
        let result = format_chat_stub("emote", "thinks", true);
        assert!(result.contains("thinks"));
    }
}
