# Review: Chunk 6 — Notations

## Summary

This is an excellent, comprehensive chunk covering the rich variety of note-attached notations in MusicXML. The tabular documentation of articulations, ornaments, and technical elements is particularly helpful. The `NotationContent` enum cleanly captures all the notation categories, and the cross-reference to earlier chunks (tied, slur, tuplet) avoids duplication. The examples are well-chosen, from basic staccato/accent combinations to guitar tablature and arpeggiated chords. There's one structural issue with the notations wrapper pattern that needs resolution (consistent with Chunk 2's findings), and some minor representation clarifications needed.

## Approved Mappings

These mappings are correct and ready for implementation:

- `notations` ✔ — Container with editorial and multiple content types
- `articulations` ✔ — All standard articulations well-documented
- `ornaments` ✔ — Complete with accidental-mark pairing
- `technical` ✔ — Impressive coverage of instrument-specific markings
- `fermata` ✔ — Type and shape attributes correctly mapped
- `arpeggiate` ✔ — Number for grouping, direction for roll
- `non-arpeggiate` ✔ — Top/bottom bracket notation
- `glissando` / `slide` ✔ — Start/stop with line-type
- `accidental-mark` ✔ — Placement and value for auxiliary notes

---

## Required Changes

### 1. Notations Wrapper Pattern — Critical Cross-Chunk Issue

**Problem:** This chunk uses the same double-wrapped list pattern identified in Chunk 2:

**Current (lines 54-57):**
```lisp
:notations ((notations
              (articulations
                (staccato)
                (accent))))
```

**Issue:** The `((notations ...))` double-parenthesis is confusing. Is `:notations` a list of `notations` forms? The Rust type shows `notations: Vec<Notations>` on `Note`, so technically yes, but this is verbose for the common case.

**Should be (Option A — Flatten common case):**
```lisp
;; When there's just one notations block (99% of cases):
:notations (notations
             (articulations
               (staccato)
               (accent)))

;; When there are multiple notations blocks (rare, editorial):
:notations ((notations :print-object :no
              (articulations (staccato)))
            (notations
              (articulations (accent))))
```

**Should be (Option B — Always use list, but cleaner syntax):**
```lisp
:notations [(notations (articulations (staccato) (accent)))]
```

**Recommendation:** Use Option A with documentation that the parser accepts either form. Single `(notations ...)` is syntactic sugar for a one-element list.

**Action Required:** This must be resolved consistently with Chunk 2's `notations` representation.

---

### 2. Tremolo Representation

**Problem:** The tremolo S-expr shows the count as a positional argument, but this is inconsistent with other patterns.

**Current (line 221):**
```lisp
(tremolo :type :single 3)
```

**Should be:**
```lisp
(tremolo :type :single :marks 3)
```

**Rationale:** Using a keyword (`:marks`) is consistent with our other patterns. The value 1-8 represents the number of tremolo beams.

---

### 3. Accidental-Mark Value Placement

**Problem:** In the ornaments example, accidental-mark uses `:value :sharp`, but in the element table it shows content as the value.

**Table (line 24):**
```
| `<accidental-mark>` | complex | value | — | placement, position |
```

**MusicXML (line 552):**
```xml
<accidental-mark placement="above">sharp</accidental-mark>
```

**S-expr (line 560):**
```lisp
(accidental-mark :placement :above :value :sharp)
```

**Analysis:** This is actually correct — the MusicXML text content becomes the `:value` keyword. Document this transformation explicitly.

**No change needed** — just add a note: "Text content of `<accidental-mark>` maps to `:value` keyword."

---

### 4. Harmonic Element Needs Detail

**Problem:** The harmonic technical element is shown as `(harmonic :natural t)` but the Rust type `Harmonic` suggests it's more complex.

**Current (line 302):**
```lisp
(harmonic :natural t)
```

**Should be (complete mapping):**
```lisp
;; Natural harmonic
(harmonic
  :natural (natural-harmonic))

;; Artificial harmonic with sounding pitch
(harmonic
  :artificial (artificial-harmonic
                :sounding-pitch (pitch :step :E :octave 6)))

;; Harmonic with base pitch shown
(harmonic
  :natural (natural-harmonic)
  :base-pitch (pitch :step :E :octave 4))
```

**Rationale:** MusicXML's `<harmonic>` has sub-elements for natural/artificial and optional base/touching/sounding pitches.

---

### 5. String and Fret Value Types

**Problem:** The table shows `(fret 5)` and `(string 1)` with bare integers, but placement attributes are also available.

**Current (lines 311-312):**
```lisp
(fret 5)
(string 1)
```

**Should also show:**
```lisp
(fret :value 5 :placement :below)
(string :value 1 :placement :above)
```

**Rationale:** When attributes are present, we need keyword form. Document both:
- Simple: `(fret 5)` when no attributes
- Full: `(fret :value 5 :placement :below)` when attributes needed

---

## Suggested Improvements

### 1. Add Slur Example

The chunk notes that slurs are in `<notations>` but covered in Chunk 2. Add a brief cross-reference example:

```lisp
;; Slur start (see Chunk 2 for details)
:notations (notations
             (slur :type :start :number 1))
```

---

### 2. Add Breath Mark Values

The table mentions breath-mark but doesn't show the different symbols:

```lisp
;; Standard comma
(breath-mark)

;; Tick mark
(breath-mark :value :tick)

;; Upbow-like
(breath-mark :value :upbow)

;; Salzedo (harp)
(breath-mark :value :salzedo)
```

