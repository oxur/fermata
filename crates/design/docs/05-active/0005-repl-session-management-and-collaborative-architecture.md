---
number: 5
title: "REPL, Session Management, and Collaborative Architecture"
author: "stable IDs"
component: All
tags: [change-me]
created: 2026-02-01
updated: 2026-02-01
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# REPL, Session Management, and Collaborative Architecture

*Fermata Interactive Framework: Design Proposal*

Covers: Phase 6 (REPL), Phase 7 (MIDI Foundation), Phase 8 (MIDI Integration)

## **Part I: Product Vision & Capabilities**

### **1.1  The Big Picture**

Fermata began as a domain-specific language for generating MusicXML from S-expressions, motivated by a simple need: precise, concise notation for human–AI conversations about music theory. Through five phases of development, it has grown into a typed intermediate representation, a MusicXML emitter, an S-expression reader/printer, and a compiler from user-facing syntax to that IR.

But notation is only the beginning. The broader vision is an *interactive music collaboration framework*: an environment where a human musician/theorist and an AI assistant work together in real time, passing musical ideas back and forth with full notational precision, shared analytical context, and audible feedback. This document proposes the architecture for that environment.

The framework rests on three pillars that have already been built or are in active development:

**The Music Theory Knowledge Base.** A custom MCP server backed by 2,691 indexed documents (concept cards, source chapters, guides) and a graph database of 2,340 concepts connected by 3,212 edges. This gives the AI deep, searchable, interconnected knowledge of music theory from fundamentals through advanced topics in set theory, group theory, and category-theoretic approaches to music.

**The Notation DSL.** Fermata itself: a Lisp dialect that compiles to MusicXML (and, via the Verovio bindings in verovioxide, to rendered notation). Phases 1–5 have established the typed IR, emitter, S-expression format, and user-facing syntax. Phase 5 is currently in its final milestones.

**The Interactive Runtime.** The subject of this document. Phase 6 introduces the REPL, session management, the notebook abstraction, and the MCP bridge that allows Claude to observe, query, and interact with the user’s live REPL sessions. Phases 7 and 8 extend this with MIDI support, closing the loop between notation, sound, and analysis.

When complete, the workflow looks like this: a user opens a Fermata REPL session, enters musical notation, and hears it played back through MIDI. Claude, connected via the MCP server, observes the session’s state in real time—the score, the command history, the notebook. The user and Claude exchange messages within the REPL itself or through the Claude Desktop interface. Claude can suggest harmonic progressions, identify voice-leading issues, perform set-class analysis, or simply play back a transposition of what the user just wrote. Every interaction—code, conversation, analysis—is captured in a unified timeline and can be curated into a structured notebook for later reference.

This is not a DAW, not a notation editor, and not a chatbot. It is a *collaborative musical thinking environment* where precise notation, deep theoretical knowledge, and real-time interaction converge.

### **1.2  Core Concepts**

#### **1.2.1  The Session**

A session is the fundamental unit of work. It encapsulates everything related to a particular musical activity: a compositional sketch, an analytical exercise, a theory lesson, or an extended piece of work on a full score. A session contains four primary components: a score, an environment, a history, and a notebook.

Sessions are named and identified by stable IDs. They can be created, listed, switched between, saved, and restored. At any given time, exactly one session is *active* in the REPL—the one receiving input and displaying output. Other sessions persist in memory or on disk and can be switched to at any time.

#### **1.2.2  The Score**

The score is the session’s musical artifact: a typed Fermata data structure (ultimately a FermataScore) that represents the accumulated, current-state notation. It may be anything from a two-measure melodic fragment to a multi-movement orchestral work. It is the *product* of the session’s work.

The score is always valid: every modification (adding a note, inserting a measure, changing a key signature) is applied through the evaluator, which validates before committing. The score can be emitted as MusicXML at any time, rendered to notation via Verovio, or played back through MIDI.

#### **1.2.3  The Environment**

The environment is the evaluation context for the session, analogous to the ENV in other Lisps. It holds symbol bindings (user-defined variables, named musical fragments, macros), the current implicit state (default part, default voice, current key/time/clef for shorthand entry), and any configuration specific to that session.

This separation matters because the score is the *result* of evaluation, while the environment is the *context* in which evaluation occurs. You might define a motif in the environment and use it in multiple places in the score, or change the default key in the environment to affect subsequent note entry without altering what’s already in the score.

#### **1.2.4  The History**

The history is a unified, ordered timeline of *everything that happened in the REPL*. It is not merely a command log. Every entry—whether a Fermata expression evaluated by the user, a REPL meta-command, a message from Claude, a conversational reply from the user, or a system event—receives a monotonically increasing index number and a timestamp. The history records events in the exact order the user experienced them.

This design is inspired by the numbered histories of the Erlang shell and the Mathematica REPL, but extended to encompass the full range of interactions that occur in a collaborative environment. When Claude queries “what happened between entries 200 and 250,” the response includes not just the code but the surrounding conversational context that motivated it.

History entries are typed and filterable. The MCP server and REPL commands can request subsets: only evaluations, only messages, only errors, or any combination. This allows both precise queries (“show me just the code”) and full-context queries (“show me everything, including the conversation”).

#### **1.2.5  The Notebook**

The notebook is the session’s analytical narrative: a structured, tree-organized document that captures the *understanding* developed through the session’s work. Where the history is an exhaustive, chronological log, the notebook is curated, organized, and intentional.

