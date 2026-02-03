# Phase 6a: Core REPL — Milestone 2: Session Model & Manager

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Session struct, SessionManager, Environment, score accumulation, session REPL commands, prompt update
> **Depends On:** Milestone 6a-1 (REPL loop exists)

---

## Overview

This milestone introduces the **Session** as the fundamental unit of work. At the end of this milestone, a user can:

- Create named sessions (`:session new <name>`)
- List all sessions (`:session list`)
- Switch between sessions (`:session switch <id>`)
- View session details (`:session info`)
- Evaluate Fermata expressions that accumulate into the session's score
- See a prompt that reflects the active session name and next history index

Each session owns its own score, environment, and (placeholder) history. The session model established here is the scaffold that all subsequent milestones build on.

---

## Architecture

### Data Model

```
SessionManager
├── sessions: HashMap<SessionId, Session>
├── active: Option<SessionId>
└── next_session_num: u64

Session
├── id: SessionId
├── metadata: SessionMetadata
├── score: FermataScore           ← accumulated from evals
├── env: Environment              ← implicit state (key, time, clef, etc.)
├── next_index: u64               ← monotonic counter for history (used in prompt)
└── (history, notebook, messages: placeholders — filled in Milestones 3–4)
```

### Score Accumulation Model

**Decision: Append-only.**

When the user evaluates a Fermata expression that produces musical content, the result is merged into the session's score. The model is:

- If the expression is a complete `(score ...)`, it **replaces** the session's score entirely.
- If the expression is a `(part ...)`, it is **appended** as a new part to the current score.
- If the expression is a `(measure ...)`, it is **appended** to the environment's current part.
- If the expression is a note, rest, chord, etc., it is **appended** to the current measure of the current part.
- If no part or measure context exists, one is created implicitly.

This keeps things simple: the user builds up a score incrementally, and each expression adds to what's there.

### Environment Defaults

The environment tracks implicit state so that shorthand expressions work:

| Default | Type | Initial Value | Effect |
|---------|------|---------------|--------|
| `current_part` | `usize` (index) | 0 | Which part bare measures/notes are added to |
| `current_key` | `Option<KeySpec>` | None | Future: infer accidentals |
| `current_time` | `Option<TimeSpec>` | None | Future: measure validation |
| `current_clef` | `Option<ClefSpec>` | None | Future: display hints |
| `current_tempo` | `Option<u32>` | None | Future: MIDI playback |

These defaults are _read_ but not yet _acted upon_ in this milestone. The environment infrastructure is established; behavior driven by defaults (accidental inference, measure validation) is future work.

---

## Task 0: Add Dependencies

Update `crates/fermata-repl/Cargo.toml`:

```toml
[dependencies]
fermata = { path = "../fermata" }
rustyline = "14"
thiserror = "2"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
```

**Notes:**
- `chrono` for timestamps on SessionMetadata (`created`, `modified`).
- `uuid` for internal SessionId generation. The user-facing identifier is a slug; the UUID is for storage/cross-referencing.

---

## Task 1: Session Types (`src/session/mod.rs`)

Create `src/session/` module:

```rust
//! Session management: the fundamental unit of work in the Fermata REPL.
//!
//! A session encapsulates a score, an environment, a history, and a notebook.
//! The SessionManager owns all sessions and tracks which one is active.

pub mod env;
pub mod eval;
pub mod manager;
pub mod types;

pub use manager::SessionManager;
pub use types::{Session, SessionId, SessionMetadata};
```

### `src/session/types.rs`

