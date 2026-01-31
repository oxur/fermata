# Review: Chunk 1 — Core Note

## Summary

This is a strong foundational chunk that correctly captures the essential complexity of MusicXML's `<note>` element. The separation of sounding duration (`duration`) from notated duration (`type`) is correctly preserved, the Rust IR structures are well-designed with proper enum variants for the note content model, and the examples are clear and instructive. There are a few consistency issues and some areas where the S-expr representation could be tightened up, but overall this is ready for implementation with minor revisions.

## Approved Mappings

These mappings are correct and ready for implementation:

- `pitch` ✔ — Clean, accurate, supports microtones
- `step` ✔ — Keyword enum, matches MusicXML
- `alter` ✔ — Numeric semitones, decimal support
- `octave` ✔ — Integer 0-9, correct range
- `duration` ✔ — Positive divisions, integer preferred
- `rest` ✔ — All three variants (simple, measure, positioned) covered
- `chord` ✔ — Boolean flag preserves MusicXML semantics
- `dot` ✔ — List representation preserves count and attributes
- `accidental` ✔ — Value + all boolean attributes preserved

---

## Required Changes

### 1. Voice Type Inconsistency

**Problem:** The `voice` field is shown as a string in examples but the Rust struct uses `Option<String>`. However, in MusicXML, `<voice>` is indeed a string (not an integer), but the S-expr example shows it inconsistently.

**Current:**
```lisp
:voice 1
```

**Should be:**
```lisp
:voice "1"
```

**Rationale:** MusicXML's `<voice>` is a string type (to support values like "1a" in complex multi-voice scenarios). The later examples correctly use `"1"`, but the initial mapping example uses bare `1`. Should be consistent throughout.

---

### 2. Boolean Representation for `:yes`/`:no` vs `t`/`nil`

**Problem:** MusicXML uses `yes-no` type for many attributes. The chunk shows `:measure :yes` for rest but `:chord t` for chord presence. The representation should be systematically defined.

**Current:**
```lisp
;; For rest measure attribute:
(rest :measure :yes)

;; For chord presence:
:chord t
```

**Should be:** Establish a clear rule:

- For MusicXML `yes-no` attributes → use `:yes`/`:no` keywords
- For pure boolean flags that are presence-based (like `<chord/>`) → use `t`/`nil`

**Rationale:** This preserves the semantic distinction between "attribute with yes/no value" and "empty element presence." Document this rule in a conventions section.

---

### 3. Note Type Wrapper Inconsistency

**Problem:** When `<type>` has no `size` attribute, it's shown as a bare keyword. When it has a `size` attribute, it becomes a nested form. This creates parsing ambiguity.

**Current:**
```lisp
;; Without size:
:type :quarter

;; With size:
:type (note-type :value :quarter :size :cue)
```

**Should be (Option A — Recommended):**
```lisp
;; Without size:
:type :quarter

;; With size (use separate field):
:type :quarter
:type-size :cue
```

**Rationale:** Option A is more ergonomic for the common case (no size). The two-field approach maps cleanly to the Rust struct and avoids polymorphic parsing of the `:type` value. Alternative Option B (always use nested form) is also acceptable but more verbose.

---

### 4. Add Grace Note Example

**Problem:** The chunk describes grace notes in the `NoteContent` enum but doesn't provide an S-expr example.

**Current:** No example provided.

**Should be:**
```xml
<note>
  <grace slash="yes"/>
  <pitch><step>D</step><octave>5</octave></pitch>
  <voice>1</voice>
  <type>eighth</type>
</note>
```

```lisp
(note
  :grace (grace :slash :yes)
  :pitch (pitch :step :D :octave 5)
  :voice "1"
  :type :eighth)
```

**Rationale:** Grace notes are common and have no `<duration>` element. An example demonstrates this important variant of `NoteContent`.

---

## Suggested Improvements

### 1. Add Print-Style Attribute Group Documentation

The chunk mentions that many elements have `default-x`, `default-y`, `font-*`, `color` but doesn't establish a pattern. Add a brief note that these will be handled in a "common attributes" section or chunk, with a forward reference.

---

### 2. Clarify Divisions Normalization Strategy

Line 272 mentions "The IR may normalize to a fixed division (e.g., 960 per quarter) during parsing" — this is a significant design decision that should be promoted to the Design Decisions table or addressed as a formal Open Question.

---

### 3. Add Cue Note Example

Similar to the grace note issue, a cue note example would round out the three content variants and show the `cue` boolean usage.

---

### 4. Document Rust Enum Naming Convention