A notebook contains sections and subsections, each holding an ordered sequence of content blocks. Blocks can be prose, references to score regions, inline musical examples (Fermata snippets that aren’t part of the main score), references to history ranges, annotations, or summaries of Claude Desktop conversations. The notebook is writable from both the REPL (via commands) and from Claude (via MCP tools), and readable from both.

The notebook is stored in memory as a typed tree data structure, not as a text file. This makes programmatic operations (appending to a section, moving sections, searching, updating references) clean and atomic. It is rendered to Markdown only on explicit export—for sharing, for external editing, or for archival.

The notebook bridges the two interfaces the user works with: the REPL (where musical work happens) and Claude Desktop (where analytical discussion happens). Both threads of activity converge in the notebook as a persistent, structured record.

#### **1.2.6  The Message Queue**

The REPL maintains a message queue for incoming communications from Claude (via the MCP server). Messages are not displayed immediately upon arrival; they are rendered at the top of the next REPL loop iteration, after the user presses Enter. This ensures that messages never interrupt the user mid-input and that the display order matches the user’s experienced order.

When messages are rendered, they are visually distinguished from REPL output (by prefix, color, or indentation) and are committed to the history with their own index numbers, preserving the unified timeline.

### **1.3  Interaction Model**

#### **1.3.1  REPL Input Modes**

The REPL must distinguish three types of user input:

**Fermata expressions** are the default mode. Any input that is not prefixed with a command or chat sigil is parsed and evaluated as Fermata syntax. Successful evaluation updates the score and/or environment and displays the result.

**REPL commands** are prefixed with a colon (e.g., :session list, :notebook show 2.4, :history 40:50, :midi devices). These are meta-operations that interact with the REPL’s own state, session management, notebook, and configuration.

**Chat messages** are conversational text directed at Claude. The recommended sigil is // (double slash), which reads naturally as a “comment” directed at the AI. When the user types // followed by text, the input is recorded as a UserMessage in the history and published to the MCP server for Claude to see and respond to.

#### **1.3.2  The REPL Loop**

The REPL’s main loop is the single-threaded serialization point for all state mutations. The canonical loop is:

(1) Drain the incoming message queue: display any pending messages from Claude, assign index numbers, and commit them to history. (2) Display the prompt, including the current session name and the next index number. (3) Read user input. (4) Classify the input (expression, command, or chat message). (5) Process: evaluate the expression, execute the command, or publish the chat message. (6) Display the result. (7) Commit the entry to history with its index number. (8) Loop.

Because all history writes happen on this single thread, ordering is deterministic and matches the user’s experience exactly.

#### **1.3.3  Claude’s View**

From Claude’s perspective (via the MCP server), the user’s REPL is an observable, queryable, and addressable system. Claude can:

•  List all sessions and their metadata.

•  Read the current score of any session (as Fermata syntax, as the typed IR summary, or as MusicXML).

•  Query the history with index ranges, time ranges, and type filters.

•  Read the notebook outline and section contents.

•  Append content to notebook sections.

•  Send messages to the user (rendered at the next REPL prompt).

•  Evaluate Fermata expressions in a session (with appropriate safeguards).

•  Query and control MIDI devices (Phases 7–8).

#### **1.3.4  Bidirectional MIDI (Phases 7–8)**

In Phase 7, the REPL gains the ability to list MIDI devices, open connections, and send MIDI messages. In Phase 8, this becomes fully bidirectional: the user can play back score content through MIDI, and can record incoming MIDI (from a keyboard or controller) into a session’s score.

Claude participates in both directions. Claude can instruct the REPL (via MCP) to play a passage, a chord, or a specific MIDI message. Conversely, when the user records MIDI input, the captured data is converted to Fermata IR and becomes part of the session’s score and history, visible to Claude for analysis.

The full closed loop: the user plays a chord on a MIDI keyboard; the REPL captures it, quantizes it, and converts it to Fermata IR; the MCP server reports it to Claude; Claude analyzes it (“That’s a Neapolitan sixth resolving to V”) and sends a message back; the user sees Claude’s analysis at their next prompt.

### **1.4  Capability Map**

The following table lists all capabilities discussed in this document, ordered by implementation dependency. Features higher in the table must be built before features that depend on them. The “Phase” column indicates the implementation phase; the “Dependencies” column lists capabilities that must be in place first.

