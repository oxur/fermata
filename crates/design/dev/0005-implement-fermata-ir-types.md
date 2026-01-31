# Implement Fermata IR Types

> **Instructions:** You are implementing the foundational IR (Intermediate Representation) types for Fermata, a Lisp DSL that compiles to MusicXML. This is Phase 1 of the implementation: defining the Rust type system that represents the full MusicXML data model.

---

## Project Context

**Fermata** is an S-expression DSL for music notation that compiles to MusicXML. The architecture has two layers:

1. **Fermata Syntax** (future) — Ergonomic, terse, user-facing
2. **Music IR** (this task) — Lossless 1:1 mapping with MusicXML

You are implementing the Music IR layer. These types must:

- Capture everything MusicXML can express (lossless round-tripping)
- Map cleanly to/from MusicXML elements and attributes
- Be idiomatic Rust (derive common traits, use Options for optional fields)

## Existing Crate Structure

```
./crates/fermata/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── main.rs
```

You will be adding an `ir/` module under `src/`.

## Target Structure

```
./crates/fermata/src/
├── lib.rs           # Add: pub mod ir;
├── main.rs
└── ir/
    ├── mod.rs       # Re-exports all submodules
    ├── common.rs    # Shared types, enums, type aliases
    ├── pitch.rs     # Pitch, Step, Alter, Octave
    ├── duration.rs  # Duration, NoteType, Dot, TimeModification
    ├── note.rs      # Note, Rest, Grace, FullNote, Accidental
    ├── measure.rs   # Measure, MusicDataElement
    ├── attributes.rs # Attributes, Time, Key, Clef, Barline
    ├── part.rs      # Part, PartList, ScorePart, PartGroup
    ├── score.rs     # ScorePartwise, Work, Identification
    ├── direction.rs # Direction, DirectionType, Dynamics, Wedge, etc.
    ├── notation.rs  # Notations, Articulations, Ornaments, Technical
    ├── beam.rs      # Beam, Stem, Notehead
    ├── voice.rs     # Backup, Forward
    └── lyric.rs     # Lyric, Syllabic, Text, Elision, Extend
```

## Implementation Guidelines

### Rust Idioms

1. **Derive common traits** on all types:

   ```rust
   #[derive(Debug, Clone, PartialEq)]
   ```

2. **Use `Option<T>`** for optional MusicXML attributes/elements

3. **Use `Vec<T>`** for repeatable elements (0 or more)

4. **Use enums** for MusicXML's enumerated types

5. **Use `Box<T>`** for recursive types if needed

6. **Naming:**
   - Rust types: PascalCase matching MusicXML element names
   - Enum variants: PascalCase
   - Fields: snake_case
   - Use `r#type` for the reserved word `type`

7. **Documentation:** Add doc comments for non-obvious types, referencing MusicXML semantics

### Type Aliases (in common.rs)

```rust
/// Tenths of staff space (MusicXML's primary unit for positioning)
pub type Tenths = f64;

/// Duration in divisions (relative to <divisions> in attributes)
pub type Divisions = i64;

/// Positive duration value
pub type PositiveDivisions = u64;

/// Semitones for pitch alteration (-2 to +2 typical, microtones possible)
pub type Semitones = f64;

/// Octave number (0-9, where 4 is the octave starting at middle C)
pub type Octave = u8;

/// Staff number (1-based)
pub type StaffNumber = u16;

/// Beam level (1-8, for 8th through 1024th notes)
pub type BeamLevel = u8;

/// Number level for spanning elements (1-16)
pub type NumberLevel = u8;

/// Voice identifier (string, not integer - allows "1a", custom IDs)
pub type Voice = String;

/// CSS-style color string
pub type Color = String;

/// Percentage (0.0 to 100.0)
pub type Percent = f64;
```

---

## Type Definitions

Below are the complete type definitions from the assembly report. Implement these exactly, organized into the modules specified.

### common.rs

```rust
//! Common types, enums, and type aliases shared across the IR.

// === Type Aliases ===

pub type Tenths = f64;
pub type Divisions = i64;
pub type PositiveDivisions = u64;
pub type Semitones = f64;
pub type Octave = u8;
pub type StaffNumber = u16;
pub type BeamLevel = u8;
pub type NumberLevel = u8;
pub type Voice = String;
pub type Color = String;
pub type Percent = f64;

// === Common Enums ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YesNo {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStop {
    Start,
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopContinue {
    Start,
    Stop,
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopSingle {
    Start,
    Stop,
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopDiscontinue {
    Start,
    Stop,
    Discontinue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AboveBelow {
    Above,
    Below,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpDown {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverUnder {
    Over,
    Under,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftCenterRight {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopMiddleBottom {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackwardForward {
    Backward,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightLeftMiddle {
    Right,
    Left,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UprightInverted {
    Upright,
    Inverted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSize {
    Full,
    Cue,
    GraceCue,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    Solid,
    Dashed,
    Dotted,
    Wavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Bold,
}

// === Attribute Group Structs ===

/// Position attributes for placement relative to staff
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,
}

/// Font size can be CSS size or numeric points
#[derive(Debug, Clone, PartialEq)]
pub enum FontSize {
    Css(CssFontSize),
    Points(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
}

/// Font attributes
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Font {
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
}

/// Combined print-style attributes
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrintStyle {
    pub position: Position,
    pub font: Font,
    pub color: Option<Color>,
}

/// Editorial information (footnote and level)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Editorial {
    pub footnote: Option<FormattedText>,
    pub level: Option<Level>,
}

/// Formatted text with optional formatting
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedText {
    pub value: String,
    pub print_style: PrintStyle,
    pub lang: Option<String>,
}

/// Level for editorial annotations
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    pub value: String,
    pub reference: Option<YesNo>,
}

/// Empty placement - used for simple articulations
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyPlacement {
    pub placement: Option<AboveBelow>,
    pub position: Position,
}
```

### pitch.rs

```rust
//! Pitch representation types.

use super::common::{Octave, Semitones};

/// A musical pitch with step, optional alteration, and octave.
#[derive(Debug, Clone, PartialEq)]
pub struct Pitch {
    pub step: Step,
    pub alter: Option<Semitones>,
    pub octave: Octave,
}

/// The seven natural pitch steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

/// Unpitched note (percussion) with optional display position.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Unpitched {
    pub display_step: Option<Step>,
    pub display_octave: Option<Octave>,
}
```

### duration.rs

