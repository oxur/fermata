# Chunk 7: Beaming & Stems

## Overview

This chunk covers beaming (connecting note flags) and stem direction, along with related print/layout attributes. Beaming is crucial for readability and rhythmic grouping.

**Key Concept:** MusicXML supports 8 levels of beaming (for up to 1024th notes). Each beam level is specified separately, using begin/continue/end/forward-hook/backward-hook values.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<beam>` | complex | beam-value | — | `number`, `repeater`, `fan`, color |
| `<stem>` | complex | stem-value | — | y-position, color |
| `<notehead>` | complex | notehead-value | — | `filled`, `parentheses`, font, color |
| Print attributes | attribute groups | — | — | Various |

## S-Expression Mappings

### beam

Specifies beam connections between notes.

**MusicXML:**
```xml
<!-- First note of beamed group -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1">begin</beam>
</note>

<!-- Middle note -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1">continue</beam>
</note>

<!-- Last note -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1">end</beam>
</note>

<!-- 16th notes (two beam levels) -->
<note>
  <pitch>...</pitch>
  <duration>1</duration>
  <type>16th</type>
  <beam number="1">begin</beam>
  <beam number="2">begin</beam>
</note>
```

**S-expr:**
```lisp
;; First eighth
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :begin)))

;; Middle eighth
(note
  :pitch (pitch :step :D :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :continue)))

;; Last eighth
(note
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :end)))

;; 16th note (two beams)
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :sixteenth
  :beams ((beam :number 1 :value :begin)
          (beam :number 2 :value :begin)))
```

**Beam Values:**
- `:begin` — Start of a beam
- `:continue` — Continuation
- `:end` — End of a beam
- `:forward-hook` — Partial beam pointing forward
- `:backward-hook` — Partial beam pointing backward

**Beam Levels:**
| Level | Note Value |
|-------|------------|
| 1 | Eighth notes |
| 2 | 16th notes |
| 3 | 32nd notes |
| 4 | 64th notes |
| 5 | 128th notes |
| 6 | 256th notes |
| 7 | 512th notes |
| 8 | 1024th notes |

**Special Attributes:**
- `fan` → `:fan` (`:rit`, `:accel`, `:none`) — Fanned beams for accel/rit
- `repeater` → `:repeater` (deprecated, was for tremolos)

**Type Definition:**
```rust
pub struct Beam {
    pub value: BeamValue,
    pub number: BeamLevel,  // 1-8
    pub repeater: Option<YesNo>,
    pub fan: Option<Fan>,
    pub color: Option<Color>,
}

pub enum BeamValue {
    Begin,
    Continue,
    End,
    ForwardHook,
    BackwardHook,
}

pub enum Fan {
    Accel,
    Rit,
    None,
}

pub type BeamLevel = u8;  // 1-8
```

---

### stem

Specifies stem direction.

**MusicXML:**
```xml
<stem>up</stem>
<stem>down</stem>
<stem>none</stem>
<stem>double</stem>

<!-- With position adjustment -->
<stem default-y="-35">down</stem>
```

**S-expr:**
```lisp
:stem :up
:stem :down
:stem :none
:stem :double

;; With position
:stem (stem :value :down :default-y -35)
```

**Stem Values:**
- `:up` — Stem points up
- `:down` — Stem points down
- `:none` — No stem (whole notes, etc.)
- `:double` — Double stem (rare)

**Mapping Rules:**
- Simple value → `:stem :value`
- With position attrs → `:stem (stem :value ... :default-y ...)`
- Position attrs control stem length/endpoint

**Type Definition:**
```rust
pub struct Stem {
    pub value: StemValue,
    pub default_y: Option<Tenths>,
    pub relative_y: Option<Tenths>,
    pub color: Option<Color>,
}

