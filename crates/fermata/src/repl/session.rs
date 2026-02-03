//! REPL session state management.
//!
//! This module manages the session state for the Fermata REPL, including:
//! - History of evaluated results (`*`, `**`, `***`)
//! - History of input expressions (`+`, `++`, `+++`)
//! - Display mode settings
//! - Render options

use crate::ir::score::ScorePartwise;
use crate::sexpr::Sexpr;

/// Display mode for REPL output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayMode {
    /// S-expression representation (structural, debugging) - default
    #[default]
    Sexpr,
    /// MusicXML output (interchange format)
    MusicXml,
    /// MEI (Music Encoding Initiative) output - requires `render` feature
    Mei,
    /// MIDI output (base64-encoded) - requires `render` feature
    Midi,
    /// Rendered PNG in terminal (via viuer) - requires `render` feature
    Png,
    /// No automatic display (value stored for later use)
    Silent,
}

impl DisplayMode {
    /// Parse a display mode from a string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sexpr" | "sexp" | "s" => Some(Self::Sexpr),
            "musicxml" | "xml" | "x" => Some(Self::MusicXml),
            "mei" | "m" => Some(Self::Mei),
            "midi" | "mid" => Some(Self::Midi),
            "png" | "p" | "render" | "r" => Some(Self::Png),
            "silent" | "quiet" | "q" => Some(Self::Silent),
            _ => None,
        }
    }

    /// Get the display name for this mode.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sexpr => "sexpr",
            Self::MusicXml => "musicxml",
            Self::Mei => "mei",
            Self::Midi => "midi",
            Self::Png => "png",
            Self::Silent => "silent",
        }
    }
}

/// Options for rendering scores to images.
#[derive(Debug, Clone)]
pub struct RenderOptions {
    /// Width in pixels (default: 800)
    pub width: u32,
    /// Page to render (1-indexed, default: 1)
    pub page: u32,
    /// Whether to show page number for multi-page scores
    pub show_page_info: bool,
    /// Dark mode override: None = auto-detect, Some(true/false) = manual
    pub dark_mode: Option<bool>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            width: 800,
            page: 1,
            show_page_info: true,
            dark_mode: None, // Auto-detect by default
        }
    }
}

impl RenderOptions {
    /// Create new render options with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the width in pixels.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the page to render (1-indexed).
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Set whether to show page info.
    pub fn show_page_info(mut self, show: bool) -> Self {
        self.show_page_info = show;
        self
    }
}

/// A value that can be stored in the REPL history.
///
/// Results are stored as `ScorePartwise` (the IR), while expressions
/// are stored as `Sexpr` (the parsed AST).
#[derive(Debug, Clone)]
pub enum HistoryValue {
    /// An evaluated result (ScorePartwise IR)
    Result(Box<ScorePartwise>),
    /// An input expression (Sexpr AST)
    Expression(Sexpr),
}

/// REPL session state.
///
/// Tracks evaluation history in Lisp style:
/// - `*`, `**`, `***` - last 3 evaluated results
/// - `+`, `++`, `+++` - last 3 input expressions
pub struct ReplSession {
    /// Current display mode
    display_mode: DisplayMode,
    /// Render options for PNG output
    render_options: RenderOptions,
    /// Dark mode override: None = auto-detect, Some(true/false) = manual
    dark_mode: Option<bool>,
    /// Last 3 evaluated results: [0]=*, [1]=**, [2]=***
    results: [Option<ScorePartwise>; 3],
    /// Last 3 input expressions: [0]=+, [1]=++, [2]=+++
    expressions: [Option<Sexpr>; 3],
    /// Whether we've warned about terminal image support
    warned_terminal_support: bool,
}

impl Default for ReplSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplSession {
    /// Create a new REPL session with default settings.
    pub fn new() -> Self {
        Self {
            display_mode: DisplayMode::default(),
            render_options: RenderOptions::default(),
            dark_mode: None, // Auto-detect by default
            results: [None, None, None],
            expressions: [None, None, None],
            warned_terminal_support: false,
        }
    }

    /// Get the current display mode.
    pub fn display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Set the display mode.
    pub fn set_display_mode(&mut self, mode: DisplayMode) {
        self.display_mode = mode;
    }

