//! Custom prompt for the Fermata REPL.

use std::borrow::Cow;
use std::sync::atomic::{AtomicUsize, Ordering};

use nu_ansi_term::Color;
use reedline::{Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus};

/// Continuation prompt for multi-line input.
const CONTINUATION_PROMPT: &str = "    ───▷  ";

/// Custom prompt for the Fermata REPL.
///
/// Displays a line number that increments with each prompt, with optional colors:
/// - Number: yellow
/// - Bracket (❱): dark green
#[derive(Debug)]
pub struct FermataPrompt {
    /// Line counter (increments with each prompt display).
    counter: AtomicUsize,
    /// Whether to use colors.
    use_colors: bool,
}

impl Default for FermataPrompt {
    fn default() -> Self {
        Self::new(true)
    }
}

impl FermataPrompt {
    /// Create a new Fermata prompt.
    pub fn new(use_colors: bool) -> Self {
        Self {
            counter: AtomicUsize::new(1),
            use_colors,
        }
    }

    /// Get the current line number (does not increment).
    fn line_number(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    /// Increment the line counter. Call this after a complete input is submitted.
    pub fn increment(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }
}

impl Prompt for FermataPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        let n = self.line_number();

        if self.use_colors {
            let number = Color::Yellow.paint(format!("{}", n));
            let bracket = Color::Green.paint("❱");
            Cow::Owned(format!(" 𝄐 [{}] {} ", number, bracket))
        } else {
            Cow::Owned(format!(" 𝄐 [{}] ❱ ", n))
        }
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _mode: PromptEditMode) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed(CONTINUATION_PROMPT)
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_new() {
        let prompt = FermataPrompt::new(false);
        let left = prompt.render_prompt_left();
        assert!(left.contains("[1]"));
    }

    #[test]
    fn test_prompt_default_uses_colors() {
        let prompt = FermataPrompt::default();
        assert!(prompt.use_colors);
    }

    #[test]
    fn test_prompt_counter_stable_without_increment() {
        let prompt = FermataPrompt::new(false);
        // Multiple renders should show the same number
        let first = prompt.render_prompt_left();
        let second = prompt.render_prompt_left();
        let third = prompt.render_prompt_left();

        assert!(first.contains("[1]"));
        assert!(second.contains("[1]"));
        assert!(third.contains("[1]"));
    }

    #[test]
    fn test_prompt_counter_increments_on_call() {
        let prompt = FermataPrompt::new(false);

        assert!(prompt.render_prompt_left().contains("[1]"));
        prompt.increment();
        assert!(prompt.render_prompt_left().contains("[2]"));
        prompt.increment();
        assert!(prompt.render_prompt_left().contains("[3]"));
    }

    #[test]
    fn test_prompt_with_colors() {
        let prompt = FermataPrompt::new(true);
        let left = prompt.render_prompt_left();
        // Should contain ANSI escape codes for yellow
        assert!(left.contains("\x1b["));
    }

    #[test]
    fn test_prompt_without_colors() {
        let prompt = FermataPrompt::new(false);
        let left = prompt.render_prompt_left();
        // Should NOT contain ANSI escape codes
        assert!(!left.contains("\x1b["));
        assert!(left.contains("❱"));
    }

    #[test]
    fn test_prompt_right_empty() {
        let prompt = FermataPrompt::new(false);
        assert_eq!(prompt.render_prompt_right(), "");
    }

    #[test]
    fn test_prompt_multiline_indicator() {
        let prompt = FermataPrompt::new(false);
        assert_eq!(
            prompt.render_prompt_multiline_indicator(),
            CONTINUATION_PROMPT
        );
    }

    #[test]
    fn test_prompt_indicator() {
        let prompt = FermataPrompt::new(false);
        assert_eq!(prompt.render_prompt_indicator(PromptEditMode::Default), "");
    }

    #[test]
    fn test_prompt_history_search_passing() {
        let prompt = FermataPrompt::new(false);
        let search = PromptHistorySearch {
            status: PromptHistorySearchStatus::Passing,
            term: "test".to_string(),
        };
        let result = prompt.render_prompt_history_search_indicator(search);
        assert!(result.contains("reverse-search"));
        assert!(result.contains("test"));
        assert!(!result.contains("failing"));
    }

    #[test]
    fn test_prompt_history_search_failing() {
        let prompt = FermataPrompt::new(false);
        let search = PromptHistorySearch {
            status: PromptHistorySearchStatus::Failing,
            term: "xyz".to_string(),
        };
        let result = prompt.render_prompt_history_search_indicator(search);
        assert!(result.contains("failing"));
        assert!(result.contains("xyz"));
    }

    #[test]
    fn test_prompt_debug() {
        let prompt = FermataPrompt::new(false);
        let debug_str = format!("{:?}", prompt);
        assert!(debug_str.contains("FermataPrompt"));
    }
}
