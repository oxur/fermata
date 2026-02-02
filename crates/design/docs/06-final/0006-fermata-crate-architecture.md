---
number: 6
title: "Fermata Crate Architecture"
author: "both the"
component: All
tags: [change-me]
created: 2026-02-01
updated: 2026-02-01
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Fermata Crate Architecture

> **Purpose:** Defines the Cargo workspace structure for the Fermata project, covering the rationale for crate boundaries, dependency management, and Cargo feature flags. This document governs how code is organized across crates for Phases 6–8 and beyond.
>
> **Date:** 2026-02-01
> **Status:** Active

---

## 1. Current State (Post-Phase 5)

The workspace has two crates:

```
crates/
├── design/       # Documentation crate (no runtime code)
└── fermata/      # Everything: IR, MusicXML, S-expr, Lang
```

The `fermata` crate contains four internal modules that map cleanly to Phases 1–5:

| Module | Phase | Purpose |
|--------|-------|---------|
| `ir/` | 1 | Typed IR mirroring MusicXML 4.0 |
| `musicxml/` | 2–3 | MusicXML emit and parse |
| `sexpr/` | 4 | S-expression read/print for IR |
| `lang/` | 5 | User-facing Fermata syntax → IR compiler |

These modules are tightly coupled — a change to a type in `ir/` often ripples through `musicxml/`, `sexpr/`, and `lang/`. They share the same dependency profile (primarily `nom`, `quick-xml`, `serde`) and represent a single conceptual unit: **the Fermata language and its format conversions**.

---

## 2. The Problem

Phase 6 introduces the REPL, sessions, history, notebooks, IPC, and an MCP bridge. Phases 7–8 add MIDI. This brings in:

- **Runtime dependencies:** `rustyline` (line editing), `chrono` (timestamps), `crossterm` (ANSI), Unix domain sockets
- **Platform-specific native dependencies:** `midir` (CoreMIDI on macOS, ALSA on Linux)
- **A binary target:** the REPL is an executable, not a library
- **Cross-process sharing:** the IPC protocol must be importable by both the REPL and the MCP server (a separate repository)

Putting all of this into the existing `fermata` crate would mean:

1. Anyone depending on `fermata` as a library (to compile notation programmatically) inherits REPL, socket, and MIDI dependencies they don't need
2. Compile times increase for all contributors, even those only touching the compiler
3. The MCP server (separate repo) would need to depend on the entire `fermata` crate just to get protocol types
4. Feature-flag spaghetti within a single crate to manage optional subsystems

---

## 3. Design Principle

**Split along dependency and deployment boundaries, not module boundaries.**

The four existing modules (`ir`, `musicxml`, `sexpr`, `lang`) should *not* become separate crates — they are too tightly coupled and change together. Extracting them would create a public API contract at every module boundary, making routine refactors (adding a field to `Note`, changing an enum variant) into multi-crate breaking changes.

Instead, new crates are introduced only when there is a clear difference in:

- **Dependency profile** (e.g., MIDI pulls in native platform libraries)
- **Deployment target** (e.g., the REPL is a binary; the core is a library)
- **Consumer set** (e.g., the IPC protocol is shared between two separate processes)

---

## 4. Proposed Structure

