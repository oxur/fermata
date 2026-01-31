# Review: Chunk 8 — Multi-Voice and Multi-Staff

## Summary

This is an excellent, clearly-written chunk that explains the critical concepts of MusicXML's sequential stream model with backup/forward cursor mechanics. The distinction between voice (melodic line) and staff (physical location) is well-articulated. The examples progress beautifully from two voices on one staff (SATB soprano/alto) through piano grand staff to the complex four-part chorale example. The cross-staff notation example (Example 4) is particularly valuable. There are only minor issues to address — this chunk is nearly ready for implementation as-is.

## Approved Mappings

These mappings are correct and ready for implementation:

- `voice` ✔ — String type correctly preserves MusicXML flexibility
- `staff` ✔ — Positive integer, 1-indexed
- `backup` ✔ — Clean duration-based cursor movement
- `forward` ✔ — Duration with optional voice/staff assignment

---

## Required Changes

### 1. Voice String Consistency Check

**Observation:** The chunk correctly identifies voice as a string type (`:voice "1"`), but earlier chunks (Chunk 1) had inconsistency showing `:voice 1` (integer). 

**Verification needed:** Confirm all examples across chunks use `:voice "1"` (string) not `:voice 1` (integer).

**Current (correct):**
```lisp
:voice "1"
```

**Rationale:** MusicXML allows non-numeric voice identifiers. String type is correct.

---

### 2. Time Signature Representation in Example 3

**Problem:** Example 3 uses the simple `:beats`/`:beat-type` form which was flagged for potential change in Chunk 3.

**Current (line 495):**
```lisp
:time (time :beats 4 :beat-type 4)
```

**Note:** This should remain consistent with whatever decision is made in Chunk 3 about time signature representation. Not a problem specific to this chunk.

---

### 3. Dots Representation

**Problem:** Example 3 shows `:dots ((dot))` which is consistent with the double-parenthesis pattern for lists.

**Current (line 513):**
```lisp
:dots ((dot))
```

**Analysis:** This is correct — matches Chunk 1's pattern. No change needed.

---

## Suggested Improvements

### 1. Add Backup/Forward Validation Rules

Document validation rules for backup/forward:

> **Validation Rules:**
> - `backup` duration should not exceed the cumulative duration since measure start or last backup
> - `forward` duration + current position should not exceed measure duration
> - Negative positions are invalid
> - Parsers should warn on position errors but may attempt recovery

---

### 2. Add Voice/Staff Numbering Conventions

Document common conventions:

> **Conventional Numbering:**
> - **Voices 1-2:** Staff 1 (treble/right hand)
> - **Voices 3-4:** Staff 2 (bass/left hand)  
> - **Soprano:** Voice 1, stems up
> - **Alto:** Voice 2, stems down
> - **Tenor:** Voice 3, stems up
> - **Bass:** Voice 4, stems down
>
> These are conventions, not requirements. MusicXML allows any voice on any staff.

---

### 3. Add Example of Multiple Backups

Show a measure with 3+ voices requiring multiple backup operations:

```lisp
(measure :number "1"
  (attributes :divisions 4)
  
  ;; Voice 1: whole note
  (note :pitch (pitch :step :C :octave 5) :duration 4 :voice "1" :type :whole)
  
  (backup :duration 4)
  
  ;; Voice 2: two half notes
  (note :pitch (pitch :step :E :octave 4) :duration 2 :voice "2" :type :half)
  (note :pitch (pitch :step :F :octave 4) :duration 2 :voice "2" :type :half)
  
  (backup :duration 4)
  
  ;; Voice 3: four quarter notes
  (note :pitch (pitch :step :G :octave 3) :duration 1 :voice "3" :type :quarter)
  (note :pitch (pitch :step :A :octave 3) :duration 1 :voice "3" :type :quarter)
  (note :pitch (pitch :step :B :octave 3) :duration 1 :voice "3" :type :quarter)
  (note :pitch (pitch :step :C :octave 4) :duration 1 :voice "3" :type :quarter))
```

---

### 4. Document Forward vs. Explicit Rest

Clarify when to use `forward` vs. an actual rest note:

> **Forward vs. Rest:**
> - `(forward :duration 1)` — Invisible; advances time with no printed rest
> - `(note :rest (rest) :duration 1)` — Visible rest printed on staff
>
> Use `forward` for:
> - Voices that don't play from the start of a measure
> - Implicit rests where the other voice(s) provide rhythmic clarity
>
> Use explicit rests for:
> - Single-voice passages where rests must be shown
> - Editorial clarity

