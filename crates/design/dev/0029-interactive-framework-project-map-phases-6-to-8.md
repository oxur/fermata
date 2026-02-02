# Fermata Interactive Framework — Project Map, Phases 6–8

> **Purpose:** Detailed roadmap for Phases 6–8 with LOE estimates, milestone decomposition, dependency analysis, and risk assessment. Used as the source for planning individual milestone documents in future sessions.
>
> **Date:** 2026-02-01
> **Related:** Bootstrap (0030), Phase 6 Overview (0031), Design Proposal (docx)

---

## 1. Scope & Boundaries

### In Scope (This Roadmap)

- Phase 6: REPL & Collaborative Infrastructure (6a–6e)
- Phase 7: MIDI Foundation (7a–7b)
- Phase 8: Bidirectional MIDI (8a–8b)

### Out of Scope (Future)

- Verovio integration (rendering MusicXML to SVG in the REPL)
- LilyPond bidirectional conversion
- Fermata language extensions (macros, user-defined forms)
- Web-based REPL or GUI
- Multi-user/collaborative sessions

---

## 2. Effort Model

### Calibration from Phases 4–5

Phase 5 produced 5 milestones (0021–0025) ranging from 865 to 1706 lines of planning documentation. Implementation of each milestone typically required 3–6 hours of Claude Code time plus review/QA cycles. The total Phase 5 effort was approximately 5 milestones × 1 planning session + 5 milestones × 1–2 implementation sessions = roughly 15–20 working sessions.

Phase 6 is structurally more complex than Phase 5 (multiple crates, IPC, concurrency) but each individual milestone is comparable in implementation size to a Phase 5 milestone. The key difference is that Phase 6 milestones have more inter-milestone dependencies and integration surface area.

### LOE Scale

For this roadmap, LOE is expressed in **milestone units** where one milestone ≈ one planning document + one implementation cycle + one QA cycle. Each milestone targets 500–1500 lines of implementation (not counting tests), which maps to roughly 3–8 hours of focused Claude Code work.

---

## 3. Phase 6: REPL & Collaborative Infrastructure

### 3.1 Sub-Phase 6a: Core REPL

**Goal:** A working interactive Fermata environment with sessions and history.
**LOE:** 3 milestones
**Risk:** Low–Medium. Well-understood problem space (REPL construction). The main risk is integration with the Phase 5 compiler, which may surface edge cases in the compiler when used interactively (partial expressions, error recovery, incremental score building).

#### Milestone 6a-1: REPL Loop & Line Editing

**Scope:**

- `fermata-repl` crate setup (Cargo.toml, dependencies, binary target)
- `rustyline` integration for line editing, Ctrl-R history search
- Core REPL loop: read → classify → eval → print
- Input classification: detect `(` for expressions, `:` for commands, pass-through for unknown
- Multi-line input: detect unmatched parentheses, continue reading on next line with `..` prompt
- Integration with `fermata` crate's `compile()` function
- Eval result display (pretty-printed Fermata syntax, or IR summary)
- Error display with source spans
- Minimal history: in-memory Vec<String> of raw inputs (replaced by typed history in 6a-3)
- `:help` and `:quit` commands

**Deliverables:** User can launch the REPL, type Fermata expressions, see compiled results or errors, and exit. No sessions yet — everything is in a single implicit context.

**Dependencies:** Phase 5 compiler (complete).

**LOE:** 1 milestone (moderate complexity — mostly integration work)

**Technical Considerations:**

- `rustyline` provides `Editor` with configurable completion, hints, and history. Start with default config; customize in 6e.
- Multi-line detection: count parentheses. If `open > close` after a line, read another. This is simple but correct for S-expressions (no string-literal edge cases since our strings use `"`).
- The Phase 5 compiler expects a complete expression. For the REPL, we need to handle "bare" inputs — e.g., just `(note c4 :q)` without wrapping in a score. The compiler's top-level interpreter (Phase 5, Milestone 5) already handles this, but verify it works in interactive context.

---

#### Milestone 6a-2: Session Model & Manager

**Scope:**