```
crates/
├── design/                  # Unchanged — documentation
│
├── fermata/                 # CORE — the language, compiler, IR, formats
│   ├── Cargo.toml           #   Pure library. Minimal dependencies.
│   └── src/                 #   No runtime, no I/O, no platform specifics.
│       ├── ir/              #   Typed IR (Phase 1)
│       ├── musicxml/        #   Emit/parse (Phases 2–3)
│       ├── sexpr/           #   S-expr read/print (Phase 4)
│       └── lang/            #   User syntax → IR (Phase 5)
│
├── fermata-repl/            # INTERACTIVE — the REPL binary + session runtime
│   ├── Cargo.toml           #   Binary crate. Depends on fermata + fermata-ipc.
│   └── src/                 #   Owns all interactive/stateful concerns.
│       ├── main.rs          #   Binary entry point
│       ├── lib.rs           #   Public API (for integration testing)
│       ├── repl/            #   Loop, prompt, commands, display
│       ├── session/         #   Session, SessionManager, Environment, eval
│       ├── history/         #   HistoryEntry, EntryKind, query, filtering
│       ├── notebook/        #   Notebook, Section, Block, tree ops, export
│       ├── messages/        #   MessageQueue, incoming message display
│       └── persist/         #   Save/load, autosave, recovery
│
├── fermata-ipc/             # PROTOCOL — shared between REPL and MCP server
│   ├── Cargo.toml           #   Tiny library. Only serde + serde_json.
│   └── src/                 #   No runtime, no I/O beyond type definitions.
│       ├── lib.rs
│       ├── protocol.rs      #   Request, Response, Error, Method types
│       ├── server.rs        #   Unix socket listener (used by REPL)
│       └── client.rs        #   Unix socket connector (used by MCP server)
│
└── fermata-midi/            # SOUND — MIDI device management and conversion
    ├── Cargo.toml           #   Optional. Native platform dependencies (midir).
    └── src/                 #   Phase 7–8 only.
        ├── lib.rs
        ├── devices.rs       #   Enumeration, connection management
        ├── convert.rs       #   Score ↔ MIDI event conversion
        ├── playback.rs      #   Timed message sending
        ├── input.rs         #   MIDI input listener
        └── quantize.rs      #   Onset/duration quantization
```

---

## 5. Rationale for Each Crate

### `fermata` (unchanged)

**What it is:** The pure-library core. Compile Fermata syntax to IR, emit MusicXML, parse MusicXML, round-trip through S-expressions.

**Why it stays as one crate:** The four modules are tightly coupled. They share types (`ir/` types flow through everything), they change together, and they have the same dependency profile. Splitting them would create artificial API boundaries that increase maintenance cost without benefit.

**Who depends on it:** Everything else. `fermata-repl` imports it for compilation. `fermata-midi` imports it for IR types. External consumers import it as a notation library.

**Dependencies:** `nom`, `quick-xml`, `serde`, `serde_json` — all lightweight, pure-Rust, no platform specifics.

### `fermata-repl`

**What it is:** The interactive runtime. REPL loop, sessions, history, notebooks, message queue, persistence. Also the binary entry point.

**Why it's separate:** It introduces a fundamentally different dependency profile: `rustyline` (terminal I/O, platform-specific readline), `chrono` (timestamps), `crossterm` or `termcolor` (ANSI), file system I/O for persistence, and threading for the IPC socket listener. None of these are needed by the core language.

**Why it's not split further:** The internal modules (session, history, notebook, messages, persist) are tightly coupled. History lives on Session. Notebook lives on Session. The REPL loop orchestrates all of them. Splitting into `fermata-session`, `fermata-history`, `fermata-notebook`, etc. would create circular dependency pressure and excessive public API surface. They belong together as modules within one crate.

**Who depends on it:** End users (as a binary). Integration tests (as a library via `lib.rs`).

### `fermata-ipc`

**What it is:** The communication protocol between the REPL process and the MCP server process.

**Why it's separate:** It is consumed by two independent codebases — the REPL (this repo) and the MCP server (separate repo). The MCP server should be able to import just the protocol types and the client connector without pulling in the entire REPL or core language.

**Size:** This is intentionally small — probably 500–800 lines total. The protocol types (`IpcRequest`, `IpcResponse`, `IpcError`, method enum), the socket server, and the socket client. That's it.

**Dependencies:** `serde`, `serde_json`, and `std::os::unix::net`. Nothing else.

### `fermata-midi`

**What it is:** MIDI device management, score-to-MIDI conversion, MIDI-to-score conversion, playback, and recording.

