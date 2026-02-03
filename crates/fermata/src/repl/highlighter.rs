//! Syntax highlighting for the Fermata REPL.
//!
//! Provides rainbow parentheses and keyword highlighting for S-expressions.

use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

/// Rainbow colors for nested parentheses.
const RAINBOW_COLORS: [Color; 6] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
];

/// Fermata syntax highlighter with rainbow parentheses.
pub struct FermataHighlighter {
    /// Whether colors are enabled
    use_colors: bool,
}

impl FermataHighlighter {
    /// Create a new highlighter.
    pub fn new(use_colors: bool) -> Self {
        Self { use_colors }
    }
}

impl Highlighter for FermataHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut styled = StyledText::new();

        if !self.use_colors {
            // No colors - just return plain text
            styled.push((Style::default(), line.to_string()));
            return styled;
        }

        let mut paren_depth: i32 = 0;
        let mut current_word = String::new();
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '(' => {
                    // Flush any pending word
                    if !current_word.is_empty() {
                        styled.push((style_for_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    // Color the paren based on depth
                    let color = RAINBOW_COLORS[paren_depth as usize % RAINBOW_COLORS.len()];
                    styled.push((Style::new().fg(color).bold(), ch.to_string()));
                    paren_depth += 1;
                }
                ')' => {
                    // Flush any pending word
                    if !current_word.is_empty() {
                        styled.push((style_for_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    // Color the paren based on depth (use depth-1 to match opening)
                    paren_depth = (paren_depth - 1).max(0);
                    let color = RAINBOW_COLORS[paren_depth as usize % RAINBOW_COLORS.len()];
                    styled.push((Style::new().fg(color).bold(), ch.to_string()));
                }
                ' ' | '\t' | '\n' => {
                    // Whitespace - flush word and add whitespace
                    if !current_word.is_empty() {
                        styled.push((style_for_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    styled.push((Style::default(), ch.to_string()));
                }
                '"' => {
                    // String literal - consume until closing quote
                    if !current_word.is_empty() {
                        styled.push((style_for_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    let mut string_content = String::from(ch);
                    while let Some(&next_ch) = chars.peek() {
                        string_content.push(chars.next().unwrap());
                        if next_ch == '"' {
                            break;
                        }
                        // Handle escape sequences
                        if next_ch == '\\' {
                            if let Some(escaped) = chars.next() {
                                string_content.push(escaped);
                            }
                        }
                    }
                    styled.push((Style::new().fg(Color::Green), string_content));
                }
                ';' => {
                    // Comment - rest of line
                    if !current_word.is_empty() {
                        styled.push((style_for_word(&current_word), current_word.clone()));
                        current_word.clear();
                    }
                    let mut comment = String::from(ch);
                    for remaining in chars.by_ref() {
                        comment.push(remaining);
                    }
                    styled.push((Style::new().fg(Color::DarkGray).italic(), comment));
                }
                _ => {
                    current_word.push(ch);
                }
            }
        }

        // Flush any remaining word
        if !current_word.is_empty() {
            styled.push((style_for_word(&current_word), current_word));
        }

        styled
    }
}

/// Determine the style for a word based on its content.
fn style_for_word(word: &str) -> Style {
    // Keywords (form names)
    if is_keyword(word) {
        return Style::new().fg(Color::Blue).bold();
    }

    // Keyword arguments (starting with :)
    if word.starts_with(':') {
        return Style::new().fg(Color::Cyan);
    }

    // Numbers
    if word.parse::<f64>().is_ok() {
        return Style::new().fg(Color::Magenta);
    }

    // Notes (e.g., c4, d#5, bb3)
    if is_note(word) {
        return Style::new().fg(Color::Yellow);
    }

    // Commands (starting with :)
    if word.starts_with(':') {
        return Style::new().fg(Color::Green);
    }

    // Default
    Style::default()
}

/// Check if a word is a Fermata keyword.
fn is_keyword(word: &str) -> bool {
    matches!(
        word.to_lowercase().as_str(),
        "score"
            | "part"
            | "measure"
            | "note"
            | "rest"
            | "chord"
            | "key"
            | "time"
            | "clef"
            | "barline"
            | "p"
            | "pp"
            | "ppp"
            | "f"
            | "ff"
            | "fff"
            | "mp"
            | "mf"
            | "sfz"
            | "fp"
            | "transpose"
            | "invert"
            | "retrograde"
    )
}

/// Check if a word looks like a note (e.g., c4, d#5, bb3).
fn is_note(word: &str) -> bool {
    let word_lower = word.to_lowercase();
    let mut chars = word_lower.chars();

    // First char must be a-g
    match chars.next() {
        Some('a'..='g') => {}
        _ => return false,
    }

    // Optional accidental (#, b, ##, bb)
    let mut rest: String = chars.collect();
    if rest.starts_with("##") {
        rest = rest[2..].to_string();
    } else if rest.starts_with('#') || rest.starts_with('b') {
        // Check for double flat
        if rest.starts_with("bb") {
            rest = rest[2..].to_string();
        } else {
            rest = rest[1..].to_string();
        }
    }

    // Must end with octave number (0-9)
    !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_note() {
        assert!(is_note("c4"));
        assert!(is_note("C4"));
        assert!(is_note("d#5"));
        assert!(is_note("Bb3"));
        assert!(is_note("f##4"));
        assert!(is_note("gbb2"));
        assert!(!is_note("hello"));
        assert!(!is_note("c"));
        assert!(!is_note("4"));
        assert!(!is_note(":q"));
    }

    #[test]
    fn test_is_keyword() {
        assert!(is_keyword("score"));
        assert!(is_keyword("SCORE"));
        assert!(is_keyword("note"));
        assert!(is_keyword("ff"));
        assert!(!is_keyword("hello"));
        assert!(!is_keyword("c4"));
    }
}
