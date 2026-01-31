# Chunk 3: Measure Structure

## Overview

This chunk covers the structural elements that define measures and their musical properties: time signatures, key signatures, clefs, and barlines. These elements typically appear at the start of a piece or when changes occur.

**Key Concept:** The `<attributes>` element is a container that groups measure-level musical properties. It can appear multiple times within a measure for mid-measure changes.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<measure>` | complex | — | music-data group | `number`, `width`, `implicit`, `non-controlling` |
| `<attributes>` | complex | — | divisions, key, time, staves, clef, etc. | — |
| `<divisions>` | simple | — | — | — |
| `<key>` | complex | See key groups | `<key-octave>` | `number`, print-style, print-object |
| `<time>` | complex | beats/beat-type OR senza-misura | `<interchangeable>` | `number`, `symbol`, `separator`, print-style |
| `<clef>` | complex | `<sign>`, `<line>` | `<clef-octave-change>` | `number`, `additional`, `size`, `after-barline` |
| `<barline>` | complex | — | bar-style, repeat, ending, etc. | `location`, `segno`, `coda`, `divisions` |

## S-Expression Mappings

### measure

A container for all musical content within one bar.

**MusicXML:**
```xml
<measure number="1" width="200">
  <attributes>...</attributes>
  <note>...</note>
  <note>...</note>
  <barline location="right">...</barline>
</measure>

<!-- Implicit measure (pickup/anacrusis) -->
<measure number="0" implicit="yes">
  <note>...</note>
</measure>
```

**S-expr:**
```lisp
(measure :number "1" :width 200
  (attributes ...)
  (note ...)
  (note ...)
  (barline :location :right ...))

;; Implicit measure
(measure :number "0" :implicit :yes
  (note ...))
```

**Mapping Rules:**
- `number` attribute → `:number` (string, can be "1", "2a", "X1", etc.)
- `width` attribute → `:width` (tenths)
- `implicit` attribute → `:implicit` (`:yes` for pickups)
- `non-controlling` → `:non-controlling` (for multi-part coordination)
- Children are the music-data group elements

**Type Definition:**
```rust
pub struct Measure {
    pub number: String,
    pub width: Option<Tenths>,
    pub implicit: Option<YesNo>,
    pub non_controlling: Option<YesNo>,
    pub content: Vec<MusicDataElement>,
}

pub enum MusicDataElement {
    Note(Note),
    Backup(Backup),
    Forward(Forward),
    Direction(Direction),
    Attributes(Attributes),
    Harmony(Harmony),
    FiguredBass(FiguredBass),
    Print(Print),
    Sound(Sound),
    Listening(Listening),
    Barline(Barline),
    Grouping(Grouping),
    Link(Link),
    Bookmark(Bookmark),
}
```

---

### attributes

Container for measure-level musical properties.

**MusicXML:**
```xml
<attributes>
  <divisions>4</divisions>
  <key>
    <fifths>2</fifths>
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
```

**S-expr:**
```lisp
(attributes
  :divisions 4
  :key (key :fifths 2 :mode :major)
  :time (time :beats "4" :beat-type "4")
  :clef (clef :sign :G :line 2))
```

**Mapping Rules:**
- Each child element → corresponding keyword with sub-form
- Multiple keys/times/clefs possible (for multi-staff parts)
- `<staves>` → `:staves` (number of staves in part)
- `<instruments>` → `:instruments` (number of instruments)
- `<part-symbol>` → `:part-symbol` (brace/bracket type)

**Type Definition:**
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
    pub transpose: Vec<Transpose>,
    pub for_part: Vec<ForPart>,
    pub directives: Vec<Directive>,
    pub measure_style: Vec<MeasureStyle>,
}
```

---

### divisions

Defines the number of divisions per quarter note for duration calculations.

**MusicXML:**
```xml
<divisions>24</divisions>
```

**S-expr:**
```lisp
:divisions 24
```

**Mapping Rules:**
- Simple integer value
- Common values: 1, 2, 4, 8, 24, 960 (higher = more rhythmic precision)
- A quarter note has duration = divisions
- An eighth note has duration = divisions/2

**Note:** The IR may normalize all durations to a fixed division (e.g., 960) during parsing to simplify arithmetic.

---

### key

Specifies the key signature.

