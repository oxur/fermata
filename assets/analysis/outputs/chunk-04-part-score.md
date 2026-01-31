# Chunk 4: Part & Score

## Overview

This chunk covers the top-level document structure: scores, parts, and their metadata. These elements define the overall organization of a musical work.

**Key Concept:** MusicXML offers two document structures:
- **score-partwise:** Parts contain measures (preferred, more common)
- **score-timewise:** Measures contain parts (deprecated)

The IR focuses on `score-partwise` as it's the standard.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<score-partwise>` | root | `<part-list>`, `<part>+` | score-header | `version` |
| `<score-timewise>` | root | `<part-list>`, `<measure>+` | score-header | `version` (deprecated) |
| `<part>` | complex | `<measure>+` | — | `id` (required) |
| `<part-list>` | complex | `<score-part>+` | `<part-group>*` | — |
| `<score-part>` | complex | `<part-name>` | many | `id` (required) |
| `<work>` | complex | — | `<work-number>`, `<work-title>`, `<opus>` | — |
| `<identification>` | complex | — | `<creator>*`, `<rights>*`, `<encoding>`, etc. | — |
| `<defaults>` | complex | — | layout, appearance, fonts | — |
| `<credit>` | complex | — | `<credit-words>*`, `<credit-image>*` | `page` |

## S-Expression Mappings

### score-partwise

The root element for a MusicXML score.

**MusicXML:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <work>
    <work-title>Symphony No. 5</work-title>
  </work>
  <identification>
    <creator type="composer">Ludwig van Beethoven</creator>
  </identification>
  <part-list>
    <score-part id="P1">
      <part-name>Violin I</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">...</measure>
  </part>
</score-partwise>
```

**S-expr:**
```lisp
(score-partwise :version "4.0"
  (work
    :title "Symphony No. 5")
  (identification
    (creator :type :composer "Ludwig van Beethoven"))
  (part-list
    (score-part :id "P1"
      :name "Violin I"))
  (part :id "P1"
    (measure :number "1" ...)))
```

**Mapping Rules:**
- `version` attribute → `:version`
- score-header group elements → corresponding forms
- `<part>` elements → `(part ...)` forms

**Type Definition:**
```rust
pub struct ScorePartwise {
    pub version: Option<String>,
    // score-header group
    pub work: Option<Work>,
    pub movement_number: Option<String>,
    pub movement_title: Option<String>,
    pub identification: Option<Identification>,
    pub defaults: Option<Defaults>,
    pub credits: Vec<Credit>,
    pub part_list: PartList,
    // parts
    pub parts: Vec<Part>,
}
```

---

### part

A single instrument or voice part containing measures.

**MusicXML:**
```xml
<part id="P1">
  <measure number="1">...</measure>
  <measure number="2">...</measure>
  <measure number="3">...</measure>
</part>
```

**S-expr:**
```lisp
(part :id "P1"
  (measure :number "1" ...)
  (measure :number "2" ...)
  (measure :number "3" ...))
```

**Mapping Rules:**
- `id` attribute → `:id` (required, must match score-part)
- Contains sequence of measures

**Type Definition:**
```rust
pub struct Part {
    pub id: String,
    pub measures: Vec<Measure>,
}
```

---

### part-list

Declares all parts in the score and their groupings.

**MusicXML:**
```xml
<part-list>
  <part-group type="start" number="1">
    <group-symbol>bracket</group-symbol>
    <group-name>Strings</group-name>
  </part-group>
  <score-part id="P1">
    <part-name>Violin I</part-name>
    <part-abbreviation>Vln. I</part-abbreviation>
  </score-part>
  <score-part id="P2">
    <part-name>Violin II</part-name>
  </score-part>
  <score-part id="P3">
    <part-name>Viola</part-name>
  </score-part>
  <score-part id="P4">
    <part-name>Cello</part-name>
  </score-part>
  <part-group type="stop" number="1"/>
