# Phase 5: Fermata Syntax — Milestone 1: Foundation

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Parser infrastructure, Fermata AST, error types, module setup
> **Estimated Implementation Time:** 2-3 hours

---

## Overview

This milestone establishes the foundation for Fermata syntax compilation:

- **Fermata AST** — Intermediate representation specific to user-facing syntax
- **Parser infrastructure** — Reuse `nom` patterns from Phase 4
- **Error types** — Compilation errors with source locations
- **Module structure** — Organize the `fermata/` module

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

## Task 1: Module Structure (`src/fermata/mod.rs`)

```rust
//! Fermata: Ergonomic S-expression syntax for music notation.
//!
//! This module provides:
//! - A typed AST for Fermata syntax
//! - Parsing from S-expression text
//! - Compilation to Music IR
//!
//! # Example
//!
//! ```rust
//! use fermata::fermata::compile;
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

## Task 2: Error Types (`src/fermata/error.rs`)

```rust
use thiserror::Error;
use crate::sexpr::error::{ParseError, ConvertError};

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq)]
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
```

---

## Task 3: Fermata AST Types (`src/fermata/ast.rs`)

Define the typed AST for Fermata syntax.

```rust
//! Fermata AST — typed representation of user-facing syntax.
//!
//! This AST captures the ergonomic forms before compilation to IR.

use crate::ir::common::StartStop;

/// A complete Fermata score
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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

/// Base duration value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationBase {
    Maxima,
    Long,
    Breve,
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    OneTwentyEighth,
    TwoFiftySixth,
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
        }
    }
}

/// Stem direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StemDirection {
    Up,
    Down,
    None,
    Double,
}

/// Articulation marks
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
    Fermata,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArpeggiateDirection {
    Up,
    Down,
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
#[derive(Debug, Clone, PartialEq)]
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
    pub mode: Mode,
}

/// Mode for key signature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
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
    Common,
    Cut,
    SenzaMisura,
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

/// Barline specification
#[derive(Debug, Clone, PartialEq)]
pub enum BarlineSpec {
    Regular,
    Double,
    Final,
    RepeatForward,
    RepeatBackward,
    RepeatBoth,
    Ending { number: u8, action: StartStop },
}

/// Slur mark
#[derive(Debug, Clone, PartialEq)]
pub struct SlurMark {
    pub action: StartStop,
    pub number: Option<u8>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syllabic {
    Single,
    Begin,
    Middle,
    End,
}
```

---

## Task 4: Defaults Module (`src/fermata/defaults.rs`)

```rust
//! Default value inference for Fermata compilation.
//!
//! When optional fields are not specified, these defaults are used.

use crate::ir::duration::NoteTypeValue;

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
pub fn key_to_fifths(root: char, mode: &str) -> i8 {
    use std::collections::HashMap;

    // Major key fifths
    let major_fifths: HashMap<char, i8> = [
        ('C', 0), ('G', 1), ('D', 2), ('A', 3), ('E', 4), ('B', 5), ('F', -1),
    ].into_iter().collect();

    // Minor keys are relative to their major
    // A minor = C major, E minor = G major, etc.
    let minor_offset: i8 = 0; // Same fifths, different mode label

    let base = match mode.to_lowercase().as_str() {
        "major" | "ionian" => *major_fifths.get(&root.to_ascii_uppercase()).unwrap_or(&0),
        "minor" | "aeolian" => {
            // Minor is 3 semitones below its relative major
            // A minor (0 fifths), E minor (1 fifth), etc.
            let minor_fifths: HashMap<char, i8> = [
                ('A', 0), ('E', 1), ('B', 2), ('D', -1), ('G', -2), ('C', -3), ('F', -4),
            ].into_iter().collect();
            *minor_fifths.get(&root.to_ascii_uppercase()).unwrap_or(&0)
        }
        "dorian" => {
            // D dorian = 0, A dorian = 1, etc.
            let dorian_fifths: HashMap<char, i8> = [
                ('D', 0), ('A', 1), ('E', 2), ('B', 3), ('G', -1), ('C', -2), ('F', -3),
            ].into_iter().collect();
            *dorian_fifths.get(&root.to_ascii_uppercase()).unwrap_or(&0)
        }
        // Add other modes as needed
        _ => 0,
    };

    base
}

/// Generate a part ID from a name
pub fn generate_part_id(name: &str, index: usize) -> String {
    format!("P{}", index + 1)
}
```