```rust
//! Core session types.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::env::Environment;

/// A unique session identifier.
///
/// Internally a UUID; the user interacts via a human-friendly slug.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    /// Create a SessionId from a slug (user-supplied name or generated).
    pub fn new(slug: impl Into<String>) -> Self {
        Self(slug.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Session metadata.
#[derive(Debug, Clone)]
pub struct SessionMetadata {
    pub name: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub tags: Vec<String>,
    /// Internal UUID for storage.
    pub uuid: Uuid,
}

impl SessionMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            name: name.into(),
            created: now,
            modified: now,
            tags: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn touch(&mut self) {
        self.modified = Utc::now();
    }
}

/// A single REPL session.
///
/// Contains the score being built, the evaluation environment, and
/// (in future milestones) history, notebook, and message queue.
pub struct Session {
    pub id: SessionId,
    pub metadata: SessionMetadata,
    /// The accumulated musical score.
    pub score: fermata::lang::ast::FermataScore,
    /// Evaluation environment (bindings, defaults).
    pub env: Environment,
    /// Next monotonic history index. Used in the prompt and for
    /// assigning indices to history entries (Milestone 3).
    pub next_index: u64,
    // Future fields (Milestones 3+):
    // pub history: Vec<HistoryEntry>,
    // pub notebook: Notebook,
    // pub message_queue: VecDeque<IncomingMessage>,
}

impl Session {
    /// Create a new empty session.
    pub fn new(id: SessionId, name: impl Into<String>) -> Self {
        Self {
            id,
            metadata: SessionMetadata::new(name),
            score: fermata::lang::ast::FermataScore::default(),
            env: Environment::default(),
            next_index: 1,
        }
    }

    /// Advance the history index counter and return the current value.
    pub fn advance_index(&mut self) -> u64 {
        let idx = self.next_index;
        self.next_index += 1;
        idx
    }

    /// Summary for display in session list.
    pub fn summary(&self) -> SessionSummary {
        SessionSummary {
            id: self.id.clone(),
            name: self.metadata.name.clone(),
            created: self.metadata.created,
            modified: self.metadata.modified,
            part_count: self.score.parts.len(),
            measure_count: self
                .score
                .parts
                .first()
                .map(|p| p.measures.len())
                .unwrap_or(0),
            history_len: self.next_index.saturating_sub(1),
        }
    }
}

/// Brief session info for listing.
#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub id: SessionId,
    pub name: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub part_count: usize,
    pub measure_count: usize,
    pub history_len: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_creation() {
        let session = Session::new(SessionId::new("test"), "Test Session");
        assert_eq!(session.id.as_str(), "test");
        assert_eq!(session.metadata.name, "Test Session");
        assert_eq!(session.next_index, 1);
        assert!(session.score.parts.is_empty());
    }

    #[test]
    fn advance_index() {
        let mut session = Session::new(SessionId::new("test"), "Test");
        assert_eq!(session.advance_index(), 1);
        assert_eq!(session.advance_index(), 2);
        assert_eq!(session.advance_index(), 3);
        assert_eq!(session.next_index, 4);
    }

    #[test]
    fn session_summary() {
        let session = Session::new(SessionId::new("test"), "Test");
        let summary = session.summary();
        assert_eq!(summary.name, "Test");
        assert_eq!(summary.part_count, 0);
        assert_eq!(summary.history_len, 0);
    }
}
```

---

## Task 2: Environment (`src/session/env.rs`)

```rust
//! Evaluation environment: bindings and implicit state.
//!
//! The environment tracks the current defaults (key, time, clef, etc.)
//! that affect how shorthand expressions are interpreted. It also holds
//! user-defined bindings (variable → value).

use std::collections::HashMap;

/// The evaluation environment for a session.
#[derive(Debug, Clone)]
pub struct Environment {
    /// User-defined symbol bindings.
    ///
    /// Values are stored as their Fermata source text for now.
    /// A richer FermataValue type can replace this later.
    pub bindings: HashMap<String, String>,

    /// Index of the current default part (0-based).
    pub current_part: usize,

    /// Index of the current default measure within the current part.
    /// `None` means "append a new measure."
    pub current_measure: Option<usize>,

    /// Current default voice.
    pub current_voice: Option<String>,

    // Implicit musical state (read for context, not yet acted upon):
    /// Current key signature.
    pub current_key: Option<KeyState>,
    /// Current time signature.
    pub current_time: Option<TimeState>,
    /// Current clef.
    pub current_clef: Option<ClefState>,
    /// Current tempo (BPM).
    pub current_tempo: Option<u32>,
}

/// Simplified key state for environment tracking.
#[derive(Debug, Clone)]
pub struct KeyState {
    pub fifths: i8,
    pub mode: String,
}

/// Simplified time state for environment tracking.
#[derive(Debug, Clone)]
pub struct TimeState {
    pub beats: u32,
    pub beat_type: u32,
}

/// Simplified clef state for environment tracking.
#[derive(Debug, Clone)]
pub struct ClefState {
    pub sign: String,
    pub line: u32,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            bindings: HashMap::new(),
            current_part: 0,
            current_measure: None,
            current_voice: None,
            current_key: None,
            current_time: None,
            current_clef: None,
            current_tempo: None,
        }
    }
}

impl Environment {
    /// Set a binding in the environment.
    pub fn bind(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.bindings.insert(name.into(), value.into());
    }

    /// Look up a binding.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.bindings.get(name).map(|s| s.as_str())
    }

    /// Reset to defaults (for a fresh session).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_environment() {
        let env = Environment::default();
        assert_eq!(env.current_part, 0);
        assert!(env.current_key.is_none());
        assert!(env.bindings.is_empty());
    }

    #[test]
    fn bind_and_get() {
        let mut env = Environment::default();
        env.bind("motif", "(note c4 :q)");
        assert_eq!(env.get("motif"), Some("(note c4 :q)"));
        assert_eq!(env.get("unknown"), None);
    }

    #[test]
    fn reset_clears_all() {
        let mut env = Environment::default();
        env.bind("x", "y");
        env.current_part = 3;
        env.current_tempo = Some(120);
        env.reset();
        assert!(env.bindings.is_empty());
        assert_eq!(env.current_part, 0);
        assert!(env.current_tempo.is_none());
    }
}
```

