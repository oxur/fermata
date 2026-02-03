//! # 𝄐 Fermata
//!
//! **An S-expression DSL for working with MusicXML**
//!
//! Fermata is a Lisp-like domain-specific language for describing musical notation.
//! It compiles to MusicXML and can import from MusicXML.
//!
//! ## Quick Example
//!
//! ```
//! use fermata::{parse, compile, CompileOptions};
//!
//! let source = r#"
//!     (score :title "Test"
//!       (part :piano
//!         (measure (note c4 :q))))
//! "#;
//!
//! let score = parse(source).unwrap();
//! let xml = compile(&score, CompileOptions::default()).unwrap();
//! assert!(xml.contains("Test"));
//! ```
//!
//! ## Parsing and Compiling
//!
//! The two main entry points are:
//! - [`parse`] - Parse Fermata source text into an AST
//! - [`compile`] - Compile an AST to a target format (MusicXML, S-expression)
//!
//! ## AST Types
//!
//! The AST represents the structure of a Fermata score:
//! - [`Score`] - A complete score with title, composer, and parts
//! - [`Part`] - A musical part with measures
//! - [`Measure`] - A measure containing notes, rests, chords, etc.
//! - [`Note`], [`Rest`], [`Chord`] - Musical elements
//! - [`Pitch`], [`Duration`] - Note attributes
//!
//! ## Lower-Level Modules
//!
//! For more control, use the lower-level modules:
//! - [`lang`] - Language parsing and compilation
//! - [`musicxml`] - MusicXML parsing and emission
//! - [`sexpr`] - S-expression parsing and printing
//! - [`ir`] - Intermediate representation (MusicXML-faithful)

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod ir;
pub mod lang;
pub mod musicxml;
pub mod repl;
pub mod sexpr;

// Re-export AST types with cleaner names
pub use lang::ast::{
    ArpeggiateDirection, Articulation, BarlineSpec, ClefSpec, DurationBase, DynamicMark,
    EndingAction, FermataChord as Chord, FermataDuration as Duration, FermataMark,
    FermataMeasure as Measure, FermataNote as Note, FermataPart as Part, FermataPitch as Pitch,
    FermataRest as Rest, FermataScore as Score, FermataTuplet as Tuplet, KeySpec, LyricSpec,
    MeasureElement, Mode, Ornament, PitchAlter, PitchStep, SlurMark, StemDirection, Syllabic,
    TempoMark, TieMark, TimeSpec,
};

// Re-export error types
pub use lang::error::{CompileError, CompileResult};

/// Output format for compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Target {
    /// MusicXML format (default)
    #[default]
    MusicXml,
    /// S-expression format (for debugging/round-trip)
    Sexpr,
}

/// Options for compilation.
#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    /// Target output format
    pub target: Target,
}

impl CompileOptions {
    /// Create options for MusicXML output.
    pub fn musicxml() -> Self {
        Self {
            target: Target::MusicXml,
        }
    }

    /// Create options for S-expression output.
    pub fn sexpr() -> Self {
        Self {
            target: Target::Sexpr,
        }
    }
}

/// Parse Fermata source code into an AST.
///
/// # Example
///
/// ```
/// use fermata::parse;
///
/// let source = r#"(score :title "Test" (part :piano (measure (note c4 :q))))"#;
/// let score = parse(source).unwrap();
/// assert_eq!(score.title, Some("Test".to_string()));
/// assert_eq!(score.parts.len(), 1);
/// ```
///
/// # Errors
///
/// Returns [`CompileError`] if the source contains syntax errors or invalid constructs.
pub fn parse(source: &str) -> CompileResult<Score> {
    let sexpr = sexpr::parser::parse(source)?;
    lang::score::parse_score_to_ast(&sexpr)
}