---

### 5. Add Cross-Staff Beam Example

Extend Example 4 to show cross-staff beaming:

```lisp
;; Cross-staff arpeggiated figure with beam
(note
  :pitch (pitch :step :C :octave 3)
  :duration 1
  :voice "1"
  :type :eighth
  :staff 2
  :beams ((beam :number 1 :value :begin)))
(note
  :pitch (pitch :step :G :octave 4)
  :duration 1
  :voice "1"
  :type :eighth
  :staff 1
  :beams ((beam :number 1 :value :continue)))
(note
  :pitch (pitch :step :C :octave 5)
  :duration 1
  :voice "1"
  :type :eighth
  :staff 1
  :beams ((beam :number 1 :value :end)))
```

---

## Open Questions Resolved

### Q: Should omitted `:voice` default to "1"?

**Recommendation:** No default; leave unspecified as `None`.

**Rationale:**
1. MusicXML doesn't mandate a default — behavior varies by application
2. Single-voice music often omits voice entirely
3. The IR should represent what's in the source, not assume
4. If voice is absent, `voice: Option<String>` is `None`
5. Higher-level processing can assign default voices if needed

---

### Q: Should Fermata syntax allow `(voice "1" ...)` wrapper?

**Recommendation:** Yes, at the higher-level Fermata DSL, not in the IR.

**Rationale:**
1. The IR must preserve MusicXML's per-note voice assignment
2. A wrapper syntax is ergonomic for composition:
   ```lisp
   ;; Higher-level Fermata (not IR)
   (voice "1"
     (q c4) (q d4) (q e4))
   ```
3. The compiler expands this to per-note `:voice "1"` assignments
4. This follows the pattern: IR is explicit/verbose, DSL is ergonomic

---

### Q: How to handle beams that connect notes on different staves?

**Recommendation:** Already handled — `:staff` on each note; beams connect normally.

**Rationale:**
1. Example 4 shows cross-staff chord notation
2. For cross-staff beams, each note has its `:staff` assignment
3. Beam elements use begin/continue/end regardless of staff
4. Rendering engine draws beam across staves
5. No special beam syntax needed

**Example documented in Suggested Improvements above.**

---

## New Questions Identified

- [ ] **Voice collision handling:** When two voices have notes at the same time on the same staff, how is horizontal offset determined? Is this purely a rendering concern, or does MusicXML encode it?

- [ ] **Maximum voices:** Is there a practical limit on voice count? MusicXML doesn't specify one. Document that voice is any string.

- [ ] **Backup past measure start:** What happens if backup duration exceeds the time since measure start? Is this an error or does it wrap to previous measure? (Answer: It's an error; backup is measure-local.)

- [ ] **Forward beyond measure:** Similarly, can forward extend past measure end? (Answer: It's an error.)

- [ ] **Voice consistency:** Should the IR validate that a voice number is used consistently (same staff, same stem direction) throughout a piece? Or is this a linting concern for higher levels?

---

## Cross-Chunk Concerns

### Chunk 1 (Core Note)

Verify the `Note` struct includes:
```rust
pub voice: Option<String>,
pub staff: Option<StaffNumber>,
```

And that all Chunk 1 examples use `:voice "1"` (string) not `:voice 1` (integer).

### Chunk 3 (Measure Structure)

The `MusicDataElement` enum includes:
```rust
Backup(Backup),
Forward(Forward),
```

Ensure the `Backup` and `Forward` structs here match that reference.

The `attributes` element defines `:staves` count — ensure cross-reference.

### Chunk 7 (Beaming)

Cross-staff beaming relies on:
- Notes having different `:staff` values
- Beams connecting them normally

Add cross-reference from Chunk 7 to this chunk's Example 4.

### Chord and Voice Interaction

Example 5 (chorale) shows an interesting pattern:
- Alto is `:chord t` with Soprano but has `:voice "2"` 
- This means "same onset time, different voice"

This is correct MusicXML — chords can span voices when they occur simultaneously. Document this explicitly.

### Direction Voice/Staff

The `<direction>` element (Chunk 5) also has optional `<voice>` and `<staff>` children. Ensure:
- Same `:voice "1"` string pattern
- Same `:staff 1` integer pattern
- Cross-reference between chunks

### Higher-Level Fermata DSL

The voice wrapper syntax suggested here:
```lisp
(voice "1" 
  (q c4) (q d4))
```

Should be documented in a "Fermata DSL" design document (separate from IR chunks). This is a usability feature that compiles down to the explicit per-note representation.