---

## Task 3: Expression Evaluation (`src/session/eval.rs`)

```rust
//! Expression evaluation within a session context.
//!
//! Wraps the Phase 5 compiler and handles score accumulation:
//! evaluated expressions are merged into the session's score.

use fermata::lang::ast::FermataScore;
use fermata::lang::CompileError;

/// Result of evaluating an expression in a session.
#[derive(Debug)]
pub enum EvalResult {
    /// A complete score was produced (replaces session score).
    Score(FermataScore),
    /// The expression compiled but produced a non-score result.
    /// Display as informational output.
    Info(String),
    /// Compilation failed.
    Error(CompileError),
}

/// Evaluate a Fermata expression and determine how it should
/// affect the session's score.
///
/// The evaluation strategy:
/// - Attempt to compile the expression as a complete score.
/// - If compilation succeeds, return the score for merging.
/// - If it fails, return the error for display.
///
/// Score accumulation strategy (append-only):
/// - Complete `(score ...)` expressions replace the session score.
/// - In a future milestone, sub-expressions (part, measure, note)
///   will be merged into the existing score at the environment's
///   current position. For now, only complete scores are supported.
pub fn eval_expression(source: &str) -> EvalResult {
    match fermata::lang::compile(source) {
        Ok(score_partwise) => {
            // Convert IR ScorePartwise back to a FermataScore summary.
            // For now, we store the result as a FermataScore reconstructed
            // from the compilation. In practice, the Phase 5 compiler
            // produces a ScorePartwise (IR), and we want to keep the
            // higher-level FermataScore for the session. This is a
            // simplification that will be refined when we add partial
            // expression support.
            let fermata_score = ir_to_fermata_score(&score_partwise);
            EvalResult::Score(fermata_score)
        }
        Err(e) => EvalResult::Error(e),
    }
}

/// Convert a compiled ScorePartwise back to a FermataScore.
///
/// This is a lossy reconstruction — the FermataScore won't have
/// all the ergonomic information from the original source. It's
/// sufficient for score accumulation and display purposes.
fn ir_to_fermata_score(ir: &fermata::ir::score::ScorePartwise) -> FermataScore {
    use fermata::lang::ast::*;

    let title = ir
        .work
        .as_ref()
        .and_then(|w| w.work_title.clone());
    let composer = ir
        .identification
        .as_ref()
        .and_then(|id| {
            id.creators
                .iter()
                .find(|c| c.creator_type.as_deref() == Some("composer"))
                .map(|c| c.value.clone())
        });

    let parts = ir.parts.iter().enumerate().map(|(i, part)| {
        let name = ir
            .part_list
            .as_ref()
            .and_then(|pl| pl.parts.get(i))
            .map(|sp| sp.part_name.clone())
            .unwrap_or_else(|| format!("Part {}", i + 1));

        let measures = part.measures.iter().map(|m| {
            FermataMeasure {
                number: m.number.as_ref().and_then(|n| n.parse().ok()),
                content: Vec::new(), // Content reconstruction deferred
            }
        }).collect();

        FermataPart {
            name,
            id: None,
            measures,
        }
    }).collect();

    FermataScore {
        title,
        composer,
        parts,
    }
}

/// Apply an evaluation result to a session's score.
///
/// Returns a display string describing what changed.
pub fn apply_to_score(
    score: &mut FermataScore,
    result: &EvalResult,
) -> Option<String> {
    match result {
        EvalResult::Score(new_score) => {
            let part_count = new_score.parts.len();
            let measure_count = new_score
                .parts
                .first()
                .map(|p| p.measures.len())
                .unwrap_or(0);

            *score = new_score.clone();

            Some(format!(
                "Score updated: {} part{}, {} measure{}",
                part_count,
                if part_count == 1 { "" } else { "s" },
                measure_count,
                if measure_count == 1 { "" } else { "s" },
            ))
        }
        EvalResult::Info(msg) => Some(msg.clone()),
        EvalResult::Error(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_valid_score() {
        let source = r#"
            (score :title "Test"
              (part :name "Piano"
                (measure (note c4 :q))))
        "#;
        match eval_expression(source) {
            EvalResult::Score(s) => {
                assert_eq!(s.title.as_deref(), Some("Test"));
                assert_eq!(s.parts.len(), 1);
            }
            other => panic!("Expected Score, got {other:?}"),
        }
    }

    #[test]
    fn eval_invalid_expression() {
        match eval_expression("(note xyz)") {
            EvalResult::Error(_) => {} // expected
            other => panic!("Expected Error, got {other:?}"),
        }
    }

    #[test]
    fn apply_score_replaces() {
        let mut score = FermataScore::default();
        let result = eval_expression(r#"
            (score (part :name "P1" (measure (note c4 :q))))
        "#);
        let msg = apply_to_score(&mut score, &result);
        assert!(msg.is_some());
        assert_eq!(score.parts.len(), 1);
    }
}
```

