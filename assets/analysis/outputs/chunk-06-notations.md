# Chunk 6: Notations

## Overview

Notations are musical symbols attached to specific notes: articulations, ornaments, technical indications, and more. They appear within the `<notations>` element inside `<note>`.

**Key Concept:** The `<notations>` wrapper can contain multiple elements from different categories (articulations, ornaments, technical, etc.). A note can have multiple `<notations>` elements for different editorial levels.

**Important:** Some elements covered in earlier chunks (tied, slur, tuplet) also live in `<notations>` but were documented there for context.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Key Attributes |
|---------|------|-------------------|-------------------|----------------|
| `<notations>` | complex | — | many categories | `print-object` |
| `<articulations>` | complex | — | many | — |
| `<ornaments>` | complex | — | many | — |
| `<technical>` | complex | — | many | — |
| `<fermata>` | complex | — | — | `type`, `shape` |
| `<arpeggiate>` | complex | — | — | `number`, `direction` |
| `<non-arpeggiate>` | complex | — | — | `type`, `number` |
| `<glissando>` | complex | text | — | `type`, `number`, `line-type` |
| `<slide>` | complex | text | — | `type`, `number`, `line-type` |
| `<accidental-mark>` | complex | value | — | placement, position |
| `<other-notation>` | complex | text | — | `type`, `number` |

## S-Expression Mappings

### notations

Container for note-attached notations.

**MusicXML:**
```xml
<note>
  <pitch>...</pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <articulations>
      <staccato/>
      <accent/>
    </articulations>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations
                (articulations
                  (staccato)
                  (accent)))))
```

**Mapping Rules:**
- `<notations>` → `(notations ...)` form
- Multiple notations elements → list of notation forms
- `print-object` attr → `:print-object`
- Contains editorial group + choice of notation elements

**Type Definition:**
```rust
pub struct Notations {
    pub print_object: Option<YesNo>,
    pub editorial: Editorial,
    pub content: Vec<NotationContent>,
}

pub enum NotationContent {
    Tied(Tied),
    Slur(Slur),
    Tuplet(Tuplet),
    Glissando(Glissando),
    Slide(Slide),
    Ornaments(Ornaments),
    Technical(Technical),
    Articulations(Articulations),
    Dynamics(Dynamics),
    Fermata(Fermata),
    Arpeggiate(Arpeggiate),
    NonArpeggiate(NonArpeggiate),
    AccidentalMark(AccidentalMark),
    OtherNotation(OtherNotation),
}
```

---

### articulations

Container for articulation marks.

**MusicXML:**
```xml
<articulations>
  <accent placement="above"/>
  <staccato/>
  <tenuto/>
</articulations>
```

**S-expr:**
```lisp
(articulations
  (accent :placement :above)
  (staccato)
  (tenuto))
```

**Available Articulations:**

| Element | Description | S-expr |
|---------|-------------|--------|
| `<accent>` | Horizontal accent (>) | `(accent)` |
| `<strong-accent>` | Vertical accent/marcato (^) | `(strong-accent :type :up)` |
| `<staccato>` | Staccato dot | `(staccato)` |
| `<tenuto>` | Tenuto line | `(tenuto)` |
| `<detached-legato>` | Portato (tenuto + staccato) | `(detached-legato)` |
| `<staccatissimo>` | Wedge/very short | `(staccatissimo)` |
| `<spiccato>` | Spiccato stroke | `(spiccato)` |
| `<scoop>` | Jazz scoop | `(scoop)` |
| `<plop>` | Jazz plop | `(plop)` |
| `<doit>` | Jazz doit | `(doit)` |
| `<falloff>` | Jazz falloff | `(falloff)` |
| `<breath-mark>` | Breath mark | `(breath-mark)` |
| `<caesura>` | Caesura | `(caesura)` |
| `<stress>` | Stress | `(stress)` |
| `<unstress>` | Unstress | `(unstress)` |
| `<soft-accent>` | Soft accent | `(soft-accent)` |
| `<other-articulation>` | Custom | `(other-articulation "text")` |

