---
number: 3
title: "MusicXML Schema Triage for Fermata"
author: "making voices"
component: All
tags: [change-me]
created: 2026-01-31
updated: 2026-01-31
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# MusicXML Schema Triage for Fermata

> Analysis of MusicXML 4.1 XSD schema for prioritizing S-expression DSL implementation.

## 1. Executive Summary

### Schema Statistics

| Category | Count |
|----------|-------|
| Element definitions | ~480 |
| Complex types | ~224 |
| Simple types | ~145 |
| Attribute groups | ~45 |
| Element groups | ~27 |

### Major Conceptual Areas

1. **Score Structure** - Document hierarchy (score-partwise, part, measure)
2. **Note Content** - Pitches, rests, durations, accidentals
3. **Attributes** - Key signatures, time signatures, clefs, transposition
4. **Directions** - Dynamics, tempo, text, rehearsal marks
5. **Notations** - Slurs, ties, articulations, ornaments, technical marks
6. **Harmony** - Chord symbols, figured bass, Roman numerals
7. **Lyrics** - Text underlays for vocal music
8. **Layout/Print** - Page formatting, system breaks, staff spacing
9. **Sound/MIDI** - Playback parameters, instrument assignments

---

## 2. Priority Categories

### Tier 1: Core (MVP)

**Criteria:** Cannot render even a simple melody without these.

These ~25 elements form the absolute minimum for representing musical notation:

| Element | Parent Context | Description | Notes |
|---------|---------------|-------------|-------|
| `<score-partwise>` | root | Score organized by parts | Primary document structure (preferred over timewise) |
| `<part>` | `<score-partwise>` | A single instrument/voice part | Contains measures; has `id` attribute |
| `<measure>` | `<part>` | A single measure | Contains music-data; has `number` attribute |
| `<attributes>` | `<measure>` | Musical attributes container | Key, time, clef, divisions |
| `<divisions>` | `<attributes>` | Divisions per quarter note | Required for duration math |
| `<key>` | `<attributes>` | Key signature | Traditional or non-traditional |
| `<time>` | `<attributes>` | Time signature | beats/beat-type or senza-misura |
| `<clef>` | `<attributes>` | Clef specification | sign + line + octave-change |
| `<note>` | `<measure>` | A note, rest, or chord tone | Most complex element in schema |
| `<pitch>` | `<note>` | Pitch specification | step + alter + octave |
| `<step>` | `<pitch>` | Note letter name | C, D, E, F, G, A, B |
| `<alter>` | `<pitch>` | Chromatic alteration | Semitones: -2 to +2 |
| `<octave>` | `<pitch>` | Octave number | 0-9 (4 = middle C octave) |
| `<duration>` | `<note>` | Sounding duration | In divisions |
| `<type>` | `<note>` | Notated duration | whole, half, quarter, eighth, 16th, etc. |
| `<rest>` | `<note>` | Rest indicator | Optional display-step/octave |
| `<chord>` | `<note>` | Chord tone indicator | Empty element; note shares timing |
| `<dot>` | `<note>` | Dotted note | Repeatable for double-dot |
| `<accidental>` | `<note>` | Displayed accidental | sharp, flat, natural, etc. |
| `<part-list>` | score-header | Part declarations | Required in header |
| `<score-part>` | `<part-list>` | Part definition | id, part-name, instruments |
| `<part-name>` | `<score-part>` | Part name | Display name |
| `<work>` | score-header | Work identification | Optional title, number |
| `<identification>` | score-header | Creator/rights info | composer, lyricist, etc. |

**Core Groups:**
- `full-note` - pitch/unpitched/rest + chord
- `duration` - duration element
- `music-data` - all measure content
- `score-header` - work, movement, identification, defaults, credits, part-list

**Total:** ~25 elements, ~10 complex types

---

### Tier 2: Important

**Criteria:** Most published sheet music uses these regularly.

