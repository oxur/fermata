---
number: 4
title: "Fermata IR S-Expression Format Reference"
author: "Duncan McGreggor"
component: All
tags: [ir, s-expression, phase-4, reference]
created: 2026-02-01
updated: 2026-02-01
state: Final
supersedes: null
superseded-by: null
version: 1.1
---

# Fermata IR S-Expression Format Reference

> **Purpose:** This document defines the S-expression format that the Music IR uses. Phase 5 (Fermata Syntax) must compile down to this format.
>
> **This is the contract:** Any ergonomic Fermata syntax must desugar to valid IR S-expressions as defined here.

---

## Format Conventions

### General Structure

All IR S-expressions follow this pattern:

```lisp
(type-name :keyword value :keyword value ...)
```

- **Head symbol** — Always the type name in kebab-case
- **Keyword arguments** — All arguments are keyword-based (`:keyword value`)
- **No positional arguments** — Everything is explicit
- **Optional fields** — Omitted when `None`/empty (not `:field nil`)

### Data Types

| Rust Type | S-expr Representation |
|-----------|----------------------|
| `String` | `"quoted string"` |
| `bool` | `t` or `nil` |
| `i32`, `u32`, `f64` | `123`, `-5`, `3.14` |
| `Option<T>` | Omit keyword entirely if `None` |
| `Vec<T>` | `(item1 item2 item3)` |
| Enum variant | Lowercase kebab-case symbol: `start`, `quarter`, `forward-hook` |
| Newtype/type alias | Unwrapped value: `Octave` (u8) → `4` |

---

## Core Musical Types

### Pitch

```lisp
(pitch :step C :octave 4)
(pitch :step F :alter 1.0 :octave 4)      ; F#4
(pitch :step B :alter -1.0 :octave 3)     ; Bb3
(pitch :step C :alter 0.5 :octave 4)      ; C quarter-sharp (microtone)
```

| Field | Type | Required | Values |
|-------|------|----------|--------|
| `:step` | symbol | ✓ | `A` `B` `C` `D` `E` `F` `G` |
| `:alter` | number | | Semitones: `-2.0` to `2.0` (supports decimals) |
| `:octave` | integer | ✓ | `0` to `9` (4 = middle C octave) |

### Rest

```lisp
(rest)
(rest :display-step E :display-octave 4)  ; Positioned rest
(rest :measure t)                          ; Whole-measure rest
```

### Unpitched

```lisp
(unpitched)
(unpitched :display-step E :display-octave 4)
```

---

## Duration Types

### NoteType

```lisp
(note-type :value quarter)
(note-type :value eighth :size cue)
```

| `:value` values | Description |
|-----------------|-------------|
| `n1024th` | 1024th note |
| `n512th` | 512th note |
| `n256th` | 256th note |
| `n128th` | 128th note |
| `n64th` | 64th note |
| `n32nd` | 32nd note |
| `n16th` | 16th note |
| `eighth` | Eighth note |
| `quarter` | Quarter note |
| `half` | Half note |
| `whole` | Whole note |
| `breve` | Breve (2 whole notes) |
| `long` | Long (4 whole notes) |
| `maxima` | Maxima (8 whole notes) |

| `:size` values | Description |
|----------------|-------------|
| `full` | Normal size |
| `cue` | Cue size |
| `grace-cue` | Grace cue size |
| `large` | Large size |

### Dot

```lisp
(dot)
(dot :placement above)
```

### TimeModification (Tuplets)

```lisp
(time-modification :actual-notes 3 :normal-notes 2)  ; Triplet
(time-modification :actual-notes 5 :normal-notes 4)  ; Quintuplet
(time-modification
  :actual-notes 3
  :normal-notes 2
  :normal-type quarter)
```

---

## Note Structure

### FullNote

```lisp
(full-note :content (pitch :step C :octave 4))
(full-note :chord t :content (pitch :step E :octave 4))  ; Chord tone
(full-note :content (rest))
```

### NoteContent Variants

**Regular note:**
```lisp
(regular
  :full-note (full-note :content (pitch :step C :octave 4))
  :duration 1)

(regular
  :full-note (full-note :content (pitch :step G :octave 4))
  :duration 2
  :ties ((tie :type start)))
```

**Grace note:**
```lisp
(grace-note
  :grace (grace :slash t)
  :full-note (full-note :content (pitch :step D :octave 5)))
```

**Cue note:**
```lisp
(cue
  :full-note (full-note :content (pitch :step C :octave 4))
  :duration 1)
```