pub enum StemValue {
    Down,
    Up,
    Double,
    None,
}
```

---

### notehead

Specifies alternate notehead shapes.

**MusicXML:**
```xml
<notehead>x</notehead>
<notehead>diamond</notehead>
<notehead>slash</notehead>
<notehead filled="no">triangle</notehead>
<notehead parentheses="yes">normal</notehead>
```

**S-expr:**
```lisp
:notehead :x
:notehead :diamond
:notehead :slash
:notehead (notehead :value :triangle :filled :no)
:notehead (notehead :value :normal :parentheses :yes)
```

**Notehead Values:**
- `:slash`, `:triangle`, `:diamond`, `:square`, `:cross`, `:x`, `:circle-x`
- `:inverted-triangle`, `:arrow-down`, `:arrow-up`
- `:circled`, `:slashed`, `:back-slashed`
- `:normal`, `:cluster`, `:circle-dot`, `:left-triangle`
- `:rectangle`, `:none`, `:do`, `:re`, `:mi`, `:fa`, `:fa-up`, `:so`, `:la`, `:ti`
- `:other`

**Mapping Rules:**
- `filled` → `:filled` (`:yes`/`:no`, overrides default fill)
- `parentheses` → `:parentheses` (`:yes` for parenthesized)
- Font and color attrs also available

**Type Definition:**
```rust
pub struct Notehead {
    pub value: NoteheadValue,
    pub filled: Option<YesNo>,
    pub parentheses: Option<YesNo>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
    pub color: Option<Color>,
}

pub enum NoteheadValue {
    Slash, Triangle, Diamond, Square, Cross, X, CircleX,
    InvertedTriangle, ArrowDown, ArrowUp,
    Circled, Slashed, BackSlashed,
    Normal, Cluster, CircleDot, LeftTriangle,
    Rectangle, None,
    Do, Re, Mi, Fa, FaUp, So, La, Ti,  // Shape notes
    Other,
}
```

---

### Print/Layout Attribute Groups

Many elements share common print and layout attributes. These are defined as attribute groups in the XSD.

**print-style Group:**
```xml
<!-- Combines position, font, and color -->
<element default-x="10" default-y="20"
         font-family="Times" font-size="12"
         color="#000000"/>
```

**S-expr:**
```lisp
;; As flat attributes
:default-x 10 :default-y 20
:font-family "Times" :font-size 12
:color "#000000"

;; Could also group as :style
:style (style :x 10 :y 20 :font "Times" :size 12 :color "#000000")
```

**Attribute Groups:**

| Group | Attributes |
|-------|------------|
| `position` | `default-x`, `default-y`, `relative-x`, `relative-y` |
| `font` | `font-family`, `font-style`, `font-size`, `font-weight` |
| `color` | `color` |
| `print-style` | position + font + color |
| `print-style-align` | print-style + `halign`, `valign` |
| `placement` | `placement` (above/below) |
| `printout` | `print-dot`, `print-lyric`, `print-object`, `print-spacing` |
| `print-object` | `print-object` (yes/no) |

**Type Definition:**
```rust
pub struct Position {
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,
}

pub struct Font {
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
}

pub struct PrintStyle {
    pub position: Position,
    pub font: Font,
    pub color: Option<Color>,
}

pub struct PrintStyleAlign {
    pub print_style: PrintStyle,
    pub halign: Option<LeftCenterRight>,
    pub valign: Option<Valign>,
}

pub struct Printout {
    pub print_dot: Option<YesNo>,
    pub print_lyric: Option<YesNo>,
    pub print_object: Option<YesNo>,
    pub print_spacing: Option<YesNo>,
}
```

---

## Interrelationships

1. **beam ↔ note type:** Beam levels must match note values (8ths use level 1, 16ths use 1+2, etc.)

2. **beam begin ↔ end:** Every begin must have a matching end at the same level.

3. **beam ↔ voice:** Beams typically don't cross voices. MusicXML uses grace notes or different voices for overlapping beams.

4. **stem ↔ beam:** Stem direction affects beam position. Beamed notes typically share stem direction.

5. **notehead ↔ note type:** Some noteheads imply different rhythmic values (slash for rhythm section).

6. **print-object ↔ all:** Many elements can be hidden with `print-object="no"`.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Beam as list | `:beams ((beam ...) ...)` | Multiple beam levels per note |
| Stem simple form | `:stem :up` for simple cases | Common case is just direction |
| Notehead simple form | `:notehead :x` for simple cases | Common case is just shape |
| Print-style attrs | Flat in IR | Direct MusicXML mapping |
| Beam number | Required | Distinguish levels |

---

## Open Questions

- [ ] **Auto-beaming:** Should IR store explicit beams, or allow implicit beaming to be computed?

- [ ] **Print attribute grouping:** Keep flat or group into `:style` sub-form?

- [ ] **Cross-staff beaming:** How to represent beams that cross staves?

---

## Examples

### Example 1: Four Beamed Eighth Notes

**Musical meaning:** Four eighth notes connected by a single beam

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <stem>up</stem>
  <beam number="1">begin</beam>
</note>
<note>
  <pitch><step>D</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <stem>up</stem>
  <beam number="1">continue</beam>
</note>
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <stem>up</stem>
  <beam number="1">continue</beam>
</note>
<note>
  <pitch><step>F</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <stem>up</stem>
  <beam number="1">end</beam>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :eighth
  :stem :up
  :beams ((beam :number 1 :value :begin)))
(note
  :pitch (pitch :step :D :octave 4)
  :duration 1
  :type :eighth
  :stem :up
  :beams ((beam :number 1 :value :continue)))
(note
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :type :eighth
  :stem :up
  :beams ((beam :number 1 :value :continue)))
(note
  :pitch (pitch :step :F :octave 4)
  :duration 1
  :type :eighth
  :stem :up
  :beams ((beam :number 1 :value :end)))
```

