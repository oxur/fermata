---
number: 1
title: "Fermata Lisp — Project Inception"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-01-30
updated: 2026-02-02
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Fermata Lisp — Project Inception

> **Purpose:** This document captures the vision, design decisions, and early thinking around the Fermata project. It's intended to bootstrap another Claude instance (or human collaborator) into the conversation so far.

---

## What Is Fermata?

**Fermata Lisp** is an S-expression-based domain-specific language (DSL) for working with music notation. It provides a concise, human-readable syntax that "compiles" to MusicXML — the standard interchange format for music notation software.

**Full name:** Fermata Lisp
**Tagline:** "An S-expression DSL for working with MusicXML"
**Repository:** `oxur/fermata` (GitHub and Codeberg)
**License:** MIT OR Apache-2.0 (dual-licensed)

---

## The Problem We're Solving

### MusicXML Is Verbose

MusicXML is excellent for interchange but painful to write by hand:

```xml
<note>
  <pitch>
    <step>C</step>
    <octave>4</octave>
  </pitch>
  <duration>1</duration>
  <type>quarter</type>
</note>
```

### We Want to Communicate Musically

The original motivation: enabling **human-AI conversations about music theory** with precision that natural language can't provide. When discussing voice leading, counterpoint, or harmonic analysis, we need to express exact musical ideas — not vague descriptions.

Instead of:
> "Play a C major chord in root position, quarter note duration"

We want:

```lisp
(chord :q (c4 e4 g4))
```

---

## Why Lisp / S-Expressions?

1. **Isomorphic to XML's tree structure** — S-expressions and XML both represent trees; the mapping is natural
2. **Homoiconicity** — The notation *is* the data structure; we can manipulate it programmatically
3. **Macros** — Enable powerful abstractions like `(cadence :authentic :key c-major)` expanding to actual notes with proper voice leading
4. **Terse but unambiguous** — Unlike ABC notation which has weird edge cases
5. **Composable** — `(transpose +2 (motif fate))`, `(invert ...)`, `(retrograde ...)`
6. **Lisp programmers exist** — The user is a Lisp programmer who wants to work in a familiar paradigm

---

## Design Goals

### 1. Multiple Execution Modes

| Mode | Description |
|------|-------------|
| **REPL** | Interactive `fermata>` prompt for exploration and live rendering |
| **Interpreter** | Evaluate `.ferm` files, output MusicXML |
| **Compiler** | Generate Rust code that produces MusicXML (for embedding) |

### 2. Bidirectional Transformations

All representations should be interconvertible:

```
MusicXML  ←→  Fermata Lisp  ←→  Rust Code
    ↑              ↑               ↑
    └──────────────┴───────────────┘
         (round-trip capable)
```

- **MusicXML → Fermata:** Import existing scores for manipulation
- **Fermata → MusicXML:** Primary output for rendering (via Verovio)
- **Fermata → Rust:** Compile to embeddable Rust code
- **Rust → Fermata:** Programmatic generation from Rust applications

### 3. Integration with Verovio

Fermata is designed to work with **verovioxide** (Rust bindings to Verovio):

```
Fermata source → MusicXML → Verovio → SVG
```

This enables immediate visual feedback in the REPL.

---

## Syntax Design (Draft)

### Primitives

#### Notes

```lisp
(note <pitch> <duration> [modifiers...])

;; Examples
(note c4 :q)              ; C4 quarter note
(note d#5 :h :dot)        ; D#5 dotted half note
(note bb3 :8 :staccato)   ; Bb3 eighth note, staccato
(note f4 :w :fermata)     ; F4 whole note with fermata
```

**Pitch notation:** Scientific pitch notation (c4 = middle C)

- Sharps: `c#4`, `f#5`
- Flats: `bb3`, `eb4`
- Natural (explicit): `cn4`

**Duration keywords:**

| Keyword | Duration |
|---------|----------|
| `:w` | whole |
| `:h` | half |
| `:q` | quarter |
| `:8` | eighth |
| `:16` | sixteenth |
| `:32` | thirty-second |

**Modifiers:** `:dot`, `:double-dot`, `:fermata`, `:staccato`, `:accent`, `:tenuto`, `:tie`, `:slur-start`, `:slur-end`

#### Rests

```lisp
(rest <duration>)

;; Examples
(rest :q)     ; quarter rest
(rest :h)     ; half rest
```

#### Chords

```lisp
(chord <duration> (<pitches...>))

;; Examples
(chord :q (c4 e4 g4))           ; C major triad, quarter note
(chord :h (d4 f#4 a4) :accent)  ; D major triad, half note, accented
```

### Containers

#### Measures

