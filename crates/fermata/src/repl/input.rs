//! Input classification and multi-line detection for the REPL.

/// Classified user input type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputKind {
    /// A Fermata expression to evaluate (starts with '(' or bare symbol).
    Expression(String),
    /// A REPL command (starts with ':').
    Command(String),
    /// A chat message (starts with '/').
    Chat(ChatKind),
    /// Empty input - re-prompt.
    Empty,
}

/// Types of chat messages (MUD-style).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatKind {
    /// Default say: `/` followed by message or `/say message`.
    Say(String),
    /// Emote: `/em action` or `/me action`.
    Emote(String),
}

/// Classify a complete input string.
///
/// Classification rules:
/// - Empty or whitespace-only → `Empty`
/// - Starts with `/` → `Chat`
/// - Starts with `:` → `Command`
/// - Everything else → `Expression`
pub fn classify(input: &str) -> InputKind {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return InputKind::Empty;
    }

    // Chat commands: /
    if let Some(rest) = trimmed.strip_prefix('/') {
        return classify_chat(rest);
    }

    // REPL commands: :
    if let Some(rest) = trimmed.strip_prefix(':') {
        return InputKind::Command(rest.to_string());
    }

    // Everything else is an expression
    InputKind::Expression(input.to_string())
}

/// Classify chat input (after stripping leading '/').
fn classify_chat(input: &str) -> InputKind {
    let content = input.trim();

    if content.is_empty() {
        return InputKind::Chat(ChatKind::Say(String::new()));
    }

    // Parse chat subcommands
    let (cmd, rest) = match content.split_once(char::is_whitespace) {
        Some((c, r)) => (c, r.trim()),
        None => (content, ""),
    };

    let chat_kind = match cmd {
        "em" | "me" => ChatKind::Emote(rest.to_string()),
        "say" => ChatKind::Say(rest.to_string()),
        _ => {
            // Default: entire content is a say message
            ChatKind::Say(content.to_string())
        }
    };

    InputKind::Chat(chat_kind)
}

