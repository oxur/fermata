# Claude Code Prompt: MusicXML Triage

> **Instructions:** Copy this entire prompt and give it to Claude Code along with the MusicXML schema files.

---

## Context

You are helping design **Fermata**, a Lisp-like DSL that compiles to MusicXML. Before we can define the S-expression syntax, we need to understand the full scope of MusicXML and prioritize which elements to support.

Your task is to analyze the MusicXML 4.0 XSD schema and produce a **triage document** that categorizes all elements by implementation priority.

## Input Files

You should have access to:
- `musicxml.xsd` — The main schema file (~12,000 lines)
- Optionally: `container.xsd`, `opus.xsd`, `sounds.xsd`

Focus primarily on `musicxml.xsd` as it contains all the musical content definitions.

## Your Task

Analyze the schema and produce a markdown file called `musicxml-triage.md` with the following structure:

### 1. Executive Summary

A brief overview of MusicXML's scope:
- Approximate number of element types
- Approximate number of complex types
- Major conceptual areas (e.g., notes, measures, directions, etc.)

### 2. Priority Categories

Categorize elements into four tiers:

#### Tier 1: Core (MVP)
Elements required to represent basic musical notation:
- Notes, rests, pitches, durations
- Measures, time signatures, key signatures, clefs
- Parts and basic score structure

**Criteria:** Cannot render even a simple melody without these.

#### Tier 2: Important
Elements needed for real-world scores:
- Dynamics, articulations, slurs, ties
- Tempo markings, rehearsal marks
- Beaming, stems
- Basic ornaments (trills, mordents)

**Criteria:** Most published sheet music uses these regularly.

#### Tier 3: Secondary
Elements for advanced notation:
- Lyrics
- Chord symbols
- Figured bass
- Advanced ornaments
- Percussion notation
- Multi-voice/multi-staff complexity

**Criteria:** Needed for specific genres or complex scores.

#### Tier 4: Deferred
Specialized or rare elements:
- Harp pedals, accordion registrations
- Scordatura
- Baroque ornamentation details
- Layout/formatting attributes
- MIDI-specific elements

**Criteria:** Can be added later without affecting core architecture.

### 3. Element Inventory

For each tier, list elements in this format:

```markdown
#### Tier N: [Name]

| Element | Parent Context | Description | Notes |
|---------|---------------|-------------|-------|
| `<note>` | `<measure>` | A single note or rest | Core building block |
| `<pitch>` | `<note>` | Pitch specification | Contains step, alter, octave |
```

### 4. Complex Types Summary

List the major complex types and what they represent:
- Which types are widely reused?
- Which types have many optional children (extension points)?

### 5. Attribute Groups

Identify recurring attribute patterns:
- Position attributes (default-x, default-y, etc.)
- Print attributes (print-object, print-style, etc.)
- Font attributes

Note: We may want to handle these uniformly in our IR.

### 6. Observations & Warnings

Note any:
- Elements that seem redundant or overlapping
- Elements whose purpose is unclear from the schema alone
- Elements that will be tricky to map to S-expressions
- Elements that have complex interdependencies

## Output Format

Produce a single file: `musicxml-triage.md`

The file should be:
- Well-organized with clear headers
- Comprehensive but not exhaustive in prose (use tables)
- Focused on information useful for designing an S-expression mapping

## Guidelines

1. **Read the `<xs:documentation>` elements** — They contain valuable descriptions
2. **Follow type references** — When you see `type="note-type"`, look up that type
3. **Note cardinality** — Is an element required, optional, or repeatable?
4. **Identify patterns** — Many elements follow similar patterns
5. **Be pragmatic** — Some elements exist for historical compatibility; note these

## Example Output Snippet

```markdown
## 2. Priority Categories

### Tier 1: Core (MVP)

These 23 elements form the minimum viable representation:

| Element | Parent | Description | Notes |
|---------|--------|-------------|-------|
| `<score-partwise>` | root | Score organized by parts | Primary document structure |
| `<part>` | `<score-partwise>` | A single instrument/voice part | Contains measures |
| `<measure>` | `<part>` | A single measure | Contains notes and directions |
| `<note>` | `<measure>` | A note, rest, or chord member | Most complex element |
| `<pitch>` | `<note>` | Note pitch | step + alter + octave |
| `<duration>` | `<note>` | Sounding duration | In divisions |
| `<type>` | `<note>` | Notated duration | quarter, eighth, etc. |
| ... | ... | ... | ... |

Total: ~23 elements, ~8 complex types
```

---

**Begin your analysis now. Take your time and be thorough.**
