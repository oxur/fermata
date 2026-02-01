# Phase 5: Fermata Syntax — Milestone 1: Foundation

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Parser infrastructure, Fermata AST, error types, module setup

---

## Overview

This milestone establishes the foundation for Fermata syntax compilation:

- **Fermata AST** — Intermediate representation specific to user-facing syntax
- **Parser infrastructure** — Reuse `nom` patterns from Phase 4
- **Error types** — Compilation errors with source locations
- **Module structure** — Organize the `lang/` module

---

## Architecture

### Data Flow

```
Fermata Source Text
        │
        ▼ (parse - reuse sexpr::parser)
Sexpr AST (untyped)
        │
        ▼ (interpret - new)
Fermata AST (typed, ergonomic)
        │
        ▼ (compile)
Music IR (typed, explicit)
```

### Why Two AST Layers?

**Sexpr AST** (Phase 4): Generic S-expressions—symbols, strings, lists.

**Fermata AST** (Phase 5): Music-specific constructs—pitches, durations, notes.

The Fermata AST provides:
- Strong typing for music concepts
- Validation at parse time
- Better error messages
- Clear compilation targets

---

## Task 0: Add `thiserror` Dependency

Add to `crates/fermata/Cargo.toml`:

```toml
[dependencies]
thiserror = "1.0"
```

Note: `thiserror` is the standard choice for library error types in Rust. It provides derive macros for the `Error` trait with minimal boilerplate. (For applications, `anyhow` is often used instead, but for libraries, `thiserror` is preferred.)

---

## Task 1: Module Structure (`src/lang/mod.rs`)

```rust
//! Fermata Language: Ergonomic S-expression syntax for music notation.
//!
//! This module provides:
//! - A typed AST for Fermata syntax
//! - Parsing from S-expression text
//! - Compilation to Music IR
//!
//! # Example
//!
//! ```rust
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
pub mod error;
pub mod pitch;
pub mod duration;
pub mod note;
pub mod chord;
pub mod tuplet;
pub mod direction;
pub mod attributes;
pub mod measure;
pub mod part;
pub mod score;
pub mod defaults;

mod compiler;

pub use ast::*;
pub use error::{CompileError, CompileResult};
pub use compiler::compile;

/// Compile Fermata source to Music IR
pub fn compile_str(source: &str) -> CompileResult<crate::ir::score::ScorePartwise> {
    compiler::compile(source)
}
```

---

## Task 2: Error Types (`src/lang/error.rs`)

```rust
//! Compilation error types for Fermata syntax.

use thiserror::Error;
use crate::sexpr::error::{ParseError, ConvertError};

/// Source location for error reporting
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl SourceSpan {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            line: 0,
            column: 0,
        }
    }

    /// Compute line/column from source text
    pub fn with_source(mut self, source: &str) -> Self {
        let mut line = 1;
        let mut column = 1;
        for (i, ch) in source.char_indices() {
            if i >= self.start {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        self.line = line;
        self.column = column;
        self
    }
}

/// Errors that can occur during Fermata compilation
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("Invalid pitch: {0}")]
    InvalidPitch(String),

    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    #[error("Invalid note: {reason}")]
    InvalidNote { reason: String },

    #[error("Invalid chord: {reason}")]
    InvalidChord { reason: String },

    #[error("Invalid tuplet: {reason}")]
    InvalidTuplet { reason: String },

    #[error("Invalid key signature: {0}")]
    InvalidKey(String),

    #[error("Invalid time signature: {0}")]
    InvalidTime(String),

    #[error("Invalid clef: {0}")]
    InvalidClef(String),

    #[error("Invalid dynamic: {0}")]
    InvalidDynamic(String),

    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("Unknown form: {0}")]
    UnknownForm(String),

    #[error("Expected {expected}, found {found}")]
    TypeMismatch {
        expected: &'static str,
        found: String,
    },

    #[error("IR conversion error: {0}")]
    IrConvert(#[from] ConvertError),

    #[error("{message}")]
    WithSpan {
        message: String,
        span: SourceSpan,
        #[source]
        source: Box<CompileError>,
    },
}

