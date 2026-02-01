//! Fermata -> IR compiler.
//!
//! This module orchestrates the compilation of Fermata syntax to Music IR.

use crate::ir::score::ScorePartwise;
use crate::sexpr::parser::parse as parse_sexpr;
use crate::sexpr::Sexpr;

use super::error::{CompileError, CompileResult};

/// Compile Fermata source text to Music IR.
///
/// # Example
///
/// ```rust,ignore
/// use fermata::lang::compile;
///
/// let source = r#"(note c4 :q)"#;
/// let result = compile(source);
/// ```
pub fn compile(source: &str) -> CompileResult<ScorePartwise> {
    // Step 1: Parse S-expression
    let sexpr = parse_sexpr(source)?;

    // Step 2: Interpret as Fermata AST
    let fermata_ast = interpret_sexpr(&sexpr)?;

    // Step 3: Compile to IR
    compile_to_ir(&fermata_ast)
}

/// Interpret an S-expression as Fermata AST
fn interpret_sexpr(_sexpr: &Sexpr) -> CompileResult<super::ast::FermataScore> {
    // TODO: Implement in later milestones
    todo!("interpret_sexpr")
}

/// Compile Fermata AST to Music IR
fn compile_to_ir(_ast: &super::ast::FermataScore) -> CompileResult<ScorePartwise> {
    // TODO: Implement in later milestones
    todo!("compile_to_ir")
}

/// Compile a single note (for testing/incremental development)
pub fn compile_note_str(source: &str) -> CompileResult<crate::ir::note::Note> {
    let sexpr = parse_sexpr(source)?;
    super::note::compile_note(&sexpr)
}

/// Compile a single pitch (for testing)
pub fn compile_pitch_str(source: &str) -> CompileResult<crate::ir::pitch::Pitch> {
    let sexpr = parse_sexpr(source)?;
    match &sexpr {
        Sexpr::Symbol(s) => super::pitch::parse_pitch(s),
        _ => Err(CompileError::type_mismatch(
            "pitch symbol",
            format!("{:?}", sexpr),
        )),
    }
}
