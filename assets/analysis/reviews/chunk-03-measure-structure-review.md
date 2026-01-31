# Review: Chunk 3 — Measure Structure

## Summary

This is a comprehensive and well-organized chunk covering the essential structural elements of musical measures. The `MusicDataElement` enum is particularly well-designed, capturing all the possible contents of a measure. The key signature representation (fifths-based with non-traditional support), time signature flexibility (including additive and senza-misura), and barline/repeat/ending structures are all correctly mapped. There are some minor inconsistencies in plural vs. singular naming and a few edge cases to clarify, but overall this chunk is solid and ready for implementation.

## Approved Mappings

These mappings are correct and ready for implementation:

- `measure` ✔ — Proper container with string number, implicit support
- `attributes` ✔ — Clean container for all measure-level properties
- `divisions` ✔ — Simple integer, critical for duration interpretation
- `key` (traditional) ✔ — Fifths-based with mode, correct range
- `key` (non-traditional) ✔ — Step/alter pairs for custom key signatures
- `time` (simple) ✔ — Beats/beat-type as strings, handles additive
- `time` (senza-misura) ✔ — Properly handles unmeasured music
- `clef` ✔ — Sign/line/octave-change covers all standard clefs
- `barline` ✔ — Location, style, repeat, ending all correct
- `repeat` ✔ — Forward/backward direction with times attribute
- `ending` ✔ — Start/stop/discontinue with number and text

---

## Required Changes

### 1. Singular vs. Plural Inconsistency in Attributes

**Problem:** The S-expr examples use singular forms (`:key`, `:time`, `:clef`) but the Rust struct uses plural (`keys: Vec<Key>`, `times: Vec<Time>`, `clefs: Vec<Clef>`). This creates ambiguity for multi-staff scenarios.

**Current (line 119-121):**
```lisp
(attributes
  :divisions 4
  :key (key :fifths 2 :mode :major)
  :time (time :beats "4" :beat-type "4")
  :clef (clef :sign :G :line 2))
```

**But for multiple clefs (line 756-757):**
```lisp
:clefs ((clef :number 1 :sign :G :line 2)
        (clef :number 2 :sign :F :line 4)))
```

**Should be (consistent plural form):**
```lisp
;; Single key/time/clef (still uses plural keywords)
(attributes
  :divisions 4
  :keys ((key :fifths 2 :mode :major))
  :times ((time :beats "4" :beat-type "4"))
  :clefs ((clef :sign :G :line 2)))

;; Multiple clefs
(attributes
  :divisions 4
  :staves 2
  :clefs ((clef :number 1 :sign :G :line 2)
          (clef :number 2 :sign :F :line 4)))
```

**Rationale:** Using consistent plural forms (`:keys`, `:times`, `:clefs`) matches the Rust Vec types and avoids special-casing single vs. multiple elements. The list always contains 1+ items when present.

---

### 2. Non-Traditional Key Representation

**Problem:** The non-traditional key uses a different structure with `:non-traditional` flag and unnamed list. This is inconsistent with other patterns.

**Current (lines 217-221):**
```lisp
(key
  :non-traditional
  ((key-step :step :F :alter 1)
   (key-step :step :C :alter 1)
   (key-step :step :G :alter 1)))
```

**Should be:**
```lisp
(key
  :key-steps ((key-step :step :F :alter 1)
              (key-step :step :C :alter 1)
              (key-step :step :G :alter 1)))
```

**Rationale:** 
1. The presence of `:key-steps` implicitly indicates non-traditional — no need for a separate flag
2. Named keyword (`:key-steps`) is more consistent with other list patterns
3. Maps cleanly to `KeyContent::NonTraditional(Vec<KeyStep>)`

---

### 3. Time Signature Composite Representation

**Problem:** Simple time signatures use `:beats`/`:beat-type` directly, but composite signatures use `:signatures` with nested forms. This creates parsing complexity.

**Current (lines 305-336):**
```lisp
;; Simple
(time :beats "4" :beat-type "4")

;; Composite
(time
  :signatures ((time-signature :beats "2" :beat-type "4")
               (time-signature :beats "3" :beat-type "8")))
```

**Should be (unified approach):**
```lisp
;; Simple (still a list, just one element)
(time :signatures ((time-signature :beats "4" :beat-type "4")))

;; Composite
(time :signatures ((time-signature :beats "2" :beat-type "4")
                   (time-signature :beats "3" :beat-type "8")))

;; Or, allow shorthand for common case:
(time :beats "4" :beat-type "4")  ;; Parser expands to single-element signatures list
```

**Rationale:** Consistent internal representation simplifies processing. The shorthand can be syntactic sugar that the parser expands.

---

### 4. Clef `number` vs `staff` Attribute Naming

**Problem:** The chunk uses both `:number` and `:staff` to refer to the staff number for clefs.

**Current (line 481):**
```text
`number` attr → `:staff` (for multi-staff)
```

**But example (line 756):**
```lisp
(clef :number 1 :sign :G :line 2)
```

**Should be:** Pick one and be consistent. Recommend `:number` since that's the MusicXML attribute name.

```lisp
(clef :number 1 :sign :G :line 2)
```

**Rationale:** IR should match MusicXML attribute names for round-tripping. The mapping rule text should say `number` attr → `:number`.

---

### 5. Barline Children Structure

**Problem:** Barline examples show children as keywords (`:bar-style`, `:repeat`, `:ending`) but some of these are elements in MusicXML, not attributes.

**Current (lines 569-572):**
```lisp
(barline :location :left
  :bar-style :heavy-light
  :repeat (repeat :direction :forward))
```

**Analysis:** This is actually fine — the keyword approach works for both attributes and simple child elements. However, the structure should be documented clearly: MusicXML child elements become keyword pairs in the S-expr, not nested positional forms.