- Session struct: id, metadata, score, env, next_index (history and notebook added later)
- SessionId: short human-friendly slugs (`session-1`, user-supplied names)
- SessionMetadata: name, created, modified, tags
- Environment struct: symbol_bindings (HashMap<String, FermataValue>), implicit state (key, time, clef, tempo, default part, default voice)
- Score accumulation: each evaluated expression that produces musical content is merged into the session's score (append measure, add note to current measure, set attributes)
- SessionManager: HashMap<SessionId, Session>, active tracking, create/list/switch/info
- REPL commands: `:session new <n>`, `:session list`, `:session switch <id>`, `:session info`
- Prompt update: `[session-name] In[n]:`

**Deliverables:** User can create multiple sessions, switch between them, and each session maintains its own score and environment.

**Dependencies:** 6a-1 (REPL loop exists).

**LOE:** 1 milestone (medium complexity — data modeling, careful environment design)

**Technical Considerations:**

- Score accumulation is the trickiest part. When the user types `(note c4 :q)`, what happens? Options: (a) append to the last measure of the current part, (b) create a new measure if none exists, (c) error if no part/measure context. Recommend: the environment tracks "current position" (current part, current measure), and bare notes append to that position. `:session info` shows the current position.
- Environment bindings: start simple (String → FermataValue where FermataValue is an IR type or a Fermata AST node). Defer closures/macros to later.
- The FermataValue type is new and needs design. It wraps anything that can be bound: a Note, a Measure, a Score, a Pitch, etc. This is the evaluated representation.

**Open Question:** Should score accumulation be append-only (each expression adds to the score) or should the score be the result of replaying all Eval entries from history? Append-only is simpler and more predictable. Replay allows "undo" by truncating history. Recommend: append-only for 6a, consider replay as a future enhancement.

---

#### Milestone 6a-3: Typed History & Query

**Scope:**

- HistoryEntry, EntryKind (Eval, Command types only — AiMessage, UserMessage, System added in 6b)
- Replace the raw String history from 6a-1 with typed entries
- Monotonic index assignment (next_index on Session)
- Index-based display: `In[47]: (note c4 :q)` / `Out[47]: #<note C4 quarter>`
- HistoryQuery struct: from/to index, time range, kind filter, text search, limit
- REPL commands: `:history [range]`, `:history --code`, `:history --grep <pat>`, `:history --since <dur>`
- Result display format for Eval entries: one line for simple values, multi-line for complex
- Error entries: display error + source span

**Deliverables:** Every expression and command is tracked with an index. User can query and filter history.

**Dependencies:** 6a-2 (sessions exist; history lives on Session).

**LOE:** 1 milestone (moderate — straightforward data structures, careful display formatting)

**Technical Considerations:**

- The ScoreDelta field on Eval entries (recording what changed in the score) is useful but can be deferred — start with None for all entries, add delta tracking when we need it for notebook capture or MCP queries.
- Text search on history: simple substring match on `input` (for Eval) or `raw` (for Command). No regex needed initially.
- Duration parsing for `--since`: support `10m`, `1h`, `30s`, `2d`. Use a simple parser, or pull in the `humantime` crate.

---

### 3.2 Sub-Phase 6b: Communication Layer

**Goal:** Bidirectional messaging between user and Claude within the REPL.
**LOE:** 2 milestones
**Risk:** Low. Conceptually simple (add message types to existing infrastructure). The main risk is the concurrency boundary: the message queue must be thread-safe since MCP messages arrive on a different thread.

#### Milestone 6b-4: Chat Mode & Message Queue

**Scope:**

- `//` prefix detection in input classifier → UserMessage
- AiMessage entry type (content + source)
- SystemEvent enum (SessionCreated, SessionSwitched, etc.) and System entry type
- IncomingMessage struct (content, source, timestamp)
- MessageQueue: `Arc<Mutex<VecDeque<IncomingMessage>>>` (shared between REPL loop and IPC server thread)
- Drain-and-display: at top of REPL loop, lock queue, drain all, assign indices, commit to history, display
- Visual distinction: AI messages prefixed with `[Claude]:` in a distinct color; user messages prefixed with `[you]:`; system messages in dim/grey
- Chat messages published to a "chat outbox" (for later MCP pickup — or, in 6b, just to a local log for testing)