| Element | Parent Context | Description | Notes |
|---------|---------------|-------------|-------|
| **Ties & Slurs** |
| `<tie>` | `<note>` | Tie sound | start/stop |
| `<tied>` | `<notations>` | Tie notation | Visual representation |
| `<slur>` | `<notations>` | Slur notation | start/stop/continue |
| **Beaming** |
| `<beam>` | `<note>` | Beam specification | begin/continue/end/forward hook/backward hook |
| `<stem>` | `<note>` | Stem direction | up/down/none/double |
| **Dynamics** |
| `<dynamics>` | `<direction-type>` or `<notations>` | Dynamic marking | p, pp, f, ff, mp, mf, sf, etc. |
| `<wedge>` | `<direction-type>` | Crescendo/decrescendo | Hairpin markings |
| **Articulations** |
| `<articulations>` | `<notations>` | Articulation container | Groups multiple articulations |
| `<staccato>` | `<articulations>` | Staccato dot | |
| `<accent>` | `<articulations>` | Horizontal accent | |
| `<tenuto>` | `<articulations>` | Tenuto line | |
| `<staccatissimo>` | `<articulations>` | Wedge articulation | |
| `<fermata>` | `<notations>` | Fermata/pause | normal, angled, square shapes |
| **Ornaments** |
| `<ornaments>` | `<notations>` | Ornament container | |
| `<trill-mark>` | `<ornaments>` | Trill symbol | |
| `<mordent>` | `<ornaments>` | Mordent | long attribute |
| `<turn>` | `<ornaments>` | Turn ornament | |
| **Directions** |
| `<direction>` | `<measure>` | Direction container | dynamics, tempo, text, etc. |
| `<direction-type>` | `<direction>` | Direction content | What kind of direction |
| `<words>` | `<direction-type>` | Text direction | Tempo words, expressions |
| `<metronome>` | `<direction-type>` | Tempo marking | beat-unit + per-minute |
| `<rehearsal>` | `<direction-type>` | Rehearsal mark | Letters/numbers |
| **Barlines** |
| `<barline>` | `<measure>` | Barline specification | Style, repeats, endings |
| `<bar-style>` | `<barline>` | Barline style | regular, light-light, light-heavy, etc. |
| `<repeat>` | `<barline>` | Repeat sign | forward/backward |
| `<ending>` | `<barline>` | Volta brackets | 1st/2nd endings |
| **Grace Notes** |
| `<grace>` | `<note>` | Grace note indicator | slash, steal-time attributes |
| **Tuplets** |
| `<time-modification>` | `<note>` | Tuplet ratio | actual-notes/normal-notes |
| `<tuplet>` | `<notations>` | Tuplet display | Bracket, numbers |
| **Voice/Staff** |
| `<voice>` | `<note>` | Voice number | For polyphony |
| `<staff>` | `<note>` | Staff number | For grand staff |
| `<backup>` | `<measure>` | Move backward | For multiple voices |
| `<forward>` | `<measure>` | Move forward | Skip duration |
| **Navigation** |
| `<segno>` | `<direction-type>` | Segno sign | |
| `<coda>` | `<direction-type>` | Coda sign | |

**Total:** ~40 elements, ~15 complex types

---

### Tier 3: Secondary

**Criteria:** Needed for specific genres or complex scores.

