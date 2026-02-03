//! REPL command dispatch.

use owo_colors::OwoColorize;

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
    /// Show the banner.
    ShowBanner,
}

/// Dispatch and execute a REPL command.
///
/// The `cmd` parameter is the command string without the leading `:`.
pub fn dispatch(
    cmd: &str,
    session: &mut ReplSession,
    use_colors: bool,
) -> ReplResult<CommandResult> {
    let trimmed = cmd.trim();
    let (name, args) = match trimmed.split_once(char::is_whitespace) {
        Some((n, a)) => (n, a.trim()),
        None => (trimmed, ""),
    };

    match name.to_lowercase().as_str() {
        "help" | "h" | "?" => Ok(cmd_help(args, use_colors)),
        "quit" | "exit" | "q" => Ok(CommandResult::Exit),
        "clear" | "cls" => Ok(cmd_clear()),
        "banner" => Ok(CommandResult::ShowBanner),
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
        if render_opts.show_page_info {
            "yes"
        } else {
            "no"
        }
    );
    CommandResult::Output(output)
}

/// Display help information.
fn cmd_help(topic: &str, use_colors: bool) -> CommandResult {
    let output = if topic.is_empty() {
        general_help(use_colors)
    } else {
        match topic.to_lowercase().as_str() {
            "commands" | "cmd" => commands_help(use_colors),
            "expressions" | "expr" => expressions_help(use_colors),
            "chat" => chat_help(use_colors),
            _ => format!(
                "Unknown help topic: {}\n\nAvailable topics: commands, expressions, chat",
                topic
            ),
        }
    };
    CommandResult::Output(output)
}

/// Format a header (bright yellow, bold).
fn header(text: &str, use_colors: bool) -> String {
    if use_colors {
        format!("{}", text.bright_yellow().bold())
    } else {
        text.to_string()
    }
}

/// Format a command (cyan).
fn cmd(text: &str, use_colors: bool) -> String {
    if use_colors {
        format!("{}", text.cyan())
    } else {
        text.to_string()
    }
}

/// Format a title (blue).
fn title(text: &str, use_colors: bool) -> String {
    if use_colors {
        format!("{}", text.blue())
    } else {
        text.to_string()
    }
}

/// Generate general help text.
fn general_help(use_colors: bool) -> String {
    format!(
        r#"{}
{}

{}
  {}    Evaluate a Fermata expression
  {}        Execute a REPL command
  {}        Send a chat message (future: Claude integration)

{}
  {}         Show this help
  {}      Exit the REPL
  {}          Clear the screen
  {}   Set display mode (sexpr, musicxml, png, silent)
  {}             Show current settings

{}
  {}      Last 1-3 evaluated results
  {}      Last 1-3 input expressions (as data)

{}
  {}    Available REPL commands
  {} Fermata expression syntax
  {}        Chat message syntax

{}
  (score :title "My Song")
  (note c4 :q)
  (chord :q c4 e4 g4)
"#,
        title("\nFermata REPL - Interactive music notation", use_colors),
        title("------------   --------------------------", use_colors),
        header("USAGE:", use_colors),
        cmd("(expression)", use_colors),
        cmd(":command", use_colors),
        cmd("/message", use_colors),
        header("COMMANDS:", use_colors),
        cmd(":help, :h, :?", use_colors),
        cmd(":quit, :exit, :q", use_colors),
        cmd(":clear, :cls", use_colors),
        cmd(":set display <mode>", use_colors),
        cmd(":settings", use_colors),
        header("HISTORY VARIABLES:", use_colors),
        cmd("*, **, ***", use_colors),
        cmd("+, ++, +++", use_colors),
        header("HELP TOPICS:", use_colors),
        cmd(":help commands", use_colors),
        cmd(":help expressions", use_colors),
        cmd(":help chat", use_colors),
        header("EXAMPLES:", use_colors),
    )
}