**Deliverables:** User can type `// hey, what key should I use here?` and it gets recorded. AI messages can be injected into the queue (programmatically, for testing) and appear at the next prompt.

**Dependencies:** 6a-3 (typed history exists).

**LOE:** 1 milestone (small–medium — mostly wiring new types into existing infrastructure)

**Technical Considerations:**

- The `//` prefix should consume the entire rest of the line as the message content (after stripping the sigil and any leading whitespace).
- For testing without the MCP bridge: add a `:test-message <text>` command that injects an AiMessage into the queue. This lets us verify the drain-and-display flow before building IPC.
- The SystemEvent enum starts small: SessionCreated(name), SessionSwitched(from, to), ReplStarted, ReplShutdown. Expand as needed.

---

#### Milestone 6b-5: Unified Timeline Verification

**Scope:**

- All five entry types interleaved in a single history Vec
- Ordering verification: write integration tests that simulate the full interleave pattern
  - User types expression → Out[n]
  - While user is "typing," inject AiMessage into queue
  - User hits Enter → AiMessage displayed first (In[n+1]), then user's result (In[n+2])
  - Verify history order matches display order
- History filtering with all types: `--code`, `--chat`, `--all`, `--system`
- `:history <range>` shows full interleaved context by default
- Display polish: column alignment for index numbers, consistent formatting across types

**Deliverables:** The unified timeline is verified to be correct under all interleaving scenarios. History filters work for all types.

**Dependencies:** 6b-4 (all entry types exist).

**LOE:** 1 milestone (small — primarily testing and verification, some display polish)

**Technical Considerations:**

- The ordering guarantee is the most important property to test. The test should simulate the concurrent scenario: a thread injecting messages into the queue while the main thread processes input. Verify that the history indices are gapless and that the order matches.
- This milestone is lighter on implementation and heavier on testing. It's a good "stabilization" milestone before building IPC.

---

### 3.3 Sub-Phase 6c: IPC & MCP Bridge

**Goal:** Claude can observe and interact with the REPL via MCP tools.
**LOE:** 2 milestones
**Risk:** Medium. This is the first multi-process integration. Risks include: protocol design issues that surface during implementation, socket lifecycle management (what if the REPL crashes? what if the MCP server connects before the REPL starts?), and serialization edge cases.

#### Milestone 6c-6: IPC Protocol & Socket Server

**Scope:**

- `fermata-ipc` crate: protocol types
  - `IpcRequest { id, method, params: serde_json::Value }`
  - `IpcResponse { id, result: Option<Value>, error: Option<IpcError> }`
  - `IpcError { code, message }`
  - Method enum: ListSessions, GetSession, GetScore, GetHistory, SendMessage, Eval, etc.
- Unix domain socket listener in REPL process
  - Bind to `~/.fermata/repl.sock` (remove stale socket on startup)
  - Accept connections on a dedicated thread
  - Read lines (one JSON object per line), parse, dispatch
- Dispatch mechanism: mpsc channel from socket thread to REPL loop
  - IPC requests are sent as `(IpcRequest, oneshot::Sender<IpcResponse>)`
  - REPL loop checks for pending IPC requests at the top of each iteration (non-blocking)
  - REPL processes request, sends response via oneshot
  - Socket thread receives response, writes JSON line back to client
- Client library (in `fermata-ipc`):
  - Connect to socket
  - Send request, wait for response (blocking)
  - Reconnect logic (if socket not available, retry with backoff)
- Integration test: spawn REPL binary, connect client, send `list_sessions`, verify response

**Deliverables:** REPL accepts IPC connections. A client can send requests and receive responses. The protocol is documented.

**Dependencies:** 6b-5 (all history types exist, message queue works).

**LOE:** 1 milestone (medium–high complexity — socket lifecycle, threading, protocol design)

**Technical Considerations:**

- Use `std::os::unix::net::UnixListener` for the socket. No need for async (tokio) — the request rate will be very low (a few requests per second at most).
- The oneshot channel for responses: use `std::sync::mpsc::channel()` with a capacity of 1, or use the `oneshot` crate. The REPL loop must check for IPC requests *without blocking* — use `mpsc::Receiver::try_recv()`.
- Stale socket cleanup: on startup, if the socket file exists, try to connect. If connection fails, the socket is stale — remove it and bind.
- The REPL should continue to work even if no MCP server is connected. The socket listener runs silently; if no client connects, there's no overhead.