**Traditional Key (by fifths):**
```xml
<key>
  <fifths>-3</fifths>
  <mode>minor</mode>
</key>

<key>
  <fifths>2</fifths>
  <mode>major</mode>
</key>
```

**S-expr:**
```lisp
;; Eb minor (3 flats)
(key :fifths -3 :mode :minor)

;; D major (2 sharps)
(key :fifths 2 :mode :major)
```

**Non-Traditional Key:**
```xml
<key>
  <key-step>F</key-step>
  <key-alter>1</key-alter>
  <key-step>C</key-step>
  <key-alter>1</key-alter>
  <key-step>G</key-step>
  <key-alter>1</key-alter>
</key>
```

**S-expr:**
```lisp
;; Custom key with F#, C#, G#
(key
  :non-traditional
  ((key-step :step :F :alter 1)
   (key-step :step :C :alter 1)
   (key-step :step :G :alter 1)))
```

**Mapping Rules:**
- Traditional: `:fifths` (-7 to +7) + optional `:mode`
- Non-traditional: `:non-traditional` with list of step/alter pairs
- `number` attribute → `:staff` (for multi-staff)
- Optional `<key-octave>` elements for specifying octave placement

**Fifths Reference:**

| Fifths | Major Key | Minor Key |
|--------|-----------|-----------|
| -7 | Cb | Ab |
| -6 | Gb | Eb |
| -5 | Db | Bb |
| -4 | Ab | F |
| -3 | Eb | C |
| -2 | Bb | G |
| -1 | F | D |
| 0 | C | A |
| +1 | G | E |
| +2 | D | B |
| +3 | A | F# |
| +4 | E | C# |
| +5 | B | G# |
| +6 | F# | D# |
| +7 | C# | A# |

**Type Definition:**
```rust
pub struct Key {
    pub content: KeyContent,
    pub number: Option<StaffNumber>,
    // print-style, print-object...
}

pub enum KeyContent {
    Traditional(TraditionalKey),
    NonTraditional(Vec<KeyStep>),
}

pub struct TraditionalKey {
    pub cancel: Option<Cancel>,
    pub fifths: i8,  // -7 to +7
    pub mode: Option<Mode>,
}

pub struct KeyStep {
    pub step: Step,
    pub alter: Semitones,
    pub accidental: Option<KeyAccidental>,
}

pub enum Mode {
    Major, Minor,
    Dorian, Phrygian, Lydian, Mixolydian, Aeolian, Locrian,
    Ionian,  // Same as major
    None,    // For atonal
}
```

---

### time

Specifies the time signature.

**Simple Time Signature:**
```xml
<time>
  <beats>4</beats>
  <beat-type>4</beat-type>
</time>

<time>
  <beats>6</beats>
  <beat-type>8</beat-type>
</time>
```

**S-expr:**
```lisp
;; 4/4
(time :beats "4" :beat-type "4")

;; 6/8
(time :beats "6" :beat-type "8")
```

**Compound/Additive Time Signatures:**
```xml
<!-- 3+2/8 -->
<time>
  <beats>3+2</beats>
  <beat-type>8</beat-type>
</time>

<!-- 2/4 + 3/8 -->
<time>
  <beats>2</beats>
  <beat-type>4</beat-type>
  <beats>3</beats>
  <beat-type>8</beat-type>
</time>
```

**S-expr:**
```lisp
;; 3+2/8
(time :beats "3+2" :beat-type "8")

;; 2/4 + 3/8 (composite)
(time
  :signatures ((time-signature :beats "2" :beat-type "4")
               (time-signature :beats "3" :beat-type "8")))
```

**Special Symbols:**
```xml
<time symbol="common">
  <beats>4</beats>
  <beat-type>4</beat-type>
</time>

<time symbol="cut">
  <beats>2</beats>
  <beat-type>2</beat-type>
</time>

<time symbol="single-number">
  <beats>3</beats>
  <beat-type>8</beat-type>
</time>
```

**S-expr:**
```lisp
;; Common time (C symbol)
(time :symbol :common :beats "4" :beat-type "4")

;; Cut time (C with slash)
(time :symbol :cut :beats "2" :beat-type "2")

;; Single number display
(time :symbol :single-number :beats "3" :beat-type "8")
```

**Senza Misura (no time signature):**
```xml
<time>
  <senza-misura>X</senza-misura>
</time>
```

**S-expr:**
```lisp
(time :senza-misura "X")
```

