# Phase 3 — Overview

> **For:** Claude Code (Opus with Rust-SKILL.md agents)
> **Created:** 2026-01-31
> **Status:** Ready for execution (after Phase 2 completion)
> **Document Series:** 1 of 7

---

## Executive Summary

**Objective:** Implement `src/musicxml/parse.rs` that transforms MusicXML 4.0 documents into the Fermata IR types.

**This is the inverse of Phase 2.** Where Phase 2 walks the IR tree and emits XML elements, Phase 3 consumes XML events and constructs the IR tree.

**Success Criteria:**

1. Parse valid MusicXML 4.0 partwise documents into IR
2. Round-trip fidelity: IR → XML → IR produces identical IR
3. Graceful error handling with informative messages
4. Clean, idiomatic Rust with comprehensive tests

**Estimated Scope:** ~18 implementation tasks across 5 milestones

---

## Why This Matters

With both emitter (Phase 2) and parser (Phase 3), Fermata achieves **round-trip capability**:

```
MusicXML file → parse() → IR → emit() → MusicXML file
                              ↓
                    Manipulate in Rust
                              ↓
                         emit() → MusicXML
```

This enables:

- Import existing scores from notation software
- Programmatic score manipulation
- Validation of the IR design (if round-trip fails, something's wrong)
- Foundation for the S-expr syntax (Phase 4+)

---

## Architecture Decision: XML Parser Strategy

### Recommended: `quick-xml` with Reader API

```toml
# Already in Cargo.toml from Phase 2
[dependencies]
quick-xml = "0.37"
```

**Rationale:**

- Already a dependency from Phase 2 (consistency, no new deps)
- `Reader` API provides streaming input (memory-efficient for large scores)
- Event-based parsing matches MusicXML's sequential structure
- Good error types with position information

**Alternative considered:**

- `roxmltree` (DOM-based) — simpler API but higher memory usage, loads entire document
- `serde` with `quick-xml` — too rigid for MusicXML's complex content models

### Event-Based Parsing Pattern

MusicXML is deeply nested with many optional elements. We use a **state-machine approach**:

```rust
// Each complex element gets a dedicated parsing function
fn parse_note(reader: &mut XmlReader, start: &BytesStart) -> Result<Note, ParseError> {
    let mut pitch = None;
    let mut duration = None;
    let mut voice = None;
    // ... collect children

    loop {
        match reader.next_event()? {
            Event::Start(e) => match e.name().as_ref() {
                b"pitch" => pitch = Some(parse_pitch(reader)?),
                b"duration" => duration = Some(reader.read_text_as::<u32>()?),
                b"voice" => voice = Some(reader.read_text()?),
                // ...
                _ => reader.skip_element()?,
            },
            Event::End(e) if e.name().as_ref() == b"note" => break,
            _ => {}
        }
    }

    // Construct IR type from collected values
    Ok(Note { /* ... */ })
}
```

---

## Module Structure

```
src/
├── ir/                    # ✅ Complete (Phase 1)
│   └── (all IR types)
├── musicxml/
│   ├── mod.rs             # Public API: emit() + parse()
│   ├── emit.rs            # ✅ Complete (Phase 2)
│   ├── writer.rs          # ✅ Complete (Phase 2)
│   ├── divisions.rs       # ✅ Complete (Phase 2)
│   ├── parse.rs           # NEW: Main parsing logic
│   ├── reader.rs          # NEW: XmlReader helper wrapper
│   └── values.rs          # NEW: String → enum parsers
└── lib.rs                 # Re-export musicxml module
```

---

## Public API

After Phase 3, the `musicxml` module exposes:

```rust
// src/musicxml/mod.rs

/// Emit a MusicXML document from IR
pub fn emit(score: &ScorePartwise) -> Result<String, EmitError>;

/// Parse a MusicXML document into IR
pub fn parse(xml: &str) -> Result<ScorePartwise, ParseError>;

/// Parse with lenient error handling (collects warnings)
pub fn parse_lenient(xml: &str) -> Result<ParseResult, ParseError>;
```

---

## Error Handling Strategy

### ParseError Type

```rust
#[derive(Debug, Clone)]
pub enum ParseError {
    /// XML syntax error from quick-xml
    Xml(String),

    /// Missing required element
    MissingElement {
        parent: String,
        element: String,
        position: usize
    },

    /// Missing required attribute
    MissingAttribute {
        element: String,
        attribute: String,
        position: usize
    },

    /// Invalid value for element or attribute
    InvalidValue {
        context: String,
        value: String,
        expected: String,
        position: usize
    },

    /// Unexpected element in context
    UnexpectedElement {
        context: String,
        element: String,
        position: usize
    },

    /// Reference to undefined ID (e.g., part references score-part)
    UndefinedReference {
        ref_type: String,
        id: String,
        position: usize
    },

    /// Generic parse error
    Other(String),
}
```

### Design Principles

1. **Always include position** — byte offset for user debugging
2. **Context matters** — "missing `<step>` in `<pitch>`" not just "missing element"
3. **Expected vs actual** — show what was expected and what was found
4. **Fail fast for required elements** — don't silently produce invalid IR
5. **Skip unknown elements gracefully** — forward compatibility with MusicXML extensions

---

## Milestone Overview

| Milestone | Focus | Tasks | Key Deliverable |
|-----------|-------|-------|-----------------|
| **1: Foundation** | Module structure, helpers | 1.1–1.4 | XmlReader, value parsers, skeleton |
| **2: Core** | Notes and attributes | 2.1–2.5 | Parse basic scores, round-trip test |
| **3: Voice/Nav** | Multi-voice, barlines | 3.1–3.4 | Two-voice scores, repeats |
| **4: Expression** | Directions, notations | 4.1–4.5 | Dynamics, articulations, slurs |
| **5: Extended** | Lyrics, ornaments, header | 5.1–5.5 | Full feature coverage |

---

## Key Differences from Phase 2

| Aspect | Phase 2 (Emitter) | Phase 3 (Parser) |
|--------|-------------------|------------------|
| **Direction** | IR → XML | XML → IR |
| **Data flow** | Walk tree, emit events | Consume events, build tree |
| **Element order** | Must emit in XSD order | Must accept in XSD order |
| **Optional fields** | Skip if `None` | Default to `None`/empty |
| **Validation** | IR assumed valid | Must validate input |
| **Error frequency** | Rare (well-formed IR) | Common (malformed XML) |
| **State** | Stateless tree walk | Stateful event processing |
| **Complexity** | Straightforward | Trickier (building vs walking) |

---

## Relationship to Other Phases

```
Phase 1: IR Types ──────────────────────────────────────┐
         (foundation for everything)                    │
                                                        ▼
Phase 2: Emitter ◄─────── Round-trip test ───────► Phase 3: Parser
         (IR → XML)                                    (XML → IR)
                                                        │
                                                        ▼
Phase 4: S-expr Read/Print ◄─────────────────────────────┘
         (S-expr ↔ IR, uses parser patterns)
                    │
                    ▼
Phase 5: Fermata Syntax
         (ergonomic DSL → IR)
```

---

## Document Series

1. **Overview** (this document)
2. **Milestone 1: Foundation** — XmlReader, value parsers, skeleton
3. **Milestone 2: Core Note & Attributes** — The heart of parsing
4. **Milestone 3: Voice, Barlines, Navigation** — Multi-voice support
5. **Milestone 4: Directions & Notations** — Expression markings
6. **Milestone 5: Extended Features** — Lyrics, ornaments, header
7. **Checklist & Success Criteria** — Final validation

---

## Getting Started

Before starting Phase 3:

1. **Ensure Phase 2 is complete** — all emitter tests pass
2. **Review Phase 2 code** — understand the IR structure and XML patterns
3. **Study `quick-xml` Reader API** — <https://docs.rs/quick-xml/latest/quick_xml/reader/>
4. **Have test fixtures ready** — MusicXML files to parse

The parser is the mirror image of the emitter. Every `emit_*` function has a corresponding `parse_*` function.

---

*Next document: Milestone 1 — Foundation*