---

#### Milestone 6c-7: MCP Tool Registration

**Scope:**

- Register tools in the MCP server (separate repo/crate):
  - repl_list_sessions → IPC: ListSessions
  - repl_get_session(id) → IPC: GetSession
  - repl_get_active_session → IPC: GetActiveSession
  - repl_get_score(session_id, format: fermata|musicxml|summary) → IPC: GetScore
  - repl_get_measures(session_id, part?, from?, to?) → IPC: GetMeasures
  - repl_get_history(session_id, from?, to?, since?, kinds?, grep?, limit?) → IPC: GetHistory
  - repl_send_message(session_id?, content) → IPC: SendMessage
  - repl_eval(session_id?, expr) → IPC: Eval
- Handler implementations in REPL: each IPC method handler accesses session state and returns serialized JSON
- Score serialization: three formats
  - "fermata": pretty-print via Phase 4/5 printer
  - "musicxml": emit via Phase 2 emitter
  - "summary": structured JSON (part count, measure count, key/time/clef)
- History serialization: filter and serialize matching entries as JSON
- Eval handler: compile expression, update score, return result; record as Eval entry with `source: Remote` annotation
- SendMessage handler: push to message queue; REPL displays at next prompt
- End-to-end test: Claude-simulated MCP call → REPL response

**Deliverables:** All session, score, history, and message MCP tools are functional. Claude can observe and interact with REPL sessions.

**Dependencies:** 6c-6 (IPC socket works).

**LOE:** 1 milestone (medium — many tools but each is a straightforward data query)

**Technical Considerations:**

- The `repl_eval` tool has security implications. Recommend: always record remote evals in history with a `[remote]` annotation. Consider a `:allow-remote-eval on|off` REPL setting.
- Score serialization for the "fermata" format requires the Phase 4 pretty-printer to handle arbitrary IR (not just S-expr roundtrip). This should already work but verify.
- The MCP server needs to handle the case where the REPL is not running (socket not available). Return a clear error: "Fermata REPL is not running. Start it with `fermata-repl`."

---

### 3.4 Sub-Phase 6d: Notebook

**Goal:** Structured analytical notebooks.
**LOE:** 2 milestones
**Risk:** Low–Medium. The data model is well-specified. The main complexity is the tree operations and the `:notebook capture` command (which bridges history and notebook).

#### Milestone 6d-8: Notebook Data Model & REPL Commands

**Scope:**

- Notebook, Section, Block structs (all variants)
- SectionId: UUID (internal) + display path ("2.4") computed from tree position
- Tree operations: find_section_by_path, find_section_by_id, append_block, insert_block, create_child_section, move_section
- Notebook attached to Session (each session gets one notebook)
- REPL commands:
  - `:notebook outline` — tree view with section numbers, titles, block counts
  - `:notebook show <path>` — display section contents with ANSI formatting
  - `:notebook new-section <parent> <title>` — create a new section
  - `:notebook append <path> <text>` — append a Prose block
  - `:notebook capture <path> In[m]:In[n] <caption>` — create HistoryRef block
- ANSI rendering: section headers in bold, prose in normal, HistoryRef as indented block with caption, InlineMusic with syntax highlighting

**Deliverables:** Notebooks are functional from the REPL. User can create structure, add content, and capture history ranges.

**Dependencies:** 6a-3 (history exists), 6b-5 (all entry types exist for capture).

**LOE:** 1 milestone (medium — tree data structure, REPL command parsing, display formatting)

---

#### Milestone 6d-9: Notebook MCP Tools & Export

**Scope:**

- IPC methods and MCP tools:
  - repl_notebook_outline(session_id) → section tree
  - repl_notebook_get_section(session_id, path_or_id) → section contents as JSON
  - repl_notebook_append(session_id, path_or_id, block_type, content) → success/error
  - repl_notebook_create_section(session_id, parent_path, title) → new section id + path
  - repl_notebook_search(session_id, query) → matching paths + excerpts
