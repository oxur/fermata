//! REPL command dispatch.

use super::error::ReplResult;

/// Result of executing a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    /// Continue the REPL loop.
    Continue,
    /// Exit the REPL.
    Exit,
    /// Display output to the user.
    Output(String),
}

/// Dispatch and execute a REPL command.
///
/// The `cmd` parameter is the command string without the leading `:`.
pub fn dispatch(cmd: &str) -> ReplResult<CommandResult> {
    let trimmed = cmd.trim();
    let (name, args) = match trimmed.split_once(char::is_whitespace) {
        Some((n, a)) => (n, a.trim()),
        None => (trimmed, ""),
    };

    match name.to_lowercase().as_str() {
        "help" | "h" | "?" => Ok(cmd_help(args)),
        "quit" | "exit" | "q" => Ok(CommandResult::Exit),
        "" => Ok(CommandResult::Continue),
        other => Ok(CommandResult::Output(format!(
            "Unknown command: :{}\nType :help for available commands.",
            other
        ))),
    }
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
  :help, :h, :?   Show this help
  :quit, :exit, :q Exit the REPL

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

  :help, :h, :?     Show help
  :help <topic>     Show help on a specific topic
  :quit, :exit, :q  Exit the REPL

Future commands (Phase 6a-M2+):
  :session          Session management
  :history          View command history
  :notebook         Notebook operations
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
        let result = dispatch("help").unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Fermata REPL")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_alias_h() {
        let result = dispatch("h").unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_dispatch_help_alias_question() {
        let result = dispatch("?").unwrap();
        assert!(matches!(result, CommandResult::Output(_)));
    }

    #[test]
    fn test_dispatch_help_topic_commands() {
        let result = dispatch("help commands").unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("REPL Commands")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_topic_expressions() {
        let result = dispatch("help expr").unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Fermata Expressions")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_topic_chat() {
        let result = dispatch("help chat").unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Chat Messages")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_help_unknown_topic() {
        let result = dispatch("help xyz").unwrap();
        match result {
            CommandResult::Output(s) => assert!(s.contains("Unknown help topic")),
            _ => panic!("Expected Output"),
        }
    }

    #[test]
    fn test_dispatch_quit() {
        let result = dispatch("quit").unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_exit() {
        let result = dispatch("exit").unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_q() {
        let result = dispatch("q").unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_empty() {
        let result = dispatch("").unwrap();
        assert_eq!(result, CommandResult::Continue);
    }

    #[test]
    fn test_dispatch_whitespace() {
        let result = dispatch("   ").unwrap();
        assert_eq!(result, CommandResult::Continue);
    }

    #[test]
    fn test_dispatch_unknown() {
        let result = dispatch("foobar").unwrap();
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
        let result = dispatch("HELP").unwrap();
        assert!(matches!(result, CommandResult::Output(_)));

        let result = dispatch("QUIT").unwrap();
        assert_eq!(result, CommandResult::Exit);
    }

    #[test]
    fn test_dispatch_with_leading_whitespace() {
        let result = dispatch("  help").unwrap();
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
}
