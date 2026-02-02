# High-Level Library API Plan

## Overview

Implement a clean, user-facing library API with:

- `fermata::parse()` - Parse Fermata source to AST
- `fermata::compile()` - Compile AST to various output formats

## Current State

The codebase has three AST layers:

1. **`ast::*` in lib.rs** - Unused stubs (Score, Part, Note, etc.)
2. **`lang::ast::*`** - Working Fermata AST (FermataScore, FermataPart, etc.)
3. **`ir::*`** - MusicXML-faithful IR (ScorePartwise, etc.)

Current internal flow:

```
Source → sexpr::parse() → Sexpr → interpret → FermataScore → compile → ScorePartwise → emit → XML
```

## Design Decision

**Use the working `lang::ast` types** rather than the unused `lib.rs` stubs:

- Already implemented and tested
- Integrated with compilation pipeline
- Represents actual Fermata DSL semantics

The `ast::*` types in lib.rs can be removed or repurposed later.

## API Design

### `fermata::parse()`

```rust
/// Parse Fermata source code into an AST.
pub fn parse(source: &str) -> Result<Score> {
    let sexpr = sexpr::parser::parse(source)?;
    lang::score::parse_score_from_sexpr(&sexpr)
}
```

Where `Score` is a re-export of `lang::ast::FermataScore`.

### `fermata::compile()`

```rust
/// Output format for compilation.
#[derive(Debug, Clone, Copy, Default)]
pub enum Target {
    #[default]
    MusicXml,
    Sexpr,      // For debugging/round-trip
    // LilyPond, // Future
}

/// Options for compilation.
#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    pub target: Target,
}

/// Compile an AST to the specified output format.
pub fn compile(score: &Score, options: CompileOptions) -> Result<String> {
    match options.target {
        Target::MusicXml => {
            let ir = lang::score::compile_fermata_score(score)?;
            musicxml::emit(&ir)
        }
        Target::Sexpr => {
            let ir = lang::score::compile_fermata_score(score)?;
            Ok(sexpr::print_sexpr(&ir.to_sexpr()))
        }
    }
}

// Convenience function
pub fn compile_to(score: &Score, target: Target) -> Result<String> {
    compile(score, CompileOptions { target })
}
```

### Re-exports

```rust
// In lib.rs
pub use lang::ast::{
    FermataScore as Score,
    FermataPart as Part,
    FermataMeasure as Measure,
    MeasureElement,
    FermataNote as Note,
    FermataRest as Rest,
    FermataChord as Chord,
    FermataPitch as Pitch,
    FermataDuration as Duration,
    // etc.
};

pub use Target;
pub use CompileOptions;
```

### Example Usage

```rust
use fermata::{parse, compile, Target, CompileOptions};

fn main() -> fermata::Result<()> {
    let source = r#"
        (score :title "Test"
          (part :piano
            (measure (note c4 :q))))
    "#;

    // Parse to AST
    let score = parse(source)?;

    // Compile to MusicXML (default)
    let xml = compile(&score, CompileOptions::default())?;

    // Or use convenience function
    let xml = fermata::compile_to(&score, Target::MusicXml)?;

    // Compile to S-expression (for debugging)
    let sexpr = fermata::compile_to(&score, Target::Sexpr)?;

    Ok(())
}
```

## Files to Modify

1. **`crates/fermata/src/lib.rs`**
   - Add `Target` enum
   - Add `CompileOptions` struct
   - Implement `parse()` function
   - Implement `compile()` and `compile_to()` functions
   - Add re-exports for AST types
   - Remove or deprecate unused `ast` module stubs

2. **`crates/fermata/src/lang/mod.rs`**
   - Ensure `ast` module is public
   - Ensure `score::parse_score_from_sexpr` is public

3. **`crates/fermata/src/lang/score.rs`**
   - Ensure `parse_score_from_sexpr` is public (may already be)

## Cleanup

**Decision: Remove the unused `ast` module entirely.**

This includes removing ~320 lines from lib.rs:

- `pub mod ast { ... }` block (lines 39-363)
- `pub use ast::*;` re-export
- Related `error` module if only used by unused functions
- Stub functions: `parse()`, `compile_to_musicxml()`, `compile_to_lilypond()`, `parse_musicxml()`, `format()`

## Verification

1. `cargo build` - compiles without errors
2. `cargo test` - all tests pass
3. Add doc tests for new API:

   ```rust
   /// ```
   /// let score = fermata::parse("(score (part :piano (measure (note c4 :q))))")?;
   /// let xml = fermata::compile(&score, Default::default())?;
   /// ```
   ```

4. Update README example to use new API
5. Verify existing CLI still works (uses internal functions)

## Future Extensions

- `Target::LilyPond` - When LilyPond emitter is implemented
- `CompileOptions::pretty` - Pretty-print output
- `CompileOptions::validate` - Run validation before compile
- `fermata::format()` - Format/pretty-print source code
