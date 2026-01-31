# Chunk 5: Directions

## Overview

Directions are musical indications that are not attached to specific notes: dynamics, tempo markings, text expressions, rehearsal marks, and navigation symbols. They float in the measure and affect playback or provide performance instructions.

**Key Concept:** In MusicXML, `<direction>` is a wrapper that contains one or more `<direction-type>` elements. This allows combining multiple related markings (e.g., "rit." text followed by a dashed line).

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<direction>` | complex | `<direction-type>+` | `<offset>`, `<sound>`, etc. | `placement`, `directive` |
| `<direction-type>` | complex | (choice of types) | — | — |
| `<dynamics>` | complex | dynamic elements | — | print-style, placement |
| `<wedge>` | complex | — | — | `type`, `number`, `spread`, etc. |
| `<words>` | complex | text content | — | print-style, placement, many text attrs |
| `<metronome>` | complex | beat-unit/per-minute | — | print-style, parentheses |
| `<rehearsal>` | complex | text content | — | print-style, enclosure |
| `<segno>` | complex | — | — | print-style |
| `<coda>` | complex | — | — | print-style |
| `<pedal>` | complex | — | — | `type`, `line`, `sign` |
| `<octave-shift>` | complex | — | — | `type`, `number`, `size` |
| `<bracket>` | complex | — | — | `type`, `number`, `line-end` |
| `<dashes>` | complex | — | — | `type`, `number` |

## S-Expression Mappings

### direction

Container for direction content.

**MusicXML:**
```xml
<direction placement="above">
  <direction-type>
    <dynamics><f/></dynamics>
  </direction-type>
  <sound dynamics="90"/>
</direction>
```

**S-expr:**
```lisp
(direction :placement :above
  (direction-type
    (dynamics (f)))
  (sound :dynamics 90))
```

**Mapping Rules:**
- `placement` attribute → `:placement` (`:above` or `:below`)
- `directive` attribute → `:directive` (`:yes` if attached to note)
- `<direction-type>` → `(direction-type ...)` forms
- `<offset>` → `:offset` (in divisions, for precise positioning)
- `<sound>` → `(sound ...)` for playback effects
- `<voice>` → `:voice` (when direction applies to specific voice)
- `<staff>` → `:staff` (when direction applies to specific staff)

**Type Definition:**
```rust
pub struct Direction {
    pub placement: Option<AboveBelow>,
    pub directive: Option<YesNo>,
    pub direction_types: Vec<DirectionType>,
    pub offset: Option<Offset>,
    pub editorial_voice_direction: EditorialVoiceDirection,
    pub staff: Option<StaffNumber>,
    pub sound: Option<Sound>,
    pub listening: Option<Listening>,
}
```

---

### direction-type

The actual content of a direction. Many types available.

**MusicXML:**
```xml
<direction-type>
  <dynamics><ff/></dynamics>
</direction-type>

<direction-type>
  <words font-style="italic">dolce</words>
</direction-type>

<direction-type>
  <wedge type="crescendo"/>
</direction-type>
```

**S-expr:**
```lisp
(direction-type
  (dynamics (ff)))

(direction-type
  (words :font-style :italic "dolce"))

(direction-type
  (wedge :type :crescendo))
```

**Type Definition:**
```rust
pub enum DirectionTypeContent {
    Rehearsal(Vec<FormattedTextId>),
    Segno(Vec<Segno>),
    Coda(Vec<Coda>),
    WordsOrSymbol(Vec<WordsOrSymbol>),
    Wedge(Wedge),
    Dynamics(Vec<Dynamics>),
    Dashes(Dashes),
    Bracket(Bracket),
    Pedal(Pedal),
    Metronome(Metronome),
    OctaveShift(OctaveShift),
    HarpPedals(HarpPedals),
    Damp(Damp),
    DampAll(DampAll),
    Eyeglasses(Eyeglasses),
    StringMute(StringMute),
    Scordatura(Scordatura),
    Image(Image),
    PrincipalVoice(PrincipalVoice),
    Percussion(Vec<Percussion>),
    AccordionRegistration(AccordionRegistration),
    StaffDivide(StaffDivide),
    OtherDirection(OtherDirection),
}
```

---

### dynamics

Dynamic markings (p, f, mf, etc.).

**MusicXML:**
```xml
<dynamics>
  <p/>
