# Chunk 8: Multi-Voice and Multi-Staff

## Overview

This chunk covers the coordination of multiple voices within a single part and music spanning multiple staves. In MusicXML, the concept of a "time cursor" tracks the current position within a measure. The `<backup>` and `<forward>` elements move this cursor backward and forward, enabling multiple voices to coexist in a single part.

**Key Concept:** MusicXML uses a sequential stream model. Notes play in sequence, each advancing the time cursor by their duration. To write a second voice that starts at the beginning of a measure, you must first `<backup>` to the beginning. This is different from a grid-based model where each voice would be a separate column.

**Voice vs Staff:** A voice is a melodic/rhythmic line (soprano, alto, tenor, bass). A staff is a physical 5-line system. Piano music typically has 2 staves but 4 voices (one per SATB part). A fugue voice might cross between staves while remaining a single logical voice.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<voice>` | simple (xs:string) | — | — | — |
| `<staff>` | simple (positive-integer) | — | — | — |
| `<backup>` | complex | `<duration>` | `<footnote>`, `<level>` | — |
| `<forward>` | complex | `<duration>` | `<footnote>`, `<level>`, `<voice>`, `<staff>` | — |

## S-Expression Mappings

### voice

The voice element indicates which musical voice a note belongs to. Voices are typically numbered 1-4 within a staff, but the element content is actually a string.

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
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

**Mapping Rules:**
- `<voice>` content → `:voice` keyword with string value
- Voice is a string, not integer — allows non-numeric identifiers
- Optional on notes (defaults vary by application)

**Type Definition:**
```rust
pub type Voice = String;  // Typically "1", "2", etc. but can be any string
```

---

### staff

The staff element indicates which staff within a part the note or direction belongs to. Staff numbers start at 1.

**MusicXML:**
```xml
<!-- Right hand on staff 1 (treble) -->
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>1</duration>
  <voice>1</voice>
  <type>quarter</type>
  <staff>1</staff>
</note>

<!-- Left hand on staff 2 (bass) -->
<note>
  <pitch><step>C</step><octave>3</octave></pitch>
  <duration>1</duration>
  <voice>2</voice>
  <type>quarter</type>
  <staff>2</staff>
</note>
```

**S-expr:**
```lisp
;; Right hand
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :voice "1"
  :type :quarter
  :staff 1)

;; Left hand
(note
  :pitch (pitch :step :C :octave 3)
  :duration 1
  :voice "2"
  :type :quarter
  :staff 2)
```

**Mapping Rules:**
- `<staff>` content → `:staff` keyword with positive integer
- Staff numbering starts at 1 (top staff)
- Required for multi-staff parts; optional for single-staff

**Type Definition:**
```rust
pub type StaffNumber = u16;  // Positive integer, typically 1-4
```

---

### backup

The backup element moves the time cursor backward by the specified duration. This is essential for writing multiple voices that start at the same time position.

**MusicXML:**
```xml
<!-- After writing voice 1 notes... -->
<backup>
  <duration>4</duration>
</backup>
<!-- Now at the beginning again, write voice 2 notes -->
```

**S-expr:**
```lisp
(backup :duration 4)
```

**With editorial elements:**
```xml
<backup>
  <duration>4</duration>
  <footnote>Editorial backup</footnote>
  <level>1</level>
</backup>
```

```lisp
(backup
  :duration 4
  :footnote "Editorial backup"
  :level 1)
```

**Mapping Rules:**
- `<duration>` → `:duration` (required, positive integer in divisions)
- Backup does NOT include voice or staff — it's a measure-level operation
- Used to move between voices, not within a voice

**Type Definition:**
```rust
pub struct Backup {
    pub duration: PositiveDivisions,
    pub footnote: Option<FormattedText>,
    pub level: Option<Level>,
}
```

---

### forward

The forward element moves the time cursor forward by the specified duration. Unlike backup, forward can be associated with a specific voice and staff.

**MusicXML:**
```xml
<!-- Skip a beat of rest without writing an explicit rest note -->
<forward>
  <duration>1</duration>
  <voice>2</voice>
  <staff>1</staff>