---

## Task 4: Session Manager (`src/session/manager.rs`)

```rust
//! Session manager: create, list, switch, and query sessions.

use std::collections::HashMap;

use super::types::{Session, SessionId, SessionSummary};

/// Manages all sessions and tracks the active one.
pub struct SessionManager {
    sessions: HashMap<SessionId, Session>,
    active: Option<SessionId>,
    next_session_num: u64,
}

impl SessionManager {
    /// Create a new SessionManager with no sessions.
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            active: None,
            next_session_num: 1,
        }
    }

    /// Create a new SessionManager with a default session already active.
    pub fn with_default_session() -> Self {
        let mut mgr = Self::new();
        mgr.create(None);
        mgr
    }

    /// Create a new session, optionally with a user-supplied name.
    ///
    /// Returns the SessionId of the new session.
    /// The new session becomes active.
    pub fn create(&mut self, name: Option<&str>) -> SessionId {
        let slug = match name {
            Some(n) => self.ensure_unique_slug(n),
            None => {
                let slug = format!("session-{}", self.next_session_num);
                self.next_session_num += 1;
                slug
            }
        };

        let display_name = name.unwrap_or(&slug).to_string();
        let id = SessionId::new(&slug);
        let session = Session::new(id.clone(), display_name);
        self.sessions.insert(id.clone(), session);
        self.active = Some(id.clone());
        id
    }

    /// Ensure a slug is unique by appending a suffix if needed.
    fn ensure_unique_slug(&mut self, base: &str) -> String {
        let slug = slugify(base);
        if !self.sessions.contains_key(&SessionId::new(&slug)) {
            return slug;
        }
        // Append incrementing suffix
        for i in 2.. {
            let candidate = format!("{slug}-{i}");
            if !self.sessions.contains_key(&SessionId::new(&candidate)) {
                return candidate;
            }
        }
        unreachable!()
    }

    /// Switch the active session.
    ///
    /// Returns `Ok(())` if the session exists, `Err` with message if not.
    pub fn switch(&mut self, id: &str) -> Result<(), String> {
        let session_id = SessionId::new(id);
        if self.sessions.contains_key(&session_id) {
            self.active = Some(session_id);
            Ok(())
        } else {
            Err(format!("No session with id '{id}'"))
        }
    }

    /// Get the active session, if any.
    pub fn active(&self) -> Option<&Session> {
        self.active
            .as_ref()
            .and_then(|id| self.sessions.get(id))
    }

    /// Get the active session mutably.
    pub fn active_mut(&mut self) -> Option<&mut Session> {
        // Can't borrow self.active and self.sessions simultaneously with
        // a simple chain — clone the id first.
        let id = self.active.clone()?;
        self.sessions.get_mut(&id)
    }

    /// Get a session by id.
    pub fn get(&self, id: &str) -> Option<&Session> {
        self.sessions.get(&SessionId::new(id))
    }

    /// Get a session by id, mutably.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(&SessionId::new(id))
    }

    /// List all sessions as summaries.
    pub fn list(&self) -> Vec<SessionSummary> {
        let mut summaries: Vec<_> = self
            .sessions
            .values()
            .map(|s| s.summary())
            .collect();
        summaries.sort_by(|a, b| a.created.cmp(&b.created));
        summaries
    }

    /// The SessionId of the active session, if any.
    pub fn active_id(&self) -> Option<&SessionId> {
        self.active.as_ref()
    }

    /// Number of sessions.
    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::with_default_session()
    }
}

/// Convert a user-supplied name to a URL-safe slug.
fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_session() {
        let mgr = SessionManager::with_default_session();
        assert_eq!(mgr.count(), 1);
        assert!(mgr.active().is_some());
    }

    #[test]
    fn create_named_session() {
        let mut mgr = SessionManager::new();
        let id = mgr.create(Some("Bach Chorale"));
        assert_eq!(id.as_str(), "bach-chorale");
        assert_eq!(mgr.active().unwrap().metadata.name, "Bach Chorale");
    }

    #[test]
    fn create_auto_numbered() {
        let mut mgr = SessionManager::new();
        let id1 = mgr.create(None);
        let id2 = mgr.create(None);
        assert_eq!(id1.as_str(), "session-1");
        assert_eq!(id2.as_str(), "session-2");
    }

    #[test]
    fn unique_slugs() {
        let mut mgr = SessionManager::new();
        mgr.create(Some("Test"));
        let id2 = mgr.create(Some("Test"));
        assert_eq!(id2.as_str(), "test-2");
    }

    #[test]
    fn switch_session() {
        let mut mgr = SessionManager::new();
        let id1 = mgr.create(Some("First"));
        let _id2 = mgr.create(Some("Second"));
        // Active is now "second"
        assert_eq!(mgr.active_id().unwrap().as_str(), "second");
        mgr.switch(id1.as_str()).unwrap();
        assert_eq!(mgr.active_id().unwrap().as_str(), "first");
    }

    #[test]
    fn switch_nonexistent() {
        let mut mgr = SessionManager::with_default_session();
        assert!(mgr.switch("nonexistent").is_err());
    }

    #[test]
    fn list_sessions() {
        let mut mgr = SessionManager::new();
        mgr.create(Some("Alpha"));
        mgr.create(Some("Beta"));
        let list = mgr.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn slugify_examples() {
        assert_eq!(slugify("Bach Chorale"), "bach-chorale");
        assert_eq!(slugify("My Session #1"), "my-session-1");
        assert_eq!(slugify("  spaces  "), "spaces");
        assert_eq!(slugify("UPPER"), "upper");
    }
}
```

