# Phase 5 — Milestone 5: Score Assembly & CLI

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Measure, part, score compilation; top-level pipeline; CLI tool; end-to-end tests
> **Depends On:** Milestones 1-4
> **Estimated Implementation Time:** 2-3 hours

---

## Overview

This milestone assembles the complete compilation pipeline. Every prior milestone
built isolated compilers for notes, chords, tuplets, attributes, and directions.
This milestone wires them together:

1. **Measure compilation** — dispatch each child form, collect into `Measure`
2. **Part compilation** — sequence measures, generate part metadata
3. **Score compilation** — assemble parts, part-list, identification
4. **Top-level interpreter** — sexpr → `FermataScore` AST
5. **Compiler orchestration** — `FermataScore` AST → `ScorePartwise` IR
6. **CLI binary** — `fermata compile input.ferm -o output.xml`
7. **End-to-end tests** — "Twinkle Twinkle" round-trip

---

## Task 1: Measure Compilation (`src/fermata/measure.rs`)

Measures dispatch each child S-expression to the appropriate sub-compiler,
collecting results into the IR `Measure` type. Attributes (key, time, clef)
appearing anywhere in the measure are gathered into a single `Attributes` block
emitted first, followed by directions and notes in source order.

```rust
//! Measure compilation for Fermata syntax.
//!
//! Compiles `(measure ...)` forms to IR `Measure`.

use crate::ir::measure::{Measure, MusicDataElement};
use crate::ir::attributes::Attributes;
use crate::ir::voice::{Backup, Forward};
use crate::sexpr::Sexpr;
use super::ast::{FermataMeasure, MeasureElement, BarlineSpec};
use super::error::{CompileError, CompileResult};
use super::defaults::DEFAULT_DIVISIONS;

/// Compile a measure S-expression.
///
/// Syntax:
/// ```lisp
/// (measure
///   (time 4 4)
///   (key c :major)
///   (clef :treble)
///   (note c4 :q)
///   (note d4 :q)
///   (note e4 :q)
///   (note f4 :q))
/// ```
///
/// Attributes (key, time, clef) are gathered into one `Attributes` block
/// and placed first. All other elements are compiled in source order.
pub fn compile_measure(sexpr: &Sexpr, number: u32) -> CompileResult<Measure> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("measure list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("measure") {
        return Err(CompileError::UnknownForm("expected (measure ...)".to_string()));
    }

    let fermata_measure = parse_measure_form(&list[1..], number)?;
    compile_fermata_measure(&fermata_measure)
}

/// Parse child S-expressions into `FermataMeasure` AST.
fn parse_measure_form(args: &[Sexpr], number: u32) -> CompileResult<FermataMeasure> {
    let mut content = Vec::new();

    for arg in args {
        if let Some(element) = classify_measure_element(arg)? {
            content.push(element);
        }
    }

    Ok(FermataMeasure {
        number: Some(number),
        content,
    })
}

/// Classify and parse a single child form inside a measure.
///
/// Returns `None` for unrecognized forms (silently skipped).
fn classify_measure_element(sexpr: &Sexpr) -> CompileResult<Option<MeasureElement>> {
    let list = match sexpr.as_list() {
        Some(l) => l,
        None => return Ok(None), // Bare atoms inside measure are ignored
    };

    let head = match list.first().and_then(|s| s.as_symbol()) {
        Some(h) => h,
        None => return Ok(None),
    };

    match head {
        // Notes & rests
        "note" => {
            let ast = super::note::parse_note_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Note(ast)))
        }
        "rest" => {
            let ast = super::note::parse_rest_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Rest(ast)))
        }

        // Compound structures
        "chord" => {
            let ast = super::chord::parse_chord_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Chord(ast)))
        }
        "tuplet" => {
            let ast = super::tuplet::parse_tuplet_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Tuplet(ast)))
        }
        "grace" | "grace-note" => {
            let ast = super::note::parse_grace_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::GraceNote(ast)))
        }

        // Attributes
        "key" => {
            let spec = super::attributes::parse_key_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Key(spec)))
        }
        "time" => {
            let spec = super::attributes::parse_time_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Time(spec)))
        }
        "clef" => {
            let spec = super::attributes::parse_clef_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Clef(spec)))
        }

        // Barlines
        "barline" => {
            let spec = parse_barline_form(&list[1..])?;
            Ok(Some(MeasureElement::Barline(spec)))
        }

        // Directions: dynamics
        name if is_dynamic_head(name) => {
            let mark = super::direction::parse_dynamic_to_ast(name)?;
            Ok(Some(MeasureElement::Dynamic(mark)))
        }

        // Directions: tempo
        "tempo" => {
            let mark = super::direction::parse_tempo_form_to_ast(&list[1..])?;
            Ok(Some(MeasureElement::Tempo(mark)))
        }

        // Voice management
        "backup" => {
            let dur = parse_duration_value(&list[1..])?;
            Ok(Some(MeasureElement::Backup(dur)))
        }
        "forward" => {
            let dur = parse_duration_value(&list[1..])?;
            Ok(Some(MeasureElement::Forward(dur)))
        }

        // Unknown forms are silently skipped
        _ => Ok(None),
    }
}

fn is_dynamic_head(name: &str) -> bool {
    matches!(name.to_lowercase().as_str(),
        "pppppp" | "ppppp" | "pppp" | "ppp" | "pp" | "p" |
        "mp" | "mf" |
        "f" | "ff" | "fff" | "ffff" | "fffff" | "ffffff" |
        "sf" | "sfp" | "sfpp" | "sfz" | "sffz" | "sfzp" |
        "fp" | "fz" | "pf" | "rf" | "rfz" | "n" |
        "cresc" | "crescendo" | "dim" | "diminuendo" | "decresc"
    )
}

fn parse_barline_form(args: &[Sexpr]) -> CompileResult<BarlineSpec> {
    if args.is_empty() {
        return Ok(BarlineSpec::Regular);
    }

    let name = args[0].as_keyword()
        .or_else(|| args[0].as_symbol())
        .ok_or_else(|| CompileError::type_mismatch("barline type", format!("{:?}", args[0])))?;

    match name {
        "regular" => Ok(BarlineSpec::Regular),
        "double" => Ok(BarlineSpec::Double),
        "final" | "end" => Ok(BarlineSpec::Final),
        "repeat-forward" | "forward" => Ok(BarlineSpec::RepeatForward),
        "repeat-backward" | "backward" => Ok(BarlineSpec::RepeatBackward),
        "repeat-both" | "both" => Ok(BarlineSpec::RepeatBoth),
        _ => Err(CompileError::UnknownForm(format!("unknown barline: {}", name))),
    }
}

