//! REPL command dispatch.

use super::error::ReplResult;
use super::session::{DisplayMode, ReplSession};

/// Result of executing a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    /// Continue the REPL loop.
    Continue,
    /// Exit the REPL.
    Exit,
    /// Display output to the user.
    Output(String),
    /// Display mode was changed.
    DisplayModeChanged(DisplayMode),
}

/// Dispatch and execute a REPL command.
///
/// The `cmd` parameter is the command string without the leading `:`.
pub fn dispatch(cmd: &str, session: &mut ReplSession) -> ReplResult<CommandResult> {
    let trimmed = cmd.trim();
    let (name, args) = match trimmed.split_once(char::is_whitespace) {
        Some((n, a)) => (n, a.trim()),
        None => (trimmed, ""),
    };

    match name.to_lowercase().as_str() {
        "help" | "h" | "?" => Ok(cmd_help(args)),
        "quit" | "exit" | "q" => Ok(CommandResult::Exit),
        "clear" | "cls" => Ok(cmd_clear()),
        "set" => cmd_set(args, session),
        "settings" => Ok(cmd_settings(session)),
        "" => Ok(CommandResult::Continue),
        other => Ok(CommandResult::Output(format!(
            "Unknown command: :{}\nType :help for available commands.",
            other
        ))),
    }
}

/// Handle the :set command.
fn cmd_set(args: &str, session: &mut ReplSession) -> ReplResult<CommandResult> {
    let trimmed = args.trim();
    let (setting, value) = match trimmed.split_once(char::is_whitespace) {
        Some((s, v)) => (s, v.trim()),
        None => (trimmed, ""),
    };

    match setting.to_lowercase().as_str() {
        "display" | "d" => {
            if value.is_empty() {
                Ok(CommandResult::Output(format!(
                    "Current display mode: {}\nUsage: :set display <mode>\nModes: sexpr, musicxml, mei, midi, png, silent",
                    session.display_mode().name()
                )))
            } else if let Some(mode) = DisplayMode::parse(value) {
                session.set_display_mode(mode);
                Ok(CommandResult::DisplayModeChanged(mode))
            } else {
                Ok(CommandResult::Output(format!(
                    "Unknown display mode: '{}'\nValid modes: sexpr, musicxml, mei, midi, png, silent",
                    value
                )))
            }
        }
        "dark-mode" | "darkmode" | "dark" => {
            if value.is_empty() {
                let current = match session.dark_mode() {
                    None => "auto",
                    Some(true) => "on",
                    Some(false) => "off",
                };
                Ok(CommandResult::Output(format!(
                    "Current dark mode: {}\nUsage: :set dark-mode <on|off|auto>",
                    current
                )))
            } else {
                match value.to_lowercase().as_str() {
                    "on" | "true" | "yes" | "1" => {
                        session.set_dark_mode(Some(true));
                        Ok(CommandResult::Output("Dark mode: on".to_string()))
                    }
                    "off" | "false" | "no" | "0" => {
                        session.set_dark_mode(Some(false));
                        Ok(CommandResult::Output("Dark mode: off".to_string()))
                    }
                    "auto" | "detect" => {
                        session.set_dark_mode(None);
                        Ok(CommandResult::Output("Dark mode: auto-detect".to_string()))
                    }
                    _ => Ok(CommandResult::Output(format!(
                        "Invalid dark mode value: '{}'\nValid values: on, off, auto",
                        value
                    ))),
                }
            }
        }
        "" => Ok(CommandResult::Output(
            "Usage: :set <setting> <value>\n\nSettings:\n  display <mode>    Set output display mode (sexpr, musicxml, png, silent)\n  dark-mode <mode>  Set dark mode (on, off, auto)".to_string()
        )),
        other => Ok(CommandResult::Output(format!(
            "Unknown setting: '{}'\nType :set for available settings.",
            other
        ))),
    }
}

/// Clear the terminal screen.
fn cmd_clear() -> CommandResult {
    // ANSI escape: clear screen and move cursor to top-left
    print!("\x1b[2J\x1b[H");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    CommandResult::Continue
}

/// Display current settings.
fn cmd_settings(session: &ReplSession) -> CommandResult {
    let render_opts = session.render_options();
    let dark_mode = match session.dark_mode() {
        None => "auto",
        Some(true) => "on",
        Some(false) => "off",
    };
    let output = format!(
        "Current Settings:\n  display mode:   {}\n  dark mode:      {}\n  render width:   {} px\n  render page:    {}\n  show page info: {}",
        session.display_mode().name(),
        dark_mode,
        render_opts.width,
        render_opts.page,
        if render_opts.show_page_info { "yes" } else { "no" }
    );
    CommandResult::Output(output)
}