| Element | Parent Context | Description | Notes |
|---------|---------------|-------------|-------|
| **Lyrics** |
| `<lyric>` | `<note>` | Lyric syllable | Multiple verses via number |
| `<syllabic>` | `<lyric>` | Syllable position | single/begin/middle/end |
| `<text>` | `<lyric>` | Lyric text | The actual text |
| `<elision>` | `<lyric>` | Elision connector | Links syllables |
| `<extend>` | `<lyric>` | Melisma line | |
| **Harmony/Chords** |
| `<harmony>` | `<measure>` | Chord symbol | Root, kind, bass, degrees |
| `<root>` | `<harmony>` | Chord root | step + alter |
| `<kind>` | `<harmony>` | Chord quality | major, minor, dominant, etc. |
| `<bass>` | `<harmony>` | Bass note | For slash chords |
| `<degree>` | `<harmony>` | Chord alterations | add/subtract/alter |
| `<frame>` | `<harmony>` | Chord diagram | Fretboard frames |
| **Figured Bass** |
| `<figured-bass>` | `<measure>` | Figured bass notation | |
| `<figure>` | `<figured-bass>` | Single figure | |
| **Technical** |
| `<technical>` | `<notations>` | Technical marks | Instrument-specific |
| `<fingering>` | `<technical>` | Fingering number | |
| `<string>` | `<technical>` | String number | |
| `<fret>` | `<technical>` | Fret number | |
| `<up-bow>` | `<technical>` | Up bow | |
| `<down-bow>` | `<technical>` | Down bow | |
| `<harmonic>` | `<technical>` | Harmonic | natural/artificial |
| `<pluck>` | `<technical>` | Pluck fingering | p, i, m, a |
| **Advanced Ornaments** |
| `<tremolo>` | `<ornaments>` | Tremolo marks | |
| `<wavy-line>` | `<ornaments>` | Trill line | |
| `<inverted-mordent>` | `<ornaments>` | Inverted mordent | |
| `<shake>` | `<ornaments>` | Shake ornament | |
| **More Articulations** |
| `<strong-accent>` | `<articulations>` | Marcato | |
| `<spiccato>` | `<articulations>` | Spiccato stroke | |
| `<detached-legato>` | `<articulations>` | Portato | |
| `<scoop>` | `<articulations>` | Jazz scoop | |
| `<plop>` | `<articulations>` | Jazz plop | |
| `<doit>` | `<articulations>` | Jazz doit | |
| `<falloff>` | `<articulations>` | Jazz falloff | |
| `<breath-mark>` | `<articulations>` | Breath mark | |
| `<caesura>` | `<articulations>` | Caesura | |
| **Glissando/Slide** |
| `<glissando>` | `<notations>` | Glissando line | |
| `<slide>` | `<notations>` | Slide | |
| **Arpeggios** |
| `<arpeggiate>` | `<notations>` | Arpeggio sign | |
| `<non-arpeggiate>` | `<notations>` | Non-arpeggio bracket | |
| **Multi-Staff** |
| `<staves>` | `<attributes>` | Number of staves | |
| `<part-symbol>` | `<attributes>` | Part brace/bracket | |
| `<staff-details>` | `<attributes>` | Staff tuning, etc. | |
| **Notehead Variants** |
| `<notehead>` | `<note>` | Notehead shape | X, diamond, slash, etc. |
| `<notehead-text>` | `<note>` | Text in notehead | |
| **Percussion** |
| `<unpitched>` | `<note>` | Unpitched note | display-step/octave |
| `<percussion>` | `<direction-type>` | Percussion pictogram | |
| **Transposition** |
| `<transpose>` | `<attributes>` | Transposition | chromatic/diatonic/octave |

**Total:** ~60 elements

---

### Tier 4: Deferred

**Criteria:** Specialized or rare; can be added later without affecting core architecture.

| Category | Elements | Notes |
|----------|----------|-------|
| **Layout/Print** | `<print>`, `<page-layout>`, `<system-layout>`, `<staff-layout>`, `<measure-layout>`, `<appearance>`, `<page-margins>`, `<system-margins>`, `<scaling>` | Visual formatting only |
| **Defaults** | `<defaults>`, `<word-font>`, `<lyric-font>`, `<music-font>`, `<lyric-language>` | Score-wide defaults |
| **Credits** | `<credit>`, `<credit-words>`, `<credit-image>`, `<credit-symbol>` | Title page text |
| **Sound/MIDI** | `<sound>`, `<midi-device>`, `<midi-instrument>`, `<midi-bank>`, `<midi-channel>`, `<midi-program>`, `<play>`, `<listen>`, `<listening>`, `<instrument-change>`, `<swing>` | Playback only |
| **Grouping** | `<grouping>`, `<feature>` | Analysis grouping |
| **Links** | `<link>`, `<bookmark>` | Document navigation |
| **Instrument-Specific** | `<harp-pedals>`, `<accordion-registration>`, `<scordatura>`, `<string-mute>`, `<brass-mute>`, `<glasses>`, `<metal>`, `<membrane>`, `<wood>`, `<beater>`, `<stick>` | Rare instruments |
| **Advanced Pedaling** | `<pedal>`, `<damp>`, `<damp-all>`, `<eyeglasses>` | Piano/harp specific |
| **Tablature** | `<tab>`, `<bend>`, `<hammer-on>`, `<pull-off>`, `<tap>`, `<hole>` | Guitar/fretted |
| **Ancient Notation** | Various Gregorian/mensural elements | Historical |
| **Images** | `<image>` | Embedded graphics |
| **Other Direction Types** | `<principal-voice>`, `<string-mute>`, `<scordatura>`, `<image>`, `<other-direction>` | Miscellaneous |
| **Encoding Details** | `<encoding>`, `<supports>`, `<software>`, `<encoder>`, `<encoding-date>` | Metadata only |