fn parse_duration_value(args: &[Sexpr]) -> CompileResult<u32> {
    args.first()
        .and_then(|s| s.as_symbol())
        .and_then(|s| s.parse::<u32>().ok())
        .ok_or(CompileError::MissingField("duration value"))
}

// ─── IR Compilation ─────────────────────────────────────────────────────────

/// Compile a `FermataMeasure` AST to IR `Measure`.
///
/// Strategy:
/// 1. Gather all attribute elements (key, time, clef)
/// 2. If any attributes found, emit a single `Attributes` block first
/// 3. Compile remaining elements in source order
pub fn compile_fermata_measure(measure: &FermataMeasure) -> CompileResult<Measure> {
    let mut ir_content: Vec<MusicDataElement> = Vec::new();

    // ── Phase 1: Gather attributes ──────────────────────────────────────
    let mut keys = Vec::new();
    let mut times = Vec::new();
    let mut clefs = Vec::new();
    let mut has_attributes = false;

    for element in &measure.content {
        match element {
            MeasureElement::Key(spec) => {
                keys.push(super::attributes::compile_key_spec(spec)?);
                has_attributes = true;
            }
            MeasureElement::Time(spec) => {
                times.push(super::attributes::compile_time_spec(spec)?);
                has_attributes = true;
            }
            MeasureElement::Clef(spec) => {
                clefs.push(super::attributes::compile_clef_spec(spec)?);
                has_attributes = true;
            }
            _ => {}
        }
    }

    if has_attributes {
        ir_content.push(MusicDataElement::Attributes(Attributes {
            divisions: Some(DEFAULT_DIVISIONS),
            keys,
            times,
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs,
            staff_details: Vec::new(),
            transposes: Vec::new(),
            directives: Vec::new(),
            measure_styles: Vec::new(),
        }));
    }

    // ── Phase 2: Compile non-attribute elements in source order ─────────
    for element in &measure.content {
        match element {
            // Skip attributes (already handled above)
            MeasureElement::Key(_) | MeasureElement::Time(_) | MeasureElement::Clef(_) => {}

            // Notes, rests
            MeasureElement::Note(ast) => {
                let note = super::note::compile_fermata_note(ast)?;
                ir_content.push(MusicDataElement::Note(note));
            }
            MeasureElement::Rest(ast) => {
                let note = super::note::compile_fermata_rest(ast)?;
                ir_content.push(MusicDataElement::Note(note));
            }

            // Compound structures
            MeasureElement::Chord(ast) => {
                let notes = super::chord::compile_fermata_chord(ast)?;
                for note in notes {
                    ir_content.push(MusicDataElement::Note(note));
                }
            }
            MeasureElement::Tuplet(ast) => {
                let notes = super::tuplet::compile_fermata_tuplet(ast)?;
                for note in notes {
                    ir_content.push(MusicDataElement::Note(note));
                }
            }
            MeasureElement::GraceNote(ast) => {
                let note = super::note::compile_fermata_grace(ast)?;
                ir_content.push(MusicDataElement::Note(note));
            }

            // Directions
            MeasureElement::Dynamic(mark) => {
                let dir = super::direction::compile_dynamic_mark(mark)?;
                ir_content.push(MusicDataElement::Direction(dir));
            }
            MeasureElement::Tempo(mark) => {
                let dir = super::direction::compile_tempo_mark(mark)?;
                ir_content.push(MusicDataElement::Direction(dir));
            }
            MeasureElement::Direction(fdir) => {
                let dir = super::direction::compile_fermata_direction(fdir)?;
                ir_content.push(MusicDataElement::Direction(dir));
            }

            // Voice management
            MeasureElement::Backup(dur) => {
                ir_content.push(MusicDataElement::Backup(Backup {
                    duration: *dur,
                }));
            }
            MeasureElement::Forward(dur) => {
                ir_content.push(MusicDataElement::Forward(Forward {
                    duration: *dur,
                    voice: None,
                    staff: None,
                }));
            }

            // Barlines
            MeasureElement::Barline(spec) => {
                let barline = compile_barline(spec)?;
                ir_content.push(MusicDataElement::Barline(barline));
            }

            // Slurs and ties are attached to notes, not standalone
            // (they modify the preceding note during compilation)
            MeasureElement::Slur(_) | MeasureElement::Tie(_) => {}
        }
    }

    Ok(Measure {
        number: measure.number.map(|n| n.to_string()).unwrap_or_default(),
        content: ir_content,
        width: None,
        implicit: None,
        non_controlling: None,
    })
}