---

## Task 5: Update REPL to Use Sessions

### 5a. Update `src/lib.rs`

```rust
//! Fermata REPL — Interactive runtime for the Fermata music notation DSL.

pub mod repl;
pub mod session;

pub use repl::Repl;
pub use session::SessionManager;
```

### 5b. Update `src/repl/mod.rs`

The `Repl` struct now owns a `SessionManager` and uses it for evaluation:

```rust
//! REPL loop: read, classify, evaluate, display.

pub mod input;

mod commands;
mod display;
mod error;

pub use error::{ReplError, ReplResult};

use crate::session::SessionManager;
use crate::session::eval::{self, EvalResult};
use commands::CommandResult;
use input::InputClassification;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

const CONTINUATION_PROMPT: &str = "      .. ";

/// The Fermata REPL.
pub struct Repl {
    editor: DefaultEditor,
    sessions: SessionManager,
    input_buffer: String,
    paren_depth: i32,
}

impl Repl {
    /// Create a new REPL with a default session.
    pub fn new() -> ReplResult<Self> {
        let editor = DefaultEditor::new()?;

        Ok(Self {
            editor,
            sessions: SessionManager::default(),
            input_buffer: String::new(),
            paren_depth: 0,
        })
    }

    /// Access the session manager (for testing).
    pub fn sessions(&self) -> &SessionManager {
        &self.sessions
    }

    /// Run the REPL loop.
    pub fn run(&mut self) -> ReplResult<()> {
        self.print_banner();

        loop {
            let prompt = if self.input_buffer.is_empty() {
                self.build_prompt()
            } else {
                CONTINUATION_PROMPT.to_string()
            };

            match self.editor.readline(&prompt) {
                Ok(line) => {
                    if self.handle_line(&line)? {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    if !self.input_buffer.is_empty() {
                        self.reset_input();
                        println!("(input cleared)");
                    }
                }
                Err(ReadlineError::Eof) => {
                    println!();
                    break;
                }
                Err(e) => return Err(ReplError::from(e)),
            }
        }

        Ok(())
    }

    /// Build the primary prompt: `[session-name] In[n]: `
    fn build_prompt(&self) -> String {
        match self.sessions.active() {
            Some(session) => {
                format!(
                    "[{}] In[{}]: ",
                    session.metadata.name,
                    session.next_index,
                )
            }
            None => "fermata> ".to_string(),
        }
    }

    fn handle_line(&mut self, line: &str) -> ReplResult<bool> {
        if !self.input_buffer.is_empty() {
            self.input_buffer.push('\n');
        }
        self.input_buffer.push_str(line);
        self.paren_depth += input::paren_depth(line);

        if self.paren_depth > 0 {
            return Ok(false);
        }

        let complete_input = std::mem::take(&mut self.input_buffer);
        self.paren_depth = 0;
        let _ = self.editor.add_history_entry(&complete_input);

        match input::classify(&complete_input) {
            InputClassification::Empty => {}

            InputClassification::Expression(expr) => {
                self.eval_expression(&expr);
            }

            InputClassification::Command(cmd) => {
                match commands::dispatch(&cmd, &mut self.sessions) {
                    CommandResult::Output(text) => println!("{text}"),
                    CommandResult::Quit => return Ok(true),
                    CommandResult::Unknown(name) => {
                        println!("Unknown command: :{name}");
                        println!("Type :help for available commands.");
                    }
                }
            }

            InputClassification::Chat(_msg) => {
                println!("{}", display::format_stub("Chat mode"));
            }
        }

        Ok(false)
    }

    /// Evaluate an expression in the active session.
    fn eval_expression(&mut self, source: &str) {
        let result = eval::eval_expression(source);

        match &result {
            EvalResult::Score(_) => {
                if let Some(session) = self.sessions.active_mut() {
                    let msg = eval::apply_to_score(&mut session.score, &result);
                    let idx = session.advance_index();
                    if let Some(description) = msg {
                        println!("Out[{idx}]: {description}");
                    }
                    session.metadata.touch();
                }
            }
            EvalResult::Info(msg) => {
                if let Some(session) = self.sessions.active_mut() {
                    let idx = session.advance_index();
                    println!("Out[{idx}]: {msg}");
                }
            }
            EvalResult::Error(e) => {
                if let Some(session) = self.sessions.active_mut() {
                    session.advance_index(); // consume index even on error
                }
                println!("{}", display::format_compile_error(e));
            }
        }
    }

    fn reset_input(&mut self) {
        self.input_buffer.clear();
        self.paren_depth = 0;
    }

    fn print_banner(&self) {
        println!("Fermata REPL v{}", env!("CARGO_PKG_VERSION"));
        println!("Type :help for commands, :quit to exit.");
        println!();
    }
}
```