    /// Get the current render options (with dark_mode synced from session).
    pub fn render_options(&self) -> RenderOptions {
        let mut opts = self.render_options.clone();
        opts.dark_mode = self.dark_mode;
        opts
    }

    /// Get mutable render options.
    pub fn render_options_mut(&mut self) -> &mut RenderOptions {
        &mut self.render_options
    }

    /// Check if we've warned about terminal support.
    pub fn has_warned_terminal_support(&self) -> bool {
        self.warned_terminal_support
    }

    /// Mark that we've warned about terminal support.
    pub fn set_warned_terminal_support(&mut self) {
        self.warned_terminal_support = true;
    }

    /// Get the dark mode setting (None = auto, Some(true/false) = override).
    pub fn dark_mode(&self) -> Option<bool> {
        self.dark_mode
    }

    /// Set dark mode: None = auto-detect, Some(true) = on, Some(false) = off.
    pub fn set_dark_mode(&mut self, mode: Option<bool>) {
        self.dark_mode = mode;
    }

    /// Store a new result, rotating the history.
    ///
    /// The new result becomes `*`, the old `*` becomes `**`, etc.
    pub fn push_result(&mut self, result: ScorePartwise) {
        // Rotate: *** = old **, ** = old *, * = new
        self.results[2] = self.results[1].take();
        self.results[1] = self.results[0].take();
        self.results[0] = Some(result);
    }

    /// Store a new expression, rotating the history.
    ///
    /// The new expression becomes `+`, the old `+` becomes `++`, etc.
    pub fn push_expression(&mut self, expr: Sexpr) {
        // Rotate: +++ = old ++, ++ = old +, + = new
        self.expressions[2] = self.expressions[1].take();
        self.expressions[1] = self.expressions[0].take();
        self.expressions[0] = Some(expr);
    }

    /// Get a result by symbol name.
    ///
    /// - `"*"` → last result
    /// - `"**"` → second-to-last result
    /// - `"***"` → third-to-last result
    pub fn get_result(&self, symbol: &str) -> Option<&ScorePartwise> {
        match symbol {
            "*" => self.results[0].as_ref(),
            "**" => self.results[1].as_ref(),
            "***" => self.results[2].as_ref(),
            _ => None,
        }
    }

    /// Get an expression by symbol name.
    ///
    /// - `"+"` → last expression
    /// - `"++"` → second-to-last expression
    /// - `"+++"` → third-to-last expression
    pub fn get_expression(&self, symbol: &str) -> Option<&Sexpr> {
        match symbol {
            "+" => self.expressions[0].as_ref(),
            "++" => self.expressions[1].as_ref(),
            "+++" => self.expressions[2].as_ref(),
            _ => None,
        }
    }

    /// Check if a symbol is a history variable.
    pub fn is_history_symbol(symbol: &str) -> bool {
        matches!(symbol, "*" | "**" | "***" | "+" | "++" | "+++")
    }

    /// Get a history value by symbol (either result or expression).
    pub fn get_history_value(&self, symbol: &str) -> Option<HistoryValue> {
        if let Some(result) = self.get_result(symbol) {
            Some(HistoryValue::Result(Box::new(result.clone())))
        } else {
            self.get_expression(symbol)
                .map(|e| HistoryValue::Expression(e.clone()))
        }
    }

    /// Check if there are any stored results.
    pub fn has_results(&self) -> bool {
        self.results[0].is_some()
    }