</part-list>
```

**S-expr:**
```lisp
(part-list
  (part-group :type :start :number 1
    :symbol :bracket
    :name "Strings")
  (score-part :id "P1"
    :name "Violin I"
    :abbreviation "Vln. I")
  (score-part :id "P2"
    :name "Violin II")
  (score-part :id "P3"
    :name "Viola")
  (score-part :id "P4"
    :name "Cello")
  (part-group :type :stop :number 1))
```

**Mapping Rules:**
- Contains interleaved `<score-part>` and `<part-group>` elements
- Order matters for visual layout

**Type Definition:**
```rust
pub struct PartList {
    pub items: Vec<PartListItem>,
}

pub enum PartListItem {
    ScorePart(ScorePart),
    PartGroup(PartGroup),
}

pub struct PartGroup {
    pub r#type: StartStop,
    pub number: Option<String>,
    pub group_name: Option<GroupName>,
    pub group_name_display: Option<NameDisplay>,
    pub group_abbreviation: Option<GroupName>,
    pub group_abbreviation_display: Option<NameDisplay>,
    pub group_symbol: Option<GroupSymbol>,
    pub group_barline: Option<GroupBarline>,
    pub group_time: Option<Empty>,
    // editorial...
}

pub enum GroupSymbol {
    None, Brace, Line, Bracket, Square,
}
```

---

### score-part

Declares a single part with its metadata.

**MusicXML:**
```xml
<score-part id="P1">
  <part-name>Piano</part-name>
  <part-name-display>
    <display-text>Piano</display-text>
  </part-name-display>
  <part-abbreviation>Pno.</part-abbreviation>
  <score-instrument id="P1-I1">
    <instrument-name>Piano</instrument-name>
    <instrument-sound>keyboard.piano.grand</instrument-sound>
  </score-instrument>
  <midi-device id="P1-I1" port="1"/>
  <midi-instrument id="P1-I1">
    <midi-channel>1</midi-channel>
    <midi-program>1</midi-program>
    <volume>80</volume>
    <pan>0</pan>
  </midi-instrument>
</score-part>
```

**S-expr:**
```lisp
(score-part :id "P1"
  :name "Piano"
  :name-display (name-display
                  (display-text "Piano"))
  :abbreviation "Pno."
  :instruments ((score-instrument :id "P1-I1"
                  :name "Piano"
                  :sound "keyboard.piano.grand"))
  :midi-devices ((midi-device :id "P1-I1" :port 1))
  :midi-instruments ((midi-instrument :id "P1-I1"
                       :channel 1
                       :program 1
                       :volume 80
                       :pan 0)))
```

**Mapping Rules:**
- `id` attribute → `:id` (required)
- `<part-name>` → `:name`
- `<part-abbreviation>` → `:abbreviation`
- `<score-instrument>` → `:instruments` list
- MIDI elements → corresponding sub-forms

**Type Definition:**
```rust
pub struct ScorePart {
    pub id: String,
    pub identification: Option<Identification>,
    pub part_link: Vec<PartLink>,
    pub part_name: PartName,
    pub part_name_display: Option<NameDisplay>,
    pub part_abbreviation: Option<PartName>,
    pub part_abbreviation_display: Option<NameDisplay>,
    pub group: Vec<String>,
    pub score_instruments: Vec<ScoreInstrument>,
    pub player: Vec<Player>,
    pub midi_devices: Vec<MidiDevice>,
    pub midi_instruments: Vec<MidiInstrument>,
}

pub struct ScoreInstrument {
    pub id: String,
    pub instrument_name: String,
    pub instrument_abbreviation: Option<String>,
    pub instrument_sound: Option<String>,
    pub solo: Option<Empty>,
    pub ensemble: Option<Ensemble>,
    pub virtual_instrument: Option<VirtualInstrument>,
}
```

---

### work

Metadata about the musical work.

**MusicXML:**
```xml
<work>
  <work-number>Op. 67</work-number>
  <work-title>Symphony No. 5 in C minor</work-title>
  <opus xlink:href="opus.xml"/>
</work>
```

**S-expr:**
```lisp
(work
  :number "Op. 67"
  :title "Symphony No. 5 in C minor"
  :opus (opus :href "opus.xml"))