**Mapping Rules:**
- Most are empty elements with placement
- `<strong-accent>` has `type` (`:up` or `:down`)
- `<breath-mark>` has breath-mark-value content
- `<caesura>` has caesura-value content

**Type Definition:**
```rust
pub struct Articulations {
    pub content: Vec<ArticulationElement>,
}

pub enum ArticulationElement {
    Accent(EmptyPlacement),
    StrongAccent(StrongAccent),
    Staccato(EmptyPlacement),
    Tenuto(EmptyPlacement),
    DetachedLegato(EmptyPlacement),
    Staccatissimo(EmptyPlacement),
    Spiccato(EmptyPlacement),
    Scoop(EmptyLine),
    Plop(EmptyLine),
    Doit(EmptyLine),
    Falloff(EmptyLine),
    BreathMark(BreathMark),
    Caesura(Caesura),
    Stress(EmptyPlacement),
    Unstress(EmptyPlacement),
    SoftAccent(EmptyPlacement),
    OtherArticulation(OtherText),
}
```

---

### ornaments

Container for ornament marks.

**MusicXML:**
```xml
<ornaments>
  <trill-mark/>
  <accidental-mark>sharp</accidental-mark>
</ornaments>

<ornaments>
  <turn/>
</ornaments>

<ornaments>
  <mordent/>
</ornaments>
```

**S-expr:**
```lisp
(ornaments
  (trill-mark)
  (accidental-mark :value :sharp))

(ornaments
  (turn))

(ornaments
  (mordent))
```

**Available Ornaments:**

| Element | Description | S-expr |
|---------|-------------|--------|
| `<trill-mark>` | Trill symbol | `(trill-mark)` |
| `<turn>` | Turn (upper-lower) | `(turn)` |
| `<delayed-turn>` | Delayed turn | `(delayed-turn)` |
| `<inverted-turn>` | Inverted turn (lower-upper) | `(inverted-turn)` |
| `<delayed-inverted-turn>` | Delayed inverted | `(delayed-inverted-turn)` |
| `<vertical-turn>` | Vertical turn | `(vertical-turn)` |
| `<inverted-vertical-turn>` | Inverted vertical | `(inverted-vertical-turn)` |
| `<shake>` | Shake | `(shake)` |
| `<wavy-line>` | Trill extension | `(wavy-line :type :start)` |
| `<mordent>` | Mordent | `(mordent)` |
| `<inverted-mordent>` | Inverted mordent | `(inverted-mordent)` |
| `<schleifer>` | Baroque slide | `(schleifer)` |
| `<tremolo>` | Tremolo marks | `(tremolo :type :single 3)` |
| `<haydn>` | Haydn turn | `(haydn)` |
| `<other-ornament>` | Custom | `(other-ornament "text")` |

**Mapping Rules:**
- Ornaments can be followed by `<accidental-mark>` for auxiliary note
- `<tremolo>` has content (number of marks 1-8) and type
- `<wavy-line>` uses start/stop/continue
- Playback attrs: `start-note`, `trill-step`, `two-note-turn`, etc.

**Type Definition:**
```rust
pub struct Ornaments {
    pub content: Vec<OrnamentWithAccidentals>,
}

pub struct OrnamentWithAccidentals {
    pub ornament: OrnamentElement,
    pub accidental_marks: Vec<AccidentalMark>,
}

pub enum OrnamentElement {
    TrillMark(EmptyTrillSound),
    Turn(HorizontalTurn),
    DelayedTurn(HorizontalTurn),
    InvertedTurn(HorizontalTurn),
    DelayedInvertedTurn(HorizontalTurn),
    VerticalTurn(EmptyTrillSound),
    InvertedVerticalTurn(EmptyTrillSound),
    Shake(EmptyTrillSound),
    WavyLine(WavyLine),
    Mordent(Mordent),
    InvertedMordent(Mordent),
    Schleifer(EmptyPlacement),
    Tremolo(Tremolo),
    Haydn(EmptyTrillSound),
    OtherOrnament(OtherText),
}
```

---

### technical

