# Phase 6a: Core REPL — Milestone 3: Typed History & Query

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** HistoryEntry, EntryKind, typed history on Session, history query/filtering, display formatting, REPL commands
> **Depends On:** Milestone 6a-2 (sessions exist, index counter exists)

---

## Overview

This milestone replaces the ad-hoc index counter and rustyline-only history with a fully typed, queryable history system. At the end of this milestone:

- Every expression evaluation and REPL command produces a `HistoryEntry` with a monotonic index, timestamp, and typed `EntryKind`
- The REPL displays results as `In[n]: (input)` / `Out[n]: (result)`
- `:history` shows recent entries; `:history 10:20` shows a specific range
- `:history --code` filters to Eval entries only
- `:history --grep <pat>` searches within entry text
- `:history --since <dur>` filters by time

Two `EntryKind` variants are implemented here: **Eval** and **Command**. The remaining three (AiMessage, UserMessage, System) arrive in Milestone 4 (6b). The history infrastructure is designed to accommodate all five from the start.

---

## Architecture

### HistoryEntry

```rust
HistoryEntry {
    index: u64,               // Monotonic, gapless, 1-based
    timestamp: DateTime<Utc>,  // Wall-clock time
    kind: EntryKind,          // What happened
}
```

### EntryKind (this milestone)

```rust
EntryKind::Eval {
    input: String,            // Raw source text
    result: Option<String>,   // Display representation of result
    error: Option<String>,    // Error message, if any
}

EntryKind::Command {
    raw: String,              // Full command text (without ':')
    output: Option<String>,   // Command output, if any
}
```

### EntryKind (added in Milestone 4)

```rust
// These are defined now as variants but not yet created by the REPL loop.
// Defining them now avoids breaking changes when Milestone 4 adds them.

EntryKind::AiMessage {
    content: String,
    source: String,           // "mcp", "test-inject", etc.
}

EntryKind::UserMessage {
    content: String,
}

EntryKind::System {
    event: SystemEvent,
}
```

### HistoryQuery

```rust
HistoryQuery {
    from_index: Option<u64>,
    to_index: Option<u64>,
    since: Option<Duration>,
    kinds: Vec<EntryKindFilter>,
    text_search: Option<String>,
    limit: Option<usize>,
}
```

### Where History Lives

History is a `Vec<HistoryEntry>` on `Session`. The REPL loop creates entries and pushes them. The index comes from `session.advance_index()`, which was established in Milestone 2.

```
Session
├── ...
├── history: Vec<HistoryEntry>     ← NEW
├── next_index: u64                ← existing (Milestone 2)
└── ...
```

---

## Task 0: Add Dependencies

Update `crates/fermata-repl/Cargo.toml` if not already present:

```toml
[dependencies]
fermata = { path = "../fermata" }
rustyline = "14"
thiserror = "2"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Notes:**
- `serde` and `serde_json` are added for future serialization of history entries (persistence, Milestone 10). Deriving `Serialize`/`Deserialize` now avoids a retrofit later.
- `chrono` was already added in Milestone 2.

---

## Task 1: History Types (`src/history/mod.rs`)

Create `src/history/` module:

```rust
//! Typed history: every REPL interaction as a numbered, timestamped entry.
//!
//! The history captures evaluations, commands, messages, and system events
//! in a unified timeline. Each entry gets a monotonic index and a timestamp.

pub mod query;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Monotonic, gapless, 1-based index.
    pub index: u64,
    /// Wall-clock timestamp.
    pub timestamp: DateTime<Utc>,
    /// What happened.
    pub kind: EntryKind,
}

impl HistoryEntry {
    /// Create a new history entry with the given index and kind.
    pub fn new(index: u64, kind: EntryKind) -> Self {
        Self {
            index,
            timestamp: Utc::now(),
            kind,
        }
    }

    /// Whether this entry is an Eval.
    pub fn is_eval(&self) -> bool {
        matches!(self.kind, EntryKind::Eval { .. })
    }

    /// Whether this entry is a Command.
    pub fn is_command(&self) -> bool {
        matches!(self.kind, EntryKind::Command { .. })
    }

    /// Whether this entry is an AiMessage.
    pub fn is_ai_message(&self) -> bool {
        matches!(self.kind, EntryKind::AiMessage { .. })
    }

