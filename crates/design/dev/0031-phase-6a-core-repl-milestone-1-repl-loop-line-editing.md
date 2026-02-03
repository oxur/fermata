# Phase 6a: Core REPL — Milestone 1: REPL Loop & Line Editing

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Crate setup, REPL loop, input classification, multi-line input, eval integration, basic display
> **Depends On:** Phases 1–5 (complete)

---

Notes from Duncan, things I would like to change in this spec:

- We need to use `reedline` instead of `rustyline`: we'll get really nice tab-completion, syntax highlighting, multi-line support, etc., with `reedline`; it's a much more modern take ... what a 21st readline library can be
- There is only going to be one binary for this project: `fermata`; the CLI work that we do in `fermata-repl` will be called when running `fermata repl`. For this, we will of course need to update the `fermata` create to pull in `fermata-repl` as a dep, and then update the `fermata` binary CLI options. Additionally, if no command is given to `fermata` then, by default, the behaviour will be the same as if it was run with `fermata repl`.
- Duncan wants to brainstorm with Claude Code about `//` when Claude Code reviews this document! I'm thinking about taking some inspiration from old MUD commands ...
  - we'll use `:` as suggested below for REPL commands instead of the admin-level `@` commands of MUDs/MUSHes.
  - But I definitely want to support /`/em <user emotes something>` that will get sent as a chat msg to Claude :-). I'm a firm believer in sharing internal state! Oh, and I'd want to do a `/me <user self-action>` as an alias for `/em`
  - MUDs had things like `/say ...`, `/whisper ...`, `/yell ...` and I think those might be nice options.
  - `/ <msg>` by itself could be aliases to `/say <msg>`. We'll chat. Want to make sure there are no collisions ...
- Question: would we benefit from any of our parsing / language setup? If so, that might introduce circular dependencies ... so we may want to create a new crate like `fermata-cli` and move all our CLI stuff in there, and have _that_ crate produce the `fermata` binary ...

---

## Overview

This milestone creates the `fermata-repl` crate and implements the foundational REPL loop. At the end of this milestone, a user can:

- Launch the `fermata-repl` binary
- Type Fermata expressions and see compiled results (or errors)
- Type `:help` and `:quit` commands
- Enter multi-line expressions (unmatched parentheses trigger continuation prompts)
- Use line editing (cursor movement, Ctrl-R history search) via `rustyline`

There are no sessions yet — everything runs in a single implicit context. The in-memory "history" at this stage is just the raw `rustyline` line buffer (replaced by typed history in Milestone 3).

---

## Architecture

### REPL Loop Flow

```
┌──────────────────────────────────────┐
│           fermata-repl               │
│                                      │
│  loop {                              │
│    1. Display prompt                 │
│    2. Read line (rustyline)          │
│    3. Accumulate multi-line input    │
│    4. Classify: expression | command │
│    5. Process:                       │
│       - Expression → compile → print │
│       - Command → dispatch → print   │
│    6. Add to rustyline history       │
│  }                                   │
└──────────┬───────────────────────────┘
           │ depends on
           ▼
┌──────────────────────────────────────┐
│           fermata (library)          │
│  lang::compile() → ScorePartwise     │
│  sexpr::printer → pretty-print       │
└──────────────────────────────────────┘
```

### Input Classification

```
Input starts with '(' or is bare text → Fermata expression
Input starts with ':'               → REPL command
Input starts with '//'              → Chat message (stub — logged, not routed until 6b)
Empty / whitespace-only             → No-op (re-prompt)
```

The `//` path is detected here but only stubbed (prints a message saying chat is not yet connected). Full chat support comes in Milestone 4.

---

## Task 0: Workspace Configuration

### 0a. Create `fermata-repl` crate directory

```
crates/fermata-repl/
├── Cargo.toml
└── src/
    ├── main.rs
    └── lib.rs
```

### 0b. `fermata-repl/Cargo.toml`

```toml
[package]
name = "fermata-repl"
version = "0.1.0"
edition = "2021"
description = "Interactive REPL for the Fermata music notation DSL"

[[bin]]
name = "fermata-repl"
path = "src/main.rs"

[lib]
name = "fermata_repl"
path = "src/lib.rs"

[dependencies]
fermata = { path = "../fermata" }
rustyline = "14"
thiserror = "2"
```