```rust
//! Duration and rhythm types.

use super::common::{AboveBelow, Position, SymbolSize};

/// Note type (notated duration symbol).
#[derive(Debug, Clone, PartialEq)]
pub struct NoteType {
    pub value: NoteTypeValue,
    pub size: Option<SymbolSize>,
}

/// Enumeration of note type values (duration symbols).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteTypeValue {
    N1024th,
    N512th,
    N256th,
    N128th,
    N64th,
    N32nd,
    N16th,
    Eighth,
    Quarter,
    Half,
    Whole,
    Breve,
    Long,
    Maxima,
}

/// Augmentation dot.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dot {
    pub placement: Option<AboveBelow>,
    pub position: Position,
}

/// Time modification for tuplets.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeModification {
    /// Actual number of notes in the tuplet
    pub actual_notes: u32,
    /// Normal number of notes the tuplet replaces
    pub normal_notes: u32,
    /// Note type of the normal notes (if different from actual)
    pub normal_type: Option<NoteTypeValue>,
    /// Number of dots on normal notes
    pub normal_dots: u32,
}
```

### note.rs

```rust
//! Note, rest, chord, and grace note types.

use super::common::*;
use super::pitch::{Pitch, Unpitched};
use super::duration::{Dot, NoteType, TimeModification};
use super::beam::Beam;
use super::notation::Notations;
use super::lyric::Lyric;

/// A note element - the fundamental music content type.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    // Position/playback attributes
    pub position: Position,
    pub dynamics: Option<f64>,
    pub end_dynamics: Option<f64>,
    pub attack: Option<Divisions>,
    pub release: Option<Divisions>,
    pub pizzicato: Option<bool>,
    pub print_object: Option<YesNo>,

    // Content variant (regular, grace, or cue)
    pub content: NoteContent,

    // Common children
    pub instrument: Vec<Instrument>,
    pub voice: Option<Voice>,
    pub r#type: Option<NoteType>,
    pub dots: Vec<Dot>,
    pub accidental: Option<Accidental>,
    pub time_modification: Option<TimeModification>,
    pub stem: Option<Stem>,
    pub notehead: Option<Notehead>,
    pub staff: Option<StaffNumber>,
    pub beams: Vec<Beam>,
    pub notations: Vec<Notations>,
    pub lyrics: Vec<Lyric>,
}

/// The three content variants for a note.
#[derive(Debug, Clone, PartialEq)]
pub enum NoteContent {
    /// Regular note with duration
    Regular {
        full_note: FullNote,
        duration: PositiveDivisions,
        ties: Vec<Tie>,
    },
    /// Grace note (no duration, steals time)
    Grace {
        grace: Grace,
        full_note: FullNote,
        ties: Vec<Tie>,
    },
    /// Cue note (for cue-sized notes)
    Cue {
        full_note: FullNote,
        duration: PositiveDivisions,
    },
}

/// Full note content: chord flag + pitch/rest/unpitched
#[derive(Debug, Clone, PartialEq)]
pub struct FullNote {
    /// If true, this note is part of a chord with the previous note
    pub chord: bool,
    /// The pitch, rest, or unpitched content
    pub content: PitchRestUnpitched,
}

/// Pitch, rest, or unpitched (mutually exclusive)
#[derive(Debug, Clone, PartialEq)]
pub enum PitchRestUnpitched {
    Pitch(Pitch),
    Rest(Rest),
    Unpitched(Unpitched),
}

/// A rest.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rest {
    /// If yes, this is a whole-measure rest
    pub measure: Option<YesNo>,
    /// Display position for the rest symbol
    pub display_step: Option<super::pitch::Step>,
    pub display_octave: Option<Octave>,
}

/// Grace note attributes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Grace {
    pub steal_time_previous: Option<Percent>,
    pub steal_time_following: Option<Percent>,
    pub make_time: Option<Divisions>,
    pub slash: Option<YesNo>,
}

/// Tie (playback, not visual).
#[derive(Debug, Clone, PartialEq)]
pub struct Tie {
    pub r#type: StartStop,
    pub time_only: Option<String>,
}

/// Accidental display.
#[derive(Debug, Clone, PartialEq)]
pub struct Accidental {
    pub value: AccidentalValue,
    pub cautionary: Option<YesNo>,
    pub editorial: Option<YesNo>,
    pub parentheses: Option<YesNo>,
    pub bracket: Option<YesNo>,
    pub size: Option<SymbolSize>,
}

/// Accidental values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccidentalValue {
    Sharp,
    Natural,
    Flat,
    DoubleSharp,
    SharpSharp,
    FlatFlat,
    DoubleFlat,
    NaturalSharp,
    NaturalFlat,
    QuarterFlat,
    QuarterSharp,
    ThreeQuartersFlat,
    ThreeQuartersSharp,
    SharpDown,
    SharpUp,
    NaturalDown,
    NaturalUp,
    FlatDown,
    FlatUp,
    TripleSharp,
    TripleFlat,
    SlashQuarterSharp,
    SlashSharp,
    SlashFlat,
    DoubleSlashFlat,
    Sharp1,
    Sharp2,
    Sharp3,
    Sharp5,
    Flat1,
    Flat2,
    Flat3,
    Flat4,
    Sori,
    Koron,
    Other,
}

/// Instrument reference (for playback)
#[derive(Debug, Clone, PartialEq)]
pub struct Instrument {
    pub id: String,
}

// Forward declarations - these are defined in beam.rs but used here
pub use super::beam::{Stem, Notehead};
```

### beam.rs

```rust
//! Beam, stem, and notehead types.

use super::common::*;

/// Beam element for note grouping.
#[derive(Debug, Clone, PartialEq)]
pub struct Beam {
    pub value: BeamValue,
    pub number: BeamLevel,
    pub fan: Option<Fan>,
    pub color: Option<Color>,
}

/// Beam value (connection type).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamValue {
    Begin,
    Continue,
    End,
    ForwardHook,
    BackwardHook,
}

/// Beam fan (for feathered beams).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fan {
    Accel,
    Rit,
    None,
}

/// Stem direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Stem {
    pub value: StemValue,
    pub default_y: Option<Tenths>,
    pub color: Option<Color>,
}

/// Stem direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StemValue {
    Down,
    Up,
    Double,
    None,
}

/// Notehead shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Notehead {
    pub value: NoteheadValue,
    pub filled: Option<YesNo>,
    pub parentheses: Option<YesNo>,
    pub font: Font,
    pub color: Option<Color>,
}

/// Notehead shape values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteheadValue {
    Slash,
    Triangle,
    Diamond,
    Square,
    Cross,
    X,
    CircleX,
    InvertedTriangle,
    ArrowDown,
    ArrowUp,
    Circled,
    Slashed,
    BackSlashed,
    Normal,
    Cluster,
    CircleDot,
    LeftTriangle,
    Rectangle,
    None,
    Do,
    Re,
    Mi,
    Fa,
    FaUp,
    So,
    La,
    Ti,
    Other,
}
```

### attributes.rs

