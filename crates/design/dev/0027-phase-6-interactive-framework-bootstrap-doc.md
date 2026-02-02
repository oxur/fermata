# Fermata Interactive Framework — Bootstrap Document

> **Purpose:** Bootstrap a new Claude session (or human collaborator) into the Fermata project for Phase 6+ development. Covers the current state, the interactive framework vision, and all architectural decisions made to date.
>
> **Supersedes:** `0006-fermata-lisp-project-bootstrap-document.md` (which covered Phases 1–5)
>
> **Date:** 2026-02-01

---

## 1. What Is Fermata?

Fermata is an S-expression DSL for music notation that compiles to MusicXML, implemented in Rust. It is the notation layer of a larger **interactive music collaboration framework** where a human musician/theorist and an AI assistant (Claude) work together in real time, passing musical ideas back and forth with full notational precision, shared analytical context, and audible feedback.

### The Stack

```
┌─────────────────────────────────────────────────────────────────┐
│  Claude (AI Assistant)                                          │
│  Connected via MCP server — observes, queries, interacts        │
└──────────────────────────────┬──────────────────────────────────┘
                               │ MCP tools (JSON over Unix socket)
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│  MCP Server (Rust)                                              │
│  Bridge between Claude and the REPL process                     │
│  Also hosts: music theory knowledge base (2691 docs, 2340       │
│  concepts, 3212 graph edges)                                    │
└──────────────────────────────┬──────────────────────────────────┘
                               │ IPC (line-delimited JSON)
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│  Fermata REPL (Rust)                                            │
│  Interactive runtime with sessions, history, notebooks          │
│  Evaluates Fermata syntax → typed IR → MusicXML                 │
│  MIDI subsystem for playback and recording (Phases 7–8)         │
└──────────────────────────────┬──────────────────────────────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
         MusicXML         MIDI Devices      Verovio
         (files)          (I/O)             (rendering,
                                             future)
```

---

## 2. What Exists (Phases 1–5)

All implemented in `crates/fermata/`:

| Phase | Name | Status | What It Built |
|-------|------|--------|---------------|
| 1 | IR Types | ✅ Complete | Typed Rust structs mirroring MusicXML 4.0 (`src/ir/`) |
| 2 | MusicXML Emitter | ✅ Complete | IR → MusicXML string (`src/musicxml/emitter/`) |
| 3 | MusicXML Parser | ✅ Complete | MusicXML string → IR (`src/musicxml/parser/`) |
| 4 | S-expr Read/Print | ✅ Complete | IR ↔ S-expression text (`src/sexpr/`) |
| 5 | Fermata Syntax | ✅ Complete | User-facing syntax → IR compiler (`src/fermata/`) |

### Repository Structure (post-Phase 5)

```
oxur/fermata/
└── crates/fermata/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── main.rs              # CLI binary (compile/check commands)
        ├── ir/                  # Phase 1: typed IR
        │   ├── mod.rs
        │   ├── common.rs        # YesNo, StartStop, Position, Font, etc.
        │   ├── pitch.rs         # Pitch, Step, Octave, Semitones
        │   ├── duration.rs      # NoteType, NoteTypeValue, Dot, TimeModification
        │   ├── note.rs          # Note, NoteContent, FullNote, Rest, Grace, Accidental
        │   ├── beam.rs          # Beam, Stem, Notehead
        │   ├── attributes.rs    # Attributes, Key, Time, Clef, Barline
        │   ├── direction.rs     # Direction, Dynamics, Wedge, Metronome, Words
        │   ├── notation.rs      # Notations, Articulations, Ornaments, Slur, Tied, Tuplet
        │   ├── voice.rs         # Backup, Forward
        │   ├── lyric.rs         # Lyric, Syllabic, TextElementData
        │   ├── measure.rs       # Measure, MusicDataElement
        │   ├── part.rs          # Part, PartList, ScorePart
        │   └── score.rs         # ScorePartwise, Work, Identification, Defaults
        ├── musicxml/            # Phases 2–3: emit/parse
        │   ├── mod.rs
        │   ├── divisions.rs
        │   ├── emitter.rs       # IR → MusicXML
        │   ├── emitter/         # Sub-modules for score, note, attributes, etc.
        │   ├── parser/          # MusicXML → IR
        │   ├── reader.rs
        │   ├── writer.rs
        │   └── values.rs
        ├── sexpr/               # Phase 4: IR ↔ S-expr
        │   ├── mod.rs
        │   ├── ast.rs           # Generic S-expression AST
        │   ├── parser.rs        # Text → S-expr (nom)
        │   ├── printer.rs       # S-expr → text (pretty-print)
        │   ├── convert/         # S-expr ↔ IR conversion (per-module)
        │   ├── error.rs
        │   └── traits.rs
        └── lang/                # Phase 5: user-facing syntax
            ├── mod.rs
            ├── ast.rs           # Fermata-specific AST types
            ├── compiler.rs      # Fermata AST → IR
            ├── error.rs         # CompileError with SourceSpan
            ├── pitch.rs         # c4, f#5, bb3 parsing
            ├── duration.rs      # :q, :quarter, :crotchet parsing
            ├── note.rs          # Note compilation
            ├── chord.rs         # Chord expansion
            ├── tuplet.rs        # Tuplet compilation
            ├── direction.rs     # Dynamics, tempo
            ├── attributes.rs    # Key, time, clef
            ├── measure.rs       # Measure assembly
            ├── part.rs          # Part compilation
            ├── score.rs         # Score compilation
            └── defaults.rs      # Default value inference
```