- Block type for MCP append: Prose, Annotation (with kind), ChatSummary (with source + timestamp)
- Markdown export: `:notebook export [section_path] [output_file]`
  - Walk tree, emit headings for sections, fenced code blocks for InlineMusic, blockquotes for Annotations, etc.
  - Default output: stdout. With file arg: write to file.
  - Section path arg: export subtree only
- End-to-end: Claude calls repl_notebook_append to add a ChatSummary; user sees it via `:notebook show`

**Deliverables:** Notebooks are fully accessible from MCP. Export to Markdown works.

**Dependencies:** 6d-8 (notebook model exists), 6c-7 (MCP tools framework exists).

**LOE:** 1 milestone (medium — MCP wiring + Markdown generation)

---

### 3.5 Sub-Phase 6e: Persistence & Polish

**Goal:** Sessions survive restarts. UX refinements.
**LOE:** 2 milestones
**Risk:** Low. Serialization is well-understood. The main risk is environment serialization (closures, if any, are not trivially serializable).

#### Milestone 6e-10: Session Persistence

**Scope:**

- File layout: `~/.fermata/sessions/<id>/` with meta.json, score.ferm, env.json, history.jsonl, notebook.json
- Save: serialize each component to its file. score.ferm via pretty-printer. history.jsonl as one JSON line per entry.
- Load: deserialize and reconstruct Session struct. History entries get their original indices.
- REPL commands: `:session save`, `:session load <id>`
- Autosave: configurable interval (default 60s). history.jsonl is append-only between saves. Full save on `:quit`.
- Recovery: on startup, check for sessions with a `.dirty` marker. Offer to recover.
- REPL startup behavior: if saved sessions exist, list them and ask which to load (or create new). If no saved sessions, create a default session.
- `:session delete <id>` — remove from memory and disk (with confirmation prompt)

**Deliverables:** Sessions survive REPL restarts. Autosave prevents data loss.

**Dependencies:** 6d-9 (notebook exists, all session components are in place).

**LOE:** 1 milestone (medium — file I/O, serialization, startup flow)

**Technical Considerations:**

- Environment serialization: bindings that are simple values (Note, Pitch, Measure, etc.) serialize as Fermata syntax strings. If we have closures or macros by this point, they either serialize as their source text or are dropped with a warning.
- The score.ferm file should be a complete, valid Fermata source file that, when compiled, reproduces the session's score. This is the most human-friendly format and enables `git diff`.
- history.jsonl: each line is `{"index": 47, "timestamp": "...", "kind": "eval", "data": {...}}`. On load, verify indices are monotonic.

---

#### Milestone 6e-11: History References & UX Polish

**Scope:**

- `$In[n]` and `$Out[n]` substitution in Fermata expressions
  - Preprocessor pass: scan input for `$In[n]` / `$Out[n]`, look up history, substitute
  - `$In[n]` → the parsed S-expression from entry n (re-evaluatable)
  - `$Out[n]` → the result value from entry n (already evaluated)
  - Error: "No Out[n]: entry n was not an Eval" or "Entry n does not exist"
- Tab completion:
  - REPL commands (`:session`, `:notebook`, `:history`, `:help`, `:quit`)
  - Session names (for `:session switch`)
  - Notebook section paths (for `:notebook show`)
  - Fermata keywords (`:q`, `:quarter`, `:major`, etc.)
- ANSI color scheme:
  - Prompt: blue/cyan
  - AI messages: green with `[Claude]:` prefix
  - User chat messages: yellow with `[you]:` prefix
  - System messages: dim grey
  - Errors: red
  - Results: default (or white)
  - History indices: dim
- `:session rename <id> <new-name>`
- Error message polish: include source span underlines (Rust-style ^^^)
- `:help <topic>` with per-command documentation

**Deliverables:** History references work. REPL feels polished and professional.

**Dependencies:** 6e-10 (persistence, all features in place).

**LOE:** 1 milestone (medium — many small features, testing across the board)

---

## 4. Phase 7: MIDI Foundation

**LOE:** 2–3 milestones
**Risk:** Medium. MIDI is well-specified but platform-specific. Testing requires virtual MIDI devices or hardware.

