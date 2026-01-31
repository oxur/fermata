# Review: Chunk 10 — Advanced and Deferred Elements

## Summary

This is an excellent "catalog" chunk that provides appropriate coverage of advanced MusicXML elements without over-committing to detailed mappings before they're needed. The decision to present provisional S-expr representations while explicitly marking them as "future work" is wise. The categorization is logical, the summary table provides a good prioritization framework, and the design principles section ensures future mappings will be consistent with earlier chunks. A few elements here (chord symbols, unpitched percussion, tablature) may warrant earlier promotion based on usage frequency.

## Approved Approach

The catalog approach is correct for these elements:

- **Chord Symbols / Harmony** ✔ — Provisional mapping looks good
- **Figured Bass** ✔ — Appropriate low priority
- **Percussion (unpitched)** ✔ — Actually covered in Chunk 1's `PitchRest::Unpitched`
- **Harp Pedals** ✔ — Specialized, low priority correct
- **Accordion Registration** ✔ — Very specialized
- **Scordatura** ✔ — String-specific, low priority
- **Early Music** ✔ — Niche use case
- **Playback/MIDI** ✔ — Important for DAW integration
- **Accessibility** ✔ — Score-following applications

---

## Required Changes

### 1. Unpitched Notes Already Covered — Update Cross-Reference

**Problem:** Unpitched notes were mentioned in Chunk 1's open questions and the `PitchRest::Unpitched` enum variant exists. This chunk should reference that.

**Current (Section 10.3):**
```lisp
(note
  :unpitched (unpitched :display-step :E :display-octave 4)
  :duration 1
  :type :quarter
  :instrument "snare")
```

**Action:** Add note: "See Chunk 1 for the `PitchRest::Unpitched` variant. This section covers additional percussion-specific elements."

---

### 2. Sound Element Already Used — Ensure Consistency

**Problem:** The `<sound>` element appears in Chunk 5 (Directions) examples. Ensure this chunk's provisional mapping is consistent.

**Chunk 5 examples:**
```lisp
(sound :tempo 120 :dynamics 75)
(sound :damper-pedal :yes)
(sound :dalsegno "segno1" :tocoda "coda1")
```

**This chunk:**
```lisp
(sound :tempo 120 :dynamics 75)
(sound :dacapo :yes)
```

**Analysis:** These are consistent. Good.

**Recommendation:** Add note: "See Chunk 5 for `<sound>` element usage with directions. This section documents additional sound attributes."

---

### 3. Tablature Already Partially Covered

**Problem:** Guitar tablature (`<string>`, `<fret>`) is covered in Chunk 6 (Technical elements). This chunk should reference that.

**Chunk 6 Example 6:**
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

**Action:** Add note: "Basic string/fret notation is covered in Chunk 6 (Technical). This section covers additional tablature features like staff-type TAB."

---

### 4. Credit and Defaults Already Covered

**Problem:** `<credit>` and `<defaults>` are covered in Chunk 4 (Part & Score). This chunk should reference that, not re-list them.

**Action:** Either remove these from 10.12 or add cross-reference: "See Chunk 4 for detailed credit and defaults mappings."

---

### 5. Instrument ID Representation

**Problem:** The percussion example shows `:instrument "snare"` as a string, but Chunk 1's Note struct has `instrument: Vec<Instrument>`.

**Current:**
```lisp
:instrument "snare"
```

**Should be:**
```lisp
:instruments ((instrument :id "snare"))
```

**Rationale:** Match Chunk 1's plural `:instruments` and nested form for the `Instrument` struct.

---

## Suggested Improvements

### 1. Promote Chord Symbols to Higher Priority

Chord symbols are extremely common in:
- Lead sheets / fake books
- Jazz charts
- Pop/rock arrangements
- Educational materials

**Recommendation:** Consider moving harmony/chord symbols from "deferred" to a core chunk, or create Chunk 11 specifically for harmony.

---

### 2. Add Nashville Number Example

The Nashville number system is mentioned but not shown:

```xml
<harmony>
  <numeral>
    <numeral-root>5</numeral-root>
    <numeral-alter>0</numeral-alter>
  </numeral>
  <kind text="">major</kind>
</harmony>
```

```lisp
(harmony
  :numeral (numeral :root 5)
  :kind :major)
```

---

### 3. Add Degree Alteration Example