```rust
//! Measure attributes: time, key, clef, barline.

use super::common::*;
use super::notation::Fermata;

/// Attributes element containing key, time, clef, etc.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Attributes {
    pub editorial: Editorial,
    pub divisions: Option<PositiveDivisions>,
    pub keys: Vec<Key>,
    pub times: Vec<Time>,
    pub staves: Option<u32>,
    pub part_symbol: Option<PartSymbol>,
    pub instruments: Option<u32>,
    pub clefs: Vec<Clef>,
    pub staff_details: Vec<StaffDetails>,
    pub transpose: Vec<Transpose>,
    pub measure_styles: Vec<MeasureStyle>,
}

/// Key signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    pub content: KeyContent,
    pub number: Option<StaffNumber>,
    pub print_object: Option<YesNo>,
}

/// Key content - traditional (fifths) or non-traditional (explicit steps)
#[derive(Debug, Clone, PartialEq)]
pub enum KeyContent {
    Traditional(TraditionalKey),
    NonTraditional(Vec<KeyStep>),
}

/// Traditional key signature.
#[derive(Debug, Clone, PartialEq)]
pub struct TraditionalKey {
    pub cancel: Option<Cancel>,
    pub fifths: i8,
    pub mode: Option<Mode>,
}

/// Key step for non-traditional keys.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyStep {
    pub step: super::pitch::Step,
    pub alter: Semitones,
    pub accidental: Option<AccidentalValue>,
}

/// Cancel previous key signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Cancel {
    pub fifths: i8,
    pub location: Option<CancelLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelLocation {
    Left,
    Right,
    BeforeBarline,
}

/// Mode for key signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Major,
    Minor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    Ionian,
    None,
}

/// Time signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub content: TimeContent,
    pub number: Option<StaffNumber>,
    pub symbol: Option<TimeSymbol>,
    pub print_object: Option<YesNo>,
}

/// Time signature content.
#[derive(Debug, Clone, PartialEq)]
pub enum TimeContent {
    Measured { signatures: Vec<TimeSignature> },
    SenzaMisura(String),
}

/// A single time signature (beats / beat-type).
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSignature {
    pub beats: String,
    pub beat_type: String,
}

/// Time signature symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSymbol {
    Common,
    Cut,
    SingleNumber,
    Note,
    DottedNote,
    Normal,
}

/// Clef.
#[derive(Debug, Clone, PartialEq)]
pub struct Clef {
    pub sign: ClefSign,
    pub line: Option<u8>,
    pub octave_change: Option<i8>,
    pub number: Option<StaffNumber>,
    pub size: Option<SymbolSize>,
    pub print_object: Option<YesNo>,
}

/// Clef sign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClefSign {
    G,
    F,
    C,
    Percussion,
    Tab,
    Jianpu,
    None,
}

/// Part symbol for grouping.
#[derive(Debug, Clone, PartialEq)]
pub struct PartSymbol {
    pub value: GroupSymbolValue,
    pub top_staff: Option<StaffNumber>,
    pub bottom_staff: Option<StaffNumber>,
    pub position: Position,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupSymbolValue {
    None,
    Brace,
    Line,
    Bracket,
    Square,
}

/// Staff details.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StaffDetails {
    pub number: Option<StaffNumber>,
    pub staff_type: Option<StaffType>,
    pub staff_lines: Option<u8>,
    pub staff_tuning: Vec<StaffTuning>,
    pub capo: Option<u8>,
    pub staff_size: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffType {
    Ossia,
    Editorial,
    Cue,
    Regular,
    Alternate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaffTuning {
    pub line: u8,
    pub tuning_step: super::pitch::Step,
    pub tuning_alter: Option<Semitones>,
    pub tuning_octave: Octave,
}

/// Transposition.
#[derive(Debug, Clone, PartialEq)]
pub struct Transpose {
    pub number: Option<StaffNumber>,
    pub diatonic: Option<i32>,
    pub chromatic: i32,
    pub octave_change: Option<i32>,
    pub double: Option<YesNo>,
}

/// Measure style (multimeasure rests, slashes, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureStyle {
    pub number: Option<StaffNumber>,
    pub content: MeasureStyleContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MeasureStyleContent {
    MultipleRest { count: u32, use_symbols: Option<YesNo> },
    MeasureRepeat { r#type: StartStop, slashes: Option<u32> },
    BeatRepeat { r#type: StartStop, slashes: Option<u32> },
    Slash { r#type: StartStop, use_stems: Option<YesNo> },
}

// === Barlines ===

/// Barline.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Barline {
    pub location: Option<RightLeftMiddle>,
    pub bar_style: Option<BarStyle>,
    pub editorial: Editorial,
    pub wavy_line: Option<WavyLine>,
    pub segno: Option<Segno>,
    pub coda: Option<Coda>,
    pub fermatas: Vec<Fermata>,
    pub ending: Option<Ending>,
    pub repeat: Option<Repeat>,
}

/// Bar style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarStyle {
    Regular,
    Dotted,
    Dashed,
    Heavy,
    LightLight,
    LightHeavy,
    HeavyLight,
    HeavyHeavy,
    Tick,
    Short,
    None,
}

/// Repeat barline.
#[derive(Debug, Clone, PartialEq)]
pub struct Repeat {
    pub direction: BackwardForward,
    pub times: Option<u32>,
    pub winged: Option<Winged>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winged {
    None,
    Straight,
    Curved,
    DoubleStraight,
    DoubleCurved,
}

/// Ending (volta).
#[derive(Debug, Clone, PartialEq)]
pub struct Ending {
    pub r#type: StartStopDiscontinue,
    pub number: String,
    pub text: Option<String>,
    pub print_object: Option<YesNo>,
    pub end_length: Option<Tenths>,
    pub text_x: Option<Tenths>,
    pub text_y: Option<Tenths>,
}

/// Wavy line (for trills across barlines).
#[derive(Debug, Clone, PartialEq)]
pub struct WavyLine {
    pub r#type: StartStopContinue,
    pub number: Option<NumberLevel>,
    pub position: Position,
}

// Forward declarations for barline children from direction.rs
pub use super::direction::{Segno, Coda};
use super::note::AccidentalValue;
```

### direction.rs

