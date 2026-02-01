# Fermata Lisp — Project Bootstrap Document

> **Purpose:** This document captures the complete context needed to continue development on Fermata. It bootstraps a new Claude instance (or human collaborator) into the project's current state, design decisions, and next steps.

---

## Quick Start

**What is Fermata?** An S-expression DSL for music notation that compiles to MusicXML.

**Where are we?**

* [x] Phase 1 (IR types) is complete
* [x] Phase 2 (MusicXML emitter) is complete
* [x] Phase 3 (the MusicXML parser) is complete
* [x] Phase 4 (S-expr Read/Print) is next
* [ ] Phase 5 (Fermata Syntax)
* [ ] Phase 6 (REPL + Verovio Integration)

**Repository structure:**

```
oxur/fermata/        # remote: ssh://git@codeberg.org/oxur/
└── crates/fermata/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── main.rs
        └── ir/
            ├── mod.rs
            ├── common.rs
            ├── pitch.rs
            ├── duration.rs
            ├── note.rs
            ├── beam.rs
            ├── attributes.rs
            ├── direction.rs
            ├── notation.rs
            ├── voice.rs
            ├── lyric.rs
            ├── measure.rs
            ├── part.rs
            └── score.rs
```

---

## Project Vision

### The Problem

MusicXML is the standard interchange format for music notation, but it's painfully verbose:

```xml
<note>
  <pitch>
    <step>C</step>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <type>quarter</type>
</note>
```

### The Solution

Fermata provides a terse, expressive S-expression syntax:

```lisp
(note c4 :q)
```

### Primary Use Case

Enable **precise human-AI communication about music theory**. When discussing voice leading, counterpoint, or harmonic analysis, we need exact musical notation—not vague descriptions.

Instead of: "Play a C major chord in root position, quarter note duration"

We want:

```lisp
(chord :q (c4 e4 g4))
```

---

## Architecture

### Two-Layer Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                     Fermata Syntax (User-Facing)                    │
│                                                                     │
│   (score :title "Twinkle"                                           │
│     (part :piano                                                    │
│       (measure (note c4 :q) (note c4 :q) ...)))                     │
│                                                                     │
│   - Terse, ergonomic                                                │
│   - Positional args for common cases                                │
│   - Defaults inferred (voice, staff, stem direction)                │
│   - Macros: (transpose +2 ...), (scale c4 :major), etc.             │
└───────────────────────────┬─────────────────────────────────────────┘
                            │ compile / desugar
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     Music IR (Implemented ✅)                       │
│                                                                     │
│   (score-partwise :version "4.0"                                    │
│     (part-list (score-part :id "P1" :name "Piano"))                 │
│     (part :id "P1"                                                  │
│       (measure :number "1"                                          │
│         (attributes :divisions 1 ...)                               │
│         (note :pitch (pitch :step :C :octave 4) ...))))             │
│                                                                     │
│   - Lossless, explicit                                              │
│   - 1:1 correspondence with MusicXML                                │
│   - All keywords, no positional args                                │
│   - No macros—fully expanded                                        │
└───────────────────────────┬─────────────────────────────────────────┘
                            │ emit / parse
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          MusicXML                                   │
└─────────────────────────────────────────────────────────────────────┘
```

### Key Architectural Decision

**Typed Music IR as the hub** — not a generic S-expression AST.

This means:

* Rust developers get an ergonomic typed API
* Validation happens at parse time
* MusicXML emitter works with concrete types
* Round-tripping is lossless

---

## Design Decisions (Locked In)

### Parser

* **Choice:** `nom` (not `pest`)

* **Rationale:** S-exprs are simple; `nom` gives better error control and streaming capability for REPL

### Two-Phase Parsing

```
Fermata source → S-expr (untyped) → Music IR (typed)
```

### Pitch Notation

* **Format:** Lowercase scientific pitch notation

* **Examples:** `c4` (middle C), `f#5`, `bb3`, `cn4` (explicit natural)
* **Internal representation:**

  ```rust
  pub struct Pitch {
      pub step: Step,        // A, B, C, D, E, F, G
      pub alter: Option<Semitones>,  // -2.0 to +2.0 (supports microtones)
      pub octave: Octave,    // 0-9, where 4 = middle C octave
  }
  ```

### Duration Keywords

Support short, long, AND British names:

| Short | Long | British | Value |
|-------|------|---------|-------|
| `:w` | `:whole` | `:semibreve` | Whole note |
| `:h` | `:half` | `:minim` | Half note |
| `:q` | `:quarter` | `:crotchet` | Quarter note |
| `:8` | `:eighth` | `:quaver` | Eighth note |
| `:16` | `:sixteenth` | `:semiquaver` | Sixteenth note |
| `:32` | `:thirty-second` | `:demisemiquaver` | 32nd note |

### Internal Duration Representation

Rational-based to handle dots and tuplets:

```rust
pub struct TimeModification {
    pub actual_notes: u32,    // e.g., 3 for triplet
    pub normal_notes: u32,    // e.g., 2 for triplet
    pub normal_type: Option<NoteTypeValue>,
    pub normal_dots: u32,
}
```