fn compile_barline(spec: &BarlineSpec) -> CompileResult<crate::ir::attributes::Barline> {
    use crate::ir::attributes::{Barline, BarStyle, BarStyleValue, BarlineLocation, Repeat, RepeatDirection};

    match spec {
        BarlineSpec::Regular => Ok(Barline {
            location: Some(BarlineLocation::Right),
            bar_style: Some(BarStyle { value: BarStyleValue::Regular, ..Default::default() }),
            repeat: None,
            ..Default::default()
        }),
        BarlineSpec::Double => Ok(Barline {
            location: Some(BarlineLocation::Right),
            bar_style: Some(BarStyle { value: BarStyleValue::LightLight, ..Default::default() }),
            repeat: None,
            ..Default::default()
        }),
        BarlineSpec::Final => Ok(Barline {
            location: Some(BarlineLocation::Right),
            bar_style: Some(BarStyle { value: BarStyleValue::LightHeavy, ..Default::default() }),
            repeat: None,
            ..Default::default()
        }),
        BarlineSpec::RepeatForward => Ok(Barline {
            location: Some(BarlineLocation::Left),
            bar_style: Some(BarStyle { value: BarStyleValue::HeavyLight, ..Default::default() }),
            repeat: Some(Repeat { direction: RepeatDirection::Forward, times: None, ..Default::default() }),
            ..Default::default()
        }),
        BarlineSpec::RepeatBackward => Ok(Barline {
            location: Some(BarlineLocation::Right),
            bar_style: Some(BarStyle { value: BarStyleValue::LightHeavy, ..Default::default() }),
            repeat: Some(Repeat { direction: RepeatDirection::Backward, times: None, ..Default::default() }),
            ..Default::default()
        }),
        BarlineSpec::RepeatBoth => Ok(Barline {
            location: Some(BarlineLocation::Right),
            bar_style: Some(BarStyle { value: BarStyleValue::LightHeavy, ..Default::default() }),
            repeat: Some(Repeat { direction: RepeatDirection::Backward, times: None, ..Default::default() }),
            ..Default::default()
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_simple_measure() {
        let src = "(measure (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))";
        let sexpr = parse(src).unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // 4 notes, no attributes
        assert_eq!(measure.content.len(), 4);
        assert_eq!(measure.number, "1");
    }

    #[test]
    fn test_measure_with_attributes() {
        let src = "(measure (time 4 4) (key c :major) (clef :treble) (note c4 :q))";
        let sexpr = parse(src).unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // 1 attributes block + 1 note
        assert_eq!(measure.content.len(), 2);
        assert!(matches!(measure.content[0], MusicDataElement::Attributes(_)));
        assert!(matches!(measure.content[1], MusicDataElement::Note(_)));
    }

    #[test]
    fn test_measure_with_chord() {
        let src = "(measure (chord :h c4 e4 g4) (chord :h f4 a4 c5))";
        let sexpr = parse(src).unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // Each chord expands to 3 notes = 6 total
        assert_eq!(measure.content.len(), 6);
    }

    #[test]
    fn test_measure_with_dynamics() {
        let src = "(measure (mf) (note c4 :q) (note d4 :q) (cresc) (note e4 :q) (note f4 :q))";
        let sexpr = parse(src).unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // mf direction + 2 notes + cresc direction + 2 notes = 6
        assert_eq!(measure.content.len(), 6);
    }
}
```

---

## Task 2: Part Compilation (`src/fermata/part.rs`)

```rust
//! Part compilation for Fermata syntax.
//!
//! Compiles `(part "Piano" ...)` to IR `Part` and `ScorePart`.

use crate::ir::score::{Part, PartListElement, ScorePart, PartName};
use crate::sexpr::Sexpr;
use super::ast::{FermataPart, FermataMeasure};
use super::error::{CompileError, CompileResult};
use super::defaults::generate_part_id;

/// Compile a part S-expression.
///
/// Syntax:
/// ```lisp
/// (part "Piano"
///   (measure ...)
///   (measure ...)
///   ...)
/// ```
///
/// Returns `(Part, ScorePart)` — the part content and its part-list entry.
pub fn compile_part(sexpr: &Sexpr, index: usize) -> CompileResult<(Part, ScorePart)> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("part list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("part") {
        return Err(CompileError::UnknownForm("expected (part ...)".to_string()));
    }

    let fermata_part = parse_part_form(&list[1..], index)?;
    compile_fermata_part(&fermata_part)
}

/// Parse a part form to AST.
///
/// The first non-keyword argument is the part name (string or symbol).
/// Optional keywords: `:id`, `:abbreviation`.
/// Remaining list forms are measures.
fn parse_part_form(args: &[Sexpr], index: usize) -> CompileResult<FermataPart> {
    if args.is_empty() {
        return Err(CompileError::MissingField("part name"));
    }

    let mut name: Option<String> = None;
    let mut id: Option<String> = None;
    let mut abbreviation: Option<String> = None;
    let mut measures: Vec<FermataMeasure> = Vec::new();
    let mut measure_number = 1u32;

    let mut i = 0;
    while i < args.len() {
        match &args[i] {
            // String literal → part name
            Sexpr::String(s) => {
                if name.is_none() {
                    name = Some(s.clone());
                }
                i += 1;
            }

            // Keyword → option
            Sexpr::Symbol(s) if s.starts_with(':') => {
                let key = &s[1..];
                i += 1;
                match key {
                    "id" => {
                        if i < args.len() {
                            id = args[i].as_symbol()
                                .or_else(|| args[i].as_string())
                                .map(|s| s.to_string());
                            i += 1;
                        }
                    }
                    "abbreviation" | "abbrev" | "abbr" => {
                        if i < args.len() {
                            abbreviation = args[i].as_string()
                                .or_else(|| args[i].as_symbol())
                                .map(|s| s.to_string());
                            i += 1;
                        }
                    }
                    _ => {
                        // Skip unknown keyword + value
                        if i < args.len() && !args[i].is_keyword() && !args[i].is_list() {
                            i += 1;
                        }
                    }
                }
            }

            // List → measure (or bare symbol as part name)
            Sexpr::List(_) => {
                let measure_ast = super::measure::parse_measure_from_sexpr(&args[i], measure_number)?;
                measures.push(measure_ast);
                measure_number += 1;
                i += 1;
            }

            // Bare symbol → part name if not yet set
            Sexpr::Symbol(s) => {
                if name.is_none() {
                    name = Some(s.clone());
                }
                i += 1;
            }
        }
    }

    let part_name = name.unwrap_or_else(|| format!("Part {}", index + 1));
    let part_id = id.unwrap_or_else(|| generate_part_id(index));

    Ok(FermataPart {
        name: part_name,
        id: Some(part_id),
        abbreviation,
        measures,
    })
}

/// Compile `FermataPart` to IR `Part` + `ScorePart` (for part-list).
fn compile_fermata_part(fermata_part: &FermataPart) -> CompileResult<(Part, ScorePart)> {
    let part_id = fermata_part.id.clone()
        .unwrap_or_else(|| "P1".to_string());

    // Compile each measure
    let mut ir_measures = Vec::new();
    for (i, measure) in fermata_part.measures.iter().enumerate() {
        let ir_measure = super::measure::compile_fermata_measure(measure)?;
        ir_measures.push(ir_measure);
    }

    let part = Part {
        id: part_id.clone(),
        measures: ir_measures,
    };

    let score_part = ScorePart {
        id: part_id,
        part_name: PartName {
            value: fermata_part.name.clone(),
            ..Default::default()
        },
        part_abbreviation: fermata_part.abbreviation.as_ref().map(|a| PartName {
            value: a.clone(),
            ..Default::default()
        }),
        ..Default::default()
    };

    Ok((part, score_part))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_simple_part() {
        let src = r#"(part "Piano"
            (measure (time 4 4) (key c :major) (clef :treble)
                (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))
            (measure
                (note g4 :q) (note a4 :q) (note b4 :q) (note c5 :q)))"#;
        let sexpr = parse(src).unwrap();
        let (part, score_part) = compile_part(&sexpr, 0).unwrap();

        assert_eq!(part.id, "P1");
        assert_eq!(part.measures.len(), 2);
        assert_eq!(score_part.part_name.value, "Piano");
    }

    #[test]
    fn test_part_with_id() {
        let src = r#"(part "Violin" :id "Vln1" :abbreviation "Vln."
            (measure (note c4 :q)))"#;
        let sexpr = parse(src).unwrap();
        let (part, score_part) = compile_part(&sexpr, 0).unwrap();

        assert_eq!(part.id, "Vln1");
        assert_eq!(score_part.part_name.value, "Violin");
    }
}
```

---

## Task 3: Score Compilation (`src/fermata/score.rs`)

```rust
//! Score compilation for Fermata syntax.
//!
//! Compiles `(score ...)` to IR `ScorePartwise`.