### Grace

```lisp
(grace)
(grace :slash t)
(grace :steal-time-previous 50.0)
(grace :steal-time-following 50.0)
```

### Tie

```lisp
(tie :type start)
(tie :type stop)
```

### Complete Note

```lisp
(note
  :content (regular
    :full-note (full-note :content (pitch :step C :octave 4))
    :duration 1)
  :voice "1"
  :type (note-type :value quarter)
  :stem (stem :value up))
```

**Full note with all common fields:**
```lisp
(note
  :content (regular
    :full-note (full-note :content (pitch :step F :alter 1.0 :octave 4))
    :duration 1
    :ties ((tie :type start)))
  :voice "1"
  :type (note-type :value quarter)
  :dots ((dot))
  :accidental (accidental :value sharp :cautionary t)
  :stem (stem :value down)
  :staff 1
  :beams ((beam :number 1 :value begin))
  :notations ((notations :content (
    (slur :type start :number 1 :placement above)))))
```

---

## Beam and Stem

### Beam

```lisp
(beam :number 1 :value begin)
(beam :number 1 :value continue)
(beam :number 1 :value end)
(beam :number 2 :value forward-hook)
(beam :number 2 :value backward-hook)
```

| `:value` | Description |
|----------|-------------|
| `begin` | Start of beam group |
| `continue` | Middle of beam group |
| `end` | End of beam group |
| `forward-hook` | Partial beam forward |
| `backward-hook` | Partial beam backward |

### Stem

```lisp
(stem :value up)
(stem :value down)
(stem :value none)
(stem :value double)
```

### Notehead

```lisp
(notehead :value normal)
(notehead :value diamond)
(notehead :value x :filled t)
```

| `:value` | Description |
|----------|-------------|
| `normal` | Standard notehead |
| `diamond` | Diamond (harmonic) |
| `x` | X notehead |
| `slash` | Slash |
| `triangle` | Triangle |
| `square` | Square |
| `none` | No notehead |

---

## Accidentals

```lisp
(accidental :value sharp)
(accidental :value flat)
(accidental :value natural)
(accidental :value double-sharp)
(accidental :value flat-flat)
(accidental :value sharp :cautionary t)
(accidental :value natural :editorial t :parentheses t)
```

| `:value` | Symbol |
|----------|--------|
| `sharp` | ♯ |
| `flat` | ♭ |
| `natural` | ♮ |
| `double-sharp` | 𝄪 |
| `flat-flat` | 𝄫 |
| `quarter-sharp` | Half-sharp |
| `quarter-flat` | Half-flat |
| `three-quarters-sharp` | 1.5 sharp |
| `three-quarters-flat` | 1.5 flat |

---

## Attributes (Measure Properties)

### Clef

```lisp
(clef :sign G :line 2)                    ; Treble
(clef :sign F :line 4)                    ; Bass
(clef :sign C :line 3)                    ; Alto
(clef :sign C :line 4)                    ; Tenor
(clef :sign G :line 2 :octave-change -1)  ; Treble 8vb
(clef :sign percussion)
```

### Key

**Traditional (circle of fifths):**
```lisp
(key :content (traditional-key :fifths 0 :mode major))    ; C major
(key :content (traditional-key :fifths -3 :mode minor))   ; C minor
(key :content (traditional-key :fifths 2 :mode major))    ; D major
(key :content (traditional-key :fifths -4))               ; Ab major (mode optional)
```

**Non-traditional:**
```lisp
(key :content (non-traditional (
  (key-step :step F :alter 1.0)
  (key-step :step C :alter 1.0))))
```

| Fifths | Major Key | Minor Key |
|--------|-----------|-----------|
| -7 | C♭ | A♭m |
| -6 | G♭ | E♭m |
| -5 | D♭ | B♭m |
| -4 | A♭ | Fm |
| -3 | E♭ | Cm |
| -2 | B♭ | Gm |
| -1 | F | Dm |
| 0 | C | Am |
| 1 | G | Em |
| 2 | D | Bm |
| 3 | A | F♯m |
| 4 | E | C♯m |
| 5 | B | G♯m |
| 6 | F♯ | D♯m |
| 7 | C♯ | A♯m |

| `:mode` | Description |
|---------|-------------|
| `major` | Major mode |
| `minor` | Minor mode |
| `dorian` | Dorian mode |
| `phrygian` | Phrygian mode |
| `lydian` | Lydian mode |
| `mixolydian` | Mixolydian mode |
| `aeolian` | Aeolian mode |
| `ionian` | Ionian mode |
| `locrian` | Locrian mode |