---

## Task 6: Update Command Dispatch (`src/repl/commands.rs`)

The command dispatch now receives the `SessionManager` and handles session commands:

```rust
//! REPL command dispatch.

use crate::session::SessionManager;

/// Result of executing a REPL command.
#[derive(Debug)]
pub enum CommandResult {
    Output(String),
    Quit,
    Unknown(String),
}

/// Dispatch a command string (without the leading ':').
pub fn dispatch(input: &str, sessions: &mut SessionManager) -> CommandResult {
    let trimmed = input.trim();
    let (cmd, args) = match trimmed.split_once(char::is_whitespace) {
        Some((c, a)) => (c, a.trim()),
        None => (trimmed, ""),
    };

    match cmd {
        "help" | "h" => handle_help(args),
        "quit" | "q" | "exit" => CommandResult::Quit,
        "session" | "s" => handle_session(args, sessions),
        _ => CommandResult::Unknown(cmd.to_string()),
    }
}

fn handle_session(args: &str, sessions: &mut SessionManager) -> CommandResult {
    let (subcmd, rest) = match args.split_once(char::is_whitespace) {
        Some((s, r)) => (s, r.trim()),
        None => (args, ""),
    };

    match subcmd {
        "new" | "create" => {
            let name = if rest.is_empty() { None } else { Some(rest) };
            let id = sessions.create(name);
            CommandResult::Output(format!(
                "Created session '{}' (active)",
                id.as_str()
            ))
        }
        "list" | "ls" => {
            let summaries = sessions.list();
            if summaries.is_empty() {
                return CommandResult::Output("No sessions.".to_string());
            }
            let active_id = sessions.active_id().map(|id| id.as_str().to_string());
            let lines: Vec<String> = summaries
                .iter()
                .map(|s| {
                    let marker = if Some(s.id.as_str().to_string()) == active_id {
                        " *"
                    } else {
                        "  "
                    };
                    format!(
                        "{} {} — {} ({} part{}, {} measure{})",
                        marker,
                        s.id.as_str(),
                        s.name,
                        s.part_count,
                        if s.part_count == 1 { "" } else { "s" },
                        s.measure_count,
                        if s.measure_count == 1 { "" } else { "s" },
                    )
                })
                .collect();
            CommandResult::Output(lines.join("\n"))
        }
        "switch" => {
            if rest.is_empty() {
                return CommandResult::Output(
                    "Usage: :session switch <id>".to_string(),
                );
            }
            match sessions.switch(rest) {
                Ok(()) => {
                    let name = sessions
                        .active()
                        .map(|s| s.metadata.name.as_str())
                        .unwrap_or(rest);
                    CommandResult::Output(format!("Switched to '{name}'"))
                }
                Err(e) => CommandResult::Output(e),
            }
        }
        "info" | "i" => {
            match sessions.active() {
                Some(session) => {
                    let info = format!(
                        "Session: {}\n\
                         Name:    {}\n\
                         Created: {}\n\
                         Parts:   {}\n\
                         Index:   {}",
                        session.id.as_str(),
                        session.metadata.name,
                        session.metadata.created.format("%Y-%m-%d %H:%M"),
                        session.score.parts.len(),
                        session.next_index,
                    );
                    CommandResult::Output(info)
                }
                None => CommandResult::Output("No active session.".to_string()),
            }
        }
        "" => CommandResult::Output(
            "Usage: :session <new|list|switch|info>".to_string(),
        ),
        other => CommandResult::Output(format!(
            "Unknown session command: '{other}'. Try :session new, list, switch, or info."
        )),
    }
}

fn handle_help(topic: &str) -> CommandResult {
    if topic.is_empty() {
        CommandResult::Output(GENERAL_HELP.to_string())
    } else {
        match topic {
            "session" | "s" => CommandResult::Output(SESSION_HELP.to_string()),
            "quit" | "exit" => CommandResult::Output(
                ":quit / :exit / :q — Exit the REPL.".to_string(),
            ),
            "help" => CommandResult::Output(
                ":help [topic] — Show help. Without a topic, lists all commands.".to_string(),
            ),
            _ => CommandResult::Output(format!("No help available for '{topic}'.")),
        }
    }
}

const GENERAL_HELP: &str = "\
Fermata REPL — Interactive music notation

Input modes:
  (expression)    Evaluate a Fermata expression
  :command         Execute a REPL command
  // message       Send a chat message (not yet connected)

Commands:
  :help [topic]             Show help
  :quit                     Exit the REPL
  :session new [name]       Create a new session
  :session list             List all sessions
  :session switch <id>      Switch active session
  :session info             Show active session details";

const SESSION_HELP: &str = "\
Session commands:
  :session new [name]       Create a new session (becomes active)
  :session list             List all sessions (* = active)
  :session switch <id>      Switch to a different session
  :session info             Show details of the active session

Aliases: :s new, :s list, :s switch, :s info";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_session_new() {
        let mut mgr = SessionManager::new();
        match dispatch("session new My Test", &mut mgr) {
            CommandResult::Output(s) => assert!(s.contains("my-test")),
            other => panic!("Expected Output, got {other:?}"),
        }
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn dispatch_session_list() {
        let mut mgr = SessionManager::new();
        mgr.create(Some("Alpha"));
        mgr.create(Some("Beta"));
        match dispatch("session list", &mut mgr) {
            CommandResult::Output(s) => {
                assert!(s.contains("alpha"));
                assert!(s.contains("beta"));
            }
            other => panic!("Expected Output, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_session_switch() {
        let mut mgr = SessionManager::new();
        mgr.create(Some("First"));
        mgr.create(Some("Second"));
        match dispatch("session switch first", &mut mgr) {
            CommandResult::Output(s) => assert!(s.contains("First")),
            other => panic!("Expected Output, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_session_info() {
        let mut mgr = SessionManager::with_default_session();
        match dispatch("session info", &mut mgr) {
            CommandResult::Output(s) => {
                assert!(s.contains("Session:"));
                assert!(s.contains("Name:"));
            }
            other => panic!("Expected Output, got {other:?}"),
        }
    }

    #[test]
    fn session_alias() {
        let mut mgr = SessionManager::with_default_session();
        match dispatch("s info", &mut mgr) {
            CommandResult::Output(s) => assert!(s.contains("Session:")),
            other => panic!("Expected Output, got {other:?}"),
        }
    }
}
```