```

**Mapping Rules:**
- `<work-number>` → `:number`
- `<work-title>` → `:title`
- `<opus>` → `:opus` (link to opus file)

**Type Definition:**
```rust
pub struct Work {
    pub work_number: Option<String>,
    pub work_title: Option<String>,
    pub opus: Option<Opus>,
}

pub struct Opus {
    // xlink attributes
    pub href: String,
    pub r#type: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
    pub show: Option<String>,
    pub actuate: Option<String>,
}
```

---

### identification

Creator and rights information.

**MusicXML:**
```xml
<identification>
  <creator type="composer">Johann Sebastian Bach</creator>
  <creator type="lyricist">Christian Friedrich Henrici</creator>
  <creator type="arranger">John Doe</creator>
  <rights>Copyright © 2024</rights>
  <encoding>
    <software>Finale 27</software>
    <encoding-date>2024-01-15</encoding-date>
    <encoder>Jane Smith</encoder>
    <supports element="accidental" type="yes"/>
    <supports element="beam" type="yes"/>
  </encoding>
  <source>Manuscript, Berlin State Library</source>
  <relation>Based on BWV 248</relation>
  <miscellaneous>
    <miscellaneous-field name="dedication">For Maria</miscellaneous-field>
  </miscellaneous>
</identification>
```

**S-expr:**
```lisp
(identification
  (creator :type :composer "Johann Sebastian Bach")
  (creator :type :lyricist "Christian Friedrich Henrici")
  (creator :type :arranger "John Doe")
  (rights "Copyright © 2024")
  (encoding
    :software "Finale 27"
    :date "2024-01-15"
    :encoder "Jane Smith"
    :supports ((supports :element "accidental" :type :yes)
               (supports :element "beam" :type :yes)))
  (source "Manuscript, Berlin State Library")
  (relation "Based on BWV 248")
  (miscellaneous
    (field :name "dedication" "For Maria")))
```

**Mapping Rules:**
- Multiple `<creator>` elements with different `type` attributes
- `<rights>` can appear multiple times
- `<encoding>` contains software/encoding metadata

**Type Definition:**
```rust
pub struct Identification {
    pub creators: Vec<TypedText>,
    pub rights: Vec<TypedText>,
    pub encoding: Option<Encoding>,
    pub source: Option<String>,
    pub relation: Vec<TypedText>,
    pub miscellaneous: Option<Miscellaneous>,
}

pub struct TypedText {
    pub r#type: Option<String>,
    pub text: String,
}

pub struct Encoding {
    pub software: Vec<String>,
    pub encoding_date: Vec<String>,
    pub encoder: Vec<TypedText>,
    pub encoding_description: Vec<String>,
    pub supports: Vec<Supports>,
}
```

---

### defaults

Score-wide default settings for layout and appearance.

**MusicXML:**
```xml
<defaults>
  <scaling>
    <millimeters>7.2319</millimeters>
    <tenths>40</tenths>
  </scaling>
  <page-layout>
    <page-height>1545</page-height>
    <page-width>1194</page-width>
    <page-margins type="both">
      <left-margin>70</left-margin>
      <right-margin>70</right-margin>
      <top-margin>88</top-margin>
      <bottom-margin>88</bottom-margin>
    </page-margins>
  </page-layout>
  <system-layout>
    <system-margins>
      <left-margin>0</left-margin>
      <right-margin>0</right-margin>
    </system-margins>
    <system-distance>121</system-distance>
    <top-system-distance>70</top-system-distance>
  </system-layout>
  <staff-layout>
    <staff-distance>65</staff-distance>
  </staff-layout>
  <music-font font-family="Bravura" font-size="20.5"/>
  <word-font font-family="Times New Roman" font-size="10.25"/>
  <lyric-font font-family="Times New Roman" font-size="10"/>
</defaults>
```

**S-expr:**
```lisp
(defaults
  :scaling (scaling :millimeters 7.2319 :tenths 40)
  :page-layout (page-layout
                 :height 1545
                 :width 1194
                 :margins (page-margins :type :both
                            :left 70 :right 70
                            :top 88 :bottom 88))
  :system-layout (system-layout
                   :margins (system-margins :left 0 :right 0)
                   :distance 121
                   :top-distance 70)
  :staff-layout (staff-layout :distance 65)
  :music-font (font :family "Bravura" :size 20.5)
  :word-font (font :family "Times New Roman" :size 10.25)
  :lyric-font (font :family "Times New Roman" :size 10))
