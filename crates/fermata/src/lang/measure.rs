//! Measure compilation for Fermata syntax.
//!
//! This module handles compiling measure S-expressions into IR Measure types.
//! It dispatches each child element to the appropriate sub-compiler and
//! gathers attributes into a single Attributes block emitted first.

use crate::ir::attributes::{Attributes, BarStyle, Barline, Clef, Key, Repeat, Time};
use crate::ir::common::{Editorial, RightLeftMiddle};
use crate::ir::direction::Direction;
use crate::ir::measure::{Measure, MusicDataElement};
use crate::ir::note::Note;
use crate::ir::voice::{Backup, Forward};
use crate::lang::ast::{
    BarlineSpec, ClefSpec, DynamicMark, EndingAction, FermataDirection, FermataMeasure, KeySpec,
    MeasureElement, TempoMark, TimeSpec,
};
use crate::lang::attributes::{compile_clef_spec, compile_key_spec, compile_time_spec};
use crate::lang::chord::compile_fermata_chord;
use crate::lang::defaults::DEFAULT_DIVISIONS;
use crate::lang::direction::{compile_dynamic_mark, compile_fermata_direction, compile_tempo_mark};
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::grace::compile_fermata_grace;
use crate::lang::note::{compile_fermata_note, compile_fermata_rest};
use crate::lang::tuplet::compile_fermata_tuplet;
use crate::sexpr::Sexpr;

/// Compile a measure S-expression into an IR Measure.
///
/// # Arguments
///
/// * `sexpr` - The S-expression representing a measure
/// * `number` - The measure number
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::measure::compile_measure;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(measure (note c4 :q) (note d4 :q))")?;
/// let measure = compile_measure(&sexpr, 1)?;
/// ```
pub fn compile_measure(sexpr: &Sexpr, number: u32) -> CompileResult<Measure> {
    let fermata_measure = parse_measure_from_sexpr(sexpr, number)?;
    compile_fermata_measure(&fermata_measure)
}

/// Parse a measure S-expression into a FermataMeasure AST.
///
/// Expected format: `(measure [content...])`
/// where content can be notes, rests, chords, tuplets, attributes, directions, etc.
pub fn parse_measure_from_sexpr(sexpr: &Sexpr, number: u32) -> CompileResult<FermataMeasure> {
    let items = sexpr.as_list().ok_or_else(|| {
        CompileError::UnknownForm(format!("expected measure list, got {:?}", sexpr))
    })?;

    if items.is_empty() {
        return Err(CompileError::UnknownForm("empty measure list".to_string()));
    }

    // Check for 'measure' head
    if let Some(head) = items[0].as_symbol() {
        if head != "measure" {
            return Err(CompileError::UnknownForm(format!(
                "expected 'measure', got '{}'",
                head
            )));
        }
    } else {
        return Err(CompileError::UnknownForm(format!(
            "expected 'measure' symbol, got {:?}",
            items[0]
        )));
    }

    // Parse measure content
    let mut content = Vec::new();

    for item in &items[1..] {
        if let Some(element) = parse_measure_element(item)? {
            content.push(element);
        }
    }

    Ok(FermataMeasure {
        number: Some(number),
        content,
    })
}