---

## Task 7: Integration Tests

Create `crates/fermata-repl/tests/session_integration.rs`:

```rust
//! Integration tests for session management.

use fermata_repl::session::SessionManager;
use fermata_repl::session::eval::{eval_expression, apply_to_score, EvalResult};

#[test]
fn create_and_eval_in_session() {
    let mut mgr = SessionManager::with_default_session();

    let source = r#"
        (score :title "Session Test"
          (part :name "Violin"
            (measure (note a4 :q) (note b4 :q) (note c5 :h))))
    "#;

    let result = eval_expression(source);
    assert!(matches!(result, EvalResult::Score(_)));

    let session = mgr.active_mut().unwrap();
    let msg = apply_to_score(&mut session.score, &result);
    assert!(msg.is_some());
    assert_eq!(session.score.parts.len(), 1);
}

#[test]
fn switch_between_sessions_preserves_scores() {
    let mut mgr = SessionManager::new();
    let id1 = mgr.create(Some("Session A"));
    let _id2 = mgr.create(Some("Session B"));

    // Eval in Session B (currently active)
    let result = eval_expression(r#"
        (score (part :name "Cello" (measure (note c3 :w))))
    "#);
    if let EvalResult::Score(_) = &result {
        let session = mgr.active_mut().unwrap();
        apply_to_score(&mut session.score, &result);
    }

    // Session B has 1 part
    assert_eq!(mgr.active().unwrap().score.parts.len(), 1);

    // Switch to Session A
    mgr.switch(id1.as_str()).unwrap();
    // Session A still has 0 parts
    assert_eq!(mgr.active().unwrap().score.parts.len(), 0);
}

#[test]
fn prompt_shows_session_and_index() {
    let mut mgr = SessionManager::new();
    mgr.create(Some("My Song"));

    let session = mgr.active().unwrap();
    let prompt = format!(
        "[{}] In[{}]: ",
        session.metadata.name, session.next_index
    );
    assert_eq!(prompt, "[My Song] In[1]: ");
}
```

