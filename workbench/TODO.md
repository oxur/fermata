# TODO

## Active

## Backlog

## Completed

- [x] Refactor crates/fermata into multiple crates -- we pivoted here to a holistic view for the future of our crates, documented in `0006 │ Fermata Crate Architecture`
- [x] Test coverage
- [x] CI/CD .github/* dir setup

```
⏺ Here's my analysis of README vs Reality:

  Discrepancies Found

  1. Output Targets
  ┌─────────────────────┬────────────────────┐
  │    README Claims    │      Reality       │
  ├─────────────────────┼────────────────────┤
  │ MusicXML ✓          │ ✓ Implemented      │
  ├─────────────────────┼────────────────────┤
  │ LilyPond            │ ❌ todo!() stub    │
  ├─────────────────────┼────────────────────┤
  │ Rust code gen       │ ❌ Not implemented │
  ├─────────────────────┼────────────────────┤
  │ SVG via verovioxide │ ❌ Not implemented │
  └─────────────────────┴────────────────────┘
  2. CLI Commands
  ┌───────────────────────────────────────┬─────────────────────────┐
  │             README Claims             │         Reality         │
  ├───────────────────────────────────────┼─────────────────────────┤
  │ fermata (starts REPL)                 │ ❌ Shows help only      │
  ├───────────────────────────────────────┼─────────────────────────┤
  │ fermata compile ... --target lilypond │ ❌ No --target option   │
  ├───────────────────────────────────────┼─────────────────────────┤
  │ fermata render ... -o score.svg       │ ❌ No render command    │
  ├───────────────────────────────────────┼─────────────────────────┤
  │ fermata compile                       │ ✓ Works                 │
  ├───────────────────────────────────────┼─────────────────────────┤
  │ fermata check                         │ ✓ Works (not in README) │
  └───────────────────────────────────────┴─────────────────────────┘
  3. Library API
  ┌────────────────────────────────┬───────────────────────────────────┐
  │         README Claims          │              Reality              │
  ├────────────────────────────────┼───────────────────────────────────┤
  │ fermata::parse()               │ ❌ todo!() stub                   │
  ├────────────────────────────────┼───────────────────────────────────┤
  │ fermata::compile_to_musicxml() │ ❌ todo!() stub                   │
  ├────────────────────────────────┼───────────────────────────────────┤
  │ —                              │ ✓ fermata::lang::compile() works  │
  ├────────────────────────────────┼───────────────────────────────────┤
  │ —                              │ ✓ fermata::musicxml::emit() works │
  └────────────────────────────────┴───────────────────────────────────┘
  4. Syntax

  The example syntax (note c4 :q) appears correct based on the lang module.

  Options

  1. Update README to match current implementation - Remove/demote unimplemented
  features to "Planned"
  2. Implement the stub functions - Wire parse()/compile_to_musicxml() to the
  working code
  3. Both - Update README and add thin wrappers to the working code

  Which approach would you prefer?
```