```rust
//! Direction types: dynamics, wedges, metronome, etc.

use super::common::*;
use super::duration::NoteTypeValue;

/// A musical direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Direction {
    pub placement: Option<AboveBelow>,
    pub directive: Option<YesNo>,
    pub direction_types: Vec<DirectionType>,
    pub offset: Option<Offset>,
    pub voice: Option<Voice>,
    pub staff: Option<StaffNumber>,
    pub sound: Option<Sound>,
}

/// Wrapper for direction type content.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionType {
    pub content: DirectionTypeContent,
}

/// Direction type content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectionTypeContent {
    Rehearsal(Vec<FormattedText>),
    Segno(Vec<Segno>),
    Coda(Vec<Coda>),
    Words(Vec<Words>),
    Symbol(Vec<FormattedSymbol>),
    Wedge(Wedge),
    Dynamics(Dynamics),
    Dashes(Dashes),
    Bracket(Bracket),
    Pedal(Pedal),
    Metronome(Metronome),
    OctaveShift(OctaveShift),
    HarpPedals(HarpPedals),
    Damp(EmptyPrintStyle),
    DampAll(EmptyPrintStyle),
    Eyeglasses(EmptyPrintStyle),
    StringMute(StringMute),
    Scordatura(Scordatura),
    Image(Image),
    PrincipalVoice(PrincipalVoice),
    Percussion(Vec<Percussion>),
    AccordionRegistration(AccordionRegistration),
    StaffDivide(StaffDivide),
    OtherDirection(OtherDirection),
}

/// Segno sign.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Segno {
    pub print_style: PrintStyle,
    pub smufl: Option<String>,
}

/// Coda sign.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Coda {
    pub print_style: PrintStyle,
    pub smufl: Option<String>,
}

/// Text direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Words {
    pub value: String,
    pub print_style: PrintStyle,
    pub justify: Option<LeftCenterRight>,
    pub lang: Option<String>,
}

/// Symbol direction.
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedSymbol {
    pub value: String,
    pub print_style: PrintStyle,
    pub justify: Option<LeftCenterRight>,
}

/// Dynamic markings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dynamics {
    pub content: Vec<DynamicElement>,
    pub print_style: PrintStyle,
    pub placement: Option<AboveBelow>,
}

/// Individual dynamic marking.
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicElement {
    P,
    PP,
    PPP,
    PPPP,
    PPPPP,
    PPPPPP,
    F,
    FF,
    FFF,
    FFFF,
    FFFFF,
    FFFFFF,
    MP,
    MF,
    SF,
    SFP,
    SFPP,
    FP,
    RF,
    RFZ,
    SFZ,
    SFFZ,
    FZ,
    N,
    PF,
    SFZP,
    OtherDynamics(String),
}

/// Crescendo/diminuendo wedge.
#[derive(Debug, Clone, PartialEq)]
pub struct Wedge {
    pub r#type: WedgeType,
    pub number: Option<NumberLevel>,
    pub spread: Option<Tenths>,
    pub niente: Option<YesNo>,
    pub line_type: Option<LineType>,
    pub position: Position,
    pub color: Option<Color>,
}

/// Wedge type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WedgeType {
    Crescendo,
    Diminuendo,
    Stop,
    Continue,
}

/// Dashes for spanning text.
#[derive(Debug, Clone, PartialEq)]
pub struct Dashes {
    pub r#type: StartStopContinue,
    pub number: Option<NumberLevel>,
    pub position: Position,
    pub color: Option<Color>,
}

/// Bracket for grouping.
#[derive(Debug, Clone, PartialEq)]
pub struct Bracket {
    pub r#type: StartStopContinue,
    pub number: Option<NumberLevel>,
    pub line_end: LineEnd,
    pub end_length: Option<Tenths>,
    pub line_type: Option<LineType>,
    pub position: Position,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnd {
    Up,
    Down,
    Both,
    Arrow,
    None,
}

/// Pedal marking.
#[derive(Debug, Clone, PartialEq)]
pub struct Pedal {
    pub r#type: PedalType,
    pub number: Option<NumberLevel>,
    pub line: Option<YesNo>,
    pub sign: Option<YesNo>,
    pub abbreviated: Option<YesNo>,
    pub print_style: PrintStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PedalType {
    Start,
    Stop,
    Sostenuto,
    Change,
    Continue,
    Discontinue,
    Resume,
}

/// Metronome marking.
#[derive(Debug, Clone, PartialEq)]
pub struct Metronome {
    pub parentheses: Option<YesNo>,
    pub content: MetronomeContent,
    pub print_style: PrintStyle,
}

/// Metronome content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum MetronomeContent {
    /// beat-unit = per-minute
    PerMinute {
        beat_unit: NoteTypeValue,
        beat_unit_dots: u32,
        per_minute: PerMinute,
    },
    /// beat-unit = beat-unit (tempo change)
    BeatEquation {
        left_unit: NoteTypeValue,
        left_dots: u32,
        right_unit: NoteTypeValue,
        right_dots: u32,
    },
    /// Metric modulation with metric note groups
    MetricModulation {
        metric_relation: Vec<MetricRelation>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerMinute {
    pub value: String,
    pub font: Font,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricRelation {
    pub left: MetronomeNote,
    pub right: MetronomeNote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetronomeNote {
    pub note_type: NoteTypeValue,
    pub dots: u32,
    pub tuplet: Option<MetronomeTuplet>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetronomeTuplet {
    pub actual_notes: u32,
    pub normal_notes: u32,
    pub r#type: StartStop,
}

/// Octave shift (8va, 8vb, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct OctaveShift {
    pub r#type: UpDownStopContinue,
    pub number: Option<NumberLevel>,
    pub size: Option<u8>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpDownStopContinue {
    Up,
    Down,
    Stop,
    Continue,
}

/// Offset for direction placement.
#[derive(Debug, Clone, PartialEq)]
pub struct Offset {
    pub value: Divisions,
    pub sound: Option<YesNo>,
}

/// Sound element for playback.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sound {
    pub tempo: Option<f64>,
    pub dynamics: Option<f64>,
    pub dacapo: Option<YesNo>,
    pub segno: Option<String>,
    pub dalsegno: Option<String>,
    pub coda: Option<String>,
    pub tocoda: Option<String>,
    pub divisions: Option<Divisions>,
    pub forward_repeat: Option<YesNo>,
    pub fine: Option<String>,
    pub time_only: Option<String>,
    pub pizzicato: Option<YesNo>,
    // MIDI-specific fields omitted for brevity
}

// Placeholder types for less common direction types
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyPrintStyle {
    pub print_style: PrintStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarpPedals {
    pub pedal_tuning: Vec<PedalTuning>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PedalTuning {
    pub pedal_step: super::pitch::Step,
    pub pedal_alter: Semitones,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringMute {
    pub r#type: OnOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnOff {
    On,
    Off,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scordatura {
    pub accord: Vec<Accord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Accord {
    pub string: u8,
    pub tuning_step: super::pitch::Step,
    pub tuning_alter: Option<Semitones>,
    pub tuning_octave: Octave,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub source: String,
    pub r#type: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrincipalVoice {
    pub r#type: StartStop,
    pub symbol: PrincipalVoiceSymbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalVoiceSymbol {
    Hauptstimme,
    Nebenstimme,
    Plain,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Percussion {
    pub content: PercussionContent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PercussionContent {
    Glass(Glass),
    Metal(Metal),
    Wood(Wood),
    Pitched(Pitched),
    Membrane(Membrane),
    Effect(Effect),
    Timpani,
    Beater(Beater),
    Stick(Stick),
    StickLocation(StickLocation),
    OtherPercussion(String),
}

// Simplified percussion types
#[derive(Debug, Clone, PartialEq)]
pub struct Glass { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Metal { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Wood { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Pitched { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Membrane { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Effect { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Beater { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct Stick { pub value: String }
#[derive(Debug, Clone, PartialEq)]
pub struct StickLocation { pub value: String }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccordionRegistration {
    pub accordion_high: bool,
    pub accordion_middle: Option<u8>,
    pub accordion_low: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaffDivide {
    pub r#type: StaffDivideSymbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffDivideSymbol {
    Down,
    Up,
    UpDown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OtherDirection {
    pub value: String,
    pub print_object: Option<YesNo>,
    pub print_style: PrintStyle,
}
```