**Mapping Rules:**
- `<beats>` → `:beats` (string, can be "3+2")
- `<beat-type>` → `:beat-type` (string)
- Multiple beats/beat-type pairs → composite signature
- `symbol` attr → `:symbol` (`:common`, `:cut`, `:single-number`, `:normal`, `:note`, `:dotted-note`)
- `<senza-misura>` → `:senza-misura`

**Type Definition:**
```rust
pub struct Time {
    pub content: TimeContent,
    pub number: Option<StaffNumber>,
    pub symbol: Option<TimeSymbol>,
    pub separator: Option<TimeSeparator>,
    // print-style, print-object...
}

pub enum TimeContent {
    Measured {
        signatures: Vec<TimeSignature>,
        interchangeable: Option<Interchangeable>,
    },
    SenzaMisura(String),
}

pub struct TimeSignature {
    pub beats: String,      // Can be "3+2"
    pub beat_type: String,
}

pub enum TimeSymbol {
    Common,
    Cut,
    SingleNumber,
    Note,
    DottedNote,
    Normal,
}
```

---

### clef

Specifies the clef.

**MusicXML:**
```xml
<!-- Treble clef -->
<clef>
  <sign>G</sign>
  <line>2</line>
</clef>

<!-- Bass clef -->
<clef>
  <sign>F</sign>
  <line>4</line>
</clef>

<!-- Treble 8va bassa -->
<clef>
  <sign>G</sign>
  <line>2</line>
  <clef-octave-change>-1</clef-octave-change>
</clef>

<!-- Percussion clef -->
<clef>
  <sign>percussion</sign>
</clef>

<!-- Tab clef -->
<clef>
  <sign>TAB</sign>
</clef>
```

**S-expr:**
```lisp
;; Treble clef
(clef :sign :G :line 2)

;; Bass clef
(clef :sign :F :line 4)

;; Treble 8va bassa
(clef :sign :G :line 2 :octave-change -1)

;; Percussion clef
(clef :sign :percussion)

;; Tab clef
(clef :sign :TAB)
```

**Mapping Rules:**
- `<sign>` → `:sign` (`:G`, `:F`, `:C`, `:percussion`, `:TAB`, `:jianpu`, `:none`)
- `<line>` → `:line` (staff line number, 1 = bottom)
- `<clef-octave-change>` → `:octave-change` (-2, -1, +1, +2)
- `number` attr → `:staff` (for multi-staff)
- `additional` attr → `:additional` (for mid-measure clef)
- `size` attr → `:size` (`:cue`, `:large`, `:full`)

**Common Clefs:**

| Name | Sign | Line | Octave-Change |
|------|------|------|---------------|
| Treble | G | 2 | — |
| Bass | F | 4 | — |
| Alto | C | 3 | — |
| Tenor | C | 4 | — |
| Treble 8va | G | 2 | +1 |
| Treble 8vb | G | 2 | -1 |
| Bass 8vb | F | 4 | -1 |

**Type Definition:**
```rust
pub struct Clef {
    pub sign: ClefSign,
    pub line: Option<u8>,
    pub clef_octave_change: Option<i8>,
    pub number: Option<StaffNumber>,
    pub additional: Option<YesNo>,
    pub size: Option<SymbolSize>,
    pub after_barline: Option<YesNo>,
    // print-style, print-object...
}

pub enum ClefSign {
    G, F, C, Percussion, TAB, Jianpu, None,
}
```

---

### barline

Specifies barline style, repeats, and endings.

**MusicXML:**
```xml
<!-- Regular barline (usually implicit) -->
<barline location="right">
  <bar-style>light-light</bar-style>
</barline>

<!-- Double bar -->
<barline location="right">
  <bar-style>light-light</bar-style>
</barline>

<!-- Final bar -->
<barline location="right">
  <bar-style>light-heavy</bar-style>
</barline>

<!-- Repeat start -->
<barline location="left">
  <bar-style>heavy-light</bar-style>
  <repeat direction="forward"/>
</barline>

<!-- Repeat end -->
<barline location="right">
  <bar-style>light-heavy</bar-style>
  <repeat direction="backward"/>
</barline>

<!-- First ending -->
<barline location="left">
  <ending type="start" number="1">1.</ending>
</barline>
<barline location="right">
  <ending type="stop" number="1"/>
</barline>
```