**No change needed**, but add documentation note: "Child elements like `<bar-style>`, `<repeat>`, and `<ending>` are represented as keyword pairs, not positional children."

---

## Suggested Improvements

### 1. Add Mode Enumeration to Design Decisions

The `Mode` enum includes church modes (Dorian, Phrygian, etc.) — this is great! Consider adding a note about why these are included (historical music, jazz education, modal jazz).

---

### 2. Document Divisions Best Practices

Add guidance on choosing divisions values:
- 1: Simple music (quarter notes only)
- 4: Allows sixteenths
- 24: Allows triplets and sixteenths (LCM of 3 and 8)
- 960: "MIDI-like" high precision (common in professional software)

---

### 3. Add Mid-Measure Clef Change Example

Mid-measure clef changes are common (e.g., piano music). Add an example showing the `additional="yes"` attribute:

```lisp
(measure :number "5"
  (note ...)  ;; in treble
  (attributes
    :clefs ((clef :sign :F :line 4 :additional :yes)))
  (note ...))  ;; now in bass
```

---

### 4. Add Key Cancellation Example

The `<cancel>` element for showing naturals when changing keys is mentioned in Open Questions. Add a brief example showing the pattern:

```xml
<key>
  <cancel>-3</cancel>  <!-- Cancel 3 flats -->
  <fifths>2</fifths>   <!-- New key: 2 sharps -->
</key>
```

```lisp
(key :cancel -3 :fifths 2 :mode :major)
```

---

### 5. Cross-Reference Forward/Backup Elements

The `MusicDataElement` enum includes `Backup` and `Forward` which are critical for multi-voice music but aren't covered in this chunk. Add a note: "See Chunk N for `backup` and `forward` elements used in multi-voice notation."

---

## Open Questions Resolved

### Q: Mid-measure attributes — How to associate attribute changes with specific time points?

**Recommendation:** By position in the measure's element sequence — this is exactly how MusicXML works.

**Rationale:**
1. MusicXML uses document order: elements are processed sequentially
2. An `<attributes>` element affects all subsequent notes until the next `<attributes>`
3. The IR preserves this by keeping `Vec<MusicDataElement>` in order
4. Time position is implicit from cumulative durations of preceding notes
5. This matches the "preserve MusicXML structure" design goal

---

### Q: Cancel key signatures — `<cancel>` element for showing naturals. Include in IR?

**Recommendation:** Yes, include in IR.

**Rationale:**
1. Key cancellation is a real notational feature (shows naturals before new key signature)
2. Omitting it loses visual information
3. Simple to add: `pub cancel: Option<Cancel>` in `TraditionalKey`
4. Round-tripping requires it

**S-expr form:**
```lisp
(key :cancel -3 :fifths 2 :mode :major)
```

---

### Q: Interchangeable time — MusicXML's `<interchangeable>` for 6/8 = 2/4 equivalence. Defer?

**Recommendation:** Include in IR structure, but mark as low-priority for implementation.

**Rationale:**
1. It's part of the schema and needed for complete round-tripping
2. Rarely used in practice (mostly educational/analytical contexts)
3. The `TimeContent::Measured` struct already has `interchangeable: Option<Interchangeable>`
4. Parser can initially skip it; add support later when needed

---

### Q: Measure width — Layout hint. Include or defer to layout engine?

**Recommendation:** Include in IR as `Option<Tenths>`.

**Rationale:**
1. It's a simple attribute, costs nothing to store
2. Required for faithful round-tripping
3. Layout engines can use or ignore it as they see fit
4. Already shown in examples (`:width 200`)

---

## New Questions Identified

- [ ] **Backup/Forward chunk:** Which chunk covers `<backup>` and `<forward>` elements? These are essential for multi-voice notation and referenced in `MusicDataElement` but not defined here.

- [ ] **Direction element:** The `MusicDataElement` enum includes `Direction` — which chunk covers tempo markings, dynamics placement, rehearsal marks, etc.?

- [ ] **Print element:** The `MusicDataElement` includes `Print` for system/page breaks and layout. Which chunk covers this?

- [ ] **Staff number type:** Is `StaffNumber` a u8, u16, or dedicated newtype? Should be consistent across all chunks.

- [ ] **Measure implicit vs. non-controlling:** What's the semantic difference? Both relate to multi-part scores. Document when each is used.

---

## Cross-Chunk Concerns

### Chunk 1 (Core Note)

The `Note` struct includes `staff: Option<StaffNumber>`. Ensure:
- `StaffNumber` type is consistent with this chunk's clef `:number` attribute
- Multi-staff examples in both chunks use the same conventions

### Chunk 2 (Time & Rhythm)

The `divisions` value defined here directly affects how `duration` is interpreted in notes. Consider adding a cross-reference:
- Chunk 2 should reference "divisions is set in `<attributes>`, see Chunk 3"
- This chunk should note "duration interpretation depends on divisions, see Chunk 1 note element"

### Backup/Forward Chunk (TBD)

The `MusicDataElement` enum references `Backup` and `Forward`. These are critical for:
- Multi-voice music (backup to write second voice)
- Chord symbols and figured bass placement
- Cross-staff notation

Ensure that chunk references the `MusicDataElement` container defined here.

### Direction Chunk (TBD)

Directions (tempo, dynamics, rehearsal marks) appear within measures as `MusicDataElement::Direction`. Ensure:
- The Direction struct is defined consistently
- Positioning relative to notes is documented (directions typically precede the note they apply to)

### Part/Score Structure Chunk (TBD)

Measures exist within parts. Ensure:
- Part → Measure containment is documented
- `non-controlling` attribute meaning for multi-part scores is explained
- Measure numbers across parts are coordinated