### Time Signature

```lisp
(time :content (measured :signatures ((time-signature :beats "4" :beat-type "4"))))
(time :content (measured :signatures ((time-signature :beats "3" :beat-type "4"))))
(time :content (measured :signatures ((time-signature :beats "6" :beat-type "8"))))

; Common/cut time
(time :content (measured :signatures ((time-signature :beats "4" :beat-type "4")))
      :symbol common)
(time :content (measured :signatures ((time-signature :beats "2" :beat-type "2")))
      :symbol cut)

; Compound signatures (2/4 + 3/8)
(time :content (measured :signatures (
  (time-signature :beats "2" :beat-type "4")
  (time-signature :beats "3" :beat-type "8"))))

; Senza misura (no time signature)
(time :content senza-misura)
```

### Complete Attributes Block

```lisp
(attributes
  :divisions 1
  :keys ((key :content (traditional-key :fifths 0 :mode major)))
  :times ((time :content (measured :signatures ((time-signature :beats "4" :beat-type "4")))))
  :staves 1
  :clefs ((clef :sign G :line 2)))
```

### Transpose

```lisp
(transpose :chromatic -2 :diatonic -1)           ; Bb instrument
(transpose :chromatic -9 :diatonic -5 :octave-change -1)  ; Eb Alto Sax
```

---

## Barlines

```lisp
(barline :location right :bar-style light-heavy)
(barline :location right :bar-style light-light)  ; Double bar
(barline :bar-style heavy-light :repeat (repeat :direction forward))
(barline :bar-style light-heavy :repeat (repeat :direction backward :times 2))
(barline :ending (ending :number "1" :type start))
```

| `:bar-style` | Description |
|--------------|-------------|
| `regular` | Normal barline |
| `dotted` | Dotted barline |
| `dashed` | Dashed barline |
| `heavy` | Thick barline |
| `light-light` | Double thin |
| `light-heavy` | Thin-thick (final) |
| `heavy-light` | Thick-thin |
| `heavy-heavy` | Double thick |
| `none` | Invisible |

---

## Directions (Dynamics, Tempo, etc.)

### Direction Structure

Directions wrap `DirectionType` elements which contain `DirectionTypeContent`:

```lisp
(direction
  :direction-types ((direction-type :content (dynamics ...)))
  :placement above)
```

### Dynamics

```lisp
(direction
  :direction-types ((direction-type :content (dynamics :content (ff))))
  :placement above)

(direction
  :direction-types ((direction-type :content (dynamics :content (mp))))
  :placement below
  :staff 1)

; Multiple dynamics
(direction
  :direction-types ((direction-type :content (dynamics :content (sf p)))))
```

| Dynamic values |
|---------------|
| `pppppp` `ppppp` `pppp` `ppp` `pp` `p` |
| `mp` `mf` |
| `f` `ff` `fff` `ffff` `fffff` `ffffff` |
| `sf` `sfp` `sfpp` `sfz` `sffz` `sfzp` |
| `fp` `fz` `pf` `rf` `rfz` |
| `n` (niente) |

### Wedge (Hairpins)

```lisp
(direction
  :direction-types ((direction-type :content (wedge :type crescendo)))
  :placement below)

(direction
  :direction-types ((direction-type :content (wedge :type stop))))

(direction
  :direction-types ((direction-type :content (wedge :type diminuendo :niente t))))
```

### Metronome (Tempo)

```lisp
(direction
  :direction-types ((direction-type :content
    (metronome :content (per-minute
      :beat-unit quarter
      :beat-unit-dots 0
      :per-minute (per-minute :value "120"))))))

; Dotted quarter = 60
(direction
  :direction-types ((direction-type :content
    (metronome :content (per-minute
      :beat-unit quarter
      :beat-unit-dots 1
      :per-minute (per-minute :value "60"))))))

; Beat equation: quarter = dotted eighth
(direction
  :direction-types ((direction-type :content
    (metronome :content (beat-equation
      :left-unit quarter
      :left-dots 0
      :right-unit eighth
      :right-dots 1)))))
```

### Words (Text Directions)

```lisp
(direction
  :direction-types ((direction-type :content (words :value "dolce"))))

(direction
  :direction-types ((direction-type :content (words :value "rit."))))
```

### Rehearsal Marks