Container for technical performance indications.

**MusicXML:**
```xml
<technical>
  <fingering>3</fingering>
</technical>

<technical>
  <up-bow/>
</technical>

<technical>
  <string>1</string>
  <fret>5</fret>
</technical>
```

**S-expr:**
```lisp
(technical
  (fingering "3"))

(technical
  (up-bow))

(technical
  (string 1)
  (fret 5))
```

**Available Technical Elements:**

| Element | Description | S-expr |
|---------|-------------|--------|
| `<up-bow>` | Up bow | `(up-bow)` |
| `<down-bow>` | Down bow | `(down-bow)` |
| `<harmonic>` | Harmonic | `(harmonic :natural t)` |
| `<open-string>` | Open string | `(open-string)` |
| `<thumb-position>` | Thumb position | `(thumb-position)` |
| `<fingering>` | Fingering | `(fingering "3")` |
| `<pluck>` | Pluck finger | `(pluck "p")` |
| `<double-tongue>` | Double tongue | `(double-tongue)` |
| `<triple-tongue>` | Triple tongue | `(triple-tongue)` |
| `<stopped>` | Stopped/muted | `(stopped)` |
| `<snap-pizzicato>` | Bartók pizz | `(snap-pizzicato)` |
| `<fret>` | Fret number | `(fret 5)` |
| `<string>` | String number | `(string 1)` |
| `<hammer-on>` | Hammer-on | `(hammer-on :type :start "H")` |
| `<pull-off>` | Pull-off | `(pull-off :type :start "P")` |
| `<bend>` | Bend | `(bend :bend-alter 1)` |
| `<tap>` | Tap | `(tap)` |
| `<heel>` | Heel (organ) | `(heel)` |
| `<toe>` | Toe (organ) | `(toe)` |
| `<fingernails>` | Fingernails | `(fingernails)` |
| `<hole>` | Woodwind hole | `(hole ...)` |
| `<arrow>` | Arrow | `(arrow ...)` |
| `<handbell>` | Handbell | `(handbell "ring")` |
| `<brass-bend>` | Brass bend | `(brass-bend)` |
| `<flip>` | Brass flip | `(flip)` |
| `<smear>` | Brass smear | `(smear)` |
| `<open>` | Open string/mute | `(open)` |
| `<half-muted>` | Half muted | `(half-muted)` |
| `<golpe>` | Flamenco golpe | `(golpe)` |
| `<other-technical>` | Custom | `(other-technical "text")` |

**Mapping Rules:**
- `<fingering>` content is the finger number/text
- `<string>` and `<fret>` are integers
- `<hammer-on>`/`<pull-off>` have type and text content
- `<harmonic>` has sub-elements for natural/artificial

**Type Definition:**
```rust
pub struct Technical {
    pub content: Vec<TechnicalElement>,
}

pub enum TechnicalElement {
    UpBow(EmptyPlacement),
    DownBow(EmptyPlacement),
    Harmonic(Harmonic),
    OpenString(EmptyPlacement),
    ThumbPosition(EmptyPlacement),
    Fingering(Fingering),
    Pluck(PlacementText),
    DoubleTongue(EmptyPlacement),
    TripleTongue(EmptyPlacement),
    Stopped(EmptyPlacementSmufl),
    SnapPizzicato(EmptyPlacement),
    Fret(Fret),
    String(MusicString),
    HammerOn(HammerOnPullOff),
    PullOff(HammerOnPullOff),
    Bend(Bend),
    Tap(Tap),
    Heel(HeelToe),
    Toe(HeelToe),
    Fingernails(EmptyPlacement),
    Hole(Hole),
    Arrow(Arrow),
    Handbell(Handbell),
    BrassBend(EmptyPlacement),
    Flip(EmptyPlacement),
    Smear(EmptyPlacement),
    Open(EmptyPlacementSmufl),
    HalfMuted(EmptyPlacementSmufl),
    Golpe(EmptyPlacement),
    OtherTechnical(OtherText),
}
```

---

### fermata

Pause/hold marking.