### notation.rs

```rust
//! Notations: articulations, ornaments, technical, slurs, etc.

use super::common::*;
use super::note::AccidentalValue;

/// Notations container (attached to notes).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Notations {
    pub print_object: Option<YesNo>,
    pub content: Vec<NotationContent>,
    pub editorial: Editorial,
}

/// Notation content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum NotationContent {
    Tied(Tied),
    Slur(Slur),
    Tuplet(Tuplet),
    Glissando(Glissando),
    Slide(Slide),
    Ornaments(Ornaments),
    Technical(Technical),
    Articulations(Articulations),
    Dynamics(super::direction::Dynamics),
    Fermata(Fermata),
    Arpeggiate(Arpeggiate),
    NonArpeggiate(NonArpeggiate),
    AccidentalMark(AccidentalMark),
    OtherNotation(OtherNotation),
}

/// Tied (visual tie marking).
#[derive(Debug, Clone, PartialEq)]
pub struct Tied {
    pub r#type: StartStopContinue,
    pub number: Option<NumberLevel>,
    pub line_type: Option<LineType>,
    pub position: Position,
    pub placement: Option<AboveBelow>,
    pub orientation: Option<OverUnder>,
    pub color: Option<Color>,
}

/// Slur marking.
#[derive(Debug, Clone, PartialEq)]
pub struct Slur {
    pub r#type: StartStopContinue,
    pub number: NumberLevel,
    pub line_type: Option<LineType>,
    pub position: Position,
    pub placement: Option<AboveBelow>,
    pub orientation: Option<OverUnder>,
    pub color: Option<Color>,
}

/// Tuplet notation (visual bracket/number).
#[derive(Debug, Clone, PartialEq)]
pub struct Tuplet {
    pub r#type: StartStop,
    pub number: Option<NumberLevel>,
    pub bracket: Option<YesNo>,
    pub show_number: Option<ShowTuplet>,
    pub show_type: Option<ShowTuplet>,
    pub line_shape: Option<LineShape>,
    pub position: Position,
    pub placement: Option<AboveBelow>,
    pub tuplet_actual: Option<TupletPortion>,
    pub tuplet_normal: Option<TupletPortion>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowTuplet {
    Actual,
    Both,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineShape {
    Straight,
    Curved,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupletPortion {
    pub tuplet_number: Option<TupletNumber>,
    pub tuplet_type: Option<TupletType>,
    pub tuplet_dots: Vec<TupletDot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupletNumber {
    pub value: u32,
    pub font: Font,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupletType {
    pub value: super::duration::NoteTypeValue,
    pub font: Font,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TupletDot {
    pub font: Font,
    pub color: Option<Color>,
}

/// Glissando.
#[derive(Debug, Clone, PartialEq)]
pub struct Glissando {
    pub r#type: StartStop,
    pub number: Option<NumberLevel>,
    pub text: Option<String>,
    pub line_type: Option<LineType>,
    pub position: Position,
}

/// Slide (portamento).
#[derive(Debug, Clone, PartialEq)]
pub struct Slide {
    pub r#type: StartStop,
    pub number: Option<NumberLevel>,
    pub text: Option<String>,
    pub line_type: Option<LineType>,
    pub position: Position,
}

/// Fermata.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Fermata {
    pub shape: Option<FermataShape>,
    pub r#type: Option<UprightInverted>,
    pub print_style: PrintStyle,
}

/// Fermata shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FermataShape {
    Normal,
    Angled,
    Square,
    DoubleAngled,
    DoubleSquare,
    DoubleDot,
    HalfCurve,
    Curlew,
}

/// Arpeggiate.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Arpeggiate {
    pub number: Option<NumberLevel>,
    pub direction: Option<UpDown>,
    pub position: Position,
    pub color: Option<Color>,
}

/// Non-arpeggiate (bracket).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NonArpeggiate {
    pub r#type: TopBottom,
    pub number: Option<NumberLevel>,
    pub position: Position,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBottom {
    Top,
    Bottom,
}

/// Accidental mark in ornaments.
#[derive(Debug, Clone, PartialEq)]
pub struct AccidentalMark {
    pub value: AccidentalValue,
    pub placement: Option<AboveBelow>,
    pub print_style: PrintStyle,
}

/// Other notation.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherNotation {
    pub value: String,
    pub r#type: StartStopSingle,
    pub number: Option<NumberLevel>,
    pub print_object: Option<YesNo>,
    pub print_style: PrintStyle,
    pub placement: Option<AboveBelow>,
}

// === Articulations ===

/// Articulations container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Articulations {
    pub content: Vec<ArticulationElement>,
}

/// Individual articulation types.
#[derive(Debug, Clone, PartialEq)]
pub enum ArticulationElement {
    Accent(EmptyPlacement),
    StrongAccent(StrongAccent),
    Staccato(EmptyPlacement),
    Tenuto(EmptyPlacement),
    DetachedLegato(EmptyPlacement),
    Staccatissimo(EmptyPlacement),
    Spiccato(EmptyPlacement),
    Scoop(EmptyLine),
    Plop(EmptyLine),
    Doit(EmptyLine),
    Falloff(EmptyLine),
    BreathMark(BreathMark),
    Caesura(Caesura),
    Stress(EmptyPlacement),
    Unstress(EmptyPlacement),
    SoftAccent(EmptyPlacement),
    OtherArticulation(OtherArticulation),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StrongAccent {
    pub r#type: Option<UpDown>,
    pub placement: Option<AboveBelow>,
    pub position: Position,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyLine {
    pub line_shape: Option<LineShape>,
    pub line_type: Option<LineType>,
    pub line_length: Option<LineLength>,
    pub placement: Option<AboveBelow>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineLength {
    Short,
    Medium,
    Long,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreathMark {
    pub value: BreathMarkValue,
    pub placement: Option<AboveBelow>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreathMarkValue {
    Empty,
    Comma,
    Tick,
    Upbow,
    Salzedo,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Caesura {
    pub value: CaesuraValue,
    pub placement: Option<AboveBelow>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaesuraValue {
    Normal,
    Thick,
    Short,
    Curved,
    Single,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OtherArticulation {
    pub value: String,
    pub placement: Option<AboveBelow>,
    pub print_style: PrintStyle,
}

// === Ornaments ===

/// Ornaments container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ornaments {
    pub content: Vec<OrnamentWithAccidentals>,
}

/// Ornament with optional accidental marks.
#[derive(Debug, Clone, PartialEq)]
pub struct OrnamentWithAccidentals {
    pub ornament: OrnamentElement,
    pub accidental_marks: Vec<AccidentalMark>,
}

/// Individual ornament types.
#[derive(Debug, Clone, PartialEq)]
pub enum OrnamentElement {
    TrillMark(EmptyTrillSound),
    Turn(Turn),
    DelayedTurn(Turn),
    InvertedTurn(Turn),
    DelayedInvertedTurn(Turn),
    VerticalTurn(EmptyTrillSound),
    InvertedVerticalTurn(EmptyTrillSound),
    Shake(EmptyTrillSound),
    WavyLine(super::attributes::WavyLine),
    Mordent(Mordent),
    InvertedMordent(Mordent),
    Schleifer(EmptyPlacement),
    Tremolo(Tremolo),
    Haydn(EmptyTrillSound),
    OtherOrnament(OtherOrnament),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyTrillSound {
    pub placement: Option<AboveBelow>,
    pub position: Position,
    pub start_note: Option<StartNote>,
    pub trill_step: Option<TrillStep>,
    pub two_note_turn: Option<TwoNoteTurn>,
    pub accelerate: Option<YesNo>,
    pub beats: Option<f64>,
    pub second_beat: Option<f64>,
    pub last_beat: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartNote { Upper, Main, Below }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrillStep { Whole, Half, Unison }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoNoteTurn { Whole, Half, None }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Turn {
    pub slash: Option<YesNo>,
    pub placement: Option<AboveBelow>,
    pub position: Position,
    pub start_note: Option<StartNote>,
    pub trill_step: Option<TrillStep>,
    pub two_note_turn: Option<TwoNoteTurn>,
    pub accelerate: Option<YesNo>,
    pub beats: Option<f64>,
    pub second_beat: Option<f64>,
    pub last_beat: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mordent {
    pub long: Option<YesNo>,
    pub approach: Option<AboveBelow>,
    pub departure: Option<AboveBelow>,
    pub placement: Option<AboveBelow>,
    pub position: Position,
    pub start_note: Option<StartNote>,
    pub trill_step: Option<TrillStep>,
    pub two_note_turn: Option<TwoNoteTurn>,
    pub accelerate: Option<YesNo>,
    pub beats: Option<f64>,
    pub second_beat: Option<f64>,
    pub last_beat: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tremolo {
    pub value: u8,
    pub r#type: Option<TremoloType>,
    pub placement: Option<AboveBelow>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TremoloType {
    Start,
    Stop,
    Single,
    Unmeasured,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OtherOrnament {
    pub value: String,
    pub placement: Option<AboveBelow>,
    pub print_style: PrintStyle,
}

// === Technical ===

/// Technical indications.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Technical {
    pub content: Vec<TechnicalElement>,
}

/// Individual technical elements.
#[derive(Debug, Clone, PartialEq)]
pub enum TechnicalElement {
    UpBow(EmptyPlacement),
    DownBow(EmptyPlacement),
    Harmonic(Harmonic),
    OpenString(EmptyPlacement),
    ThumbPosition(EmptyPlacement),
    Fingering(Fingering),
    Pluck(Pluck),
    DoubleTongue(EmptyPlacement),
    TripleTongue(EmptyPlacement),
    Stopped(EmptyPlacement),
    SnapPizzicato(EmptyPlacement),
    Fret(Fret),
    String(StringNumber),
    HammerOn(HammerPull),
    PullOff(HammerPull),
    Bend(Bend),
    Tap(Tap),
    Heel(HeelToe),
    Toe(HeelToe),
    Fingernails(EmptyPlacement),
    Hole(Hole),
    Arrow(Arrow),
    Handbell(Handbell),
    BrassBend(EmptyPlacement),
    Flip(EmptyPlacement),
    Smear(EmptyPlacement),
    Open(EmptyPlacement),
    HalfMuted(EmptyPlacement),
    HarmonMute(HarmonMute),
    Golpe(EmptyPlacement),
    OtherTechnical(OtherTechnical),
}

// Simplified technical types
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Harmonic {
    pub natural: bool,
    pub artificial: bool,
    pub base_pitch: bool,
    pub touching_pitch: bool,
    pub sounding_pitch: bool,
    pub placement: Option<AboveBelow>,
    pub print_object: Option<YesNo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fingering {
    pub value: String,
    pub substitution: Option<YesNo>,
    pub alternate: Option<YesNo>,
    pub placement: Option<AboveBelow>,
    pub print_style: PrintStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pluck {
    pub value: String,
    pub placement: Option<AboveBelow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fret {
    pub value: u8,
    pub font: Font,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringNumber {
    pub value: u8,
    pub placement: Option<AboveBelow>,
    pub print_style: PrintStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HammerPull {
    pub value: String,
    pub r#type: StartStop,
    pub number: Option<NumberLevel>,
    pub placement: Option<AboveBelow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bend {
    pub bend_alter: Semitones,
    pub pre_bend: bool,
    pub release: Option<BendRelease>,
    pub with_bar: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BendRelease { Early, Late }

#[derive(Debug, Clone, PartialEq)]
pub struct Tap {
    pub value: String,
    pub hand: Option<TapHand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapHand { Left, Right }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeelToe {
    pub substitution: Option<YesNo>,
    pub placement: Option<AboveBelow>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hole {
    pub hole_type: Option<String>,
    pub hole_closed: HoleClosed,
    pub hole_shape: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HoleClosed {
    pub value: HoleClosedValue,
    pub location: Option<HoleClosedLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoleClosedValue { Yes, No, Half }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoleClosedLocation { Right, Bottom, Left, Top }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Arrow {
    pub direction: Option<ArrowDirection>,
    pub style: Option<ArrowStyle>,
    pub smufl: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowDirection {
    Left, Up, Right, Down,
    Northwest, Northeast, Southeast, Southwest,
    LeftRight, UpDown, NorthwestSoutheast, NortheastSouthwest,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowStyle { Single, Double, Filled, Hollow, Paired, Combined, Other }

#[derive(Debug, Clone, PartialEq)]
pub struct Handbell {
    pub value: HandbellValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandbellValue {
    Belltree, Damp, Echo, Gyro, HandMartellato, MalletLift, MalletTable,
    Martellato, MartellatoLift, MutedMartellato, PluckLift, Swing,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarmonMute {
    pub open: bool,
    pub half: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OtherTechnical {
    pub value: String,
    pub placement: Option<AboveBelow>,
    pub print_style: PrintStyle,
}
```