</forward>
```

**S-expr:**
```lisp
(forward :duration 1 :voice "2" :staff 1)
```

**Simple forward (duration only):**
```lisp
(forward :duration 2)
```

**Mapping Rules:**
- `<duration>` → `:duration` (required, positive integer)
- `<voice>` → `:voice` (optional string)
- `<staff>` → `:staff` (optional positive integer)
- Forward is like an invisible rest — it occupies time but produces no notation

**Type Definition:**
```rust
pub struct Forward {
    pub duration: PositiveDivisions,
    pub voice: Option<Voice>,
    pub staff: Option<StaffNumber>,
    pub footnote: Option<FormattedText>,
    pub level: Option<Level>,
}
```

---

## Interrelationships

### backup/forward ↔ measure content

The `<backup>` and `<forward>` elements are siblings to `<note>`, `<direction>`, etc. within a measure. They appear in the linear stream of measure content:

```lisp
(measure :number "1"
  ;; Voice 1
  (note :pitch (pitch :step :E :octave 5) :duration 1 :voice "1" :type :quarter)
  (note :pitch (pitch :step :D :octave 5) :duration 1 :voice "1" :type :quarter)

  ;; Back to start for voice 2
  (backup :duration 2)

  ;; Voice 2
  (note :pitch (pitch :step :C :octave 4) :duration 2 :voice "2" :type :half))
```

### voice ↔ staff

- A **voice** is a melodic line that can span staves
- A **staff** is a physical location for notation
- One voice can appear on multiple staves (cross-staff notation)
- Multiple voices can share a staff (common in choral music)

### staff ↔ attributes

The number of staves is defined in `<attributes><staves>`:

```xml
<attributes>
  <staves>2</staves>
  <clef number="1"><sign>G</sign><line>2</line></clef>
  <clef number="2"><sign>F</sign><line>4</line></clef>
</attributes>
```

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Voice as string | `:voice "1"` | MusicXML allows non-numeric identifiers |
| Staff as integer | `:staff 1` | Always a positive integer in MusicXML |
| Backup/forward as forms | `(backup :duration 4)` | Consistent with other measure-level elements |
| Preserve cursor model | Keep backup/forward explicit | Lossless round-trip; higher-level syntax can abstract |
| Editorial groups | Flatten into keywords | `:footnote` and `:level` as optional keywords |

---

## Open Questions

- [ ] **Implicit voice/staff** — Should omitted `:voice` default to "1"? MusicXML behavior varies by exporting application.

- [ ] **Higher-level voice grouping** — Should Fermata syntax allow `(voice "1" ...)` wrapper? The IR preserves MusicXML's per-note voice assignment.

- [ ] **Cross-staff beaming** — How to handle beams that connect notes on different staves? Currently handled by `<staff>` on individual notes.

---

## Examples

### Example 1: Two Voices in One Staff (SATB Soprano + Alto)

**Musical meaning:** Quarter notes in parallel thirds — soprano on E5-D5-C5-D5, alto on C5-B4-A4-B4

**MusicXML:**
```xml
<measure number="1">
  <attributes>
    <divisions>1</divisions>
  </attributes>

  <!-- Voice 1 (Soprano) -->
  <note>
    <pitch><step>E</step><octave>5</octave></pitch>
    <duration>1</duration>
    <voice>1</voice>
    <type>quarter</type>
    <stem>up</stem>
  </note>
  <note>
    <pitch><step>D</step><octave>5</octave></pitch>
    <duration>1</duration>
    <voice>1</voice>
    <type>quarter</type>
    <stem>up</stem>
  </note>

  <backup>
    <duration>2</duration>
  </backup>

  <!-- Voice 2 (Alto) -->
  <note>
    <pitch><step>C</step><octave>5</octave></pitch>
    <duration>1</duration>
    <voice>2</voice>
    <type>quarter</type>
    <stem>down</stem>
  </note>
  <note>
    <pitch><step>B</step><octave>4</octave></pitch>
    <duration>1</duration>
    <voice>2</voice>
    <type>quarter</type>
    <stem>down</stem>
  </note>