```lisp
(direction
  :direction-types ((direction-type :content
    (rehearsal ((formatted-text :value "A"))))))
```

### Segno and Coda

```lisp
(direction
  :direction-types ((direction-type :content (segno))))

(direction
  :direction-types ((direction-type :content (coda))))
```

### Pedal

```lisp
(direction
  :direction-types ((direction-type :content (pedal :type start))))

(direction
  :direction-types ((direction-type :content (pedal :type stop))))
```

---

## Notations

Notations use a `:content` field containing a list of `NotationContent` variants:

```lisp
(notations :content (
  (slur :type start :number 1 :placement above)
  (tied :type start)
  (articulations :content ((staccato :placement above)))))
```

### Slurs

```lisp
(notations :content (
  (slur :type start :number 1 :placement above)))

(notations :content (
  (slur :type stop :number 1)))
```

### Tied (Visual Tie)

```lisp
(notations :content (
  (tied :type start)))

(notations :content (
  (tied :type stop)))
```

**Note:** `tie` (in note content) is for playback; `tied` (in notations) is for display. Use both for tied notes.

### Tuplet (Visual Bracket)

```lisp
(notations :content (
  (tuplet :type start :number 1 :bracket t :show-number actual)))

(notations :content (
  (tuplet :type stop :number 1)))
```

### Fermata

```lisp
(notations :content (
  (fermata :shape normal)))

(notations :content (
  (fermata :shape angled :type inverted)))
```

### Articulations

Articulations use `:content` with a flat list of articulation elements:

```lisp
(notations :content (
  (articulations :content (
    (staccato :placement above)))))

(notations :content (
  (articulations :content (
    (accent)
    (tenuto)))))

(notations :content (
  (articulations :content (
    (staccato)
    (accent)))))
```

| Articulation elements |
|----------------------|
| `(accent)` `(strong-accent)` (marcato) |
| `(staccato)` `(staccatissimo)` |
| `(tenuto)` |
| `(detached-legato)` (portato) |
| `(spiccato)` |
| `(breath-mark :value comma)` |
| `(caesura :value normal)` |
| `(stress)` `(unstress)` |

### Ornaments

Ornaments use `:content` with `OrnamentWithAccidentals` wrappers:

```lisp
(notations :content (
  (ornaments :content (
    (ornament-with-accidentals
      :ornament (trill-mark :placement above)
      :accidental-marks ())))))

(notations :content (
  (ornaments :content (
    (ornament-with-accidentals
      :ornament (mordent :long t)
      :accidental-marks ())))))

(notations :content (
  (ornaments :content (
    (ornament-with-accidentals
      :ornament (turn :placement above)
      :accidental-marks ((accidental-mark :value sharp :placement above)))))))
```

| Ornament elements |
|------------------|
| `(trill-mark)` |
| `(turn)` `(delayed-turn)` `(inverted-turn)` |
| `(mordent)` `(inverted-mordent)` |
| `(shake)` |
| `(tremolo :value 3 :type single)` |
| `(wavy-line :type start)` |

### Technical

```lisp
(notations :content (
  (technical :content (
    (up-bow)
    (down-bow)))))

(notations :content (
  (technical :content (
    (fingering :value "1" :placement above)))))

(notations :content (
  (technical :content (
    (string :value 1)
    (fret :value 3)))))
```

### Glissando and Slide

```lisp
(notations :content (
  (glissando :type start :text "gliss.")))

(notations :content (
  (slide :type start)))
```

### Arpeggiate

```lisp
(notations :content (
  (arpeggiate :direction up)))
```

---

## Voice Management

### Backup

```lisp
(backup :duration 4)
```

Used to move backward in time to write a second voice.

### Forward

```lisp
(forward :duration 2 :voice "2" :staff 1)
```

Used to skip forward (e.g., for rests not explicitly written).

---

## Lyrics

Lyrics use a `:content` field with `LyricContent` variants:

```lisp
(lyric
  :number "1"
  :content (syllable
    :syllabic single
    :text (text-element-data :value "word")))

(lyric
  :number "1"
  :content (syllable
    :syllabic begin
    :text (text-element-data :value "hel")))

(lyric
  :number "1"
  :content (syllable
    :syllabic end
    :text (text-element-data :value "lo")))

(lyric
  :number "1"
  :content (syllable
    :syllabic middle
    :text (text-element-data :value "lu")
    :extend (extend :type start)))

; Melisma (extend only)
(lyric
  :number "1"
  :content (extend-only :extend (extend :type continue)))

; Special vocalizations
(lyric :number "1" :content laughing)
(lyric :number "1" :content humming)
```