    /// Whether this entry is a UserMessage.
    pub fn is_user_message(&self) -> bool {
        matches!(self.kind, EntryKind::UserMessage { .. })
    }

    /// Whether this entry is a System event.
    pub fn is_system(&self) -> bool {
        matches!(self.kind, EntryKind::System { .. })
    }

    /// Get the searchable text content of this entry.
    pub fn text_content(&self) -> &str {
        match &self.kind {
            EntryKind::Eval { input, .. } => input,
            EntryKind::Command { raw, .. } => raw,
            EntryKind::AiMessage { content, .. } => content,
            EntryKind::UserMessage { content } => content,
            EntryKind::System { event } => event.description(),
        }
    }

    /// A short type label for display.
    pub fn type_label(&self) -> &'static str {
        match &self.kind {
            EntryKind::Eval { .. } => "eval",
            EntryKind::Command { .. } => "cmd",
            EntryKind::AiMessage { .. } => "ai",
            EntryKind::UserMessage { .. } => "user",
            EntryKind::System { .. } => "sys",
        }
    }
}

/// The type of a history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EntryKind {
    /// A Fermata expression evaluation.
    Eval {
        /// The raw source text entered by the user.
        input: String,
        /// Display representation of the result (if successful).
        result: Option<String>,
        /// Error message (if compilation/evaluation failed).
        error: Option<String>,
    },

    /// A REPL command (`:help`, `:session list`, etc.).
    Command {
        /// The full command text (without the leading `:`).
        raw: String,
        /// The command's output, if any.
        output: Option<String>,
    },

    /// A message from Claude (via MCP). Added in Milestone 4.
    AiMessage {
        /// The message content.
        content: String,
        /// Where the message came from ("mcp", "test-inject", etc.).
        source: String,
    },

    /// A message from the user to Claude (via `//` prefix). Added in Milestone 4.
    UserMessage {
        /// The message content (without the `//` prefix).
        content: String,
    },

    /// A system event. Added in Milestone 4.
    System {
        /// The event that occurred.
        event: SystemEvent,
    },
}

/// System events recorded in history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    SessionCreated { name: String },
    SessionSwitched { from: String, to: String },
    ReplStarted,
    ReplShutdown,
}

impl SystemEvent {
    /// A description for display and search.
    pub fn description(&self) -> &str {
        match self {
            SystemEvent::SessionCreated { .. } => "session created",
            SystemEvent::SessionSwitched { .. } => "session switched",
            SystemEvent::ReplStarted => "REPL started",
            SystemEvent::ReplShutdown => "REPL shutdown",
        }
    }
}

/// Filter for entry kinds in queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKindFilter {
    Eval,
    Command,
    AiMessage,
    UserMessage,
    System,
}

