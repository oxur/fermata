//! Fermata Language: Ergonomic S-expression syntax for music notation.
//!
//! This module provides:
//! - A typed AST for Fermata syntax
//! - Parsing from S-expression text
//! - Compilation to Music IR
//!
//! # Example
//!
//! ```rust,ignore
//! use fermata::lang::compile;
//!
//! let source = r#"
//!     (score :title "Test"
//!       (part :piano
//!         (measure
//!           (note c4 :q))))
//! "#;
//!
//! let score = compile(source)?;
//! ```

pub mod ast;
pub mod attributes;
pub mod chord;
pub mod connectors;
pub mod defaults;
pub mod direction;
pub mod duration;
pub mod error;
pub mod grace;
pub mod measure;
pub mod note;
pub mod part;
pub mod pitch;
pub mod score;
pub mod tuplet;

mod compiler;

pub use ast::*;
pub use compiler::{
    check, compile, compile_measure_str, compile_note_str, compile_part_str, compile_pitch_str,
};
pub use error::{CompileError, CompileResult};
pub use part::CompiledPart;
pub use score::{compile_fermata_score, compile_score, parse_score_to_ast};

/// Compile Fermata source to Music IR
pub fn compile_str(source: &str) -> CompileResult<crate::ir::score::ScorePartwise> {
    compiler::compile(source)
}