| \# | Capability | Phase | Dependencies | Description |
| :---- | :---- | :---- | :---- | :---- |
| C1 | Core REPL loop | 6a | Phase 5 (compiler) | Read–eval–print loop with Fermata evaluation, prompt, and result display |
| C2 | Session data model | 6a | C1 | Session struct with score, environment, history, notebook, message queue |
| C3 | Session management | 6a | C2 | Create, list, switch, save, and restore sessions; active session tracking |
| C4 | Numbered history | 6a | C2 | Monotonic indexing of all REPL events; typed EntryKind enum |
| C5 | REPL commands | 6a | C1, C4 | Colon-prefixed meta-commands (:session, :history, :help, etc.) |
| C6 | History filtering | 6b | C4 | Query history by index range, time range, and entry type |
| C7 | Chat input mode | 6b | C4 | // prefix for conversational messages; UserMessage entry type |
| C8 | Message queue | 6b | C2 | Incoming message buffer; drain-and-display at top of REPL loop |
| C9 | Unified timeline | 6b | C4, C7, C8 | Eval, Command, AiMessage, UserMessage, and System entries interleaved in order |
| C10 | IPC socket server | 6c | C2 | Unix domain socket on REPL process; line-delimited JSON protocol |
| C11 | MCP–REPL bridge | 6c | C10 | MCP server connects to REPL socket; translates MCP tool calls to REPL queries |
| C12 | Session query tools | 6c | C3, C11 | MCP tools: list\_sessions, get\_session, get\_active\_session |
| C13 | Score query tools | 6c | C2, C11 | MCP tools: get\_score (as Fermata, as MusicXML), get\_measures |
| C14 | History query tools | 6c | C6, C11 | MCP tools: get\_history with index/time/type filters |
| C15 | Message send tool | 6c | C8, C11 | MCP tool: send\_message → enqueues in REPL’s message queue |
| C16 | Remote eval tool | 6c | C1, C11 | MCP tool: eval\_in\_session → evaluates Fermata expr in a session |
| C17 | Notebook data model | 6d | C2 | Tree-structured notebook: sections, blocks (Prose, ScoreRef, InlineMusic, HistoryRef, Annotation, ChatSummary) |
| C18 | Notebook REPL commands | 6d | C5, C17 | :notebook show, :notebook append, :notebook capture, :notebook outline |
| C19 | Notebook MCP tools | 6d | C11, C17 | MCP tools: notebook\_get\_outline, notebook\_get\_section, notebook\_append, notebook\_search |
| C20 | Session persistence | 6e | C2, C17 | Save/load sessions to disk: score.ferm, env.json, history.jsonl, notebook.json |
| C21 | History references | 6e | C4, C9 | In\[n\] and Out\[n\] referenceable in subsequent expressions |
| C22 | MIDI device enumeration | 7a | C1 | List available system MIDI devices; display in REPL |
| C23 | MIDI device connection | 7a | C22 | Open/close MIDI device connections; connection state per session |
| C24 | MIDI output (send) | 7b | C23 | Send MIDI messages (note on/off, CC, program change) to open device |
| C25 | Score–to–MIDI conversion | 7b | C2, C24 | Convert Fermata IR to MIDI events; handle tempo, dynamics, articulation |
| C26 | MIDI playback command | 7b | C5, C25 | :play command in REPL; play score, measures, or selection |
| C27 | MIDI MCP tools (output) | 7b | C11, C24, C25 | MCP tools: list\_midi\_devices, play\_score, play\_midi\_message |
| C28 | MIDI input (receive) | 8a | C23 | Receive MIDI messages from open device; buffer incoming events |
| C29 | MIDI–to–IR conversion | 8a | C28 | Quantize and convert incoming MIDI to Fermata IR; configurable quantization |
| C30 | MIDI recording | 8b | C28, C29, C4 | Record MIDI input into session history and score; :record command |
| C31 | MIDI MCP tools (input) | 8b | C11, C28, C29 | MCP tools: get\_midi\_input, get\_last\_recording; Claude analyzes what user played |
| C32 | Full MIDI loop | 8b | C27, C31 | Bidirectional MIDI: user plays → Claude sees; Claude sends → user hears |

*Sub-phases (6a, 6b, etc.) are suggestive groupings for implementation ordering, not rigid boundaries. Some capabilities may be implemented concurrently.*

## **Part II: Technical Design**

### **2.1  Session Data Model**

A session is the top-level container for all state related to a unit of work. In Rust, the core structure is:

struct Session {
    id: SessionId,                          // UUID or short stable ID
    metadata: SessionMetadata,              // name, created, modified, tags
    score: FermataScore,                    // the musical artifact
    env: Environment,                       // bindings, macros, defaults
    history: Vec\<HistoryEntry\>,             // unified timeline
    next\_index: u64,                        // next history index to assign
    notebook: Notebook,                     // structured narrative
    message\_queue: VecDeque\<IncomingMessage\>, // pending messages from Claude
    midi\_state: Option\<MidiSessionState\>,   // Phase 7+
}

**SessionId** should be a short, human-friendly identifier (e.g., session-1, session-2, or a user-supplied slug like bach-chorale) rather than a UUID. UUIDs are useful for storage but poor for REPL interaction. A mapping from slug to UUID (or to a monotonic integer) handles uniqueness internally.

**SessionMetadata** holds the display name, creation and last-modified timestamps, optional tags (for organization), and any score-level metadata the user has set (title, composer, key — mapped from MusicXML’s Work and Identification elements).

**Environment** holds symbol bindings (variable names to Fermata values), macro definitions, and the implicit state: the current default part, voice, key signature, time signature, clef, and tempo. These defaults govern how shorthand expressions are interpreted. For example, if the current key is D major, a bare (note f4 :q) can be inferred as F♯4 without the user specifying the accidental.

#### **2.1.1  Session Manager**

The SessionManager owns all sessions and tracks which one is active:

struct SessionManager {
    sessions: HashMap\<SessionId, Session\>,
    active: Option\<SessionId\>,
    next\_session\_num: u64,
}

Operations: create(name) → SessionId, switch(id), list() → Vec\<SessionSummary\>, save(id), load(id), delete(id). On REPL startup, the SessionManager either loads persisted sessions or creates a fresh default session.

### **2.2  Unified History**

#### **2.2.1  Entry Structure**

Every event in the REPL receives a HistoryEntry:

struct HistoryEntry {
    index: u64,                // monotonic, gapless
    timestamp: DateTime\<Utc\>,  // wall-clock time
    kind: EntryKind,           // what happened
}