---

## Acceptance Criteria

1. ✅ `SessionManager` creates, lists, switches, and queries sessions
2. ✅ Each session owns an independent score, environment, and index counter
3. ✅ `:session new <name>` creates a named session and makes it active
4. ✅ `:session list` shows all sessions with `*` marking the active one
5. ✅ `:session switch <id>` switches the active session
6. ✅ `:session info` displays session metadata, part count, and current index
7. ✅ The prompt displays `[session-name] In[n]: ` with live session name and index
8. ✅ Evaluating a complete `(score ...)` expression updates the active session's score
9. ✅ The history index advances on each expression (including errors)
10. ✅ Session slugs are generated from names (lowercase, hyphenated, unique)
11. ✅ `Environment` holds implicit state and bindings (infrastructure ready for future use)
12. ✅ All unit tests and integration tests pass
13. ✅ The `fermata` crate is unchanged

---

## Implementation Notes

1. **Score accumulation is whole-score only in this milestone.** The `eval_expression` function compiles the full input via `fermata::lang::compile()`, which expects a `(score ...)` form. Partial expression handling (bare notes, measures) requires changes to the compiler's top-level dispatch, which is deferred. Users must wrap their input in a score form for now.

2. **`ir_to_fermata_score` is lossy.** Converting from the IR `ScorePartwise` back to `FermataScore` loses note-level detail (the `content` vec on each measure is empty). This is acceptable because the session score is used for metadata display and accumulation tracking. Full round-tripping is a future enhancement.

3. **Session ID is a slug, not a UUID.** The `SessionId` is the slug (e.g., `bach-chorale`). The UUID is on `SessionMetadata` for storage. This keeps REPL interactions human-friendly (`:session switch bach-chorale` rather than `:session switch 3f7a...`).

4. **`dispatch` signature changed.** The command dispatch function now takes `&mut SessionManager`. This requires updating the calling code in `Repl::handle_line`. The Milestone 1 `dispatch(input: &str)` signature is superseded.

5. **Auto-created default session.** `SessionManager::default()` creates a `session-1` session automatically. This means the user always has an active session on startup without needing to run `:session new`. The behavior of startup with persisted sessions is deferred to Milestone 10 (persistence).

---

*Next: Milestone 3 — Typed History & Query*