/// Parse a single measure element from an S-expression.
///
/// Returns `None` for elements that should be silently skipped.
fn parse_measure_element(sexpr: &Sexpr) -> CompileResult<Option<MeasureElement>> {
    let items = match sexpr.as_list() {
        Some(list) if !list.is_empty() => list,
        _ => return Ok(None), // Skip non-list or empty list items
    };

    let head = match items[0].as_symbol() {
        Some(s) => s,
        None => return Ok(None),
    };

    let element = match head {
        "note" => {
            let fermata_note = crate::lang::note::parse_note_form(&items[1..])?;
            MeasureElement::Note(fermata_note)
        }
        "rest" => {
            let fermata_rest = crate::lang::note::parse_rest_form(&items[1..])?;
            MeasureElement::Rest(fermata_rest)
        }
        "chord" => {
            let fermata_chord = crate::lang::chord::parse_chord_form(&items[1..])?;
            MeasureElement::Chord(fermata_chord)
        }
        "tuplet" => {
            let fermata_tuplet = crate::lang::tuplet::parse_tuplet_form(&items[1..])?;
            MeasureElement::Tuplet(fermata_tuplet)
        }
        "grace" => {
            let fermata_grace = crate::lang::grace::parse_grace_form(&items[1..])?;
            MeasureElement::GraceNote(fermata_grace)
        }
        "key" => {
            let key_spec = crate::lang::attributes::parse_key_form(&items[1..])?;
            MeasureElement::Key(key_spec)
        }
        "time" => {
            let time_spec = crate::lang::attributes::parse_time_form(&items[1..])?;
            MeasureElement::Time(time_spec)
        }
        "clef" => {
            if items.len() < 2 {
                return Err(CompileError::InvalidClef(
                    "clef requires a type".to_string(),
                ));
            }
            let clef_name = items[1].as_keyword().ok_or_else(|| {
                CompileError::InvalidClef("expected clef type keyword".to_string())
            })?;
            let clef_spec = crate::lang::attributes::parse_clef_name(clef_name)?;
            MeasureElement::Clef(clef_spec)
        }
        "barline" => {
            let barline_spec = parse_barline_form(&items[1..])?;
            MeasureElement::Barline(barline_spec)
        }
        "tempo" => {
            let tempo_mark = crate::lang::direction::parse_tempo_form(&items[1..])?;
            MeasureElement::Tempo(tempo_mark)
        }
        "backup" => {
            if items.len() < 2 {
                return Err(CompileError::MissingField("backup duration"));
            }
            let duration = crate::lang::note::parse_u32(&items[1])?;
            MeasureElement::Backup(duration)
        }
        "forward" => {
            if items.len() < 2 {
                return Err(CompileError::MissingField("forward duration"));
            }
            let duration = crate::lang::note::parse_u32(&items[1])?;
            MeasureElement::Forward(duration)
        }
        // Dynamics
        "p" | "pp" | "ppp" | "pppp" | "ppppp" | "pppppp" | "mp" | "mf" | "f" | "ff" | "fff"
        | "ffff" | "fffff" | "ffffff" | "fp" | "sf" | "sfp" | "sfpp" | "sfz" | "sffz" | "sfzp"
        | "fz" | "pf" | "rf" | "rfz" | "cresc" | "cresc-stop" | "dim" | "dim-stop" | "n" => {
            let dynamic = crate::lang::direction::parse_dynamic_name(head)?;
            MeasureElement::Dynamic(dynamic)
        }
        // Direction elements
        "rehearsal" | "words" | "segno" | "coda" | "pedal" => {
            let direction = parse_direction_form(head, &items[1..])?;
            MeasureElement::Direction(direction)
        }
        _ => {
            // Unknown element - skip it
            return Ok(None);
        }
    };

    Ok(Some(element))
}

/// Parse a barline specification from S-expression arguments.
fn parse_barline_form(args: &[Sexpr]) -> CompileResult<BarlineSpec> {
    if args.is_empty() {
        return Ok(BarlineSpec::Regular);
    }

    let barline_type = args[0]
        .as_keyword()
        .or_else(|| args[0].as_symbol())
        .ok_or_else(|| {
            CompileError::UnknownForm(format!("expected barline type, got {:?}", args[0]))
        })?;

    match barline_type.to_lowercase().as_str() {
        "regular" | "single" => Ok(BarlineSpec::Regular),
        "double" => Ok(BarlineSpec::Double),
        "final" | "end" => Ok(BarlineSpec::Final),
        "repeat-forward" | "repeat-start" | "start-repeat" => Ok(BarlineSpec::RepeatForward),
        "repeat-backward" | "repeat-end" | "end-repeat" => Ok(BarlineSpec::RepeatBackward),
        "repeat-both" => Ok(BarlineSpec::RepeatBoth),
        "ending" => {
            // Parse ending number and action
            if args.len() < 3 {
                return Err(CompileError::MissingField("ending number and action"));
            }
            let number = crate::lang::note::parse_u32(&args[1])? as u8;
            let action_str = args[2]
                .as_keyword()
                .or_else(|| args[2].as_symbol())
                .ok_or_else(|| CompileError::MissingField("ending action"))?;
            let action = match action_str.to_lowercase().as_str() {
                "start" => EndingAction::Start,
                "stop" => EndingAction::Stop,
                "discontinue" => EndingAction::Discontinue,
                _ => {
                    return Err(CompileError::UnknownForm(format!(
                        "unknown ending action: {}",
                        action_str
                    )));
                }
            };
            Ok(BarlineSpec::Ending { number, action })
        }
        _ => Err(CompileError::UnknownForm(format!(
            "unknown barline type: {}",
            barline_type
        ))),
    }
}