</dynamics>

<dynamics>
  <ff/>
</dynamics>

<dynamics>
  <mp/>
</dynamics>

<dynamics>
  <sfz/>
</dynamics>

<!-- Combined dynamics -->
<dynamics>
  <f/>
  <p/>
</dynamics>

<!-- Custom dynamics -->
<dynamics>
  <other-dynamics>ffff</other-dynamics>
</dynamics>
```

**S-expr:**
```lisp
;; Simple dynamics
(dynamics (p))
(dynamics (ff))
(dynamics (mp))
(dynamics (sfz))

;; Combined (fp = forte-piano)
(dynamics (f) (p))

;; Custom
(dynamics (other-dynamics "ffff"))
```

**Dynamic Elements:**
- Standard: `(p)`, `(pp)`, `(ppp)`, `(pppp)`, `(ppppp)`, `(pppppp)`
- Standard: `(f)`, `(ff)`, `(fff)`, `(ffff)`, `(fffff)`, `(ffffff)`
- Medium: `(mp)`, `(mf)`
- Accented: `(sf)`, `(sfp)`, `(sfpp)`, `(fp)`, `(rf)`, `(rfz)`, `(sfz)`, `(sffz)`, `(fz)`, `(sfzp)`, `(pf)`
- Niente: `(n)`
- Custom: `(other-dynamics "text")`

**Mapping Rules:**
- Each dynamic level → empty element form
- Multiple dynamics combined → sequence of forms
- Custom text → `(other-dynamics "text")`
- Print-style attrs on container

**Type Definition:**
```rust
pub struct Dynamics {
    pub content: Vec<DynamicElement>,
    // print-style-align, placement, text-decoration, enclosure...
}

pub enum DynamicElement {
    P, PP, PPP, PPPP, PPPPP, PPPPPP,
    F, FF, FFF, FFFF, FFFFF, FFFFFF,
    MP, MF,
    SF, SFP, SFPP, FP, RF, RFZ, SFZ, SFFZ, FZ, N, PF, SFZP,
    OtherDynamics(String),
}
```

---

### wedge

Crescendo/decrescendo hairpins.

**MusicXML:**
```xml
<!-- Start of crescendo -->
<direction>
  <direction-type>
    <wedge type="crescendo" number="1"/>
  </direction-type>
</direction>

<!-- End of crescendo -->
<direction>
  <direction-type>
    <wedge type="stop" number="1"/>
  </direction-type>
</direction>

<!-- Decrescendo -->
<direction>
  <direction-type>
    <wedge type="diminuendo"/>
  </direction-type>
</direction>
```

**S-expr:**
```lisp
;; Start of crescendo
(direction
  (direction-type
    (wedge :type :crescendo :number 1)))

;; End
(direction
  (direction-type
    (wedge :type :stop :number 1)))

;; Decrescendo
(direction
  (direction-type
    (wedge :type :diminuendo)))
```

**Mapping Rules:**
- `type` → `:type` (`:crescendo`, `:diminuendo`, `:stop`, `:continue`)
- `number` → `:number` (for overlapping wedges)
- `spread` → `:spread` (opening width in tenths)
- `niente` → `:niente` (`:yes` for "al niente")
- Line-type attrs for dashed wedges

**Type Definition:**
```rust
pub struct Wedge {
    pub r#type: WedgeType,
    pub number: Option<NumberLevel>,
    pub spread: Option<Tenths>,
    pub niente: Option<YesNo>,
    // line-type, dashed-formatting, position, color...
}

pub enum WedgeType {
    Crescendo,
    Diminuendo,
    Stop,
    Continue,
}
```

---

### words

Text directions (tempo words, expressions, etc.).

**MusicXML:**
```xml
<direction-type>
  <words font-style="italic" font-size="12">dolce</words>
</direction-type>

<direction-type>
  <words font-weight="bold">Allegro</words>
</direction-type>

<direction-type>
  <words xml:lang="it">con fuoco</words>
</direction-type>
```

**S-expr:**
```lisp
(words :font-style :italic :font-size 12 "dolce")

(words :font-weight :bold "Allegro")

