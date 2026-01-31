# Claude Chat Prompt: Chunk Review Template

> **Instructions:** Use this template when starting a new chat session to review a specific chunk. Replace `[N]` with the chunk number and paste the chunk content where indicated.

---

## System Context

You are reviewing S-expression mappings for Fermata, a Lisp DSL that compiles to MusicXML. A previous Claude Code session analyzed the MusicXML schema and proposed mappings. Your job is to review these for correctness, consistency, and quality.

## Project Background

**Fermata** is an S-expression DSL for music notation. Key decisions already made:

1. **Architecture:** Typed Music IR as the hub (not generic S-exprs)
2. **Pitch:** lowercase scientific notation (`c4`, `f#5`, `bb3`)
3. **Duration:** short (`:q`) and long (`:quarter`) forms, plus British (`:crotchet`)
4. **Dynamics:** separate positioned elements
5. **Tuplets:** wrapper form with explicit note durations
6. **Parser:** `nom` (Rust)

**IR Design Goals:**
- Lossless round-tripping with MusicXML
- Clean mapping to typed Rust structures
- Preserve MusicXML's semantic distinctions

## Your Task

Review Chunk [N]: **[Chunk Name]** for:

### 1. Correctness
- Do the S-expr mappings accurately represent MusicXML semantics?
- Are required vs. optional elements correctly identified?
- Are enumerations complete?

### 2. Consistency
- Do naming conventions match our established patterns?
- Are similar concepts handled the same way?
- Do cross-references to other chunks make sense?

### 3. Completeness
- Are any important elements missing?
- Are the examples sufficient?
- Are edge cases addressed?

### 4. Rust IR Viability
- Can these S-exprs map cleanly to Rust structs/enums?
- Are there any representations that would be awkward in Rust?

### 5. Open Questions
- Review each open question and provide your recommendation
- Identify any new questions the analysis missed

## Output Format

Produce a review document with this structure:

```markdown
# Review: Chunk [N] — [Name]

## Summary

[2-3 sentence overall assessment]

## Approved Mappings

These mappings are correct and ready for implementation:
- `element-name` ✓
- `element-name` ✓

## Required Changes

### [Element or Issue Name]

**Problem:** [Description]

**Current:**
```lisp
(current-form ...)
```

**Should be:**
```lisp
(corrected-form ...)
```

**Rationale:** [Why this change is necessary]

---

## Suggested Improvements

### [Improvement Name]

[Description of optional enhancement]

---

## Open Questions Resolved

### Q: [Original question]

**Recommendation:** [Your answer]

**Rationale:** [Why]

---

## New Questions Identified

- [ ] [New question 1]
- [ ] [New question 2]

---

## Cross-Chunk Concerns

[Any issues that affect other chunks]
```

---

## Chunk [N] Content

[PASTE THE CHUNK CONTENT HERE]

---

## Additional Context

### MusicXML Reference

If you need to verify something against the MusicXML spec, key resources are:
- XSD documentation in `<xs:documentation>` elements
- W3C docs at: https://www.w3.org/2021/06/musicxml40/

### Other Chunks (for cross-reference)

[Optionally paste summaries of related chunks if relevant]

---

**Begin your review. Be thorough but constructive.**