Chord degrees (add9, #11, b13) are common:

```xml
<harmony>
  <root><root-step>C</root-step></root>
  <kind>dominant</kind>
  <degree>
    <degree-value>9</degree-value>
    <degree-alter>1</degree-alter>
    <degree-type>add</degree-type>
  </degree>
</harmony>
```

```lisp
(harmony
  :root (root :step :C)
  :kind :dominant
  :degrees ((degree :value 9 :alter 1 :type :add)))  ;; C7#9
```

---

### 4. Document Percussion Instrument Mapping

Add note about `<score-instrument>` to percussion line mapping:

> **Percussion Mapping:**
> - `<score-instrument>` in part-list defines available instruments
> - `<instrument>` on notes references the instrument ID
> - `display-step` and `display-octave` determine staff position
> - Common mappings (GM drum map, etc.) could be predefined

---

### 5. Add Frame Diagram Barre Example

Barre notation is common in chord diagrams:

```xml
<frame>
  <frame-strings>6</frame-strings>
  <frame-frets>4</frame-frets>
  <first-fret>1</first-fret>
  <frame-note>
    <string>6</string>
    <fret>1</fret>
    <barre type="start"/>
  </frame-note>
  <frame-note>
    <string>1</string>
    <fret>1</fret>
    <barre type="stop"/>
  </frame-note>
</frame>
```

```lisp
(frame
  :strings 6
  :frets 4
  :first-fret 1
  :notes ((frame-note :string 6 :fret 1 :barre :start)
          (frame-note :string 1 :fret 1 :barre :stop)))
```

---

## Open Questions Resolved

### Q: Should IR use structured chord data or preserve text-based shortcuts like "Cm7"?

**Recommendation:** Both — structured for IR, text for display.

**Rationale:**
1. MusicXML's `<kind>` has both structured value (`minor-seventh`) and text attribute (`"m7"`)
2. IR should preserve both:
   ```lisp
   (kind :value :minor-seventh :text "m7")
   ```
3. Structured data enables transposition, analysis, playback
4. Text preserves the original notation style (Cm7 vs Cmin7 vs C-7)
5. Round-tripping requires both

---

### Q: Should guitar tablature be moved to an earlier tier?

**Recommendation:** The basic elements (string, fret) are already in Chunk 6. Additional TAB features can remain deferred.

**Rationale:**
1. `<string>` and `<fret>` technical elements are covered
2. TAB-specific features like:
   - `<staff-type>TAB</staff-type>`
   - `<staff-details>` with `<staff-tuning>`
   - Specialized noteheads
   
   These are less common and can remain deferred.
3. Most guitar music can be represented with standard notation + string/fret annotations

---

### Q: How much MIDI-specific data should IR preserve vs. compute on export?

**Recommendation:** Preserve all MIDI data in IR; compute only what's missing.

**Rationale:**
1. Round-tripping requires preserving original MIDI settings
2. User may have specific MIDI channel/program assignments
3. On export, compute defaults only for missing values:
   - Default MIDI channel based on part order
   - Default program based on `instrument-sound`
   - Default volume/pan if not specified
4. `<midi-instrument>` and `<midi-device>` are already in Chunk 4

---

### Q: Should print/layout elements be separate from musical content?

**Recommendation:** Interleaved in IR, matching MusicXML structure.

**Rationale:**
1. MusicXML interleaves `<print>` elements with musical content
2. Page/system breaks are position-specific (after measure 16, etc.)
3. Separating them loses the positional information
4. IR can have a `MusicDataElement::Print` variant (already in Chunk 3)
5. Higher-level processing can extract layout if needed

---

### Q: Percussion instrument-to-line mapping?

**Recommendation:** Defer to implementation; use General MIDI drum map as default.

**Rationale:**
1. MusicXML uses `display-step`/`display-octave` for positioning
2. These are already captured in `<unpitched>` element
3. Instrument-to-position mapping varies by score
4. Implementation can provide GM drum map defaults
5. Custom mappings via `<score-instrument>` definitions

---

## New Questions Identified

- [ ] **Harmony placement:** The `<harmony>` element appears in measure content (like directions). Does it need offset for precise positioning? (Answer: Yes, it has optional `<offset>`.)

- [ ] **Frame as standalone vs. in harmony:** Can `<frame>` appear outside of `<harmony>`? (Answer: No, it's always a child of `<harmony>`.)

- [ ] **Figured bass duration:** How does figured bass duration interact with note durations? Does it have its own duration or align with bass notes?

- [ ] **Scordatura scope:** Is scordatura a one-time indication or does it persist until changed? (Answer: Persists until another scordatura or "normal tuning" indication.)

- [ ] **Play element completeness:** The `<play>` element has many sub-elements (ipa, mute, semi-pitched, etc.). Document all or defer?

---

## Cross-Chunk Concerns

### Chunk 1 (Core Note)

The `PitchRest::Unpitched` variant is defined but not fully documented. This chunk provides additional context for percussion notation.

### Chunk 3 (Measure Structure)

The `MusicDataElement` enum includes:
- `Harmony(Harmony)` — referenced here
- `FiguredBass(FiguredBass)` — referenced here
- `Print(Print)` — referenced here

Ensure the struct definitions here align with those enum variants.

### Chunk 4 (Part & Score)

MIDI elements (`midi-instrument`, `midi-device`) are covered in Chunk 4's `ScorePart`. Ensure consistency.

`<credit>` and `<defaults>` are fully covered in Chunk 4 — remove from this chunk or add cross-reference.

### Chunk 5 (Directions)

Many of these elements appear in `<direction>`:
- `<harp-pedals>`
- `<accordion-registration>`
- `<scordatura>`
- Percussion pictograms

Ensure the `DirectionTypeContent` enum includes these variants.

### Chunk 6 (Notations — Technical)

Tablature elements (`string`, `fret`, `hammer-on`, `pull-off`, `bend`) are covered in Chunk 6. This chunk adds context but shouldn't duplicate.

### Priority Reassessment

Based on this review, suggested priority changes:

| Element | Current | Suggested | Reason |
|---------|---------|-----------|--------|
| Chord symbols | Medium | **High** | Very common in lead sheets |
| Fret diagrams | Medium | Medium | Already have string/fret |
| Unpitched | Medium | **Already covered** | In Chunk 1 |
| Sound | Medium | **Already covered** | In Chunk 5 |
| Print | Medium | Medium | Layout feature |