/// Generate commands help text.
fn commands_help(use_colors: bool) -> String {
    format!(
        r#"{}

Commands start with ':' and control the REPL itself.

  {}         Show help
  {}         Show help on a specific topic
  {}      Exit the REPL
  {}          Clear the screen
  {}   Set output display mode
  {}             Show current settings

{}
  {}     S-expression output (default, for debugging)
  {}  MusicXML output (interchange format)
  {}       MEI output (requires 'render' feature)
  {}      MIDI output (requires 'render' feature)
  {}       Rendered notation in terminal (requires 'render' feature)
  {}    No output (value stored for later use)

{}
  {}             Render last result as PNG
  {}      Render specific page
  {}      Export to file
"#,
        header("REPL Commands", use_colors),
        cmd(":help, :h, :?", use_colors),
        cmd(":help <topic>", use_colors),
        cmd(":quit, :exit, :q", use_colors),
        cmd(":clear, :cls", use_colors),
        cmd(":set display <mode>", use_colors),
        cmd(":settings", use_colors),
        header("DISPLAY MODES:", use_colors),
        cmd("sexpr", use_colors),
        cmd("musicxml", use_colors),
        cmd("mei", use_colors),
        cmd("midi", use_colors),
        cmd("png", use_colors),
        cmd("silent", use_colors),
        header("Future commands (Phase 6a-M2+):", use_colors),
        cmd(":render", use_colors),
        cmd(":render page N", use_colors),
        cmd(":export <file>", use_colors),
    )
}

/// Generate expressions help text.
fn expressions_help(use_colors: bool) -> String {
    format!(
        r#"{}

Expressions start with '(' and are evaluated as Fermata notation.

{}
  {}           Define a complete score
  {} Define a part
  {}         Define a measure
  {}  Define a note
  {}          Define a rest
  {} Define a chord

{}
  {}   Natural notes (middle C = c4)
  {}        Sharps and flats
  {}          Case-insensitive

{}
  {}    Whole note
  {}    Half note
  {}    Quarter note
  {}    Eighth note
  {}   Sixteenth note

{}
  {}                    Quarter note middle C
  {}             Half note C major chord
  {} Measure with note and rest
"#,
        header("Fermata Expressions", use_colors),
        header("BASIC FORMS:", use_colors),
        cmd("(score ...)", use_colors),
        cmd("(part :instrument ...)", use_colors),
        cmd("(measure ...)", use_colors),
        cmd("(note <pitch> <dur>)", use_colors),
        cmd("(rest <dur>)", use_colors),
        cmd("(chord <dur> <pitches>...)", use_colors),
        header("PITCHES:", use_colors),
        cmd("c4, d4, e4...", use_colors),
        cmd("c#4, db4", use_colors),
        cmd("C4, D4", use_colors),
        header("DURATIONS:", use_colors),
        cmd(":w", use_colors),
        cmd(":h", use_colors),
        cmd(":q", use_colors),
        cmd(":8", use_colors),
        cmd(":16", use_colors),
        header("EXAMPLES:", use_colors),
        cmd("(note c4 :q)", use_colors),
        cmd("(chord :h c4 e4 g4)", use_colors),
        cmd("(measure (note c4 :q) (rest :q))", use_colors),
    )
}

