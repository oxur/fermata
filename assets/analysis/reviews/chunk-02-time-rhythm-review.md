# Review: Chunk 2 — Time & Rhythm

## Summary

This is a well-structured chunk that correctly handles the tricky distinction between sound/playback elements (`tie`, `time-modification`) and notation/display elements (`tied`, `tuplet`). The examples are excellent, particularly the tie chain example (Example 4) which demonstrates the `stop`+`start` pattern. There are some inconsistencies in how `notations` is represented that need to be resolved, and a few mapping details need clarification, but the core design is sound and ready for implementation with minor revisions.

## Approved Mappings

These mappings are correct and ready for implementation:

- `time-modification` ✔ — Ratio representation is clean and complete
- `tie` ✔ — Simple start/stop with proper list representation for multiple ties
- `tied` ✔ — All type variants including `let-ring` and `continue`
- `grace` ✔ — Sub-form structure with all timing attributes

---

## Required Changes

### 1. Notations Wrapper Inconsistency

**Problem:** The `notations` representation is inconsistent and confusing. Sometimes it appears as a double-wrapped list, sometimes as a single form.

**Current (Example 1, lines 478-479):**
```lisp
:notations ((notations
              (tuplet :type :start :bracket :yes))))
```

**Current (Example 4, line 662):**
```lisp
:notations ((notations (tied :type :start))))
```

**Problem Analysis:** The outer list `((...))` suggests `:notations` contains a list of `notations` groups. But in MusicXML, a note has a single `<notations>` element (or none), which can contain multiple notation children. The double-parenthesis structure is confusing.

**Should be (Option A — Recommended):**
```lisp
;; Single notations container with multiple children
:notations (notations
             (tuplet :type :start :bracket :yes)
             (articulations (accent)))
```

**Should be (Option B — If multiple notation groups needed):**
```lisp
;; If MusicXML truly allows multiple <notations> elements
:notations ((tuplet :type :start :bracket :yes)
            (tied :type :start))
```

**Rationale:** Looking at the MusicXML schema, `<notations>` appears 0-or-more times within `<note>`, and each contains multiple notation items. If we need to support multiple `<notations>` blocks, use a clearer structure. If only one block is typical, flatten it.

**Action Required:** Clarify the exact MusicXML structure and choose a consistent representation.

---

### 2. Tuplet-Portion Naming Mismatch

**Problem:** The S-expr uses `tuplet-portion` but MusicXML uses `tuplet-actual` and `tuplet-normal`. The Rust struct is named `TupletPortion`.

**Current (lines 113-118):**
```lisp
:actual (tuplet-portion
          :number 3
          :type :eighth)
:normal (tuplet-portion
          :number 2
          :type :eighth)
```

**Should be:**
```lisp
:actual (tuplet-actual
          :number 3
          :type :eighth)
:normal (tuplet-normal
          :number 2
          :type :eighth)
```

**Rationale:** The S-expr should match MusicXML element names for round-tripping clarity. The Rust struct can still be named `TupletPortion` since both `tuplet-actual` and `tuplet-normal` share the same structure.

---

### 3. Normal-Dot Representation Unclear

**Problem:** The `normal-dots` mapping says "count of dots" but doesn't show how multiple `<normal-dot/>` elements map to a count.

**Current (line 66):**
```lisp
:normal-dots (count of dots)
```

**Should be:**
```lisp
;; In MusicXML, multiple <normal-dot/> elements
<time-modification>
  <actual-notes>3</actual-notes>
  <normal-notes>2</normal-notes>
  <normal-type>eighth</normal-type>
  <normal-dot/>
  <normal-dot/>
</time-modification>

;; S-expr options:

;; Option A (count):
:normal-dots 2

;; Option B (list, like note dots):
:normal-dots ((dot) (dot))
```

**Recommendation:** Use a simple count (`:normal-dots 2`) since `<normal-dot/>` has no attributes in this context, unlike note `<dot>` elements which can have placement.

---

### 4. Tuplet `number` Attribute Default

**Problem:** The `number` attribute for nested tuplets defaults to 1 in MusicXML. Should we include it explicitly or omit when 1?

**Current:** Not shown in basic examples, mentioned only for nested tuplets.

**Should be:** Document the rule explicitly:
- Omit `:number` when value is 1 (matches default)
- Include `:number N` when N > 1 (for nested tuplets)

**Rationale:** Consistency with "omit when default" principle established in Chunk 1.

---

### 5. Missing Voice Field in Some Examples

**Problem:** Some examples include `:voice "1"` and some omit it. This creates ambiguity.

**Current (Example 5, lines 725-728):**
```lisp
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :type :sixteenth
  ...)
```

No `:voice` field shown, but it was present in earlier examples.

**Should be:** Either consistently include `:voice "1"` in all examples OR add a note that `voice` is optional and these examples omit it for brevity.

---

## Suggested Improvements

