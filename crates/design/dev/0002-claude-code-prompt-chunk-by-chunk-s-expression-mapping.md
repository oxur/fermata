# Claude Code Prompt: Chunk-by-Chunk S-Expression Mapping

> **Instructions:** Give this prompt to Claude Code after the triage is complete. Include the triage output and the MusicXML schema files.

---

## Context

You are helping design **Fermata**, a Lisp-like DSL that compiles to MusicXML. We've completed a triage of MusicXML elements. Now we need to design the actual S-expression mappings.

**Important:** We're designing the **Music IR** — a low-level, lossless intermediate representation. This is NOT the final user-facing Fermata syntax (which will be higher-level and more ergonomic). The IR must:

1. **Capture everything MusicXML can express** — Lossless round-tripping
2. **Preserve MusicXML's semantic distinctions** — If MusicXML distinguishes two concepts, we do too
3. **Map cleanly to/from MusicXML** — Direct structural correspondence

## Prior Design Decisions

These decisions have already been made. Your mappings must respect them:

### Syntax Conventions
- **Pitch notation:** lowercase scientific pitch (`c4`, `f#5`, `bb3`, `cn4` for explicit natural)
- **Duration keywords:** support both short and long forms, plus British names
  - `:w` / `:whole` / `:semibreve`
  - `:h` / `:half` / `:minim`
  - `:q` / `:quarter` / `:crotchet`
  - `:8` / `:eighth` / `:quaver`
  - `:16` / `:sixteenth` / `:semiquaver`
  - `:32` / `:thirty-second` / `:demisemiquaver`
- **Keywords:** use Lisp-style keywords with colons (`:staccato`, `:fermata`)
- **Attributes:** as keyword-value pairs in the form head

### Structural Conventions
- **Dynamics:** separate positioned elements, not modifiers on notes
- **Tuplets:** wrapper form; notes inside specify their own durations
- **Internal duration:** will use rational representation

### S-Expression Style
```lisp
;; General form
(element-name :attr1 value1 :attr2 value2
  (child1 ...)
  (child2 ...))

;; Example note
(note
  :pitch (pitch :step c :alter 0 :octave 4)
  :duration 1
  :type :quarter
  :voice 1)
```

## Your Task

Produce **10 chunk files**, one for each category below. Work through them in order, as later chunks may reference earlier ones.

### Chunk Definitions

| Chunk | File | MusicXML Focus |
|-------|------|----------------|
| 1 | `chunk-01-core-note.md` | `<note>`, `<pitch>`, `<duration>`, `<type>`, `<rest>`, `<chord>`, `<dot>`, `<accidental>` |
| 2 | `chunk-02-time-rhythm.md` | `<time-modification>`, `<tuplet>`, `<tie>`, `<tied>`, grace notes |
| 3 | `chunk-03-measure-structure.md` | `<measure>`, `<attributes>`, `<divisions>`, `<time>`, `<key>`, `<clef>`, `<barline>` |
| 4 | `chunk-04-part-score.md` | `<part>`, `<part-list>`, `<score-partwise>`, `<score-timewise>`, `<identification>`, `<work>` |
| 5 | `chunk-05-directions.md` | `<direction>`, `<direction-type>`, `<dynamics>`, `<wedge>`, `<words>`, `<metronome>`, `<segno>`, `<coda>` |
| 6 | `chunk-06-notations.md` | `<notations>`, `<articulations>`, `<ornaments>`, `<technical>`, `<fermata>`, `<arpeggiate>` |
| 7 | `chunk-07-beaming-stems.md` | `<beam>`, `<stem>`, print/layout attributes |
| 8 | `chunk-08-multi-voice.md` | `<voice>`, `<staff>`, `<backup>`, `<forward>`, multi-staff handling |
| 9 | `chunk-09-lyrics.md` | `<lyric>`, `<text>`, `<syllabic>`, `<elision>`, `<extend>` |
| 10 | `chunk-10-advanced.md` | Everything deferred: figured bass, chord symbols, percussion, harp pedals, etc. |

## Output Format for Each Chunk

Each chunk file should follow this structure:

```markdown
# Chunk N: [Name]

## Overview

Brief description of what this chunk covers and its role in music notation.

## MusicXML Elements Covered

| Element | Type | Required Children | Optional Children | Attributes |
|---------|------|-------------------|-------------------|------------|
| `<element>` | complex | child1, child2 | child3, child4 | attr1, attr2 |

## S-Expression Mappings

### element-name

**MusicXML:**
```xml
<example attr="value">
  <child>content</child>