**S-expr:**
```lisp
;; Double bar
(barline :location :right
  :bar-style :light-light)

;; Final bar
(barline :location :right
  :bar-style :light-heavy)

;; Repeat start
(barline :location :left
  :bar-style :heavy-light
  :repeat (repeat :direction :forward))

;; Repeat end
(barline :location :right
  :bar-style :light-heavy
  :repeat (repeat :direction :backward))

;; First ending start
(barline :location :left
  :ending (ending :type :start :number "1" :text "1."))

;; First ending end
(barline :location :right
  :ending (ending :type :stop :number "1"))
```

**Mapping Rules:**
- `location` attr → `:location` (`:left`, `:right`, `:middle`)
- `<bar-style>` → `:bar-style` keyword
- `<repeat>` → `:repeat` sub-form
- `<ending>` → `:ending` sub-form
- `<segno>` → `:segno`
- `<coda>` → `:coda`
- `<fermata>` → `:fermata` (can have up to 2)

**Bar Styles:**
- `:regular`, `:dotted`, `:dashed`, `:heavy`, `:light-light`, `:light-heavy`, `:heavy-light`, `:heavy-heavy`, `:tick`, `:short`, `:none`

**Type Definition:**
```rust
pub struct Barline {
    pub location: Option<RightLeftMiddle>,
    pub bar_style: Option<BarStyleColor>,
    pub editorial: Editorial,
    pub wavy_line: Option<WavyLine>,
    pub segno: Option<Segno>,
    pub coda: Option<Coda>,
    pub fermatas: Vec<Fermata>,  // 0-2
    pub ending: Option<Ending>,
    pub repeat: Option<Repeat>,
}

pub struct Repeat {
    pub direction: BackwardForward,
    pub times: Option<u32>,
    pub winged: Option<Winged>,
}

pub struct Ending {
    pub r#type: StartStopDiscontinue,
    pub number: String,  // Can be "1, 2" for combined
    pub text: Option<String>,
    // print-style...
}

pub enum BarStyle {
    Regular, Dotted, Dashed, Heavy,
    LightLight, LightHeavy, HeavyLight, HeavyHeavy,
    Tick, Short, None,
}
```

---

## Interrelationships

1. **attributes ↔ divisions ↔ note duration**: The `<divisions>` value determines how `<duration>` is interpreted. Must be set before notes.

2. **attributes ↔ key/time/clef**: These usually appear together at the start; changes can occur mid-piece.

3. **barline ↔ repeat ↔ ending**: These work together for repeat structures:
   - Repeat start: barline with `<repeat direction="forward"/>`
   - Repeat end: barline with `<repeat direction="backward"/>`
   - Endings: `<ending>` elements mark alternative passages

4. **clef.number ↔ staves**: For multi-staff parts, clef's `number` attribute indicates which staff.

5. **measure ↔ barline**: Barlines are children of measures. Right barline is often explicit; left barline for repeat starts.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Measure number type | String | MusicXML allows "1a", "X1", etc. |
| Key representation | Fifths-based for traditional | Direct MusicXML mapping; -7 to +7 |
| Time beats type | String | Allows "3+2" for additive |
| Clef sign type | Keywords | Finite set of valid signs |
| Barline location | Default `:right` | Most common case |
| Divisions normalization | Defer to parser | IR can normalize to fixed value |

---

## Open Questions

- [ ] **Mid-measure attributes:** How to associate attribute changes with specific time points? By position in element sequence?

- [ ] **Cancel key signatures:** `<cancel>` element for showing naturals. Include in IR?

- [ ] **Interchangeable time:** MusicXML's `<interchangeable>` for 6/8 = 2/4 equivalence. Defer?

- [ ] **Measure width:** Layout hint. Include or defer to layout engine?

---

## Examples

### Example 1: Simple Measure with Key, Time, Clef

**Musical meaning:** First measure of a piece in G major, 4/4, treble clef

**MusicXML:**
```xml
<measure number="1">
  <attributes>
    <divisions>4</divisions>
    <key>
      <fifths>1</fifths>
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
    <pitch><step>G</step><octave>4</octave></pitch>
    <duration>4</duration>
    <type>quarter</type>
  </note>
  <!-- more notes... -->
</measure>
```

