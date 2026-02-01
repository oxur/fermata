//! Note compilation for Fermata syntax.
//!
//! This module handles compiling note S-expressions into IR Note types.

use crate::ir::note::Note;
use crate::sexpr::Sexpr;

use super::error::CompileResult;

/// Compile a note S-expression into an IR Note.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::note::compile_note;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(note c4 :q)")?;
/// let note = compile_note(&sexpr)?;
/// ```
pub fn compile_note(_sexpr: &Sexpr) -> CompileResult<Note> {
    // TODO: Implement in Milestone 2
    todo!("compile_note")
}

#[cfg(test)]
mod tests {
    // Tests will be added in Milestone 2 when compile_note is implemented
}
