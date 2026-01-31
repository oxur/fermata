# Chunk 2: Time & Rhythm

## Overview

This chunk covers elements that modify the rhythmic behavior of notes: tuplets, ties, and grace notes. These elements alter how durations are interpreted and connected.

**Key Concepts:**
- **Tuplets** change the ratio of actual-to-normal notes (e.g., 3 eighths in the time of 2)
- **Ties** connect notes of the same pitch across time
- **Grace notes** are ornamental notes without their own rhythmic duration

**Important Distinction:** MusicXML has both `<tie>` (sound/playback) and `<tied>` (notation/visual). The IR preserves both.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<time-modification>` | complex | `<actual-notes>`, `<normal-notes>` | `<normal-type>`, `<normal-dot>` | — |
| `<tuplet>` | complex | — | `<tuplet-actual>`, `<tuplet-normal>` | `type`, `number`, `bracket`, `show-number`, `show-type` |
| `<tie>` | complex | — | — | `type` (required), `time-only` |
| `<tied>` | complex | — | — | `type` (required), `number`, line-type attrs, position attrs, bezier attrs |
| `<grace>` | complex | — | — | `steal-time-previous`, `steal-time-following`, `make-time`, `slash` |

## S-Expression Mappings

### time-modification

Specifies the ratio for tuplets: how many actual notes fit in the space of normal notes.

**MusicXML:**
```xml
<!-- Triplet eighths (3 in the time of 2) -->
<time-modification>
  <actual-notes>3</actual-notes>
  <normal-notes>2</normal-notes>
  <normal-type>eighth</normal-type>
</time-modification>

<!-- Quintuplet sixteenths (5 in the time of 4) -->
<time-modification>
  <actual-notes>5</actual-notes>
  <normal-notes>4</normal-notes>
  <normal-type>16th</normal-type>
</time-modification>
```

**S-expr:**
```lisp
;; Triplet eighths
(time-modification
  :actual-notes 3
  :normal-notes 2
  :normal-type :eighth)

;; Quintuplet sixteenths
(time-modification
  :actual-notes 5
  :normal-notes 4
  :normal-type :sixteenth)
```

**Mapping Rules:**
- `<actual-notes>` → `:actual-notes` (integer)
- `<normal-notes>` → `:normal-notes` (integer)
- `<normal-type>` → `:normal-type` (note-type-value keyword)
- `<normal-dot>` → `:normal-dots` (count of dots)

**Type Definition:**
```rust
pub struct TimeModification {
    pub actual_notes: u32,
    pub normal_notes: u32,
    pub normal_type: Option<NoteTypeValue>,
    pub normal_dots: u32,  // Count of <normal-dot> elements
}
```

---

### tuplet

Controls the *display* of tuplet notation (brackets, numbers). Appears in `<notations>`.

**MusicXML:**
```xml
<!-- Start of triplet with bracket and number -->
<notations>
  <tuplet type="start" bracket="yes" show-number="actual">
    <tuplet-actual>
      <tuplet-number>3</tuplet-number>
      <tuplet-type>eighth</tuplet-type>
    </tuplet-actual>
    <tuplet-normal>
      <tuplet-number>2</tuplet-number>
      <tuplet-type>eighth</tuplet-type>
    </tuplet-normal>
  </tuplet>
</notations>

<!-- End of triplet -->
<notations>
  <tuplet type="stop"/>
</notations>
```

**S-expr:**
```lisp
;; Start of triplet
(tuplet
  :type :start
  :bracket :yes
  :show-number :actual
  :actual (tuplet-portion
            :number 3
            :type :eighth)
  :normal (tuplet-portion
            :number 2
            :type :eighth))

;; End of triplet
(tuplet :type :stop)
```

**Mapping Rules:**
- `type` attribute → `:type` (`:start` or `:stop`)
- `number` attribute → `:number` (for nested tuplets)
- `bracket` → `:bracket` (`:yes` or `:no`)
- `show-number` → `:show-number` (`:actual`, `:both`, `:none`)
- `show-type` → `:show-type` (`:actual`, `:both`, `:none`)
- `<tuplet-actual>` → `:actual` sub-form
- `<tuplet-normal>` → `:normal` sub-form

**Type Definition:**
```rust
pub struct Tuplet {
    pub r#type: StartStop,
    pub number: Option<NumberLevel>,  // 1-16, for nested tuplets
    pub bracket: Option<YesNo>,
    pub show_number: Option<ShowTuplet>,
    pub show_type: Option<ShowTuplet>,
    pub line_shape: Option<LineShape>,
    pub actual: Option<TupletPortion>,
    pub normal: Option<TupletPortion>,
    // position attributes...
}