impl EntryKindFilter {
    /// Check if a history entry matches this filter.
    pub fn matches(&self, entry: &HistoryEntry) -> bool {
        match self {
            EntryKindFilter::Eval => entry.is_eval(),
            EntryKindFilter::Command => entry.is_command(),
            EntryKindFilter::AiMessage => entry.is_ai_message(),
            EntryKindFilter::UserMessage => entry.is_user_message(),
            EntryKindFilter::System => entry.is_system(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_eval_entry() {
        let entry = HistoryEntry::new(
            1,
            EntryKind::Eval {
                input: "(note c4 :q)".to_string(),
                result: Some("C4 quarter".to_string()),
                error: None,
            },
        );
        assert_eq!(entry.index, 1);
        assert!(entry.is_eval());
        assert!(!entry.is_command());
        assert_eq!(entry.text_content(), "(note c4 :q)");
        assert_eq!(entry.type_label(), "eval");
    }

    #[test]
    fn create_command_entry() {
        let entry = HistoryEntry::new(
            2,
            EntryKind::Command {
                raw: "session list".to_string(),
                output: Some("2 sessions".to_string()),
            },
        );
        assert!(entry.is_command());
        assert_eq!(entry.type_label(), "cmd");
    }

    #[test]
    fn kind_filter_matches() {
        let eval_entry = HistoryEntry::new(
            1,
            EntryKind::Eval {
                input: "test".to_string(),
                result: None,
                error: None,
            },
        );
        assert!(EntryKindFilter::Eval.matches(&eval_entry));
        assert!(!EntryKindFilter::Command.matches(&eval_entry));
    }

    #[test]
    fn system_event_description() {
        let event = SystemEvent::SessionCreated {
            name: "test".to_string(),
        };
        assert_eq!(event.description(), "session created");
    }

    #[test]
    fn entry_serialization_roundtrip() {
        let entry = HistoryEntry::new(
            42,
            EntryKind::Eval {
                input: "(note c4 :q)".to_string(),
                result: Some("OK".to_string()),
                error: None,
            },
        );
        let json = serde_json::to_string(&entry).unwrap();
        let restored: HistoryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.index, 42);
        assert!(restored.is_eval());
    }
}
```

---

## Task 2: History Query (`src/history/query.rs`)

```rust
//! History query and filtering.
//!
//! HistoryQuery provides composable filters for searching history entries.
//! Filters are AND-composed: an entry must match all specified criteria.

use chrono::{Duration, Utc};

use super::{EntryKindFilter, HistoryEntry};

/// A composable query over history entries.
#[derive(Debug, Clone, Default)]
pub struct HistoryQuery {
    /// Start of index range (inclusive).
    pub from_index: Option<u64>,
    /// End of index range (inclusive).
    pub to_index: Option<u64>,
    /// Only entries within this duration from now.
    pub since: Option<Duration>,
    /// Only entries of these kinds (empty = all kinds).
    pub kinds: Vec<EntryKindFilter>,
    /// Substring search within entry text content.
    pub text_search: Option<String>,
    /// Maximum number of results to return.
    pub limit: Option<usize>,
}

impl HistoryQuery {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set index range.
    pub fn range(mut self, from: u64, to: u64) -> Self {
        self.from_index = Some(from);
        self.to_index = Some(to);
        self
    }

    /// Filter to only code (Eval) entries.
    pub fn code_only(mut self) -> Self {
        self.kinds = vec![EntryKindFilter::Eval];
        self
    }

    /// Filter to chat entries (AI + User messages).
    pub fn chat_only(mut self) -> Self {
        self.kinds = vec![EntryKindFilter::AiMessage, EntryKindFilter::UserMessage];
        self
    }

    /// Filter to system events.
    pub fn system_only(mut self) -> Self {
        self.kinds = vec![EntryKindFilter::System];
        self
    }

    /// Set a text search filter.
    pub fn grep(mut self, pattern: impl Into<String>) -> Self {
        self.text_search = Some(pattern.into());
        self
    }

    /// Set a time filter.
    pub fn since_duration(mut self, duration: Duration) -> Self {
        self.since = Some(duration);
        self
    }

    /// Set a result limit.
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Execute this query against a history slice.
    ///
    /// Returns matching entries in their original order.
    pub fn execute<'a>(&self, history: &'a [HistoryEntry]) -> Vec<&'a HistoryEntry> {
        let cutoff = self.since.map(|d| Utc::now() - d);

        let mut results: Vec<&HistoryEntry> = history
            .iter()
            .filter(|entry| {
                // Index range filter
                if let Some(from) = self.from_index {
                    if entry.index < from {
                        return false;
                    }
                }
                if let Some(to) = self.to_index {
                    if entry.index > to {
                        return false;
                    }
                }

                // Time filter
                if let Some(ref cutoff) = cutoff {
                    if entry.timestamp < *cutoff {
                        return false;
                    }
                }

                // Kind filter (empty = match all)
                if !self.kinds.is_empty()
                    && !self.kinds.iter().any(|k| k.matches(entry))
                {
                    return false;
                }

                // Text search
                if let Some(ref pattern) = self.text_search {
                    let content = entry.text_content().to_lowercase();
                    if !content.contains(&pattern.to_lowercase()) {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Apply limit (from the end, to show most recent)
        if let Some(limit) = self.limit {
            if results.len() > limit {
                results = results.split_off(results.len() - limit);
            }
        }

        results
    }
}

/// Parse a duration string like "10m", "1h", "30s", "2d".
///
/// Returns None if the string can't be parsed.
pub fn parse_duration(input: &str) -> Option<Duration> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    let (num_str, suffix) = input.split_at(
        input
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(input.len()),
    );

    let num: i64 = num_str.parse().ok()?;

    match suffix.trim() {
        "s" | "sec" | "second" | "seconds" => Some(Duration::seconds(num)),
        "m" | "min" | "minute" | "minutes" => Some(Duration::minutes(num)),
        "h" | "hr" | "hour" | "hours" => Some(Duration::hours(num)),
        "d" | "day" | "days" => Some(Duration::days(num)),
        _ => None,
    }
}

/// Parse a history range string like "10:20", "5:", ":15", "42".
///
/// Returns (from, to) where both are optional.
pub fn parse_range(input: &str) -> (Option<u64>, Option<u64>) {
    let trimmed = input.trim();

    if let Some((left, right)) = trimmed.split_once(':') {
        let from = left.trim().parse().ok();
        let to = right.trim().parse().ok();
        (from, to)
    } else {
        // Single number: show just that entry
        if let Ok(n) = trimmed.parse::<u64>() {
            (Some(n), Some(n))
        } else {
            (None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::{EntryKind, HistoryEntry};

    fn make_eval(index: u64, input: &str) -> HistoryEntry {
        HistoryEntry::new(
            index,
            EntryKind::Eval {
                input: input.to_string(),
                result: Some("ok".to_string()),
                error: None,
            },
        )
    }

    fn make_command(index: u64, raw: &str) -> HistoryEntry {
        HistoryEntry::new(
            index,
            EntryKind::Command {
                raw: raw.to_string(),
                output: None,
            },
        )
    }

    fn sample_history() -> Vec<HistoryEntry> {
        vec![
            make_eval(1, "(note c4 :q)"),
            make_eval(2, "(note d4 :q)"),
            make_command(3, "session list"),
            make_eval(4, "(chord (c4 e4 g4) :h)"),
            make_command(5, "help"),
            make_eval(6, "(note e4 :q)"),
        ]
    }

    #[test]
    fn query_all() {
        let history = sample_history();
        let results = HistoryQuery::new().execute(&history);
        assert_eq!(results.len(), 6);
    }

    #[test]
    fn query_by_range() {
        let history = sample_history();
        let results = HistoryQuery::new().range(2, 4).execute(&history);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].index, 2);
        assert_eq!(results[2].index, 4);
    }

    #[test]
    fn query_code_only() {
        let history = sample_history();
        let results = HistoryQuery::new().code_only().execute(&history);
        assert_eq!(results.len(), 4); // entries 1, 2, 4, 6
        assert!(results.iter().all(|e| e.is_eval()));
    }

    #[test]
    fn query_grep() {
        let history = sample_history();
        let results = HistoryQuery::new().grep("chord").execute(&history);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].index, 4);
    }

    #[test]
    fn query_grep_case_insensitive() {
        let history = sample_history();
        let results = HistoryQuery::new().grep("NOTE").execute(&history);
        assert_eq!(results.len(), 3); // entries 1, 2, 6
    }

    #[test]
    fn query_with_limit() {
        let history = sample_history();
        let results = HistoryQuery::new().limit(2).execute(&history);
        assert_eq!(results.len(), 2);
        // Should be the last 2 entries
        assert_eq!(results[0].index, 5);
        assert_eq!(results[1].index, 6);
    }

    #[test]
    fn query_combined_filters() {
        let history = sample_history();
        let results = HistoryQuery::new()
            .range(1, 5)
            .code_only()
            .execute(&history);
        assert_eq!(results.len(), 3); // entries 1, 2, 4
    }

    #[test]
    fn parse_duration_variants() {
        assert_eq!(parse_duration("10m"), Some(Duration::minutes(10)));
        assert_eq!(parse_duration("1h"), Some(Duration::hours(1)));
        assert_eq!(parse_duration("30s"), Some(Duration::seconds(30)));
        assert_eq!(parse_duration("2d"), Some(Duration::days(2)));
        assert_eq!(parse_duration("5min"), Some(Duration::minutes(5)));
        assert_eq!(parse_duration(""), None);
        assert_eq!(parse_duration("abc"), None);
    }

    #[test]
    fn parse_range_variants() {
        assert_eq!(parse_range("10:20"), (Some(10), Some(20)));
        assert_eq!(parse_range("5:"), (Some(5), None));
        assert_eq!(parse_range(":15"), (None, Some(15)));
        assert_eq!(parse_range("42"), (Some(42), Some(42)));
        assert_eq!(parse_range("abc"), (None, None));
    }
}
```

---

## Task 3: Add History to Session (`src/session/types.rs` updates)

Add the history `Vec` to the `Session` struct:

```rust
// In Session struct, add:
use crate::history::HistoryEntry;

pub struct Session {
    pub id: SessionId,
    pub metadata: SessionMetadata,
    pub score: fermata::lang::ast::FermataScore,
    pub env: Environment,
    pub next_index: u64,
    /// Typed history of all interactions in this session.
    pub history: Vec<HistoryEntry>,
}

impl Session {
    pub fn new(id: SessionId, name: impl Into<String>) -> Self {
        Self {
            id,
            metadata: SessionMetadata::new(name),
            score: fermata::lang::ast::FermataScore::default(),
            env: Environment::default(),
            next_index: 1,
            history: Vec::new(),
        }
    }

    /// Push a history entry. Automatically uses the next index.
    pub fn push_history(&mut self, kind: crate::history::EntryKind) -> u64 {
        let idx = self.advance_index();
        let entry = HistoryEntry::new(idx, kind);
        self.history.push(entry);
        idx
    }

    // ... existing methods unchanged ...
}
```

---

## Task 4: History Display Formatting (`src/repl/display.rs` updates)

Add history display functions:

```rust
use crate::history::{EntryKind, HistoryEntry};

/// Format a history entry for REPL display.
pub fn format_history_entry(entry: &HistoryEntry) -> String {
    match &entry.kind {
        EntryKind::Eval {
            input,
            result,
            error,
        } => {
            let mut lines = Vec::new();
            lines.push(format!("In[{}]:  {}", entry.index, input.trim()));
            if let Some(result) = result {
                lines.push(format!("Out[{}]: {}", entry.index, result));
            }
            if let Some(error) = error {
                lines.push(format!("Err[{}]: {}", entry.index, error));
            }
            lines.join("\n")
        }

        EntryKind::Command { raw, output } => {
            let mut lines = Vec::new();
            lines.push(format!("In[{}]:  :{}", entry.index, raw));
            if let Some(output) = output {
                // Indent multi-line command output
                for line in output.lines() {
                    lines.push(format!("        {line}"));
                }
            }
            lines.join("\n")
        }

        EntryKind::AiMessage { content, source } => {
            format!(
                "[{}] [Claude ({})]: {}",
                entry.index, source, content
            )
        }

        EntryKind::UserMessage { content } => {
            format!("[{}] [you]: {}", entry.index, content)
        }

        EntryKind::System { event } => {
            format!("[{}] (system) {}", entry.index, format_system_event(event))
        }
    }
}

fn format_system_event(event: &crate::history::SystemEvent) -> String {
    use crate::history::SystemEvent;
    match event {
        SystemEvent::SessionCreated { name } => {
            format!("Session '{name}' created")
        }
        SystemEvent::SessionSwitched { from, to } => {
            format!("Switched from '{from}' to '{to}'")
        }
        SystemEvent::ReplStarted => "REPL started".to_string(),
        SystemEvent::ReplShutdown => "REPL shutdown".to_string(),
    }
}

/// Format a list of history entries with separators.
pub fn format_history_listing(entries: &[&HistoryEntry]) -> String {
    if entries.is_empty() {
        return "(no matching entries)".to_string();
    }

    entries
        .iter()
        .map(|e| format_history_entry(e))
        .collect::<Vec<_>>()
        .join("\n")
}
```

---

## Task 5: History REPL Commands (`src/repl/commands.rs` updates)

Add the `:history` command family to the dispatch:

```rust
// Add to the match in dispatch():
"history" | "hist" => handle_history(args, sessions),

// New handler function:
fn handle_history(args: &str, sessions: &SessionManager) -> CommandResult {
    let session = match sessions.active() {
        Some(s) => s,
        None => return CommandResult::Output("No active session.".to_string()),
    };

    if session.history.is_empty() {
        return CommandResult::Output("(no history)".to_string());
    }

    let query = parse_history_args(args);
    let results = query.execute(&session.history);
    let display = crate::repl::display::format_history_listing(&results);
    CommandResult::Output(display)
}

/// Parse history command arguments into a HistoryQuery.
///
/// Examples:
///   :history              → last 20 entries
///   :history 10:20        → entries 10 through 20
///   :history --code       → only Eval entries
///   :history --grep note  → entries containing "note"
///   :history --since 10m  → entries from the last 10 minutes
///   :history 5:15 --code  → Eval entries in range 5–15
fn parse_history_args(args: &str) -> crate::history::query::HistoryQuery {
    use crate::history::query::{parse_duration, parse_range, HistoryQuery};

    let mut query = HistoryQuery::new();
    let mut tokens: Vec<&str> = args.split_whitespace().collect();

    // Default: show last 20 entries if no range specified
    let mut has_range = false;

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            "--code" => {
                query = query.code_only();
                i += 1;
            }
            "--chat" => {
                query = query.chat_only();
                i += 1;
            }
            "--system" | "--sys" => {
                query = query.system_only();
                i += 1;
            }
            "--all" => {
                // No filter — show everything (kinds stays empty = all)
                i += 1;
            }
            "--grep" => {
                if i + 1 < tokens.len() {
                    query = query.grep(tokens[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--since" => {
                if i + 1 < tokens.len() {
                    if let Some(dur) = parse_duration(tokens[i + 1]) {
                        query = query.since_duration(dur);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            token => {
                // Try to parse as a range (e.g., "10:20", "42")
                let (from, to) = parse_range(token);
                if from.is_some() || to.is_some() {
                    query.from_index = from;
                    query.to_index = to;
                    has_range = true;
                }
                i += 1;
            }
        }
    }

    // Default limit if no range specified
    if !has_range && query.from_index.is_none() && query.to_index.is_none() {
        query = query.limit(20);
    }

    query
}
```

Also update `GENERAL_HELP` to include history commands:

```rust
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
  :session info             Show active session details
  :history [range]          Show history (e.g., :history 10:20)
  :history --code           Show only evaluations
  :history --grep <pat>     Search in history
  :history --since <dur>    Recent history (e.g., 10m, 1h)";
```

---

## Task 6: Update REPL Loop to Record History

The REPL loop must now create typed history entries for every interaction. Update `src/repl/mod.rs`:

```rust
// In handle_line(), replace the Expression and Command arms:

InputClassification::Expression(expr) => {
    self.eval_and_record(&expr);
}

InputClassification::Command(cmd) => {
    self.execute_and_record(&cmd);
}

InputClassification::Chat(msg) => {
    // Stub: just acknowledge. Full support in Milestone 4.
    println!("{}", display::format_stub("Chat mode"));
    // Still record it as a command for now
    if let Some(session) = self.sessions.active_mut() {
        session.push_history(crate::history::EntryKind::Command {
            raw: format!("// {msg}"),
            output: Some("(chat not yet connected)".to_string()),
        });
    }
}
```

New methods on `Repl`:

```rust
/// Evaluate a Fermata expression, update the session score,
/// and record the result in history.
fn eval_and_record(&mut self, source: &str) {
    let result = eval::eval_expression(source);

    let (result_str, error_str) = match &result {
        EvalResult::Score(_) => {
            let session = self.sessions.active_mut().unwrap();
            let msg = eval::apply_to_score(&mut session.score, &result);
            (msg, None)
        }
        EvalResult::Info(msg) => (Some(msg.clone()), None),
        EvalResult::Error(e) => (None, Some(display::format_compile_error(e))),
    };

    // Record in history
    if let Some(session) = self.sessions.active_mut() {
        let idx = session.push_history(crate::history::EntryKind::Eval {
            input: source.to_string(),
            result: result_str.clone(),
            error: error_str.clone(),
        });

        // Display
        println!("In[{idx}]:  {}", source.trim());
        if let Some(ref result) = result_str {
            println!("Out[{idx}]: {result}");
        }
        if let Some(ref error) = error_str {
            println!("Err[{idx}]: {error}");
        }

        session.metadata.touch();
    }
}

/// Execute a REPL command and record it in history.
fn execute_and_record(&mut self, cmd: &str) {
    let result = commands::dispatch(cmd, &mut self.sessions);

    let output = match &result {
        CommandResult::Output(text) => {
            println!("{text}");
            Some(text.clone())
        }
        CommandResult::Quit => None,
        CommandResult::Unknown(name) => {
            let msg = format!("Unknown command: :{name}");
            println!("{msg}");
            println!("Type :help for available commands.");
            Some(msg)
        }
    };

    // Record in history (except quit — the session may already be gone)
    if !matches!(result, CommandResult::Quit) {
        if let Some(session) = self.sessions.active_mut() {
            session.push_history(crate::history::EntryKind::Command {
                raw: cmd.to_string(),
                output,
            });
        }
    }

    // Handle quit after recording
    if matches!(result, CommandResult::Quit) {
        // The caller checks the return from handle_line
    }
}
```

**Note:** The `handle_line` method needs to return `Ok(true)` for quit. The cleanest approach is to have `execute_and_record` return whether quit was requested:

```rust
/// Returns true if the REPL should exit.
fn execute_and_record(&mut self, cmd: &str) -> bool {
    let result = commands::dispatch(cmd, &mut self.sessions);
    let is_quit = matches!(result, CommandResult::Quit);

    let output = match &result {
        CommandResult::Output(text) => {
            println!("{text}");
            Some(text.clone())
        }
        CommandResult::Quit => None,
        CommandResult::Unknown(name) => {
            let msg = format!("Unknown command: :{name}");
            println!("{msg}");
            println!("Type :help for available commands.");
            Some(msg)
        }
    };

    if !is_quit {
        if let Some(session) = self.sessions.active_mut() {
            session.push_history(crate::history::EntryKind::Command {
                raw: cmd.to_string(),
                output,
            });
        }
    }

    is_quit
}
```

And in `handle_line`:

```rust
InputClassification::Command(cmd) => {
    if self.execute_and_record(&cmd) {
        return Ok(true);
    }
}
```

---

## Task 7: Update `src/lib.rs`

```rust
pub mod repl;
pub mod session;
pub mod history;

pub use repl::Repl;
pub use session::SessionManager;
```

---

## Task 8: Integration Tests

Create `crates/fermata-repl/tests/history_integration.rs`:

```rust
//! Integration tests for typed history.

use fermata_repl::history::{EntryKind, HistoryEntry};
use fermata_repl::history::query::HistoryQuery;
use fermata_repl::session::SessionManager;
use fermata_repl::session::eval::{eval_expression, apply_to_score, EvalResult};

/// Simulate a sequence of REPL interactions and verify history.
#[test]
fn history_records_evals_and_commands() {
    let mut mgr = SessionManager::with_default_session();
    let session = mgr.active_mut().unwrap();

    // Simulate: user evaluates an expression
    let source = r#"(score (part :name "P" (measure (note c4 :q))))"#;
    let result = eval_expression(source);
    let result_str = if let EvalResult::Score(_) = &result {
        apply_to_score(&mut session.score, &result)
    } else {
        None
    };
    session.push_history(EntryKind::Eval {
        input: source.to_string(),
        result: result_str,
        error: None,
    });

    // Simulate: user runs a command
    session.push_history(EntryKind::Command {
        raw: "session info".to_string(),
        output: Some("Session: session-1".to_string()),
    });

    // Simulate: another eval
    session.push_history(EntryKind::Eval {
        input: "(note d4 :q)".to_string(),
        result: None,
        error: Some("expected (score ...)".to_string()),
    });

    assert_eq!(session.history.len(), 3);
    assert_eq!(session.history[0].index, 1);
    assert_eq!(session.history[1].index, 2);
    assert_eq!(session.history[2].index, 3);
}

#[test]
fn history_query_code_only() {
    let mut mgr = SessionManager::with_default_session();
    let session = mgr.active_mut().unwrap();

    session.push_history(EntryKind::Eval {
        input: "(note c4 :q)".to_string(),
        result: Some("ok".to_string()),
        error: None,
    });
    session.push_history(EntryKind::Command {
        raw: "help".to_string(),
        output: None,
    });
    session.push_history(EntryKind::Eval {
        input: "(note d4 :q)".to_string(),
        result: Some("ok".to_string()),
        error: None,
    });

    let results = HistoryQuery::new()
        .code_only()
        .execute(&session.history);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|e| e.is_eval()));
}

#[test]
fn history_query_grep() {
    let mut mgr = SessionManager::with_default_session();
    let session = mgr.active_mut().unwrap();

    session.push_history(EntryKind::Eval {
        input: "(note c4 :q)".to_string(),
        result: None,
        error: None,
    });
    session.push_history(EntryKind::Eval {
        input: "(chord (c4 e4 g4) :h)".to_string(),
        result: None,
        error: None,
    });
    session.push_history(EntryKind::Eval {
        input: "(note d4 :q)".to_string(),
        result: None,
        error: None,
    });

    let results = HistoryQuery::new()
        .grep("chord")
        .execute(&session.history);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].index, 2);
}

#[test]
fn history_indices_are_monotonic_and_gapless() {
    let mut mgr = SessionManager::with_default_session();
    let session = mgr.active_mut().unwrap();

    for i in 0..10 {
        session.push_history(EntryKind::Eval {
            input: format!("(note c{} :q)", i),
            result: None,
            error: None,
        });
    }

    for (i, entry) in session.history.iter().enumerate() {
        assert_eq!(entry.index, (i + 1) as u64);
    }
}

#[test]
fn history_entries_serialize_to_jsonl() {
    let mut mgr = SessionManager::with_default_session();
    let session = mgr.active_mut().unwrap();

    session.push_history(EntryKind::Eval {
        input: "(note c4 :q)".to_string(),
        result: Some("ok".to_string()),
        error: None,
    });
    session.push_history(EntryKind::Command {
        raw: "help".to_string(),
        output: Some("Fermata REPL".to_string()),
    });

    // Verify each entry can be serialized to one JSON line
    for entry in &session.history {
        let json = serde_json::to_string(entry).unwrap();
        assert!(!json.contains('\n'), "JSONL entry must be single-line");
        // Verify roundtrip
        let restored: HistoryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.index, entry.index);
    }
}
```

---

## Acceptance Criteria

1. ✅ `HistoryEntry` struct with `index`, `timestamp`, and `EntryKind`
2. ✅ `EntryKind` has all five variants (Eval, Command, AiMessage, UserMessage, System) — only Eval and Command are created by the REPL loop in this milestone
3. ✅ Every expression evaluation creates an Eval entry in the active session's history
4. ✅ Every command execution creates a Command entry in the active session's history
5. ✅ The REPL displays `In[n]: (input)` / `Out[n]: (result)` / `Err[n]: (error)` for evaluations
6. ✅ `:history` shows the last 20 entries (default)
7. ✅ `:history 10:20` shows entries in the specified index range
8. ✅ `:history --code` shows only Eval entries
9. ✅ `:history --grep <pat>` filters by case-insensitive substring match
10. ✅ `:history --since <dur>` filters by time (supports `s`, `m`, `h`, `d` suffixes)
11. ✅ Filters compose: `:history 1:100 --code --grep chord` works
12. ✅ History indices are monotonic, gapless, and 1-based
13. ✅ History entries serialize to JSON (for future persistence)
14. ✅ All unit tests and integration tests pass
15. ✅ The `fermata` crate is unchanged

---

## Implementation Notes

1. **All five `EntryKind` variants are defined now.** Even though only Eval and Command are used in this milestone, defining all five avoids a breaking API change in Milestone 4. The AiMessage, UserMessage, and System variants are inert — the REPL loop doesn't create them yet, but tests can construct them and the display/query infrastructure handles them.

2. **`push_history` is the single entry point.** All history writes go through `Session::push_history()`, which handles index assignment. This ensures the monotonic, gapless invariant. The REPL loop must never manually create a `HistoryEntry` with an index — always use `push_history`.

3. **Text search is case-insensitive substring matching.** No regex, no fuzzy search. This is intentionally simple. If users need regex, it can be added later. The `--grep` flag echoes Git's interface.

4. **The limit applies from the end.** When `--limit 20` is active (the default with no range), the query returns the _last_ 20 entries, not the first 20. This matches the expected behavior: `:history` shows recent interactions.

5. **`serde` derives on history types.** These are needed for JSONL serialization (Milestone 10: persistence) and for IPC (Milestone 6: MCP bridge). Adding them now means the types are ready without a retrofit.

6. **The prompt now reads from live session state.** The prompt format is `[session-name] In[n]:` where `n` is `session.next_index`. After each interaction, `next_index` advances, and the prompt updates. This gives the user a continuous sense of progress.

---

*Next: Milestone 4 (6b) — Chat Mode & Message Queue*