**Notes:**

- `rustyline` 14 is the current stable release. It provides `Editor`, `Helper` trait for completions, and configurable key bindings.
- The crate has both a `[[bin]]` target (the REPL executable) and a `[lib]` target (for integration testing and future use by the MCP bridge).
- `thiserror` for REPL-specific error types.
- No `chrono`, `serde`, or `uuid` yet — those arrive with sessions (Milestone 2) and typed history (Milestone 3).

### 0c. Update workspace `Cargo.toml`

In the root `Cargo.toml` (or wherever the workspace is defined), add:

```toml
[workspace]
members = [
    "crates/fermata",
    "crates/fermata-repl",
]
```

### 0d. Verify build

```bash
cargo build -p fermata-repl
```

This should compile successfully with an empty `main.rs` and `lib.rs`.

---

## Task 1: Module Structure

### 1a. `src/lib.rs`

```rust
//! Fermata REPL — Interactive runtime for the Fermata music notation DSL.
//!
//! This crate provides:
//! - A read-eval-print loop for Fermata expressions
//! - Session management (multiple named sessions with independent scores)
//! - Numbered, typed history of all REPL interactions
//! - Communication channel for AI assistant messages
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use fermata_repl::repl::Repl;
//!
//! let mut repl = Repl::new()?;
//! repl.run()?;
//! ```

pub mod repl;

pub use repl::Repl;
```

### 1b. `src/main.rs`

```rust
//! Fermata REPL binary entry point.

use fermata_repl::Repl;
use std::process;

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("fermata-repl: {e}");
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut repl = Repl::new()?;
    repl.run()?;
    Ok(())
}
```

### 1c. `src/repl/mod.rs`

```rust
//! REPL loop: read, classify, evaluate, display.

mod commands;
mod display;
mod error;
mod input;

pub use error::ReplError;

use input::InputClassification;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

/// The Fermata REPL.
pub struct Repl {
    editor: DefaultEditor,
    /// Accumulator for multi-line input.
    input_buffer: String,
    /// Tracks parenthesis depth for multi-line detection.
    paren_depth: i32,
}
```

### 1d. Full module tree (this milestone)

```
src/
├── main.rs              # Binary entry point
├── lib.rs               # Library root
└── repl/
    ├── mod.rs           # Repl struct, run() loop
    ├── input.rs         # Input classification, multi-line accumulation
    ├── commands.rs      # :help, :quit dispatch
    ├── display.rs       # Result formatting, error formatting
    └── error.rs         # ReplError enum
```

---

## Task 2: Error Types (`src/repl/error.rs`)

```rust
//! REPL error types.

use thiserror::Error;