    /// Check if there are any stored expressions.
    pub fn has_expressions(&self) -> bool {
        self.expressions[0].is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::part::PartList;

    // Helper to create a minimal ScorePartwise for testing
    fn test_score(title: &str) -> ScorePartwise {
        use crate::ir::score::Work;
        ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_title: Some(title.to_string()),
                ..Default::default()
            }),
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        }
    }

    // Helper to create a test Sexpr
    fn test_sexpr(name: &str) -> Sexpr {
        Sexpr::List(vec![
            Sexpr::symbol("score"),
            Sexpr::keyword("title"),
            Sexpr::String(name.to_string()),
        ])
    }

    // === DisplayMode tests ===

    #[test]
    fn test_display_mode_default() {
        assert_eq!(DisplayMode::default(), DisplayMode::Sexpr);
    }

    #[test]
    fn test_display_mode_parse() {
        assert_eq!(DisplayMode::parse("sexpr"), Some(DisplayMode::Sexpr));
        assert_eq!(DisplayMode::parse("sexp"), Some(DisplayMode::Sexpr));
        assert_eq!(DisplayMode::parse("s"), Some(DisplayMode::Sexpr));
        assert_eq!(DisplayMode::parse("musicxml"), Some(DisplayMode::MusicXml));
        assert_eq!(DisplayMode::parse("xml"), Some(DisplayMode::MusicXml));
        assert_eq!(DisplayMode::parse("mei"), Some(DisplayMode::Mei));
        assert_eq!(DisplayMode::parse("m"), Some(DisplayMode::Mei));
        assert_eq!(DisplayMode::parse("midi"), Some(DisplayMode::Midi));
        assert_eq!(DisplayMode::parse("mid"), Some(DisplayMode::Midi));
        assert_eq!(DisplayMode::parse("png"), Some(DisplayMode::Png));
        assert_eq!(DisplayMode::parse("render"), Some(DisplayMode::Png));
        assert_eq!(DisplayMode::parse("silent"), Some(DisplayMode::Silent));
        assert_eq!(DisplayMode::parse("quiet"), Some(DisplayMode::Silent));
        assert_eq!(DisplayMode::parse("invalid"), None);
    }

    #[test]
    fn test_display_mode_parse_case_insensitive() {
        assert_eq!(DisplayMode::parse("SEXPR"), Some(DisplayMode::Sexpr));
        assert_eq!(DisplayMode::parse("MusicXML"), Some(DisplayMode::MusicXml));
        assert_eq!(DisplayMode::parse("MEI"), Some(DisplayMode::Mei));
        assert_eq!(DisplayMode::parse("MIDI"), Some(DisplayMode::Midi));
        assert_eq!(DisplayMode::parse("PNG"), Some(DisplayMode::Png));
    }

    #[test]
    fn test_display_mode_name() {
        assert_eq!(DisplayMode::Sexpr.name(), "sexpr");
        assert_eq!(DisplayMode::MusicXml.name(), "musicxml");
        assert_eq!(DisplayMode::Mei.name(), "mei");
        assert_eq!(DisplayMode::Midi.name(), "midi");
        assert_eq!(DisplayMode::Png.name(), "png");
        assert_eq!(DisplayMode::Silent.name(), "silent");
    }

    // === RenderOptions tests ===

    #[test]
    fn test_render_options_default() {
        let opts = RenderOptions::default();
        assert_eq!(opts.width, 800);
        assert_eq!(opts.page, 1);
        assert!(opts.show_page_info);
    }

    #[test]
    fn test_render_options_builder() {
        let opts = RenderOptions::new()
            .width(1200)
            .page(2)
            .show_page_info(false);
        assert_eq!(opts.width, 1200);
        assert_eq!(opts.page, 2);
        assert!(!opts.show_page_info);
    }

    // === ReplSession result history tests ===

    #[test]
    fn test_session_new() {
        let session = ReplSession::new();
        assert_eq!(session.display_mode(), DisplayMode::Sexpr);
        assert!(!session.has_results());
        assert!(!session.has_expressions());
    }

    #[test]
    fn test_session_push_result_single() {
        let mut session = ReplSession::new();
        session.push_result(test_score("First"));

        assert!(session.has_results());
        assert!(session.get_result("*").is_some());
        assert!(session.get_result("**").is_none());
        assert!(session.get_result("***").is_none());
    }

    #[test]
    fn test_session_push_result_rotation() {
        let mut session = ReplSession::new();
        session.push_result(test_score("First"));
        session.push_result(test_score("Second"));
        session.push_result(test_score("Third"));

        // * should be Third, ** should be Second, *** should be First
        let star = session.get_result("*").unwrap();
        let star_star = session.get_result("**").unwrap();
        let star_star_star = session.get_result("***").unwrap();

        assert_eq!(
            star.work.as_ref().unwrap().work_title,
            Some("Third".to_string())
        );
        assert_eq!(
            star_star.work.as_ref().unwrap().work_title,
            Some("Second".to_string())
        );
        assert_eq!(
            star_star_star.work.as_ref().unwrap().work_title,
            Some("First".to_string())
        );
    }

    #[test]
    fn test_session_push_result_overflow() {
        let mut session = ReplSession::new();
        session.push_result(test_score("First"));
        session.push_result(test_score("Second"));
        session.push_result(test_score("Third"));
        session.push_result(test_score("Fourth"));

        // First should be gone, Fourth is *, Third is **, Second is ***
        let star = session.get_result("*").unwrap();
        let star_star_star = session.get_result("***").unwrap();

        assert_eq!(
            star.work.as_ref().unwrap().work_title,
            Some("Fourth".to_string())
        );
        assert_eq!(
            star_star_star.work.as_ref().unwrap().work_title,
            Some("Second".to_string())
        );
    }

    // === ReplSession expression history tests ===

    #[test]
    fn test_session_push_expression_single() {
        let mut session = ReplSession::new();
        session.push_expression(test_sexpr("First"));

        assert!(session.has_expressions());
        assert!(session.get_expression("+").is_some());
        assert!(session.get_expression("++").is_none());
        assert!(session.get_expression("+++").is_none());
    }

    #[test]
    fn test_session_push_expression_rotation() {
        let mut session = ReplSession::new();
        session.push_expression(test_sexpr("First"));
        session.push_expression(test_sexpr("Second"));
        session.push_expression(test_sexpr("Third"));

        // + should be Third, ++ should be Second, +++ should be First
        let plus = session.get_expression("+").unwrap();
        let plus_plus = session.get_expression("++").unwrap();
        let plus_plus_plus = session.get_expression("+++").unwrap();

        // Check that they're different (we can't easily check content without more helpers)
        assert!(matches!(plus, Sexpr::List(_)));
        assert!(matches!(plus_plus, Sexpr::List(_)));
        assert!(matches!(plus_plus_plus, Sexpr::List(_)));
    }

    // === History symbol tests ===

    #[test]
    fn test_is_history_symbol() {
        assert!(ReplSession::is_history_symbol("*"));
        assert!(ReplSession::is_history_symbol("**"));
        assert!(ReplSession::is_history_symbol("***"));
        assert!(ReplSession::is_history_symbol("+"));
        assert!(ReplSession::is_history_symbol("++"));
        assert!(ReplSession::is_history_symbol("+++"));
        assert!(!ReplSession::is_history_symbol("****"));
        assert!(!ReplSession::is_history_symbol("++++"));
        assert!(!ReplSession::is_history_symbol("foo"));
    }

    #[test]
    fn test_get_result_invalid_symbol() {
        let session = ReplSession::new();
        assert!(session.get_result("foo").is_none());
        assert!(session.get_result("****").is_none());
        assert!(session.get_result("+").is_none()); // + is expression, not result
    }

    #[test]
    fn test_get_expression_invalid_symbol() {
        let session = ReplSession::new();
        assert!(session.get_expression("foo").is_none());
        assert!(session.get_expression("++++").is_none());
        assert!(session.get_expression("*").is_none()); // * is result, not expression
    }

    // === Display mode setting tests ===

    #[test]
    fn test_session_set_display_mode() {
        let mut session = ReplSession::new();
        assert_eq!(session.display_mode(), DisplayMode::Sexpr);

        session.set_display_mode(DisplayMode::Png);
        assert_eq!(session.display_mode(), DisplayMode::Png);

        session.set_display_mode(DisplayMode::MusicXml);
        assert_eq!(session.display_mode(), DisplayMode::MusicXml);
    }

    // === Terminal warning tests ===

    #[test]
    fn test_session_terminal_warning() {
        let mut session = ReplSession::new();
        assert!(!session.has_warned_terminal_support());

        session.set_warned_terminal_support();
        assert!(session.has_warned_terminal_support());
    }

    // === get_history_value tests ===

    #[test]
    fn test_get_history_value_result() {
        let mut session = ReplSession::new();
        session.push_result(test_score("Test"));

        let value = session.get_history_value("*");
        assert!(matches!(value, Some(HistoryValue::Result(_))));
    }

    #[test]
    fn test_get_history_value_expression() {
        let mut session = ReplSession::new();
        session.push_expression(test_sexpr("Test"));

        let value = session.get_history_value("+");
        assert!(matches!(value, Some(HistoryValue::Expression(_))));
    }

    #[test]
    fn test_get_history_value_none() {
        let session = ReplSession::new();
        assert!(session.get_history_value("*").is_none());
        assert!(session.get_history_value("+").is_none());
        assert!(session.get_history_value("foo").is_none());
    }
}