use crate::ir::score::{
    ScorePartwise, PartList, PartListElement,
    Work, Identification, TypedText,
};
use crate::sexpr::Sexpr;
use super::ast::FermataScore;
use super::error::{CompileError, CompileResult};

/// Compile a score S-expression.
///
/// Syntax:
/// ```lisp
/// (score
///   :title "My Composition"
///   :composer "J. S. Bach"
///   (part "Piano"
///     (measure ...)))
/// ```
pub fn compile_score(sexpr: &Sexpr) -> CompileResult<ScorePartwise> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch("score list", format!("{:?}", sexpr)))?;

    if list.first().and_then(|s| s.as_symbol()) != Some("score") {
        return Err(CompileError::UnknownForm("expected (score ...)".to_string()));
    }

    let fermata_score = parse_score_form(&list[1..])?;
    compile_fermata_score(&fermata_score)
}

fn parse_score_form(args: &[Sexpr]) -> CompileResult<FermataScore> {
    let mut title = None;
    let mut composer = None;
    let mut parts = Vec::new();
    let mut part_index = 0usize;

    let mut i = 0;
    while i < args.len() {
        match &args[i] {
            // Keywords for metadata
            Sexpr::Symbol(s) if s.starts_with(':') => {
                let key = &s[1..];
                i += 1;
                match key {
                    "title" => {
                        if i < args.len() {
                            title = args[i].as_string()
                                .or_else(|| args[i].as_symbol())
                                .map(|s| s.to_string());
                            i += 1;
                        }
                    }
                    "composer" => {
                        if i < args.len() {
                            composer = args[i].as_string()
                                .or_else(|| args[i].as_symbol())
                                .map(|s| s.to_string());
                            i += 1;
                        }
                    }
                    _ => {
                        // Skip unknown keyword + value
                        if i < args.len() && !args[i].is_keyword() && !args[i].is_list() {
                            i += 1;
                        }
                    }
                }
            }

            // List → part
            Sexpr::List(l) => {
                if l.first().and_then(|s| s.as_symbol()) == Some("part") {
                    let part_ast = super::part::parse_part_from_sexpr(&args[i], part_index)?;
                    parts.push(part_ast);
                    part_index += 1;
                }
                i += 1;
            }

            _ => i += 1,
        }
    }

    if parts.is_empty() {
        return Err(CompileError::MissingField("at least one part"));
    }

    Ok(FermataScore { title, composer, parts })
}

/// Compile `FermataScore` AST to IR `ScorePartwise`.
pub fn compile_fermata_score(ast: &FermataScore) -> CompileResult<ScorePartwise> {
    let mut part_list_content = Vec::new();
    let mut ir_parts = Vec::new();

    for (i, fermata_part) in ast.parts.iter().enumerate() {
        let (part, score_part) = super::part::compile_fermata_part(fermata_part)?;
        part_list_content.push(PartListElement::ScorePart(score_part));
        ir_parts.push(part);
    }

    // Work (title)
    let work = ast.title.as_ref().map(|t| Work {
        work_title: Some(t.clone()),
        ..Default::default()
    });

    // Identification (composer)
    let identification = ast.composer.as_ref().map(|c| Identification {
        creators: vec![TypedText {
            value: c.clone(),
            r#type: Some("composer".to_string()),
        }],
        ..Default::default()
    });

    Ok(ScorePartwise {
        version: Some("4.0".to_string()),
        work,
        identification,
        part_list: PartList {
            content: part_list_content,
        },
        parts: ir_parts,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parser::parse;

    #[test]
    fn test_minimal_score() {
        let src = r#"(score
            :title "Test"
            :composer "Me"
            (part "Piano"
                (measure (time 4 4) (key c :major) (clef :treble)
                    (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))))"#;

        let sexpr = parse(src).unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert_eq!(score.version, Some("4.0".to_string()));
        assert!(score.work.is_some());
        assert_eq!(score.work.as_ref().unwrap().work_title.as_deref(), Some("Test"));
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.part_list.content.len(), 1);
    }

    #[test]
    fn test_multi_part_score() {
        let src = r#"(score
            :title "Duet"
            (part "Violin"
                (measure (note c5 :q)))
            (part "Cello"
                (measure (note c3 :q))))"#;

        let sexpr = parse(src).unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert_eq!(score.parts.len(), 2);
        assert_eq!(score.part_list.content.len(), 2);
    }
}
```

---

## Task 4: Top-Level Compiler (`src/fermata/compiler.rs`)

Wire together parsing → AST → IR → (optional) MusicXML emission.

```rust
//! Top-level Fermata compiler.
//!
//! This is the main entry point. It replaces the `todo!()` stubs from
//! Milestone 1 with real implementations.

