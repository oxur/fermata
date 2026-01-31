# Review: Chunk 4 — Part & Score

## Summary

This chunk provides excellent coverage of the top-level document structure, with clear examples ranging from minimal valid scores to full orchestral layouts with part groupings and MIDI settings. The `PartListItem` enum elegantly handles the interleaved `score-part` and `part-group` elements. The decision to focus on `score-partwise` (over `score-timewise`) is correct. There are some minor inconsistencies in child element representation and a few keyword naming choices to revisit, but overall this is comprehensive and well-structured.

## Approved Mappings

These mappings are correct and ready for implementation:

- `score-partwise` ✔ — Clean root structure with version and header
- `part` ✔ — Simple container with required id and measures
- `part-list` ✔ — `PartListItem` enum is elegant solution
- `part-group` ✔ — Start/stop with symbol, name, barline
- `score-part` ✔ — Full instrument metadata support
- `work` ✔ — Number, title, opus link
- `identification` ✔ — Creators, rights, encoding well-handled
- `credit` ✔ — Page-positioned text with formatting
- `score-instrument` ✔ — Name, sound, virtual instrument
- `midi-device` / `midi-instrument` ✔ — Complete MIDI mapping

---

## Required Changes

### 1. Inconsistent Child vs. Keyword Representation

**Problem:** Some elements use positional children while others use keyword pairs inconsistently. Compare the two patterns:

**Pattern A — Positional children (identification, lines 365-379):**
```lisp
(identification
  (creator :type :composer "Johann Sebastian Bach")
  (rights "Copyright © 2024")
  (encoding ...))
```

**Pattern B — Keyword pairs (defaults, lines 454-469):**
```lisp
(defaults
  :scaling (scaling ...)
  :page-layout (page-layout ...)
  :music-font (font ...))
```

**Recommendation:** Establish a clear rule. Suggest:
- **Repeatable elements** (Vec in Rust) → positional children: `(identification (creator ...) (creator ...) (rights ...))`
- **Singular/optional elements** (Option in Rust) → keyword pairs: `(defaults :scaling ... :page-layout ...)`

This matches the Rust struct shapes and makes parsing predictable.

---

### 2. Movement Number/Title Representation

**Problem:** In Example 2, `movement-number` and `movement-title` appear as bare keywords at the `score-partwise` level, but they're not inside a sub-form. This is inconsistent with how other header elements (work, identification) are handled.

**Current (lines 653-654):**
```lisp
(score-partwise :version "4.0"
  (work ...)
  :movement-number "1"
  :movement-title "Prelude"
  (identification ...))
```

**Problem:** Mixing positional children `(work ...)` with keyword pairs `:movement-number` at the same level is confusing.

**Should be (Option A — All keywords):**
```lisp
(score-partwise :version "4.0"
  :work (work :number "BWV 846" :title "Prelude in C Major")
  :movement-number "1"
  :movement-title "Prelude"
  :identification (identification ...)
  :part-list (part-list ...)
  :parts ((part :id "P1" ...)))
```

**Should be (Option B — All positional):**
```lisp
(score-partwise :version "4.0"
  (work :number "BWV 846" :title "Prelude in C Major")
  (movement-number "1")
  (movement-title "Prelude")
  (identification ...)
  (part-list ...)
  (part :id "P1" ...))
```

**Recommendation:** Option B (all positional) is more consistent with XML structure where these are all child elements.

---

### 3. Part-Group Number Type

**Problem:** The S-expr shows `:number 1` (integer) but the Rust struct has `number: Option<String>`.

**Current (line 159):**
```lisp
(part-group :type :start :number 1 ...)
```

**Rust (line 191):**
```rust
pub number: Option<String>,
```

**Should be:**
```lisp
(part-group :type :start :number "1" ...)
```

**Rationale:** MusicXML's `number` attribute is a token type that can be non-numeric. Use string for consistency with the Rust type.

---

### 4. Creator Text Content Placement

**Problem:** In the S-expr, the creator text appears as a trailing positional argument, which is unusual for our patterns.

**Current (line 366):**
```lisp
(creator :type :composer "Johann Sebastian Bach")
```

**Question:** Is the text positional (third argument) or should it use a keyword?

**Should be (explicit keyword):**
```lisp
(creator :type :composer :name "Johann Sebastian Bach")
```

**Or (if preserving the "text content" pattern):** Document this as a special case where the final positional argument represents XML text content, similar to how MusicXML handles it.

**Recommendation:** Use `:name` or `:text` keyword for clarity. The pattern `(element :attr1 val1 "text content")` is valid but should be documented if used.

---

### 5. Encoding Date Element Name

**Problem:** The S-expr uses `:date` but MusicXML element is `<encoding-date>`.

**Current (line 372):**
```lisp
(encoding
  :software "Finale 27"
  :date "2024-01-15"
  ...)
```

**Should be:**
```lisp
(encoding
  :software "Finale 27"
  :encoding-date "2024-01-15"
  ...)
```

**Rationale:** Match MusicXML element names for round-tripping clarity. Abbreviations should be avoided in the IR.

---

## Suggested Improvements

### 1. Add Virtual Instrument Example

The `ScoreInstrument` struct includes `virtual_instrument` but no example shows it. Modern notation software uses this for sample libraries:

```lisp
(score-instrument :id "P1-I1"
  :name "Solo Violin"
  :sound "strings.violin"
  :virtual (virtual-instrument
             :library "Vienna Symphonic Library"
             :name "Violin 1 Sustain"))
```

---

### 2. Document GroupBarline Values

The `<group-barline>` element has specific values (yes, no, Mensurstrich) that affect how barlines connect across grouped parts. Add these to the mapping rules.

---

### 3. Add Nested Part Groups Example

Orchestral scores often have nested groups (e.g., "Strings" containing "Violins" subgroup). The `number` attribute distinguishes these:

```lisp
(part-list
  (part-group :type :start :number "1" :symbol :bracket :name "Orchestra")
  (part-group :type :start :number "2" :symbol :brace :name "Violins")
  (score-part :id "P1" :name "Violin I")
  (score-part :id "P2" :name "Violin II")
  (part-group :type :stop :number "2")
  (score-part :id "P3" :name "Viola")
  (part-group :type :stop :number "1"))
```

---

### 4. Credit-Image Support

The chunk mentions `<credit-image>` in the element table but doesn't show an example. Add:

```lisp
(credit :page 1
  (credit-image :source "logo.png" :type "image/png"
    :default-x 100 :default-y 1500
    :height 50 :width 200))
```

---

### 5. Document Standard instrument-sound Values

The `instrument-sound` element uses standardized strings from the General MIDI sound set extensions. Reference the MusicXML sounds DTD or provide common examples:
- `keyboard.piano.grand`
- `strings.violin`
- `brass.trumpet`
- `wind.flutes.flute`

---

## Open Questions Resolved

### Q: Do we need to parse score-timewise and convert to partwise? Or reject it?

**Recommendation:** Parse and convert to partwise.

**Rationale:**
1. The conversion is mathematically straightforward (transpose the measures×parts matrix)
2. Some legacy files and export tools produce timewise format
3. Rejecting valid MusicXML would limit interoperability
4. The IR only needs to store partwise; the parser handles conversion

**Implementation:** Add a `from_timewise()` method that transposes the structure during parsing.

---

### Q: Should the IR validate that part IDs match score-part IDs?

**Recommendation:** Yes, validate during parsing, not in the IR types.

**Rationale:**
1. Invalid ID references indicate a malformed file
2. Validation at parse time gives clear error messages
3. The IR types themselves don't need to enforce this (Rust's type system can't easily express this constraint)
4. A `validate()` method on `ScorePartwise` can check referential integrity

---

### Q: The `<opus>` element links to external files. How to handle?

**Recommendation:** Store the xlink attributes; don't resolve external references.

**Rationale:**
1. Opus links are metadata, not essential for rendering
2. Resolving external files adds complexity (network access, file paths)
3. The IR should faithfully store what's in the XML
4. Higher-level tools can resolve opus links if needed
5. The `Opus` struct already captures all xlink attributes ✔

---

### Q: Layout defaults — Store in IR but ignore for initial rendering?

**Recommendation:** Yes, store in IR; defer rendering support.

**Rationale:**
1. Required for round-tripping
2. Layout rendering is complex and can be implemented later
3. The data structure is already well-defined
4. Initial implementation can use sensible defaults
5. Document that `defaults` is "parse and store, render later"

---

## New Questions Identified

- [ ] **Text content pattern:** How do we consistently handle XML elements with text content plus attributes? Document the `(element :attr val "text")` pattern or use explicit `:text` keyword.

- [ ] **Font defaults inheritance:** The `defaults` section specifies fonts for music/words/lyrics. How do these interact with inline font specifications on individual elements?

- [ ] **Multiple rights/sources:** The `Identification` struct has `rights: Vec<TypedText>` and `relation: Vec<TypedText>`. When are multiples used? Document or add example.

- [ ] **Player element:** The `ScorePart` struct includes `player: Vec<Player>` but it's not covered. What is this for? (Answer: MusicXML 4.0 added `<player>` for performer assignment in educational/practice contexts)

- [ ] **Part-link element:** Similarly, `part_link: Vec<PartLink>` is in the struct but not documented. This is for linked parts (e.g., concert vs. transposed scores).

---

## Cross-Chunk Concerns

### Chunk 3 (Measure Structure)

The `Part` struct contains `measures: Vec<Measure>`. Ensure:
- The `Measure` struct from Chunk 3 is the same type
- `MusicDataElement` enum is imported/shared correctly
- Implicit measures (pickup/anacrusis) are handled in both chunks

### Chunk 1 (Core Note)

The `instrument` field on notes references `ScoreInstrument.id`:
```lisp
(note :instrument (instrument :id "P1-I1") ...)
```
Ensure the ID validation covers this reference too.

### Notation Software Round-Trip Testing

This chunk defines what Fermata exports. Consider creating test cases that:
1. Import MusicXML from Finale, Sibelius, MuseScore, Dorico
2. Round-trip through Fermata
3. Verify structure preservation (especially part groups, MIDI settings, credits)

### Encoding/Software Identification

When Fermata exports MusicXML, it should identify itself:
```lisp
(encoding
  :software "Fermata 0.1.0"
  :encoding-date "2024-03-15")
```
Ensure this is auto-populated during export.
