# Review: Chunk 7 — Beaming & Stems

## Summary

This is a well-structured chunk covering beaming mechanics, stem direction, and notehead shapes. The beam level documentation (1-8 for eighth through 1024th notes) is particularly clear, and the examples progress nicely from simple eighth-note beams to partial beams (hooks) and fanned beams. The decision to allow simple forms (`:stem :up`, `:notehead :x`) while supporting full attribute forms is good ergonomics. The print-style attribute groups are well-documented. There are some minor representation inconsistencies to resolve, but overall this chunk is solid.

## Approved Mappings

These mappings are correct and ready for implementation:

- `beam` ✔ — Level-based with begin/continue/end/hook values
- `stem` ✔ — Simple and attributed forms both supported
- `notehead` ✔ — Comprehensive value list including shape notes
- Print attribute groups ✔ — Well-documented position/font/color

**Beam values:** ✔
- `:begin`, `:continue`, `:end`, `:forward-hook`, `:backward-hook`

**Stem values:** ✔
- `:up`, `:down`, `:none`, `:double`

**Notehead values:** ✔
- Full enumeration including percussion, shape notes, and special symbols

---

## Required Changes

### 1. Beams List Pattern — Same Issue as Notations

**Problem:** The beams representation uses the same double-parenthesis pattern:

**Current:**
```lisp
:beams ((beam :number 1 :value :begin))
```

**Analysis:** This is actually correct here since a note can have multiple beam elements (one per level). For 16th notes:
```lisp
:beams ((beam :number 1 :value :begin)
        (beam :number 2 :value :begin))
```

The outer parentheses denote the list, inner ones denote each beam. **This is fine** — but should be documented consistently with the notations pattern decision.

**Recommendation:** Document that `:beams` always takes a list of `(beam ...)` forms. No change needed, but ensure consistency with `:notations` and `:ties` patterns.

---

### 2. Stem Simple vs. Complex Form Inconsistency

**Problem:** The chunk shows two forms for stems but doesn't clearly define when each is used.

**Simple form:**
```lisp
:stem :up
```

**Complex form:**
```lisp
:stem (stem :value :down :default-y -35)
```

**Should document:** The parser accepts either:
- Simple keyword → `:stem :up` (when only direction matters)
- Nested form → `:stem (stem :value :down ...)` (when position/color attributes present)

**Recommendation:** Add explicit rule: "Use simple form when no attributes; use nested form when attributes are needed. Parser normalizes simple form to nested form internally."

---

### 3. Notehead Same Pattern as Stem

**Problem:** Same simple/complex duality:

**Simple:**
```lisp
:notehead :x
```

**Complex:**
```lisp
:notehead (notehead :value :triangle :filled :no)
```

**Recommendation:** Same documentation as stem — both forms valid, simple is sugar for nested with just `:value`.

---

### 4. Beam Number Type

**Problem:** The S-expr shows `:number 1` as integer, and the Rust type is `BeamLevel = u8`. This is correct, but should explicitly state it's 1-8, not 0-7.

**Current:**
```lisp
(beam :number 1 :value :begin)
```

**Recommendation:** Add note: "`:number` is 1-indexed (1-8), matching MusicXML. Level 1 = eighth notes."

---

### 5. Print-Style Grouping Decision Deferred

**Problem:** The chunk shows both flat and grouped approaches but doesn't decide:

**Flat (shown):**
```lisp
:default-x 10 :default-y 20 :font-family "Times" :font-size 12 :color "#000000"
```

**Grouped (suggested):**
```lisp
:style (style :x 10 :y 20 :font "Times" :size 12 :color "#000000")
```

**Recommendation:** Flat approach for IR (matches MusicXML, required for round-tripping). The grouped approach could be higher-level Fermata sugar. Mark this as a resolved question.

---

## Suggested Improvements

### 1. Add Grace Note Beaming Example

Grace notes often have separate beaming from main notes:

```lisp
;; Grace note group (beamed separately)
(note
  :grace (grace :slash :yes)
  :pitch (pitch :step :D :octave 5)
  :type :sixteenth
  :beams ((beam :number 1 :value :begin)
          (beam :number 2 :value :begin)))
(note
  :grace (grace)
  :pitch (pitch :step :C :octave 5)
  :type :sixteenth
  :beams ((beam :number 1 :value :end)
          (beam :number 2 :value :end)))
;; Main note (not beamed to grace notes)
(note
  :pitch (pitch :step :B :octave 4)
  :duration 4
  :type :quarter)
```

---

### 2. Add Cross-Staff Beaming Note

The Open Questions mention cross-staff beaming. Add a brief note:

> **Cross-staff beaming:** Notes can have `staff` attribute to indicate which staff they appear on. Beams connect notes regardless of staff assignment. The beam is typically drawn from the first note's staff. See Chunk 3 for multi-staff parts.

---

### 3. Add Tremolo Beam Clarification

