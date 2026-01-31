# Chunk 10: Advanced and Deferred Elements

## Overview

This chunk provides a survey of MusicXML elements that are deferred to later implementation phases. These include specialized notations for specific instruments, rare compositional techniques, and historical notation systems.

**Important:** Unlike previous chunks which provide detailed mappings, this chunk serves as a **reference catalog**. Elements here are documented for completeness but detailed IR mappings may be developed later based on actual usage needs.

These elements fall into several categories:
1. **Instrument-specific** — Harp pedals, percussion notation, guitar tablature
2. **Harmonic analysis** — Chord symbols, figured bass, Nashville numbers
3. **Specialized notation** — Accordion, scordatura, early music symbols
4. **Playback-only** — MIDI-specific data, virtual instrument controls
5. **Accessibility** — Listening hints, assessment data

## Categories and Elements

### 10.1 Chord Symbols and Harmony Analysis

#### harmony

The `<harmony>` element represents chord symbols and functional harmony analysis.

**MusicXML Structure:**
```xml
<harmony type="explicit" print-frame="no">
  <root>
    <root-step>C</root-step>
  </root>
  <kind text="m7">minor-seventh</kind>
  <bass>
    <bass-step>E</bass-step>
    <bass-alter>-1</bass-alter>
  </bass>
</harmony>
```

**Provisional S-expr:**
```lisp
(harmony :type :explicit :print-frame :no
  :root (root :step :C)
  :kind (kind :value :minor-seventh :text "m7")
  :bass (bass :step :E :alter -1))
```