use crate::ir::score::ScorePartwise;
use crate::sexpr::Sexpr;
use crate::sexpr::parser::parse as parse_sexpr_str;
use super::ast::FermataScore;
use super::error::{CompileError, CompileResult};

/// Compile Fermata source text to Music IR.
///
/// This is the primary public API.
///
/// # Example
///
/// ```rust
/// use fermata::fermata::compile;
///
/// let source = r#"
///     (score
///       :title "Hello"
///       (part "Piano"
///         (measure (time 4 4) (key c :major) (clef :treble)
///           (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))))
/// "#;
/// let score = compile(source).unwrap();
/// ```
pub fn compile(source: &str) -> CompileResult<ScorePartwise> {
    // Step 1: Parse S-expression
    let sexpr = parse_sexpr(source)?;

    // Step 2: Interpret as Fermata AST
    let ast = interpret_sexpr(&sexpr)?;

    // Step 3: Compile AST to IR
    compile_to_ir(&ast)
}

/// Compile Fermata source and emit MusicXML string.
pub fn compile_to_musicxml(source: &str) -> CompileResult<String> {
    let score = compile(source)?;

    // Use the existing MusicXML emitter (Phase 2)
    let xml = crate::musicxml::emit::emit_score(&score);
    Ok(xml)
}

// ─── Internal Pipeline Steps ────────────────────────────────────────────────

/// Parse source text to S-expression.
fn parse_sexpr(source: &str) -> CompileResult<Sexpr> {
    parse_sexpr_str(source)
        .map_err(|e| CompileError::ParseError(format!("{}", e)))
}

/// Interpret top-level S-expression as Fermata AST.
///
/// Handles two forms:
/// 1. `(score ...)` → full score
/// 2. `(part ...)` → single-part score (auto-wrapped)
///
/// For convenience during development, single elements like `(note c4 :q)`
/// are wrapped in a minimal score structure.
fn interpret_sexpr(sexpr: &Sexpr) -> CompileResult<FermataScore> {
    let list = sexpr.as_list()
        .ok_or_else(|| CompileError::type_mismatch(
            "top-level list",
            format!("{:?}", sexpr)
        ))?;

    let head = list.first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::UnknownForm("top-level form must start with a symbol".to_string()))?;

    match head {
        "score" => super::score::parse_score_to_ast(sexpr),

        "part" => {
            // Wrap single part in a score
            let part_ast = super::part::parse_part_from_sexpr(sexpr, 0)?;
            Ok(FermataScore {
                title: None,
                composer: None,
                parts: vec![part_ast],
            })
        }

        "measure" => {
            // Wrap single measure in a part and score
            let measure_ast = super::measure::parse_measure_from_sexpr(sexpr, 1)?;
            Ok(FermataScore {
                title: None,
                composer: None,
                parts: vec![super::ast::FermataPart {
                    name: "Music".to_string(),
                    id: Some("P1".to_string()),
                    abbreviation: None,
                    measures: vec![measure_ast],
                }],
            })
        }

        // Convenience: bare note/chord/etc. wrapped in minimal score
        "note" | "rest" | "chord" | "tuplet" | "grace" => {
            let element = super::measure::classify_measure_element_public(sexpr)?;
            Ok(FermataScore {
                title: None,
                composer: None,
                parts: vec![super::ast::FermataPart {
                    name: "Music".to_string(),
                    id: Some("P1".to_string()),
                    abbreviation: None,
                    measures: vec![super::ast::FermataMeasure {
                        number: Some(1),
                        content: element.into_iter().collect(),
                    }],
                }],
            })
        }

        _ => Err(CompileError::UnknownForm(format!(
            "unrecognized top-level form '{}' (expected score, part, measure, or note)",
            head
        ))),
    }
}

/// Compile Fermata AST to IR.
fn compile_to_ir(ast: &FermataScore) -> CompileResult<ScorePartwise> {
    super::score::compile_fermata_score(ast)
}

// ─── Convenience Functions (for testing / incremental development) ──────────

/// Compile a single note expression.
pub fn compile_note_str(source: &str) -> CompileResult<crate::ir::note::Note> {
    let sexpr = parse_sexpr(source)?;
    super::note::compile_note(&sexpr)
}

/// Compile a single chord expression.
pub fn compile_chord_str(source: &str) -> CompileResult<Vec<crate::ir::note::Note>> {
    let sexpr = parse_sexpr(source)?;
    super::chord::compile_chord(&sexpr)
}

/// Compile a single measure expression.
pub fn compile_measure_str(source: &str) -> CompileResult<crate::ir::measure::Measure> {
    let sexpr = parse_sexpr(source)?;
    super::measure::compile_measure(&sexpr, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_full_score() {
        let src = r#"
            (score
              :title "Test Score"
              :composer "Test Composer"
              (part "Piano"
                (measure
                  (time 4 4)
                  (key c :major)
                  (clef :treble)
                  (note c4 :q)
                  (note d4 :q)
                  (note e4 :q)
                  (note f4 :q))))
        "#;

        let score = compile(src).unwrap();
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 1);
    }

    #[test]
    fn test_compile_bare_measure() {
        let src = "(measure (note c4 :q) (note d4 :q))";
        let score = compile(src).unwrap();

        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 1);
    }

    #[test]
    fn test_compile_bare_note() {
        let src = "(note c4 :q)";
        let score = compile(src).unwrap();

        assert_eq!(score.parts.len(), 1);
    }

    #[test]
    fn test_compile_to_musicxml() {
        let src = r#"
            (score
              :title "Hello"
              (part "Piano"
                (measure (time 4 4) (key c :major) (clef :treble)
                  (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))))
        "#;

        let xml = compile_to_musicxml(src).unwrap();
        assert!(xml.contains("<score-partwise"));
        assert!(xml.contains("Piano"));
    }
}
```

---

## Task 5: CLI Binary (`src/bin/fermata.rs`)

```rust
//! Fermata CLI — compile .ferm files to MusicXML.
//!
//! Usage:
//!   fermata compile input.ferm -o output.xml
//!   fermata compile input.ferm              # prints to stdout
//!   fermata check input.ferm                # parse-only validation

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "compile" => cmd_compile(&args[2..]),
        "check" => cmd_check(&args[2..]),
        "help" | "--help" | "-h" => print_usage(),
        "version" | "--version" | "-V" => println!("fermata 0.1.0"),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn cmd_compile(args: &[String]) {
    let (input, output) = parse_io_args(args);

    let source = read_input(&input);

    match fermata::fermata::compile_to_musicxml(&source) {
        Ok(xml) => {
            match output {
                Some(path) => {
                    fs::write(&path, &xml).unwrap_or_else(|e| {
                        eprintln!("Error writing {}: {}", path, e);
                        process::exit(1);
                    });
                    eprintln!("Wrote {}", path);
                }
                None => {
                    io::stdout().write_all(xml.as_bytes()).unwrap();
                }
            }
        }
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_check(args: &[String]) {
    let (input, _) = parse_io_args(args);

    let source = read_input(&input);

    match fermata::fermata::compile(&source) {
        Ok(score) => {
            let part_count = score.parts.len();
            let measure_count: usize = score.parts.iter()
                .map(|p| p.measures.len())
                .sum();

            eprintln!("✓ Valid Fermata source");
            eprintln!("  {} part(s), {} measure(s)", part_count, measure_count);
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            process::exit(1);
        }
    }
}

fn parse_io_args(args: &[String]) -> (Option<String>, Option<String>) {
    let mut input = None;
    let mut output = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                i += 1;
                if i < args.len() {
                    output = Some(args[i].clone());
                }
            }
            _ => {
                if input.is_none() {
                    input = Some(args[i].clone());
                }
            }
        }
        i += 1;
    }

    (input, output)
}

