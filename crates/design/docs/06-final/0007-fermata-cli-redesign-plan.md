---
number: 7
title: "Fermata CLI Redesign Plan"
author: "default with"
component: All
tags: [change-me]
created: 2026-02-02
updated: 2026-02-02
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Fermata CLI Redesign Plan

## Overview

Migrate from hand-rolled stdlib argument parsing to clap, and expand CLI capabilities with informational `show` commands and improved workflow commands.

## Dependencies

Add to `crates/fermata/Cargo.toml`:

```toml
clap = { version = "4", features = ["derive"] }
```

## CLI Structure

```
fermata [OPTIONS] <COMMAND>

Commands:
  compile    Compile Fermata source to output format
  check      Validate Fermata source without compiling
  import     Convert external formats to Fermata (future)
  fmt        Format Fermata source (future)
  repl       Start interactive REPL (future)
  show       Display reference information

Options:
  -v, --version    Print version
  -h, --help       Print help
  --no-color       Disable colored output
```

## Commands Detail

### `fermata compile`

```
fermata compile [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file (use '-' for stdin, omit for stdin)

Options:
  -o, --output <FILE>   Output file (omit for stdout)
  -t, --target <FMT>    Output format [default: musicxml]
                        Possible values: musicxml, xml, ly, lilypond
  -h, --help            Print help
```

**Stdin/stdout convention (following rustc):**

- No file arg or `-` = read from stdin
- No `-o` = write to stdout
- `-o -` = explicit stdout

### `fermata check`

```
fermata check [FILE]

Arguments:
  [FILE]  Input file (use '-' for stdin)
```

### `fermata show`

```
fermata show [OPTIONS] <TOPIC>

Options:
  --format <FMT>   Output format [default: text]
                   Possible values: text, json

Topics:
  targets        Supported output formats
  syntax         Quick syntax reference
  durations      Duration symbols (:w, :h, :q, :8, etc.)
  pitches        Pitch notation (C4, D#5, Bb3, etc.)
  clefs          Available clefs
  keys           Key signatures and modes
  dynamics       Dynamic markings (pp, p, mp, mf, f, ff, etc.)
  articulations  Articulation types
  ornaments      Ornament types
  instruments    Instrument shortcuts
  barlines       Barline types
  accidentals    Accidental variations
  noteheads      Notehead shapes
  fermatas       Fermata shapes
```

### `fermata import` (future)

```
fermata import [OPTIONS] <FILE>

Arguments:
  <FILE>  Input file to import

Options:
  -o, --output <FILE>   Output Fermata file
  --from <FORMAT>       Source format (auto-detected from extension)
                        Possible values: musicxml, xml
```

### `fermata fmt` (future)

```
fermata fmt [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file (use '-' for stdin)

Options:
  -o, --output <FILE>   Output file (omit to modify in place)
  --check               Check if formatted, don't modify
```

### `fermata repl` (future)

```
fermata repl [OPTIONS]

Options:
  --no-color    Disable colored output
```

## Implementation Plan

### Phase 1: Clap Migration (Core)

1. **Read clap guide** from `assets/ai/ai-rust/guides/09-cli.md`
2. **Add clap dependency** to Cargo.toml
3. **Define CLI structure** using derive macros:

   ```rust
   #[derive(Parser)]
   #[command(name = "fermata", version, about)]
   struct Cli {
       #[command(subcommand)]
       command: Commands,
   }

   #[derive(Subcommand)]
   enum Commands {
       Compile { ... },
       Check { ... },
       Show { ... },
   }
   ```

4. **Migrate compile command** - preserve existing behavior
5. **Migrate check command** - preserve existing behavior
6. **Update tests** if any CLI tests exist

### Phase 2: Show Commands

1. **Create `src/cli/show.rs`** module for show command handlers
2. **Implement `show targets`** - list output formats with descriptions
3. **Implement `show syntax`** - comprehensive quick reference
4. **Implement `show durations`** - all duration keywords with examples
5. **Implement `show dynamics`** - all dynamic markings
6. **Implement `show articulations`** - articulation types
7. **Implement `show ornaments`** - ornament types
8. **Implement `show clefs`** - available clefs
9. **Implement `show keys`** - key signatures and modes
10. **Implement `show instruments`** - instrument shortcuts
11. **Implement `show pitches`** - pitch notation guide
12. **Implement `show barlines`** - barline types
13. **Implement `show accidentals`** - accidental variations
14. **Implement `show noteheads`** - notehead shapes
15. **Implement `show fermatas`** - fermata shapes

### Phase 3: Enhanced Compile (Future)

1. **Add `--target` flag** with validation
2. **Implement LilyPond output** (stub exists in lib.rs)
3. **Add format aliases** (xml = musicxml, ly = lilypond)

### Phase 4: Additional Commands (Future)

1. **Implement `import`** - MusicXML to Fermata conversion
2. **Implement `fmt`** - source formatting
3. **Implement `repl`** - interactive mode

## Files to Modify

- `crates/fermata/Cargo.toml` - add clap dependency
- `crates/fermata/src/main.rs` - replace CLI implementation
- `crates/fermata/src/cli/mod.rs` - new module (optional organization)
- `crates/fermata/src/cli/show.rs` - show command implementations

## Files to Read First

- `assets/ai/ai-rust/guides/09-cli.md` - CLI best practices with clap
- `assets/ai/ai-rust/skills/claude/SKILL.md` - Rust development skill
- `crates/fermata/src/ir/` - enum definitions for show commands
- `crates/fermata/src/lang/` - language constructs for show commands

## Verification

1. `cargo build` - compiles without errors
2. `cargo test` - all tests pass
3. `fermata --version` - prints version
4. `fermata --help` - shows all commands
5. `fermata compile --help` - shows compile options
6. `fermata show targets` - lists output formats
7. `fermata show durations` - lists duration symbols
8. `fermata compile score.fm -o out.xml` - produces MusicXML
9. `echo '(score ...)' | fermata compile` - stdin works
10. `fermata check score.fm` - validates without output

## Design Decisions

1. **Colorized output**: Yes, `show` commands will use colors by default with `--no-color` global flag to disable
2. **Machine-readable output**: Yes, support `--format json` option for tooling integration
3. **Topic grouping**: Defer - start with flat topic list, consider grouping later if needed

## Additional Dependencies

```toml
# For colorized terminal output (same as twyg logging library)
owo-colors = "4"
serde_json = "1" # for --format json output
```