</example>
```

**S-expr:**
```lisp
(example :attr value
  (child content))
```

**Mapping Rules:**
- XML attribute `attr` → keyword `:attr`
- XML child `<child>` → nested form `(child ...)`
- Text content → final positional argument or `:text` keyword

**Type Definitions (for Rust IR):**
```rust
pub struct Example {
    pub attr: Option<String>,
    pub child: Child,
}
```

[Repeat for each element]

## Interrelationships

How elements in this chunk relate to each other and to elements in other chunks.

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| How to represent X | Y | Because Z |

## Open Questions

- [ ] Question 1 — context and options
- [ ] Question 2 — context and options

## Examples

### Example 1: [Description]

**Musical meaning:** [What this represents musically]

**MusicXML:**
```xml
[Full XML example]
```

**S-expr:**
```lisp
[Corresponding S-expr]
```

### Example 2: [Description]
[...]
```

## Guidelines

### General Principles

1. **One-to-one mapping where possible** — If MusicXML has one element, we have one form
2. **Preserve semantics** — Don't merge concepts that MusicXML keeps separate
3. **Use keywords for optional attributes** — Required elements as positional children
4. **Be explicit** — Prefer `:type :start` over `:start` when type is a distinct concept

### Naming Conventions

- Element names: lowercase with hyphens (`time-modification`)
- Keywords: lowercase with hyphens (`:print-object`)
- Enum values: keywords (`:sharp`, `:flat`, `:natural`)
- Boolean values: use actual booleans or `:yes`/`:no` to match MusicXML

### When in Doubt

- **Check the XSD documentation** — It often clarifies intent
- **Look at real-world examples** — How do Finale, Sibelius, MuseScore export this?
- **Document the question** — Add to Open Questions if genuinely uncertain

### Common Patterns

**Optional wrapper:**
```lisp
;; MusicXML: <notations><fermata/></notations>
;; S-expr: notations wrapping is explicit
(note ...
  (notations
    (fermata)))
```

**Repeatable elements:**
```lisp
;; MusicXML: multiple <beam> elements
;; S-expr: use a list
(note ...
  :beams ((beam :number 1 :type :begin)
          (beam :number 2 :type :begin)))
```

**Enumerated types:**
```lisp
;; MusicXML: <stem>down</stem>
;; S-expr: keyword value
(note ...
  :stem :down)
```

**Start/stop pairs:**
```lisp
;; MusicXML: <slur type="start" number="1"/>
;; S-expr: explicit type
(slur :type :start :number 1)
```

## Chunk-Specific Instructions

### Chunk 1 (Core Note)
- This is the foundation. Be very thorough.
- Pay special attention to the distinction between `<duration>` (sounding) and `<type>` (notated).
- Handle chord representation carefully (the `<chord/>` flag approach).

### Chunk 2 (Time & Rhythm)
- Tuplet handling is critical — reference our prior discussion about notes specifying their own durations.
- Grace notes have special duration semantics.

### Chunk 3 (Measure Structure)
- The `<attributes>` element is complex with many children.
- Time and key signatures have multiple representation options.

### Chunk 4 (Part & Score)
- Two document forms exist: `score-partwise` and `score-timewise`.
- Part metadata lives in `<part-list>`.

### Chunk 5 (Directions)
- `<direction>` is a wrapper; `<direction-type>` contains the actual content.
- Multiple direction-types can coexist.

### Chunk 6 (Notations)
- `<notations>` is a wrapper on notes.
- Many sub-categories: articulations, ornaments, technical, etc.

### Chunk 7 (Beaming & Stems)
- Beaming uses level numbers (1-8).
- Print/layout attributes may need special handling.

### Chunk 8 (Multi-voice)
- `<backup>` and `<forward>` manipulate the time cursor.
- Voice and staff numbers are crucial for polyphony.

### Chunk 9 (Lyrics)
- Syllabic handling (single, begin, middle, end) is important.
- Multiple verse support via lyric numbers.

### Chunk 10 (Advanced)
- Brief coverage is fine — these are deferred.
- Note which elements exist and their purpose, but detailed mapping is optional.

---

## Begin

Start with Chunk 1. Produce `chunk-01-core-note.md`, then proceed through each subsequent chunk. Take your time with each one — quality over speed.

If you encounter something genuinely ambiguous, document it in Open Questions rather than guessing.
