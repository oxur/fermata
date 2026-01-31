# Review: Chunk 9 — Lyrics

## Summary

This is a thorough and well-organized chunk covering the complexities of lyric representation in MusicXML. The syllabic handling explanation is particularly clear, and the examples progress nicely from simple words through hyphenation, melismas, multiple verses, and elisions. The `LyricContent` enum elegantly handles the different content types (syllable, extend-only, laughing, humming). There are some representation choices for elisions that need clarification, and the usual simple/complex form pattern appears, but overall this is solid work.

## Approved Mappings

These mappings are correct and ready for implementation:

- `lyric` ✔ — Container with number/name identification
- `syllabic` ✔ — Four-value enum (single/begin/middle/end)
- `text` ✔ — Simple string or attributed form
- `extend` ✔ — Start/stop/continue for melismas
- `laughing` / `humming` ✔ — Boolean flags for special vocal effects
- `end-line` / `end-paragraph` ✔ — Karaoke/MIDI support

---

## Required Changes

### 1. Elision Representation Needs Clarity

**Problem:** Two different representations are shown for elision without a clear decision:

**Option A — Extensions list (lines 246-249):**
```lisp
(lyric :number "1"
  :syllabic :single
  :text "heav"
  :extensions ((elision :value "'") (text "n")))
```

**Option B — Flat keywords (lines 254-258):**
```lisp
(lyric :number "1"
  :syllabic :single
  :text "heav"
  :elision "'"
  :text-2 "n")
```

**Recommendation:** Use Option A (`:extensions` list).

**Rationale:**
1. MusicXML allows multiple elision-text pairs in sequence (though rare)
2. The `:extensions` list maps cleanly to `Vec<LyricExtension>` in Rust
3. `:text-2`, `:text-3` etc. doesn't scale
4. Option A matches the Rust struct design in the chunk

**Action:** Remove Option B from the chunk or clearly mark it as "not recommended."

---

### 2. Text Simple/Complex Form Pattern

**Problem:** Same pattern as stem/notehead — simple form for plain text, complex for attributed.

**Simple:**
```lisp
:text "love"
```

**Complex:**
```lisp
:text (text :value "SHOUT" :font-weight :bold :color "#FF0000")
```

**Recommendation:** This is fine — document consistently:
- Simple string when no attributes
- Nested `(text :value ... :attr ...)` when attributes present
- Parser normalizes simple to complex internally

---

### 3. Lyrics List Pattern

**Observation:** Uses `((lyric ...) (lyric ...))` pattern:

```lisp
:lyrics ((lyric :number "1" :syllabic :single :text "Day")
         (lyric :number "2" :syllabic :single :text "Night"))
```

**Analysis:** This is correct and consistent with `:beams`, `:ties`, etc. A note can have multiple lyrics (multiple verses). The outer parens are the list, inner are each lyric form.

**No change needed** — but ensure consistency documentation across all list-valued fields.

---

### 4. Extend-Only Lyric Representation

**Problem:** When a note has only an extend (middle/end of melisma), the representation could be cleaner.

**Current (lines 342-344):**
```lisp
:lyrics ((lyric :number "1"
           :extend (extend :type :continue))))
```

**Analysis:** This is correct — an extend-only lyric has no `:text` or `:syllabic`. The `LyricContent::ExtendOnly` enum variant handles this.

**No change needed** — but document: "When `:text` is absent and `:extend` is present, this represents a melisma continuation (no printed text)."

---

## Suggested Improvements

### 1. Add Complex Elision Example

Show a more complex elision case (multiple elision-text pairs):

```lisp
;; "going to" contracted to "gonna" sung as one syllable
(lyric :number "1"
  :syllabic :single
  :text "gon"
  :extensions ((elision :value "")
               (syllabic :end)
               (text "na")))
```

Note: The extension can include its own `:syllabic` for proper hyphenation behavior.

---

### 2. Add Language Attribute Example

Show `xml:lang` usage for multilingual scores:

```lisp
;; Latin text with language tag
(lyric :number "1"
  :lang "la"
  :syllabic :begin
  :text "Ky")

;; Or on the text element itself
(lyric :number "1"
  :syllabic :begin
  :text (text :value "Ky" :lang "la"))
```

---

### 3. Document Lyric Placement

Add note about `:placement` attribute:

> **Lyric Placement:**
> - `:placement :below` — Standard for vocal music (default)
> - `:placement :above` — Sometimes used for translation lines
> - Multiple verses stack vertically: verse 1 closest to staff, verse 2 below, etc.

---

### 4. Add Print-Object Interaction

Document how `print-object` works with lyrics:

```lisp
;; Lyric exists for playback/MIDI but not printed
(lyric :number "1"
  :print-object :no
  :syllabic :single
  :text "hidden")
```

---

### 5. Add Chord Note Lyric Convention

Document how lyrics work with chords:

> **Lyrics on Chords:**
> - Typically only the first note of a chord has lyrics
> - Chord notes (`:chord t`) usually don't have lyrics
> - If they do, it represents a divisi vocal line

---

## Open Questions Resolved

### Q: Should there be a simpler syntax for common elisions like "heav'n"?

**Recommendation:** Keep `:extensions` for IR; add sugar in higher-level DSL.

**Rationale:**
1. The IR must faithfully represent MusicXML structure
2. Elision sequences can be arbitrarily complex
3. Higher-level Fermata DSL could support:
   ```lisp
   ;; Fermata sugar (not IR)
   (lyric "heav'n")  ; Auto-detects elision from apostrophe
   ```
4. The compiler expands to full `:extensions` representation

---

### Q: Should `print-lyric` be on the note or lyric level in IR?

**Recommendation:** Both, matching MusicXML.

**Rationale:**
1. MusicXML has `print-lyric` as a note attribute (suppresses all lyrics)
2. Individual `<lyric>` elements have `print-object` (suppresses that lyric)
3. IR should preserve both for round-tripping:
   - Note: `:print-lyric :no` (suppresses all lyrics on this note)
   - Lyric: `:print-object :no` (suppresses this specific lyric)

---

### Q: How to handle mixed-language lyrics?

**Recommendation:** Use `:lang` at most specific level.

**Rationale:**
1. `<lyric>` can have `xml:lang` for the entire lyric
2. `<text>` can override with its own `xml:lang`
3. IR preserves both:
   ```lisp
   (lyric :number "1" :lang "la"
     :syllabic :single
     :text (text :value "Amen" :lang "he"))  ; Override for this word
   ```
4. Rendering uses most specific language for font selection, hyphenation rules, etc.

---

## New Questions Identified

- [ ] **Lyric number as string vs. integer:** The chunk shows `:number "1"` (string). Is this correct? MusicXML uses NMTOKEN which allows digits, so "1" is valid. Verify consistency with other numbered elements.

- [ ] **Multiple text elements without elision:** Can a lyric have multiple `<text>` elements without intervening `<elision>`? If so, what does this mean? (Answer: Unlikely, but check schema.)

- [ ] **Extend without preceding syllable:** Can a lyric line start with an extend (orphaned melisma)? How to handle?

- [ ] **Lyric editorial markup:** The `<lyric>` can contain editorial elements (`<footnote>`, `<level>`). Are these captured in the IR?

- [ ] **Syllabic inference:** If `:syllabic` is omitted, should parsers infer from context (hyphens in text, position in word)? Or require explicit syllabic?

---

## Cross-Chunk Concerns

### Chunk 1 (Core Note)

The `Note` struct should include:
```rust
pub lyrics: Vec<Lyric>,
```

And notes should support:
```rust
pub print_lyric: Option<YesNo>,  // Suppress all lyrics on this note
```

### Chunk 7 (Beaming)

Lyric syllables typically align with beamed notes. No direct interaction, but rhythmic grouping affects lyric placement.

### Chunk 5 (Directions)

Standalone text (not attached to notes) uses `<words>` in directions. Clarify distinction:
- `<lyric>` — Sung text attached to notes
- `<words>` — Expressive/tempo text in directions

### Chunk 4 (Part/Score)

Score-level settings may affect lyric fonts:
- `<defaults><lyric-font>` sets default font for lyrics
- Individual lyrics can override with `:font-*` attributes

### SMuFL Glyphs

The `<elision>` element can use `:smufl` for special glyphs. Ensure SMuFL glyph naming conventions are documented somewhere.

### MIDI/Karaoke Export

The `end-line` and `end-paragraph` elements are specifically for MIDI lyric meta-events (RP-017). If Fermata exports MIDI with lyrics, these flags affect timing of lyric display changes.