impl CompileError {
    pub fn with_span(self, span: SourceSpan) -> Self {
        CompileError::WithSpan {
            message: self.to_string(),
            span,
            source: Box::new(self),
        }
    }

    pub fn type_mismatch(expected: &'static str, found: impl AsRef<str>) -> Self {
        CompileError::TypeMismatch {
            expected,
            found: found.as_ref().to_string(),
        }
    }
}

pub type CompileResult<T> = Result<T, CompileError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_types() {
        let err = CompileError::InvalidPitch("xyz".to_string());
        assert!(err.to_string().contains("xyz"));

        let err = CompileError::type_mismatch("pitch", "list");
        assert!(err.to_string().contains("pitch"));
        assert!(err.to_string().contains("list"));
    }

    #[test]
    fn test_source_span() {
        let source = "line1\nline2\nline3";
        let span = SourceSpan::new(7, 12).with_source(source);
        assert_eq!(span.line, 2);
        assert_eq!(span.column, 2);
    }

    #[test]
    fn test_source_span_default() {
        let span = SourceSpan::default();
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 0);
        assert_eq!(span.line, 0);
        assert_eq!(span.column, 0);
    }
}
```

---

## Task 3: Fermata AST Types (`src/lang/ast.rs`)

Define the typed AST for Fermata syntax.

```rust
//! Fermata AST — typed representation of user-facing syntax.
//!
//! This AST captures the ergonomic forms before compilation to IR.

use crate::ir::common::StartStop;

/// A complete Fermata score
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FermataScore {
    pub title: Option<String>,
    pub composer: Option<String>,
    pub parts: Vec<FermataPart>,
}

/// A part in the score
#[derive(Debug, Clone, PartialEq)]
pub struct FermataPart {
    pub name: String,
    pub id: Option<String>,
    pub measures: Vec<FermataMeasure>,
}

/// A measure containing music elements
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FermataMeasure {
    pub number: Option<u32>,
    pub content: Vec<MeasureElement>,
}

/// Elements that can appear in a measure
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureElement {
    Note(FermataNote),
    Rest(FermataRest),
    Chord(FermataChord),
    Tuplet(FermataTuplet),
    GraceNote(FermataGraceNote),
    Dynamic(DynamicMark),
    Tempo(TempoMark),
    Direction(FermataDirection),
    Key(KeySpec),
    Time(TimeSpec),
    Clef(ClefSpec),
    Barline(BarlineSpec),
    Slur(SlurMark),
    Tie(TieMark),
    Fermata(FermataMark),
    Backup(u32),
    Forward(u32),
}

/// A single note
#[derive(Debug, Clone, PartialEq)]
pub struct FermataNote {
    pub pitch: FermataPitch,
    pub duration: FermataDuration,
    pub voice: Option<u32>,
    pub staff: Option<u32>,
    pub stem: Option<StemDirection>,
    pub articulations: Vec<Articulation>,
    pub ornaments: Vec<Ornament>,
    pub tie: Option<StartStop>,
    pub slur: Option<StartStop>,
    pub lyric: Option<LyricSpec>,
}

/// A rest
#[derive(Debug, Clone, PartialEq)]
pub struct FermataRest {
    pub duration: FermataDuration,
    pub voice: Option<u32>,
    pub staff: Option<u32>,
    pub measure_rest: bool,
}

/// A chord (multiple simultaneous pitches)
#[derive(Debug, Clone, PartialEq)]
pub struct FermataChord {
    pub pitches: Vec<FermataPitch>,
    pub duration: FermataDuration,
    pub voice: Option<u32>,
    pub staff: Option<u32>,
    pub stem: Option<StemDirection>,
    pub articulations: Vec<Articulation>,
    pub ornaments: Vec<Ornament>,
    pub arpeggiate: Option<ArpeggiateDirection>,
}

/// A grace note
#[derive(Debug, Clone, PartialEq)]
pub struct FermataGraceNote {
    pub pitch: FermataPitch,
    pub slash: bool,
    pub duration: Option<FermataDuration>,
}

/// A tuplet wrapper
#[derive(Debug, Clone, PartialEq)]
pub struct FermataTuplet {
    pub actual: u32,
    pub normal: u32,
    pub notes: Vec<MeasureElement>,
}