**Total:** ~100+ elements

---

## 3. Element Inventory Details

### Core Note Structure

The `<note>` element is the most complex in MusicXML. Its content model:

```
note =
  (grace, ((full-note, tie*) | (cue, full-note)))
  | (cue, full-note, duration)
  | (full-note, duration, tie*)

  + instrument*
  + editorial-voice
  + type?
  + dot*
  + accidental?
  + time-modification?
  + stem?
  + notehead?
  + notehead-text?
  + staff?
  + beam{0-8}
  + notations*
  + lyric*
  + play?
  + listen?
```

Where `full-note` = `chord?, (pitch | unpitched | rest)`

### music-data Group

Elements that can appear directly in a measure:

1. `<note>` - Notes, rests, chord tones
2. `<backup>` - Move backward in time (for voices)
3. `<forward>` - Move forward in time
4. `<direction>` - Dynamics, tempo, text, etc.
5. `<attributes>` - Key, time, clef changes
6. `<harmony>` - Chord symbols
7. `<figured-bass>` - Figured bass
8. `<print>` - Layout instructions
9. `<sound>` - Playback parameters
10. `<listening>` - Listening exam info
11. `<barline>` - Barline specifications
12. `<grouping>` - Analysis grouping
13. `<link>` - Hyperlinks
14. `<bookmark>` - Named locations

---

## 4. Complex Types Summary

### Most Widely Reused Types

| Type | Used By | Purpose |
|------|---------|---------|
| `empty` | ~30 elements | Elements with no content |
| `empty-placement` | ~25 elements | Empty + print-style + placement |
| `formatted-text` | ~15 elements | Text with formatting |
| `placement-text` | ~10 elements | Text with placement |
| `start-stop` | Many attributes | Begin/end marking |
| `start-stop-continue` | Slurs, wedges | Three-state marking |
| `yes-no` | ~100 attributes | Boolean values |

### Types with Many Optional Children

| Type | Optional Children | Notes |
|------|-------------------|-------|
| `note` | 15+ optional elements | Most complex |
| `direction-type` | 20+ choices | Very extensible |
| `attributes` | 10+ optional elements | Key, time, clef, etc. |
| `notations` | 14 optional elements | Articulations, ornaments, etc. |
| `sound` | 10+ attributes | Playback parameters |
| `harmony` | Many sub-elements | Chord specification |

---

## 5. Attribute Groups

### Position Attributes

```xml
<xs:attributeGroup name="position">
  <xs:attribute name="default-x" type="tenths"/>
  <xs:attribute name="default-y" type="tenths"/>
  <xs:attribute name="relative-x" type="tenths"/>
  <xs:attribute name="relative-y" type="tenths"/>
</xs:attributeGroup>
```

**Used by:** Most visual elements. Fermata should likely have a unified `:position` keyword or allow these as options.

### Font Attributes

```xml
<xs:attributeGroup name="font">
  <xs:attribute name="font-family" type="font-family"/>
  <xs:attribute name="font-style" type="font-style"/> <!-- normal | italic -->
  <xs:attribute name="font-size" type="font-size"/>
  <xs:attribute name="font-weight" type="font-weight"/> <!-- normal | bold -->
</xs:attributeGroup>
```

### Print-Style Group

```xml
<xs:attributeGroup name="print-style">
  <xs:attributeGroup ref="position"/>
  <xs:attributeGroup ref="font"/>
  <xs:attributeGroup ref="color"/>
</xs:attributeGroup>
```