### Milestone 7a-12: MIDI Device Layer

**Scope:**

- `fermata-midi` crate setup
- `midir` crate integration for cross-platform MIDI
- Device enumeration: list output and input ports with names
- Connection management: open, close, track state per session (MidiSessionState)
- REPL commands: `:midi devices`, `:midi connect <port>`, `:midi disconnect`
- MCP tool: repl_midi_devices

**LOE:** 1 milestone

### Milestone 7b-13: MIDI Output & Playback

**Scope:**

- MIDI message types (NoteOn, NoteOff, CC, ProgramChange)
- Raw message sending to open output port
- Score-to-MIDI conversion:
  - Pitch → MIDI note number (C4 = 60)
  - Duration → MIDI ticks (at configurable TPQN and tempo)
  - Dynamics → velocity (pp=32 through ff=112)
  - Articulation → duration scaling (staccato = 50%, legato = 100%)
- Playback engine: spawn thread, send timed messages, respect tempo
- REPL commands: `:play` (whole score), `:play 1:4` (measures 1–4), `:play $Out[47]`
- MCP tools: repl_midi_play, repl_midi_send

**LOE:** 1–2 milestones (conversion is straightforward; real-time playback requires care)

---

## 5. Phase 8: Bidirectional MIDI

**LOE:** 2–3 milestones
**Risk:** Medium–High. MIDI input quantization is a research-adjacent problem. Enharmonic spelling requires musical heuristics.

### Milestone 8a-14: MIDI Input & Quantization

**Scope:**

- MIDI input listener (background thread, buffers events with timestamps)
- Quantization engine: snap onsets/durations to configurable grid
- MIDI note number → Fermata Pitch (with enharmonic spelling from key context)
- Chord detection (simultaneous onsets within tolerance)
- MCP tool: repl_midi_get_input_buffer

**LOE:** 1–2 milestones

### Milestone 8b-15: Recording & Full Loop

**Scope:**

- `:record` / `:stop` commands
- Captured MIDI → quantized → Fermata IR → appended to score + history
- MCP tool: repl_midi_get_recording
- Full bidirectional loop: user plays → Claude analyzes → Claude responds
- Integration tests with virtual MIDI ports

**LOE:** 1 milestone

---

## 6. Summary Table

| Phase | Sub | Milestone | Name | LOE | Dependencies | Risk |
|-------|-----|-----------|------|-----|--------------|------|
| 6 | a | 6a-1 | REPL Loop & Line Editing | 1 | Phase 5 | Low |
| 6 | a | 6a-2 | Session Model & Manager | 1 | 6a-1 | Med |
| 6 | a | 6a-3 | Typed History & Query | 1 | 6a-2 | Low |
| 6 | b | 6b-4 | Chat Mode & Message Queue | 1 | 6a-3 | Low |
| 6 | b | 6b-5 | Unified Timeline Verification | 1 | 6b-4 | Low |
| 6 | c | 6c-6 | IPC Protocol & Socket Server | 1 | 6b-5 | Med |
| 6 | c | 6c-7 | MCP Tool Registration | 1 | 6c-6 | Med |
| 6 | d | 6d-8 | Notebook Model & REPL Commands | 1 | 6a-3, 6b-5 | Med |
| 6 | d | 6d-9 | Notebook MCP Tools & Export | 1 | 6d-8, 6c-7 | Low |
| 6 | e | 6e-10 | Session Persistence | 1 | 6d-9 | Low |
| 6 | e | 6e-11 | History References & UX Polish | 1 | 6e-10 | Low |
| 7 | a | 7a-12 | MIDI Device Layer | 1 | 6a-1 | Med |
| 7 | b | 7b-13 | MIDI Output & Playback | 1–2 | 7a-12, 6c-7 | Med |
| 8 | a | 8a-14 | MIDI Input & Quantization | 1–2 | 7a-12 | High |
| 8 | b | 8b-15 | Recording & Full Loop | 1 | 8a-14, 6c-7 | Med |
| | | | **Total** | **14–17** | | |

---

## 7. Dependency Graph