fn read_input(path: &Option<String>) -> String {
    match path {
        Some(p) => fs::read_to_string(p).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", p, e);
            process::exit(1);
        }),
        None => {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap_or_else(|e| {
                eprintln!("Error reading stdin: {}", e);
                process::exit(1);
            });
            buf
        }
    }
}

fn print_usage() {
    eprintln!("Fermata — Ergonomic music notation compiler");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  fermata compile <input.ferm> [-o <output.xml>]");
    eprintln!("  fermata check <input.ferm>");
    eprintln!("  fermata help");
    eprintln!("  fermata version");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  fermata compile twinkle.ferm -o twinkle.xml");
    eprintln!("  fermata compile twinkle.ferm | xmllint --format -");
    eprintln!("  fermata check twinkle.ferm");
    eprintln!("  cat twinkle.ferm | fermata compile");
}
```

Add to `Cargo.toml`:

```toml
[[bin]]
name = "fermata"
path = "src/bin/fermata.rs"
```

---

## Task 6: End-to-End Integration Tests

Create `tests/fermata_e2e.rs`:

```rust
//! End-to-end integration tests for the Fermata compiler.
//!
//! These tests compile complete Fermata source to MusicXML and validate
//! the output structure.

use fermata::fermata::compile;
use fermata::fermata::compile_to_musicxml;
use fermata::ir::measure::MusicDataElement;
use fermata::ir::score::PartListElement;

/// "Twinkle Twinkle Little Star" — the canonical test case
#[test]
fn test_twinkle_twinkle() {
    let source = r#"
        (score
          :title "Twinkle Twinkle Little Star"
          :composer "Traditional"

          (part "Piano"
            (measure
              (time 4 4)
              (key c :major)
              (clef :treble)
              (note c4 :q) (note c4 :q) (note g4 :q) (note g4 :q))

            (measure
              (note a4 :q) (note a4 :q) (note g4 :h))

            (measure
              (note f4 :q) (note f4 :q) (note e4 :q) (note e4 :q))

            (measure
              (note d4 :q) (note d4 :q) (note c4 :h))

            (measure
              (note g4 :q) (note g4 :q) (note f4 :q) (note f4 :q))

            (measure
              (note e4 :q) (note e4 :q) (note d4 :h))

            (measure
              (note g4 :q) (note g4 :q) (note f4 :q) (note f4 :q))

            (measure
              (note e4 :q) (note e4 :q) (note d4 :h)
              (barline :final))))
    "#;

    let score = compile(source).unwrap();

    // Structure checks
    assert_eq!(score.parts.len(), 1);
    assert_eq!(score.parts[0].measures.len(), 8);

    // Title and composer
    assert_eq!(
        score.work.as_ref().unwrap().work_title.as_deref(),
        Some("Twinkle Twinkle Little Star")
    );

    // Part list
    if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
        assert_eq!(sp.part_name.value, "Piano");
    }

    // First measure has attributes + 4 notes
    let m1 = &score.parts[0].measures[0];
    assert!(matches!(m1.content[0], MusicDataElement::Attributes(_)));

    // Count total notes across all measures
    let total_notes: usize = score.parts[0].measures.iter()
        .flat_map(|m| m.content.iter())
        .filter(|e| matches!(e, MusicDataElement::Note(_)))
        .count();
    // 4+3+4+3+4+3+4+3 = 28 notes
    assert_eq!(total_notes, 28);
}

/// Test with dynamics and tempo
#[test]
fn test_score_with_dynamics() {
    let source = r#"
        (score
          :title "Dynamic Test"
          (part "Piano"
            (measure
              (time 4 4) (key c :major) (clef :treble)
              (tempo "Allegro" :q 120)
              (mf)
              (note c4 :q) (note d4 :q)
              (cresc)
              (note e4 :q) (note f4 :q))))
    "#;

    let score = compile(source).unwrap();

    let m1 = &score.parts[0].measures[0];

    // Should have: attributes, tempo direction, mf direction, note, note, cresc, note, note
    let dir_count = m1.content.iter()
        .filter(|e| matches!(e, MusicDataElement::Direction(_)))
        .count();
    assert!(dir_count >= 2, "Expected at least 2 directions, got {}", dir_count);
}

/// Test chords
#[test]
fn test_score_with_chords() {
    let source = r#"
        (score
          (part "Piano"
            (measure
              (time 4 4) (key c :major) (clef :treble)
              (chord :h c4 e4 g4)
              (chord :h f4 a4 c5))))
    "#;

    let score = compile(source).unwrap();

    let m1 = &score.parts[0].measures[0];
    let note_count = m1.content.iter()
        .filter(|e| matches!(e, MusicDataElement::Note(_)))
        .count();
    // 2 chords × 3 notes each = 6
    assert_eq!(note_count, 6);
}