**MusicXML:**
```xml
<notations>
  <fermata type="upright"/>
</notations>

<notations>
  <fermata type="inverted" shape="angled"/>
</notations>
```

**S-expr:**
```lisp
(fermata :type :upright)

(fermata :type :inverted :shape :angled)
```

**Mapping Rules:**
- `type` → `:type` (`:upright` or `:inverted`)
- `shape` → `:shape` (`:normal`, `:angled`, `:square`, `:double-angled`, `:double-square`, `:double-dot`, `:half-curve`, `:curlew`)
- Content can be fermata-shape value (alternative to attribute)

**Type Definition:**
```rust
pub struct Fermata {
    pub shape: Option<FermataShape>,
    pub r#type: Option<UprightInverted>,
    // print-style...
}

pub enum FermataShape {
    Normal, Angled, Square,
    DoubleAngled, DoubleSquare, DoubleDot,
    HalfCurve, Curlew,
}
```

---

### arpeggiate

Arpeggio marking on a chord.

**MusicXML:**
```xml
<!-- On each note of the chord -->
<note>
  <pitch>...</pitch>
  <notations>
    <arpeggiate number="1" direction="up"/>
  </notations>
</note>
<note>
  <chord/>
  <pitch>...</pitch>
  <notations>
    <arpeggiate number="1"/>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :notations ((notations
                (arpeggiate :number 1 :direction :up))))

(note
  :chord t
  :pitch (pitch :step :E :octave 4)
  :notations ((notations
                (arpeggiate :number 1))))
```

**Mapping Rules:**
- `number` → `:number` (groups notes in arpeggio)
- `direction` → `:direction` (`:up`, `:down`, or omit for wavy line)
- Should appear on all chord notes with same number

**Type Definition:**
```rust
pub struct Arpeggiate {
    pub number: Option<NumberLevel>,
    pub direction: Option<UpDown>,
    pub unbroken: Option<YesNo>,
    // position, placement, color...
}
```

---

### non-arpeggiate

Bracket indicating chord should not be arpeggiated.

**MusicXML:**
```xml
<note>
  <pitch>...</pitch>
  <notations>
    <non-arpeggiate type="bottom"/>
  </notations>
</note>
<note>
  <chord/>
  <pitch>...</pitch>
  <notations>
    <non-arpeggiate type="top"/>
  </notations>
</note>
```

**S-expr:**
```lisp
(non-arpeggiate :type :bottom)
(non-arpeggiate :type :top)
```

**Mapping Rules:**
- `type` → `:type` (`:top` or `:bottom`)
- `number` → `:number` (for multiple brackets)

---

### glissando and slide

Lines between notes.

**MusicXML:**
```xml
<!-- On first note -->
<notations>
  <glissando type="start" line-type="wavy">gliss.</glissando>
</notations>

<!-- On second note -->
<notations>
  <glissando type="stop"/>
</notations>
```

**S-expr:**
```lisp
;; Start
(glissando :type :start :line-type :wavy "gliss.")

;; Stop
(glissando :type :stop)
```

**Mapping Rules:**
- `type` → `:type` (`:start`, `:stop`, `:continue`)
- `number` → `:number` (for multiple glissandos)
- `line-type` → `:line-type` (`:solid`, `:dashed`, `:dotted`, `:wavy`)
- Text content → final argument (optional text label)
- `<slide>` is similar but always solid line

---

### accidental-mark

Accidental shown independently of the note's pitch.

**MusicXML:**
```xml
<ornaments>
  <trill-mark/>
  <accidental-mark placement="above">sharp</accidental-mark>
</ornaments>
```

**S-expr:**
```lisp
(ornaments
  (trill-mark)
  (accidental-mark :placement :above :value :sharp))
```

**Mapping Rules:**
- Content is accidental-value (sharp, flat, natural, etc.)
- Often follows ornaments to show auxiliary note pitch
- Same values as `<accidental>` element

---

## Interrelationships

1. **articulations ↔ note position:** Articulations are placed relative to the note.

2. **ornaments ↔ accidental-mark:** Ornaments can include accidental marks for the auxiliary pitch.

