//! Pitch parsing for Fermata syntax.
//!
//! This module handles parsing pitch strings like "c4", "f#5", "bb3" into
//! IR pitch types.

use crate::ir::pitch::Pitch;

use super::error::CompileResult;

/// Parse a pitch string into an IR Pitch.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::pitch::parse_pitch;
///
/// let pitch = parse_pitch("c4")?;
/// let pitch = parse_pitch("f#5")?;
/// let pitch = parse_pitch("bb3")?;
/// ```
pub fn parse_pitch(_pitch_str: &str) -> CompileResult<Pitch> {
    // TODO: Implement in Milestone 2
    todo!("parse_pitch")
}

#[cfg(test)]
mod tests {
    // Tests will be added in Milestone 2 when parse_pitch is implemented
}