</measure>
```

**S-expr:**
```lisp
(measure :number "1"
  (attributes
    :divisions 1)

  ;; Voice 1 (Soprano)
  (note
    :pitch (pitch :step :E :octave 5)
    :duration 1
    :voice "1"
    :type :quarter
    :stem :up)
  (note
    :pitch (pitch :step :D :octave 5)
    :duration 1
    :voice "1"
    :type :quarter
    :stem :up)

  (backup :duration 2)

  ;; Voice 2 (Alto)
  (note
    :pitch (pitch :step :C :octave 5)
    :duration 1
    :voice "2"
    :type :quarter
    :stem :down)
  (note
    :pitch (pitch :step :B :octave 4)
    :duration 1
    :voice "2"
    :type :quarter
    :stem :down))
```

---

### Example 2: Piano Grand Staff (Two Staves, Two Voices)

**Musical meaning:** Right hand plays melody on staff 1, left hand plays accompaniment on staff 2

**MusicXML:**
```xml
<part id="P1">
  <measure number="1">
    <attributes>
      <divisions>1</divisions>
      <staves>2</staves>
      <clef number="1">
        <sign>G</sign>
        <line>2</line>
      </clef>
      <clef number="2">
        <sign>F</sign>
        <line>4</line>
      </clef>
    </attributes>

    <!-- Right hand (staff 1) -->
    <note>
      <pitch><step>E</step><octave>5</octave></pitch>
      <duration>2</duration>
      <voice>1</voice>
      <type>half</type>
      <staff>1</staff>
    </note>

    <backup>
      <duration>2</duration>
    </backup>

    <!-- Left hand (staff 2) -->
    <note>
      <pitch><step>C</step><octave>3</octave></pitch>
      <duration>1</duration>
      <voice>2</voice>
      <type>quarter</type>
      <staff>2</staff>
    </note>
    <note>
      <pitch><step>G</step><octave>3</octave></pitch>
      <duration>1</duration>
      <voice>2</voice>
      <type>quarter</type>
      <staff>2</staff>
    </note>
  </measure>
</part>
```

**S-expr:**
```lisp
(part :id "P1"
  (measure :number "1"
    (attributes
      :divisions 1
      :staves 2
      :clefs ((clef :number 1 :sign :G :line 2)
              (clef :number 2 :sign :F :line 4)))

    ;; Right hand (staff 1)
    (note
      :pitch (pitch :step :E :octave 5)
      :duration 2
      :voice "1"
      :type :half
      :staff 1)

    (backup :duration 2)

    ;; Left hand (staff 2)
    (note
      :pitch (pitch :step :C :octave 3)
      :duration 1
      :voice "2"
      :type :quarter
      :staff 2)
    (note
      :pitch (pitch :step :G :octave 3)
      :duration 1
      :voice "2"
      :type :quarter
      :staff 2)))
```

---

### Example 3: Forward for Implicit Rest

**Musical meaning:** Voice 2 enters after one beat of implicit rest

**MusicXML:**
```xml
<measure number="1">
  <attributes>
    <divisions>1</divisions>
    <time>
      <beats>4</beats>
      <beat-type>4</beat-type>
    </time>
  </attributes>

  <!-- Voice 1 plays all four beats -->
  <note>
    <pitch><step>C</step><octave>4</octave></pitch>
    <duration>4</duration>
    <voice>1</voice>
    <type>whole</type>
  </note>

  <backup>
    <duration>4</duration>
  </backup>

  <!-- Voice 2 rests for one beat, then enters -->
  <forward>
    <duration>1</duration>
    <voice>2</voice>
  </forward>
  <note>
    <pitch><step>E</step><octave>4</octave></pitch>
    <duration>3</duration>
    <voice>2</voice>
    <type>half</type>
    <dot/>
  </note>
</measure>
```

**S-expr:**
```lisp
(measure :number "1"
  (attributes
    :divisions 1
    :time (time :beats 4 :beat-type 4))

  ;; Voice 1: whole note
  (note
    :pitch (pitch :step :C :octave 4)
    :duration 4
    :voice "1"
    :type :whole)

  (backup :duration 4)

  ;; Voice 2: skip one beat, then dotted half
  (forward :duration 1 :voice "2")
  (note
    :pitch (pitch :step :E :octave 4)
    :duration 3
    :voice "2"
    :type :half
    :dots ((dot))))