enum EntryKind {
    Eval {
        input: String,
        parsed: Option\<SExpr\>,
        result: Option\<String\>,
        error: Option\<String\>,
        score\_delta: Option\<ScoreDelta\>,
    },
    Command {
        raw: String,
        output: Option\<String\>,
    },
    AiMessage {
        content: String,
        source: MessageSource,
    },
    UserMessage {
        content: String,
    },
    System {
        event: SystemEvent,
    },
}

**Eval** entries capture the full lifecycle of a Fermata expression: the raw input, the parsed S-expression (if parsing succeeded), the display representation of the result, any error, and an optional ScoreDelta describing what changed in the score. The ScoreDelta is valuable for history queries: “show me all entries that modified the bass line” becomes a filter on score\_delta rather than re-evaluating each entry.

**Command** entries record REPL meta-commands and their output. These are useful for understanding the user’s workflow (when they switched sessions, checked the notebook, etc.) but are often filtered out in code-focused queries.

**AiMessage** entries are messages from Claude that were displayed to the user. The source field records whether the message came from a direct send\_message MCP call, from an eval\_in\_session result, or from a MIDI analysis.

**UserMessage** entries are conversational messages from the user directed at Claude, entered via the // prefix (or whatever chat sigil is chosen).

**System** entries record significant events: session creation/switch, MIDI device connection/disconnection, errors, and other state changes that aren’t directly triggered by user input.

#### **2.2.2  Ordering Guarantee**

The ordering guarantee is critical: the history must reflect the order the user experienced events, not the order they were generated internally. The REPL’s main loop is the serialization point. The flow within each loop iteration is:

First, drain the message queue. All pending messages from Claude are dequeued, displayed, and committed to history in the order they arrived. Each gets the next available index. Second, the user’s input is processed (eval, command, or chat) and committed with the next index. This means that if Claude sent two messages while the user was typing, those messages will appear in history (and on screen) before the user’s input, which matches the user’s experience: they hit Enter, saw the messages, then saw their own result.

#### **2.2.3  Concurrency Model**

The REPL’s main loop runs on a single thread and exclusively owns the history Vec. No locks are needed on the history itself. The concurrency boundary is the message channel:

MCP Server thread(s)
    │
    │  mpsc::Sender\<IncomingMessage\>
    ▼
┌─────────────┐
│  Channel    │  (bounded, provides backpressure)
└─────┬───────┘
      │  mpsc::Receiver\<IncomingMessage\>
      ▼
┌──────────────┐
│ REPL Loop    │  single-threaded serialization point
│              │
│ history: Vec │  exclusively owned
│ next\_idx: u64│
└──────────────┘

For MCP read queries (Claude asks for history), there are two viable approaches. The simpler approach is to route all queries through the same channel: the MCP handler sends a query request, the REPL loop processes it on its thread, and sends the response back via a oneshot channel. This adds a small amount of latency but avoids all concurrency complexity. The alternative is to share the history behind an Arc\<RwLock\<Vec\<HistoryEntry\>\>\>, allowing the MCP handler to take read locks directly. Given the data sizes involved (serializing a few hundred entries is microseconds), the single-threaded approach is recommended as the starting point.

#### **2.2.4  History Filtering**

The filtering API supports composition of constraints:

struct HistoryQuery {
    from\_index: Option\<u64\>,
    to\_index: Option\<u64\>,
    since: Option\<DateTime\<Utc\>\>,
    until: Option\<DateTime\<Utc\>\>,
    kinds: Vec\<EntryKindFilter\>,  // empty \= all
    text\_search: Option\<String\>,  // search within input/content
    limit: Option\<usize\>,
}

enum EntryKindFilter {
    Eval,
    Command,
    AiMessage,
    UserMessage,
    System,
}

On the REPL side, this maps to command syntax: :history 40:50 shows entries 40–50 (all types); :history 40:50 \--code shows only Eval entries; :history 40:50 \--chat shows AiMessage and UserMessage entries; :history \--since 10m shows the last 10 minutes.

On the MCP side, the same structure is exposed as a get\_history tool with JSON parameters.

#### **2.2.5  History References in Expressions**

Following the precedent of Mathematica’s In\[n\]/Out\[n\] and Erlang’s shell bindings, history results should be referenceable in subsequent expressions. If the user evaluated (note c4 :q) at index 47, they should be able to write (transpose \+2 $Out\[47\]) to transpose that result up a whole step.

Implementation: the evaluator checks for $In\[n\] and $Out\[n\] forms during evaluation, looks up the corresponding history entry, and substitutes the parsed expression (for In) or the result value (for Out). This requires that Eval entries store their result not just as a display string but also as a retrievable Fermata value. The stored parsed and result fields serve this purpose.

**Consideration:** History references create an implicit dependency between entries. If the user modifies or replays earlier entries, downstream references could become stale. The simplest policy is that history is append-only and references are to the immutable record of what happened. This matches Mathematica’s behavior and avoids complex dependency tracking.

### **2.3  Notebook Architecture**

#### **2.3.1  Data Model**

The notebook is stored in memory as a typed tree, never as raw text:

struct Notebook {
    metadata: NotebookMetadata,  // title, created, modified
    root: Vec\<Section\>,          // top-level sections
}

struct Section {
    id: SectionId,               // stable, survives reordering
    title: String,
    content: Vec\<Block\>,
    children: Vec\<Section\>,
}

enum Block {
    Prose(String),
    ScoreRef {
        session\_id: SessionId,
        part: Option\<PartSelector\>,
        measures: MeasureRange,
        snapshot: Option\<SnapshotId\>,
    },
    InlineMusic {
        source: String,
        compiled: Option\<FermataScore\>,
    },
    HistoryRef {
        range: RangeInclusive\<u64\>,
        caption: Option\<String\>,
    },
    Annotation {
        kind: AnnotationKind,
        text: String,
        anchor: Option\<ScoreRef\>,
    },
    ChatSummary {
        source: String,
        timestamp: DateTime\<Utc\>,
        content: String,
    },
}