| `:syllabic` | Description |
|-------------|-------------|
| `single` | Complete word |
| `begin` | Start of multi-syllable word |
| `middle` | Middle syllable |
| `end` | Final syllable |

---

## Measure

```lisp
(measure
  :number "1"
  :content (
    (attributes ...)
    (note ...)
    (note ...)
    (note ...)
    (note ...)))

(measure
  :number "2"
  :width 150.0
  :content (
    (direction :direction-types ((direction-type :content (dynamics :content (mf)))))
    (note ...)
    (note ...)
    (barline :location right :bar-style light-heavy)))
```

---

## Part

```lisp
(part
  :id "P1"
  :measures (
    (measure :number "1" :content (...))
    (measure :number "2" :content (...))))
```

---

## Part List

```lisp
(part-list
  :content (
    (score-part
      :id "P1"
      :part-name (part-name :value "Piano"))))

(part-list
  :content (
    (score-part
      :id "P1"
      :part-name (part-name :value "Violin")
      :part-abbreviation (part-name :value "Vln."))
    (score-part
      :id "P2"
      :part-name (part-name :value "Cello")
      :part-abbreviation (part-name :value "Vc."))))
```

---

## Complete Score

```lisp
(score-partwise
  :version "4.0"
  :work (work :work-title "My Composition")
  :identification (identification
    :creators ((typed-text :value "Composer Name" :type "composer")))
  :part-list (part-list
    :content ((score-part
      :id "P1"
      :part-name (part-name :value "Piano"))))
  :parts ((part
    :id "P1"
    :measures (
      (measure
        :number "1"
        :content (
          (attributes
            :divisions 1
            :keys ((key :content (traditional-key :fifths 0 :mode major)))
            :times ((time :content (measured :signatures ((time-signature :beats "4" :beat-type "4")))))
            :clefs ((clef :sign G :line 2)))
          (note
            :content (regular
              :full-note (full-note :content (pitch :step C :octave 4))
              :duration 1)
            :voice "1"
            :type (note-type :value quarter))
          (note
            :content (regular
              :full-note (full-note :content (pitch :step D :octave 4))
              :duration 1)
            :voice "1"
            :type (note-type :value quarter))
          (note
            :content (regular
              :full-note (full-note :content (pitch :step E :octave 4))
              :duration 1)
            :voice "1"
            :type (note-type :value quarter))
          (note
            :content (regular
              :full-note (full-note :content (pitch :step F :octave 4))
              :duration 1)
            :voice "1"
            :type (note-type :value quarter))))))))
```

---

## Phase 5 Compilation Targets

When designing Fermata syntax, these are the transformations needed:

### Pitches

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `c4` | `(pitch :step C :octave 4)` |
| `f#5` | `(pitch :step F :alter 1.0 :octave 5)` |
| `bb3` | `(pitch :step B :alter -1.0 :octave 3)` |
| `c+4` (quarter sharp) | `(pitch :step C :alter 0.5 :octave 4)` |
| `r` (rest) | `(rest)` |

### Durations

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `:q` or `quarter` | `(note-type :value quarter)` |
| `:h` or `half` | `(note-type :value half)` |
| `:w` or `whole` | `(note-type :value whole)` |
| `:e` or `eighth` | `(note-type :value eighth)` |
| `:s` or `sixteenth` | `(note-type :value n16th)` |
| `:h.` (dotted half) | `(note-type :value half)` + `((dot))` |
| `:q..` (double-dotted) | `(note-type :value quarter)` + `((dot) (dot))` |

### Notes

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(note c4 :q)` | `(note :content (regular :full-note (full-note :content (pitch :step C :octave 4)) :duration 1) :type (note-type :value quarter))` |
| `(note r :h)` | `(note :content (regular :full-note (full-note :content (rest)) :duration 2) :type (note-type :value half))` |
| `(grace-note d5)` | `(note :content (grace-note :grace (grace) :full-note (full-note :content (pitch :step D :octave 5))))` |

### Chords

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(chord :q c4 e4 g4)` | Three `(note ...)` elements; 2nd and 3rd have `:chord t` in `full-note` |