```

**Mapping Rules:**
- Layout elements are optional but important for rendering
- Tenths are the unit for most measurements
- Scaling defines tenths-to-millimeters ratio

**Note:** Layout elements may be deferred in initial implementation.

---

### credit

Text and images that appear on score pages (title, composer, etc.).

**MusicXML:**
```xml
<credit page="1">
  <credit-type>title</credit-type>
  <credit-words default-x="595" default-y="1527"
    font-size="24" font-weight="bold"
    justify="center" valign="top">Symphony No. 5</credit-words>
</credit>
<credit page="1">
  <credit-type>composer</credit-type>
  <credit-words default-x="1124" default-y="1427"
    font-size="12" justify="right">Ludwig van Beethoven</credit-words>
</credit>
```

**S-expr:**
```lisp
(credit :page 1
  :type "title"
  (credit-words
    :default-x 595 :default-y 1527
    :font-size 24 :font-weight :bold
    :justify :center :valign :top
    "Symphony No. 5"))

(credit :page 1
  :type "composer"
  (credit-words
    :default-x 1124 :default-y 1427
    :font-size 12 :justify :right
    "Ludwig van Beethoven"))
```

**Mapping Rules:**
- `page` attribute → `:page` (page number)
- `<credit-type>` → `:type` (title, subtitle, composer, etc.)
- `<credit-words>` → `(credit-words ...)` with text
- Can have multiple `<credit-words>` for mixed formatting

---

## Interrelationships

1. **part.id ↔ score-part.id**: Every `<part>` must have an `id` matching a `<score-part>` in `<part-list>`.

2. **score-instrument.id ↔ midi-instrument.id**: MIDI settings reference instrument IDs.

3. **part-group ↔ score-part**: Part groups bracket contiguous score-parts via start/stop.

4. **defaults ↔ measure layout**: Page and system layout affect how measures are distributed.

5. **credit.page ↔ rendering**: Credits specify page number for placement.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Focus on score-partwise | Yes | It's the primary format; timewise is deprecated |
| Part ID type | String | Matches MusicXML; allows "P1", "Piano", etc. |
| Preserve MIDI elements | Yes (in IR) | Needed for round-tripping |
| Preserve layout/defaults | Yes (in IR) | Needed for faithful reproduction |
| Credit handling | Preserve structure | Complex formatting needs to be maintained |

---

## Open Questions

- [ ] **score-timewise support:** Do we need to parse it and convert to partwise? Or reject it?

- [ ] **Part ID validation:** Should the IR validate that part IDs match score-part IDs?

- [ ] **Opus handling:** The `<opus>` element links to external files. How to handle?

- [ ] **Layout defaults:** Store in IR but ignore for initial rendering?

---

## Examples

### Example 1: Minimal Valid Score

**Musical meaning:** Simplest possible valid MusicXML

**MusicXML:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Music</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>1</divisions>
        <time><beats>4</beats><beat-type>4</beat-type></time>
        <clef><sign>G</sign><line>2</line></clef>
      </attributes>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <type>whole</type>
      </note>
    </measure>
  </part>
</score-partwise>
```

**S-expr:**
```lisp
(score-partwise :version "4.0"
  (part-list
    (score-part :id "P1" :name "Music"))
  (part :id "P1"
    (measure :number "1"
      (attributes
        :divisions 1
        :time (time :beats "4" :beat-type "4")
        :clef (clef :sign :G :line 2))
      (note
        :pitch (pitch :step :C :octave 4)
        :duration 4
        :type :whole))))
```

---

### Example 2: Score with Work Metadata

**Musical meaning:** Score with complete metadata