/// Generate chat help text.
fn chat_help(use_colors: bool) -> String {
    format!(
        r#"{}

Chat messages start with '/' and are used for communication
(future: with Claude AI assistant).

{}
  {}          Say something
  {}      Explicit say
  {}        Emote (describe an action)
  {}        Alias for /em

{}
  /what key is this in?
  /em scratches head
  /me is confused about the time signature
"#,
        header("Chat Messages", use_colors),
        header("SYNTAX:", use_colors),
        cmd("/message", use_colors),
        cmd("/say message", use_colors),
        cmd("/em action", use_colors),
        cmd("/me action", use_colors),
        header("EXAMPLES:", use_colors),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_help() {
        let mut session = ReplSession::new();
        let result = dispatch("help", &mut session, false).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Fermata REPL")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_alias_h() {
        let mut session = ReplSession::new();
        let result = dispatch("h", &mut session, false).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_dispatch_help_alias_question() {
        let mut session = ReplSession::new();
        let result = dispatch("?", &mut session, false).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_dispatch_help_topic_commands() {
        let mut session = ReplSession::new();
        let result = dispatch("help commands", &mut session, false).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("REPL Commands")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_topic_expressions() {
        let mut session = ReplSession::new();
        let result = dispatch("help expr", &mut session, false).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Fermata Expressions")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_topic_chat() {
        let mut session = ReplSession::new();
        let result = dispatch("help chat", &mut session, false).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Chat Messages")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_unknown_topic() {
        let mut session = ReplSession::new();
        let result = dispatch("help xyz", &mut session, false).unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Unknown help topic")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_quit() {
        let mut session = ReplSession::new();
        let result = dispatch("quit", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_exit() {
        let mut session = ReplSession::new();
        let result = dispatch("exit", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_q() {
        let mut session = ReplSession::new();
        let result = dispatch("q", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_empty() {
        let mut session = ReplSession::new();
        let result = dispatch("", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::Continue);
    }

    #[test]
    fn test_dispatch_whitespace() {
        let mut session = ReplSession::new();
        let result = dispatch("   ", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::Continue);
    }

    #[test]
    fn test_dispatch_unknown() {
        let mut session = ReplSession::new();
        let result = dispatch("foobar", &mut session, false).unwrap();
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
        let result = dispatch("HELP", &mut session, false).unwrap();
        assert!(matches!(result, CommandResult::Output(_)));

        let result = dispatch("QUIT", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_with_leading_whitespace() {
        let mut session = ReplSession::new();
        let result = dispatch("  help", &mut session, false).unwrap();
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

        let result = dispatch("set display sexpr", &mut session, false).unwrap();
        assert_eq!(
            result,
            CommandResult::DisplayModeChanged(DisplayMode::Sexpr)
        );
        assert_eq!(session.display_mode(), DisplayMode::Sexpr);
    }

    #[test]
    fn test_dispatch_set_display_png() {
        let mut session = ReplSession::new();
        let result = dispatch("set display png", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::Png));
        assert_eq!(session.display_mode(), DisplayMode::Png);
    }

    #[test]
    fn test_dispatch_set_display_musicxml() {
        let mut session = ReplSession::new();
        let result = dispatch("set display xml", &mut session, false).unwrap();
        assert_eq!(
            result,
            CommandResult::DisplayModeChanged(DisplayMode::MusicXml)
        );
        assert_eq!(session.display_mode(), DisplayMode::MusicXml);
    }

    #[test]
    fn test_dispatch_set_display_silent() {
        let mut session = ReplSession::new();
        let result = dispatch("set display silent", &mut session, false).unwrap();
        assert_eq!(
            result,
            CommandResult::DisplayModeChanged(DisplayMode::Silent)
        );
        assert_eq!(session.display_mode(), DisplayMode::Silent);
    }

    #[test]
    fn test_dispatch_set_display_alias_d() {
        let mut session = ReplSession::new();
        let result = dispatch("set d png", &mut session, false).unwrap();
        assert_eq!(result, CommandResult::DisplayModeChanged(DisplayMode::Png));
    }

    #[test]
    fn test_dispatch_set_display_no_value() {
        let mut session = ReplSession::new();
        let result = dispatch("set display", &mut session, false).unwrap();
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
        let result = dispatch("set display invalid", &mut session, false).unwrap();
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
        let result = dispatch("set", &mut session, false).unwrap();
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
        let result = dispatch("set foo bar", &mut session, false).unwrap();
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
        let result = dispatch("settings", &mut session, false).unwrap();
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

        let result = dispatch("settings", &mut session, false).unwrap();
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