The Rust enum values like `N1024th` use `N` prefix to avoid starting with a digit. This convention should be documented for consistency across all chunks.

---

### 5. Clarify Duration Alias Scope

The duration keyword aliases (`:w`, `:h`, `:q`, etc.) are excellent but should be clearly marked as NOT part of the IR — they belong in higher-level Fermata parsing. Consider moving to a "Fermata Conventions" or "Syntactic Sugar" section.

---

## Open Questions Resolved

### Q: Should we include `:id` on every form or only when present?

**Recommendation:** Only when present.

**Rationale:** 
1. Most notes don't have IDs in typical MusicXML
2. Adding `:id nil` everywhere adds noise
3. Rust's `Option<String>` handles this cleanly — `None` when absent
4. This matches the "omit when default" principle used elsewhere (e.g., `:alter` omitted when 0)

---

### Q: Should print-style attributes be grouped into a `:style` sub-form or kept flat?

**Recommendation:** Keep flat at the IR level, but define a `PrintStyle` struct in Rust.

**Rationale:**
1. MusicXML keeps them flat as attributes, so the IR should match for round-tripping
2. The Rust IR can use composition: `pub print_style: PrintStyle` where `PrintStyle` contains all the positioning/font fields
3. S-expr stays flat: `:default-x 10 :default-y 20 :color "#FF0000"`
4. Higher-level Fermata syntax could introduce grouping if ergonomically useful

**S-expr form:**
```lisp
(note
  :pitch (pitch :step :C :octave 4)
  :default-x 80
  :default-y -10
  :color "#FF0000"
  ...)
```

**Rust IR:**
```rust
pub struct Note {
    pub print_style: PrintStyle,
    // ... other fields
}

pub struct PrintStyle {
    pub default_x: Option<Tenths>,
    pub default_y: Option<Tenths>,
    pub relative_x: Option<Tenths>,
    pub relative_y: Option<Tenths>,
    pub font_family: Option<String>,
    pub font_style: Option<FontStyle>,
    pub font_size: Option<FontSize>,
    pub font_weight: Option<FontWeight>,
    pub color: Option<Color>,
}
```

---

### Q: Unpitched notes — How to represent `<unpitched>` for percussion?

**Recommendation:** Defer to Chunk 10 as noted, but sketch the pattern here for consistency:

```lisp
(note
  :unpitched (unpitched :display-step :E :display-octave 4)
  :duration 1
  :voice "1"
  :type :quarter
  :instrument (instrument :id "P1-I36"))
```

**Rationale:** This parallels the `rest` structure with display position and maintains consistency with the `PitchRest` enum design.

---

## New Questions Identified

- [ ] **Divisions normalization:** Should the IR normalize all `duration` values to a fixed divisions base (e.g., 960 per quarter)? If so, how do we preserve the original divisions for round-tripping? This affects playback accuracy and import/export fidelity.

- [ ] **Tie representation:** The `NoteContent` enum shows `ties: Vec<Tie>` but ties aren't covered in this chunk. Should there be a forward reference to wherever ties are covered, or should basic tie representation be included here?

- [ ] **Beam reference:** The Rust struct includes `beams: Vec<Beam>` but beams aren't covered. Which chunk handles beams?

- [ ] **Note ordering in S-expr:** Should there be a canonical ordering of keyword arguments in note forms? For parser simplicity, we might want to specify that certain fields come first (e.g., `:pitch`/`:rest`/`:unpitched` before `:duration`).

- [ ] **Duration alias location:** Where should the duration keyword aliases (`:w`, `:h`, `:q`, etc.) be formally specified? They're not IR-level, so they need a home in the higher-level Fermata syntax documentation.

---

## Cross-Chunk Concerns

### Beams/Ties/Slurs Chunk

The `Note` struct references `beams: Vec<Beam>`, `ties: Vec<Tie>`, and `notations: Vec<Notations>`. Ensure those chunks reference back to this note structure and maintain consistency with the field names and types defined here.

### Percussion Chunk (Chunk 10?)

Unpitched notes need to follow the same `NoteContent` pattern established here. The `PitchRest::Unpitched` variant is defined but not fully documented — ensure Chunk 10 provides complete coverage.

### Common Attributes Chunk

The print-style attributes (`default-x`, `default-y`, `font-*`, `color`) appear on many elements. Consider a dedicated section or chunk that defines the `PrintStyle` struct and S-expr pattern once, then reference it from all chunks that use these attributes.

### Time Modification / Tuplets Chunk

The `Note` struct includes `time_modification: Option<TimeModification>` — ensure the tuplets chunk references this field and explains how it interacts with `duration` and `type`.