(words :lang "it" "con fuoco")
```

**Mapping Rules:**
- Text content → final argument or `:text`
- Font attrs → `:font-style`, `:font-weight`, `:font-size`, `:font-family`
- Position attrs → `:default-x`, `:default-y`, etc.
- `xml:lang` → `:lang`
- `justify` → `:justify` (`:left`, `:center`, `:right`)
- `enclosure` → `:enclosure` (`:rectangle`, `:oval`, `:none`, etc.)

**Type Definition:**
```rust
pub struct Words {
    pub text: String,
    pub lang: Option<String>,
    pub justify: Option<LeftCenterRight>,
    pub enclosure: Option<EnclosureShape>,
    // print-style-align, text-direction, text-decoration...
}
```

---

### metronome

Tempo marking with beat unit and rate.

**MusicXML:**
```xml
<!-- Quarter = 120 -->
<direction-type>
  <metronome>
    <beat-unit>quarter</beat-unit>
    <per-minute>120</per-minute>
  </metronome>
</direction-type>

<!-- Dotted quarter = 80 -->
<direction-type>
  <metronome>
    <beat-unit>quarter</beat-unit>
    <beat-unit-dot/>
    <per-minute>80</per-minute>
  </metronome>
</direction-type>

<!-- Quarter = Half (metric modulation) -->
<direction-type>
  <metronome>
    <beat-unit>quarter</beat-unit>
    <beat-unit>half</beat-unit>
  </metronome>
</direction-type>

<!-- With parentheses -->
<direction-type>
  <metronome parentheses="yes">
    <beat-unit>quarter</beat-unit>
    <per-minute>120</per-minute>
  </metronome>
</direction-type>
```

**S-expr:**
```lisp
;; Quarter = 120
(metronome
  :beat-unit :quarter
  :per-minute 120)

;; Dotted quarter = 80
(metronome
  :beat-unit :quarter
  :beat-unit-dot t
  :per-minute 80)

;; Metric modulation
(metronome
  :beat-unit :quarter
  :beat-unit-2 :half)

;; With parentheses
(metronome :parentheses :yes
  :beat-unit :quarter
  :per-minute 120)
```

**Mapping Rules:**
- `<beat-unit>` → `:beat-unit` (note-type-value)
- `<beat-unit-dot>` → `:beat-unit-dot` (boolean or count)
- `<per-minute>` → `:per-minute` (can be string for "ca. 120")
- Second beat-unit → `:beat-unit-2` (for metric modulation)
- `parentheses` attr → `:parentheses`

**Type Definition:**
```rust
pub struct Metronome {
    pub parentheses: Option<YesNo>,
    pub content: MetronomeContent,
    // print-style-align...
}

pub enum MetronomeContent {
    /// beat-unit = per-minute
    PerMinute {
        beat_unit: NoteTypeValue,
        beat_unit_dots: u32,
        per_minute: PerMinute,
    },
    /// beat-unit = beat-unit (metric modulation)
    BeatEquation {
        left_unit: NoteTypeValue,
        left_dots: u32,
        right_unit: NoteTypeValue,
        right_dots: u32,
    },
    /// Modern metronome with more options
    Modern(Vec<MetronomeRelation>),
}

pub enum PerMinute {
    Value(f64),
    Text(String),  // "ca. 120", "120-132"
}
```

---

### rehearsal

Rehearsal marks (letters, numbers, section names).

**MusicXML:**
```xml
<direction-type>
  <rehearsal enclosure="square">A</rehearsal>
</direction-type>

<direction-type>
  <rehearsal enclosure="rectangle">Verse</rehearsal>
</direction-type>

<direction-type>
  <rehearsal enclosure="circle">1</rehearsal>
</direction-type>
```

**S-expr:**
```lisp
(rehearsal :enclosure :square "A")

(rehearsal :enclosure :rectangle "Verse")

(rehearsal :enclosure :circle "1")
```

**Mapping Rules:**
- Text content → final argument
- `enclosure` → `:enclosure` (default `:square`)
- Print-style attrs for position, font, color

**Type Definition:**
```rust
pub struct Rehearsal {
    pub text: String,
    pub enclosure: Option<EnclosureShape>,
    pub lang: Option<String>,
    // print-style-align...
}
```

---

### segno and coda

Navigation markers for jumps.

**MusicXML:**
```xml
<direction-type>
  <segno/>