### Tuplets

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(tuplet 3:2 ...)` | Notes with `(time-modification :actual-notes 3 :normal-notes 2)` and `(notations :content ((tuplet :type start/stop ...)))` |

### Dynamics

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(ff)` or `(dynamic ff)` | `(direction :direction-types ((direction-type :content (dynamics :content (ff)))))` |
| `(cresc)` | `(direction :direction-types ((direction-type :content (wedge :type crescendo))))` |
| `(decresc)` or `(dim)` | `(direction :direction-types ((direction-type :content (wedge :type diminuendo))))` |

### Tempo

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(tempo :q 120)` | `(direction :direction-types ((direction-type :content (metronome :content (per-minute :beat-unit quarter :beat-unit-dots 0 :per-minute (per-minute :value "120"))))))` |
| `(tempo "Allegro" :q 120)` | `(direction :direction-types ((direction-type :content (words :value "Allegro")) (direction-type :content (metronome ...))))` |

### Key Signatures

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(key c :major)` | `(key :content (traditional-key :fifths 0 :mode major))` |
| `(key g :major)` | `(key :content (traditional-key :fifths 1 :mode major))` |
| `(key f :major)` | `(key :content (traditional-key :fifths -1 :mode major))` |
| `(key a :minor)` | `(key :content (traditional-key :fifths 0 :mode minor))` |
| `(key d :dorian)` | `(key :content (traditional-key :fifths 0 :mode dorian))` |

### Time Signatures

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(time 4 4)` | `(time :content (measured :signatures ((time-signature :beats "4" :beat-type "4"))))` |
| `(time 3 4)` | `(time :content (measured :signatures ((time-signature :beats "3" :beat-type "4"))))` |
| `(time 6 8)` | `(time :content (measured :signatures ((time-signature :beats "6" :beat-type "8"))))` |
| `(time :common)` | `(time :content (measured :signatures ((time-signature :beats "4" :beat-type "4"))) :symbol common)` |
| `(time :cut)` | `(time :content (measured :signatures ((time-signature :beats "2" :beat-type "2"))) :symbol cut)` |

### Clefs

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(clef :treble)` | `(clef :sign G :line 2)` |
| `(clef :bass)` | `(clef :sign F :line 4)` |
| `(clef :alto)` | `(clef :sign C :line 3)` |
| `(clef :tenor)` | `(clef :sign C :line 4)` |
| `(clef :treble-8vb)` | `(clef :sign G :line 2 :octave-change -1)` |

### Articulations

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(staccato)` | `(notations :content ((articulations :content ((staccato)))))` |
| `(accent)` | `(notations :content ((articulations :content ((accent)))))` |
| `(tenuto)` | `(notations :content ((articulations :content ((tenuto)))))` |
| `(marcato)` | `(notations :content ((articulations :content ((strong-accent)))))` |

### Slurs and Ties

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(slur :start)` | `(notations :content ((slur :type start :number 1)))` |
| `(slur :stop)` | `(notations :content ((slur :type stop :number 1)))` |
| `(tie :start)` | `(tie :type start)` in note content + `(notations :content ((tied :type start)))` |

### Ornaments

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(trill)` | `(notations :content ((ornaments :content ((ornament-with-accidentals :ornament (trill-mark) :accidental-marks ())))))` |
| `(mordent)` | `(notations :content ((ornaments :content ((ornament-with-accidentals :ornament (mordent) :accidental-marks ())))))` |
| `(turn)` | `(notations :content ((ornaments :content ((ornament-with-accidentals :ornament (turn) :accidental-marks ())))))` |
| `(fermata)` | `(notations :content ((fermata :shape normal)))` |

### Lyrics

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(lyric "word")` | `(lyric :number "1" :content (syllable :syllabic single :text (text-element-data :value "word")))` |
| `(lyric "hel" :begin)` | `(lyric :number "1" :content (syllable :syllabic begin :text (text-element-data :value "hel")))` |
| `(lyric "lo" :end)` | `(lyric :number "1" :content (syllable :syllabic end :text (text-element-data :value "lo")))` |

### Repeats and Endings

| Fermata (Ergonomic) | IR (Explicit) |
|---------------------|---------------|
| `(repeat :forward)` | `(barline :bar-style heavy-light :repeat (repeat :direction forward))` |
| `(repeat :backward)` | `(barline :bar-style light-heavy :repeat (repeat :direction backward))` |
| `(ending 1 :start)` | `(barline :ending (ending :number "1" :type start))` |
| `(fine)` | `(barline :location right :bar-style light-heavy)` |

---

*This document is the contract. Phase 5 compiles TO this format. Phase 4 reads/writes this format. MusicXML emitter consumes this format.*