/// Test triplets
#[test]
fn test_score_with_tuplets() {
    let source = r#"
        (score
          (part "Piano"
            (measure
              (time 4 4) (key c :major) (clef :treble)
              (tuplet 3:2 (note c4 :8) (note d4 :8) (note e4 :8))
              (note f4 :q) (note g4 :q) (note c5 :q))))
    "#;

    let score = compile(source).unwrap();

    // Tuplet expands to 3 notes + 3 regular notes = 6
    let m1 = &score.parts[0].measures[0];
    let note_count = m1.content.iter()
        .filter(|e| matches!(e, MusicDataElement::Note(_)))
        .count();
    assert_eq!(note_count, 6);

    // First 3 notes should have time-modification
    let notes: Vec<_> = m1.content.iter()
        .filter_map(|e| match e {
            MusicDataElement::Note(n) => Some(n),
            _ => None,
        })
        .collect();

    assert!(notes[0].time_modification.is_some());
    assert!(notes[1].time_modification.is_some());
    assert!(notes[2].time_modification.is_some());
    assert!(notes[3].time_modification.is_none()); // Not a tuplet note
}

/// Multi-part score
#[test]
fn test_multi_part() {
    let source = r#"
        (score
          :title "Simple Duet"
          (part "Violin"
            (measure (time 4 4) (key g :major) (clef :treble)
              (note g4 :q) (note a4 :q) (note b4 :q) (note a4 :q)))
          (part "Cello"
            (measure (time 4 4) (key g :major) (clef :bass)
              (note g2 :h) (note d3 :h))))
    "#;

    let score = compile(source).unwrap();

    assert_eq!(score.parts.len(), 2);
    assert_eq!(score.part_list.content.len(), 2);

    // IDs match
    assert_eq!(score.parts[0].id, "P1");
    assert_eq!(score.parts[1].id, "P2");

    if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
        assert_eq!(sp.id, "P1");
    }
    if let PartListElement::ScorePart(sp) = &score.part_list.content[1] {
        assert_eq!(sp.id, "P2");
    }
}

/// Test MusicXML output
#[test]
fn test_musicxml_output() {
    let source = r#"
        (score
          :title "XML Test"
          (part "Piano"
            (measure (time 4 4) (key c :major) (clef :treble)
              (note c4 :q) (note d4 :q) (note e4 :q) (note f4 :q))))
    "#;

    let xml = compile_to_musicxml(source).unwrap();

    // Basic XML structure
    assert!(xml.contains("<score-partwise"), "Missing score-partwise");
    assert!(xml.contains("<part-list>"), "Missing part-list");
    assert!(xml.contains("<score-part"), "Missing score-part");
    assert!(xml.contains("Piano"), "Missing part name");
    assert!(xml.contains("<measure"), "Missing measure");
    assert!(xml.contains("<note"), "Missing note");
    assert!(xml.contains("<pitch"), "Missing pitch");
    assert!(xml.contains("<step>C</step>"), "Missing step");
    assert!(xml.contains("<octave>4</octave>"), "Missing octave");
    assert!(xml.contains("<divisions>"), "Missing divisions");
}

/// Convenience: bare note compiles
#[test]
fn test_bare_note_compiles() {
    let source = "(note c4 :q)";
    let score = compile(source).unwrap();

    assert_eq!(score.parts.len(), 1);
    assert_eq!(score.parts[0].measures.len(), 1);
}

/// Convenience: bare measure compiles
#[test]
fn test_bare_measure_compiles() {
    let source = "(measure (note c4 :q) (rest :q) (note e4 :q) (note f4 :q))";
    let score = compile(source).unwrap();

    assert_eq!(score.parts.len(), 1);
    assert_eq!(score.parts[0].measures.len(), 1);
}

/// Grace notes in context
#[test]
fn test_grace_notes_in_score() {
    let source = r#"
        (score
          (part "Flute"
            (measure (time 4 4) (clef :treble)
              (grace d5 :slash)
              (note c5 :q)
              (note d5 :q)
              (note e5 :q)
              (note f5 :q))))
    "#;

    let score = compile(source).unwrap();

    let m1 = &score.parts[0].measures[0];
    let note_count = m1.content.iter()
        .filter(|e| matches!(e, MusicDataElement::Note(_)))
        .count();
    // 1 grace note + 4 regular notes = 5
    assert_eq!(note_count, 5);
}

/// Key signatures compile correctly across modes
#[test]
fn test_modal_keys() {
    let source = r#"
        (score
          (part "Music"
            (measure (key d :dorian) (time 4 4) (clef :treble)
              (note d4 :w))))
    "#;

    let score = compile(source).unwrap();
    // D dorian = major fifths for D (2) + dorian offset (-2) = 0
    // Same key signature as C major
    let m1 = &score.parts[0].measures[0];
    if let MusicDataElement::Attributes(attrs) = &m1.content[0] {
        if let Some(key) = attrs.keys.first() {
            use fermata::ir::attributes::KeyContent;
            if let KeyContent::Traditional(tk) = &key.content {
                assert_eq!(tk.fifths, 0);
            }
        }
    }
}
```

---

## Task 7: Example Files

Create `examples/twinkle.ferm`:

```lisp
;; Twinkle Twinkle Little Star
;; Traditional melody, arranged for piano

(score
  :title "Twinkle Twinkle Little Star"
  :composer "Traditional"

  (part "Piano"
    ;; Phrase 1
    (measure
      (time 4 4)
      (key c :major)
      (clef :treble)
      (note c4 :q) (note c4 :q) (note g4 :q) (note g4 :q))

    (measure
      (note a4 :q) (note a4 :q) (note g4 :h))

    ;; Phrase 2
    (measure
      (note f4 :q) (note f4 :q) (note e4 :q) (note e4 :q))

    (measure
      (note d4 :q) (note d4 :q) (note c4 :h))

    ;; Phrase 3 (repeat of bridge)
    (measure
      (note g4 :q) (note g4 :q) (note f4 :q) (note f4 :q))

    (measure
      (note e4 :q) (note e4 :q) (note d4 :h))

    ;; Phrase 4 (repeat of bridge)
    (measure
      (note g4 :q) (note g4 :q) (note f4 :q) (note f4 :q))

    (measure
      (note e4 :q) (note e4 :q) (note d4 :h))))