```lisp
(measure [attributes...] <elements...>)

;; Examples
(measure
  (note c4 :q)
  (note d4 :q)
  (note e4 :q)
  (note f4 :q))

(measure :time (4 4) :key c-major
  (note c4 :w))
```

#### Staves

```lisp
(staff <clef> <measures...>)

;; Clefs: :treble, :bass, :alto, :tenor, :percussion

(staff :treble
  (measure (note c5 :q) (note d5 :q) (note e5 :h))
  (measure (note f5 :w)))
```

#### Parts

```lisp
(part <instrument> <staves...>)

;; Single staff instrument
(part :violin
  (staff :treble
    (measure ...)))

;; Grand staff instrument (piano)
(part :piano
  (staff :treble
    (measure ...))
  (staff :bass
    (measure ...)))
```

#### Scores

```lisp
(score [metadata...] <parts...>)

(score
  :title "Minuet in G"
  :composer "J.S. Bach"
  :tempo 120

  (part :piano
    (staff :treble ...)
    (staff :bass ...)))
```

### Transformations (Macros)

These expand into concrete note sequences:

```lisp
;; Transposition
(transpose +2 (note c4 :q))        ; → (note d4 :q)
(transpose -12 (chord :q (c4 e4 g4)))  ; down an octave

;; Inversion
(invert c4 (note e4 :q))           ; invert around C4

;; Retrograde
(retrograde
  (note c4 :q) (note d4 :q) (note e4 :q))
; → (note e4 :q) (note d4 :q) (note c4 :q)

;; Augmentation / Diminution
(augment 2 (note c4 :q))           ; → (note c4 :h)
(diminish 2 (note c4 :h))          ; → (note c4 :q)
```

### Music Theory Macros

These encode music theory knowledge:

```lisp
;; Scales (return pitch lists)
(scale c4 :major)      ; → (c4 d4 e4 f4 g4 a4 b4 c5)
(scale a4 :minor)      ; → (a4 b4 c5 d5 e5 f5 g5 a5)
(scale d4 :dorian)     ; → (d4 e4 f4 g4 a4 b4 c5 d5)

;; Chord voicings
(chord-voicing c4 :major)     ; → (c4 e4 g4)
(chord-voicing c4 :major7)    ; → (c4 e4 g4 b4)
(chord-voicing c4 :dom7)      ; → (c4 e4 g4 bb4)

;; Cadences (with voice leading)
(cadence :authentic :key c-major)
; → properly voiced V-I progression

(cadence :plagal :key g-major)
; → properly voiced IV-I progression

;; Chord progressions
(progression c-major I IV V I)
; → sequence of chords with idiomatic voice leading
```

### Naming and References

```lisp
;; Define a named motif
(def fate-motif
  (note g4 :8)
  (note g4 :8)
  (note g4 :8)
  (note eb4 :h :fermata))

;; Use it
(measure fate-motif)
(measure (transpose +2 fate-motif))
(measure (retrograde fate-motif))
```

---

## Implementation Strategy

### Language: Rust

The interpreter and compiler will be written in Rust for:

- Integration with verovioxide (our Verovio bindings)
- Performance
- Single-binary distribution
- Memory safety

### Parser

Use `nom` or `pest` for parsing S-expressions into an AST.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Fermata Crate                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────┐    ┌─────────┐    ┌──────────────────────┐     │
│  │ Parser  │───▶│   AST   │───▶│  Evaluator / Macro   │     │
│  │  (nom)  │    │         │    │     Expander         │     │
│  └─────────┘    └─────────┘    └──────────────────────┘     │
│                                          │                  │
│                      ┌───────────────────┼────────────┐     │
│                      ▼                   ▼            ▼     │
│               ┌───────────┐      ┌───────────┐   ┌───────┐  │
│               │ MusicXML  │      │   Rust    │   │ Ferm  │  │
│               │  Emitter  │      │  Codegen  │   │ Pretty│  │
│               └───────────┘      └───────────┘   │ Print │  │
│                      │                  │        └───────┘  │
│                      ▼                  ▼                   │
│               .musicxml file        .rs file                │
│                      │                                      │
│                      ▼                                      │
│               ┌─────────────┐                               │
│               │ verovioxide │ (optional, for REPL preview)  │
│               └─────────────┘                               │
│                      │                                      │
│                      ▼                                      │
│                     SVG                                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Crate Structure

```
fermata/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Public API
│   ├── ast.rs           # AST types
│   ├── parser.rs        # S-expression parser
│   ├── eval.rs          # Interpreter / macro expander
│   ├── musicxml.rs      # MusicXML code generation
│   ├── rust_codegen.rs  # Rust code generation
│   ├── pretty.rs        # Pretty-printer (AST → Fermata source)
│   └── repl.rs          # Interactive REPL
├── src/bin/
│   └── fermata.rs       # CLI binary
├── examples/
│   ├── simple_score.ferm
│   ├── bach_minuet.ferm
│   └── theory_demo.ferm
└── tests/
    ├── parser_tests.rs
    ├── eval_tests.rs
    └── roundtrip_tests.rs
```

