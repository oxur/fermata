# Chunk 1: Core Note

## Overview

The note is the fundamental building block of music notation. This chunk covers the essential elements needed to represent a single note or rest: pitch, duration, accidentals, dots, and chord membership.

In MusicXML, the `<note>` element is the most complex element in the schema, with 15+ optional children. At the IR level, we preserve this complexity to ensure lossless round-tripping.

**Key Distinction:** MusicXML separates *sounding duration* (`<duration>`) from *notated duration* (`<type>`). A dotted quarter note has `<type>quarter</type>` but a `<duration>` that's 1.5× the base quarter value. We preserve this distinction in the IR.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<note>` | complex | See content model | Many (15+) | `default-x`, `default-y`, `dynamics`, `end-dynamics`, `attack`, `release`, `pizzicato` |
| `<pitch>` | complex | `<step>`, `<octave>` | `<alter>` | — |
| `<step>` | simple (enum) | — | — | — |
| `<alter>` | simple (semitones) | — | — | — |
| `<octave>` | simple (0-9) | — | — | — |
| `<duration>` | simple (positive-divisions) | — | — | — |
| `<type>` | complex | note-type-value | — | `size` |
| `<rest>` | complex | — | `<display-step>`, `<display-octave>` | `measure` |
| `<chord>` | empty | — | — | — |
| `<dot>` | empty-placement | — | — | `placement`, print-style |
| `<accidental>` | complex | accidental-value | — | `cautionary`, `editorial`, `parentheses`, `bracket`, `size` |

## S-Expression Mappings

### note

The note element has a complex content model with three variants:
1. **Regular note:** full-note + duration + optional ties
2. **Grace note:** grace + full-note + optional ties (no duration)
3. **Cue note:** cue + full-note + duration

**MusicXML:**
```xml
<note default-x="80" dynamics="75">
  <pitch>
    <step>C</step>
    <alter>1</alter>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
  <accidental>sharp</accidental>
  <stem>up</stem>
</note>
```

**S-expr:**
```lisp
(note
  :default-x 80
  :dynamics 75
  :pitch (pitch :step :C :alter 1 :octave 4)
  :duration 1
  :voice 1
  :type :quarter
  :accidental (accidental :value :sharp)
  :stem :up)
```

**Mapping Rules:**
- All XML attributes → keyword pairs
- Complex children → nested forms
- Simple children with single value → keyword with value
- The `full-note` group (pitch/unpitched/rest + chord) is flattened — include whichever applies

**Type Definition (for Rust IR):**
```rust
pub struct Note {
    // Position attributes
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,

    // Playback attributes
    pub dynamics: Option<NonNegativeDecimal>,
    pub end_dynamics: Option<NonNegativeDecimal>,
    pub attack: Option<Divisions>,
    pub release: Option<Divisions>,
    pub pizzicato: Option<bool>,

    // Content — one of these three variants
    pub content: NoteContent,

    // Common elements (after the variant-specific content)
    pub instrument: Vec<Instrument>,
    pub voice: Option<String>,
    pub r#type: Option<NoteType>,
    pub dots: Vec<Dot>,
    pub accidental: Option<Accidental>,
    pub time_modification: Option<TimeModification>,
    pub stem: Option<Stem>,
    pub notehead: Option<Notehead>,
    pub notehead_text: Option<NoteheadText>,
    pub staff: Option<StaffNumber>,
    pub beams: Vec<Beam>,
    pub notations: Vec<Notations>,
    pub lyrics: Vec<Lyric>,
    pub play: Option<Play>,
    pub listen: Option<Listen>,
}

pub enum NoteContent {
    /// Regular note: pitch/unpitched/rest + duration + optional ties
    Regular {
        full_note: FullNote,
        duration: PositiveDivisions,
        ties: Vec<Tie>,
    },
    /// Grace note: no duration
    Grace {
        grace: Grace,
        full_note: FullNote,
        ties: Vec<Tie>,
    },
    /// Cue note: smaller size, no playback
    Cue {
        cue: bool,
        full_note: FullNote,
        duration: PositiveDivisions,
    },
    /// Grace-cue note
    GraceCue {
        grace: Grace,
        cue: bool,
        full_note: FullNote,
    },
}

pub struct FullNote {
    pub chord: bool,
    pub pitch_rest: PitchRest,
}

pub enum PitchRest {
    Pitch(Pitch),
    Unpitched(Unpitched),
    Rest(Rest),
}
```

---

### pitch