/// Parse a direction form from S-expression arguments.
fn parse_direction_form(head: &str, args: &[Sexpr]) -> CompileResult<FermataDirection> {
    match head {
        "rehearsal" => {
            if args.is_empty() {
                return Err(CompileError::MissingField("rehearsal mark text"));
            }
            let text = args[0]
                .as_string()
                .ok_or_else(|| CompileError::type_mismatch("string", format!("{:?}", args[0])))?;
            Ok(FermataDirection::Rehearsal(text.to_string()))
        }
        "words" => {
            if args.is_empty() {
                return Err(CompileError::MissingField("words text"));
            }
            let text = args[0]
                .as_string()
                .ok_or_else(|| CompileError::type_mismatch("string", format!("{:?}", args[0])))?;
            Ok(FermataDirection::Words(text.to_string()))
        }
        "segno" => Ok(FermataDirection::Segno),
        "coda" => Ok(FermataDirection::Coda),
        "pedal" => {
            if args.is_empty() {
                return Err(CompileError::MissingField("pedal type"));
            }
            let pedal_type = args[0]
                .as_keyword()
                .ok_or_else(|| CompileError::type_mismatch("keyword", format!("{:?}", args[0])))?;
            match pedal_type.to_lowercase().as_str() {
                "start" => Ok(FermataDirection::PedalStart),
                "stop" => Ok(FermataDirection::PedalStop),
                _ => Err(CompileError::UnknownForm(format!(
                    "unknown pedal type: {}",
                    pedal_type
                ))),
            }
        }
        _ => Err(CompileError::UnknownForm(format!(
            "unknown direction: {}",
            head
        ))),
    }
}

/// Classify a measure element for public use.
///
/// This function is used to determine the type of a measure element
/// from its S-expression representation.
pub fn classify_measure_element_public(sexpr: &Sexpr) -> CompileResult<Option<MeasureElement>> {
    parse_measure_element(sexpr)
}