**Why it's separate:** `midir` links against native platform MIDI frameworks (CoreMIDI on macOS, ALSA on Linux, WinMM on Windows). These are heavy, platform-specific dependencies with native build requirements. A developer working on the REPL's notebook feature shouldn't need ALSA development headers installed.

**Why it exists at all (vs. being part of fermata-repl):** Beyond the dependency argument, MIDI is a cleanly separable concern. The conversion logic (Score ↔ MIDI events) is pure and testable independently. The device management is platform-specific and benefits from isolated testing. And it's plausible that MIDI conversion could be useful outside the REPL context (e.g., a batch `fermata-to-midi` CLI tool).

---

## 6. Cargo Features

The REPL crate uses Cargo features to make MIDI support opt-in:

```toml
# fermata-repl/Cargo.toml

[features]
default = []
midi = ["dep:fermata-midi"]

[dependencies]
fermata = { path = "../fermata" }
fermata-ipc = { path = "../fermata-ipc" }

[dependencies.fermata-midi]
path = "../fermata-midi"
optional = true
```

In the REPL source, MIDI-dependent code is gated:

```rust
// session/mod.rs
pub struct Session {
    // ... always present ...
    #[cfg(feature = "midi")]
    pub midi_state: Option<MidiSessionState>,
}

// repl/commands.rs
#[cfg(feature = "midi")]
fn handle_midi_command(args: &str) -> Result<()> { ... }
```

### Build Profiles

| Profile | Command | Use Case |
|---------|---------|----------|
| Development (no MIDI) | `cargo build -p fermata-repl` | Phase 6 work, fast builds |
| Development (with MIDI) | `cargo build -p fermata-repl --features midi` | Phase 7–8 work |
| Release | `cargo build --release -p fermata-repl --features midi` | Distribution |
| Library only | `cargo build -p fermata` | External consumers, CI |

### Future Feature Flags

The same pattern extends to future optional integrations:

```toml
[features]
default = []
midi = ["dep:fermata-midi"]
# verovio = ["dep:verovioxide"]     # Future: notation rendering
# web = ["dep:fermata-web"]          # Future: web UI
```

---

## 7. Dependency Graph

```
                    ┌──────────┐
                    │ fermata  │  (pure library, no optional deps)
                    └────┬─────┘
                         │
              ┌──────────┼──────────────┐
              │          │              │
              ▼          ▼              ▼
     ┌──────────┐  ┌───────────┐  ┌───────────┐
     │ fermata  │  │ fermata   │  │ fermata   │
     │ -repl    │  │ -ipc      │  │ -midi     │
     └──┬───┬───┘  └───────────┘  └───────────┘
        │   │           ▲              ▲
        │   └───────────┘              │
        └──────────────────────────────┘
                  (optional, via feature flag)

External:
     ┌───────────┐
     │ MCP       │──depends──▶ fermata-ipc (client only)
     │ Server    │──depends──▶ fermata (for IR types in tool responses)
     └───────────┘
```

---

## 8. What This Means in Practice

**For Phase 6 development:** Create `fermata-repl` and `fermata-ipc`. The `fermata` crate is unchanged. All new code lands in the new crates. Claude Code working on the compiler never rebuilds REPL code; Claude Code working on the REPL gets fast incremental builds because `fermata` (the big dependency) is already compiled.

**For the MCP server:** Add `fermata-ipc` as a dependency (path or git). Import `fermata_ipc::client::IpcClient` and `fermata_ipc::protocol::*`. No dependency on `fermata-repl` or `fermata-midi`.

**For Phase 7–8:** Create `fermata-midi`. Add it as an optional dependency to `fermata-repl`. Gate MIDI code behind `#[cfg(feature = "midi")]`. Developers without MIDI hardware (or without interest in MIDI) are unaffected.

**For external library users:** `cargo add fermata` gives you the notation DSL. No REPL, no MIDI, no sockets. Just the language.

---

*Document version: 2026-02-01*