**SectionId** is a stable identifier (a UUID or a monotonic integer) that does not change when sections are reordered, inserted, or deleted. The display numbering (1, 1.1, 1.2, 2, 2.1, etc.) is computed from position in the tree and is *not stored*. This means that when a section is moved or a new section is inserted, all display numbers update automatically, but references by SectionId remain valid.

**Block variants** cover the primary content types:

*Prose* is free-form text. It can be written by the user (via REPL commands) or by Claude (via MCP tools). It is the primary narrative element.

*ScoreRef* points to a region of the session’s score: a range of measures, optionally filtered to a specific part or voice. If snapshot is None, the reference is live (always reflects the current score state). If snapshot is Some, it captures the score state at the time the reference was created, which is useful for “here’s what it looked like when we analyzed it.”

*InlineMusic* is a standalone Fermata snippet that is not part of the session’s score. It’s used for illustrative examples: “here’s what a Neapolitan sixth looks like in root position.” The compiled field caches the compilation result.

*HistoryRef* pulls a range of history entries into the notebook. When rendered, it shows the full interleaved context: code, results, messages, and conversation. The caption provides editorial context.

*Annotation* is a tagged observation attached optionally to a score region. The kind field (harmonic, voice-leading, form, rhythm, etc.) enables filtering: “show me all harmonic annotations in the notebook.”

*ChatSummary* is a digest of a Claude Desktop conversation (or a REPL chat exchange) that has been condensed and inserted into the notebook for continuity.

#### **2.3.2  Operations**

All notebook operations are tree manipulations on the in-memory structure:

**Append:** Find the target section (by SectionId or by display path like “2.4”), push a Block onto its content Vec. O(depth) for navigation, O(1) for the append.

**Insert:** Same navigation, insert at a specific position within the content Vec.

**Move section:** Detach from parent’s children Vec, attach to new parent. All display numbering recomputes automatically.

**Search:** Walk the tree, match against block contents. Returns (SectionId, block index) pairs.

**Get outline:** Walk the tree, collect (display\_number, title, block\_count, SectionId) tuples.

None of these operations require text parsing. They are all typed, in-memory, and fast.

#### **2.3.3  Serialization Strategy**

Three output representations serve different purposes:

**Native serialization** (the save file): written to \~/.fermata/sessions/\<id\>/notebook.json (or .cbor for binary efficiency). This is a lossless, round-trippable encoding of the Notebook struct. JSON is recommended initially for debuggability; a binary format can be substituted later without changing the in-memory model.

**Markdown export** (the share format): produced on demand by :notebook export or by an MCP tool. Walks the tree and emits Markdown with headings for sections, fenced code blocks for InlineMusic, blockquotes for Annotations, and so on. This is lossy (SectionIds, SnapshotIds, and typed Block variants are not preserved) but human-readable and useful for external sharing.

**REPL display** (the terminal format): what :notebook show 2.4 prints. Uses ANSI colors, indentation, and abbreviated representations. Optimized for quick reference, not archival.

#### **2.3.4  Notebook Capture Command**

The :notebook capture command bridges history and notebook:

:notebook capture 2.4 In\[44\]:In\[47\] "Chromatic approach to F4"

This creates a HistoryRef block in section 2.4, spanning entries 44–47, with the given caption. When the notebook is viewed or exported, the referenced entries are rendered inline—showing the full interleaved context of code, results, and conversation. This documents not just what was built, but how and why.

### **2.4  IPC Architecture & MCP Bridge**

#### **2.4.1  Communication Model**

The REPL process and MCP server communicate via a Unix domain socket. The REPL is the server (it opens the socket on startup at a well-known path like \~/.fermata/repl.sock). The MCP server is a client (it connects when it needs to interact with the REPL).

The protocol is line-delimited JSON: each message is a single JSON object terminated by a newline. Request–response pairs are correlated by a request\_id field. This is deliberately simple—no HTTP overhead, no framing complexity, no external dependencies. The choice of Unix domain socket over TCP is intentional: for the primary use case (REPL and MCP server on the same machine), Unix sockets are faster, have no network configuration, and don’t require port management.

If a future use case requires remote access (REPL on one machine, Claude on another), the protocol is transport-agnostic. Switching from Unix socket to TCP is a one-line change at the listener level; the JSON protocol remains identical.

#### **2.4.2  Protocol Shape**

Requests and responses follow a minimal JSON-RPC-like structure:

// Request
{"id": "req-1", "method": "list\_sessions", "params": {}}

// Response (success)
{"id": "req-1", "result": {"sessions": \[...\]}}

// Response (error)
{"id": "req-1", "error": {"code": "not\_found", "message": "No such session"}}

The method names map 1:1 to the MCP tool names. The MCP server is a thin translation layer: it receives an MCP tool call, formats it as a JSON request, sends it over the socket, waits for the response, and returns the result to the MCP framework.

#### **2.4.3  MCP Tool Surface**

The following tools would be exposed through the MCP server. Each corresponds to a method on the REPL’s IPC interface:

| Tool | Phase | Description |
| :---- | :---- | :---- |
| repl\_list\_sessions | 6c | List all sessions with metadata (id, name, created, modified, measure count, active flag) |
| repl\_get\_session | 6c | Get full session detail: metadata, score summary, environment summary, history length |
| repl\_get\_active\_session | 6c | Shortcut for the currently active session’s detail |
| repl\_get\_score | 6c | Get session’s score as Fermata syntax, MusicXML, or IR summary; optional part/measure filters |
| repl\_get\_measures | 6c | Get a specific measure range from a session’s score |
| repl\_get\_history | 6c | Query history with index range, time range, type filters, text search, and limit |
| repl\_send\_message | 6c | Send a message to the REPL’s message queue; displayed at next prompt |
| repl\_eval | 6c | Evaluate a Fermata expression in a session; returns result or error |
| repl\_notebook\_outline | 6d | Get the notebook’s section tree: display numbers, titles, block counts, section IDs |
| repl\_notebook\_get\_section | 6d | Get a section’s full content: all blocks with their types and data |
| repl\_notebook\_append | 6d | Append a block (prose, annotation, chat summary, etc.) to a notebook section |
| repl\_notebook\_create\_section | 6d | Create a new section under a given parent with a title |
| repl\_notebook\_search | 6d | Search notebook content; returns matching section paths and block excerpts |
| repl\_midi\_devices | 7a | List available system MIDI devices with connection status |
| repl\_midi\_play | 7b | Play a score region, a Fermata expression, or raw MIDI events through a connected device |
| repl\_midi\_send | 7b | Send a specific MIDI message (note, CC, program change) to a device |
| repl\_midi\_get\_recording | 8b | Get the most recent MIDI recording as Fermata IR |
| repl\_midi\_get\_input\_buffer | 8a | Get raw MIDI input events since last read |

#### **2.4.4  Security Considerations**

The repl\_eval tool deserves particular attention. Allowing Claude to evaluate arbitrary expressions in the user’s session is powerful but must be carefully bounded. Recommended safeguards:

*Read-only by default:* the MCP tool could distinguish between eval (which can modify the score) and eval\_preview (which evaluates but does not commit changes). Claude would use eval\_preview for analysis and exploration, and eval only with explicit user confirmation.

*Audit trail:* every remote eval is recorded in history as an Eval entry with a source field indicating it came from the MCP server, not from the user’s keyboard. The user always sees what was evaluated on their behalf.

*Scope limits:* remote eval should not be able to execute REPL commands (the : prefix), send messages as the user, or access the filesystem. It should only be able to evaluate Fermata expressions within the musical domain.

### **2.5  Session Persistence**

Sessions are saved to and loaded from disk at \~/.fermata/sessions/\<id\>/. Each session directory contains:

\~/.fermata/sessions/bach-chorale/
├── meta.json        \# SessionMetadata (name, timestamps, tags)
├── score.ferm       \# Score as Fermata syntax (human-readable, round-trippable)
├── env.json         \# Environment bindings and implicit state
├── history.jsonl    \# History entries, one JSON object per line (append-friendly)
└── notebook.json    \# Full notebook tree structure

**score.ferm** uses Fermata syntax rather than MusicXML or a binary format. This ensures that saved sessions are human-readable, editable in any text editor, and version-controllable with Git. The pretty-printer (from Phase 4\) ensures consistent formatting.

**history.jsonl** uses JSON Lines format (one JSON object per line). This is append-friendly: the REPL can write new entries by appending lines to the file without rewriting the entire history. It is also streamable: loading history can be done lazily or filtered during deserialization. For large sessions with thousands of entries, this avoids loading the entire history into memory at startup if not needed.

**Autosave:** The REPL should periodically autosave the active session (e.g., every 60 seconds or every N history entries). The history.jsonl format makes this cheap—only new entries since the last save need to be appended. The score and environment may need a full rewrite, but these are small.

**Session recovery:** On startup, the REPL checks for unsaved state (e.g., a .lock file or a dirty flag in meta.json) and offers to recover the previous session, similar to how editors recover from crashes.

### **2.6  MIDI Architecture (Phases 7–8)**

#### **2.6.1  Phase 7: Foundation**

Phase 7 establishes the MIDI subsystem. The recommended Rust crate for cross-platform MIDI is *midir*, which provides enumeration, connection, and message I/O for MIDI devices on macOS (CoreMIDI), Linux (ALSA), and Windows (WinMM).

The MIDI subsystem is per-session: each session can have its own MIDI device connections and playback state. This allows one session to be connected to a piano sound module while another is connected to a synthesizer, or one session to have no MIDI at all.

**Device enumeration** lists available input and output ports with their names and system identifiers. The REPL command :midi devices and the MCP tool repl\_midi\_devices expose this information.

**Device connection** opens a MIDI output (and optionally input) port. The connection is stored in the session’s MidiSessionState. Opening a connection does not start recording or playback; it establishes the communication channel.

**Message sending** is the foundation of output. A MIDI message is a small byte sequence (1–3 bytes for channel messages). The REPL can send arbitrary MIDI messages for testing and verification.

#### **2.6.2  Score-to-MIDI Conversion**

Converting a Fermata score to MIDI events requires several mapping steps:

*Pitch mapping:* Fermata’s Pitch (step \+ octave \+ alter) maps to a MIDI note number (0–127). Middle C (C4) \= 60\. Each semitone is \+1. Accidentals map directly: C♯4 \= 61, B♭3 \= 58\.

*Duration mapping:* Fermata’s rational durations map to MIDI ticks at a given tempo and ticks-per-quarter-note (TPQN) resolution. A quarter note at TPQN=480 \= 480 ticks. A dotted half at the same resolution \= 1440 ticks.