**S-expr:**
```lisp
(measure :number "1"
  (attributes
    :divisions 4
    :key (key :fifths 1 :mode :major)
    :time (time :beats "4" :beat-type "4")
    :clef (clef :sign :G :line 2))
  (note
    :pitch (pitch :step :G :octave 4)
    :duration 4
    :type :quarter))
```

---

### Example 2: Piano Grand Staff (Two Clefs)

**Musical meaning:** Piano part with treble and bass clef

**MusicXML:**
```xml
<measure number="1">
  <attributes>
    <divisions>4</divisions>
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
  <!-- notes with staff="1" or staff="2" -->
</measure>
```

**S-expr:**
```lisp
(measure :number "1"
  (attributes
    :divisions 4
    :staves 2
    :clefs ((clef :number 1 :sign :G :line 2)
            (clef :number 2 :sign :F :line 4))))
```

---

### Example 3: Repeat with First and Second Endings

**Musical meaning:** 8-bar section with repeat, first ending leads to repeat, second ending continues

**MusicXML:**
```xml
<!-- Measure 1 - repeat start -->
<measure number="1">
  <barline location="left">
    <bar-style>heavy-light</bar-style>
    <repeat direction="forward"/>
  </barline>
  <!-- content -->
</measure>

<!-- Measure 7 - first ending start -->
<measure number="7">
  <barline location="left">
    <ending type="start" number="1">1.</ending>
  </barline>
  <!-- content -->
</measure>

<!-- Measure 8 - first ending end + repeat back -->
<measure number="8">
  <!-- content -->
  <barline location="right">
    <bar-style>light-heavy</bar-style>
    <ending type="stop" number="1"/>
    <repeat direction="backward"/>
  </barline>
</measure>

<!-- Measure 9 - second ending -->
<measure number="9">
  <barline location="left">
    <ending type="start" number="2">2.</ending>
  </barline>
  <!-- content -->
  <barline location="right">
    <ending type="stop" number="2"/>
  </barline>
</measure>
```

**S-expr:**
```lisp
;; Measure 1
(measure :number "1"
  (barline :location :left
    :bar-style :heavy-light
    :repeat (repeat :direction :forward))
  ;; content...
  )

;; Measure 7
(measure :number "7"
  (barline :location :left
    :ending (ending :type :start :number "1" :text "1."))
  ;; content...
  )

;; Measure 8
(measure :number "8"
  ;; content...
  (barline :location :right
    :bar-style :light-heavy
    :ending (ending :type :stop :number "1")
    :repeat (repeat :direction :backward)))

;; Measure 9
(measure :number "9"
  (barline :location :left
    :ending (ending :type :start :number "2" :text "2."))
  ;; content...
  (barline :location :right
    :ending (ending :type :stop :number "2")))
```

---

### Example 4: Time Signature Change

**Musical meaning:** Piece changes from 4/4 to 6/8

**MusicXML:**
```xml
<measure number="16">
  <!-- last measure in 4/4 -->
</measure>
<measure number="17">
  <attributes>
    <time>
      <beats>6</beats>
      <beat-type>8</beat-type>
    </time>
  </attributes>
  <!-- first measure in 6/8 -->
</measure>
```

**S-expr:**
```lisp
(measure :number "16"
  ;; content in 4/4...
  )

(measure :number "17"
  (attributes
    :time (time :beats "6" :beat-type "8"))
  ;; content in 6/8...
  )
```

---

### Example 5: Pickup Measure (Anacrusis)

**Musical meaning:** Piece starts with one beat before measure 1

**MusicXML:**
```xml
<measure number="0" implicit="yes">
  <attributes>
    <divisions>4</divisions>
    <key><fifths>0</fifths></key>
    <time><beats>4</beats><beat-type>4</beat-type></time>
    <clef><sign>G</sign><line>2</line></clef>
  </attributes>
  <note>
    <pitch><step>G</step><octave>4</octave></pitch>
    <duration>4</duration>
    <type>quarter</type>
  </note>
</measure>
<measure number="1">
  <!-- full measure -->
</measure>
```

**S-expr:**
```lisp
(measure :number "0" :implicit :yes
  (attributes
    :divisions 4
    :key (key :fifths 0)
    :time (time :beats "4" :beat-type "4")
    :clef (clef :sign :G :line 2))
  (note
    :pitch (pitch :step :G :octave 4)
    :duration 4
    :type :quarter))

(measure :number "1"
  ;; full measure...
  )
```