3. **arpeggiate ↔ chord:** Arpeggio marking must appear on all chord notes with same number.

4. **glissando/slide start ↔ stop:** Must match by number across notes.

5. **fermata ↔ barline:** Fermatas can also appear on barlines (not in notations).

6. **technical ↔ instrument:** Some technical marks are instrument-specific.

---

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Preserve notations wrapper | Yes | Allows multiple notation groups per note |
| Articulation elements | Empty forms with placement | Matches MusicXML; simple |
| Ornament grouping | Ornament + accidental-marks | MusicXML pattern for auxiliary notes |
| Technical variety | All elements available | Different instruments need different marks |
| Fermata shape | Attribute, not content | More flexible |

---

## Open Questions

- [ ] **Multiple notations elements:** When to use multiple `<notations>` vs. one with multiple children?

- [ ] **Ornament playback:** Include start-note, trill-step attrs in IR or defer?

- [ ] **Technical elements:** Complete coverage vs. defer uncommon ones?

---

## Examples

### Example 1: Staccato and Accent

**Musical meaning:** Short, emphasized note

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>2</duration>
  <type>eighth</type>
  <notations>
    <articulations>
      <staccato placement="above"/>
      <accent placement="above"/>
    </articulations>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 2
  :type :eighth
  :notations ((notations
                (articulations
                  (staccato :placement :above)
                  (accent :placement :above)))))
```

---

### Example 2: Trill with Accidental

**Musical meaning:** Trill to the upper note (sharp)

**MusicXML:**
```xml
<note>
  <pitch><step>E</step><octave>5</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <ornaments>
      <trill-mark/>
      <accidental-mark placement="above">sharp</accidental-mark>
    </ornaments>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :E :octave 5)
  :duration 4
  :type :quarter
  :notations ((notations
                (ornaments
                  (trill-mark)
                  (accidental-mark :placement :above :value :sharp)))))
```

---

### Example 3: Fingering

**Musical meaning:** Play with finger 3

**MusicXML:**
```xml
<note>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <technical>
      <fingering placement="above">3</fingering>
    </technical>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :G :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations
                (technical
                  (fingering :placement :above "3")))))
```

---

### Example 4: Fermata

**Musical meaning:** Hold the note

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>5</octave></pitch>
  <duration>4</duration>
  <type>whole</type>
  <notations>
    <fermata type="upright"/>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 5)
  :duration 4
  :type :whole
  :notations ((notations
                (fermata :type :upright))))
```

---

### Example 5: Arpeggiated Chord

**Musical meaning:** Roll the chord upward

**MusicXML:**
```xml
<note>
  <pitch><step>C</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <arpeggiate direction="up"/>
  </notations>
</note>
<note>
  <chord/>
  <pitch><step>E</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <arpeggiate/>
  </notations>
</note>
<note>
  <chord/>
  <pitch><step>G</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <arpeggiate/>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations (arpeggiate :direction :up))))
(note
  :chord t
  :pitch (pitch :step :E :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations (arpeggiate))))
(note
  :chord t
  :pitch (pitch :step :G :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations (arpeggiate))))
```

---

### Example 6: Guitar Tablature Notation

**Musical meaning:** Play on string 2, fret 3

**MusicXML:**
```xml
<note>
  <pitch><step>D</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <technical>
      <string>2</string>
      <fret>3</fret>
    </technical>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :D :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations
                (technical
                  (string 2)
                  (fret 3)))))
```

---

### Example 7: Mordent

**Musical meaning:** Quick alternation with lower note

**MusicXML:**
```xml
<note>
  <pitch><step>A</step><octave>4</octave></pitch>
  <duration>4</duration>
  <type>quarter</type>
  <notations>
    <ornaments>
      <mordent long="yes"/>
    </ornaments>
  </notations>
</note>
```

**S-expr:**
```lisp
(note
  :pitch (pitch :step :A :octave 4)
  :duration 4
  :type :quarter
  :notations ((notations
                (ornaments
                  (mordent :long :yes)))))
```