pub struct TupletPortion {
    pub number: Option<u32>,
    pub r#type: Option<NoteTypeValue>,
    pub dots: u32,
}

pub enum ShowTuplet {
    Actual,
    Both,
    None,
}
```

---

### tie

Indicates a tie for *sound/playback*. Appears directly in `<note>`.

**MusicXML:**
```xml
<!-- First note of tie -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <tie type="start"/>
  <type>quarter</type>
</note>

<!-- Second note of tie -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <tie type="stop"/>
  <type>quarter</type>
</note>

<!-- Note that is both end of one tie and start of another -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <tie type="stop"/>
  <tie type="start"/>
  <type>quarter</type>
</note>
```

**S-expr:**
```lisp
;; Start of tie
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :ties ((tie :type :start))
  :type :quarter)

;; End of tie
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :ties ((tie :type :stop))
  :type :quarter)

;; Continue (stop + start)
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :ties ((tie :type :stop) (tie :type :start))
  :type :quarter)
```

**Mapping Rules:**
- Each `<tie>` element → `(tie ...)` form
- Multiple `<tie>` elements → list of tie forms
- `type` attribute → `:type` (`:start` or `:stop`)
- `time-only` attribute → `:time-only` (for repeat passages)

**Type Definition:**
```rust
pub struct Tie {
    pub r#type: StartStop,
    pub time_only: Option<TimeOnly>,
}
```

---

### tied

Visual representation of a tie. Appears in `<notations>`.

**MusicXML:**
```xml
<notations>
  <tied type="start" orientation="over" default-y="20"/>
</notations>

<notations>
  <tied type="stop"/>
</notations>

<!-- Let-ring (undamped strings) -->
<notations>
  <tied type="let-ring"/>
</notations>
```

**S-expr:**
```lisp
;; Start of tie (visual)
(tied
  :type :start
  :orientation :over
  :default-y 20)

;; End of tie
(tied :type :stop)

;; Let-ring
(tied :type :let-ring)
```

**Mapping Rules:**
- `type` attribute → `:type` (`:start`, `:stop`, `:continue`, `:let-ring`)
- `number` attribute → `:number` (for multiple ties on same note)
- `orientation` → `:orientation` (`:over` or `:under`)
- Line-type attrs (`line-type`, `dash-length`, `space-length`) → corresponding keywords
- Position attrs (`default-x/y`, `relative-x/y`) → corresponding keywords
- Bezier attrs (`bezier-x`, `bezier-y`, etc.) → corresponding keywords

**Type Definition:**
```rust
pub struct Tied {
    pub r#type: TiedType,
    pub number: Option<NumberLevel>,
    pub line_type: Option<LineType>,
    pub dash_length: Option<Tenths>,
    pub space_length: Option<Tenths>,
    pub orientation: Option<OverUnder>,
    // position attributes...
    // bezier attributes...
}

pub enum TiedType {
    Start,
    Stop,
    Continue,
    LetRing,
}
```

---

### grace

Marks a note as a grace note (no counted duration).

**MusicXML:**
```xml
<!-- Slashed grace note (acciaccatura) -->
<note>
  <grace slash="yes"/>
  <pitch>...</pitch>
  <type>eighth</type>
</note>

<!-- Unslashed grace note (appoggiatura) -->
<note>
  <grace/>
  <pitch>...</pitch>
  <type>eighth</type>
</note>

<!-- Grace note that steals time from following note -->
<note>
  <grace steal-time-following="33"/>
  <pitch>...</pitch>
  <type>eighth</type>
</note>
```

**S-expr:**
```lisp
;; Slashed grace note
(note
  :grace (grace :slash :yes)
  :pitch (pitch :step :D :octave 5)
  :type :eighth)

;; Unslashed grace note
(note
  :grace (grace)
  :pitch (pitch :step :D :octave 5)
  :type :eighth)

;; Grace note stealing time from following
(note
  :grace (grace :steal-time-following 33)
  :pitch (pitch :step :D :octave 5)
  :type :eighth)