```

Create `examples/chords.ferm`:

```lisp
;; Simple chord progression: I-IV-V-I in C major

(score
  :title "Chord Progression"
  :composer "Example"

  (part "Piano"
    (measure
      (time 4 4)
      (key c :major)
      (clef :treble)
      (mf)
      (chord :h c4 e4 g4)      ;; I (C major)
      (chord :h f4 a4 c5))     ;; IV (F major)

    (measure
      (chord :h g4 b4 d5)      ;; V (G major)
      (chord :h c4 e4 g4)      ;; I (C major)
      (barline :final))))
```

Create `examples/tuplets.ferm`:

```lisp
;; Triplet demonstration

(score
  :title "Triplet Etude"

  (part "Clarinet"
    (measure
      (time 4 4)
      (key g :major)
      (clef :treble)
      (tempo "Moderato" :q 100)
      (mf)
      (tuplet 3:2 (note g4 :8) (note a4 :8) (note b4 :8))
      (note d5 :q)
      (tuplet 3:2 (note c5 :8) (note b4 :8) (note a4 :8))
      (note g4 :q))

    (measure
      (tuplet 3:2 (note b4 :8) (note c5 :8) (note d5 :8))
      (tuplet 3:2 (note e5 :8) (note d5 :8) (note c5 :8))
      (note b4 :h)
      (barline :final))))
```

---

## AST Helper Functions

Several modules need public `parse_*_to_ast` and `parse_*_from_sexpr` functions
that earlier milestones specified as internal. This task adds the public
variants needed by measure/part/score compilation.

Add to each module as needed:

```rust
// In note.rs
pub fn parse_note_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataNote> { ... }
pub fn parse_rest_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataRest> { ... }
pub fn parse_grace_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataGraceNote> { ... }

// In chord.rs
pub fn parse_chord_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataChord> { ... }

// In tuplet.rs
pub fn parse_tuplet_form_to_ast(args: &[Sexpr]) -> CompileResult<FermataTuplet> { ... }

// In attributes.rs
pub fn parse_key_form_to_ast(args: &[Sexpr]) -> CompileResult<KeySpec> { ... }
pub fn parse_time_form_to_ast(args: &[Sexpr]) -> CompileResult<TimeSpec> { ... }
pub fn parse_clef_form_to_ast(args: &[Sexpr]) -> CompileResult<ClefSpec> { ... }

// In direction.rs
pub fn parse_dynamic_to_ast(name: &str) -> CompileResult<DynamicMark> { ... }
pub fn parse_tempo_form_to_ast(args: &[Sexpr]) -> CompileResult<TempoMark> { ... }

// In measure.rs
pub fn parse_measure_from_sexpr(sexpr: &Sexpr, number: u32) -> CompileResult<FermataMeasure> { ... }
pub fn classify_measure_element_public(sexpr: &Sexpr) -> CompileResult<Option<MeasureElement>> { ... }

// In part.rs
pub fn parse_part_from_sexpr(sexpr: &Sexpr, index: usize) -> CompileResult<FermataPart> { ... }

// In score.rs
pub fn parse_score_to_ast(sexpr: &Sexpr) -> CompileResult<FermataScore> { ... }
```

These are typically thin wrappers around the existing internal `parse_*_form`
functions, with the addition of extracting the child args from the full sexpr.

---

## Module Exports (`src/fermata/mod.rs`)

```rust
//! Fermata: Ergonomic music notation syntax.
//!
//! This module compiles Fermata source text to the Music IR.

pub mod ast;
pub mod compiler;
pub mod error;
pub mod defaults;

mod pitch;
mod duration;
mod note;
mod chord;
mod tuplet;
mod grace;
mod attributes;
mod direction;
mod measure;
mod part;
mod score;

// Re-export the main entry points
pub use compiler::compile;
pub use compiler::compile_to_musicxml;
pub use compiler::compile_note_str;
pub use compiler::compile_chord_str;
pub use compiler::compile_measure_str;
pub use error::{CompileError, CompileResult};
```

---

## Acceptance Criteria

1. ✅ `(measure ...)` compiles to IR `Measure` with correct element ordering
2. ✅ Attributes (key, time, clef) gathered into single block at measure start
3. ✅ `(part "Name" ...)` compiles to `(Part, ScorePart)` pair
4. ✅ Part IDs auto-generated as P1, P2, etc.
5. ✅ `(score ...)` compiles to `ScorePartwise` with part-list
6. ✅ Title and composer propagate to Work and Identification
7. ✅ Convenience forms work: bare `(note ...)`, `(measure ...)`, `(part ...)`
8. ✅ `compile()` produces valid `ScorePartwise` IR
9. ✅ `compile_to_musicxml()` produces valid MusicXML string
10. ✅ CLI: `fermata compile input.ferm -o output.xml` works
11. ✅ CLI: `fermata check input.ferm` validates without emitting
12. ✅ "Twinkle Twinkle" round-trip test passes
13. ✅ Multi-part scores compile with matching IDs
14. ✅ Scores with dynamics, chords, tuplets, grace notes all compile
15. ✅ All tests pass

---

## Implementation Notes

1. **Attribute coalescing** — All key/time/clef from anywhere in the measure
   are gathered into one Attributes block emitted first. This matches MusicXML
   convention. If the user needs mid-measure attribute changes, they should
   split into multiple measures.

2. **Part ID generation** — `generate_part_id(0)` → "P1", etc. Must match
   between Part and ScorePart.

3. **Convenience wrapping** — Bare notes, measures, and parts are automatically
   wrapped in a minimal score structure. This makes incremental development
   and testing much easier.

4. **Error context** — The top-level `compile()` function should catch errors
   from each stage and add context about which part/measure/element failed.

5. **MusicXML emission** — This task uses the Phase 2 emitter. If it's not
   yet complete enough, `compile_to_musicxml` can return a placeholder or
   partial output. The IR output from `compile()` is the primary deliverable.

6. **CLI stdin** — When no input file is given, read from stdin. This enables
   piping: `echo "(note c4 :q)" | fermata compile`.

---

*This completes Phase 5 planning. All 5 milestones are specified.*