**Related Elements:**
- `<root>` — Root pitch of chord
- `<kind>` — Chord quality (major, minor, dominant, etc.)
- `<bass>` — Bass note if different from root
- `<degree>` — Chord alterations (add9, #11, etc.)
- `<inversion>` — Chord inversion number
- `<function>` — Roman numeral/functional analysis (V, ii, IV/V)
- `<numeral>` — Nashville number system

**Type Definition (Preliminary):**
```rust
pub struct Harmony {
    pub r#type: Option<HarmonyType>,  // explicit, implied, alternate
    pub print_object: Option<YesNo>,
    pub print_frame: Option<YesNo>,
    pub arrangement: Option<HarmonyArrangement>,
    // Content
    pub chords: Vec<HarmonyChord>,  // root/function + kind + (inversion|bass) + degree*
    pub frame: Option<Frame>,
    pub offset: Option<Offset>,
    pub staff: Option<StaffNumber>,
}

pub enum HarmonyType { Explicit, Implied, Alternate }
```

---

#### frame (Guitar/Fretboard Diagrams)

Chord diagrams for fretted instruments.

**MusicXML:**
```xml
<frame>
  <frame-strings>6</frame-strings>
  <frame-frets>4</frame-frets>
  <first-fret>1</first-fret>
  <frame-note>
    <string>6</string>
    <fret>0</fret>
  </frame-note>
  <frame-note>
    <string>5</string>
    <fret>3</fret>
    <fingering>1</fingering>
  </frame-note>
</frame>
```

**Provisional S-expr:**
```lisp
(frame
  :strings 6
  :frets 4
  :first-fret 1
  :notes ((frame-note :string 6 :fret 0)
          (frame-note :string 5 :fret 3 :fingering 1)))
```

---

### 10.2 Figured Bass

Baroque-style bass figures for continuo realization.

**MusicXML:**
```xml
<figured-bass>
  <figure>
    <figure-number>6</figure-number>
  </figure>
  <figure>
    <prefix>flat</prefix>
    <figure-number>5</figure-number>
  </figure>
</figured-bass>
```

**Provisional S-expr:**
```lisp
(figured-bass
  :figures ((figure :number 6)
            (figure :prefix :flat :number 5)))
```

**Related Elements:**
- `<figure>` — Single figure in the stack
- `<prefix>` — Accidental before the number
- `<figure-number>` — The number itself
- `<suffix>` — Modifications (slash, backslash for raised/lowered)
- `<extend>` — Continuation line

**Type Definition (Preliminary):**
```rust
pub struct FiguredBass {
    pub figures: Vec<Figure>,
    pub duration: Option<PositiveDivisions>,
    pub parentheses: Option<YesNo>,
}

pub struct Figure {
    pub prefix: Option<StyleText>,  // Accidental or plus
    pub figure_number: Option<StyleText>,
    pub suffix: Option<StyleText>,  // Slash, backslash, etc.
    pub extend: Option<Extend>,
}
```

---

### 10.3 Percussion

#### unpitched

Percussion notation where notes represent instruments rather than pitches.

**MusicXML:**
```xml
<note>
  <unpitched>
    <display-step>E</display-step>
    <display-octave>4</display-octave>
  </unpitched>
  <duration>1</duration>
  <type>quarter</type>
  <instrument id="snare"/>
</note>
```

**Provisional S-expr:**
```lisp
(note
  :unpitched (unpitched :display-step :E :display-octave 4)
  :duration 1
  :type :quarter
  :instrument "snare")
```

#### stick / beater

Indicates the type of mallet or beater to use.

**MusicXML:**
```xml
<direction>
  <direction-type>
    <percussion>
      <stick>
        <stick-type>snare stick</stick-type>
        <stick-material>wood</stick-material>
      </stick>
    </percussion>
  </direction-type>
</direction>
```

**Provisional S-expr:**
```lisp
(direction
  (percussion
    (stick :type :snare-stick :material :wood)))
```

#### Percussion pictograms

Various percussion instrument symbols:
- `<timpani/>` — Timpani pictogram
- `<membrane>` — Membrane instruments (bass drum, snare, etc.)
- `<metal>` — Metal instruments (cymbals, triangle, etc.)
- `<wood>` — Wood instruments (wood block, claves, etc.)
- `<pitched>` — Pitched percussion (xylophone, vibes, etc.)
- `<effect>` — Sound effects

---

### 10.4 Harp

#### harp-pedals

Harp pedal diagram showing the position of all seven pedals.

**MusicXML:**
```xml
<direction>
  <direction-type>
    <harp-pedals>
      <pedal-tuning>
        <pedal-step>D</pedal-step>
        <pedal-alter>-1</pedal-alter>
      </pedal-tuning>
      <pedal-tuning>
        <pedal-step>C</pedal-step>
        <pedal-alter>0</pedal-alter>
      </pedal-tuning>
      <!-- ... B, E, F, G, A pedals ... -->
    </harp-pedals>
  </direction-type>
</direction>
```

**Provisional S-expr:**
```lisp
(direction
  (harp-pedals
    ((pedal :step :D :alter -1)
     (pedal :step :C :alter 0)
     (pedal :step :B :alter 0)
     (pedal :step :E :alter 0)
     (pedal :step :F :alter 0)
     (pedal :step :G :alter 0)
     (pedal :step :A :alter 0))))
```

---

### 10.5 Accordion

#### accordion-registration

Specifies which reed banks are engaged.

**MusicXML:**
```xml
<direction>
  <direction-type>
    <accordion-registration>
      <accordion-high/>
      <accordion-middle>3</accordion-middle>
      <accordion-low/>
    </accordion-registration>
  </direction-type>
</direction>
```

**Provisional S-expr:**
```lisp
(direction
  (accordion-registration
    :high t
    :middle 3
    :low t))
```

---

### 10.6 Tablature

#### Guitar Tablature

Fret/string notation for guitar and other fretted instruments.

**MusicXML (on note):**
```xml
<note>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>1</duration>
  <notations>
    <technical>
      <string>1</string>
      <fret>5</fret>
    </technical>
  </notations>
</note>
```

**Provisional S-expr:**
```lisp
(note
  :pitch (pitch :step :E :octave 4)
  :duration 1
  :notations (notations
    (technical
      :string 1
      :fret 5)))
```

---

### 10.7 Scordatura

Altered tuning for stringed instruments.

**MusicXML:**
```xml
<direction>
  <direction-type>
    <scordatura>
      <accord string="1">
        <tuning-step>E</tuning-step>
        <tuning-alter>-1</tuning-alter>
        <tuning-octave>5</tuning-octave>
      </accord>
    </scordatura>
  </direction-type>
</direction>
```

**Provisional S-expr:**
```lisp
(direction
  (scordatura
    ((accord :string 1
       :tuning (pitch :step :E :alter -1 :octave 5)))))
```

---

### 10.8 Early Music / Historical Notation

#### mensural / square notation

Elements for historical notation styles.

**Relevant elements:**
- `<staff-type>` — Can be `mensural`, `neume`, `TAB`
- Historical note values: `<maxima>`, `<long>`, `<breve>`
- Special barlines for mensural music

#### senza-misura

Indicates unmeasured/free rhythm notation.

**MusicXML:**
```xml
<attributes>
  <time>
    <senza-misura/>
  </time>
</attributes>
```

**Provisional S-expr:**
```lisp
(attributes
  :time (time :senza-misura t))
```

---

### 10.9 Playback and MIDI

#### sound

Controls playback parameters.

**MusicXML:**
```xml
<sound tempo="120" dynamics="75"/>
<sound dacapo="yes"/>
<sound fine="yes"/>
<sound coda="coda1"/>
```

**Provisional S-expr:**
```lisp
(sound :tempo 120 :dynamics 75)
(sound :dacapo :yes)
(sound :fine :yes)
(sound :coda "coda1")
```

#### play

Note-level playback instructions.

**MusicXML (on note):**
```xml
<note>
  <!-- ... pitch/duration ... -->
  <play>
    <ipa>a</ipa>
  </play>
</note>

<note>
  <!-- ... pitch/duration ... -->
  <play>
    <mute>straight</mute>
  </play>
</note>
```

#### midi-instrument / midi-device

MIDI-specific settings.

**MusicXML:**
```xml
<midi-instrument id="P1-I1">
  <midi-channel>1</midi-channel>
  <midi-program>1</midi-program>
  <volume>80</volume>
  <pan>0</pan>
</midi-instrument>
```

---

### 10.10 Accessibility and Machine Listening

#### listen

Instructions for score-following applications.

**MusicXML (on note):**
```xml
<note>
  <!-- ... -->
  <listen>
    <assess type="no" player="conductor"/>
    <wait player="soloist"/>
  </listen>
</note>
```

#### listening

Measure-level listening instructions.

**MusicXML:**
```xml
<listening>
  <sync type="tempo" player="P1"/>
</listening>
```

---

### 10.11 Grouping and Analysis

#### grouping

For musicological analysis.

**MusicXML:**
```xml
<grouping type="start" number="1" member-of="theme1">
  <feature type="phrase">A</feature>
</grouping>
```

**Provisional S-expr:**
```lisp
(grouping :type :start :number 1 :member-of "theme1"
  (feature :type "phrase" :value "A"))
```

---

### 10.12 Other Deferred Elements

#### print

Page/system layout control.

**MusicXML:**
```xml
<print new-page="yes" new-system="yes">
  <system-layout>
    <system-margins>
      <left-margin>70</left-margin>
      <right-margin>0</right-margin>
    </system-margins>
    <top-system-distance>211</top-system-distance>
  </system-layout>
</print>
```

#### credit

Title page and header/footer content.

**MusicXML:**
```xml
<credit page="1">
  <credit-type>title</credit-type>
  <credit-words font-size="24">Symphony No. 5</credit-words>
</credit>
```

#### defaults

Score-wide default settings for fonts, layout, etc.

---

## Summary Table

| Category | Elements | Priority | Notes |
|----------|----------|----------|-------|
| Chord Symbols | harmony, root, kind, bass, degree, function, numeral | Medium | Common in lead sheets |
| Fret Diagrams | frame, frame-note, barre | Medium | Guitar/ukulele music |
| Figured Bass | figured-bass, figure, prefix, suffix | Low | Baroque music |
| Percussion | unpitched, stick, beater, timpani, membrane, etc. | Medium | Drum notation |
| Harp | harp-pedals, pedal-tuning | Low | Orchestral harp |
| Accordion | accordion-registration | Low | Specialized |
| Tablature | string, fret (in technical) | Medium | Guitar music |
| Scordatura | scordatura, accord | Low | String music |
| Early Music | mensural elements, senza-misura | Low | Historical editions |
| Playback | sound, play, midi-* | Medium | DAW integration |
| Accessibility | listen, listening, assess | Low | Score following |
| Analysis | grouping, feature | Low | Musicology |
| Layout | print, credit, defaults | Medium | Publishing |

---

## Design Principles for Future Mappings

When detailed mappings are needed for these elements:

1. **Follow existing patterns** — Use the same keyword-value pairs, nesting, and naming conventions established in Chunks 1-9.

2. **Preserve MusicXML semantics** — Don't merge distinct concepts; maintain lossless round-trip capability.

3. **Use structured forms** — Complex elements get nested forms; simple values become keywords.

4. **Document edge cases** — These specialized elements often have subtle requirements.

5. **Consider playback vs. notation** — Many elements affect playback only; IR should capture both aspects.

---

## Open Questions

- [ ] **Chord symbol representation** — Should IR use structured chord data or preserve text-based shortcuts like "Cm7"?

- [ ] **Tablature priority** — Guitar tablature is common; should it be moved to an earlier tier?

- [ ] **MIDI data** — How much MIDI-specific data should IR preserve vs. compute on export?

- [ ] **Print/layout elements** — Should these be separate from musical content or interleaved as in MusicXML?

- [ ] **Percussion mapping** — Need instrument-to-line mapping for common percussion setups.

---

## Next Steps

1. **Usage analysis** — Examine real MusicXML files to determine actual usage frequency of these elements.

2. **Prioritization** — Based on Fermata's target use cases, select elements for detailed mapping in future iterations.

3. **Instrument support** — Define which instruments Fermata will support and prioritize their specific elements.

4. **Playback strategy** — Decide how much playback control to expose vs. compute automatically.