/// Check if input needs more lines (unbalanced parentheses).
///
/// Respects double-quoted strings: parens inside `"..."` are not counted.
/// Returns `true` if the input has unmatched open parentheses.
pub fn needs_continuation(input: &str) -> bool {
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut prev_backslash = false;

    for ch in input.chars() {
        if in_string {
            if ch == '"' && !prev_backslash {
                in_string = false;
            }
            prev_backslash = ch == '\\';
            continue;
        }

        match ch {
            '"' => in_string = true,
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }

    // Need continuation if we have unmatched open parens
    depth > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== classify() tests =====

    #[test]
    fn test_classify_empty() {
        assert_eq!(classify(""), InputKind::Empty);
    }

    #[test]
    fn test_classify_whitespace_only() {
        assert_eq!(classify("   "), InputKind::Empty);
        assert_eq!(classify("\t\n"), InputKind::Empty);
    }

    #[test]
    fn test_classify_expression_with_parens() {
        let result = classify("(score :title \"Test\")");
        assert!(matches!(result, InputKind::Expression(s) if s.contains("score")));
    }

    #[test]
    fn test_classify_expression_bare_symbol() {
        let result = classify("note");
        assert!(matches!(result, InputKind::Expression(s) if s == "note"));
    }

    #[test]
    fn test_classify_expression_preserves_whitespace() {
        let input = "  (score)  ";
        let result = classify(input);
        // Expression preserves original input including leading/trailing whitespace
        assert!(matches!(result, InputKind::Expression(s) if s == input));
    }

    #[test]
    fn test_classify_command_help() {
        let result = classify(":help");
        assert_eq!(result, InputKind::Command("help".to_string()));
    }

    #[test]
    fn test_classify_command_quit() {
        let result = classify(":quit");
        assert_eq!(result, InputKind::Command("quit".to_string()));
    }

    #[test]
    fn test_classify_command_with_args() {
        let result = classify(":history 10");
        assert_eq!(result, InputKind::Command("history 10".to_string()));
    }

    #[test]
    fn test_classify_command_with_leading_whitespace() {
        let result = classify("  :help");
        assert_eq!(result, InputKind::Command("help".to_string()));
    }

    #[test]
    fn test_classify_chat_simple() {
        let result = classify("/hello");
        assert_eq!(result, InputKind::Chat(ChatKind::Say("hello".to_string())));
    }

    #[test]
    fn test_classify_chat_with_space() {
        let result = classify("/ hello world");
        assert_eq!(
            result,
            InputKind::Chat(ChatKind::Say("hello world".to_string()))
        );
    }

    #[test]
    fn test_classify_chat_say_explicit() {
        let result = classify("/say hello");
        assert_eq!(result, InputKind::Chat(ChatKind::Say("hello".to_string())));
    }

    #[test]
    fn test_classify_chat_emote_em() {
        let result = classify("/em scratches head");
        assert_eq!(
            result,
            InputKind::Chat(ChatKind::Emote("scratches head".to_string()))
        );
    }

    #[test]
    fn test_classify_chat_emote_me() {
        let result = classify("/me is confused");
        assert_eq!(
            result,
            InputKind::Chat(ChatKind::Emote("is confused".to_string()))
        );
    }

    #[test]
    fn test_classify_chat_empty() {
        let result = classify("/");
        assert_eq!(result, InputKind::Chat(ChatKind::Say(String::new())));
    }

    // ===== needs_continuation() tests =====

    #[test]
    fn test_needs_continuation_empty() {
        assert!(!needs_continuation(""));
    }

    #[test]
    fn test_needs_continuation_balanced() {
        assert!(!needs_continuation("(score)"));
        assert!(!needs_continuation("(note (pitch c4))"));
        assert!(!needs_continuation("((()))"));
    }

    #[test]
    fn test_needs_continuation_unbalanced_open() {
        assert!(needs_continuation("("));
        assert!(needs_continuation("(score"));
        assert!(needs_continuation("(note ("));
    }

    #[test]
    fn test_needs_continuation_unbalanced_close() {
        // Extra close parens don't need continuation (they're a syntax error)
        assert!(!needs_continuation(")"));
        assert!(!needs_continuation("(score))"));
    }

    #[test]
    fn test_needs_continuation_string_with_parens() {
        // Parens inside strings should be ignored
        assert!(!needs_continuation("(score :title \"(test)\")"));
        assert!(needs_continuation("(score :title \"(test)\""));
    }

    #[test]
    fn test_needs_continuation_escaped_quote() {
        // Escaped quotes inside strings
        assert!(!needs_continuation(r#"(score :title "test\"more")"#));
        assert!(needs_continuation(r#"(score :title "test\"more""#));
    }

    #[test]
    fn test_needs_continuation_multiline() {
        let input = "(score\n  :title \"Test\"";
        assert!(needs_continuation(input));
    }

    #[test]
    fn test_needs_continuation_no_parens() {
        assert!(!needs_continuation("hello"));
        assert!(!needs_continuation(":quit"));
    }

    // ===== ChatKind tests =====

    #[test]
    fn test_chat_kind_say_debug() {
        let chat = ChatKind::Say("test".to_string());
        let debug_str = format!("{:?}", chat);
        assert!(debug_str.contains("Say"));
    }

    #[test]
    fn test_chat_kind_emote_clone() {
        let chat = ChatKind::Emote("thinks".to_string());
        let cloned = chat.clone();
        assert_eq!(chat, cloned);
    }

    // ===== InputKind tests =====

    #[test]
    fn test_input_kind_debug() {
        let input = InputKind::Empty;
        let debug_str = format!("{:?}", input);
        assert!(debug_str.contains("Empty"));
    }

    #[test]
    fn test_input_kind_clone() {
        let input = InputKind::Command("test".to_string());
        let cloned = input.clone();
        assert_eq!(input, cloned);
    }
}