/// Compile an AST to the specified output format.
///
/// # Example
///
/// ```
/// use fermata::{parse, compile, CompileOptions, Target};
///
/// let source = r#"(score :title "Test" (part :piano (measure (note c4 :q))))"#;
/// let score = parse(source).unwrap();
///
/// // Compile to MusicXML (default)
/// let xml = compile(&score, CompileOptions::default()).unwrap();
/// assert!(xml.contains("<work-title>Test</work-title>"));
///
/// // Compile to S-expression
/// let sexpr = compile(&score, CompileOptions::sexpr()).unwrap();
/// assert!(sexpr.contains("score"));
/// ```
///
/// # Errors
///
/// Returns [`CompileError`] if the AST cannot be compiled to the target format.
pub fn compile(score: &Score, options: CompileOptions) -> CompileResult<String> {
    // Compile AST to IR
    let ir = lang::score::compile_fermata_score(score)?;

    match options.target {
        Target::MusicXml => musicxml::emit(&ir).map_err(|e| CompileError::emit(e.to_string())),
        Target::Sexpr => {
            use sexpr::ToSexpr;
            Ok(sexpr::print_sexpr(&ir.to_sexpr()))
        }
    }
}

/// Compile an AST to a specific target format.
///
/// This is a convenience function equivalent to:
/// ```ignore
/// compile(score, CompileOptions { target })
/// ```
///
/// # Example
///
/// ```
/// use fermata::{parse, compile_to, Target};
///
/// let score = parse("(score (part :piano (measure (note c4 :q))))").unwrap();
/// let xml = compile_to(&score, Target::MusicXml).unwrap();
/// ```
pub fn compile_to(score: &Score, target: Target) -> CompileResult<String> {
    compile(score, CompileOptions { target })
}

/// Check if Fermata source is valid without fully compiling.
///
/// This is faster than [`parse`] followed by [`compile`] when you only
/// need to validate syntax.
///
/// # Example
///
/// ```
/// use fermata::check;
///
/// assert!(check("(score (part :piano))").is_ok());
/// assert!(check("(invalid-form)").is_err());
/// ```
///
/// # Errors
///
/// Returns [`CompileError`] describing what's wrong with the source.
pub fn check(source: &str) -> CompileResult<()> {
    lang::check(source)
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_score() {
        let source = "(score :title \"Test\")";
        let score = parse(source).unwrap();
        assert_eq!(score.title, Some("Test".to_string()));
    }

    #[test]
    fn test_parse_score_with_part() {
        let source = "(score (part :piano (measure (note c4 :q))))";
        let score = parse(source).unwrap();
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].name, "Piano");
    }

    #[test]
    fn test_compile_to_musicxml() {
        let source = "(score :title \"Test\" (part :piano (measure (note c4 :q))))";
        let score = parse(source).unwrap();
        let xml = compile(&score, CompileOptions::musicxml()).unwrap();
        assert!(xml.contains("<work-title>Test</work-title>"));
        assert!(xml.contains("<part-name>Piano</part-name>"));
    }

    #[test]
    fn test_compile_to_sexpr() {
        let source = "(score :title \"Test\")";
        let score = parse(source).unwrap();
        let sexpr = compile(&score, CompileOptions::sexpr()).unwrap();
        assert!(sexpr.contains("score"));
        assert!(sexpr.contains("Test"));
    }

    #[test]
    fn test_compile_to_convenience() {
        let source = "(score)";
        let score = parse(source).unwrap();
        let xml = compile_to(&score, Target::MusicXml).unwrap();
        assert!(xml.contains("score-partwise"));
    }

    #[test]
    fn test_check_valid() {
        assert!(check("(score (part :piano))").is_ok());
    }

    #[test]
    fn test_check_invalid() {
        assert!(check("(invalid-form)").is_err());
    }

    #[test]
    fn test_check_syntax_error() {
        assert!(check("(score (").is_err());
    }

    #[test]
    fn test_compile_options_default() {
        let opts = CompileOptions::default();
        assert_eq!(opts.target, Target::MusicXml);
    }

    #[test]
    fn test_target_default() {
        assert_eq!(Target::default(), Target::MusicXml);
    }
}