### 1. Add Dotted Note in Tuplet Example

The chunk covers `normal-dots` conceptually but doesn't show a real example. A dotted-note tuplet (e.g., swing eighths as dotted-eighth + sixteenth in triplet feel) would clarify this edge case:

```xml
<time-modification>
  <actual-notes>2</actual-notes>
  <normal-notes>3</normal-notes>
  <normal-type>eighth</normal-type>
  <normal-dot/>
</time-modification>
```

---

### 2. Cross-Reference to Slur Element

Example 3 (Grace Note) uses `(slur :type :start :number 1)` but slurs aren't defined in this chunk. Add a note: "See Chunk N for slur element definition."

---

### 3. Add Let-Ring Example with Context

The `let-ring` tied type is mentioned but not shown in a full example. For guitar/percussion music, this is common:

```lisp
(note
  :pitch (pitch :step :E :octave 2)
  :duration 4
  :voice "1"
  :type :quarter
  :notations (notations
               (tied :type :let-ring)))
```

---

### 4. Document Grace Note + Chord Interaction

Can grace notes be part of a chord? If so, how does `:chord t` interact with `:grace (grace ...)`? This edge case should be documented or deferred to an open question.

---

### 5. Add `time-only` Tie Attribute Example

The `time-only` attribute for repeats is mentioned but not shown:

```lisp
;; Tie that only applies on specific repeat passes
(tie :type :start :time-only "1,2")
```

---

## Open Questions Resolved

### Q: How to associate tuplet start/stop across notes in IR? By `number` attribute alone?

**Recommendation:** Yes, by `number` attribute alone — this matches MusicXML's design.

**Rationale:** 
1. MusicXML uses `number` (1-16) to pair start/stop tuplets
2. The IR should preserve this mechanism for round-tripping
3. Higher-level Fermata processing can build explicit tuplet groups during semantic analysis
4. For simple (non-nested) tuplets, `number` defaults to 1 and can be omitted

---

### Q: Should IR compute actual sounding duration from `steal-time-*` or leave that to playback?

**Recommendation:** Leave duration computation to playback.

**Rationale:**
1. The IR layer should be a faithful representation of MusicXML, not a computed interpretation
2. `steal-time-*` values are percentages — the actual stolen time depends on the following/previous note's duration, which requires context
3. Playback engines handle this interpretation; the IR just stores the attributes
4. `make-time` is in divisions and could theoretically be computed, but for consistency, leave all timing computation to later stages

---

### Q: Nested tuplets — Rarely used but supported. Need to test with real-world examples.

**Recommendation:** Support via `number` attribute; defer extensive testing to implementation phase.

**Rationale:**
1. The current design supports nested tuplets via `number` attribute (1-16)
2. Real-world nested tuplets are rare (maybe some Chopin, Liszt, or contemporary music)
3. The MusicXML structure handles it correctly; we just need to preserve it
4. Create a test case during implementation, but don't block the design phase

---

## New Questions Identified

- [ ] **Notations container structure:** Does MusicXML allow multiple `<notations>` elements per note, or exactly one? The current representation suggests multiple, but examples show one. Clarify and standardize.

- [ ] **Tuplet display without time-modification:** Can a `<tuplet>` notation appear without corresponding `<time-modification>`? (e.g., for display-only tuplet bracket on non-standard groupings). If so, how should IR handle?

- [ ] **Grace note chords:** Can multiple grace notes form a chord (using `:chord t`)? Document the expected structure.

- [ ] **Tied `continue` type:** When exactly is `<tied type="continue"/>` used vs. paired stop+start? Is this for very long ties (3+ notes)?

- [ ] **Bezier control points:** The `tied` element mentions bezier attributes. Should these be in a `:bezier` sub-form or kept flat? (Similar to print-style question in Chunk 1.)

---

## Cross-Chunk Concerns

### Chunk 1 (Core Note)

The `Note` struct in Chunk 1 includes:
- `ties: Vec<Tie>` — matches this chunk's `(tie ...)` form ✔
- `time_modification: Option<TimeModification>` — matches ✔
- `notations: Vec<Notations>` — **needs alignment** with the representation chosen here

Ensure the Chunk 1 struct matches whatever `notations` structure is finalized.

### Slurs Chunk (TBD)

Example 3 uses `slur` elements. Ensure:
- Slur follows same start/stop pattern as `tied`
- `number` attribute works the same way (for multiple simultaneous slurs)
- Slur is defined in a notations chunk with cross-reference here

### Articulations/Ornaments Chunk (TBD)

The `<notations>` element contains many children beyond ties and tuplets:
- articulations
- ornaments  
- dynamics
- fermatas
- arpeggiate
- etc.

Ensure those chunks reference the `notations` container structure established here.

### Beaming Chunk (TBD)

Grace notes often have beaming. Ensure:
- Grace notes can have `:beams` field
- Beam behavior with grace notes is documented (typically separate beam groups from main notes)