**Recommendation:** These three groups appear together frequently. Consider a unified `:style` option in Fermata that expands to these.

### Common Attribute Patterns

| Pattern | Attributes | Usage |
|---------|------------|-------|
| `placement` | `placement="above\|below"` | Articulations, dynamics |
| `orientation` | `orientation="over\|under"` | Slurs, ties |
| `line-type` | `line-type="solid\|dashed\|dotted\|wavy"` | Lines |
| `print-object` | `print-object="yes\|no"` | Hide elements |
| `optional-unique-id` | `id="..."` | XML ID for linking |

---

## 6. Observations & Warnings

### Elements That Seem Redundant

1. **`<tie>` vs `<tied>`** - `<tie>` is for sound (MIDI), `<tied>` is for notation (visual). Need both for complete representation.

2. **`<score-timewise>` vs `<score-partwise>`** - Same content, different hierarchy. Use partwise; timewise is deprecated.

3. **`<grace>` creates three note variants** - Regular notes, grace notes, and cue notes have different content models. The choice of (grace+full-note | cue+full-note+duration | full-note+duration) is complex.

### Elements with Unclear Purpose

1. **`<listening>`** - For listening exams; very specialized.
2. **`<grouping>`** - For analytical grouping; unclear use cases.
3. **`<feature>`** - Part of grouping; unclear.

### Tricky S-Expression Mappings

1. **Chord representation** - MusicXML uses `<chord/>` on subsequent notes; S-expr might want explicit grouping: `(chord :q (c4 e4 g4))`

2. **Voice/Staff assignment** - MusicXML has `<voice>` and `<staff>` elements; need clear S-expr syntax.

3. **Duration vs Type** - MusicXML has both sounding `<duration>` (in divisions) and notated `<type>` (quarter, eighth). S-expr should probably use notated type and compute duration.

4. **Divisions** - Divisions per quarter note varies; Fermata IR should normalize to a fixed division (e.g., 960 per quarter).

5. **Backup/Forward** - Used for multiple voices. S-expr should have explicit voice containers to avoid needing these.

6. **Nested notations** - `<notations>` contains `<articulations>` which contains individual marks. Flatten in S-expr?

### Complex Interdependencies

1. **Part references** - `<part>` elements reference `<score-part>` by ID. Need to maintain mapping.

2. **Instrument references** - Notes can reference instruments via `<instrument>` element ID.

3. **Beam groups** - Beam start/continue/end must be balanced. Consider auto-beaming in Fermata.

4. **Tuplets** - `<time-modification>` (sound) and `<tuplet>` (notation) must agree.

5. **Cross-staff notes** - `<staff>` element can differ from expected staff. Complex for piano.

### Implementation Recommendations

1. **Start with score-partwise** - Ignore timewise completely.

2. **Normalize divisions** - Use fixed internal division (e.g., 960 per quarter = LCM of common subdivisions).

3. **Explicit voice containers** - Avoid backup/forward by making voices explicit in S-expr.

4. **Defer layout** - Ignore print/layout elements initially; let rendering engine handle.

5. **Chord syntax** - Use explicit chord grouping: `(chord :q (c4 e4 g4))` rather than MusicXML's implicit model.

6. **Auto-beaming** - Generate beams automatically by default; allow override.

---

## 7. Suggested Fermata S-Expression Mapping

Based on this analysis, here's a proposed minimal syntax for Tier 1:

```lisp
(score
  (title "Untitled")
  (composer "Anonymous")

  (part :piano
    (staff :treble
      (measure
        (key c :major)
        (time 4 4)
        (note c4 :q)
        (note d4 :q)
        (note e4 :q)
        (note f4 :q)))))
```

Expanding to Tier 2:

```lisp
(score
  (tempo 120 :q)

  (part :violin
    (staff :treble
      (measure
        (key g :major)
        (time 3 4)
        (note g4 :q :mf (slur :start))
        (note a4 :q)
        (note b4 :q (slur :stop))))))
```

This maps cleanly to MusicXML while being more readable.

---

*Generated from MusicXML 4.1 XSD analysis*