**MusicXML:**
```xml
<pitch>
  <step>F</step>
  <alter>-1</alter>
  <octave>5</octave>
</pitch>
```

**S-expr:**
```lisp
(pitch :step :F :alter -1 :octave 5)
```

**Short form (for convenience at higher Fermata levels, not in IR):**
```lisp
;; NOT in IR — this is higher-level Fermata syntax
fb5  ;; F-flat in octave 5
```

**Mapping Rules:**
- `<step>` → `:step` keyword with value `:A` through `:G`
- `<alter>` → `:alter` keyword with semitone value (can be decimal for microtones)
- `<octave>` → `:octave` keyword with integer 0-9

**Type Definition:**
```rust
pub struct Pitch {
    pub step: Step,
    pub alter: Option<Semitones>,  // Can be fractional for microtones
    pub octave: Octave,
}

pub enum Step {
    A, B, C, D, E, F, G,
}

pub type Semitones = f64;  // Allows microtones: -1.5, 0.5, etc.
pub type Octave = u8;      // 0-9
```

---

### step

**MusicXML:**
```xml
<step>C</step>
```

**S-expr:**
```lisp
:step :C
```

**Values:** `:A`, `:B`, `:C`, `:D`, `:E`, `:F`, `:G`

---

### alter

**MusicXML:**
```xml
<alter>1</alter>      <!-- sharp -->
<alter>-1</alter>     <!-- flat -->
<alter>0</alter>      <!-- natural -->
<alter>0.5</alter>    <!-- quarter-tone sharp (microtone) -->
```

**S-expr:**
```lisp
:alter 1     ; sharp
:alter -1    ; flat
:alter 0     ; natural (or omit)
:alter 0.5   ; quarter-tone sharp
```

**Mapping Rules:**
- Value is in semitones (positive = sharp direction, negative = flat direction)
- Decimal values allowed for microtones
- If absent, assume 0 (natural)

---

### octave

**MusicXML:**
```xml
<octave>4</octave>
```

**S-expr:**
```lisp
:octave 4
```

**Values:** 0-9 where octave 4 contains middle C (C4 = 261.63 Hz in A440)

---

### duration

Represents the *sounding* duration in divisions of a quarter note.

**MusicXML:**
```xml
<duration>1</duration>      <!-- one division (depends on divisions setting) -->
<duration>24</duration>     <!-- if divisions=24, this is a quarter note -->
<duration>36</duration>     <!-- if divisions=24, this is a dotted quarter -->
```

**S-expr:**
```lisp
:duration 1
:duration 24
:duration 36
```

**Mapping Rules:**
- Value is in `divisions` units (set by `<divisions>` in `<attributes>`)
- Prefer integer values to avoid roundoff
- The IR may normalize to a fixed division (e.g., 960 per quarter) during parsing

**Type Definition:**
```rust
pub type Divisions = i64;  // Signed to allow negative for backup
pub type PositiveDivisions = u64;
```

---

### type (note-type)

Represents the *notated* duration (what you see on the page).

**MusicXML:**
```xml
<type>quarter</type>
<type size="cue">eighth</type>
```

**S-expr:**
```lisp
:type :quarter
:type (note-type :value :eighth :size :cue)
```

**When no size attribute needed:**
```lisp
:type :quarter
```

**When size attribute present:**
```lisp
:type (note-type :value :quarter :size :cue)
```

**Values:**
- `:1024th`, `:512th`, `:256th`, `:128th`, `:64th`, `:32nd`, `:16th`, `:eighth`, `:quarter`, `:half`, `:whole`, `:breve`, `:long`, `:maxima`

**Duration keyword aliases (for ergonomics):**
| Short | Long | British | MusicXML |
|-------|------|---------|----------|
| `:w` | `:whole` | `:semibreve` | `whole` |
| `:h` | `:half` | `:minim` | `half` |
| `:q` | `:quarter` | `:crotchet` | `quarter` |
| `:8` | `:eighth` | `:quaver` | `eighth` |
| `:16` | `:sixteenth` | `:semiquaver` | `16th` |
| `:32` | `:thirty-second` | `:demisemiquaver` | `32nd` |

**Type Definition:**
```rust
pub struct NoteType {
    pub value: NoteTypeValue,
    pub size: Option<SymbolSize>,
}

pub enum NoteTypeValue {
    N1024th, N512th, N256th, N128th, N64th, N32nd, N16th,
    Eighth, Quarter, Half, Whole, Breve, Long, Maxima,
}

pub enum SymbolSize {
    Full,
    Cue,
    GraceCue,
    Large,
}
```

