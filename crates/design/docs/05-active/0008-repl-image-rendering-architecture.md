---
number: 8
title: "REPL Image Rendering Architecture"
author: "Duncan McGreggor"
component: All
tags: [change-me]
created: 2026-02-02
updated: 2026-02-02
state: Active
supersedes: null
superseded-by: null
version: 1.0
---

# REPL Image Rendering Architecture

## Core Insight

**Value and display are separate concerns.** In a Lisp REPL:

- `(+ 1 2)` produces VALUE `3`, which is then DISPLAYED
- The evaluation and printing are distinct phases

For Fermata:

- `(note c4 :q)` produces VALUE `ScorePartwise` IR
- DISPLAY could be: S-expr, MusicXML, PNG (rendered notation)

**The stdout analogy**: Rendering PNG in-terminal is like printing to stdout - it's the visual representation of the data, not the data itself.

## Proposed Architecture

### Display Modes

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayMode {
    #[default]
    Sexpr,      // Structural, debugging (current behavior)
    MusicXml,   // Interchange format
    Png,        // Rendered notation via viuer
    Silent,     // Store value, no output
}
```

### Render Options (Builder Pattern)

```rust
pub struct RenderOptions {
    width: u32,          // Default: 800
    page: u32,           // Default: 1 (1-indexed)
    show_page_info: bool // Default: true
}
```

### Session State (Lisp-style History)

```rust
pub struct ReplSession {
    display_mode: DisplayMode,
    render_options: RenderOptions,

    // Results: *, **, *** (last 3 evaluated values)
    results: [Option<ScorePartwise>; 3],   // [0]=*, [1]=**, [2]=***

    // Expressions: +, ++, +++ (last 3 input expressions)
    expressions: [Option<String>; 3],       // [0]=+, [1]=++, [2]=+++
}

impl ReplSession {
    /// Rotate history and store new result
    pub fn push_result(&mut self, result: ScorePartwise) {
        self.results[2] = self.results[1].take();  // *** = old **
        self.results[1] = self.results[0].take();  // ** = old *
        self.results[0] = Some(result);            // * = new
    }

    /// Rotate history and store new expression
    pub fn push_expression(&mut self, expr: String) {
        self.expressions[2] = self.expressions[1].take();
        self.expressions[1] = self.expressions[0].take();
        self.expressions[0] = Some(expr);
    }

    /// Get result by symbol: "*", "**", "***"
    pub fn get_result(&self, sym: &str) -> Option<&ScorePartwise> {
        match sym {
            "*" => self.results[0].as_ref(),
            "**" => self.results[1].as_ref(),
            "***" => self.results[2].as_ref(),
            _ => None,
        }
    }

    /// Get expression by symbol: "+", "++", "+++"
    pub fn get_expression(&self, sym: &str) -> Option<&str> {
        match sym {
            "+" => self.expressions[0].as_deref(),
            "++" => self.expressions[1].as_deref(),
            "+++" => self.expressions[2].as_deref(),
            _ => None,
        }
    }
}
```

### History Variable Usage

These are **special symbols** that resolve to stored values - no re-evaluation happens.

| Symbol | Type | Returns |
|--------|------|---------|
| `*` | Value | Last evaluated result (ScorePartwise) |
| `**` | Value | Second-to-last result |
| `***` | Value | Third-to-last result |
| `+` | Sexpr | Last expression as data (AST, unevaluated) |
| `++` | Sexpr | Second-to-last expression as data |
| `+++` | Sexpr | Third-to-last expression as data |

**Example session:**

```lisp
fermata> (note c4 :q)
=> <ScorePartwise displayed>

fermata> *              ;; Returns the stored value
=> <same ScorePartwise>

fermata> ++             ;; Returns the expression AS DATA
=> (note c4 :q)

fermata> (transpose * 2) ;; Use last result in new expression
=> <transposed ScorePartwise>
```

**Implementation:** These are resolved during evaluation as special symbols, not via string substitution.

## UX Commands

| Command | Description |
|---------|-------------|
| `:set display <mode>` | Set global display mode (sexpr/xml/png/silent) |
| `:render` | Render last result as PNG |
| `:render page N` | Render specific page |
| `:render width N` | Render with custom width |
| `:export <file>` | Export to file (format from extension) |
| `:settings` | Show current settings |

## Rendering Pipeline

```
ScorePartwise (VALUE)
       ↓
   Renderable trait
       ↓
  ┌────┴────┬────────────┐
  ↓         ↓            ↓
render_   render_     render_png()
sexpr()   musicxml()   [feature-gated]
  ↓         ↓            ↓
String    String     verovioxide
                         ↓
                      PNG bytes
                         ↓
                    viuer::print()
                         ↓
                    Terminal display
```

## Feature Gating

```toml
[features]
default = []
render = ["dep:verovioxide", "dep:viuer", "dep:image"]

[dependencies]
viuer = { version = "0.7", optional = true }
image = { version = "0.25", optional = true }
verovioxide = { version = "...", optional = true }
```

## Design Decisions

1. **Default display mode**: `Sexpr` (explicit `:render` for PNG)
2. **History variables**: Lisp-style `*/**/***/+/++/+++` as special symbols
3. **Auto-render**: Yes, when display mode is Png
4. **Pagination**: First page + "Page 1 of N" indicator, `:render page N` for others
5. **Symbol resolution**: Special symbols resolved during evaluation (not string substitution)
6. **Terminal fallback**: Detect lack of image support, warn once, then use block characters

## Implementation Order

### Phase 1: Session State & History

1. Add `ReplSession` struct with history arrays (`results[3]`, `expressions[3]`)
2. Update REPL loop to store results/expressions after each evaluation
3. Add special symbol resolution for `*/**/***/+/++/+++` in evaluator

### Phase 2: Display Modes

4. Add `DisplayMode` enum (Sexpr, MusicXml, Png, Silent)
2. Add `RenderOptions` struct with builder pattern
3. Add `:set display <mode>` command
4. Add `:settings` command

### Phase 3: Rendering (feature-gated)

8. Add `render` feature flag to Cargo.toml
2. Add viuer + image dependencies (optional)
3. Implement terminal capability detection with one-time warning
4. Add `:render` command with page/width options
5. Implement auto-render when display mode is Png

### Phase 4: Export & Polish

13. Add `:export <file>` command (PNG, SVG, MusicXML based on extension)
2. Add page count and "Page 1 of N" indicator
3. Add `:render page N` for pagination

## Files to Modify

| File | Changes |
|------|---------|
| `Cargo.toml` | Add `render` feature, optional viuer/image deps |
| `repl/mod.rs` | Add ReplSession with history, update eval loop |
| `repl/session.rs` | **NEW** - ReplSession, history management, symbol resolution |
| `repl/display.rs` | Add DisplayMode, RenderOptions, display_result() |
| `repl/render.rs` | **NEW** - viuer integration, terminal detection (feature-gated) |
| `repl/commands.rs` | Add :set, :render, :export, :settings commands |
| `repl/error.rs` | Add RenderError variants |
| `lang/eval.rs` or similar | Hook for resolving `*/**/***/+/++/+++` symbols |

## Verification

1. `cargo build` - builds without render feature
2. `cargo build --features render` - builds with rendering
3. `:set display png` + expression - shows rendered notation
4. `:render` - renders last result
5. `:export test.png` - creates PNG file