/// A pitch (parsed from "c4", "f#5", etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct FermataPitch {
    pub step: PitchStep,
    pub alter: Option<PitchAlter>,
    pub octave: u8,
}

/// Pitch letter name
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchStep {
    C, D, E, F, G, A, B,
}

/// Pitch alteration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitchAlter {
    Sharp,           // #, +1 semitone
    Flat,            // b, -1 semitone
    DoubleSharp,     // x or ##
    DoubleFlat,      // bb
    Natural,         // n (explicit)
    QuarterSharp,    // + (microtone)
    QuarterFlat,     // d (microtone)
    ThreeQuarterSharp,
    ThreeQuarterFlat,
}

impl PitchAlter {
    pub fn to_semitones(&self) -> f64 {
        match self {
            PitchAlter::DoubleFlat => -2.0,
            PitchAlter::ThreeQuarterFlat => -1.5,
            PitchAlter::Flat => -1.0,
            PitchAlter::QuarterFlat => -0.5,
            PitchAlter::Natural => 0.0,
            PitchAlter::QuarterSharp => 0.5,
            PitchAlter::Sharp => 1.0,
            PitchAlter::ThreeQuarterSharp => 1.5,
            PitchAlter::DoubleSharp => 2.0,
        }
    }
}

/// Duration specification
#[derive(Debug, Clone, PartialEq)]
pub struct FermataDuration {
    pub base: DurationBase,
    pub dots: u8,
}

impl Default for FermataDuration {
    fn default() -> Self {
        Self {
            base: DurationBase::Quarter,
            dots: 0,
        }
    }
}

/// Base duration value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DurationBase {
    Maxima,
    Long,
    Breve,
    Whole,
    Half,
    #[default]
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    OneTwentyEighth,
    TwoFiftySixth,
    FiveTwelfth,
    OneThousandTwentyFourth,
}

impl DurationBase {
    /// Duration relative to whole note (1.0 = whole)
    pub fn to_fraction(&self) -> f64 {
        match self {
            DurationBase::Maxima => 8.0,
            DurationBase::Long => 4.0,
            DurationBase::Breve => 2.0,
            DurationBase::Whole => 1.0,
            DurationBase::Half => 0.5,
            DurationBase::Quarter => 0.25,
            DurationBase::Eighth => 0.125,
            DurationBase::Sixteenth => 0.0625,
            DurationBase::ThirtySecond => 0.03125,
            DurationBase::SixtyFourth => 0.015625,
            DurationBase::OneTwentyEighth => 0.0078125,
            DurationBase::TwoFiftySixth => 0.00390625,
            DurationBase::FiveTwelfth => 0.001953125,
            DurationBase::OneThousandTwentyFourth => 0.0009765625,
        }
    }
}

/// Stem direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StemDirection {
    #[default]
    Up,
    Down,
    None,
    Double,
}

/// Articulation marks (excluding fermata, which is a notation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Articulation {
    Staccato,
    Staccatissimo,
    Accent,
    StrongAccent,  // marcato
    Tenuto,
    DetachedLegato,
    BreathMark,
    Caesura,
}

/// Fermata mark (separate from articulations per MusicXML/IR structure)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FermataMark {
    pub shape: FermataShape,
    pub inverted: bool,
}

/// Fermata shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FermataShape {
    #[default]
    Normal,
    Angled,
    Square,
    DoubleAngled,
    DoubleSquare,
    DoubleDot,
    HalfCurve,
    Curlew,
}

/// Ornament marks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ornament {
    Trill,
    Mordent,
    InvertedMordent,
    Turn,
    InvertedTurn,
    DelayedTurn,
    Shake,
    Tremolo(u8),  // number of beams
}

/// Arpeggio direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArpeggiateDirection {
    Up,
    Down,
    #[default]
    None,
}

/// Dynamic marking
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicMark {
    // Standard dynamics
    PPPPPP, PPPPP, PPPP, PPP, PP, P,
    MP, MF,
    F, FF, FFF, FFFF, FFFFF, FFFFFF,
    // Combined dynamics
    FP, SF, SFP, SFPP, SFZ, SFFZ, SFZP, FZ, PF, RF, RFZ,
    // Niente
    N,
    // Wedges
    Crescendo(StartStop),
    Diminuendo(StartStop),
}