*Dynamics mapping:* Fermata’s dynamics (pp through ff) map to MIDI velocity values. A common mapping: pp=32, p=48, mp=64, mf=80, f=96, ff=112. Crescendo/diminuendo hairpins require interpolation across note sequences.

*Articulation mapping:* Staccato shortens the sounding duration (e.g., 50% of notated duration). Legato extends it. Accents increase velocity. These are standard MIDI performance conventions.

#### **2.6.3  Phase 8: Bidirectional MIDI**

Phase 8 adds MIDI input: receiving, quantizing, and converting MIDI messages from external devices into Fermata IR.

**MIDI input listener:** When a session has an open MIDI input port, a background thread listens for incoming messages and buffers them with timestamps. The listener runs independently of the REPL loop, since MIDI messages can arrive at any time.

**Quantization:** Raw MIDI input has microsecond timing. To convert this into notated rhythm, a quantizer snaps note onsets and durations to the nearest musical grid (e.g., sixteenth notes). Quantization is configurable: the grid resolution, the swing amount, and the tolerance for “lose” notes can all be adjusted. This is a well-studied problem with known algorithms.

**MIDI-to-IR conversion:** Once quantized, the note events are grouped into chords (simultaneous onsets), assigned durations based on quantized lengths, and converted to Fermata IR (Note, Chord, Rest values). Pitch is the reverse of the output mapping: MIDI note 60 \= C4, 61 \= C♯4/D♭4 (enharmonic spelling requires context—the current key signature disambiguates).

**Recording workflow:** The user enters :record mode, plays on their MIDI controller, and exits with :stop. The captured events are quantized, converted to IR, appended to the score, and committed to history. The user can review and edit the result before it becomes permanent.

**The full loop:** User plays → REPL captures MIDI → quantizes and converts to IR → MCP server reports new content → Claude analyzes (“That’s a Neapolitan sixth\!”) → Claude sends message via MCP → REPL displays analysis at next prompt. This is the complete bidirectional musical collaboration cycle.

## **Part III: Project Plan & Open Questions**

### **3.1  Proposed Implementation Sequence**

The work is organized into three phases, each with internal sub-phases. Sub-phases represent logical groupings, not rigid stage gates—features within a sub-phase can be developed concurrently, and integration across sub-phases is expected to be continuous.

#### **Phase 6: REPL & Collaborative Infrastructure**

This is the largest phase and the core of this proposal. It establishes the interactive runtime that all subsequent work builds on.

**6a — Core REPL (estimated 2–3 milestones).** The REPL loop, Fermata evaluation integration, session data model, session manager, numbered history (Eval and Command entry types only), and basic REPL commands (:session, :history, :help). At the end of 6a, the user can open the REPL, create sessions, enter Fermata expressions, see results, and browse history. This is a usable (if minimal) interactive Fermata environment.