</direction-type>

<direction-type>
  <coda/>
</direction-type>
```

**S-expr:**
```lisp
(segno)

(coda)
```

**Mapping Rules:**
- Usually empty with just position attrs
- `smufl` attr for alternate glyphs

**Type Definition:**
```rust
pub struct Segno {
    pub smufl: Option<SmuflSegnoGlyphName>,
    // print-style-align...
}

pub struct Coda {
    pub smufl: Option<SmuflCodaGlyphName>,
    // print-style-align...
}
```

---

### pedal

Piano pedal markings.

**MusicXML:**
```xml
<!-- Pedal down -->
<direction-type>
  <pedal type="start" line="yes" sign="yes"/>
</direction-type>

<!-- Pedal up -->
<direction-type>
  <pedal type="stop"/>
</direction-type>

<!-- Quick pedal change -->
<direction-type>
  <pedal type="change"/>
</direction-type>

<!-- With line continuation -->
<direction-type>
  <pedal type="continue"/>
</direction-type>
```

**S-expr:**
```lisp
;; Pedal down
(pedal :type :start :line :yes :sign :yes)

;; Pedal up
(pedal :type :stop)

;; Quick change
(pedal :type :change)

;; Continue line
(pedal :type :continue)
```

**Mapping Rules:**
- `type` → `:type` (`:start`, `:stop`, `:continue`, `:change`, etc.)
- `line` → `:line` (`:yes` for bracket/line style)
- `sign` → `:sign` (`:yes` for Ped symbol)
- `number` → `:number` (for sostenuto, una corda)

---

### octave-shift

8va/8vb/15ma markings.

**MusicXML:**
```xml
<!-- 8va start -->
<direction-type>
  <octave-shift type="up" size="8"/>
</direction-type>

<!-- 8va end -->
<direction-type>
  <octave-shift type="stop" size="8"/>
</direction-type>

<!-- 15ma -->
<direction-type>
  <octave-shift type="up" size="15"/>
</direction-type>

<!-- 8vb -->
<direction-type>
  <octave-shift type="down" size="8"/>
</direction-type>
```

**S-expr:**
```lisp
;; 8va start
(octave-shift :type :up :size 8)

;; 8va end
(octave-shift :type :stop :size 8)

;; 15ma
(octave-shift :type :up :size 15)

;; 8vb
(octave-shift :type :down :size 8)
```

**Mapping Rules:**
- `type` → `:type` (`:up`, `:down`, `:stop`, `:continue`)
- `size` → `:size` (8, 15, 22)
- `number` → `:number` (for overlapping shifts)

---

## Interrelationships

1. **direction ↔ sound:** The `<sound>` child affects playback (tempo, dynamics).

2. **wedge start ↔ stop:** Wedges require matching start/stop with same `number`.

3. **pedal start ↔ stop:** Pedal markings require matching pairs.

4. **octave-shift ↔ notes:** Notes within the shift sound an octave different.

5. **segno/coda ↔ barline:** Navigation can also appear in barlines.

6. **metronome ↔ sound tempo:** Metronome display vs. `<sound tempo="...">` for playback.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Direction-type wrapper | Preserve | MusicXML structure; allows combining |
| Dynamics as elements | Empty forms like `(f)` | Matches MusicXML; composable |
| Wedge number | Required for overlapping | Disambiguate start/stop pairs |
| Words text position | Final argument | Natural reading |
| Metronome structure | Distinct content types | Different representations need different fields |

---

## Open Questions

- [ ] **Combined direction-types:** When multiple types in one direction, do they share position?

- [ ] **Sound element:** Include playback-only info in IR, or separate layer?

- [ ] **Dashes/brackets:** Start/stop pairs like wedges. Document in detail?

---

## Examples

### Example 1: Forte Dynamic

**Musical meaning:** Play loudly

**MusicXML:**
```xml
<direction placement="below">
  <direction-type>
    <dynamics>
      <f/>
    </dynamics>
  </direction-type>
  <sound dynamics="90"/>
</direction>
```

**S-expr:**
```lisp
(direction :placement :below
  (direction-type
    (dynamics (f)))
  (sound :dynamics 90))