/// Tempo marking
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TempoMark {
    pub text: Option<String>,
    pub beat_unit: Option<DurationBase>,
    pub beat_unit_dots: u8,
    pub per_minute: Option<u32>,
}

/// General direction
#[derive(Debug, Clone, PartialEq)]
pub enum FermataDirection {
    Words(String),
    Rehearsal(String),
    Segno,
    Coda,
    PedalStart,
    PedalStop,
}

/// Key signature specification
#[derive(Debug, Clone, PartialEq)]
pub struct KeySpec {
    pub root: PitchStep,
    pub root_alter: Option<PitchAlter>,  // For F# major, Bb minor, etc.
    pub mode: Mode,
}

/// Mode for key signature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Major,
    Minor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Ionian,
    Locrian,
}

/// Time signature specification
#[derive(Debug, Clone, PartialEq)]
pub enum TimeSpec {
    Simple { beats: u8, beat_type: u8 },
    Compound { signatures: Vec<(u8, u8)> },  // e.g., [(2,4), (3,8)] for 2/4 + 3/8
    Common,
    Cut,
    SenzaMisura,
}

impl Default for TimeSpec {
    fn default() -> Self {
        TimeSpec::Simple { beats: 4, beat_type: 4 }
    }
}

/// Clef specification
#[derive(Debug, Clone, PartialEq)]
pub enum ClefSpec {
    Treble,
    Bass,
    Alto,
    Tenor,
    Treble8vb,
    Treble8va,
    Bass8vb,
    Bass8va,
    Percussion,
    Tab,
    // Explicit sign/line
    Custom { sign: char, line: u8, octave_change: Option<i8> },
}

impl Default for ClefSpec {
    fn default() -> Self {
        ClefSpec::Treble
    }
}

/// Action for endings (start, stop, or discontinue for jumps)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndingAction {
    Start,
    Stop,
    Discontinue,  // When jumping back (e.g., D.S. skips ending 1)
}

/// Barline specification
#[derive(Debug, Clone, PartialEq)]
pub enum BarlineSpec {
    Regular,
    Double,
    Final,
    RepeatForward,
    RepeatBackward,
    RepeatBoth,
    Ending { number: u8, action: EndingAction },
}

impl Default for BarlineSpec {
    fn default() -> Self {
        BarlineSpec::Regular
    }
}

/// Slur mark
#[derive(Debug, Clone, PartialEq)]
pub struct SlurMark {
    pub action: StartStop,
    pub number: u8,  // Defaults to 1 in parsing
}

impl Default for SlurMark {
    fn default() -> Self {
        Self {
            action: StartStop::Start,
            number: 1,
        }
    }
}

/// Tie mark
#[derive(Debug, Clone, PartialEq)]
pub struct TieMark {
    pub action: StartStop,
}

