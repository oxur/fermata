# Fermata IR Quick Reference

> **Purpose:** Reference for agents implementing S-expr serialization.
> **Location:** `crates/fermata/src/ir/`

---

## Type Aliases (common.rs)

```rust
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
```

---

## Common Enums (common.rs)

```rust
pub enum YesNo { Yes, No }
pub enum StartStop { Start, Stop }
pub enum StartStopContinue { Start, Stop, Continue }
pub enum StartStopSingle { Start, Stop, Single }
pub enum StartStopDiscontinue { Start, Stop, Discontinue }
pub enum AboveBelow { Above, Below }
pub enum UpDown { Up, Down }
pub enum OverUnder { Over, Under }
pub enum LeftCenterRight { Left, Center, Right }
pub enum TopMiddleBottom { Top, Middle, Bottom }
pub enum BackwardForward { Backward, Forward }
pub enum FontStyle { Normal, Italic }
pub enum FontWeight { Normal, Bold }
pub enum LineType { Solid, Dashed, Dotted, Wavy }
```

---

## Common Structs (common.rs)

```rust
pub struct Position {
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,
}

pub struct Font {
    pub family: Option<String>,
    pub style: Option<FontStyle>,
    pub size: Option<FontSize>,
    pub weight: Option<FontWeight>,
}

pub struct PrintStyle {
    pub position: Position,
    pub font: Font,
    pub color: Option<Color>,
}
```

---

## Pitch (pitch.rs)

```rust
pub struct Pitch {
    pub step: Step,
    pub alter: Option<Semitones>,  // f64
    pub octave: Octave,            // u8
}

pub enum Step { A, B, C, D, E, F, G }

pub struct Unpitched {
    pub display_step: Option<Step>,
    pub display_octave: Option<Octave>,
}
```

---

## Note (note.rs)

```rust
pub struct Note {
    pub content: NoteContent,
    pub instrument: Option<Instrument>,
    pub editorial_voice: EditorialVoice,
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
    // ... more fields
}

pub enum NoteContent {
    Regular {
        full_note: FullNote,
        duration: PositiveDivisions,
        ties: Vec<Tie>,
    },
    Grace {
        grace: Grace,
        full_note: FullNote,
        ties: Vec<Tie>,
    },
    Cue {
        full_note: FullNote,
        duration: PositiveDivisions,
    },
}

pub struct FullNote {
    pub chord: bool,
    pub content: PitchRestUnpitched,
}

pub enum PitchRestUnpitched {
    Pitch(Pitch),
    Rest(Rest),
    Unpitched(Unpitched),
}

pub struct Rest {
    pub display_step: Option<Step>,
    pub display_octave: Option<Octave>,
    pub measure: Option<YesNo>,
}
```

---

## Duration (duration.rs)

```rust
pub struct NoteType {
    pub value: NoteTypeValue,
    pub size: Option<SymbolSize>,
}

pub enum NoteTypeValue {
    Maxima, Long, Breve, Whole, Half, Quarter, Eighth,
    Sixteenth, ThirtySecond, SixtyFourth,
    OneHundredTwentyEighth, TwoHundredFiftySixth,
    FiveHundredTwelfth, OneThousandTwentyFourth,
}

pub struct Dot {
    pub print_style: PrintStyle,
    pub placement: Option<AboveBelow>,
}

pub struct TimeModification {
    pub actual_notes: u32,
    pub normal_notes: u32,
    pub normal_type: Option<NoteTypeValue>,
    pub normal_dots: Vec<Dot>,  // Vec, not u32!
}
```

---

## Attributes (attributes.rs)

```rust
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
    pub transpose: Vec<Transpose>,  // NOTE: "transpose" not "transposes"
    pub measure_styles: Vec<MeasureStyle>,
}

pub struct Key {
    pub number: Option<StaffNumber>,
    pub content: KeyContent,
    pub print_style: PrintStyle,
    pub print_object: Option<YesNo>,
}

pub enum KeyContent {
    Traditional(TraditionalKey),      // Tuple variant!
    NonTraditional(Vec<KeyStep>),     // Tuple variant!
}

pub struct TraditionalKey {
    pub cancel: Option<Cancel>,
    pub fifths: i32,
    pub mode: Option<Mode>,
}

pub struct Time {
    pub number: Option<StaffNumber>,
    pub content: TimeContent,
    pub print_style_align: PrintStyleAlign,
    pub print_object: Option<YesNo>,
}

pub enum TimeContent {
    Measured { signatures: Vec<TimeSignature> },  // Has "signatures" field
    SenzaMisura(String),
}

pub struct TimeSignature {
    pub beats: String,
    pub beat_type: String,
}

pub struct Clef {
    pub number: Option<StaffNumber>,
    pub sign: ClefSign,
    pub line: Option<i32>,
    pub octave_change: Option<i32>,
    pub print_style: PrintStyle,
    pub print_object: Option<YesNo>,
}
```

---

## Measure (measure.rs)

```rust
pub struct Measure {
    pub number: String,
    pub implicit: Option<YesNo>,
    pub non_controlling: Option<YesNo>,
    pub width: Option<Tenths>,
    pub content: Vec<MusicDataElement>,
}

pub enum MusicDataElement {
    Note(Box<Note>),
    Backup(Backup),
    Forward(Forward),
    Direction(Box<Direction>),
    Attributes(Box<Attributes>),
    Barline(Barline),
    // ... more variants
}
```

---

## Part & Score (part.rs, score.rs)

```rust
pub struct Part {
    pub id: String,
    pub measures: Vec<Measure>,
}

pub struct PartList {
    pub content: Vec<PartListElement>,
}

pub enum PartListElement {
    ScorePart(ScorePart),
    PartGroup(PartGroup),
}

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
```

---

## Common Mistakes to Avoid

| Wrong | Correct |
|-------|---------|
| `attrs.transposes` | `attrs.transpose` |
| `KeyContent::Traditional { fifths, .. }` | `KeyContent::Traditional(TraditionalKey { .. })` |
| `TimeContent::Measured { beats, beat_types }` | `TimeContent::Measured { signatures }` |
| `TimeContent::Unmeasured` | `TimeContent::SenzaMisura(String)` |
| `note.pitch` | `note.content` → `FullNote` → `content` → `PitchRestUnpitched::Pitch` |
| `Barline` in `ir::measure` | `Barline` in `ir::attributes` |
| `Rest` in `ir::note` | `Rest` in `ir::pitch` |

---

## Import Patterns

```rust
// Core types
use crate::ir::score::ScorePartwise;
use crate::ir::part::{Part, PartList, PartListElement, ScorePart};
use crate::ir::measure::{Measure, MusicDataElement};
use crate::ir::note::{Note, NoteContent, FullNote, PitchRestUnpitched};
use crate::ir::pitch::{Pitch, Step, Rest, Unpitched};
use crate::ir::attributes::{Attributes, Key, Time, Clef, Barline};
use crate::ir::duration::{NoteType, NoteTypeValue, Dot, TimeModification};
use crate::ir::common::{YesNo, StartStop, Position, PrintStyle};
```

---

*Generated from `crates/fermata/src/ir/`*