/// Compile a FermataMeasure AST to an IR Measure.
///
/// This function:
/// 1. Gathers all attributes (key, time, clef) into a single Attributes block
/// 2. Emits the Attributes block first
/// 3. Compiles other elements in source order
pub fn compile_fermata_measure(measure: &FermataMeasure) -> CompileResult<Measure> {
    let mut ir_content: Vec<MusicDataElement> = Vec::new();

    // Gather attributes (key, time, clef)
    let mut keys: Vec<Key> = Vec::new();
    let mut times: Vec<Time> = Vec::new();
    let mut clefs: Vec<Clef> = Vec::new();
    let mut has_attributes = false;

    // First pass: collect attributes
    for element in &measure.content {
        match element {
            MeasureElement::Key(spec) => {
                keys.push(compile_key_spec(spec)?);
                has_attributes = true;
            }
            MeasureElement::Time(spec) => {
                times.push(compile_time_spec(spec)?);
                has_attributes = true;
            }
            MeasureElement::Clef(spec) => {
                clefs.push(compile_clef_spec(spec)?);
                has_attributes = true;
            }
            _ => {}
        }
    }

    // Emit attributes block first if we have any
    if has_attributes {
        let attributes = Attributes {
            editorial: Editorial::default(),
            divisions: Some(DEFAULT_DIVISIONS as u64),
            keys,
            times,
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs,
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };
        ir_content.push(MusicDataElement::Attributes(Box::new(attributes)));
    }

    // Second pass: compile non-attribute elements in order
    for element in &measure.content {
        match element {
            // Skip attributes (already handled)
            MeasureElement::Key(_) | MeasureElement::Time(_) | MeasureElement::Clef(_) => continue,

            // Notes
            MeasureElement::Note(fermata_note) => {
                let note = compile_fermata_note(fermata_note)?;
                ir_content.push(MusicDataElement::Note(Box::new(note)));
            }

            // Rests
            MeasureElement::Rest(fermata_rest) => {
                let note = compile_fermata_rest(fermata_rest)?;
                ir_content.push(MusicDataElement::Note(Box::new(note)));
            }

            // Chords
            MeasureElement::Chord(fermata_chord) => {
                let notes = compile_fermata_chord(fermata_chord)?;
                for note in notes {
                    ir_content.push(MusicDataElement::Note(Box::new(note)));
                }
            }

            // Tuplets
            MeasureElement::Tuplet(fermata_tuplet) => {
                let notes = compile_fermata_tuplet(fermata_tuplet)?;
                for note in notes {
                    ir_content.push(MusicDataElement::Note(Box::new(note)));
                }
            }

            // Grace notes
            MeasureElement::GraceNote(fermata_grace) => {
                let note = compile_fermata_grace(fermata_grace)?;
                ir_content.push(MusicDataElement::Note(Box::new(note)));
            }

            // Dynamics
            MeasureElement::Dynamic(dynamic_mark) => {
                let direction = compile_dynamic_mark(dynamic_mark)?;
                ir_content.push(MusicDataElement::Direction(Box::new(direction)));
            }

            // Tempo
            MeasureElement::Tempo(tempo_mark) => {
                let direction = compile_tempo_mark(tempo_mark)?;
                ir_content.push(MusicDataElement::Direction(Box::new(direction)));
            }

            // Direction
            MeasureElement::Direction(fermata_direction) => {
                let direction = compile_fermata_direction(fermata_direction)?;
                ir_content.push(MusicDataElement::Direction(Box::new(direction)));
            }

            // Barlines
            MeasureElement::Barline(barline_spec) => {
                let barline = compile_barline_spec(barline_spec)?;
                ir_content.push(MusicDataElement::Barline(Box::new(barline)));
            }

            // Backup/Forward
            MeasureElement::Backup(duration) => {
                let backup = Backup {
                    duration: (*duration as u64) * (DEFAULT_DIVISIONS as u64),
                    editorial: Editorial::default(),
                };
                ir_content.push(MusicDataElement::Backup(backup));
            }

            MeasureElement::Forward(duration) => {
                let forward = Forward {
                    duration: (*duration as u64) * (DEFAULT_DIVISIONS as u64),
                    voice: None,
                    staff: None,
                    editorial: Editorial::default(),
                };
                ir_content.push(MusicDataElement::Forward(forward));
            }

            // These are handled by note.rs internally or not yet implemented
            MeasureElement::Slur(_) | MeasureElement::Tie(_) | MeasureElement::Fermata(_) => {
                // Slurs, ties, and fermatas are typically attached to notes
                // rather than being standalone measure elements
            }
        }
    }

    Ok(Measure {
        number: measure
            .number
            .map(|n| n.to_string())
            .unwrap_or_else(|| "1".to_string()),
        implicit: None,
        non_controlling: None,
        width: None,
        content: ir_content,
    })
}

