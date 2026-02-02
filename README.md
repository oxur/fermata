# 𝄐 Fermata

[![][build-badge]][build]
[![][crate-badge]][crate]
[![][tag-badge]][tag]
[![][docs-badge]][docs]

[![][logo]][logo-large]

**An S-expression DSL for working with MusicXML**

> *In music, a fermata (𝄐) indicates that a note should be held longer than its written value —
> a pause, a moment of expressiveness left to the performer's discretion.*

Fermata is a Lisp-like domain-specific language for describing musical notation.

## Example

```lisp
;; A simple C major scale
(score
  :title "C Major Scale"
  :composer "Anonymous"
  (part :id "P1" :name "Piano"
    (measure
      :time (4 4)
      :clef :treble
      :key 0
      (note C4 :q)
      (note D4 :q)
      (note E4 :q)
      (note F4 :q))
    (measure
      (note G4 :q)
      (note A4 :q)
      (note B4 :q)
      (note C5 :q))))
```

## Features

### Implemented

| Feature | Description |
|---------|-------------|
| **Compile** | Fermata source → MusicXML |
| **Import** | MusicXML → Fermata source |
| **Check** | Validate Fermata source files |
| **Show** | Built-in reference for durations, pitches, clefs, dynamics, etc. |
| **MusicXML Parser** | Full MusicXML 4.0 parsing |
| **MusicXML Emitter** | Full MusicXML 4.0 generation |

### Planned

| Feature | Description |
|---------|-------------|
| **LilyPond** | Compile to LilyPond for publication-quality PDF engraving |
| **REPL** | Interactive prompt for experimentation |
| **Macros** | Define reusable patterns like `(cadence :authentic :key c-major)` |
| **Transformations** | `(transpose +2 ...)`, `(invert ...)`, `(retrograde ...)` |
| **Theory-aware** | Built-in knowledge of scales, chords, intervals, voice leading |
| **SVG** | Direct rendering via [verovioxide](https://github.com/oxur/verovioxide) |

## Installation

```bash
cargo install fermata
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
fermata = "0.1"
```

## Usage

### CLI Commands

```bash
# Compile Fermata source to MusicXML
fermata compile score.fm -o score.musicxml

# Import MusicXML to Fermata source
fermata import score.musicxml -o score.fm

# Validate a Fermata file
fermata check score.fm

# Show reference information
fermata show durations
fermata show pitches
fermata show dynamics
fermata show --help  # list all topics

# Machine-readable output
fermata show durations --format json
```

### Stdin/Stdout Support

```bash
# Read from stdin, write to stdout
cat score.fm | fermata compile > score.musicxml

# Convert MusicXML from stdin
cat score.musicxml | fermata import > score.fm
```

### As a Library

```rust,ignore
use fermata::{parse, compile, CompileOptions, Target};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse Fermata source to AST
    let source = r#"
        (score :title "My Song"
          (part :piano
            (measure
              (note c4 :q)
              (note d4 :q)
              (note e4 :q)
              (note f4 :q))))
    "#;
    let score = parse(source)?;

    // Compile to MusicXML (default)
    let xml = compile(&score, CompileOptions::default())?;
    std::fs::write("output.musicxml", &xml)?;

    // Or compile to S-expression format
    let sexpr = compile(&score, CompileOptions::sexpr())?;
    println!("{}", sexpr);

    Ok(())
}
```

For lower-level control, use the modules directly:

- `fermata::lang` - Language parsing and compilation
- `fermata::musicxml` - MusicXML parsing and emission
- `fermata::sexpr` - S-expression parsing and printing
- `fermata::ir` - MusicXML-faithful intermediate representation

## Language Reference

Use `fermata show` to explore the language:

```bash
fermata show syntax        # Quick syntax overview
fermata show durations     # :w :h :q :8 :16 :32 :64
fermata show pitches       # C4, D#5, Bb3, etc.
fermata show dynamics      # pp, p, mp, mf, f, ff, etc.
fermata show articulations # staccato, accent, tenuto, etc.
fermata show clefs         # treble, bass, alto, tenor, etc.
fermata show keys          # Key signatures and modes
```

### Quick Reference

```lisp
;; Notes with pitch and duration
(note C4 :q)              ; C4 quarter note
(note D#5 :h)             ; D#5 half note
(note Bb3 :8)             ; Bb3 eighth note

;; Rests
(rest :q)                 ; quarter rest
(rest :w)                 ; whole rest

;; Chords
(chord :q C4 E4 G4)       ; C major triad, quarter

;; Durations
:w                        ; whole
:h                        ; half
:q                        ; quarter
:8                        ; eighth
:16                       ; sixteenth
:32                       ; thirty-second

;; Score structure
(score
  :title "Title"
  :composer "Composer"
  (part :id "P1" :name "Piano"
    (measure
      :time (4 4)
      :clef :treble
      :key 0                ; 0 = C major, 1 = G major, -1 = F major
      (note C4 :q)
      ...)))

;; Dynamics (planned)
:pp :p :mp :mf :f :ff

;; Articulations (planned)
:staccato :accent :tenuto :fermata

;; Theory macros (planned)
(scale c :major)          ; => (c4 d4 e4 f4 g4 a4 b4 c5)
(chord-voicing c :dom7)   ; => (c4 e4 g4 bb4)
```

## Related Projects

- [verovioxide](https://github.com/oxur/verovioxide) — Rust bindings for Verovio music engraving
- [Verovio](https://www.verovio.org/) — Fast, lightweight music notation engraving library
- [LilyPond](https://lilypond.org/) — Music engraving program for publication-quality scores

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

*"Music is the space between the notes."* — Claude Debussy

[//]: ---Named-Links---

[logo]: assets/images/logo/v1-x250.png
[logo-large]: assets/images/logo/v1.png
[build]: https://github.com/oxur/fermata/actions/workflows/cicd.yml
[build-badge]: https://github.com/oxur/fermata/actions/workflows/cicd.yml/badge.svg
[crate]: https://crates.io/crates/fermata
[crate-badge]: https://img.shields.io/crates/v/fermata.svg
[docs]: https://docs.rs/fermata/
[docs-badge]: https://img.shields.io/badge/rust-documentation-blue.svg
[tag-badge]: https://img.shields.io/github/tag/oxur/fermata.svg
[tag]: https://github.com/oxur/fermata/tags