**6b — Communication Layer (estimated 1–2 milestones).** Chat input mode (// prefix), the message queue, UserMessage and AiMessage entry types, the unified timeline. At the end of 6b, all five entry types are in the history, and the interleaving order matches the user’s experience. The REPL can display messages from Claude, and the user can send messages to Claude (even though the MCP bridge isn’t built yet—messages are queued locally for testing).

**6c — IPC & MCP Bridge (estimated 2–3 milestones).** The Unix domain socket server in the REPL, the line-delimited JSON protocol, the MCP server’s client connection, and the session/score/history/message MCP tools. At the end of 6c, Claude can observe and interact with the REPL through the MCP server: list sessions, read scores, query history, send messages, and evaluate expressions.

**6d — Notebook (estimated 2 milestones).** The notebook data model, REPL commands (:notebook show, :notebook append, :notebook capture, :notebook outline), and MCP tools (notebook\_outline, notebook\_get\_section, notebook\_append, notebook\_create\_section, notebook\_search). At the end of 6d, the full collaborative infrastructure is in place.

**6e — Persistence & Polish (estimated 1–2 milestones).** Session save/load to disk, autosave, session recovery, history references ($In\[n\]/$Out\[n\]), and UX refinements (ANSI coloring, tab completion, input editing).

#### **Phase 7: MIDI Foundation**

**7a — Device Layer (estimated 1 milestone).** MIDI device enumeration, connection management, MidiSessionState, REPL commands (:midi devices, :midi connect, :midi disconnect), and the repl\_midi\_devices MCP tool.

**7b — MIDI Output (estimated 1–2 milestones).** Message sending, score-to-MIDI conversion (pitch, duration, dynamics, articulation mapping), the :play REPL command, and MCP tools (repl\_midi\_play, repl\_midi\_send).

#### **Phase 8: Bidirectional MIDI**

**8a — MIDI Input (estimated 1–2 milestones).** MIDI input listener (background thread), input event buffering, quantization engine, MIDI-to-IR conversion, enharmonic spelling, and the repl\_midi\_get\_input\_buffer MCP tool.

**8b — Recording & Full Loop (estimated 1–2 milestones).** The :record/:stop workflow, recording-to-score integration, repl\_midi\_get\_recording MCP tool, and the complete bidirectional MIDI–Claude analysis loop.

### **3.2  Estimated Scope**

Phase 6 is the largest effort, with approximately 8–12 milestones spanning the REPL core, communication, IPC, notebook, and persistence. Phases 7 and 8 are smaller—approximately 2–3 milestones each—because they build on the infrastructure established in Phase 6\.

Each milestone follows the pattern established in Phases 4 and 5: a detailed planning document, implementation by Claude Code, and QA review. The milestone planning documents should be produced in the same style as the Phase 5 milestones (0021–0025), with explicit deliverables, test criteria, and integration checkpoints.

### **3.3  Open Questions**

#### **3.3.1  Technical & Architectural**

| \# | Question | Discussion |
| :---- | :---- | :---- |
| Q1 | Chat message sigil | The proposal uses // for conversational messages. Alternatives include \> (markdown quote feel), @ (addressing someone), or a mode toggle (:chat / :code). The // approach is simple and unambiguous but may conflict with single-line comment conventions if Fermata ever adopts them. The choice should feel natural in a terminal context. |
| Q2 | IPC: single-threaded vs. RwLock for reads | The proposal recommends routing all MCP queries through the REPL’s main thread (via channel) for simplicity. If latency becomes a concern, an Arc\<RwLock\<Vec\<HistoryEntry\>\>\> allows direct read access from the MCP handler thread. The decision can be deferred until performance profiling under realistic workloads. |
| Q3 | Score snapshots in notebook references | ScoreRef blocks can be live (always reflecting current score) or snapshot-based (capturing state at reference time). The implementation of snapshots requires either copy-on-write semantics or a version history for the score. This could be deferred: start with live references only, add snapshots when the use case demands it. |
| Q4 | Notebook scope: one per session or multiple? | The proposal assumes one notebook per session. Multiple notebooks per session adds organizational power but also complexity. A pragmatic middle ground: one notebook per session with the ability to link to other sessions’ notebooks via cross-references. |
| Q5 | Environment serialization format | The Environment contains Fermata values (AST nodes, closures, macro definitions). Closures are not trivially serializable. Options: serialize only named bindings (not closures), use a restricted environment model where all bindings are data, or accept that some environment state is lost on save/load. This needs resolution before Phase 6e. |
| Q6 | History size limits | For long-running sessions with thousands of entries, should history be bounded (ring buffer) or unbounded? The JSONL format supports lazy loading, so the in-memory representation could hold only recent entries with older entries on disk. A reasonable default: keep all entries in memory up to some threshold (e.g., 10,000), then page to disk. |
| Q7 | MIDI quantization algorithm | Multiple approaches exist: nearest-grid-point, score-following (using existing notation as a template), or ML-based. The nearest-grid-point approach is simplest and should be the starting point, with more sophisticated algorithms added later if needed. |
| Q8 | Enharmonic spelling strategy | When converting MIDI note number 61 to Fermata IR, is it C♯4 or D♭4? The current key signature is the primary disambiguator, but chromatic passages and modulations require heuristics (prefer simpler intervals, minimize accidentals, respect common voice-leading patterns). This is a research question with well-known approaches in the music informatics literature. |
| Q9 | REPL line editing library | Rust options include rustyline (readline-like, mature), reedline (modern, used by Nushell), and crossterm-based custom solutions. The choice affects tab completion, multi-line input, history search (Ctrl-R), and ANSI rendering. Recommendation: rustyline for initial development due to maturity and simplicity. |
| Q10 | Multi-line input handling | Fermata expressions (especially scores) can span many lines. The REPL needs to detect incomplete expressions (unmatched parentheses) and continue reading on the next line, similar to how Python or Lisps handle multi-line input. This is a parser-level feature: the S-expression reader reports “incomplete” rather than “error” for unmatched delimiters. |

#### **3.3.2  Project & Process**

| \# | Question | Discussion |
| :---- | :---- | :---- |
| P1 | MCP server integration strategy | The music theory MCP server already exists with 18+ tools. Should the REPL bridge be added to that server (expanding its scope) or be a separate MCP server? A single server is simpler for the user (one MCP configuration). A separate server has cleaner separation of concerns. The servers could also be combined at the MCP framework level (multiple backends, one interface). |
| P2 | Testing strategy for IPC | The IPC bridge involves two processes communicating over a socket. Integration testing requires either spawning both processes in the test harness or mocking one side. The Rust testing ecosystem has good support for this (e.g., tokio::test for async, assert\_cmd for process testing), but the approach should be decided early to avoid test debt. |
| P3 | MIDI testing without hardware | Phases 7–8 require MIDI devices for full testing. Options: virtual MIDI ports (available on macOS via IAC, on Linux via snd-virmidi), software synthesizers (FluidSynth), or mock MIDI backends in the test suite. The development environment should support all three. |
| P4 | Milestone sizing for Phase 6 | Phase 6 is estimated at 8–12 milestones. Given the pattern from Phases 4–5 (where milestones averaged 800–1700 lines of planning documentation and multiple implementation sessions), Phase 6 is a substantial effort. The sub-phase boundaries (6a–6e) help manage scope, but the milestone breakdown within each sub-phase needs detailed planning. |
| P5 | Documentation strategy | Each previous phase produced planning documents and format references. Phase 6 introduces several new document types: the IPC protocol spec, the MCP tool reference, the REPL command reference, and the notebook format spec. These should be living documents updated as implementation progresses. |
| P6 | Crate structure | Should the REPL, IPC protocol, MCP bridge, and MIDI subsystem be separate crates in the fermata workspace, or modules within the existing fermata crate? Separate crates improve compile times and enforce API boundaries but add dependency management overhead. Recommendation: fermata-repl, fermata-ipc, and fermata-midi as separate crates; the MCP bridge lives in the existing MCP server crate. |

*This document is a living proposal.*

Decisions made during implementation will be reflected in updated versions.
