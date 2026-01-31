//! Fermata IR (Intermediate Representation) types.
//!
//! This module defines the typed data structures that represent MusicXML content.
//! The IR provides a lossless, round-trippable representation of MusicXML documents.
//!
//! # Module Organization
//!
//! - [`common`] - Shared types, enums, and type aliases
//! - [`pitch`] - Pitch representation (Step, Octave, etc.)
//! - [`duration`] - Duration and rhythm types (NoteType, Dot, etc.)
//! - [`beam`] - Beam, stem, and notehead types
//! - [`note`] - Note, rest, and grace note types
//! - [`attributes`] - Measure attributes (Key, Time, Clef, Barline)
//! - [`direction`] - Directions (Dynamics, Wedge, Metronome, etc.)
//! - [`notation`] - Notations (Articulations, Ornaments, Slurs, etc.)
//! - [`voice`] - Voice-related types (Backup, Forward)
//! - [`lyric`] - Lyric types
//! - [`measure`] - Measure and music data types
//! - [`part`] - Part and part-list types
//! - [`score`] - Score-level types
//!
//! # Example
//!
//! ```
//! use fermata::ir::{Pitch, Step, ScorePartwise};
//!
//! let pitch = Pitch {
//!     step: Step::C,
//!     alter: None,
//!     octave: 4,
//! };
//! ```

pub mod attributes;
pub mod beam;
pub mod common;
pub mod direction;
pub mod duration;
pub mod lyric;
pub mod measure;
pub mod notation;
pub mod note;
pub mod part;
pub mod pitch;
pub mod score;
pub mod voice;

// Re-export main types for convenience
pub use attributes::{Attributes, Barline, Clef, Key, Time};
pub use beam::{Beam, Notehead, Stem};
pub use direction::{Direction, DirectionType, Dynamics, Metronome, Wedge};
pub use duration::{Dot, NoteType, NoteTypeValue, TimeModification};
pub use lyric::{Lyric, Syllabic};
pub use measure::{Measure, MusicDataElement};
pub use notation::{Articulations, Fermata, Notations, Ornaments, Slur, Technical, Tied, Tuplet};
pub use note::{Accidental, FullNote, Grace, Note, NoteContent, Rest};
pub use part::{Part, PartGroup, PartList, PartListElement, PartName, ScorePart};
pub use pitch::{Pitch, Step, Unpitched};
pub use score::ScorePartwise;
pub use voice::{Backup, Forward};

// Re-export common types
pub use common::*;