---

## Task 5: Basic Compiler Stub (`src/fermata/compiler.rs`)

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
/// ```rust
/// use fermata::fermata::compile;
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
fn interpret_sexpr(sexpr: &Sexpr) -> CompileResult<super::ast::FermataScore> {
    // TODO: Implement in Milestone 5
    todo!("interpret_sexpr")
}

/// Compile Fermata AST to Music IR
fn compile_to_ir(ast: &super::ast::FermataScore) -> CompileResult<ScorePartwise> {
    // TODO: Implement in Milestone 5
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
pub mod fermata;
```

---

## Task 7: Integration Tests

Create `tests/fermata_foundation.rs`:

```rust
//! Foundation tests for Fermata syntax module.

use fermata::fermata::error::CompileError;

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
    use fermata::fermata::error::SourceSpan;

    let source = "line1\nline2\nline3";
    let span = SourceSpan::new(7, 12).with_source(source);
    assert_eq!(span.line, 2);
    assert_eq!(span.column, 2);
}

#[test]
fn test_duration_base_fractions() {
    use fermata::fermata::ast::DurationBase;

    assert_eq!(DurationBase::Whole.to_fraction(), 1.0);
    assert_eq!(DurationBase::Half.to_fraction(), 0.5);
    assert_eq!(DurationBase::Quarter.to_fraction(), 0.25);
    assert_eq!(DurationBase::Eighth.to_fraction(), 0.125);
}

#[test]
fn test_pitch_alter_semitones() {
    use fermata::fermata::ast::PitchAlter;

    assert_eq!(PitchAlter::Sharp.to_semitones(), 1.0);
    assert_eq!(PitchAlter::Flat.to_semitones(), -1.0);
    assert_eq!(PitchAlter::DoubleSharp.to_semitones(), 2.0);
    assert_eq!(PitchAlter::QuarterSharp.to_semitones(), 0.5);
}

#[test]
fn test_defaults_duration_to_divisions() {
    use fermata::fermata::defaults::{duration_to_divisions, DEFAULT_DIVISIONS};
    use fermata::ir::duration::NoteTypeValue;

    assert_eq!(duration_to_divisions(NoteTypeValue::Quarter, 0), DEFAULT_DIVISIONS);
    assert_eq!(duration_to_divisions(NoteTypeValue::Half, 0), DEFAULT_DIVISIONS * 2);

    // Dotted quarter = 1.5 * quarter
    let dotted_quarter = duration_to_divisions(NoteTypeValue::Quarter, 1);
    assert_eq!(dotted_quarter, DEFAULT_DIVISIONS + DEFAULT_DIVISIONS / 2);
}

#[test]
fn test_defaults_key_to_fifths() {
    use fermata::fermata::defaults::key_to_fifths;

    assert_eq!(key_to_fifths('C', "major"), 0);
    assert_eq!(key_to_fifths('G', "major"), 1);
    assert_eq!(key_to_fifths('F', "major"), -1);
    assert_eq!(key_to_fifths('D', "major"), 2);
    assert_eq!(key_to_fifths('A', "minor"), 0);
}
```

---

## Acceptance Criteria

1. ✅ `src/fermata/` module exists with proper structure
2. ✅ `CompileError` enum covers all error types with spans
3. ✅ `SourceSpan` computes line/column from source
4. ✅ Fermata AST types defined for all musical constructs
5. ✅ `defaults.rs` provides duration, key, and ID generation helpers
6. ✅ Compiler stub compiles (with `todo!()` bodies)
7. ✅ All tests pass

---

## Implementation Notes

1. **Reuse Phase 4 parser** — The S-expr parser already handles tokenization; we just need to interpret the AST differently.

2. **AST completeness** — The AST types should cover everything in the IR format reference, but with ergonomic forms.

3. **Error context** — Use `CompileError::with_span()` to preserve source locations through the compilation pipeline.

4. **Incremental compilation** — Provide `compile_pitch_str`, `compile_note_str`, etc. for testing individual elements.

---

*Next: Milestone 2 — Core Musical Elements (Pitches, Durations, Notes)*