```

**Mapping Rules:**
- `<grace/>` → `:grace (grace ...)` sub-form
- `slash` attribute → `:slash` (`:yes` for acciaccatura)
- `steal-time-previous` → `:steal-time-previous` (percentage)
- `steal-time-following` → `:steal-time-following` (percentage)
- `make-time` → `:make-time` (in divisions, adds to duration)

**Note:** Grace notes do NOT have a `<duration>` element. Their duration is determined by `steal-time-*` or `make-time` attributes, or by performance practice.

**Type Definition:**
```rust
pub struct Grace {
    pub steal_time_previous: Option<Percent>,
    pub steal_time_following: Option<Percent>,
    pub make_time: Option<Divisions>,
    pub slash: Option<YesNo>,
}
```

---

## Interrelationships

1. **time-modification ↔ tuplet**: Both are needed for tuplets:
   - `<time-modification>` (in `<note>`) → affects *sound/duration*
   - `<tuplet>` (in `<notations>`) → affects *display/brackets*
   - Every note in a tuplet has `<time-modification>`; only first/last have `<tuplet>`

2. **tie ↔ tied**: Both are needed for ties:
   - `<tie>` (in `<note>`) → affects *playback* (notes sound connected)
   - `<tied>` (in `<notations>`) → affects *display* (visual arc)
   - Usually both are present and match

3. **grace ↔ duration**: Grace notes have NO `<duration>` element. The note's rhythmic structure is:
   - Regular: `full-note + duration + tie*`
   - Grace: `grace + full-note + tie*` (no duration)

4. **tuplet nesting**: The `number` attribute (1-16) distinguishes nested tuplets.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| time-modification location | In `<note>`, preserved as child | Matches MusicXML; affects each note's duration |
| tuplet location | In `<notations>`, preserved | MusicXML structure; separate from sound |
| tie vs tied | Preserve both separately | Semantic distinction (sound vs. display) |
| grace representation | Sub-form `:grace (grace ...)` | Mirrors MusicXML structure; allows attributes |
| Multiple ties | List of tie forms | Note can have stop+start (tie continuation) |

---

## Open Questions

- [ ] **Tuplet bracket spanning:** How to associate tuplet start/stop across notes in IR? By `number` attribute alone?

- [ ] **Grace note duration normalization:** Should IR compute actual sounding duration from `steal-time-*` or leave that to playback?

- [ ] **Nested tuplets:** Rarely used but supported. Need to test with real-world examples.

---

## Examples

### Example 1: Simple Triplet (3 eighth notes in time of 2)

**Musical meaning:** Three evenly-spaced notes in one beat

**MusicXML:**
```xml
<!-- First note of triplet -->
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>2</duration>
  <voice>1</voice>
  <type>eighth</type>
  <time-modification>
    <actual-notes>3</actual-notes>
    <normal-notes>2</normal-notes>
    <normal-type>eighth</normal-type>
  </time-modification>
  <notations>
    <tuplet type="start" bracket="yes"/>
  </notations>
</note>

<!-- Second note -->
<note>
  <pitch><step>D</step><octave>4</octave></pitch>
  <duration>2</duration>
  <voice>1</voice>
  <type>eighth</type>
  <time-modification>
    <actual-notes>3</actual-notes>
    <normal-notes>2</normal-notes>
    <normal-type>eighth</normal-type>
  </time-modification>
</note>

<!-- Third note -->
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>2</duration>
  <voice>1</voice>
  <type>eighth</type>
  <time-modification>
    <actual-notes>3</actual-notes>
    <normal-notes>2</normal-notes>
    <normal-type>eighth</normal-type>
  </time-modification>
  <notations>
    <tuplet type="stop"/>
  </notations>
</note>
```

**S-expr:**
```lisp
;; First note
(note
  :pitch (pitch :step :C :octave 4)
  :duration 2
  :voice "1"
  :type :eighth
  :time-modification (time-modification
                       :actual-notes 3
                       :normal-notes 2
                       :normal-type :eighth)
  :notations ((notations
                (tuplet :type :start :bracket :yes))))

;; Second note
(note
  :pitch (pitch :step :D :octave 4)
  :duration 2
  :voice "1"
  :type :eighth
  :time-modification (time-modification
                       :actual-notes 3
                       :normal-notes 2
                       :normal-type :eighth))

;; Third note
(note
  :pitch (pitch :step :E :octave 4)
  :duration 2
  :voice "1"
  :type :eighth
  :time-modification (time-modification
                       :actual-notes 3
                       :normal-notes 2
                       :normal-type :eighth)
  :notations ((notations
                (tuplet :type :stop))))
```

---

### Example 2: Tied Notes Across Measures

**Musical meaning:** Quarter note tied to eighth note = 1.5 beats sustained

**MusicXML:**
```xml
<!-- First note (end of measure 1) -->
<note>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>4</duration>
  <tie type="start"/>
  <voice>1</voice>
  <type>quarter</type>
  <notations>
    <tied type="start"/>
  </notations>