```

---

### Example 2: Crescendo Hairpin

**Musical meaning:** Gradually get louder over several notes

**MusicXML:**
```xml
<!-- At start of crescendo -->
<direction>
  <direction-type>
    <wedge type="crescendo" number="1"/>
  </direction-type>
</direction>

<!-- At end of crescendo (with target dynamic) -->
<direction>
  <direction-type>
    <wedge type="stop" number="1"/>
  </direction-type>
  <direction-type>
    <dynamics><ff/></dynamics>
  </direction-type>
  <sound dynamics="100"/>
</direction>
```

**S-expr:**
```lisp
;; Start
(direction
  (direction-type
    (wedge :type :crescendo :number 1)))

;; End with target dynamic
(direction
  (direction-type
    (wedge :type :stop :number 1))
  (direction-type
    (dynamics (ff)))
  (sound :dynamics 100))
```

---

### Example 3: Tempo Marking

**Musical meaning:** Allegro, quarter = 120

**MusicXML:**
```xml
<direction placement="above">
  <direction-type>
    <words font-weight="bold" font-size="14">Allegro</words>
  </direction-type>
  <direction-type>
    <metronome>
      <beat-unit>quarter</beat-unit>
      <per-minute>120</per-minute>
    </metronome>
  </direction-type>
  <sound tempo="120"/>
</direction>
```

**S-expr:**
```lisp
(direction :placement :above
  (direction-type
    (words :font-weight :bold :font-size 14 "Allegro"))
  (direction-type
    (metronome
      :beat-unit :quarter
      :per-minute 120))
  (sound :tempo 120))
```

---

### Example 4: Piano Pedaling

**Musical meaning:** Sustain pedal down and up

**MusicXML:**
```xml
<!-- Pedal down -->
<direction>
  <direction-type>
    <pedal type="start" line="no" sign="yes"/>
  </direction-type>
  <sound damper-pedal="yes"/>
</direction>

<!-- Notes... -->

<!-- Pedal up -->
<direction>
  <direction-type>
    <pedal type="stop"/>
  </direction-type>
  <sound damper-pedal="no"/>
</direction>
```

**S-expr:**
```lisp
;; Pedal down
(direction
  (direction-type
    (pedal :type :start :line :no :sign :yes))
  (sound :damper-pedal :yes))

;; Pedal up
(direction
  (direction-type
    (pedal :type :stop))
  (sound :damper-pedal :no))
```

---

### Example 5: 8va Passage

**Musical meaning:** Play an octave higher than written

**MusicXML:**
```xml
<!-- Start 8va -->
<direction>
  <direction-type>
    <octave-shift type="up" size="8"/>
  </direction-type>
</direction>

<!-- Notes played octave higher... -->

<!-- End 8va -->
<direction>
  <direction-type>
    <octave-shift type="stop" size="8"/>
  </direction-type>
</direction>
```

**S-expr:**
```lisp
;; Start 8va
(direction
  (direction-type
    (octave-shift :type :up :size 8)))

;; End 8va
(direction
  (direction-type
    (octave-shift :type :stop :size 8)))
```

---

### Example 6: D.S. al Coda

**Musical meaning:** Navigation instruction with segno and coda

**MusicXML:**
```xml
<!-- At the segno sign -->
<direction>
  <direction-type>
    <segno/>
  </direction-type>
</direction>

<!-- At the coda jump point -->
<direction>
  <direction-type>
    <words>To Coda</words>
  </direction-type>
  <direction-type>
    <coda/>
  </direction-type>
</direction>

<!-- D.S. instruction -->
<direction>
  <direction-type>
    <words font-style="italic">D.S. al Coda</words>
  </direction-type>
  <sound dalsegno="segno1" tocoda="coda1"/>
</direction>

<!-- At the coda -->
<direction>
  <direction-type>
    <coda/>
  </direction-type>
</direction>
```

**S-expr:**
```lisp
;; Segno sign
(direction
  (direction-type (segno)))

;; To Coda
(direction
  (direction-type (words "To Coda"))
  (direction-type (coda)))

;; D.S. instruction
(direction
  (direction-type
    (words :font-style :italic "D.S. al Coda"))
  (sound :dalsegno "segno1" :tocoda "coda1"))

;; Coda section
(direction
  (direction-type (coda)))
```
