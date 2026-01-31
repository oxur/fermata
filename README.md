# 𝄐 Fermata

**An S-expression DSL for working with MusicXML**

> *In music, a fermata (𝄐) indicates that a note should be held longer than its written value —
> a pause, a moment of expressiveness left to the performer's discretion.*

Fermata is a Lisp-like domain-specific language for describing musical notation. It compiles to
MusicXML, LilyPond, and Rust, enabling precise musical communication between humans and machines.

## Status: 🚧 Under Construction

This crate is in early development. The API will change significantly.

## Vision

```lisp
;; Describe music naturally
(score
 (title "Prelude in C")
 (composer "J.S. Bach")
 (tempo 66 :quarter)

 (part :piano
       (staff :treble
              (measure
               (voice
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5))
                (chord :8 (c5 e5 g5)))))
       (staff :bass
              (measure
               (voice
                (note c3 :h)
                (note c4 :h))))))
```

### Output Targets

| Target | Use Case |
|--------|----------|
| **MusicXML** | Import into Finale, Sibelius, MuseScore, Dorico |
| **LilyPond** | Publication-quality PDF engraving |
| **Rust** | Embed notation generation in Rust applications |
| **SVG** | Direct rendering via [verovioxide](https://github.com/oxur/verovioxide) |

### Planned Features

- **REPL** — Interactive `fermata>` prompt for experimentation
- **Macros** — Define reusable patterns like `(cadence :authentic :key c-major)`
- **Transformations** — `(transpose +2 ...)`, `(invert ...)`, `(retrograde ...)`
- **Bidirectional** — Parse MusicXML back into Fermata expressions
- **Theory-aware** — Built-in knowledge of scales, chords, intervals, voice leading

## Installation

```bash
cargo install fermata
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
fermata = "0.0.1"
```

## Usage

### As a CLI

```bash
# Start the REPL
fermata

# Compile to MusicXML
fermata compile score.fm -o score.musicxml

# Compile to LilyPond
fermata compile score.fm -o score.ly --target lilypond

# Render directly to SVG (requires verovioxide)
fermata render score.fm -o score.svg
```

### As a Library

```rust
use fermata::{parse, compile_to_musicxml};

fn main() -> fermata::Result<()> {
let source = r#"
(score
 (part :piano
       (measure
        (note c4 :q)
        (note d4 :q)
        (note e4 :q)
        (note f4 :q))))
"#;

let ast = parse(source)?;
let musicxml = compile_to_musicxml(&ast)?;

std::fs::write("output.musicxml", musicxml)?;
Ok(())
}
```

## Language Reference

*Coming soon — see [LANGUAGE.md](LANGUAGE.md) for the developing specification.*

### Quick Reference

```lisp
;; Notes
(note c4 :q) ; C4 quarter note
(note d#5 :h :dot) ; D#5 dotted half
(note bb3 :8 :staccato) ; Bb3 eighth, staccato

;; Rests
(rest :q) ; quarter rest
(rest :w) ; whole rest

;; Chords
(chord :q (c4 e4 g4)) ; C major triad, quarter

;; Durations
:w ; whole
:h ; half
:q ; quarter
:8 ; eighth
:16 ; sixteenth
:32 ; thirty-second

;; Articulations & Ornaments
:staccato :accent :tenuto :fermata
:trill :mordent :turn

;; Dynamics
:pp :p :mp :mf :f :ff
(cresc :from pp :to f :over 4) ; 4 measures

;; Structure
(score ...)
(part <instrument> ...)
(staff <clef> ...)
(measure ...)
(voice ...)

;; Clefs
:treble :bass :alto :tenor :percussion

;; Key signatures
(key c :major)
(key a :minor)
(key f# :major)

;; Time signatures
(time 4 4)
(time 6 8)
(time 2 2)

;; Theory macros (planned)
(scale c :major) ; => (c4 d4 e4 f4 g4 a4 b4 c5)
(chord-voicing c :dom7) ; => (c4 e4 g4 bb4)
(cadence :authentic :key c) ; => V-I progression
(progression c :major I IV V I)
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
