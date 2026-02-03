//! Custom prompt for the Fermata REPL.

use std::borrow::Cow;

use reedline::{Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus};

/// Primary prompt shown for new input.
const PRIMARY_PROMPT: &str = " 𝄐 N ❱ ";
/// Continuation prompt for multi-line input.
const CONTINUATION_PROMPT: &str = "  ───▷  ";

/// Custom prompt for the Fermata REPL.
#[derive(Debug, Default)]
pub struct FermataPrompt;

impl FermataPrompt {
    /// Create a new Fermata prompt.
    pub fn new() -> Self {
        Self
    }
}

impl Prompt for FermataPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        Cow::Borrowed(PRIMARY_PROMPT)
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
        let prompt = FermataPrompt::new();
        assert_eq!(prompt.render_prompt_left(), PRIMARY_PROMPT);
    }

    #[test]
    fn test_prompt_default() {
        let prompt = FermataPrompt::default();
        assert_eq!(prompt.render_prompt_left(), PRIMARY_PROMPT);
    }

    #[test]
    fn test_prompt_left() {
        let prompt = FermataPrompt::new();
        assert_eq!(prompt.render_prompt_left(), PRIMARY_PROMPT);
    }

    #[test]
    fn test_prompt_right_empty() {
        let prompt = FermataPrompt::new();
        assert_eq!(prompt.render_prompt_right(), "");
    }

    #[test]
    fn test_prompt_multiline_indicator() {
        let prompt = FermataPrompt::new();
        assert_eq!(
            prompt.render_prompt_multiline_indicator(),
            CONTINUATION_PROMPT
        );
    }

    #[test]
    fn test_prompt_indicator() {
        let prompt = FermataPrompt::new();
        assert_eq!(prompt.render_prompt_indicator(PromptEditMode::Default), "");
    }

    #[test]
    fn test_prompt_history_search_passing() {
        let prompt = FermataPrompt::new();
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
        let prompt = FermataPrompt::new();
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
        let prompt = FermataPrompt::new();
        let debug_str = format!("{:?}", prompt);
        assert!(debug_str.contains("FermataPrompt"));
    }
}
