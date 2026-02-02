# Phase 6: REPL & Collaborative Infrastructure — Overview

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Complete interactive runtime — REPL, sessions, history, notebook, IPC, MCP bridge
> **Depends On:** Phases 1–5 (complete)
> **Related:** Design Proposal (docx), Bootstrap Document (0030)

---

## Summary

Phase 6 transforms Fermata from a batch compiler into an interactive collaborative environment. It is the largest phase in the project, spanning five sub-phases (6a–6e) and approximately 9–12 milestones. The deliverables include a new `fermata-repl` crate (the REPL binary and session infrastructure) and a new `fermata-ipc` crate (the protocol layer connecting the REPL to the MCP server).

### Success Criteria

At the end of Phase 6:

1. A user can launch the Fermata REPL, create sessions, enter Fermata syntax, and see results
2. Sessions persist across REPL restarts (save/load)
3. Every REPL interaction is captured in a numbered, typed, filterable history
4. The user and Claude can exchange messages within the REPL (// chat mode)
5. Claude can observe sessions, read scores, query history, and send messages via MCP tools
6. The user can maintain structured analytical notebooks with prose, score references, and history captures
7. Claude can read and write to notebooks via MCP tools
8. History entries are referenceable in subsequent expressions ($In[n], $Out[n])

---

## Architecture

### Crate Layout

```
oxur/fermata/
└── crates/
    ├── fermata/                 # Existing — unchanged by Phase 6
    │   └── src/
    │       ├── ir/              # Typed IR (Phase 1)
    │       ├── musicxml/        # Emit/parse (Phases 2–3)
    │       ├── sexpr/           # S-expr read/print (Phase 4)
    │       └── lang/            # User syntax → IR (Phase 5)
    │
    ├── fermata-repl/            # NEW — Phase 6
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs          # REPL binary entry point
    │       ├── lib.rs           # Public API (for testing, for MCP bridge)
    │       ├── repl/
    │       │   ├── mod.rs       # REPL loop, input classification
    │       │   ├── prompt.rs    # Prompt rendering (session, index, ANSI)
    │       │   ├── commands.rs  # :command dispatch and handlers
    │       │   └── display.rs   # Result/message formatting, ANSI output
    │       ├── session/
    │       │   ├── mod.rs       # Session struct, SessionManager
    │       │   ├── manager.rs   # Create, list, switch, delete
    │       │   ├── env.rs       # Environment (bindings, defaults)
    │       │   └── eval.rs      # Expression evaluation (wraps fermata compiler)
    │       ├── history/
    │       │   ├── mod.rs       # HistoryEntry, EntryKind
    │       │   ├── query.rs     # HistoryQuery, filtering, text search
    │       │   └── refs.rs      # $In[n]/$Out[n] substitution
    │       ├── notebook/
    │       │   ├── mod.rs       # Notebook, Section, Block
    │       │   ├── ops.rs       # Append, insert, move, search
    │       │   ├── render.rs    # REPL display rendering (ANSI)
    │       │   └── export.rs    # Markdown export
    │       ├── messages/
    │       │   ├── mod.rs       # IncomingMessage, MessageQueue
    │       │   └── display.rs   # Message rendering (prefix, color)
    │       └── persist/
    │           ├── mod.rs       # Save/load orchestration
    │           ├── score.rs     # score.ferm read/write
    │           ├── history.rs   # history.jsonl append/read
    │           ├── notebook.rs  # notebook.json read/write
    │           └── recovery.rs  # Crash recovery, autosave
    │
    └── fermata-ipc/             # NEW — Phase 6c
        ├── Cargo.toml
        └── src/
            ├── lib.rs           # Public API
            ├── protocol.rs      # Request/Response/Error types
            ├── server.rs        # Unix socket listener (in REPL process)
            └── client.rs        # Unix socket connector (in MCP server)
```

### Dependency Graph

```
fermata-repl ──depends──▶ fermata       (for compiler, IR types)
fermata-repl ──depends──▶ fermata-ipc   (for socket server)
MCP server   ──depends──▶ fermata-ipc   (for socket client)
```

The MCP server (separate repo) adds `fermata-ipc` as a dependency. The new REPL MCP tools are registered alongside the existing music theory tools, or in a separate tool group — this is an open question (see project map, Q: P1).

### Data Flow

```
User types in REPL
     │
     ▼
┌────────────────┐     mpsc channel     ┌──────────────────┐
│   REPL Loop    │◄─────────────────────│  IPC Server      │
│  (main thread) │                      │  (socket thread) │
│                │                      └────────┬─────────┘
│  • Drain msgs  │                               │
│  • Read input  │                               │ Unix socket
│  • Eval/exec   │                               │
│  • Commit hist │                      ┌────────┴─────────┐
│  • Display     │                      │  MCP Server      │
│                │                      │  (separate proc) │
│  Owns:         │                      │                  │
│  • Sessions    │                      │  Translates MCP  │
│  • History     │                      │  tool calls to   │
│  • Notebooks   │                      │  IPC requests    │
└────────────────┘                      └──────────────────┘
```

---

## Sub-Phase Breakdown

### 6a — Core REPL (Milestones 1–3)

**Goal:** A working interactive Fermata environment with sessions and basic history.

**Milestone 1: REPL Loop & Evaluation**

- Binary entry point (`fermata-repl`)
- Core REPL loop: read → classify → eval → print
- Integration with the Phase 5 compiler for expression evaluation
- Basic prompt (no session info yet)
- Eval and Command entry types only (minimal history)
- Multi-line input handling (detect incomplete S-expressions)
- Line editing library integration (rustyline recommended)

**Milestone 2: Session Model**

- Session struct (score, env, history, next_index)
- SessionManager (create, list, switch, get_active)
- Environment basics (implicit state: current key, time, clef, tempo)
- Score accumulation (evaluated expressions update the session's score)
- REPL commands: `:session new <name>`, `:session list`, `:session switch <id>`, `:session info`
- Prompt update: show session name and next index

**Milestone 3: History & Commands**

- Complete HistoryEntry / EntryKind types
- Numbered history display: `In[47]:`, `Out[47]:`
- HistoryQuery struct with index range and type filters
- REPL commands: `:history [range]`, `:history --code`, `:help`
- History text search: `:history --grep <pattern>`

### 6b — Communication Layer (Milestones 4–5)

**Goal:** Bidirectional messaging between user and Claude within the REPL.

**Milestone 4: Chat & Messages**

- `//` prefix detection → UserMessage entry type
- AiMessage entry type
- MessageQueue (VecDeque with mpsc channel for thread-safe enqueue)
- Drain-and-display at top of REPL loop
- Visual distinction for AI messages vs. user messages vs. eval output
- System entry type for session events

**Milestone 5: Unified Timeline**

- All five entry types (Eval, Command, AiMessage, UserMessage, System) interleaved in history
- Ordering guarantee: messages displayed before the input that triggered the drain
- History filtering expanded: `--chat` (AI + User messages), `--all`, `--system`
- Verify: history replay matches user's experienced order

### 6c — IPC & MCP Bridge (Milestones 6–7)

**Goal:** Claude can observe and interact with the REPL via MCP tools.

**Milestone 6: IPC Protocol & Socket**

- `fermata-ipc` crate: Request, Response, Error protocol types
- Unix domain socket listener in REPL process (~/.fermata/repl.sock)
- Socket handler thread: receive JSON requests, dispatch via mpsc to REPL loop
- REPL loop: process IPC requests between user inputs (or via oneshot response channel)
- Client library: connect, send request, receive response
- Integration test: start REPL, connect client, send list_sessions, verify response

**Milestone 7: MCP Tools**

- MCP tool registrations in the MCP server for all session/score/history/message tools
- repl_list_sessions, repl_get_session, repl_get_active_session
- repl_get_score (Fermata syntax, MusicXML, or IR summary output formats)
- repl_get_measures (with part and range filters)
- repl_get_history (with HistoryQuery parameters)
- repl_send_message (enqueue → REPL displays at next prompt)
- repl_eval (evaluate expression in session; audit trail in history)
- End-to-end test: Claude (via MCP) reads a session, sends a message, user sees it

### 6d — Notebook (Milestones 8–9)

**Goal:** Structured analytical notebooks, writable and readable from both REPL and MCP.

**Milestone 8: Notebook Data Model & REPL Commands**

- Notebook, Section, Block structs (full enum with all variants)
- SectionId generation (stable, survives reordering)
- Display numbering computation from tree position
- Tree operations: append, insert, move section, search
- REPL commands: `:notebook outline`, `:notebook show <path>`, `:notebook new-section <parent> <title>`
- REPL commands: `:notebook append <path> <text>`, `:notebook capture <path> In[m]:In[n] <caption>`
- ANSI rendering for notebook display

**Milestone 9: Notebook MCP Tools & Export**

- repl_notebook_outline, repl_notebook_get_section
- repl_notebook_append (Prose, Annotation, ChatSummary block types)
- repl_notebook_create_section
- repl_notebook_search
- Markdown export: `:notebook export [path] [file]`
- End-to-end: Claude appends a summary to section 2.4, user sees it via `:notebook show 2.4`

### 6e — Persistence & Polish (Milestones 10–11)

**Goal:** Sessions survive restarts; UX refinements.

**Milestone 10: Persistence**

- Session save: score.ferm + env.json + history.jsonl + notebook.json + meta.json
- Session load: reconstruct Session from disk files
- REPL commands: `:session save`, `:session load <id>`
- Autosave (configurable interval, append-only for history)
- Session recovery (detect dirty state on startup, offer recovery)
- REPL startup: auto-load previous session or create default

**Milestone 11: History References & Polish**

- $In[n] and $Out[n] substitution in expressions
- Tab completion for REPL commands and session names
- ANSI color scheme: AI messages, user messages, errors, results, system events
- `:session delete <id>` (with confirmation)
- `:session rename <id> <name>`
- Error message polish: source spans, suggestions
- Man page or `:help <topic>` for all commands

---

## REPL Command Reference (Complete)

| Command | Sub-phase | Description |
|---------|-----------|-------------|
| `:help [topic]` | 6a | Show help; optional topic |
| `:session new <name>` | 6a | Create a new session |
| `:session list` | 6a | List all sessions |
| `:session switch <id>` | 6a | Switch active session |
| `:session info` | 6a | Current session detail |
| `:session save` | 6e | Save active session to disk |
| `:session load <id>` | 6e | Load session from disk |
| `:session delete <id>` | 6e | Delete session (with confirm) |
| `:session rename <id> <name>` | 6e | Rename a session |
| `:history [range]` | 6a | Show history (e.g., `:history 40:50`) |
| `:history --code` | 6a | Show only Eval entries |
| `:history --chat` | 6b | Show only AI + User messages |
| `:history --all` | 6b | Show all entry types |
| `:history --system` | 6b | Show system events |
| `:history --grep <pat>` | 6a | Text search in history |
| `:history --since <dur>` | 6a | Time-based filter (e.g., `10m`, `1h`) |
| `:notebook outline` | 6d | Show notebook section tree |
| `:notebook show <path>` | 6d | Show section contents (e.g., `2.4`) |
| `:notebook new-section <parent> <title>` | 6d | Create section |
| `:notebook append <path> <text>` | 6d | Append prose to section |
| `:notebook capture <path> In[m]:In[n] <caption>` | 6d | Capture history range |
| `:notebook export [path] [file]` | 6d | Export to Markdown |

---

## Key Dependencies (External Crates)

| Crate | Purpose | Sub-phase |
|-------|---------|-----------|
| `rustyline` | Line editing, history search (Ctrl-R), tab completion | 6a |
| `chrono` | Timestamps for history entries | 6a |
| `serde` + `serde_json` | Serialization for IPC protocol, persistence | 6a |
| `uuid` (or similar) | Session and section IDs | 6a |
| `tokio` or `std::os::unix` | Unix domain socket | 6c |
| `crossterm` or `termcolor` | ANSI color output | 6a (basic), 6e (polish) |

**Note on async:** The REPL loop is synchronous (single-threaded, blocking on input). The IPC socket listener runs on a separate thread. The simplest approach uses `std::os::unix::net` for the socket and `std::sync::mpsc` for the channel. Tokio is only needed if we want async I/O on the socket, which is unlikely to be necessary given the low request rate.

---

## Testing Strategy

| Level | What | How |
|-------|------|-----|
| Unit | History filtering, notebook tree ops, duration conversion | Standard `#[test]` |
| Integration | REPL eval → score update → history entry | In-process tests using `fermata-repl` as a library |
| IPC | Socket server ↔ client round-trip | Spawn REPL, connect client, verify responses |
| End-to-end | Full flow: user input → eval → MCP query → message → display | Test harness driving both REPL and MCP client |

---

## Open Design Decisions for Milestone Planning

These should be resolved during milestone planning, not before:

| Decision | Context | When |
|----------|---------|------|
| Chat sigil: `//` vs alternatives | Need to verify no conflict with Fermata syntax | Milestone 4 |
| Async vs sync IPC | Determines socket library choice | Milestone 6 |
| Session auto-creation on startup | Always create "default" session, or ask? | Milestone 2 |
| Score accumulation model | Append-only, or full replace on each eval? | Milestone 2 |
| Environment mutability | Are bindings immutable (rebind only) or mutable? | Milestone 2 |

---

*Next: See Project Map document for LOE estimates and dependency analysis.*
*Then: Individual milestone planning documents will be created per sub-phase.*