---

### 3. Add Caesura Types

Similarly for caesura:

```lisp
;; Standard railroad tracks
(caesura)

;; Thick caesura
(caesura :value :thick)

;; Short caesura
(caesura :value :short)

;; Curved caesura
(caesura :value :curved)

;; Single stroke
(caesura :value :single)
```

---

### 4. Add Bend Element Detail

The bend element for guitar is complex:

```lisp
(bend
  :bend-alter 1.0        ;; Semitones (1 = whole step bend)
  :pre-bend :yes         ;; Bend before plucking
  :release :yes          ;; Release the bend
  :with-bar :yes)        ;; Use whammy bar
```

---

### 5. Add Wavy-Line Example with Trill

The wavy-line element extends trills:

```lisp
;; Trill with extension line
(note
  :pitch (pitch :step :C :octave 5)
  :duration 4
  :type :quarter
  :notations (notations
               (ornaments
                 (trill-mark)
                 (wavy-line :type :start :number 1))))

;; On following notes
(note
  :pitch (pitch :step :C :octave 5)
  :duration 4
  :type :quarter
  :notations (notations
               (ornaments
                 (wavy-line :type :continue :number 1))))

;; End of trill line
(note
  :pitch (pitch :step :D :octave 5)
  :duration 4
  :type :quarter
  :notations (notations
               (ornaments
                 (wavy-line :type :stop :number 1))))
```

---

## Open Questions Resolved

### Q: When to use multiple `<notations>` vs. one with multiple children?

**Recommendation:** Multiple `<notations>` elements are for different editorial levels; use one with multiple children for related markings.

**Rationale:**
1. The `<notations>` element has an `editorial` group (footnote, level)
2. Multiple `<notations>` allows different editorial attributions
3. Example: Original score has staccato; editor adds accent → two `<notations>` with different editorial sources
4. For normal use, one `<notations>` with multiple children is preferred
5. Document: "Use multiple `(notations ...)` forms only when editorial attribution differs."

---

### Q: Include start-note, trill-step attrs in IR or defer?

**Recommendation:** Include in IR.

**Rationale:**
1. These attributes exist on ornament elements (`start-note`, `trill-step`, `two-note-turn`, `accelerate`, `beats`, `second-beat`, `last-beat`)
2. They're needed for playback interpretation
3. Required for round-tripping
4. The Rust types already accommodate them (`EmptyTrillSound` struct)
5. Implementation can initially ignore them for playback, but store for fidelity

---

### Q: Complete coverage of technical elements vs. defer uncommon ones?

**Recommendation:** Include all in IR; document as "complete coverage for round-tripping."

**Rationale:**
1. The element list is finite and already enumerated
2. Storage cost is minimal (enum variants)
3. Uncommon elements (handbell, golpe, fingernails) are important for specific repertoire
4. Omitting any breaks round-tripping for those scores
5. Implementation can display unknown technicals as text fallback

---

## New Questions Identified

- [ ] **Dynamics in notations vs. directions:** The `NotationContent` enum includes `Dynamics`. When should dynamics appear in `<notations>` (note-attached) vs. `<direction>` (measure-positioned)? Document the distinction.

- [ ] **Multiple ornaments on one note:** Can a note have multiple ornament types (e.g., trill AND mordent)? If so, are they in one `(ornaments ...)` or separate?

- [ ] **Ornament accidental-mark ordering:** The MusicXML schema shows accidental-marks follow the ornament. Is this order preserved in the S-expr? Document: ornament first, then accidental-marks.

- [ ] **Articulation stacking order:** When multiple articulations appear (staccato + accent + tenuto), does order matter for rendering? Document rendering conventions.

- [ ] **Technical elements and tablature:** For TAB notation, which technical elements are required vs. optional? Consider a tablature-specific example.

---

## Cross-Chunk Concerns

### Chunk 2 (Time & Rhythm) — CRITICAL

The `notations` wrapper pattern must be resolved consistently:
- Chunk 2 shows `((notations (tuplet :type :start)))`  
- Chunk 6 shows `((notations (articulations (staccato))))`
- Both need the same resolution

**Action:** Create a unified "Notations Wrapper Convention" and apply to both chunks.

### Chunk 1 (Core Note)

The `Note` struct has:
```rust
pub notations: Vec<Notations>,
```

Ensure the S-expr representation matches. The `Vec` allows multiple `(notations ...)` forms, but document when this is used.

### Chunk 5 (Directions)

Dynamics can appear in both:
- `<direction>` → `(direction (direction-type (dynamics (f))))` — measure-positioned
- `<notations>` → `(notations (dynamics (sf)))` — note-attached

Document the distinction:
- Direction dynamics: General dynamic level changes (p, f, mf)
- Notation dynamics: Note-specific attacks (sf, sfz, fp on a specific note)

### Chunk 3 (Measure Structure)

Fermatas can appear in both:
- `<notations>` (this chunk) — on a note
- `<barline>` (Chunk 3) — on a barline

Both are valid; barline fermata is for holds at phrase endings without a specific note.

### Slur/Tied Coverage

This chunk correctly notes that tied, slur, and tuplet are covered in Chunk 2. Ensure:
- The `NotationContent` enum includes these types
- Cross-references are bidirectional (Chunk 2 should reference Chunk 6)