### Key Architectural Decisions (Locked In)

- **Typed Music IR as the hub** — not a generic S-expression AST
- **Two-phase parsing:** `Fermata source → S-expr (untyped) → Music IR (typed)`
- **Parser:** `nom` (not `pest`) — simple grammar, good error recovery, streaming
- **Pitch notation:** lowercase scientific (`c4`, `f#5`, `bb3`, `cn4`)
- **Duration keywords:** short (`:q`), long (`:quarter`), and British (`:crotchet`)
- **Dynamics:** separate positioned elements (matches MusicXML `<direction>` model)
- **Tuplets:** wrapper form with explicit note durations
- **Chords:** `:chord t` flag on subsequent notes (matches MusicXML)
- **Internal duration:** rational-based (`TimeModification`) for dots and tuplets
- **Voice type:** `String` (not integer) — allows custom identifiers like `"1a"`

### Data Flow

```
Fermata syntax    →  Fermata AST  →  Music IR  →  MusicXML
   (note c4 :q)      (typed)        (typed)       <note>...</note>

MusicXML  →  Music IR  →  S-expr text
                          (score-partwise :version "4.0" ...)
```

---

## 3. What We're Building Now: Phase 6

Phase 6 transforms Fermata from a compiler into an **interactive collaborative environment**. The REPL is not just a read-eval-print loop — it is a stateful, session-based, multi-channel workspace where the user and Claude co-create and analyze music.

### 3.1 Core Concepts

**Session** — The fundamental unit of work. Contains a score, an environment, a history, a notebook, and a message queue. Sessions are named, identified by stable IDs, and can be created, listed, switched, saved, and restored.

**Score** — The session's musical artifact: a typed `FermataScore` representing accumulated notation. Always valid (modifications go through the evaluator). Can be emitted as MusicXML at any time.

**Environment** — The evaluation context: symbol bindings, macro definitions, implicit state (current default part, voice, key, time, clef, tempo). Analogous to ENV in other Lisps.

**History** — A unified, ordered timeline of *everything that happened in the REPL*. Not just commands — every eval, every REPL command, every message from Claude, every reply from the user, every system event. Each entry gets a monotonic index (à la Erlang shell, Mathematica). The history records events in the exact order the user experienced them.

**Notebook** — A structured analytical narrative stored as a typed tree (not raw text). Contains sections with prose, score references, inline musical examples, history references, annotations, and chat summaries. Writable from both REPL and MCP. Rendered to Markdown only on export.

**Message Queue** — Incoming messages from Claude, buffered and displayed at the next REPL prompt. Never interrupts the user mid-input.

### 3.2 Session Data Model

```rust
struct Session {
    id: SessionId,
    metadata: SessionMetadata,
    score: FermataScore,
    env: Environment,
    history: Vec<HistoryEntry>,
    next_index: u64,
    notebook: Notebook,
    message_queue: VecDeque<IncomingMessage>,
    midi_state: Option<MidiSessionState>,   // Phase 7+
}
```

### 3.3 Unified History

Every REPL event gets a `HistoryEntry` with a monotonic index:

```rust
struct HistoryEntry {
    index: u64,
    timestamp: DateTime<Utc>,
    kind: EntryKind,
}

enum EntryKind {
    Eval { input, parsed, result, error, score_delta },
    Command { raw, output },
    AiMessage { content, source },
    UserMessage { content },
    System { event },
}
```

Ordering guarantee: the REPL's main loop is the single-threaded serialization point. Messages from Claude are drained at the top of each loop iteration, then the user's input is processed. History reflects the user's experienced order.

### 3.4 Notebook

Stored as a typed tree, not text:

```rust
struct Notebook {
    metadata: NotebookMetadata,
    root: Vec<Section>,
}

struct Section {
    id: SectionId,       // stable, survives reordering
    title: String,
    content: Vec<Block>,
    children: Vec<Section>,
}

enum Block {
    Prose(String),
    ScoreRef { session_id, part, measures, snapshot },
    InlineMusic { source, compiled },
    HistoryRef { range, caption },
    Annotation { kind, text, anchor },
    ChatSummary { source, timestamp, content },
}
```

Display numbering (1.1, 2.4, etc.) is computed from tree position, not stored. SectionId is stable across reordering.

### 3.5 REPL Input Modes

Three input types, distinguished by prefix:

| Prefix | Mode | Example |
|--------|------|---------|
| (none) | Fermata expression | `(note c4 :q)` |
| `:` | REPL command | `:session list` |
| `//` | Chat message to Claude | `// what key is this in?` |

### 3.6 REPL Loop

```
loop {
    1. Drain message queue → display, assign indices, commit to history
    2. Display prompt (session name + next index)
    3. Read user input
    4. Classify (expression / command / chat)
    5. Process (eval / execute / publish)
    6. Display result
    7. Commit to history
}
```

### 3.7 IPC Architecture

The REPL opens a Unix domain socket at `~/.fermata/repl.sock`. The MCP server connects as a client. Protocol: line-delimited JSON, request–response with `id` correlation.

```
MCP Server ──── Unix socket ──── REPL Process
  (client)     (JSON lines)       (server)
```

### 3.8 MCP Tool Surface

| Tool | Sub-phase | Purpose |
|------|-----------|---------|
| `repl_list_sessions` | 6c | List sessions with metadata |
| `repl_get_session` | 6c | Session detail (metadata, score summary, history length) |
| `repl_get_active_session` | 6c | Currently active session |
| `repl_get_score` | 6c | Score as Fermata/MusicXML/IR summary |
| `repl_get_measures` | 6c | Specific measure range |
| `repl_get_history` | 6c | History with index/time/type filters |
| `repl_send_message` | 6c | Enqueue message for REPL display |
| `repl_eval` | 6c | Evaluate Fermata expression in session |
| `repl_notebook_outline` | 6d | Notebook section tree |
| `repl_notebook_get_section` | 6d | Section contents |
| `repl_notebook_append` | 6d | Append block to section |
| `repl_notebook_create_section` | 6d | Create new section |
| `repl_notebook_search` | 6d | Search notebook content |

### 3.9 Session Persistence

```
~/.fermata/sessions/<id>/
├── meta.json        # name, timestamps, tags
├── score.ferm       # Fermata syntax (human-readable, round-trippable)
├── env.json         # Bindings and implicit state
├── history.jsonl    # One JSON object per line (append-friendly)
└── notebook.json    # Full notebook tree
```

---

## 4. What Comes After: Phases 7–8

### Phase 7: MIDI Foundation

MIDI device enumeration, connection management, score-to-MIDI conversion, playback commands and MCP tools. Uses the `midir` crate for cross-platform MIDI.

### Phase 8: Bidirectional MIDI

MIDI input, quantization, MIDI-to-IR conversion, recording workflow, and the complete bidirectional loop: user plays → REPL captures → Claude analyzes → Claude responds.

---

## 5. Phase 6 Sub-Phases

| Sub-phase | Focus | Key Deliverables |
|-----------|-------|------------------|
| **6a** | Core REPL | REPL loop, Fermata eval, session data model, session manager, numbered history (Eval + Command only), basic REPL commands |
| **6b** | Communication | Chat input mode (// prefix), message queue, UserMessage + AiMessage entry types, unified timeline |
| **6c** | IPC & MCP Bridge | Unix socket server, JSON protocol, MCP client connection, session/score/history/message MCP tools |
| **6d** | Notebook | Notebook data model, REPL commands, MCP tools, capture command |
| **6e** | Persistence & Polish | Save/load, autosave, session recovery, history references ($In[n]/$Out[n]), ANSI coloring, line editing |

---

## 6. Proposed Crate Structure (Phase 6+)

```
oxur/fermata/
└── crates/
    ├── fermata/             # Existing: IR, MusicXML, S-expr, Fermata syntax
    ├── fermata-repl/        # NEW: REPL binary, session manager, history, notebook
    ├── fermata-ipc/         # NEW: IPC protocol types, socket server/client
    └── fermata-midi/        # NEW (Phase 7): MIDI device management, conversion
```

The MCP bridge lives in the existing MCP server crate (separate repo), using `fermata-ipc` as a dependency to communicate with the REPL.

---

## 7. Related Projects

| Project | Role |
|---------|------|
| `oxur/fermata` | The notation DSL (this project) |
| `oxur/verovioxide` | Rust bindings to Verovio (MusicXML → SVG rendering) |
| Music Theory MCP Server | Knowledge base: 2691 docs, 2340 concepts, 3212 graph edges; will also host the REPL bridge tools |

---

## 8. Reference Documents

| Doc # | Name | Relevance |
|-------|------|-----------|
| 0003 | MusicXML Schema Triage | MusicXML element priority tiers; still accurate |
| 0004 | IR S-expr Format Reference | Contract between Fermata syntax and IR; Phase 5 compilation targets |
| 0006 | Original Bootstrap | Phases 1–5 context (superseded by this document for Phase 6+) |
| 0020 | Phase 5 Overview | Module layout and milestone structure template |
| 0021–0025 | Phase 5 Milestones | Implementation plan template (task/code/test pattern) |
| design-proposal | Fermata Interactive Framework Design Proposal | Full technical design (docx); the source for this document |

---

*Document version: 2026-02-01*
*Phase: 6 of 8 (REPL & Collaborative Infrastructure)*