### voice.rs

```rust
//! Voice-related types: backup, forward.

use super::common::*;

/// Move backward in time within a measure (for multiple voices).
#[derive(Debug, Clone, PartialEq)]
pub struct Backup {
    pub duration: PositiveDivisions,
    pub editorial: Editorial,
}

/// Move forward in time within a measure.
#[derive(Debug, Clone, PartialEq)]
pub struct Forward {
    pub duration: PositiveDivisions,
    pub voice: Option<Voice>,
    pub staff: Option<StaffNumber>,
    pub editorial: Editorial,
}
```

### lyric.rs

```rust
//! Lyric types.

use super::common::*;

/// Lyric element attached to a note.
#[derive(Debug, Clone, PartialEq)]
pub struct Lyric {
    pub number: Option<String>,
    pub name: Option<String>,
    pub justify: Option<LeftCenterRight>,
    pub placement: Option<AboveBelow>,
    pub print_object: Option<YesNo>,
    pub content: LyricContent,
    pub end_line: bool,
    pub end_paragraph: bool,
}

/// Lyric content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum LyricContent {
    /// A syllable with text
    Syllable {
        syllabic: Option<Syllabic>,
        text: TextElementData,
        /// Additional syllables after elisions
        extensions: Vec<LyricExtension>,
        /// Extending line
        extend: Option<Extend>,
    },
    /// Just an extending line
    ExtendOnly(Extend),
    /// Laughing indication
    Laughing,
    /// Humming indication
    Humming,
}

/// Syllabic position in word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syllabic {
    Single,
    Begin,
    End,
    Middle,
}

/// Additional syllable after elision.
#[derive(Debug, Clone, PartialEq)]
pub struct LyricExtension {
    pub elision: Elision,
    pub syllabic: Option<Syllabic>,
    pub text: TextElementData,
}

/// Text element with formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct TextElementData {
    pub value: String,
    pub font: Font,
    pub color: Option<Color>,
    pub lang: Option<String>,
}

/// Elision between syllables.
#[derive(Debug, Clone, PartialEq)]
pub struct Elision {
    pub value: String,
    pub font: Font,
    pub color: Option<Color>,
}

/// Lyric extension line.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Extend {
    pub r#type: Option<StartStopContinue>,
    pub position: Position,
    pub color: Option<Color>,
}
```