```
Phase 5 (complete)
    │
    ▼
  6a-1  REPL Loop
    │
    ▼
  6a-2  Session Model ──────────────────────────────┐
    │                                                │
    ▼                                                │
  6a-3  Typed History ──────────────────┐            │
    │                                   │            │
    ▼                                   │            │
  6b-4  Chat & Messages                 │            │
    │                                   │            │
    ▼                                   │            │
  6b-5  Unified Timeline ──────┐        │            │
    │                          │        │            │
    ▼                          ▼        ▼            │
  6c-6  IPC Socket         6d-8  Notebook Model      │
    │                          │                     │
    ▼                          ▼                     │
  6c-7  MCP Tools ────────▶ 6d-9  Notebook MCP       │
                               │                     │
                               ▼                     │
                           6e-10  Persistence ◄──────┘
                               │
                               ▼
                           6e-11  Polish

  6a-1 ───▶ 7a-12  MIDI Devices
                │
                ▼
  6c-7 ───▶ 7b-13  MIDI Output
                │
                ▼
            8a-14  MIDI Input
                │
                ▼
  6c-7 ───▶ 8b-15  Full MIDI Loop
```

**Critical path:** 6a-1 → 6a-2 → 6a-3 → 6b-4 → 6b-5 → 6c-6 → 6c-7 → 6d-9 → 6e-10 → 6e-11

**Parallelizable:** 6d-8 (notebook model) can start as soon as 6a-3 is done, in parallel with 6b-4/6b-5. However, the MCP tools for notebook (6d-9) require 6c-7.

**MIDI is independent:** 7a-12 only depends on 6a-1 (the REPL loop). MIDI device enumeration and connection can be developed in parallel with 6b–6d. However, MIDI MCP tools (7b-13) require 6c-7.

---

## 8. Open Questions

### Technical / Architectural

| # | Question | Impact | Recommendation |
|---|----------|--------|----------------|
| Q1 | Chat sigil: `//` vs `>` vs `@` | UX, input parsing | `//` — no conflict with Fermata syntax, reads as "comment to Claude" |
| Q2 | IPC: std sockets vs tokio | Crate complexity | `std::os::unix::net` — low request rate, no need for async |
| Q3 | Score snapshots in ScoreRef blocks | Notebook complexity | Defer — start with live references only |
| Q4 | Score accumulation model | Session semantics | Append-only — simplest, most predictable |
| Q5 | Environment closures | Serialization | Defer closures — start with value bindings only |
| Q6 | History size limits | Memory | Unbounded initially; add paging to disk if sessions exceed 10K entries |
| Q7 | MIDI quantization algorithm | Accuracy | Nearest-grid-point; add sophistication later |
| Q8 | Enharmonic spelling | Correctness | Key-context-based; flag ambiguous cases |
| Q9 | Line editing library | UX | `rustyline` (mature, well-documented) |
| Q10 | Multi-line input handling | REPL UX | Parenthesis counting; continuation prompt `..` |

### Project / Process

| # | Question | Impact | Recommendation |
|---|----------|--------|----------------|
| P1 | MCP server integration | Deployment | Single server with tool groups — simpler for user configuration |
| P2 | IPC testing strategy | Quality | Spawn REPL in test, connect client, verify — no mocking |
| P3 | MIDI testing without hardware | Testability | Virtual MIDI ports (IAC on macOS, snd-virmidi on Linux) |
| P4 | Milestone planning cadence | Process | Plan one sub-phase at a time; 6a milestones first, then 6b, etc. |
| P5 | Documentation strategy | Maintainability | IPC protocol spec, MCP tool reference, REPL command reference as living docs |
| P6 | Crate publication order | Distribution | fermata-ipc first (no deps), then fermata-repl (depends on fermata + fermata-ipc) |

---

## 9. Next Steps

1. **Resolve Q1, Q2, Q4, Q9** (quick decisions needed before 6a milestone planning)
2. **Plan 6a milestones in detail** (0032, 0033, 0034 milestone documents)
3. **Set up `fermata-repl` crate** with Cargo workspace configuration
4. **Implement 6a-1** (REPL loop — the foundation for everything)

---

*Document version: 2026-02-01*
*Covers: Phases 6–8*
