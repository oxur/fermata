//! Fermata REPL - Interactive read-eval-print loop.
//!
//! The REPL provides an interactive environment for evaluating Fermata
//! expressions and building musical scores incrementally.
//!
//! # Input Types
//!
//! - **Expressions**: S-expressions starting with `(` are evaluated as Fermata notation
//! - **Commands**: Lines starting with `:` are REPL commands (`:help`, `:quit`)
//! - **Chat**: Lines starting with `/` are chat messages (stub for Claude integration)
//!
//! # Example Session
//!
//! ```text
//! fermata> (score :title "Test")
//! (score ...)
//! fermata> :help
//! ...
//! fermata> :quit
//! ```

pub mod commands;
pub mod display;
pub mod error;
pub mod input;
pub mod prompt;
pub mod validator;

use std::path::PathBuf;

use reedline::{FileBackedHistory, Reedline, Signal};

use crate::lang::compile;

pub use error::{ReplError, ReplResult};
pub use input::{ChatKind, InputKind};

use commands::CommandResult;
use display::{format_banner, format_chat_stub, format_compile_error, format_eval_result};
use input::classify;
use prompt::FermataPrompt;
use validator::FermataValidator;

/// The Fermata REPL.
pub struct Repl {
    /// The reedline editor instance.
    editor: Reedline,
    /// Whether colors are enabled.
    use_colors: bool,
}

impl Repl {
    /// Create a new REPL instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the history file cannot be created.
    pub fn new(use_colors: bool) -> ReplResult<Self> {
        let editor = Self::create_editor()?;
        Ok(Self { editor, use_colors })
    }

    /// Create the reedline editor with history and validation.
    fn create_editor() -> ReplResult<Reedline> {
        // Set up history file
        let history_path = Self::history_path()?;

        // Ensure parent directory exists
        if let Some(parent) = history_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let history = Box::new(
            FileBackedHistory::with_file(1000, history_path)
                .map_err(|e| ReplError::reedline(e.to_string()))?,
        );

        let editor = Reedline::create()
            .with_validator(Box::new(FermataValidator::new()))
            .with_history(history);

        Ok(editor)
    }

    /// Get the history file path.
    fn history_path() -> ReplResult<PathBuf> {
        let data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        Ok(data_dir.join("fermata").join("repl_history"))
    }

    /// Run the REPL loop until quit or EOF.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an unrecoverable I/O error.
    pub fn run(&mut self) -> ReplResult<()> {
        // Print banner
        println!("{}", format_banner(self.use_colors));

        let prompt = FermataPrompt::new();

        loop {
            match self.editor.read_line(&prompt) {
                Ok(Signal::Success(line)) => {
                    if self.handle_input(&line)? {
                        break;
                    }
                }
                Ok(Signal::CtrlC) => {
                    // Clear current input, continue
                    println!("^C");
                }
                Ok(Signal::CtrlD) => {
                    // Exit
                    println!();
                    break;
                }
                Err(e) => {
                    return Err(ReplError::reedline(e.to_string()));
                }
            }
        }

        Ok(())
    }

    /// Handle a complete input line.
    ///
    /// Returns `true` if the REPL should exit.
    fn handle_input(&mut self, line: &str) -> ReplResult<bool> {
        match classify(line) {
            InputKind::Empty => {
                // Just re-prompt
                Ok(false)
            }
            InputKind::Expression(expr) => {
                self.eval_expression(&expr);
                Ok(false)
            }
            InputKind::Command(cmd) => self.handle_command(&cmd),
            InputKind::Chat(chat) => {
                self.handle_chat(&chat);
                Ok(false)
            }
        }
    }

    /// Evaluate a Fermata expression and display the result.
    fn eval_expression(&self, source: &str) {
        match compile(source) {
            Ok(score) => {
                println!("{}", format_eval_result(&score, self.use_colors));
            }
            Err(e) => {
                println!("{}", format_compile_error(&e, self.use_colors));
            }
        }
    }

    /// Handle a REPL command.
    ///
    /// Returns `true` if the REPL should exit.
    fn handle_command(&self, cmd: &str) -> ReplResult<bool> {
        match commands::dispatch(cmd)? {
            CommandResult::Continue => Ok(false),
            CommandResult::Exit => Ok(true),
            CommandResult::Output(msg) => {
                println!("{}", msg);
                Ok(false)
            }
        }
    }

    /// Handle a chat message (stub for Phase 6b).
    fn handle_chat(&self, chat: &ChatKind) {
        let (kind, message) = match chat {
            ChatKind::Say(msg) => ("say", msg.as_str()),
            ChatKind::Emote(msg) => ("emote", msg.as_str()),
        };

        // For now, just acknowledge the chat
        println!("{}", format_chat_stub(kind, message, self.use_colors));
        println!(
            "{}",
            display::format_info("(Chat integration coming in Phase 6b)", self.use_colors)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_path() {
        let path = Repl::history_path().unwrap();
        assert!(path.ends_with("repl_history"));
        assert!(path.to_string_lossy().contains("fermata"));
    }

    #[test]
    fn test_classify_reexport() {
        // Verify classify is accessible
        let result = classify("(test)");
        assert!(matches!(result, InputKind::Expression(_)));
    }

    #[test]
    fn test_input_kind_reexport() {
        // Verify InputKind is accessible
        let _kind = InputKind::Empty;
    }

    #[test]
    fn test_chat_kind_reexport() {
        // Verify ChatKind is accessible
        let _chat = ChatKind::Say("test".to_string());
    }

    #[test]
    fn test_repl_error_reexport() {
        // Verify ReplError is accessible
        let err = ReplError::message("test");
        assert!(err.to_string().contains("test"));
    }
}