/// Errors that can occur in the REPL.
#[derive(Debug, Error)]
pub enum ReplError {
    #[error("Readline error: {0}")]
    Readline(#[from] rustyline::error::ReadlineError),

    #[error("Compilation error: {0}")]
    Compile(#[from] fermata::lang::CompileError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Message(String),
}

impl ReplError {
    pub fn message(msg: impl Into<String>) -> Self {
        ReplError::Message(msg.into())
    }
}

pub type ReplResult<T> = Result<T, ReplError>;
```

---

## Task 3: Input Classification (`src/repl/input.rs`)

```rust
//! Input classification and multi-line accumulation.

/// Classified user input.
#[derive(Debug, Clone, PartialEq)]
pub enum InputClassification {
    /// A Fermata expression to evaluate.
    Expression(String),
    /// A REPL command (without the leading ':').
    Command(String),
    /// A chat message to Claude (without the leading '//').
    Chat(String),
    /// Empty input — re-prompt.
    Empty,
}

/// Classify a complete input string.
pub fn classify(input: &str) -> InputClassification {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return InputClassification::Empty;
    }

    if let Some(rest) = trimmed.strip_prefix("//") {
        return InputClassification::Chat(rest.trim().to_string());
    }

    if let Some(rest) = trimmed.strip_prefix(':') {
        return InputClassification::Command(rest.to_string());
    }

    InputClassification::Expression(input.to_string())
}

/// Count the net parenthesis depth of a string.
///
/// Returns a positive number if there are unmatched opening parens,
/// zero if balanced, negative if there are unmatched closing parens.
///
/// Respects double-quoted strings: parentheses inside `"..."` are not counted.
pub fn paren_depth(input: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut prev_backslash = false;

    for ch in input.chars() {
        if in_string {
            if ch == '"' && !prev_backslash {
                in_string = false;
            }
            prev_backslash = ch == '\\';
            continue;
        }

        match ch {
            '"' => in_string = true,
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_expression() {
        assert_eq!(
            classify("(note c4 :q)"),
            InputClassification::Expression("(note c4 :q)".to_string())
        );
    }

    #[test]
    fn classify_bare_text_as_expression() {
        assert_eq!(
            classify("c4"),
            InputClassification::Expression("c4".to_string())
        );
    }

    #[test]
    fn classify_command() {
        assert_eq!(
            classify(":help"),
            InputClassification::Command("help".to_string())
        );
    }

    #[test]
    fn classify_command_with_args() {
        assert_eq!(
            classify(":session list"),
            InputClassification::Command("session list".to_string())
        );
    }

    #[test]
    fn classify_chat() {
        assert_eq!(
            classify("// what key is this?"),
            InputClassification::Chat("what key is this?".to_string())
        );
    }

    #[test]
    fn classify_chat_no_space() {
        assert_eq!(
            classify("//hello"),
            InputClassification::Chat("hello".to_string())
        );
    }

    #[test]
    fn classify_empty() {
        assert_eq!(classify(""), InputClassification::Empty);
        assert_eq!(classify("   "), InputClassification::Empty);
    }

    #[test]
    fn paren_depth_balanced() {
        assert_eq!(paren_depth("(note c4 :q)"), 0);
        assert_eq!(paren_depth("(score (part (measure)))"), 0);
    }

    #[test]
    fn paren_depth_open() {
        assert_eq!(paren_depth("(score"), 1);
        assert_eq!(paren_depth("(score (part"), 2);
    }

    #[test]
    fn paren_depth_ignores_strings() {
        assert_eq!(paren_depth(r#"(note "(" :q)"#), 0);
        assert_eq!(paren_depth(r#"(title "Hello (World)")"#), 0);
    }

    #[test]
    fn paren_depth_escaped_quote_in_string() {
        assert_eq!(paren_depth(r#"(title "He said \"hi\"")"#), 0);
    }

    #[test]
    fn paren_depth_empty() {
        assert_eq!(paren_depth(""), 0);
        assert_eq!(paren_depth("hello"), 0);
    }
}
```

---

## Task 4: Command Dispatch (`src/repl/commands.rs`)

```rust
//! REPL command dispatch.
//!
//! Commands are prefixed with `:` in the REPL. This module parses the
//! command name and arguments, then dispatches to the appropriate handler.

/// Result of executing a REPL command.
#[derive(Debug)]
pub enum CommandResult {
    /// Print this text to the user.
    Output(String),
    /// Exit the REPL.
    Quit,
    /// Unknown command.
    Unknown(String),
}

/// Dispatch a command string (without the leading ':').
pub fn dispatch(input: &str) -> CommandResult {
    let trimmed = input.trim();
    let (cmd, args) = match trimmed.split_once(char::is_whitespace) {
        Some((c, a)) => (c, a.trim()),
        None => (trimmed, ""),
    };

    match cmd {
        "help" | "h" => handle_help(args),
        "quit" | "q" | "exit" => CommandResult::Quit,
        _ => CommandResult::Unknown(cmd.to_string()),
    }
}

fn handle_help(topic: &str) -> CommandResult {
    if topic.is_empty() {
        CommandResult::Output(GENERAL_HELP.to_string())
    } else {
        match topic {
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
  (expression)   Evaluate a Fermata expression
  :command        Execute a REPL command
  // message      Send a chat message (not yet connected)

Commands:
  :help [topic]   Show help
  :quit           Exit the REPL

Fermata expressions:
  (note c4 :q)                    A quarter-note middle C
  (note f#5 :e)                   An eighth-note F#5
  (chord (c4 e4 g4) :h)          A half-note C major chord
  (measure (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))

More commands will be available in future milestones:
  :session, :history, :notebook, :midi";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_help() {
        match dispatch("help") {
            CommandResult::Output(s) => assert!(s.contains("Fermata REPL")),
            other => panic!("Expected Output, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_help_topic() {
        match dispatch("help quit") {
            CommandResult::Output(s) => assert!(s.contains(":quit")),
            other => panic!("Expected Output, got {other:?}"),
        }
    }

    #[test]
    fn dispatch_quit() {
        assert!(matches!(dispatch("quit"), CommandResult::Quit));
        assert!(matches!(dispatch("q"), CommandResult::Quit));
        assert!(matches!(dispatch("exit"), CommandResult::Quit));
    }

    #[test]
    fn dispatch_unknown() {
        match dispatch("foobar") {
            CommandResult::Unknown(cmd) => assert_eq!(cmd, "foobar"),
            other => panic!("Expected Unknown, got {other:?}"),
        }
    }
}
```

---

## Task 5: Display Formatting (`src/repl/display.rs`)

```rust
//! Display formatting for REPL output.
//!
//! Handles formatting of evaluation results, errors, and informational messages.
//! ANSI color support is minimal at this stage (plain text); full coloring
//! arrives in Milestone 11 (6e).

use fermata::ir::score::ScorePartwise;

/// Format a successful evaluation result for display.
///
/// For now, produces a brief summary. Full pretty-printing comes with
/// session context (Milestone 2) where we can show the score delta.
pub fn format_eval_result(score: &ScorePartwise) -> String {
    let part_count = score.parts.len();
    let measure_count = score
        .parts
        .first()
        .map(|p| p.measures.len())
        .unwrap_or(0);

    let title = score
        .work
        .as_ref()
        .and_then(|w| w.work_title.as_deref())
        .unwrap_or("(untitled)");

    let mut parts = Vec::new();
    parts.push(format!("Score: {title}"));
    parts.push(format!(
        "  {part_count} part{}, {measure_count} measure{}",
        if part_count == 1 { "" } else { "s" },
        if measure_count == 1 { "" } else { "s" },
    ));

    for (i, part) in score.parts.iter().enumerate() {
        let name = score
            .part_list
            .as_ref()
            .and_then(|pl| pl.parts.get(i))
            .map(|sp| sp.part_name.as_str())
            .unwrap_or("(unnamed)");
        let measures = part.measures.len();
        parts.push(format!("  Part {}: {} ({} measures)", i + 1, name, measures));
    }

    parts.join("\n")
}

/// Format a compilation error for display.
pub fn format_compile_error(err: &fermata::lang::CompileError) -> String {
    format!("Error: {err}")
}

/// Format a "not yet implemented" stub message.
pub fn format_stub(feature: &str) -> String {
    format!("[{feature} is not yet available — coming in a future milestone]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use fermata::ir::score::ScorePartwise;

    #[test]
    fn format_empty_score() {
        let score = ScorePartwise::default();
        let output = format_eval_result(&score);
        assert!(output.contains("(untitled)"));
        assert!(output.contains("0 parts"));
    }

    #[test]
    fn format_stub_message() {
        let msg = format_stub("Chat mode");
        assert!(msg.contains("Chat mode"));
        assert!(msg.contains("not yet available"));
    }
}
```

---

## Task 6: REPL Loop (`src/repl/mod.rs` — full implementation)

```rust
//! REPL loop: read, classify, evaluate, display.

mod commands;
mod display;
mod error;
mod input;

pub use error::{ReplError, ReplResult};

use commands::CommandResult;
use input::InputClassification;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

const PRIMARY_PROMPT: &str = "fermata> ";
const CONTINUATION_PROMPT: &str = "      .. ";

/// The Fermata REPL.
pub struct Repl {
    editor: DefaultEditor,
    /// Accumulator for multi-line input.
    input_buffer: String,
    /// Tracks parenthesis depth for multi-line detection.
    paren_depth: i32,
}

impl Repl {
    /// Create a new REPL instance.
    pub fn new() -> ReplResult<Self> {
        let editor = DefaultEditor::new()?;

        Ok(Self {
            editor,
            input_buffer: String::new(),
            paren_depth: 0,
        })
    }

    /// Run the REPL loop until quit or EOF.
    pub fn run(&mut self) -> ReplResult<()> {
        self.print_banner();

        loop {
            let prompt = if self.input_buffer.is_empty() {
                PRIMARY_PROMPT
            } else {
                CONTINUATION_PROMPT
            };

            match self.editor.readline(prompt) {
                Ok(line) => {
                    if self.handle_line(&line)? {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // Ctrl-C: clear current multi-line buffer, re-prompt
                    if !self.input_buffer.is_empty() {
                        self.reset_input();
                        println!("(input cleared)");
                    }
                }
                Err(ReadlineError::Eof) => {
                    // Ctrl-D: exit
                    println!();
                    break;
                }
                Err(e) => return Err(ReplError::from(e)),
            }
        }

        Ok(())
    }

    /// Process a single line of input.
    ///
    /// Returns `true` if the REPL should exit.
    fn handle_line(&mut self, line: &str) -> ReplResult<bool> {
        // Accumulate into the multi-line buffer
        if !self.input_buffer.is_empty() {
            self.input_buffer.push('\n');
        }
        self.input_buffer.push_str(line);

        // Update paren depth
        self.paren_depth += input::paren_depth(line);

        // If parens are still unbalanced, continue reading
        if self.paren_depth > 0 {
            return Ok(false);
        }

        // We have a complete input — process it
        let complete_input = std::mem::take(&mut self.input_buffer);
        self.paren_depth = 0;

        // Add to rustyline history (raw line history)
        let _ = self.editor.add_history_entry(&complete_input);

        // Classify and process
        match input::classify(&complete_input) {
            InputClassification::Empty => {}

            InputClassification::Expression(expr) => {
                self.eval_expression(&expr);
            }

            InputClassification::Command(cmd) => {
                match commands::dispatch(&cmd) {
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

    /// Evaluate a Fermata expression and display the result.
    fn eval_expression(&self, source: &str) {
        match fermata::lang::compile(source) {
            Ok(score) => {
                println!("{}", display::format_eval_result(&score));
            }
            Err(e) => {
                println!("{}", display::format_compile_error(&e));
            }
        }
    }

    /// Reset the multi-line input buffer.
    fn reset_input(&mut self) {
        self.input_buffer.clear();
        self.paren_depth = 0;
    }

    /// Print the startup banner.
    fn print_banner(&self) {
        println!("Fermata REPL v{}", env!("CARGO_PKG_VERSION"));
        println!("Type :help for commands, :quit to exit.");
        println!();
    }
}

/// Evaluate a Fermata expression and return the result.
///
/// This is the library-facing API for integration tests. It bypasses
/// the interactive loop and directly compiles the expression.
pub fn eval_str(source: &str) -> Result<fermata::ir::score::ScorePartwise, ReplError> {
    Ok(fermata::lang::compile(source)?)
}
```

---

## Task 7: Integration Test

Create `crates/fermata-repl/tests/eval_integration.rs`:

```rust
//! Integration tests for REPL evaluation.
//!
//! These test the eval path without launching the interactive loop.

use fermata_repl::eval_str;

#[test]
fn eval_simple_score() {
    let source = r#"
        (score :title "Test"
          (part :name "Piano"
            (measure
              (note c4 :q)
              (note d4 :q)
              (note e4 :q)
              (note f4 :q))))
    "#;

    let score = eval_str(source).expect("should compile");
    assert_eq!(score.parts.len(), 1);
    assert_eq!(score.parts[0].measures.len(), 1);
}

#[test]
fn eval_error_gives_repl_error() {
    let source = "(note xyz :q)";
    let result = eval_str(source);
    assert!(result.is_err());
}

#[test]
fn eval_chord() {
    let source = r#"
        (score
          (part :name "Piano"
            (measure
              (chord (c4 e4 g4) :h)
              (chord (d4 f4 a4) :h))))
    "#;

    let score = eval_str(source).expect("should compile");
    assert_eq!(score.parts.len(), 1);
}
```

---

## Task 8: Input Classification and Multi-Line Tests

Create `crates/fermata-repl/tests/input_tests.rs`:

```rust
//! Tests for input classification and multi-line detection.

use fermata_repl::repl::input::{classify, paren_depth, InputClassification};

#[test]
fn multiline_scenario() {
    // Simulate typing a multi-line expression:
    // Line 1: "(score"       → depth 1, keep reading
    // Line 2: "  (part"      → depth 2, keep reading
    // Line 3: "    (measure" → depth 3, keep reading
    // Line 4: "      (note c4 :q))))" → depth ~0, complete

    let mut depth: i32 = 0;
    let lines = [
        "(score",
        "  (part :name \"Piano\"",
        "    (measure",
        "      (note c4 :q))))",
    ];

    for line in &lines {
        depth += paren_depth(line);
    }

    assert_eq!(depth, 0, "Should be balanced after all lines");
}

#[test]
fn colon_prefix_is_command() {
    assert!(matches!(
        classify(":session new test"),
        InputClassification::Command(_)
    ));
}

#[test]
fn double_slash_is_chat() {
    assert!(matches!(
        classify("// what tempo?"),
        InputClassification::Chat(_)
    ));
}

#[test]
fn open_paren_is_expression() {
    assert!(matches!(
        classify("(note c4 :q)"),
        InputClassification::Expression(_)
    ));
}

#[test]
fn bare_text_is_expression() {
    // Bare text (without parens or sigils) is treated as an expression.
    // The compiler will decide if it's valid.
    assert!(matches!(
        classify("c4"),
        InputClassification::Expression(_)
    ));
}
```

**Note:** For these tests to compile, `input.rs` types and functions must be re-exported. Add to `src/repl/mod.rs`:

```rust
pub mod input;
```

(Make `input` module public for test access. The `commands` and `display` modules can remain private.)

---

## Acceptance Criteria

1. ✅ `fermata-repl` crate exists in the workspace and builds (`cargo build -p fermata-repl`)
2. ✅ `cargo run -p fermata-repl` launches a REPL with a banner and prompt
3. ✅ Typing `(score (part :name "Piano" (measure (note c4 :q))))` produces a score summary
4. ✅ Typing an invalid expression (e.g., `(note xyz)`) shows a meaningful error
5. ✅ `:help` prints command reference
6. ✅ `:quit`, `:q`, `:exit`, Ctrl-D all exit cleanly
7. ✅ Multi-line input: typing `(score` produces a `..` continuation prompt; completing the expression evaluates it
8. ✅ Ctrl-C during multi-line input clears the buffer and returns to the primary prompt
9. ✅ `//` input is recognized as chat and shows a "not yet available" stub
10. ✅ Line editing works: arrow keys, Ctrl-A/E for home/end, Ctrl-R for history search
11. ✅ All unit tests pass (`cargo test -p fermata-repl`)
12. ✅ Integration tests pass
13. ✅ `fermata` crate is unchanged — no modifications to existing code

---

## Implementation Notes

1. **`rustyline` version:** Target v14.x. The API uses `DefaultEditor` (no custom `Helper` needed yet — tab completion comes in Milestone 11). If the version in crates.io has changed, adjust accordingly; the API has been stable across major versions.

2. **Multi-line detection is simple but sufficient.** Counting parentheses works for S-expressions because our only quoting mechanism is `"strings"`. There are no block comments, no heredocs, no alternative string literals. The `paren_depth` function handles the string-interior case.

3. **The compiler expects a complete expression.** The Phase 5 `compile()` function expects a full `(score ...)` form at the top level. For the REPL, users may also want to type bare sub-expressions like `(note c4 :q)` or even `(measure ...)`. Verify that the compiler handles these gracefully (it should, via the Phase 5 interpreter's top-level dispatch). If the compiler only accepts `(score ...)`, we may need to wrap bare expressions — but defer that to Milestone 2 where session context determines how to handle partial input.

4. **No sessions, no typed history.** This milestone deliberately avoids introducing sessions or typed history. The goal is to get the REPL loop working end-to-end with the compiler. Sessions arrive in Milestone 2; typed history in Milestone 3. The `rustyline` Editor has its own internal history buffer which provides Ctrl-R search and arrow-key recall for free.

5. **Public module for testing.** The `input` module is made public so that integration tests can directly test `classify` and `paren_depth`. The `commands` and `display` modules are implementation details and stay private.

6. **Error propagation.** `ReplError` wraps `ReadlineError`, `CompileError`, and `io::Error`. The REPL loop catches `CompileError` at the eval site and displays it without crashing. Only fatal errors (readline failure, I/O error) propagate up and terminate the REPL.

---

_Next: Milestone 2 — Session Model & Manager_