---

### rest

**MusicXML:**
```xml
<!-- Simple rest -->
<rest/>

<!-- Whole-measure rest -->
<rest measure="yes"/>

<!-- Rest with specific staff position -->
<rest>
  <display-step>B</display-step>
  <display-octave>4</display-octave>
</rest>
```

**S-expr:**
```lisp
;; Simple rest
(rest)

;; Whole-measure rest
(rest :measure :yes)

;; Rest with display position
(rest :display-step :B :display-octave 4)
```

**Mapping Rules:**
- Empty `<rest/>` → `(rest)`
- `measure="yes"` → `:measure :yes`
- Display position → `:display-step` and `:display-octave`

**Type Definition:**
```rust
pub struct Rest {
    pub measure: Option<YesNo>,
    pub display_step: Option<Step>,
    pub display_octave: Option<Octave>,
}
```

---

### chord

Indicates that this note is an additional chord tone sounding with the previous note.

**MusicXML:**
```xml
<!-- First note of chord -->
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
</note>
<!-- Second note of chord (has <chord/>) -->
<note>
  <chord/>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
</note>
<!-- Third note of chord -->
<note>
  <chord/>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>quarter</type>
</note>
```

**S-expr:**
```lisp
;; First note (no :chord flag)
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :quarter)

;; Second note (has :chord flag)
(note
  :chord t
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :type :quarter)

;; Third note
(note
  :chord t
  :pitch (pitch :step :G :octave 4)
  :duration 1
  :type :quarter)
```

**Mapping Rules:**
- `<chord/>` → `:chord t` (boolean true)
- Absence of `<chord/>` → omit `:chord` or `:chord nil`
- Duration of chord notes should equal the first note's duration

**Note:** Higher-level Fermata syntax may use explicit chord grouping:
```lisp
;; NOT IR — higher-level syntax
(chord :quarter c4 e4 g4)
```

---

### dot

Adds durational augmentation (50% per dot).

**MusicXML:**
```xml
<!-- Single dot -->
<dot/>

<!-- Double dot (two elements) -->
<dot/>
<dot/>

<!-- Dot with placement -->
<dot placement="above"/>
```

**S-expr:**
```lisp
;; Single dot
:dots ((dot))

;; Double dot
:dots ((dot) (dot))

;; Dot with placement
:dots ((dot :placement :above))
```

**Mapping Rules:**
- Multiple dots → multiple `(dot)` forms in a list
- Placement attribute → `:placement` keyword

**Type Definition:**
```rust
pub struct Dot {
    pub placement: Option<AboveBelow>,
    // print-style attributes...
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    // etc.
}
```

---

### accidental

