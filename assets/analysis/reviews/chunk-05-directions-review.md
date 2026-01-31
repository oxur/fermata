# Review: Chunk 5 — Directions

## Summary

This is a comprehensive chunk covering the extensive `<direction>` system in MusicXML. The examples are particularly strong, showing real-world scenarios like crescendo hairpins with target dynamics, tempo markings combining text and metronome, and D.S. al Coda navigation. The `DirectionTypeContent` enum captures the full variety of direction types well. There are some minor representation issues with the metronome metric modulation syntax and the recurring "text as final argument" pattern that needs documentation, but overall this is solid work ready for implementation.

## Approved Mappings

These mappings are correct and ready for implementation:

- `direction` ✔ — Clean wrapper with placement, staff, voice
- `direction-type` ✔ — Preserves MusicXML's combining capability
- `dynamics` ✔ — Empty element forms `(f)`, `(pp)` are elegant
- `wedge` ✔ — Start/stop/continue with number for overlapping
- `words` ✔ — Text with full formatting attributes
- `rehearsal` ✔ — Text with enclosure options
- `segno` / `coda` ✔ — Simple navigation markers
- `pedal` ✔ — All pedal types (start/stop/change/continue)
- `octave-shift` ✔ — Up/down/stop with size (8, 15, 22)
- `sound` ✔ — Playback attributes (tempo, dynamics, pedal state)

---

## Required Changes

### 1. Metronome Metric Modulation Syntax

**Problem:** The metric modulation (beat-unit = beat-unit) uses `:beat-unit-2` which is awkward and doesn't scale to dotted notes on both sides.

**Current (lines 394-397):**
```lisp
;; Metric modulation
(metronome
  :beat-unit :quarter
  :beat-unit-2 :half)
```

**Should be:**
```lisp
;; Metric modulation: quarter = half
(metronome
  :left (beat-unit :value :quarter)
  :right (beat-unit :value :half))

;; With dots: dotted quarter = half
(metronome
  :left (beat-unit :value :quarter :dots 1)
  :right (beat-unit :value :half))
```

**Rationale:** 
1. The Rust enum `MetronomeContent::BeatEquation` has `left_unit`, `left_dots`, `right_unit`, `right_dots`
2. Using `:left` and `:right` sub-forms maps cleanly to this structure
3. Allows dots on either side without proliferating keywords

---

### 2. Per-Minute Type Handling

**Problem:** The `per-minute` value can be either a number or a string (for "ca. 120" or "120-132"). The S-expr examples only show numeric values.

**Current (line 386):**
```lisp
(metronome
  :beat-unit :quarter
  :per-minute 120)
```

**Should also show:**
```lisp
;; Approximate tempo
(metronome
  :beat-unit :quarter
  :per-minute "ca. 120")

;; Tempo range
(metronome
  :beat-unit :quarter
  :per-minute "120-132")
```

**Rationale:** The Rust type `PerMinute::Text(String)` supports this, so the S-expr should document it.

---

### 3. Document Text Content Pattern

**Problem:** This chunk uses the "text as final argument" pattern extensively but it's not formally documented.

**Examples:**
```lisp
(words :font-style :italic "dolce")
(rehearsal :enclosure :square "A")
(other-dynamics "ffff")
```

**Should add documentation:**

> **Text Content Convention:** Elements with text content place the text as the final positional argument after all keyword pairs. This mirrors MusicXML's structure where text is the element content and attributes are separate.
>
> ```lisp
> (element :attr1 val1 :attr2 val2 "text content")
> ```
>
> In the Rust struct, this maps to a `text: String` field.

---

### 4. Wedge Number Should Be String

**Problem:** Wedge `:number` is shown as integer but should match MusicXML's number-level type (1-16, could be treated as string for consistency).

**Current (line 252):**
```lisp
(wedge :type :crescendo :number 1)
```

**Analysis:** MusicXML's `number-level` is 1-16 and typically represented as integer. This is actually fine as-is, but document that it's an integer in range 1-16.

**No change needed** — just add documentation note that `:number` is integer 1-16.

---

### 5. Sound Element Needs More Documentation

**Problem:** The `<sound>` element has many attributes for playback control but only a few are shown.

**Current examples show:** `dynamics`, `tempo`, `damper-pedal`, `dalsegno`, `tocoda`

**Should document additional common attributes:**
```lisp
(sound
  :tempo 120              ;; BPM
  :dynamics 90            ;; MIDI velocity (1-127 scale in MusicXML)
  :damper-pedal :yes      ;; Sustain pedal
  :soft-pedal :yes        ;; Una corda
  :sostenuto-pedal :yes   ;; Sostenuto
  :forward-repeat :yes    ;; Take forward repeat
  :fine "fine1"           ;; Fine marker ID
  :dalsegno "segno1"      ;; Jump to segno
  :dacapo :yes            ;; D.C.
  :coda "coda1"           ;; Coda marker ID
  :tocoda "coda1"         ;; Jump to coda
  :segno "segno1")        ;; Segno marker ID
```

---

## Suggested Improvements

### 1. Add Dashes and Bracket Examples

The element table mentions `<dashes>` and `<bracket>` but they're listed in Open Questions without examples. These are common for "rit.----" or grouping expressions:

```lisp
;; Start of dashed line (after "rit." text)
(direction
  (direction-type
    (words :font-style :italic "rit."))
  (direction-type
    (dashes :type :start :number 1)))

;; End of dashed line
(direction
  (direction-type
    (dashes :type :stop :number 1)))

;; Bracket for grouping
(direction
  (direction-type
    (bracket :type :start :number 1 :line-end :down)))

(direction
  (direction-type
    (bracket :type :stop :number 1 :line-end :down)))
```

---

### 2. Add Offset Example

The `<offset>` element is mentioned but not shown. It's important for precise positioning:

```lisp
;; Dynamic that appears slightly after the beat
(direction :placement :below
  (direction-type
    (dynamics (sf)))
  :offset 2)  ;; 2 divisions after the current position
```

---

### 3. Add Combined Dynamics Example (fp, sfz, etc.)

The chunk shows `(dynamics (f) (p))` for combined dynamics but doesn't explain this is how `fp` (forte-piano) is encoded:

```lisp
;; fp (forte-piano) - loud attack, immediately soft
(dynamics (f) (p))

;; sfp (sforzando-piano)
(dynamics (sf) (p))

;; sfz is a single element, not combined
(dynamics (sfz))
```

---

### 4. Add Principal Voice Example

The `DirectionTypeContent` enum includes `PrincipalVoice` but it's not documented. This is used for analysis/educational purposes:

```lisp
(direction
  (direction-type
    (principal-voice :type :start :symbol :hauptstimme)))
```

---

### 5. Document Harp Pedals

The enum includes `HarpPedals` — this is specialized but worth a brief example for completeness:

```lisp
(direction
  (direction-type
    (harp-pedals
      (pedal-tuning :step :D :alter -1)
      (pedal-tuning :step :C :alter 0)
      (pedal-tuning :step :B :alter 0)
      ;; ... all 7 pedals
      )))
```

---

## Open Questions Resolved

### Q: When multiple types in one direction, do they share position?

**Recommendation:** Yes, they share the same musical position but may have individual offsets.

**Rationale:**
1. MusicXML groups related markings in one `<direction>` precisely so they share position
2. Example 3 shows "Allegro" text + metronome together — they appear at the same beat
3. Example 2 shows wedge stop + dynamic together — the dynamic is the target of the crescendo
4. Individual `<direction-type>` elements can have their own position attributes for fine-tuning
5. The `<offset>` on `<direction>` affects all contained types equally

---

### Q: Include playback-only info in IR, or separate layer?

**Recommendation:** Include `<sound>` in the IR.

**Rationale:**
1. `<sound>` is integral to MusicXML structure — it's a child of `<direction>`
2. Required for round-tripping
3. Playback and notation are interleaved in the source; separating them loses structure
4. Higher-level processing can extract a "playback track" if needed
5. Many applications need both (notation display + MIDI export)

---

### Q: Dashes/brackets — Start/stop pairs like wedges. Document in detail?

**Recommendation:** Yes, add examples (see Suggested Improvements above).

**Rationale:**
1. Dashes are very common for "rit." and "accel." lines
2. Brackets are used for grouping expressions and editorial markings
3. They follow the same start/stop pattern as wedges
4. The `number` attribute disambiguates overlapping lines

---

## New Questions Identified

- [ ] **Direction vs. notation placement:** Some markings can appear as either `<direction>` (measure-level) or inside `<notations>` (note-attached). When should each be used? Document the distinction.

- [ ] **Dynamics in notations:** The `<dynamics>` element can also appear in `<notations>` for note-attached dynamics (e.g., `sfz` on a specific note). Is this covered in another chunk?

- [ ] **SMuFL glyph references:** The `Segno` and `Coda` structs have `smufl` fields for alternate glyphs. Should we document common SMuFL glyph names?

- [ ] **Scordatura:** The enum includes `Scordatura` for altered tunings. Which chunk covers this? (Guitar/strings use case)

- [ ] **Image in directions:** The enum includes `Image` — can directions contain embedded images? Document the use case.

- [ ] **Sound element completeness:** Should we define a complete `Sound` struct with all attributes, or is it covered in another chunk?

---

## Cross-Chunk Concerns

### Chunk 3 (Measure Structure)

Directions appear within measures as `MusicDataElement::Direction`. Ensure:
- The `Direction` struct here matches the enum variant in Chunk 3
- Document that directions typically precede the note they apply to

### Chunk 6 (Notations) — Likely

Some direction-like elements can appear in `<notations>`:
- `<dynamics>` can be note-attached
- `<fermata>` appears in notations (mentioned in barline context in Chunk 3)
- Articulations vs. directions distinction needs clarity

### Barline Navigation (Chunk 3)

Segno and coda can appear in both:
- `<direction>` (this chunk) — positioned in the measure
- `<barline>` (Chunk 3) — attached to the barline

Document when each is used: barline attachment is for navigation structure, direction is for visual placement.

### Sound Element Scope

The `<sound>` element has extensive attributes. Consider:
- Documenting the complete `Sound` struct in this chunk or a dedicated "Playback" chunk
- Cross-referencing MIDI elements from Chunk 4 (score-part MIDI settings)

### Tie/Slur vs. Direction Lines

Wedges, dashes, and brackets use start/stop pairs like ties and slurs. Ensure consistent:
- Use of `number` attribute for disambiguation
- Pattern for start/stop/continue types
- Position attribute handling