/// Display help information.
fn cmd_help(topic: &str) -> CommandResult {
    let output = if topic.is_empty() {
        GENERAL_HELP.to_string()
    } else {
        match topic.to_lowercase().as_str() {
            "commands" | "cmd" => COMMANDS_HELP.to_string(),
            "expressions" | "expr" => EXPRESSIONS_HELP.to_string(),
            "chat" => CHAT_HELP.to_string(),
            _ => format!(
                "Unknown help topic: {}\n\nAvailable topics: commands, expressions, chat",
                topic
            ),
        }
    };
    CommandResult::Output(output)
}

const GENERAL_HELP: &str = r#"Fermata REPL - Interactive music notation

USAGE:
  (expression)    Evaluate a Fermata expression
  :command        Execute a REPL command
  /message        Send a chat message (future: Claude integration)

COMMANDS:
  :help, :h, :?         Show this help
  :quit, :exit, :q      Exit the REPL
  :clear, :cls          Clear the screen
  :set display <mode>   Set display mode (sexpr, musicxml, png, silent)
  :settings             Show current settings

HISTORY VARIABLES:
  *, **, ***      Last 1-3 evaluated results
  +, ++, +++      Last 1-3 input expressions (as data)

HELP TOPICS:
  :help commands    Available REPL commands
  :help expressions Fermata expression syntax
  :help chat        Chat message syntax

EXAMPLES:
  (score :title "My Song")
  (note c4 :q)
  (chord :q c4 e4 g4)
"#;

const COMMANDS_HELP: &str = r#"REPL Commands

Commands start with ':' and control the REPL itself.

  :help, :h, :?         Show help
  :help <topic>         Show help on a specific topic
  :quit, :exit, :q      Exit the REPL
  :clear, :cls          Clear the screen
  :set display <mode>   Set output display mode
  :settings             Show current settings

DISPLAY MODES:
  sexpr     S-expression output (default, for debugging)
  musicxml  MusicXML output (interchange format)
  mei       MEI output (requires 'render' feature)
  midi      MIDI output (requires 'render' feature)
  png       Rendered notation in terminal (requires 'render' feature)
  silent    No output (value stored for later use)

Future commands (Phase 6a-M2+):
  :render             Render last result as PNG
  :render page N      Render specific page
  :export <file>      Export to file
"#;

const EXPRESSIONS_HELP: &str = r#"Fermata Expressions

Expressions start with '(' and are evaluated as Fermata notation.

BASIC FORMS:
  (score ...)           Define a complete score
  (part :instrument ...) Define a part
  (measure ...)         Define a measure
  (note <pitch> <dur>)  Define a note
  (rest <dur>)          Define a rest
  (chord <dur> <pitches>...) Define a chord

PITCHES:
  c4, d4, e4...   Natural notes (middle C = c4)
  c#4, db4        Sharps and flats
  C4, D4          Case-insensitive

DURATIONS:
  :w    Whole note
  :h    Half note
  :q    Quarter note
  :8    Eighth note
  :16   Sixteenth note

EXAMPLES:
  (note c4 :q)                    Quarter note middle C
  (chord :h c4 e4 g4)             Half note C major chord
  (measure (note c4 :q) (rest :q)) Measure with note and rest
"#;

const CHAT_HELP: &str = r#"Chat Messages

Chat messages start with '/' and are used for communication
(future: with Claude AI assistant).

SYNTAX:
  /message          Say something
  /say message      Explicit say
  /em action        Emote (describe an action)
  /me action        Alias for /em

