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
pub mod config;
pub mod display;
pub mod error;
pub mod highlighter;
pub mod input;
pub mod prompt;
#[cfg(feature = "render")]
pub mod render;
pub mod session;
pub mod validator;

use std::path::PathBuf;

use reedline::{FileBackedHistory, Reedline, Signal};

use crate::lang::compile;
use crate::sexpr::parser::parse as parse_sexpr;
use crate::sexpr::{Sexpr, print_sexpr};

pub use error::{ReplError, ReplResult};
pub use input::{ChatKind, InputKind};
pub use session::{DisplayMode, HistoryValue, RenderOptions, ReplSession};

use commands::CommandResult;
use config::ReplConfig;
use display::{format_banner, format_chat_stub, format_compile_error, format_result_for_mode};
use input::classify;
use prompt::FermataPrompt;
use validator::FermataValidator;

/// The Fermata REPL.
pub struct Repl {
    /// The reedline editor instance.
    editor: Reedline,
    /// Whether colors are enabled.
    use_colors: bool,
    /// Session state (history, display mode, etc.)
    session: ReplSession,
    /// Configuration
    config: ReplConfig,
}

impl Repl {
    /// Create a new REPL instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the history file cannot be created.
    pub fn new(use_colors: bool) -> ReplResult<Self> {
        let editor = Self::create_editor(use_colors)?;
        let config = ReplConfig::load()?;
        Ok(Self {
            editor,
            use_colors,
            session: ReplSession::new(),
            config,
        })
    }

    /// Get a reference to the session.
    pub fn session(&self) -> &ReplSession {
        &self.session
    }

    /// Get a mutable reference to the session.
    pub fn session_mut(&mut self) -> &mut ReplSession {
        &mut self.session
    }

    /// Create the reedline editor with history, validation, and syntax highlighting.
    fn create_editor(use_colors: bool) -> ReplResult<Reedline> {
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
            .with_highlighter(Box::new(highlighter::FermataHighlighter::new(use_colors)))
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
        self.print_banner();

        // Flush stdout and stderr before reedline takes over terminal
        use std::io::Write;
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();

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
            InputKind::Command(cmd) => {
                let use_colors = self.use_colors;
                self.handle_command(&cmd, use_colors)
            }
            InputKind::Chat(chat) => {
                self.handle_chat(&chat);
                Ok(false)
            }
        }
    }

    /// Evaluate a Fermata expression and display the result.
    fn eval_expression(&mut self, source: &str) {
        // Step 1: Parse the source to get the Sexpr
        let sexpr = match parse_sexpr(source) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", format_compile_error(&e.into(), self.use_colors));
                return;
            }
        };

        // Step 2: Check if it's a bare history symbol
        if let Sexpr::Symbol(sym) = &sexpr {
            if ReplSession::is_history_symbol(sym) {
                self.display_history_value(sym);
                return;
            }
        }

        // Step 3: Compile the expression
        match compile(source) {
            Ok(score) => {
                // Store the expression and result in history
                self.session.push_expression(sexpr);
                self.session.push_result(score.clone());

                // Display the result based on current display mode
                let mode = self.session.display_mode();
                let options = self.session.render_options();
                if let Some(output) =
                    format_result_for_mode(&score, mode, self.use_colors, &options)
                {
                    println!("{}", output);
                }
            }
            Err(e) => {
                println!("{}", format_compile_error(&e, self.use_colors));
            }
        }
    }

    /// Display a history value (*, **, ***, +, ++, +++)
    fn display_history_value(&self, symbol: &str) {
        match self.session.get_history_value(symbol) {
            Some(HistoryValue::Result(score)) => {
                // Display result based on current display mode
                let mode = self.session.display_mode();
                let options = self.session.render_options();
                if let Some(output) =
                    format_result_for_mode(&score, mode, self.use_colors, &options)
                {
                    println!("{}", output);
                }
            }
            Some(HistoryValue::Expression(sexpr)) => {
                // Expressions are always displayed as S-expressions (they ARE data)
                let output = print_sexpr(&sexpr);
                if self.use_colors {
                    use owo_colors::OwoColorize;
                    println!("{}", output.cyan());
                } else {
                    println!("{}", output);
                }
            }
            None => {
                let msg = format!("No value for '{}' (history is empty)", symbol);
                println!("{}", display::format_warning(&msg, self.use_colors));
            }
        }
    }

    /// Handle a REPL command.
    ///
    /// Returns `true` if the REPL should exit.
    fn handle_command(&mut self, cmd: &str, use_colors: bool) -> ReplResult<bool> {
        match commands::dispatch(cmd, &mut self.session, use_colors)? {
            CommandResult::Continue => Ok(false),
            CommandResult::Exit => Ok(true),
            CommandResult::Output(msg) => {
                println!("{}", msg);
                Ok(false)
            }
            CommandResult::DisplayModeChanged(mode) => {
                let msg = format!("Display mode set to: {}", mode.name());
                println!("{}", display::format_info(&msg, use_colors));
                Ok(false)
            }
            CommandResult::ShowBanner => {
                self.print_banner();
                Ok(false)
            }
        }
    }

    /// Print the banner.
    fn print_banner(&self) {
        let banner_text = self.config.banner_text();
        println!("{}", format_banner(&banner_text, self.use_colors));
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