### measure.rs

```rust
//! Measure and music data types.

use super::common::*;
use super::note::Note;
use super::attributes::{Attributes, Barline};
use super::direction::Direction;
use super::voice::{Backup, Forward};

/// A measure within a part.
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
    pub number: String,
    pub implicit: Option<YesNo>,
    pub non_controlling: Option<YesNo>,
    pub width: Option<Tenths>,
    pub content: Vec<MusicDataElement>,
}

/// Elements that can appear within a measure.
#[derive(Debug, Clone, PartialEq)]
pub enum MusicDataElement {
    Note(Note),
    Backup(Backup),
    Forward(Forward),
    Direction(Direction),
    Attributes(Attributes),
    // Harmony(Harmony), // Deferred
    // FiguredBass(FiguredBass), // Deferred
    // Print(Print), // Deferred
    // Sound(Sound), // Covered in Direction
    // Listening(Listening), // Deferred
    Barline(Barline),
    // Grouping(Grouping), // Deferred
    // Link(Link), // Deferred
    // Bookmark(Bookmark), // Deferred
}
```

### part.rs

```rust
//! Part and part-list types.

use super::common::*;
use super::measure::Measure;

/// A musical part containing measures.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    pub id: String,
    pub measures: Vec<Measure>,
}

/// The part-list element.
#[derive(Debug, Clone, PartialEq)]
pub struct PartList {
    pub content: Vec<PartListElement>,
}

/// Elements within part-list.
#[derive(Debug, Clone, PartialEq)]
pub enum PartListElement {
    ScorePart(ScorePart),
    PartGroup(PartGroup),
}

/// Score-part definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ScorePart {
    pub id: String,
    pub identification: Option<Identification>,
    pub part_name: PartName,
    pub part_name_display: Option<NameDisplay>,
    pub part_abbreviation: Option<PartName>,
    pub part_abbreviation_display: Option<NameDisplay>,
    pub group: Vec<String>,
    pub score_instruments: Vec<ScoreInstrument>,
    pub midi_devices: Vec<MidiDevice>,
    pub midi_instruments: Vec<MidiInstrument>,
}

/// Part name.
#[derive(Debug, Clone, PartialEq)]
pub struct PartName {
    pub value: String,
    pub print_style: PrintStyle,
    pub print_object: Option<YesNo>,
    pub justify: Option<LeftCenterRight>,
}

/// Name display for alternative formatting.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NameDisplay {
    pub print_object: Option<YesNo>,
    pub content: Vec<NameDisplayContent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NameDisplayContent {
    DisplayText(FormattedText),
    AccidentalText(AccidentalText),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccidentalText {
    pub value: super::note::AccidentalValue,
    pub print_style: PrintStyle,
}

/// Score instrument (for playback).
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreInstrument {
    pub id: String,
    pub instrument_name: String,
    pub instrument_abbreviation: Option<String>,
    pub instrument_sound: Option<String>,
    pub solo_or_ensemble: Option<SoloOrEnsemble>,
    pub virtual_instrument: Option<VirtualInstrument>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SoloOrEnsemble {
    Solo,
    Ensemble(u32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VirtualInstrument {
    pub virtual_library: Option<String>,
    pub virtual_name: Option<String>,
}

/// MIDI device assignment.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MidiDevice {
    pub value: String,
    pub port: Option<u16>,
    pub id: Option<String>,
}

/// MIDI instrument settings.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiInstrument {
    pub id: String,
    pub midi_channel: Option<u8>,
    pub midi_name: Option<String>,
    pub midi_bank: Option<u16>,
    pub midi_program: Option<u8>,
    pub midi_unpitched: Option<u8>,
    pub volume: Option<f64>,
    pub pan: Option<f64>,
    pub elevation: Option<f64>,
}

/// Part group (for grouping parts in the score).
#[derive(Debug, Clone, PartialEq)]
pub struct PartGroup {
    pub r#type: StartStop,
    pub number: Option<String>,
    pub group_name: Option<GroupName>,
    pub group_name_display: Option<NameDisplay>,
    pub group_abbreviation: Option<GroupName>,
    pub group_abbreviation_display: Option<NameDisplay>,
    pub group_symbol: Option<GroupSymbol>,
    pub group_barline: Option<GroupBarline>,
    pub group_time: Option<()>, // Empty element
    pub editorial: Editorial,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupName {
    pub value: String,
    pub print_style: PrintStyle,
    pub justify: Option<LeftCenterRight>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupSymbol {
    pub value: super::attributes::GroupSymbolValue,
    pub position: Position,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupBarline {
    pub value: GroupBarlineValue,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBarlineValue {
    Yes,
    No,
    Mensurstrich,
}

// Forward reference
pub use super::score::Identification;
```

### score.rs

