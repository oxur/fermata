//! MusicXML emission module.
//!
//! This module provides functionality to emit MusicXML 4.0 documents from the IR types.
//! The emission logic is organized into focused submodules:
//!
//! - `score`: Score-level emission (emit_score, part-list, measures)
//! - `note`: Note emission (pitch, rest, grace notes, beams, stems)
//! - `attributes`: Attributes emission (key, time, clef, transpose)
//! - `direction`: Direction emission (dynamics, wedges, metronome, pedal)
//! - `notation`: Notation emission (tied, slur, tuplet, articulations)
//! - `barline`: Barline emission (repeats, endings, fermatas)
//! - `voice`: Voice navigation (backup, forward)
//! - `helpers`: String conversion utilities
//!
//! # Example
//!
//! ```ignore
//! use fermata::ir::ScorePartwise;
//! use fermata::musicxml::emit::emit_score;
//!
//! let score: ScorePartwise = // ... create or parse a score
//! let xml = emit_score(&score)?;
//! println!("{}", xml);
//! ```

mod attributes;
mod barline;
mod direction;
mod helpers;
mod notation;
mod note;
mod score;
mod voice;

// Re-export the main public API
pub use helpers::note_type_value_to_string;
pub use score::emit_score;