---

### Example 2: 16th Notes (Two Beam Levels)

**Musical meaning:** Four 16th notes with primary and secondary beams

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <beam number="1">begin</beam>
  <beam number="2">begin</beam>
</note>
<note>
  <pitch><step>D</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <beam number="1">continue</beam>
  <beam number="2">continue</beam>
</note>
<note>
  <pitch><step>E</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <beam number="1">continue</beam>
  <beam number="2">continue</beam>
</note>
<note>
  <pitch><step>F</step><octave>5</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <beam number="1">end</beam>
  <beam number="2">end</beam>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :type :sixteenth
  :beams ((beam :number 1 :value :begin)
          (beam :number 2 :value :begin)))
(note
  :pitch (pitch :step :D :octave 5)
  :duration 1
  :type :sixteenth
  :beams ((beam :number 1 :value :continue)
          (beam :number 2 :value :continue)))
(note
  :pitch (pitch :step :E :octave 5)
  :duration 1
  :type :sixteenth
  :beams ((beam :number 1 :value :continue)
          (beam :number 2 :value :continue)))
(note
  :pitch (pitch :step :F :octave 5)
  :duration 1
  :type :sixteenth
  :beams ((beam :number 1 :value :end)
          (beam :number 2 :value :end)))
```

---

### Example 3: Partial Beams (Hooks)

**Musical meaning:** Dotted eighth + sixteenth pattern

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>3</duration>
  <type>eighth</type>
  <dot/>
  <beam number="1">begin</beam>
</note>
<note>
  <pitch><step>D</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>16th</type>
  <beam number="1">end</beam>
  <beam number="2">backward-hook</beam>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 3
  :type :eighth
  :dots ((dot))
  :beams ((beam :number 1 :value :begin)))
(note
  :pitch (pitch :step :D :octave 4)
  :duration 1
  :type :sixteenth
  :beams ((beam :number 1 :value :end)
          (beam :number 2 :value :backward-hook)))
```

---

### Example 4: X Noteheads (Percussion)

**Musical meaning:** Hi-hat or rhythmic hits

**MusicXML:**
```xml
<note>
  <unpitched>
    <display-step>G</display-step>
    <display-octave>5</display-octave>
  </unpitched>
  <duration>1</duration>
  <type>quarter</type>
  <notehead>x</notehead>
  <stem>up</stem>
</note>
```

**S-expr:**
```lisp
(note
  :unpitched (unpitched :display-step :G :display-octave 5)
  :duration 1
  :type :quarter
  :notehead :x
  :stem :up)
```

---

### Example 5: Fanned Beams (Accelerando)

**Musical meaning:** Gradually speed up

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1" fan="accel">begin</beam>
</note>
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1">continue</beam>
</note>
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1">continue</beam>
</note>
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>1</duration>
  <type>eighth</type>
  <beam number="1">end</beam>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :begin :fan :accel)))
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :continue)))
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :continue)))
(note
  :pitch (pitch :step :C :octave 4)
  :duration 1
  :type :eighth
  :beams ((beam :number 1 :value :end)))
```

---

### Example 6: Slash Noteheads (Rhythm Section)

**Musical meaning:** Improvised chording in rhythm

**MusicXML:**
```xml
<note>
  <pitch><step>B</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notehead>slash</notehead>
  <stem>up</stem>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :B :octave 4)
  :duration 4
  :type :quarter
  :notehead :slash
  :stem :up)
```