```rust
//! Score-level types.

use super::common::*;
use super::part::{Part, PartList};

/// The root score-partwise element.
#[derive(Debug, Clone, PartialEq)]
pub struct ScorePartwise {
    pub version: Option<String>,
    pub work: Option<Work>,
    pub movement_number: Option<String>,
    pub movement_title: Option<String>,
    pub identification: Option<Identification>,
    pub defaults: Option<Defaults>,
    pub credits: Vec<Credit>,
    pub part_list: PartList,
    pub parts: Vec<Part>,
}

/// Work information.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Work {
    pub work_number: Option<String>,
    pub work_title: Option<String>,
    pub opus: Option<Opus>,
}

/// Opus reference.
#[derive(Debug, Clone, PartialEq)]
pub struct Opus {
    pub href: String,
}

/// Identification (creators, rights, encoding).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Identification {
    pub creators: Vec<TypedText>,
    pub rights: Vec<TypedText>,
    pub encoding: Option<Encoding>,
    pub source: Option<String>,
    pub relations: Vec<TypedText>,
    pub miscellaneous: Option<Miscellaneous>,
}

/// Text with a type attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedText {
    pub value: String,
    pub r#type: Option<String>,
}

/// Encoding information.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Encoding {
    pub content: Vec<EncodingContent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncodingContent {
    EncodingDate(String),
    Encoder(TypedText),
    Software(String),
    EncodingDescription(String),
    Supports(Supports),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Supports {
    pub r#type: YesNo,
    pub element: String,
    pub attribute: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Miscellaneous {
    pub fields: Vec<MiscellaneousField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MiscellaneousField {
    pub name: String,
    pub value: String,
}

/// Score defaults (layout, scaling, fonts).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Defaults {
    pub scaling: Option<Scaling>,
    pub page_layout: Option<PageLayout>,
    pub system_layout: Option<SystemLayout>,
    pub staff_layout: Vec<StaffLayout>,
    pub appearance: Option<Appearance>,
    pub music_font: Option<Font>,
    pub word_font: Option<Font>,
    pub lyric_fonts: Vec<LyricFont>,
    pub lyric_languages: Vec<LyricLanguage>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scaling {
    pub millimeters: f64,
    pub tenths: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageLayout {
    pub page_height: Option<Tenths>,
    pub page_width: Option<Tenths>,
    pub page_margins: Vec<PageMargins>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageMargins {
    pub r#type: Option<MarginType>,
    pub left: Tenths,
    pub right: Tenths,
    pub top: Tenths,
    pub bottom: Tenths,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginType { Odd, Even, Both }

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemLayout {
    pub system_margins: Option<SystemMargins>,
    pub system_distance: Option<Tenths>,
    pub top_system_distance: Option<Tenths>,
    pub system_dividers: Option<SystemDividers>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemMargins {
    pub left: Tenths,
    pub right: Tenths,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemDividers {
    pub left_divider: Option<Divider>,
    pub right_divider: Option<Divider>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Divider {
    pub print_object: Option<YesNo>,
    pub print_style: PrintStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaffLayout {
    pub number: Option<StaffNumber>,
    pub staff_distance: Option<Tenths>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Appearance {
    pub line_widths: Vec<LineWidth>,
    pub note_sizes: Vec<NoteSize>,
    pub distances: Vec<Distance>,
    pub other_appearances: Vec<OtherAppearance>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineWidth {
    pub r#type: String,
    pub value: Tenths,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoteSize {
    pub r#type: NoteSizeType,
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteSizeType { Cue, Grace, GraceCue, Large }

#[derive(Debug, Clone, PartialEq)]
pub struct Distance {
    pub r#type: String,
    pub value: Tenths,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OtherAppearance {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LyricFont {
    pub number: Option<String>,
    pub name: Option<String>,
    pub font: Font,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LyricLanguage {
    pub number: Option<String>,
    pub name: Option<String>,
    pub lang: String,
}

/// Credit for title, composer, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Credit {
    pub page: Option<u32>,
    pub content: Vec<CreditContent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreditContent {
    CreditType(String),
    Link(Link),
    Bookmark(Bookmark),
    CreditImage(CreditImage),
    CreditWords(CreditWords),
    CreditSymbol(CreditSymbol),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    pub href: String,
    pub r#type: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
    pub show: Option<String>,
    pub actuate: Option<String>,
    pub name: Option<String>,
    pub element: Option<String>,
    pub position: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bookmark {
    pub id: String,
    pub name: Option<String>,
    pub element: Option<String>,
    pub position: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditImage {
    pub source: String,
    pub r#type: String,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditWords {
    pub value: String,
    pub print_style: PrintStyle,
    pub justify: Option<LeftCenterRight>,
    pub halign: Option<LeftCenterRight>,
    pub valign: Option<super::common::TopMiddleBottom>,
    pub lang: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditSymbol {
    pub value: String,
    pub print_style: PrintStyle,
    pub justify: Option<LeftCenterRight>,
    pub halign: Option<LeftCenterRight>,
    pub valign: Option<super::common::TopMiddleBottom>,
}
```

### mod.rs

```rust
//! Fermata IR (Intermediate Representation) types.
//!
//! This module defines the typed data structures that represent MusicXML content.
//! The IR provides a lossless, round-trippable representation of MusicXML documents.

pub mod common;
pub mod pitch;
pub mod duration;
pub mod note;
pub mod beam;
pub mod attributes;
pub mod direction;
pub mod notation;
pub mod voice;
pub mod lyric;
pub mod measure;
pub mod part;
pub mod score;

// Re-export main types for convenience
pub use score::ScorePartwise;
pub use part::{Part, PartList, ScorePart};
pub use measure::{Measure, MusicDataElement};
pub use note::{Note, NoteContent, FullNote, Rest, Grace, Accidental};
pub use pitch::{Pitch, Step, Unpitched};
pub use duration::{NoteType, NoteTypeValue, Dot, TimeModification};
pub use attributes::{Attributes, Key, Time, Clef, Barline};
pub use direction::{Direction, DirectionType, Dynamics, Wedge, Metronome};
pub use notation::{Notations, Articulations, Ornaments, Technical, Fermata, Slur, Tied, Tuplet};
pub use beam::{Beam, Stem, Notehead};
pub use voice::{Backup, Forward};
pub use lyric::{Lyric, Syllabic};
pub use common::*;
```

---

## Task Checklist

1. [ ] Create `src/ir/` directory
2. [ ] Create all module files listed above
3. [ ] Update `src/lib.rs` to include `pub mod ir;`
4. [ ] Ensure all files compile without errors (`cargo check`)
5. [ ] Run `cargo fmt` to format code
6. [ ] Run `cargo clippy` and fix any warnings

## Validation

After implementation, this should work:

```rust
use fermata::ir::{ScorePartwise, Part, Measure, Note, Pitch, Step};

fn main() {
    let pitch = Pitch {
        step: Step::C,
        alter: None,
        octave: 4,
    };
    println!("Created pitch: {:?}", pitch);
}
```

---

## Notes

- Some types have forward references (e.g., `Stem` and `Notehead` used in `note.rs` but defined in `beam.rs`). Use `pub use` re-exports to handle this.
- The `r#type` syntax is used for fields named `type` (reserved word in Rust).
- Implement `Default` where it makes sense (types with many optional fields).
- This is the IR layer only — no parsing, emission, or S-expr handling yet.

**Good luck! Build a solid foundation.** 🏗️