**MusicXML:**
```xml
<score-partwise version="4.0">
  <work>
    <work-number>BWV 846</work-number>
    <work-title>Prelude in C Major</work-title>
  </work>
  <movement-number>1</movement-number>
  <movement-title>Prelude</movement-title>
  <identification>
    <creator type="composer">Johann Sebastian Bach</creator>
    <rights>Public Domain</rights>
    <encoding>
      <software>Fermata 0.1.0</software>
      <encoding-date>2024-03-15</encoding-date>
    </encoding>
  </identification>
  <part-list>
    <score-part id="P1">
      <part-name>Piano</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">...</measure>
  </part>
</score-partwise>
```

**S-expr:**
```lisp
(score-partwise :version "4.0"
  (work
    :number "BWV 846"
    :title "Prelude in C Major")
  :movement-number "1"
  :movement-title "Prelude"
  (identification
    (creator :type :composer "Johann Sebastian Bach")
    (rights "Public Domain")
    (encoding
      :software "Fermata 0.1.0"
      :date "2024-03-15"))
  (part-list
    (score-part :id "P1" :name "Piano"))
  (part :id "P1"
    (measure :number "1" ...)))
```

---

### Example 3: Multi-Part Score with Grouping

**Musical meaning:** String quartet with grouped parts

**MusicXML:**
```xml
<score-partwise version="4.0">
  <part-list>
    <part-group type="start" number="1">
      <group-symbol>bracket</group-symbol>
      <group-name>String Quartet</group-name>
      <group-barline>yes</group-barline>
    </part-group>
    <score-part id="P1">
      <part-name>Violin I</part-name>
      <part-abbreviation>Vln. I</part-abbreviation>
    </score-part>
    <score-part id="P2">
      <part-name>Violin II</part-name>
      <part-abbreviation>Vln. II</part-abbreviation>
    </score-part>
    <score-part id="P3">
      <part-name>Viola</part-name>
      <part-abbreviation>Vla.</part-abbreviation>
    </score-part>
    <score-part id="P4">
      <part-name>Violoncello</part-name>
      <part-abbreviation>Vc.</part-abbreviation>
    </score-part>
    <part-group type="stop" number="1"/>
  </part-list>
  <part id="P1">...</part>
  <part id="P2">...</part>
  <part id="P3">...</part>
  <part id="P4">...</part>
</score-partwise>
```

**S-expr:**
```lisp
(score-partwise :version "4.0"
  (part-list
    (part-group :type :start :number 1
      :symbol :bracket
      :name "String Quartet"
      :barline :yes)
    (score-part :id "P1" :name "Violin I" :abbreviation "Vln. I")
    (score-part :id "P2" :name "Violin II" :abbreviation "Vln. II")
    (score-part :id "P3" :name "Viola" :abbreviation "Vla.")
    (score-part :id "P4" :name "Violoncello" :abbreviation "Vc.")
    (part-group :type :stop :number 1))
  (part :id "P1" ...)
  (part :id "P2" ...)
  (part :id "P3" ...)
  (part :id "P4" ...))
```

---

### Example 4: Piano Part with MIDI

**Musical meaning:** Piano part with MIDI playback settings

**MusicXML:**
```xml
<part-list>
  <score-part id="P1">
    <part-name>Piano</part-name>
    <score-instrument id="P1-I1">
      <instrument-name>Acoustic Grand Piano</instrument-name>
      <instrument-sound>keyboard.piano.grand</instrument-sound>
    </score-instrument>
    <midi-device id="P1-I1" port="1"/>
    <midi-instrument id="P1-I1">
      <midi-channel>1</midi-channel>
      <midi-program>1</midi-program>
      <volume>80</volume>
      <pan>0</pan>
    </midi-instrument>
  </score-part>
</part-list>
```

**S-expr:**
```lisp
(part-list
  (score-part :id "P1"
    :name "Piano"
    :instruments ((score-instrument :id "P1-I1"
                    :name "Acoustic Grand Piano"
                    :sound "keyboard.piano.grand"))
    :midi-devices ((midi-device :id "P1-I1" :port 1))
    :midi-instruments ((midi-instrument :id "P1-I1"
                         :channel 1
                         :program 1
                         :volume 80
                         :pan 0))))
```