Tremolos (covered in Chunk 6 ornaments) interact with beaming. Clarify:

> **Tremolo vs. beaming:** Single-note tremolos use `<tremolo>` in ornaments, not beams. Two-note tremolos use `<tremolo type="start/stop">`. The deprecated `repeater` attribute was for tremolo-like beams.

---

### 4. Document Beam Validation Rules

Add explicit validation:
- Every `:begin` must have matching `:end` at same level
- Levels must be contiguous (can't have level 2 without level 1)
- `:forward-hook` and `:backward-hook` are standalone (no matching pair)

---

### 5. Add Feathered/Fanned Beam Detail

The fanned beam example is good but could note:
- `:fan :accel` — beams converge (accelerando visual)
- `:fan :rit` — beams diverge (ritardando visual)
- Fan only on `:begin`, others continue/end normally

---

## Open Questions Resolved

### Q: Should IR store explicit beams, or allow implicit beaming to be computed?

**Recommendation:** Store explicit beams in IR; compute implicit beaming at a higher level.

**Rationale:**
1. MusicXML stores explicit beams — IR must match for round-tripping
2. Implicit beaming rules vary by time signature, style, and convention
3. The IR is a representation layer, not a computation layer
4. Higher-level Fermata syntax can omit beams; the compiler adds them
5. This follows the "IR matches MusicXML" principle

**Implementation:**
- IR: Always stores explicit `<beam>` elements
- Fermata DSL: Can use `(auto-beam ...)` wrapper that computes beams
- Export: Beams are already explicit, written as-is

---

### Q: Print attribute grouping — Keep flat or group into `:style` sub-form?

**Recommendation:** Keep flat in IR.

**Rationale:**
1. MusicXML has them as flat attributes — IR must match
2. Grouping loses the specific attribute names needed for round-tripping
3. Rust structs can use composition (`PrintStyle` containing `Position`, `Font`, etc.)
4. S-expr stays flat: `:default-x 10 :default-y 20 :color "#FF0000"`
5. Higher-level Fermata syntax could introduce grouping sugar

---

### Q: How to represent beams that cross staves?

**Recommendation:** Notes have `staff` attribute; beams connect regardless.

**Rationale:**
1. In MusicXML, cross-staff beaming uses `<staff>` element on each note
2. The `<beam>` elements connect notes normally (begin/continue/end)
3. Rendering determines beam position based on note staff assignments
4. No special beam syntax needed — just note staff assignment

**Example:**
```lisp
;; Note on staff 1
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :type :eighth
  :staff 1
  :beams ((beam :number 1 :value :begin)))

;; Note on staff 2 (cross-staff)
(note
  :pitch (pitch :step :C :octave 3)
  :duration 1
  :type :eighth
  :staff 2
  :beams ((beam :number 1 :value :end)))
```

---

## New Questions Identified

- [ ] **Beam color inheritance:** If beam 1 has a color, do beams 2+ inherit it? Or must each level specify color? Document MusicXML behavior.

- [ ] **Chord beaming:** In a chord, only one note typically has beam elements. Is this the first note? Document the convention.

- [ ] **Backward/forward hook positioning:** Does the hook direction affect rendering, or is it purely semantic? (Answer: It affects which side the partial beam appears on.)

- [ ] **Stem length adjustment:** The `default-y` on stem controls endpoint. How does this interact with beamed notes where the beam determines stem length?

- [ ] **Notehead `filled` override:** For half notes, `filled="yes"` makes them look like quarters. Is this common? Document use cases.

---

## Cross-Chunk Concerns

### Chunk 1 (Core Note)

The `Note` struct includes:
```rust
pub beams: Vec<Beam>,
pub stem: Option<Stem>,
pub notehead: Option<Notehead>,
```

Ensure these types match what's defined here. The `Vec<Beam>` correctly allows multiple beam levels.

### Chunk 2 (Time & Rhythm)

Grace notes have separate beaming. Ensure:
- Grace note examples show beaming
- Document that grace notes are not beamed with main notes

### Chunk 6 (Notations — Tremolo)

Tremolo interacts with beaming:
- Single-note tremolo: Uses `<tremolo>` ornament, not beams
- Two-note tremolo: Uses `<tremolo type="start/stop">`
- Document the distinction to avoid confusion

### Chunk 3 (Measure Structure — Multi-staff)

Cross-staff beaming relies on `<staff>` element. Ensure:
- Staff assignment is documented in Chunk 3
- Cross-reference from this chunk to Chunk 3

### Print-Style Attributes (All Chunks)

The print-style attribute groups defined here apply to many elements across all chunks. Consider:
- Creating a "Common Attributes" reference section
- Or documenting print-style once and referencing it
- The Rust `PrintStyle` struct can be shared across element types

### Unpitched Notes (Chunk 1 / Chunk 10?)

Example 4 shows unpitched notes with noteheads. Ensure:
- `unpitched` element is properly documented
- Percussion notation examples show notehead usage