The displayed accidental (independent of pitch's `<alter>`).

**MusicXML:**
```xml
<accidental>sharp</accidental>
<accidental cautionary="yes" parentheses="yes">natural</accidental>
<accidental editorial="yes" bracket="yes">flat</accidental>
```

**S-expr:**
```lisp
(accidental :value :sharp)
(accidental :value :natural :cautionary :yes :parentheses :yes)
(accidental :value :flat :editorial :yes :bracket :yes)
```

**Values:**
- Basic: `:sharp`, `:natural`, `:flat`, `:double-sharp`, `:double-flat`
- Quarter-tones: `:quarter-flat`, `:quarter-sharp`, `:three-quarters-flat`, `:three-quarters-sharp`
- Historical: `:sharp-sharp`, `:flat-flat`, `:natural-sharp`, `:natural-flat`
- And many more for extended/microtonal systems

**Mapping Rules:**
- Accidental value → `:value` keyword
- Boolean attributes → `:cautionary`, `:editorial`, `:parentheses`, `:bracket`
- Size → `:size` (`:full`, `:cue`, `:large`)

**Type Definition:**
```rust
pub struct Accidental {
    pub value: AccidentalValue,
    pub cautionary: Option<YesNo>,
    pub editorial: Option<YesNo>,
    pub parentheses: Option<YesNo>,
    pub bracket: Option<YesNo>,
    pub size: Option<SymbolSize>,
    // print-style attributes...
}

pub enum AccidentalValue {
    Sharp, Natural, Flat, DoubleSharp, DoubleFlat,
    QuarterFlat, QuarterSharp, ThreeQuartersFlat, ThreeQuartersSharp,
    // ... many more
}
```

---

## Interrelationships

1. **pitch ↔ accidental**: The `<alter>` in `<pitch>` indicates the *sounding* pitch; `<accidental>` indicates what's *displayed*. They usually match, but cautionary/courtesy accidentals may show an accidental when `<alter>` is 0.

2. **duration ↔ type ↔ dot**: These three work together:
   - `<type>` = base note value (quarter, eighth, etc.)
   - `<dot>` = augmentation dots
   - `<duration>` = actual sounding duration in divisions
   - Example: dotted quarter with divisions=24: `<type>quarter</type><dot/><duration>36</duration>`

3. **chord**: The `<chord/>` element creates implicit grouping with the *previous* note. The IR preserves this; higher-level syntax may group explicitly.

4. **rest ↔ pitch**: A note contains either `<pitch>`, `<unpitched>`, or `<rest>` — they're mutually exclusive.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Pitch representation | Structured `(pitch :step :C :alter 0 :octave 4)` | Matches MusicXML structure; allows microtones |
| Duration/type distinction | Preserve both as separate fields | MusicXML semantics; needed for playback vs. display |
| Chord representation | Boolean `:chord t` flag | Matches MusicXML; higher-level syntax can group |
| Step values | Keywords `:A` through `:G` | Readable, matches enum nature |
| Alter values | Numeric (semitones) | Supports microtones; -1 = flat, +1 = sharp |
| Dot representation | List of `(dot)` forms | Preserves count and individual attributes |
| Accidental attributes | All preserved | Needed for cautionary/editorial marks |

---

## Open Questions

- [ ] **Unpitched notes** — How to represent `<unpitched>` for percussion? Need `<display-step>` and `<display-octave>`. Covered in Chunk 10?

- [ ] **Note ID** — MusicXML supports `id` attribute for all elements. Should we include `:id` on every form or only when present?

- [ ] **Print-style attributes** — Many elements have `default-x`, `default-y`, `font-*`, `color`. Should these be grouped into a `:style` sub-form or kept flat?

---

## Examples

### Example 1: Simple Quarter Note C4

**Musical meaning:** Middle C, quarter note duration

**MusicXML:**
```xml
<note>
  <pitch>
    <step>C</step>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :voice "1"
  :type :quarter)
```

---

### Example 2: Dotted Eighth Note F#5

**Musical meaning:** F-sharp in octave 5, dotted eighth note

**MusicXML:**
```xml
<note>
  <pitch>
    <step>F</step>
    <alter>1</alter>
    <octave>5</octave>
  </pitch>
  <duration>3</duration>
  <voice>1</voice>
  <type>eighth</type>
  <dot/>
  <accidental>sharp</accidental>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :F :alter 1 :octave 5)
  :duration 3
  :voice "1"
  :type :eighth
  :dots ((dot))
  :accidental (accidental :value :sharp))
```

---

### Example 3: C Major Chord (C4-E4-G4)

**Musical meaning:** Three-note chord, quarter note duration

**MusicXML:**
```xml
<note>
  <pitch>
    <step>C</step>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
</note>
<note>
  <chord/>
  <pitch>
    <step>E</step>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
</note>
<note>
  <chord/>
  <pitch>
    <step>G</step>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :voice "1"
  :type :quarter)
(note
  :chord t
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :voice "1"
  :type :quarter)
(note
  :chord t
  :pitch (pitch :step :G :octave 4)
  :duration 1
  :voice "1"
  :type :quarter)
```

---

### Example 4: Quarter Rest

**Musical meaning:** One beat of silence

**MusicXML:**
```xml
<note>
  <rest/>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
</note>
```

**S-expr:**
```lisp
(note
  :rest (rest)
  :duration 1
  :voice "1"
  :type :quarter)
```

---

### Example 5: Whole-Measure Rest

**Musical meaning:** Rest for entire measure (value adapts to time signature)

**MusicXML:**
```xml
<note>
  <rest measure="yes"/>
  <duration>4</duration>
  <voice>1</voice>
</note>
```

**S-expr:**
```lisp
(note
  :rest (rest :measure :yes)
  :duration 4
  :voice "1")
```

Note: No `<type>` element for whole-measure rests.

---

### Example 6: Note with Cautionary Accidental

**Musical meaning:** B-natural after a B-flat, with parenthesized reminder

**MusicXML:**
```xml
<note>
  <pitch>
    <step>B</step>
    <alter>0</alter>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
  <accidental cautionary="yes" parentheses="yes">natural</accidental>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :B :alter 0 :octave 4)
  :duration 1
  :voice "1"
  :type :quarter
  :accidental (accidental
                :value :natural
                :cautionary :yes
                :parentheses :yes))
```