### Dynamics

**Separate positioned elements**, not modifiers on notes:

```lisp
(measure
  (dynamic :ff)
  (note c4 :q)    ; ff applies here
  (note d4 :q))
```

This matches MusicXML's `<direction>` model where dynamics attach to positions.

### Tuplets

**Wrapper form with explicit note durations** (matching MusicXML):

```lisp
(tuplet (3 2)
  (note c4 :8)
  (note d4 :8)
  (note e4 :8))
```

Each note specifies its own notated duration. The tuplet applies the time modification ratio (3:2 = triplet).

MusicXML attaches `<time-modification>` to each note individually, and we preserve this.

### Chords

Use `:chord t` flag on subsequent notes (matching MusicXML's model):

```lisp
(note :pitch (pitch :step :C :octave 4) :duration 4 :type :quarter)
(note :chord t :pitch (pitch :step :E :octave 4) :duration 4 :type :quarter)
(note :chord t :pitch (pitch :step :G :octave 4) :duration 4 :type :quarter)
```

### Tie vs Tied

Both preserved (MusicXML distinguishes them):

* `<tie>` — Playback (sound)
* `<tied>` — Visual (notation)

### Voice Type

`String`, not integer — allows custom identifiers like "1a"

---

## Implementation Phases

### Phase 1: IR Types ✅ COMPLETE

* All Rust types implemented in `src/ir/`

* Matches MusicXML 4.0 structure
* Derives `Debug, Clone, PartialEq`

### Phase 2: MusicXML Emitter ← **YOU ARE HERE**

* Walk IR tree → emit MusicXML

* Handle `divisions` calculations
* Manage element ordering
* Test with real notation software (MuseScore, Finale, etc.)

### Phase 3: MusicXML Parser

* Parse MusicXML → IR

* Validates round-trip capability

### Phase 4: S-expr Read/Print

* Pretty-printer: IR → S-expr text

* Reader: S-expr text → IR
* This is the low-level IR syntax

### Phase 5: Fermata Syntax

* Higher-level ergonomic syntax

* Desugars to IR
* Macros for music theory operations

### Phase 6: REPL + Verovio Integration

* Interactive exploration

* Visual rendering via verovioxide

---

## IR Module Summary

### Core Types

| Module | Primary Types | Purpose |
|--------|---------------|---------|
| `common` | `YesNo`, `StartStop`, `Position`, `Font`, `PrintStyle` | Shared enums and attribute groups |
| `pitch` | `Pitch`, `Step`, `Unpitched` | Pitch representation |
| `duration` | `NoteType`, `NoteTypeValue`, `Dot`, `TimeModification` | Duration and rhythm |
| `note` | `Note`, `NoteContent`, `FullNote`, `Rest`, `Grace`, `Accidental` | The note element (complex!) |
| `beam` | `Beam`, `Stem`, `Notehead` | Visual note properties |
| `attributes` | `Attributes`, `Key`, `Time`, `Clef`, `Barline` | Measure attributes |
| `direction` | `Direction`, `DirectionType`, `Dynamics`, `Wedge`, `Metronome` | Musical directions |
| `notation` | `Notations`, `Articulations`, `Ornaments`, `Technical`, `Slur`, `Tied`, `Tuplet` | Note notations |
| `voice` | `Backup`, `Forward` | Voice/time management |
| `lyric` | `Lyric`, `Syllabic`, `TextElementData` | Lyrics |
| `measure` | `Measure`, `MusicDataElement` | Measure container |
| `part` | `Part`, `PartList`, `ScorePart` | Part definitions |
| `score` | `ScorePartwise`, `Work`, `Identification`, `Defaults` | Top-level score |

### Note Structure (Most Complex)

```rust
pub struct Note {
    pub content: NoteContent,      // Regular, Grace, or Cue
    pub voice: Option<Voice>,
    pub r#type: Option<NoteType>,  // quarter, eighth, etc.
    pub dots: Vec<Dot>,
    pub accidental: Option<Accidental>,
    pub time_modification: Option<TimeModification>,
    pub stem: Option<Stem>,
    pub staff: Option<StaffNumber>,
    pub beams: Vec<Beam>,
    pub notations: Vec<Notations>,
    pub lyrics: Vec<Lyric>,
    // ... more fields
}

pub enum NoteContent {
    Regular { full_note: FullNote, duration: PositiveDivisions, ties: Vec<Tie> },
    Grace { grace: Grace, full_note: FullNote, ties: Vec<Tie> },
    Cue { full_note: FullNote, duration: PositiveDivisions },
}

pub struct FullNote {
    pub chord: bool,
    pub content: PitchRestUnpitched,
}
```

---

## Phase 2 Guidance: MusicXML Emitter

### Approach

Create `src/musicxml/emit.rs` that walks the IR and produces MusicXML.

Suggested structure:

```
src/
├── ir/          # ✅ Done
└── musicxml/
    ├── mod.rs
    └── emit.rs  # IR → MusicXML
```

### Key Challenges

1. **`divisions` Calculation**
   * MusicXML uses `<divisions>` to define duration units
   * Must compute LCM of all durations in part to set divisions
   * Then scale all `<duration>` values accordingly

2. **Element Ordering**
   * MusicXML is order-sensitive (defined by XSD)
   * Children must appear in specific order
   * Example: In `<note>`, pitch comes before duration comes before type

3. **Optional vs Required**
   * Many IR fields are `Option<T>`
   * Only emit elements when `Some`
   * But some elements are required in certain contexts

4. **Attribute Groups**
   * `Position`, `PrintStyle`, etc. expand to multiple XML attributes
   * Need helper functions to emit these consistently

### Suggested Implementation Order

1. **Start with a minimal example:**

   ```rust
   fn emit_score(score: &ScorePartwise) -> String
   ```

   Target: Emit "Twinkle Twinkle" (simple melody, one part, basic attributes)

2. **Build up incrementally:**
   * Score structure (part-list, parts, measures)
   * Notes (pitch, duration, type)
   * Attributes (divisions, time, key, clef)
   * Then: dynamics, articulations, etc.

3. **Test with real software:**
   * MuseScore (free, good MusicXML support)
   * Validate output opens correctly

### Example Target Output

For this IR:

```rust
ScorePartwise {
    version: Some("4.0".to_string()),
    part_list: PartList { content: vec![
        PartListElement::ScorePart(ScorePart {
            id: "P1".to_string(),
            part_name: PartName { value: "Piano".to_string(), .. },
            ..
        })
    ]},
    parts: vec![Part {
        id: "P1".to_string(),
        measures: vec![Measure {
            number: "1".to_string(),
            content: vec![
                MusicDataElement::Attributes(Attributes {
                    divisions: Some(1),
                    keys: vec![Key { content: KeyContent::Traditional(TraditionalKey { fifths: 0, mode: Some(Mode::Major), .. }), .. }],
                    times: vec![Time { content: TimeContent::Measured { signatures: vec![TimeSignature { beats: "4".to_string(), beat_type: "4".to_string() }] }, .. }],
                    clefs: vec![Clef { sign: ClefSign::G, line: Some(2), .. }],
                    ..
                }),
                MusicDataElement::Note(Note { /* C4 quarter */ }),
                MusicDataElement::Note(Note { /* D4 quarter */ }),
                MusicDataElement::Note(Note { /* E4 quarter */ }),
                MusicDataElement::Note(Note { /* F4 quarter */ }),
            ],
            ..
        }],
    }],
    ..
}
```

Should emit:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>1</divisions>
        <key>
          <fifths>0</fifths>
          <mode>major</mode>
        </key>
        <time>
          <beats>4</beats>
          <beat-type>4</beat-type>
        </time>
        <clef>
          <sign>G</sign>
          <line>2</line>
        </clef>
      </attributes>
      <note>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <duration>1</duration>
        <type>quarter</type>
      </note>
      <!-- ... more notes ... -->
    </measure>
  </part>
</score-partwise>
```

### XML Writing Options

1. **String concatenation** — Simple but error-prone
2. **`quick-xml`** — Popular, fast XML library
3. **`xml-rs`** — Streaming XML writer
4. **Manual `Write` trait** — Good control, moderate complexity

Recommendation: Start with `quick-xml` for the `Writer` API, or even simple string building for the first pass.

---

## Related Projects

### verovioxide

* Rust bindings to Verovio

* Will consume MusicXML output for rendering
* Repository: `oxur/verovioxide`

### Future: MCP Server

* Music theory operations via Model Context Protocol

* Will use Fermata for notation interchange

---

## Reference Materials

### MusicXML Resources

* **Spec:** <https://www.w3.org/2021/06/musicxml40/>

* **XSD:** <https://github.com/w3c/musicxml> (schema/ directory)
* **Tutorial:** <https://www.w3.org/2021/06/musicxml40/tutorial/>

### Key MusicXML Concepts

* `score-partwise` vs `score-timewise` — We use partwise (organized by part, then measure)

* `divisions` — Duration unit; quarter note = `divisions` value
* `duration` vs `type` — `duration` is sounding time, `type` is notated symbol
* `tie` vs `tied` — Sound vs visual
* `<chord/>` — Empty element flag, not a container

---

## Open Questions (Low Priority)

These were identified during design but can be resolved as encountered:

| Question | Context | Recommendation |
|----------|---------|----------------|
| Unpitched notes | Percussion | Add when needed |
| Nested tuplets | Complex rhythms | Defer until encountered |
| score-timewise support | Rare format | Parse and convert to partwise |
| Auto-beaming | Computed vs explicit | Store explicit in IR |
| Chord symbols | Harmony | Structured in IR |

---

## Summary

**Fermata** is an S-expression DSL for music notation.

**Current state:** IR types are implemented. Ready to build the MusicXML emitter.

**Next step:** Create `src/musicxml/emit.rs` that converts IR → MusicXML, starting with simple scores and building up.

**Key principle:** The IR is a lossless 1:1 mapping with MusicXML. Every design choice preserves MusicXML semantics.

---

*Document version: 2026-01-31*
*Phase: 2 of 6 (MusicXML Emitter)*