/// Lyric specification
#[derive(Debug, Clone, PartialEq)]
pub struct LyricSpec {
    pub text: String,
    pub syllabic: Syllabic,
    pub verse: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Syllabic {
    #[default]
    Single,
    Begin,
    Middle,
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_base_fractions() {
        assert_eq!(DurationBase::Whole.to_fraction(), 1.0);
        assert_eq!(DurationBase::Half.to_fraction(), 0.5);
        assert_eq!(DurationBase::Quarter.to_fraction(), 0.25);
        assert_eq!(DurationBase::Eighth.to_fraction(), 0.125);
        assert_eq!(DurationBase::OneThousandTwentyFourth.to_fraction(), 0.0009765625);
    }

    #[test]
    fn test_pitch_alter_semitones() {
        assert_eq!(PitchAlter::Sharp.to_semitones(), 1.0);
        assert_eq!(PitchAlter::Flat.to_semitones(), -1.0);
        assert_eq!(PitchAlter::DoubleSharp.to_semitones(), 2.0);
        assert_eq!(PitchAlter::QuarterSharp.to_semitones(), 0.5);
    }

    #[test]
    fn test_defaults() {
        assert_eq!(DurationBase::default(), DurationBase::Quarter);
        assert_eq!(StemDirection::default(), StemDirection::Up);
        assert_eq!(Mode::default(), Mode::Major);
        assert_eq!(ClefSpec::default(), ClefSpec::Treble);
        assert_eq!(TimeSpec::default(), TimeSpec::Simple { beats: 4, beat_type: 4 });
    }

    #[test]
    fn test_slur_mark_default() {
        let slur = SlurMark::default();
        assert_eq!(slur.number, 1);
        assert_eq!(slur.action, StartStop::Start);
    }
}
```

---

## Task 4: Defaults Module (`src/lang/defaults.rs`)

```rust
//! Default value inference for Fermata compilation.
//!
//! When optional fields are not specified, these defaults are used.

use crate::ir::duration::NoteTypeValue;
use super::ast::{PitchStep, PitchAlter};

/// Default divisions per quarter note
pub const DEFAULT_DIVISIONS: u32 = 960;

/// Default voice number
pub const DEFAULT_VOICE: &str = "1";

/// Default staff number
pub const DEFAULT_STAFF: u32 = 1;

/// Default MusicXML version
pub const DEFAULT_MUSICXML_VERSION: &str = "4.0";

/// Calculate divisions value for a duration
pub fn duration_to_divisions(note_type: NoteTypeValue, dots: u8) -> u32 {
    let base = match note_type {
        NoteTypeValue::Maxima => DEFAULT_DIVISIONS * 32,
        NoteTypeValue::Long => DEFAULT_DIVISIONS * 16,
        NoteTypeValue::Breve => DEFAULT_DIVISIONS * 8,
        NoteTypeValue::Whole => DEFAULT_DIVISIONS * 4,
        NoteTypeValue::Half => DEFAULT_DIVISIONS * 2,
        NoteTypeValue::Quarter => DEFAULT_DIVISIONS,
        NoteTypeValue::Eighth => DEFAULT_DIVISIONS / 2,
        NoteTypeValue::N16th => DEFAULT_DIVISIONS / 4,
        NoteTypeValue::N32nd => DEFAULT_DIVISIONS / 8,
        NoteTypeValue::N64th => DEFAULT_DIVISIONS / 16,
        NoteTypeValue::N128th => DEFAULT_DIVISIONS / 32,
        NoteTypeValue::N256th => DEFAULT_DIVISIONS / 64,
        NoteTypeValue::N512th => DEFAULT_DIVISIONS / 128,
        NoteTypeValue::N1024th => DEFAULT_DIVISIONS / 256,
    };

    // Apply dots: each dot adds half of the previous value
    let mut total = base;
    let mut dot_value = base / 2;
    for _ in 0..dots {
        total += dot_value;
        dot_value /= 2;
    }
    total
}

/// Compute the fifths value for a key signature
///
/// Handles both natural keys (C major, G major) and keys with accidentals
/// (F# major, Bb minor, etc.)
pub fn key_to_fifths(root: PitchStep, root_alter: Option<PitchAlter>, mode: &str) -> i8 {
    // Base fifths for natural major keys
    let natural_major_fifths: i8 = match root {
        PitchStep::C => 0,
        PitchStep::G => 1,
        PitchStep::D => 2,
        PitchStep::A => 3,
        PitchStep::E => 4,
        PitchStep::B => 5,
        PitchStep::F => -1,
    };

    // Adjust for accidentals on the root
    let alter_offset: i8 = match root_alter {
        Some(PitchAlter::Sharp) => 7,        // F# major = 6 = F major (-1) + 7
        Some(PitchAlter::Flat) => -7,        // Gb major = -6 = G major (1) - 7
        Some(PitchAlter::DoubleSharp) => 14,
        Some(PitchAlter::DoubleFlat) => -14,
        _ => 0,
    };

    let major_fifths = natural_major_fifths + alter_offset;

    // Adjust for mode (relative to major)
    let mode_offset: i8 = match mode.to_lowercase().as_str() {
        "major" | "ionian" => 0,
        "minor" | "aeolian" => -3,  // A minor = C major, so -3 from relative major
        "dorian" => -2,
        "phrygian" => -4,
        "lydian" => 1,
        "mixolydian" => -1,
        "locrian" => -5,
        _ => 0,
    };

    major_fifths + mode_offset
}

/// Generate a part ID from index
pub fn generate_part_id(index: usize) -> String {
    format!("P{}", index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_to_divisions() {
        assert_eq!(duration_to_divisions(NoteTypeValue::Quarter, 0), DEFAULT_DIVISIONS);
        assert_eq!(duration_to_divisions(NoteTypeValue::Half, 0), DEFAULT_DIVISIONS * 2);

        // Dotted quarter = 1.5 * quarter
        let dotted_quarter = duration_to_divisions(NoteTypeValue::Quarter, 1);
        assert_eq!(dotted_quarter, DEFAULT_DIVISIONS + DEFAULT_DIVISIONS / 2);
    }

    #[test]
    fn test_key_to_fifths_natural_major() {
        assert_eq!(key_to_fifths(PitchStep::C, None, "major"), 0);
        assert_eq!(key_to_fifths(PitchStep::G, None, "major"), 1);
        assert_eq!(key_to_fifths(PitchStep::F, None, "major"), -1);
        assert_eq!(key_to_fifths(PitchStep::D, None, "major"), 2);
    }

    #[test]
    fn test_key_to_fifths_with_accidentals() {
        // F# major = 6 fifths
        assert_eq!(key_to_fifths(PitchStep::F, Some(PitchAlter::Sharp), "major"), 6);
        // Bb major = -2 fifths
        assert_eq!(key_to_fifths(PitchStep::B, Some(PitchAlter::Flat), "major"), -2);
        // Gb major = -6 fifths
        assert_eq!(key_to_fifths(PitchStep::G, Some(PitchAlter::Flat), "major"), -6);
    }

    #[test]
    fn test_key_to_fifths_minor() {
        assert_eq!(key_to_fifths(PitchStep::A, None, "minor"), -3);
        assert_eq!(key_to_fifths(PitchStep::E, None, "minor"), -2);
        assert_eq!(key_to_fifths(PitchStep::D, None, "minor"), -1);
    }

    #[test]
    fn test_key_to_fifths_modes() {
        // D dorian = same key sig as C major = 0 fifths
        assert_eq!(key_to_fifths(PitchStep::D, None, "dorian"), 0);
        // E phrygian = same key sig as C major = 0 fifths
        assert_eq!(key_to_fifths(PitchStep::E, None, "phrygian"), 0);
        // F lydian = same key sig as C major = 0 fifths
        assert_eq!(key_to_fifths(PitchStep::F, None, "lydian"), 0);
    }

    #[test]
    fn test_generate_part_id() {
        assert_eq!(generate_part_id(0), "P1");
        assert_eq!(generate_part_id(1), "P2");
        assert_eq!(generate_part_id(9), "P10");
    }
}
```

---

## Task 5: Basic Compiler Stub (`src/lang/compiler.rs`)

```rust
//! Fermata → IR compiler.
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
        _ => Err(CompileError::type_mismatch("pitch symbol", format!("{:?}", sexpr))),
    }
}
```

---

## Task 6: Update `src/lib.rs`

Add the new module to the library:

```rust
// In src/lib.rs, add:
pub mod lang;
```

---

## Acceptance Criteria

1. ✅ `src/lang/` module exists with proper structure
2. ✅ `thiserror` added to `Cargo.toml`
3. ✅ `CompileError` enum covers all error types with spans
4. ✅ `SourceSpan` computes line/column from source
5. ✅ Fermata AST types defined for all musical constructs
6. ✅ `defaults.rs` provides duration, key, and ID generation helpers
7. ✅ Compiler stub compiles (with `todo!()` bodies)
8. ✅ All tests pass (inline `#[cfg(test)]` modules)

---

## Implementation Notes

1. **Reuse Phase 4 parser** — The S-expr parser already handles tokenization; we just need to interpret the AST differently.

2. **AST completeness** — The AST types should cover everything in the IR format reference, but with ergonomic forms.

3. **Error context** — Use `CompileError::with_span()` to preserve source locations through the compilation pipeline.

4. **Incremental compilation** — Provide `compile_pitch_str`, `compile_note_str`, etc. for testing individual elements.

5. **Default derives** — Use `#[derive(Default)]` where sensible to simplify construction and testing.

---

*Next: Milestone 2 — Core Musical Elements (Pitches, Durations, Notes)*