### CLI Interface

```bash
# Start REPL
$ fermata
fermata> (note c4 :q)
; => MusicXML output or rendered preview

# Compile to MusicXML
$ fermata compile score.ferm -o score.musicxml

# Compile to Rust
$ fermata compile score.ferm --target rust -o score.rs

# Convert MusicXML to Fermata
$ fermata import score.musicxml -o score.ferm

# Render directly to SVG (requires verovioxide)
$ fermata render score.ferm -o score.svg
```

---

## Relationship to Other Projects

### verovioxide (oxur/verovioxide)

Rust bindings to Verovio. Fermata uses verovioxide for:

- REPL preview rendering
- Direct SVG output

### The oxur Ecosystem

```
oxur/
├── fermata/        # This project — Lisp DSL for music notation
├── verovioxide/    # Rust bindings to Verovio
└── (future)        # MCP server for music theory, etc.
```

---

## Prior Art Considered

We researched existing solutions:

| Project | Language | Notes |
|---------|----------|-------|
| FOMUS | Common Lisp | Closest prior art; outputs MusicXML/LilyPond; CL-only |
| Opusmodus | Common Lisp | Commercial, powerful, proprietary |
| Common Music | CL/Scheme | Ancient (2007), abandoned |
| Slippery Chicken | Common Lisp | Algorithmic composition focus |
| MusicDsl | Java/Xtext | Academic thesis, Eclipse-based |
| Hum | Rust | Audio output only, not notation |

**What makes Fermata unique:**

1. S-expression syntax (Lisp-like)
2. MusicXML as primary target
3. Written in Rust (embeddable, modern tooling)
4. REPL + compiler + bidirectional transforms
5. Designed for human-AI musical communication

---

## Open Questions

### Syntax Decisions Still Being Explored

1. **Pitch representation:** Is `c4` clear enough, or should we support `C4`, `c-4`, etc.?

2. **Duration syntax:** `:q` for quarter is terse; should we also support `:quarter`?

3. **Attribute syntax:** Keywords (`:staccato`) vs. wrapper forms `(staccato (note c4 :q))`?

4. **Measure boundaries:** Explicit `(measure ...)` vs. implicit from time signature?

5. **Tuplets:** How to represent triplets, quintuplets, etc.?

   ```lisp
   ; Option A: wrapper
   (tuplet 3 2 (note c4 :8) (note d4 :8) (note e4 :8))

   ; Option B: modifier
   (note c4 :8 :tuplet-3)  ; unclear
   ```

6. **Dynamics:** As modifiers or separate elements?

   ```lisp
   ; Option A: modifier
   (note c4 :q :mf)

   ; Option B: separate
   (dynamic :mf)
   (note c4 :q)
   ```

### Implementation Priorities

1. **Phase 1:** Parser + AST + MusicXML emitter (core functionality)
2. **Phase 2:** REPL with verovioxide integration
3. **Phase 3:** Macro system and music theory macros
4. **Phase 4:** Rust codegen
5. **Phase 5:** MusicXML → Fermata importer

---

## Example: Complete Score

```lisp
;; Twinkle Twinkle Little Star (first phrase)

(score
  :title "Twinkle Twinkle Little Star"
  :composer "Traditional"
  :tempo 100
  :time (4 4)
  :key c-major

  (part :piano
    (staff :treble
      (measure
        (note c4 :q) (note c4 :q) (note g4 :q) (note g4 :q))
      (measure
        (note a4 :q) (note a4 :q) (note g4 :h))
      (measure
        (note f4 :q) (note f4 :q) (note e4 :q) (note e4 :q))
      (measure
        (note d4 :q) (note d4 :q) (note c4 :h)))))
```

This would compile to MusicXML and render to:

```
𝄞 4/4  C  C  G  G | A  A  G     | F  F  E  E | D  D  C     ‖
```

---

## Summary

**Fermata Lisp** is a new S-expression DSL for music notation that:

- Provides a **terse, expressive syntax** for writing music
- **Compiles to MusicXML** for interoperability with all major notation software
- Is **written in Rust** for performance and embeddability
- Supports **interactive exploration** via REPL with visual feedback
- Enables **precise human-AI communication** about musical concepts
- Is **bidirectional** — can import, export, and round-trip between formats

It fills a gap: nothing else combines Lisp syntax + MusicXML output + Rust implementation + interactive REPL + designed for human-AI communication.

---

*Document version: 2025-01-30*
*Project status: Design phase*