EXAMPLES:
  /what key is this in?
  /em scratches head
  /me is confused about the time signature
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_help() {
        let mut session = ReplSession::new();
        let result = dispatch("help", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Fermata REPL")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_alias_h() {
        let mut session = ReplSession::new();
        let result = dispatch("h", &mut session).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_dispatch_help_alias_question() {
        let mut session = ReplSession::new();
        let result = dispatch("?", &mut session).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_dispatch_help_topic_commands() {
        let mut session = ReplSession::new();
        let result = dispatch("help commands", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("REPL Commands")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_topic_expressions() {
        let mut session = ReplSession::new();
        let result = dispatch("help expr", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Fermata Expressions")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_topic_chat() {
        let mut session = ReplSession::new();
        let result = dispatch("help chat", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Chat Messages")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_unknown_topic() {
        let mut session = ReplSession::new();
        let result = dispatch("help xyz", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Unknown help topic")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_quit() {
        let mut session = ReplSession::new();
        let result = dispatch("quit", &mut session).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_exit() {
        let mut session = ReplSession::new();
        let result = dispatch("exit", &mut session).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_q() {
        let mut session = ReplSession::new();
        let result = dispatch("q", &mut session).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_empty() {
        let mut session = ReplSession::new();
        let result = dispatch("", &mut session).unwrap();
        assert_eq!(result, CommandResult::Continue);
    }

    #[test]
    fn test_dispatch_whitespace() {
        let mut session = ReplSession::new();
        let result = dispatch("   ", &mut session).unwrap();
        assert_eq!(result, CommandResult::Continue);
    }

    #[test]
    fn test_dispatch_unknown() {
        let mut session = ReplSession::new();
        let result = dispatch("foobar", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("Unknown command"));
                assert!(s.contains("foobar"));
            }
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_case_insensitive() {
        let mut session = ReplSession::new();
        let result = dispatch("HELP", &mut session).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));

        let result = dispatch("QUIT", &mut session).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_with_leading_whitespace() {
        let mut session = ReplSession::new();
        let result = dispatch("  help", &mut session).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_command_result_debug() {
        let result = CommandResult::Continue;
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("Continue"));
    }

    #[test]
    fn test_command_result_clone() {
        let result = CommandResult::Output("test".to_string());
        let cloned = result.clone();
        assert_eq!(result, cloned);
    }

    // === :set command tests ===

    #[test]
    fn test_dispatch_set_display_sexpr() {
        let mut session = ReplSession::new();
        session.set_display_mode(DisplayMode::Png); // Start with non-default

        let result = dispatch("set display sexpr", &mut session).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::Sexpr));
        assert_eq!(session.display_mode(), DisplayMode::Sexpr);
    }

    #[test]
    fn test_dispatch_set_display_png() {
        let mut session = ReplSession::new();
        let result = dispatch("set display png", &mut session).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::Png));
        assert_eq!(session.display_mode(), DisplayMode::Png);
    }

    #[test]
    fn test_dispatch_set_display_musicxml() {
        let mut session = ReplSession::new();
        let result = dispatch("set display xml", &mut session).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::MusicXml));
        assert_eq!(session.display_mode(), DisplayMode::MusicXml);
    }

    #[test]
    fn test_dispatch_set_display_silent() {
        let mut session = ReplSession::new();
        let result = dispatch("set display silent", &mut session).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::Silent));
        assert_eq!(session.display_mode(), DisplayMode::Silent);
    }

    #[test]
    fn test_dispatch_set_display_alias_d() {
        let mut session = ReplSession::new();
        let result = dispatch("set d png", &mut session).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::Png));
    }

    #[test]
    fn test_dispatch_set_display_no_value() {
        let mut session = ReplSession::new();
        let result = dispatch("set display", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("Current display mode"));
                assert!(s.contains("sexpr"));
            }
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_set_display_invalid() {
        let mut session = ReplSession::new();
        let result = dispatch("set display invalid", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("Unknown display mode"));
                assert!(s.contains("invalid"));
            }
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_set_no_args() {
        let mut session = ReplSession::new();
        let result = dispatch("set", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("Usage:"));
                assert!(s.contains("display"));
            }
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_set_unknown_setting() {
        let mut session = ReplSession::new();
        let result = dispatch("set foo bar", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("Unknown setting"));
                assert!(s.contains("foo"));
            }
            _ => panic!("Expected Output"),
        }
    }

    // === :settings command tests ===

    #[test]
    fn test_dispatch_settings() {
        let mut session = ReplSession::new();
        let result = dispatch("settings", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("Current Settings"));
                assert!(s.contains("display mode"));
                assert!(s.contains("sexpr"));
                assert!(s.contains("render width"));
                assert!(s.contains("800"));
            }
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_settings_after_change() {
        let mut session = ReplSession::new();
        session.set_display_mode(DisplayMode::Png);
        session.render_options_mut().width = 1200;

        let result = dispatch("settings", &mut session).unwrap();
        match result {
            CommandResult::Output(s) => {
                assert!(s.contains("png"));
                assert!(s.contains("1200"));
            }
            _ => panic!("Expected Output"),
        }
    }

    // === DisplayModeChanged variant tests ===

    #[test]
    fn test_command_result_display_mode_changed_debug() {
        let result = CommandResult::DisplayModeChanged(DisplayMode::Png);
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("DisplayModeChanged"));
        assert!(debug_str.contains("Png"));
    }

    #[test]
    fn test_command_result_display_mode_changed_eq() {
        let a = CommandResult::DisplayModeChanged(DisplayMode::Png);
        let b = CommandResult::DisplayModeChanged(DisplayMode::Png);
        let c = CommandResult::DisplayModeChanged(DisplayMode::Sexpr);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