</note>

<!-- Second note (start of measure 2) -->
<note>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>2</duration>
  <tie type="stop"/>
  <voice>1</voice>
  <type>eighth</type>
  <notations>
    <tied type="stop"/>
  </notations>
</note>
```

**S-expr:**
```lisp
;; First note
(note
  :pitch (pitch :step :G :octave 4)
  :duration 4
  :ties ((tie :type :start))
  :voice "1"
  :type :quarter
  :notations ((notations
                (tied :type :start))))

;; Second note
(note
  :pitch (pitch :step :G :octave 4)
  :duration 2
  :ties ((tie :type :stop))
  :voice "1"
  :type :eighth
  :notations ((notations
                (tied :type :stop))))
```

---

### Example 3: Slashed Grace Note (Acciaccatura)

**Musical meaning:** Quick ornamental note before the main note

**MusicXML:**
```xml
<!-- Grace note -->
<note>
  <grace slash="yes"/>
  <pitch><step>D</step><octave>5</octave></pitch>
  <voice>1</voice>
  <type>eighth</type>
  <notations>
    <slur type="start" number="1"/>
  </notations>
</note>

<!-- Main note -->
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>4</duration>
  <voice>1</voice>
  <type>quarter</type>
  <notations>
    <slur type="stop" number="1"/>
  </notations>
</note>
```

**S-expr:**
```lisp
;; Grace note
(note
  :grace (grace :slash :yes)
  :pitch (pitch :step :D :octave 5)
  :voice "1"
  :type :eighth
  :notations ((notations
                (slur :type :start :number 1))))

;; Main note
(note
  :pitch (pitch :step :C :octave 5)
  :duration 4
  :voice "1"
  :type :quarter
  :notations ((notations
                (slur :type :stop :number 1))))
```

---

### Example 4: Tie Chain (Three Notes Tied Together)

**Musical meaning:** Long sustained note across three note heads

**MusicXML:**
```xml
<!-- First note -->
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>4</duration>
  <tie type="start"/>
  <type>quarter</type>
  <notations><tied type="start"/></notations>
</note>

<!-- Middle note -->
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>4</duration>
  <tie type="stop"/>
  <tie type="start"/>
  <type>quarter</type>
  <notations>
    <tied type="stop"/>
    <tied type="start"/>
  </notations>
</note>

<!-- Last note -->
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>2</duration>
  <tie type="stop"/>
  <type>eighth</type>
  <notations><tied type="stop"/></notations>
</note>
```

**S-expr:**
```lisp
;; First note
(note
  :pitch (pitch :step :E :octave 4)
  :duration 4
  :ties ((tie :type :start))
  :type :quarter
  :notations ((notations (tied :type :start))))

;; Middle note
(note
  :pitch (pitch :step :E :octave 4)
  :duration 4
  :ties ((tie :type :stop) (tie :type :start))
  :type :quarter
  :notations ((notations
                (tied :type :stop)
                (tied :type :start))))

;; Last note
(note
  :pitch (pitch :step :E :octave 4)
  :duration 2
  :ties ((tie :type :stop))
  :type :eighth
  :notations ((notations (tied :type :stop))))
```

---

### Example 5: Quintuplet (5 in the time of 4)

**Musical meaning:** Five sixteenth notes in the space of four

**MusicXML (first and last notes shown):**
```xml
<!-- First note -->
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <time-modification>
    <actual-notes>5</actual-notes>
    <normal-notes>4</normal-notes>
    <normal-type>16th</normal-type>
  </time-modification>
  <notations>
    <tuplet type="start" show-number="actual"/>
  </notations>
</note>
<!-- ... notes 2-4 ... -->
<!-- Fifth note -->
<note>
  <pitch><step>G</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <time-modification>
    <actual-notes>5</actual-notes>
    <normal-notes>4</normal-notes>
    <normal-type>16th</normal-type>
  </time-modification>
  <notations>
    <tuplet type="stop"/>
  </notations>
</note>
```

**S-expr (first and last):**
```lisp
;; First note
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :type :sixteenth
  :time-modification (time-modification
                       :actual-notes 5
                       :normal-notes 4
                       :normal-type :sixteenth)
  :notations ((notations
                (tuplet :type :start :show-number :actual))))

;; Fifth note
(note
  :pitch (pitch :step :G :octave 5)
  :duration 1
  :type :sixteenth
  :time-modification (time-modification
                       :actual-notes 5
                       :normal-notes 4
                       :normal-type :sixteenth)
  :notations ((notations
                (tuplet :type :stop))))
```