```

---

### Example 4: Cross-Staff Notation

**Musical meaning:** A chord where the bass note is written on the bass staff but the upper notes are on the treble staff, all part of voice 1

**MusicXML:**
```xml
<!-- Bass note on staff 2, but voice 1 -->
<note>
  <pitch><step>C</step><octave>3</octave></pitch>
  <duration>2</duration>
  <voice>1</voice>
  <type>half</type>
  <staff>2</staff>
</note>
<!-- Upper notes on staff 1, chord, voice 1 -->
<note>
  <chord/>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>2</duration>
  <voice>1</voice>
  <type>half</type>
  <staff>1</staff>
</note>
<note>
  <chord/>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>2</duration>
  <voice>1</voice>
  <type>half</type>
  <staff>1</staff>
</note>
```

**S-expr:**
```lisp
;; Bass note on staff 2
(note
  :pitch (pitch :step :C :octave 3)
  :duration 2
  :voice "1"
  :type :half
  :staff 2)

;; Upper notes on staff 1 (chord)
(note
  :chord t
  :pitch (pitch :step :G :octave 4)
  :duration 2
  :voice "1"
  :type :half
  :staff 1)
(note
  :chord t
  :pitch (pitch :step :C :octave 5)
  :duration 2
  :voice "1"
  :type :half
  :staff 1)
```

---

### Example 5: Four-Part Chorale (Four Voices on Two Staves)

**Musical meaning:** SATB on two staves — Soprano/Alto on staff 1, Tenor/Bass on staff 2

**MusicXML:**
```xml
<measure number="1">
  <attributes>
    <divisions>1</divisions>
    <staves>2</staves>
    <clef number="1"><sign>G</sign><line>2</line></clef>
    <clef number="2"><sign>F</sign><line>4</line></clef>
  </attributes>

  <!-- Soprano (voice 1, staff 1) -->
  <note>
    <pitch><step>G</step><octave>4</octave></pitch>
    <duration>1</duration><voice>1</voice><type>quarter</type><staff>1</staff>
  </note>
  <!-- Alto (voice 2, staff 1) -->
  <note>
    <chord/>
    <pitch><step>D</step><octave>4</octave></pitch>
    <duration>1</duration><voice>2</voice><type>quarter</type><staff>1</staff>
  </note>

  <backup><duration>1</duration></backup>

  <!-- Tenor (voice 3, staff 2) -->
  <note>
    <pitch><step>B</step><octave>3</octave></pitch>
    <duration>1</duration><voice>3</voice><type>quarter</type><staff>2</staff>
  </note>
  <!-- Bass (voice 4, staff 2) -->
  <note>
    <chord/>
    <pitch><step>G</step><octave>2</octave></pitch>
    <duration>1</duration><voice>4</voice><type>quarter</type><staff>2</staff>
  </note>
</measure>
```

**S-expr:**
```lisp
(measure :number "1"
  (attributes
    :divisions 1
    :staves 2
    :clefs ((clef :number 1 :sign :G :line 2)
            (clef :number 2 :sign :F :line 4)))

  ;; Soprano (voice 1, staff 1)
  (note :pitch (pitch :step :G :octave 4)
        :duration 1 :voice "1" :type :quarter :staff 1)
  ;; Alto (voice 2, staff 1, chord with soprano)
  (note :chord t
        :pitch (pitch :step :D :octave 4)
        :duration 1 :voice "2" :type :quarter :staff 1)

  (backup :duration 1)

  ;; Tenor (voice 3, staff 2)
  (note :pitch (pitch :step :B :octave 3)
        :duration 1 :voice "3" :type :quarter :staff 2)
  ;; Bass (voice 4, staff 2, chord with tenor)
  (note :chord t
        :pitch (pitch :step :G :octave 2)
        :duration 1 :voice "4" :type :quarter :staff 2))
```
