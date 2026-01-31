# Claude Chat Prompt: Assembly & Consistency Check

> **Instructions:** Use this prompt after all chunk reviews are complete. Include all reviewed chunks and their review documents.

---

## System Context

You are performing the final assembly and consistency check for Fermata's Music IR — the S-expression intermediate representation that maps to MusicXML.

Previous phases have:

1. Triaged MusicXML elements by priority
2. Produced S-expr mappings for 10 chunks
3. Reviewed each chunk individually

Now we need to consolidate everything into a unified specification and verify cross-chunk consistency.

## Project Background

**Fermata** is an S-expression DSL for music notation. Key decisions:

- **Architecture:** Typed Music IR as the hub
- **Pitch:** lowercase scientific notation (`c4`, `f#5`, `bb3`)
- **Duration:** short (`:q`), long (`:quarter`), and British (`:crotchet`) forms
- **Dynamics:** separate positioned elements
- **Tuplets:** wrapper form with explicit note durations
- **Parser:** `nom` (Rust)
- **Goal:** Lossless MusicXML round-tripping

## Your Task

Produce a comprehensive **Assembly Report** that:

### 1. Naming Consistency Audit

Check that all chunks use consistent naming for:

- **Element names:** Are they uniformly hyphenated lowercase?
- **Keywords:** Are attribute keywords consistent? (e.g., always `:type` not sometimes `:kind`)
- **Enum values:** Are similar concepts named the same way across chunks?
- **Positional vs keyword arguments:** Is the pattern consistent?

Produce a table of any inconsistencies found.

### 2. Cross-Reference Validation

Verify that:

- When Chunk A references an element defined in Chunk B, the reference is correct
- Parent-child relationships are consistent across chunks
- Shared types (like duration, pitch) are used uniformly

List any cross-reference issues.

### 3. Unified Type Definitions

Consolidate all Rust type definitions into a coherent set:

- Remove duplicates
- Resolve naming conflicts
- Organize into logical modules

Produce a draft `types.rs` outline showing all IR types.

### 4. Completeness Check

Verify coverage against the original triage:

- Are all Tier 1 (Core) elements mapped?
- Are all Tier 2 (Important) elements mapped?
- What gaps exist in Tier 3 (Secondary)?
- Confirm Tier 4 (Deferred) is intentionally deferred

### 5. Resolved vs. Outstanding Questions

Compile:

- All questions that were resolved during reviews
- All questions that remain open
- Recommendations for resolving open questions

### 6. Final IR Specification Draft

Produce a clean, consolidated document showing:

- All S-expr forms organized by category
- Complete syntax reference
- Type definitions
- Example round-trip (Fermata → MusicXML → Fermata)

## Output Format

```markdown
# Fermata Music IR: Assembly Report

## Executive Summary

[Overall status, key findings, readiness assessment]

---

## 1. Naming Consistency Audit

### Inconsistencies Found

| Issue | Chunk A | Chunk B | Recommendation |
|-------|---------|---------|----------------|
| [desc] | `name-a` | `name-b` | Use `name-a` |

### Standardized Naming Conventions

[Final conventions document]

---

## 2. Cross-Reference Validation

### Issues Found

| Reference | In Chunk | Refers To | Issue |
|-----------|----------|-----------|-------|
| `element` | 5 | 1 | [description] |

### Resolution

[How each issue was resolved]

---

## 3. Unified Type Definitions

### Module Organization

```

fermata_ir/
├── mod.rs
├── pitch.rs      # Pitch, Step, Alter, Octave
├── duration.rs   # Duration, DurationType
├── note.rs       # Note, Rest, Chord, Grace
├── measure.rs    # Measure, Attributes, Time, Key, Clef
├── part.rs       # Part, PartList, ScorePartwise
├── direction.rs  # Direction, DirectionType, Dynamics, etc.
├── notation.rs   # Notations, Articulations, Ornaments, etc.
├── layout.rs     # Beam, Stem, print attributes
├── voice.rs      # Voice, Staff, Backup, Forward
└── lyric.rs      # Lyric, Syllabic, Text

```

### Complete Type Listing

[All types with their fields]

---

## 4. Completeness Check

### Tier 1 (Core) Coverage: [X/Y] ✓

| Element | Status | Notes |
|---------|--------|-------|
| `<note>` | ✓ Mapped | Chunk 1 |
| ... | ... | ... |

### Tier 2 (Important) Coverage: [X/Y]

[Same format]

### Tier 3 (Secondary) Coverage: [X/Y]

[Same format]

### Tier 4 (Deferred): Confirmed Deferred

[List of intentionally deferred elements]

---

## 5. Questions Status

### Resolved Questions

| Question | Resolution | Resolved In |
|----------|------------|-------------|
| [question] | [answer] | Chunk N review |

### Outstanding Questions

| Question | Context | Priority | Recommendation |
|----------|---------|----------|----------------|
| [question] | [context] | High/Med/Low | [suggestion] |

---

## 6. Final IR Specification Draft

### Quick Reference

[Table of all forms with brief syntax]

### Detailed Specification

[Full specification organized by category]

### Round-Trip Example

**Fermata Source:**
```lisp
[Complete example]
```

**Generated MusicXML:**

```xml
[Corresponding XML]
```

**Re-imported Fermata:**

```lisp
[Should match original]
```

---

## Appendix A: Full S-Expr Grammar (EBNF)

[Formal grammar if useful]

## Appendix B: Change Log from Reviews

[Summary of all changes made during reviews]

```

---

## Input Documents

### Reviewed Chunks

```

assets/analysis/outputs/chunk-01-core-note.md
assets/analysis/outputs/chunk-02-time-rhythm.md
assets/analysis/outputs/chunk-03-measure-structure.md
assets/analysis/outputs/chunk-04-part-score.md
assets/analysis/outputs/chunk-05-directions.md
assets/analysis/outputs/chunk-06-notations.md
assets/analysis/outputs/chunk-07-beaming-stems.md
assets/analysis/outputs/chunk-08-multi-voice.md
assets/analysis/outputs/chunk-09-lyrics.md
assets/analysis/outputs/chunk-10-advanced.md

```

### Review Documents

```

assets/analysis/reviews/chunk-01-core-note-review.md
assets/analysis/reviews/chunk-02-time-rhythm-review.md
assets/analysis/reviews/chunk-03-measure-structure-review.md
assets/analysis/reviews/chunk-04-part-score-review.md
assets/analysis/reviews/chunk-05-directions-review.md
assets/analysis/reviews/chunk-06-notations-review.md
assets/analysis/reviews/chunk-07-beaming-stems-review.md
assets/analysis/reviews/chunk-08-multi-voice-review.md
assets/analysis/reviews/chunk-09-lyrics-review.md
assets/analysis/reviews/chunk-10-advanced-review.md

```

### Original Triage

`crates/design/dev/0001-claude-code-prompt-musicxml-triage.md`

---

**Begin the assembly. Be meticulous about consistency.**