/// Compile a barline specification to an IR Barline.
fn compile_barline_spec(spec: &BarlineSpec) -> CompileResult<Barline> {
    let (bar_style, location, repeat) = match spec {
        BarlineSpec::Regular => (Some(BarStyle::Regular), None, None),
        BarlineSpec::Double => (Some(BarStyle::LightLight), None, None),
        BarlineSpec::Final => (
            Some(BarStyle::LightHeavy),
            Some(RightLeftMiddle::Right),
            None,
        ),
        BarlineSpec::RepeatForward => (
            Some(BarStyle::HeavyLight),
            Some(RightLeftMiddle::Left),
            Some(Repeat {
                direction: crate::ir::common::BackwardForward::Forward,
                times: None,
                winged: None,
            }),
        ),
        BarlineSpec::RepeatBackward => (
            Some(BarStyle::LightHeavy),
            Some(RightLeftMiddle::Right),
            Some(Repeat {
                direction: crate::ir::common::BackwardForward::Backward,
                times: None,
                winged: None,
            }),
        ),
        BarlineSpec::RepeatBoth => (
            Some(BarStyle::HeavyHeavy),
            None,
            Some(Repeat {
                direction: crate::ir::common::BackwardForward::Backward,
                times: None,
                winged: None,
            }),
        ),
        BarlineSpec::Ending {
            number: _,
            action: _,
        } => {
            // Endings are more complex and would need additional IR support
            (Some(BarStyle::Regular), None, None)
        }
    };

    Ok(Barline {
        location,
        bar_style,
        editorial: Editorial::default(),
        wavy_line: None,
        segno: None,
        coda: None,
        fermatas: vec![],
        ending: None,
        repeat,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::note::{NoteContent, PitchRestUnpitched};
    use crate::ir::pitch::Step as IrStep;
    use crate::lang::ast::{FermataDuration, FermataNote, FermataPitch, FermataRest, PitchStep};
    use crate::sexpr::parse;

    // === parse_barline_form tests ===

    #[test]
    fn test_parse_barline_form_empty() {
        let args: Vec<Sexpr> = vec![];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::Regular);
    }

    #[test]
    fn test_parse_barline_form_regular() {
        let args = vec![Sexpr::keyword("regular")];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::Regular);
    }

    #[test]
    fn test_parse_barline_form_double() {
        let args = vec![Sexpr::keyword("double")];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::Double);
    }

    #[test]
    fn test_parse_barline_form_final() {
        let args = vec![Sexpr::keyword("final")];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::Final);
    }

    #[test]
    fn test_parse_barline_form_repeat_forward() {
        let args = vec![Sexpr::keyword("repeat-forward")];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::RepeatForward);
    }

    #[test]
    fn test_parse_barline_form_repeat_backward() {
        let args = vec![Sexpr::keyword("repeat-backward")];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::RepeatBackward);
    }

    #[test]
    fn test_parse_barline_form_repeat_both() {
        let args = vec![Sexpr::keyword("repeat-both")];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(result, BarlineSpec::RepeatBoth);
    }

    #[test]
    fn test_parse_barline_form_ending() {
        let args = vec![
            Sexpr::keyword("ending"),
            Sexpr::Integer(1),
            Sexpr::keyword("start"),
        ];
        let result = parse_barline_form(&args).unwrap();
        assert_eq!(
            result,
            BarlineSpec::Ending {
                number: 1,
                action: EndingAction::Start
            }
        );
    }

    #[test]
    fn test_parse_barline_form_unknown() {
        let args = vec![Sexpr::keyword("unknown")];
        assert!(parse_barline_form(&args).is_err());
    }

    // === parse_direction_form tests ===

    #[test]
    fn test_parse_direction_form_rehearsal() {
        let args = vec![Sexpr::String("A".to_string())];
        let result = parse_direction_form("rehearsal", &args).unwrap();
        assert_eq!(result, FermataDirection::Rehearsal("A".to_string()));
    }

    #[test]
    fn test_parse_direction_form_words() {
        let args = vec![Sexpr::String("dolce".to_string())];
        let result = parse_direction_form("words", &args).unwrap();
        assert_eq!(result, FermataDirection::Words("dolce".to_string()));
    }

    #[test]
    fn test_parse_direction_form_segno() {
        let args: Vec<Sexpr> = vec![];
        let result = parse_direction_form("segno", &args).unwrap();
        assert_eq!(result, FermataDirection::Segno);
    }

    #[test]
    fn test_parse_direction_form_coda() {
        let args: Vec<Sexpr> = vec![];
        let result = parse_direction_form("coda", &args).unwrap();
        assert_eq!(result, FermataDirection::Coda);
    }

    #[test]
    fn test_parse_direction_form_pedal_start() {
        let args = vec![Sexpr::keyword("start")];
        let result = parse_direction_form("pedal", &args).unwrap();
        assert_eq!(result, FermataDirection::PedalStart);
    }

    #[test]
    fn test_parse_direction_form_pedal_stop() {
        let args = vec![Sexpr::keyword("stop")];
        let result = parse_direction_form("pedal", &args).unwrap();
        assert_eq!(result, FermataDirection::PedalStop);
    }

    // === parse_measure_from_sexpr tests ===

    #[test]
    fn test_parse_measure_from_sexpr_simple() {
        let sexpr = parse("(measure (note c4 :q))").unwrap();
        let measure = parse_measure_from_sexpr(&sexpr, 1).unwrap();
        assert_eq!(measure.number, Some(1));
        assert_eq!(measure.content.len(), 1);
    }

    #[test]
    fn test_parse_measure_from_sexpr_with_rest() {
        let sexpr = parse("(measure (rest :q))").unwrap();
        let measure = parse_measure_from_sexpr(&sexpr, 1).unwrap();
        assert_eq!(measure.content.len(), 1);
        assert!(matches!(measure.content[0], MeasureElement::Rest(_)));
    }

    #[test]
    fn test_parse_measure_from_sexpr_with_attributes() {
        let sexpr =
            parse("(measure (key c :major) (time 4 4) (clef :treble) (note c4 :q))").unwrap();
        let measure = parse_measure_from_sexpr(&sexpr, 1).unwrap();
        assert_eq!(measure.content.len(), 4);
    }

    #[test]
    fn test_parse_measure_from_sexpr_empty() {
        let sexpr = parse("(measure)").unwrap();
        let measure = parse_measure_from_sexpr(&sexpr, 1).unwrap();
        assert_eq!(measure.number, Some(1));
        assert!(measure.content.is_empty());
    }

    #[test]
    fn test_parse_measure_from_sexpr_not_list() {
        let sexpr = Sexpr::symbol("measure");
        assert!(parse_measure_from_sexpr(&sexpr, 1).is_err());
    }

    #[test]
    fn test_parse_measure_from_sexpr_wrong_head() {
        let sexpr = parse("(note c4 :q)").unwrap();
        assert!(parse_measure_from_sexpr(&sexpr, 1).is_err());
    }

    // === compile_measure tests ===

    #[test]
    fn test_compile_measure_simple() {
        let sexpr = parse("(measure (note c4 :q))").unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();
        assert_eq!(measure.number, "1");
        assert_eq!(measure.content.len(), 1);
    }

    #[test]
    fn test_compile_measure_with_attributes() {
        let sexpr = parse("(measure (key c :major) (time 4 4) (note c4 :q))").unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // Should have attributes first, then note
        assert!(measure.content.len() >= 2);

        // First element should be attributes
        assert!(matches!(
            measure.content[0],
            MusicDataElement::Attributes(_)
        ));
    }

    #[test]
    fn test_compile_measure_with_chord() {
        let sexpr = parse("(measure (chord (c4 e4 g4) :q))").unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // Chord expands to 3 notes
        assert_eq!(measure.content.len(), 3);
    }

    #[test]
    fn test_compile_measure_with_dynamic() {
        let sexpr = parse("(measure (ff) (note c4 :q))").unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // Should have direction and note
        assert_eq!(measure.content.len(), 2);
        assert!(matches!(measure.content[0], MusicDataElement::Direction(_)));
    }

    #[test]
    fn test_compile_measure_with_barline() {
        let sexpr = parse("(measure (note c4 :q) (barline :final))").unwrap();
        let measure = compile_measure(&sexpr, 1).unwrap();

        // Should have note and barline
        assert_eq!(measure.content.len(), 2);
        assert!(matches!(measure.content[1], MusicDataElement::Barline(_)));
    }

    // === compile_fermata_measure tests ===

    #[test]
    fn test_compile_fermata_measure_basic() {
        let measure = FermataMeasure {
            number: Some(1),
            content: vec![MeasureElement::Note(FermataNote {
                pitch: FermataPitch {
                    step: PitchStep::C,
                    alter: None,
                    octave: 4,
                },
                duration: FermataDuration::default(),
                voice: None,
                staff: None,
                stem: None,
                articulations: vec![],
                ornaments: vec![],
                tie: None,
                slur: None,
                lyric: None,
            })],
        };

        let ir_measure = compile_fermata_measure(&measure).unwrap();
        assert_eq!(ir_measure.number, "1");
        assert_eq!(ir_measure.content.len(), 1);
    }

    #[test]
    fn test_compile_fermata_measure_with_rest() {
        let measure = FermataMeasure {
            number: Some(2),
            content: vec![MeasureElement::Rest(FermataRest {
                duration: FermataDuration::default(),
                voice: None,
                staff: None,
                measure_rest: false,
            })],
        };

        let ir_measure = compile_fermata_measure(&measure).unwrap();
        assert_eq!(ir_measure.number, "2");
        assert_eq!(ir_measure.content.len(), 1);

        if let MusicDataElement::Note(note) = &ir_measure.content[0] {
            if let NoteContent::Regular { full_note, .. } = &note.content {
                assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
            } else {
                panic!("Expected Regular content");
            }
        } else {
            panic!("Expected Note element");
        }
    }

    #[test]
    fn test_compile_fermata_measure_empty() {
        let measure = FermataMeasure {
            number: Some(1),
            content: vec![],
        };

        let ir_measure = compile_fermata_measure(&measure).unwrap();
        assert_eq!(ir_measure.number, "1");
        assert!(ir_measure.content.is_empty());
    }

    #[test]
    fn test_compile_fermata_measure_no_number() {
        let measure = FermataMeasure {
            number: None,
            content: vec![],
        };

        let ir_measure = compile_fermata_measure(&measure).unwrap();
        assert_eq!(ir_measure.number, "1"); // Default
    }

    // === compile_barline_spec tests ===

    #[test]
    fn test_compile_barline_spec_regular() {
        let barline = compile_barline_spec(&BarlineSpec::Regular).unwrap();
        assert_eq!(barline.bar_style, Some(BarStyle::Regular));
    }

    #[test]
    fn test_compile_barline_spec_double() {
        let barline = compile_barline_spec(&BarlineSpec::Double).unwrap();
        assert_eq!(barline.bar_style, Some(BarStyle::LightLight));
    }

    #[test]
    fn test_compile_barline_spec_final() {
        let barline = compile_barline_spec(&BarlineSpec::Final).unwrap();
        assert_eq!(barline.bar_style, Some(BarStyle::LightHeavy));
        assert_eq!(barline.location, Some(RightLeftMiddle::Right));
    }

    #[test]
    fn test_compile_barline_spec_repeat_forward() {
        let barline = compile_barline_spec(&BarlineSpec::RepeatForward).unwrap();
        assert_eq!(barline.bar_style, Some(BarStyle::HeavyLight));
        assert!(barline.repeat.is_some());
    }

    #[test]
    fn test_compile_barline_spec_repeat_backward() {
        let barline = compile_barline_spec(&BarlineSpec::RepeatBackward).unwrap();
        assert_eq!(barline.bar_style, Some(BarStyle::LightHeavy));
        assert!(barline.repeat.is_some());
    }

    // === classify_measure_element_public tests ===

    #[test]
    fn test_classify_measure_element_public_note() {
        let sexpr = parse("(note c4 :q)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MeasureElement::Note(_)));
    }

    #[test]
    fn test_classify_measure_element_public_rest() {
        let sexpr = parse("(rest :q)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MeasureElement::Rest(_)));
    }

    #[test]
    fn test_classify_measure_element_public_chord() {
        let sexpr = parse("(chord (c4 e4 g4) :q)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MeasureElement::Chord(_)));
    }

    #[test]
    fn test_classify_measure_element_public_key() {
        let sexpr = parse("(key c :major)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MeasureElement::Key(_)));
    }

    #[test]
    fn test_classify_measure_element_public_time() {
        let sexpr = parse("(time 4 4)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MeasureElement::Time(_)));
    }

    #[test]
    fn test_classify_measure_element_public_clef() {
        let sexpr = parse("(clef :treble)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_some());
        assert!(matches!(result.unwrap(), MeasureElement::Clef(_)));
    }

    #[test]
    fn test_classify_measure_element_public_unknown() {
        let sexpr = parse("(unknown-element)").unwrap();
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_classify_measure_element_public_not_list() {
        let sexpr = Sexpr::symbol("note");
        let result = classify_measure_element_public(&sexpr).unwrap();
        assert!(result.is_none());
    }
}
