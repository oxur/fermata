//! Direction compilation for Fermata syntax.
//!
//! This module handles compiling direction S-expressions (dynamics, tempo,
//! rehearsal marks, etc.) into IR Direction types.

use crate::ir::common::{AboveBelow, Font, FormattedText, PrintStyle, StartStop};
use crate::ir::direction::{
    Coda, Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics, Metronome,
    MetronomeContent, Pedal, PedalType, PerMinute, Segno, Wedge, WedgeType, Words,
};
use crate::ir::duration::NoteTypeValue;
use crate::sexpr::Sexpr;

use super::ast::{DurationBase, DynamicMark, FermataDirection, TempoMark};
use super::error::{CompileError, CompileResult};

// =============================================================================
// Dynamic Compilation
// =============================================================================

/// Compile a dynamic S-expression into an IR Direction.
///
/// Supports forms like:
/// - `(ff)` - fortissimo
/// - `(pp)` - pianissimo
/// - `(cresc)` - crescendo start
/// - `(dim)` - diminuendo start
/// - `(cresc-stop)` - crescendo stop
pub fn compile_dynamic(sexpr: &Sexpr) -> CompileResult<Direction> {
    let args = sexpr
        .as_list()
        .ok_or_else(|| CompileError::type_mismatch("list", format!("{:?}", sexpr)))?;

    if args.is_empty() {
        return Err(CompileError::InvalidDynamic(
            "empty dynamic form".to_string(),
        ));
    }

    // First element should be the dynamic name
    let name = args
        .first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::InvalidDynamic("expected dynamic symbol".to_string()))?;

    let mark = parse_dynamic_name(name)?;
    compile_dynamic_mark(&mark)
}

/// Parse a dynamic name into a DynamicMark.
pub fn parse_dynamic_name(name: &str) -> CompileResult<DynamicMark> {
    match name.to_lowercase().as_str() {
        // Piano dynamics (softest to loud)
        "pppppp" => Ok(DynamicMark::PPPPPP),
        "ppppp" => Ok(DynamicMark::PPPPP),
        "pppp" => Ok(DynamicMark::PPPP),
        "ppp" => Ok(DynamicMark::PPP),
        "pp" => Ok(DynamicMark::PP),
        "p" => Ok(DynamicMark::P),
        "mp" => Ok(DynamicMark::MP),

        // Forte dynamics
        "mf" => Ok(DynamicMark::MF),
        "f" => Ok(DynamicMark::F),
        "ff" => Ok(DynamicMark::FF),
        "fff" => Ok(DynamicMark::FFF),
        "ffff" => Ok(DynamicMark::FFFF),
        "fffff" => Ok(DynamicMark::FFFFF),
        "ffffff" => Ok(DynamicMark::FFFFFF),

        // Combined dynamics
        "fp" => Ok(DynamicMark::FP),
        "sf" => Ok(DynamicMark::SF),
        "sfp" => Ok(DynamicMark::SFP),
        "sfpp" => Ok(DynamicMark::SFPP),
        "sfz" => Ok(DynamicMark::SFZ),
        "sffz" => Ok(DynamicMark::SFFZ),
        "sfzp" => Ok(DynamicMark::SFZP),
        "fz" => Ok(DynamicMark::FZ),
        "pf" => Ok(DynamicMark::PF),
        "rf" => Ok(DynamicMark::RF),
        "rfz" => Ok(DynamicMark::RFZ),
        "n" | "niente" => Ok(DynamicMark::N),

        // Crescendo/diminuendo
        "cresc" | "crescendo" => Ok(DynamicMark::Crescendo(StartStop::Start)),
        "cresc-stop" | "crescendo-stop" => Ok(DynamicMark::Crescendo(StartStop::Stop)),
        "dim" | "diminuendo" | "decresc" | "decrescendo" => {
            Ok(DynamicMark::Diminuendo(StartStop::Start))
        }
        "dim-stop" | "diminuendo-stop" | "decresc-stop" | "decrescendo-stop" => {
            Ok(DynamicMark::Diminuendo(StartStop::Stop))
        }

        _ => Err(CompileError::InvalidDynamic(format!(
            "unknown dynamic: {}",
            name
        ))),
    }
}

/// Compile a DynamicMark into an IR Direction.
pub fn compile_dynamic_mark(mark: &DynamicMark) -> CompileResult<Direction> {
    match mark {
        // Handle crescendo/diminuendo as wedges
        DynamicMark::Crescendo(action) => {
            let wedge_type = match action {
                StartStop::Start => WedgeType::Crescendo,
                StartStop::Stop => WedgeType::Stop,
            };
            Ok(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: wedge_type,
                        number: Some(1),
                        spread: None,
                        niente: None,
                        line_type: None,
                        position: Default::default(),
                        color: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })
        }
        DynamicMark::Diminuendo(action) => {
            let wedge_type = match action {
                StartStop::Start => WedgeType::Diminuendo,
                StartStop::Stop => WedgeType::Stop,
            };
            Ok(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: wedge_type,
                        number: Some(1),
                        spread: None,
                        niente: None,
                        line_type: None,
                        position: Default::default(),
                        color: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })
        }
        // All other dynamics
        _ => {
            let element = dynamic_mark_to_element(mark)?;
            Ok(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![element],
                        print_style: PrintStyle::default(),
                        placement: Some(AboveBelow::Below),
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })
        }
    }
}

/// Convert a DynamicMark to a DynamicElement.
fn dynamic_mark_to_element(mark: &DynamicMark) -> CompileResult<DynamicElement> {
    match mark {
        DynamicMark::PPPPPP => Ok(DynamicElement::PPPPPP),
        DynamicMark::PPPPP => Ok(DynamicElement::PPPPP),
        DynamicMark::PPPP => Ok(DynamicElement::PPPP),
        DynamicMark::PPP => Ok(DynamicElement::PPP),
        DynamicMark::PP => Ok(DynamicElement::PP),
        DynamicMark::P => Ok(DynamicElement::P),
        DynamicMark::MP => Ok(DynamicElement::MP),
        DynamicMark::MF => Ok(DynamicElement::MF),
        DynamicMark::F => Ok(DynamicElement::F),
        DynamicMark::FF => Ok(DynamicElement::FF),
        DynamicMark::FFF => Ok(DynamicElement::FFF),
        DynamicMark::FFFF => Ok(DynamicElement::FFFF),
        DynamicMark::FFFFF => Ok(DynamicElement::FFFFF),
        DynamicMark::FFFFFF => Ok(DynamicElement::FFFFFF),
        DynamicMark::FP => Ok(DynamicElement::FP),
        DynamicMark::SF => Ok(DynamicElement::SF),
        DynamicMark::SFP => Ok(DynamicElement::SFP),
        DynamicMark::SFPP => Ok(DynamicElement::SFPP),
        DynamicMark::SFZ => Ok(DynamicElement::SFZ),
        DynamicMark::SFFZ => Ok(DynamicElement::SFFZ),
        DynamicMark::SFZP => Ok(DynamicElement::SFZP),
        DynamicMark::FZ => Ok(DynamicElement::FZ),
        DynamicMark::PF => Ok(DynamicElement::PF),
        DynamicMark::RF => Ok(DynamicElement::RF),
        DynamicMark::RFZ => Ok(DynamicElement::RFZ),
        DynamicMark::N => Ok(DynamicElement::N),
        DynamicMark::Crescendo(_) | DynamicMark::Diminuendo(_) => {
            Err(CompileError::InvalidDynamic(
                "crescendo/diminuendo should be handled as wedges".to_string(),
            ))
        }
    }
}

// =============================================================================
// Tempo Compilation
// =============================================================================

/// Compile a tempo S-expression into an IR Direction.
///
/// Supports forms like:
/// - `(tempo :q 120)` - quarter note = 120 BPM
/// - `(tempo "Allegro" :q 120)` - with text
/// - `(tempo :q. 60)` - dotted quarter = 60 BPM
/// - `(tempo "Adagio")` - text only
pub fn compile_tempo(sexpr: &Sexpr) -> CompileResult<Direction> {
    let args = sexpr
        .as_list()
        .ok_or_else(|| CompileError::type_mismatch("list", format!("{:?}", sexpr)))?;

    if args.is_empty() {
        return Err(CompileError::InvalidDuration(
            "empty tempo form".to_string(),
        ));
    }

    // First element should be the symbol "tempo"
    let head = args
        .first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::InvalidDuration("expected 'tempo' symbol".to_string()))?;

    if head != "tempo" {
        return Err(CompileError::InvalidDuration(format!(
            "expected 'tempo' form, got '{}'",
            head
        )));
    }

    let mark = parse_tempo_form(&args[1..])?;
    compile_tempo_mark(&mark)
}

/// Parse tempo arguments into a TempoMark.
pub fn parse_tempo_form(args: &[Sexpr]) -> CompileResult<TempoMark> {
    if args.is_empty() {
        return Err(CompileError::InvalidDuration(
            "tempo requires arguments".to_string(),
        ));
    }

    let mut text = None;
    let mut beat_unit = None;
    let mut beat_unit_dots = 0u8;
    let mut per_minute = None;

    let mut i = 0;
    while i < args.len() {
        match &args[i] {
            // String literal is tempo text
            Sexpr::String(s) => {
                text = Some(s.clone());
                i += 1;
            }
            // Keyword is beat unit (e.g., :q, :h, :e)
            Sexpr::Keyword(k) => {
                let (base, dots) = parse_beat_unit_keyword(k)?;
                beat_unit = Some(base);
                beat_unit_dots = dots;
                i += 1;

                // Handle trailing dots as separate symbols (sexpr parser separates :q and .)
                while i < args.len() {
                    if let Sexpr::Symbol(s) = &args[i] {
                        if s == "." {
                            beat_unit_dots += 1;
                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            // Integer is per minute value
            Sexpr::Integer(n) => {
                if *n <= 0 || *n > 999 {
                    return Err(CompileError::InvalidDuration(format!(
                        "BPM out of range: {}",
                        n
                    )));
                }
                per_minute = Some(*n as u32);
                i += 1;
            }
            // Handle dot symbol that might appear for dotted rhythm (should follow a beat unit)
            Sexpr::Symbol(s) if s == "." => {
                // Dot without preceding beat unit - skip it (error will surface if no beat unit)
                beat_unit_dots += 1;
                i += 1;
            }
            _ => {
                return Err(CompileError::InvalidDuration(format!(
                    "unexpected tempo argument: {:?}",
                    args[i]
                )));
            }
        }
    }

    Ok(TempoMark {
        text,
        beat_unit,
        beat_unit_dots,
        per_minute,
    })
}

/// Parse a beat unit keyword like :q, :h, :e, :q. (dotted quarter).
fn parse_beat_unit_keyword(s: &str) -> CompileResult<(DurationBase, u8)> {
    // Check for dots at the end
    let dots = s.chars().filter(|c| *c == '.').count() as u8;
    let base_str = s.trim_end_matches('.');

    let base = match base_str {
        "w" | "whole" => DurationBase::Whole,
        "h" | "half" => DurationBase::Half,
        "q" | "quarter" => DurationBase::Quarter,
        "e" | "eighth" => DurationBase::Eighth,
        "s" | "sixteenth" | "16th" => DurationBase::Sixteenth,
        "32" | "32nd" => DurationBase::ThirtySecond,
        "64" | "64th" => DurationBase::SixtyFourth,
        _ => {
            return Err(CompileError::InvalidDuration(format!(
                "unknown beat unit: {}",
                s
            )));
        }
    };

    Ok((base, dots))
}

/// Compile a TempoMark into an IR Direction.
pub fn compile_tempo_mark(mark: &TempoMark) -> CompileResult<Direction> {
    let mut direction_types = Vec::new();

    // Add words if there's tempo text
    if let Some(text) = &mark.text {
        direction_types.push(DirectionType {
            content: DirectionTypeContent::Words(vec![Words {
                value: text.clone(),
                print_style: PrintStyle::default(),
                justify: None,
                lang: None,
            }]),
        });
    }

    // Add metronome if there's a beat unit and per minute
    if mark.beat_unit.is_some() && mark.per_minute.is_some() {
        let beat_unit = duration_base_to_note_type(mark.beat_unit.as_ref().unwrap());
        let pm = mark.per_minute.unwrap();

        direction_types.push(DirectionType {
            content: DirectionTypeContent::Metronome(Metronome {
                parentheses: None,
                content: MetronomeContent::PerMinute {
                    beat_unit,
                    beat_unit_dots: mark.beat_unit_dots as u32,
                    per_minute: PerMinute {
                        value: pm.to_string(),
                        font: Font::default(),
                    },
                },
                print_style: PrintStyle::default(),
            }),
        });
    }

    if direction_types.is_empty() {
        return Err(CompileError::InvalidDuration(
            "tempo requires text or metronome marking".to_string(),
        ));
    }

    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types,
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

/// Convert a DurationBase to a NoteTypeValue.
fn duration_base_to_note_type(base: &DurationBase) -> NoteTypeValue {
    match base {
        DurationBase::Maxima => NoteTypeValue::Maxima,
        DurationBase::Long => NoteTypeValue::Long,
        DurationBase::Breve => NoteTypeValue::Breve,
        DurationBase::Whole => NoteTypeValue::Whole,
        DurationBase::Half => NoteTypeValue::Half,
        DurationBase::Quarter => NoteTypeValue::Quarter,
        DurationBase::Eighth => NoteTypeValue::Eighth,
        DurationBase::Sixteenth => NoteTypeValue::N16th,
        DurationBase::ThirtySecond => NoteTypeValue::N32nd,
        DurationBase::SixtyFourth => NoteTypeValue::N64th,
        DurationBase::OneTwentyEighth => NoteTypeValue::N128th,
        DurationBase::TwoFiftySixth => NoteTypeValue::N256th,
        DurationBase::FiveTwelfth => NoteTypeValue::N512th,
        DurationBase::OneThousandTwentyFourth => NoteTypeValue::N1024th,
    }
}

// =============================================================================
// General Direction Compilation
// =============================================================================

/// Compile a direction S-expression into an IR Direction.
///
/// Supports forms like:
/// - `(rehearsal "A")` - rehearsal mark
/// - `(words "dolce")` - text direction
/// - `(segno)` - segno sign
/// - `(coda)` - coda sign
/// - `(pedal :start)` - pedal start
/// - `(pedal :stop)` - pedal stop
pub fn compile_direction(sexpr: &Sexpr) -> CompileResult<Direction> {
    let args = sexpr
        .as_list()
        .ok_or_else(|| CompileError::type_mismatch("list", format!("{:?}", sexpr)))?;

    if args.is_empty() {
        return Err(CompileError::UnknownForm(
            "empty direction form".to_string(),
        ));
    }

    let head = args
        .first()
        .and_then(|s| s.as_symbol())
        .ok_or_else(|| CompileError::UnknownForm("expected direction symbol".to_string()))?;

    match head {
        "rehearsal" => compile_rehearsal(&args[1..]),
        "words" => compile_words(&args[1..]),
        "segno" => compile_segno(),
        "coda" => compile_coda(),
        "pedal" => compile_pedal(&args[1..]),
        _ => Err(CompileError::UnknownForm(format!(
            "unknown direction: {}",
            head
        ))),
    }
}

/// Compile a FermataDirection to IR Direction.
pub fn compile_fermata_direction(dir: &FermataDirection) -> CompileResult<Direction> {
    match dir {
        FermataDirection::Words(text) => compile_words_text(text),
        FermataDirection::Rehearsal(mark) => compile_rehearsal_text(mark),
        FermataDirection::Segno => compile_segno(),
        FermataDirection::Coda => compile_coda(),
        FermataDirection::PedalStart => compile_pedal_action(PedalType::Start),
        FermataDirection::PedalStop => compile_pedal_action(PedalType::Stop),
    }
}

/// Compile a rehearsal mark from arguments.
fn compile_rehearsal(args: &[Sexpr]) -> CompileResult<Direction> {
    if args.is_empty() {
        return Err(CompileError::MissingField("rehearsal mark text"));
    }

    let text = args[0]
        .as_string()
        .ok_or_else(|| CompileError::type_mismatch("string", format!("{:?}", args[0])))?;

    compile_rehearsal_text(text)
}

/// Compile a rehearsal mark from a text string.
fn compile_rehearsal_text(text: &str) -> CompileResult<Direction> {
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Rehearsal(vec![FormattedText {
                value: text.to_string(),
                print_style: PrintStyle::default(),
                lang: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

/// Compile a words direction from arguments.
fn compile_words(args: &[Sexpr]) -> CompileResult<Direction> {
    if args.is_empty() {
        return Err(CompileError::MissingField("words text"));
    }

    let text = args[0]
        .as_string()
        .ok_or_else(|| CompileError::type_mismatch("string", format!("{:?}", args[0])))?;

    compile_words_text(text)
}

/// Compile a words direction from a text string.
fn compile_words_text(text: &str) -> CompileResult<Direction> {
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Words(vec![Words {
                value: text.to_string(),
                print_style: PrintStyle::default(),
                justify: None,
                lang: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

/// Compile a segno sign.
fn compile_segno() -> CompileResult<Direction> {
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Segno(vec![Segno {
                print_style: PrintStyle::default(),
                smufl: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

/// Compile a coda sign.
fn compile_coda() -> CompileResult<Direction> {
    Ok(Direction {
        placement: Some(AboveBelow::Above),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Coda(vec![Coda {
                print_style: PrintStyle::default(),
                smufl: None,
            }]),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

/// Compile a pedal from arguments.
fn compile_pedal(args: &[Sexpr]) -> CompileResult<Direction> {
    if args.is_empty() {
        return Err(CompileError::MissingField("pedal type (:start or :stop)"));
    }

    let action = args[0]
        .as_keyword()
        .ok_or_else(|| CompileError::type_mismatch("keyword", format!("{:?}", args[0])))?;

    let pedal_type = match action {
        "start" => PedalType::Start,
        "stop" => PedalType::Stop,
        "change" => PedalType::Change,
        "continue" => PedalType::Continue,
        "sostenuto" => PedalType::Sostenuto,
        _ => {
            return Err(CompileError::InvalidDynamic(format!(
                "unknown pedal type: {}",
                action
            )));
        }
    };

    compile_pedal_action(pedal_type)
}

/// Compile a pedal with a specific action.
fn compile_pedal_action(pedal_type: PedalType) -> CompileResult<Direction> {
    Ok(Direction {
        placement: Some(AboveBelow::Below),
        directive: None,
        direction_types: vec![DirectionType {
            content: DirectionTypeContent::Pedal(Pedal {
                r#type: pedal_type,
                number: None,
                line: None,
                sign: None,
                abbreviated: None,
                print_style: PrintStyle::default(),
            }),
        }],
        offset: None,
        voice: None,
        staff: None,
        sound: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parse;

    // =============================================================================
    // Dynamic Tests
    // =============================================================================

    mod dynamic_tests {
        use super::*;

        #[test]
        fn test_compile_dynamic_ff() {
            let sexpr = parse("(ff)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            assert_eq!(dir.placement, Some(AboveBelow::Below));
            assert_eq!(dir.direction_types.len(), 1);
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content.len(), 1);
                assert_eq!(d.content[0], DynamicElement::FF);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_pp() {
            let sexpr = parse("(pp)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::PP);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_p() {
            let sexpr = parse("(p)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::P);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_f() {
            let sexpr = parse("(f)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::F);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_mp() {
            let sexpr = parse("(mp)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::MP);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_mf() {
            let sexpr = parse("(mf)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::MF);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_ppp() {
            let sexpr = parse("(ppp)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::PPP);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_pppp() {
            let sexpr = parse("(pppp)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::PPPP);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_fff() {
            let sexpr = parse("(fff)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::FFF);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_ffff() {
            let sexpr = parse("(ffff)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::FFFF);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_sf() {
            let sexpr = parse("(sf)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::SF);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_sfz() {
            let sexpr = parse("(sfz)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::SFZ);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_fp() {
            let sexpr = parse("(fp)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::FP);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_rfz() {
            let sexpr = parse("(rfz)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Dynamics(d) = &dir.direction_types[0].content {
                assert_eq!(d.content[0], DynamicElement::RFZ);
            } else {
                panic!("Expected Dynamics content");
            }
        }

        #[test]
        fn test_compile_dynamic_cresc() {
            let sexpr = parse("(cresc)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Wedge(w) = &dir.direction_types[0].content {
                assert_eq!(w.r#type, WedgeType::Crescendo);
            } else {
                panic!("Expected Wedge content");
            }
        }

        #[test]
        fn test_compile_dynamic_cresc_stop() {
            let sexpr = parse("(cresc-stop)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Wedge(w) = &dir.direction_types[0].content {
                assert_eq!(w.r#type, WedgeType::Stop);
            } else {
                panic!("Expected Wedge content");
            }
        }

        #[test]
        fn test_compile_dynamic_dim() {
            let sexpr = parse("(dim)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Wedge(w) = &dir.direction_types[0].content {
                assert_eq!(w.r#type, WedgeType::Diminuendo);
            } else {
                panic!("Expected Wedge content");
            }
        }

        #[test]
        fn test_compile_dynamic_dim_stop() {
            let sexpr = parse("(dim-stop)").unwrap();
            let dir = compile_dynamic(&sexpr).unwrap();
            if let DirectionTypeContent::Wedge(w) = &dir.direction_types[0].content {
                assert_eq!(w.r#type, WedgeType::Stop);
            } else {
                panic!("Expected Wedge content");
            }
        }

        #[test]
        fn test_compile_dynamic_invalid() {
            let sexpr = parse("(xyz)").unwrap();
            assert!(compile_dynamic(&sexpr).is_err());
        }

        // DynamicMark conversion tests
        #[test]
        fn test_dynamic_mark_to_element_all_dynamics() {
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::PPPPPP).unwrap(),
                DynamicElement::PPPPPP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::PPPPP).unwrap(),
                DynamicElement::PPPPP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::PPPP).unwrap(),
                DynamicElement::PPPP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::PPP).unwrap(),
                DynamicElement::PPP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::PP).unwrap(),
                DynamicElement::PP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::P).unwrap(),
                DynamicElement::P
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::MP).unwrap(),
                DynamicElement::MP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::MF).unwrap(),
                DynamicElement::MF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::F).unwrap(),
                DynamicElement::F
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FF).unwrap(),
                DynamicElement::FF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FFF).unwrap(),
                DynamicElement::FFF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FFFF).unwrap(),
                DynamicElement::FFFF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FFFFF).unwrap(),
                DynamicElement::FFFFF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FFFFFF).unwrap(),
                DynamicElement::FFFFFF
            );
        }

        #[test]
        fn test_dynamic_mark_to_element_combined() {
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FP).unwrap(),
                DynamicElement::FP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::SF).unwrap(),
                DynamicElement::SF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::SFP).unwrap(),
                DynamicElement::SFP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::SFPP).unwrap(),
                DynamicElement::SFPP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::SFZ).unwrap(),
                DynamicElement::SFZ
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::SFFZ).unwrap(),
                DynamicElement::SFFZ
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::SFZP).unwrap(),
                DynamicElement::SFZP
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::FZ).unwrap(),
                DynamicElement::FZ
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::PF).unwrap(),
                DynamicElement::PF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::RF).unwrap(),
                DynamicElement::RF
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::RFZ).unwrap(),
                DynamicElement::RFZ
            );
            assert_eq!(
                dynamic_mark_to_element(&DynamicMark::N).unwrap(),
                DynamicElement::N
            );
        }

        #[test]
        fn test_dynamic_mark_to_element_crescendo_error() {
            assert!(dynamic_mark_to_element(&DynamicMark::Crescendo(StartStop::Start)).is_err());
            assert!(dynamic_mark_to_element(&DynamicMark::Diminuendo(StartStop::Start)).is_err());
        }
    }

    // =============================================================================
    // Tempo Tests
    // =============================================================================

    mod tempo_tests {
        use super::*;

        #[test]
        fn test_compile_tempo_quarter_120() {
            let sexpr = parse("(tempo :q 120)").unwrap();
            let dir = compile_tempo(&sexpr).unwrap();
            assert_eq!(dir.placement, Some(AboveBelow::Above));
            if let DirectionTypeContent::Metronome(m) = &dir.direction_types[0].content {
                if let MetronomeContent::PerMinute {
                    beat_unit,
                    beat_unit_dots,
                    per_minute,
                } = &m.content
                {
                    assert_eq!(*beat_unit, NoteTypeValue::Quarter);
                    assert_eq!(*beat_unit_dots, 0);
                    assert_eq!(per_minute.value, "120");
                } else {
                    panic!("Expected PerMinute content");
                }
            } else {
                panic!("Expected Metronome content");
            }
        }

        #[test]
        fn test_compile_tempo_half_60() {
            let sexpr = parse("(tempo :h 60)").unwrap();
            let dir = compile_tempo(&sexpr).unwrap();
            if let DirectionTypeContent::Metronome(m) = &dir.direction_types[0].content {
                if let MetronomeContent::PerMinute { beat_unit, .. } = &m.content {
                    assert_eq!(*beat_unit, NoteTypeValue::Half);
                } else {
                    panic!("Expected PerMinute content");
                }
            } else {
                panic!("Expected Metronome content");
            }
        }

        #[test]
        fn test_compile_tempo_eighth_144() {
            let sexpr = parse("(tempo :e 144)").unwrap();
            let dir = compile_tempo(&sexpr).unwrap();
            if let DirectionTypeContent::Metronome(m) = &dir.direction_types[0].content {
                if let MetronomeContent::PerMinute { beat_unit, .. } = &m.content {
                    assert_eq!(*beat_unit, NoteTypeValue::Eighth);
                } else {
                    panic!("Expected PerMinute content");
                }
            } else {
                panic!("Expected Metronome content");
            }
        }

        #[test]
        fn test_compile_tempo_with_text() {
            let sexpr = parse("(tempo \"Allegro\" :q 120)").unwrap();
            let dir = compile_tempo(&sexpr).unwrap();
            assert_eq!(dir.direction_types.len(), 2);

            // First should be words
            if let DirectionTypeContent::Words(w) = &dir.direction_types[0].content {
                assert_eq!(w[0].value, "Allegro");
            } else {
                panic!("Expected Words content");
            }

            // Second should be metronome
            if let DirectionTypeContent::Metronome(m) = &dir.direction_types[1].content {
                if let MetronomeContent::PerMinute { per_minute, .. } = &m.content {
                    assert_eq!(per_minute.value, "120");
                }
            } else {
                panic!("Expected Metronome content");
            }
        }

        #[test]
        fn test_compile_tempo_text_only() {
            let sexpr = parse("(tempo \"Adagio\")").unwrap();
            let dir = compile_tempo(&sexpr).unwrap();
            assert_eq!(dir.direction_types.len(), 1);
            if let DirectionTypeContent::Words(w) = &dir.direction_types[0].content {
                assert_eq!(w[0].value, "Adagio");
            } else {
                panic!("Expected Words content");
            }
        }

        #[test]
        fn test_compile_tempo_dotted_quarter() {
            let sexpr = parse("(tempo :q. 60)").unwrap();
            let dir = compile_tempo(&sexpr).unwrap();
            if let DirectionTypeContent::Metronome(m) = &dir.direction_types[0].content {
                if let MetronomeContent::PerMinute { beat_unit_dots, .. } = &m.content {
                    assert_eq!(*beat_unit_dots, 1);
                } else {
                    panic!("Expected PerMinute content");
                }
            } else {
                panic!("Expected Metronome content");
            }
        }

        #[test]
        fn test_compile_tempo_invalid_empty() {
            let sexpr = parse("(tempo)").unwrap();
            assert!(compile_tempo(&sexpr).is_err());
        }

        // Beat unit parsing tests
        #[test]
        fn test_parse_beat_unit_whole() {
            let (base, dots) = parse_beat_unit_keyword("w").unwrap();
            assert_eq!(base, DurationBase::Whole);
            assert_eq!(dots, 0);
        }

        #[test]
        fn test_parse_beat_unit_half() {
            let (base, dots) = parse_beat_unit_keyword("h").unwrap();
            assert_eq!(base, DurationBase::Half);
            assert_eq!(dots, 0);
        }

        #[test]
        fn test_parse_beat_unit_quarter() {
            let (base, dots) = parse_beat_unit_keyword("q").unwrap();
            assert_eq!(base, DurationBase::Quarter);
            assert_eq!(dots, 0);
        }

        #[test]
        fn test_parse_beat_unit_eighth() {
            let (base, dots) = parse_beat_unit_keyword("e").unwrap();
            assert_eq!(base, DurationBase::Eighth);
            assert_eq!(dots, 0);
        }

        #[test]
        fn test_parse_beat_unit_sixteenth() {
            let (base, dots) = parse_beat_unit_keyword("s").unwrap();
            assert_eq!(base, DurationBase::Sixteenth);
            assert_eq!(dots, 0);
        }

        #[test]
        fn test_parse_beat_unit_dotted() {
            let (base, dots) = parse_beat_unit_keyword("q.").unwrap();
            assert_eq!(base, DurationBase::Quarter);
            assert_eq!(dots, 1);
        }

        #[test]
        fn test_parse_beat_unit_double_dotted() {
            let (base, dots) = parse_beat_unit_keyword("q..").unwrap();
            assert_eq!(base, DurationBase::Quarter);
            assert_eq!(dots, 2);
        }

        #[test]
        fn test_parse_beat_unit_invalid() {
            assert!(parse_beat_unit_keyword("x").is_err());
        }

        // Duration base conversion tests
        #[test]
        fn test_duration_base_to_note_type_all() {
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Maxima),
                NoteTypeValue::Maxima
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Long),
                NoteTypeValue::Long
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Breve),
                NoteTypeValue::Breve
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Whole),
                NoteTypeValue::Whole
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Half),
                NoteTypeValue::Half
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Quarter),
                NoteTypeValue::Quarter
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Eighth),
                NoteTypeValue::Eighth
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::Sixteenth),
                NoteTypeValue::N16th
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::ThirtySecond),
                NoteTypeValue::N32nd
            );
            assert_eq!(
                duration_base_to_note_type(&DurationBase::SixtyFourth),
                NoteTypeValue::N64th
            );
        }
    }

    // =============================================================================
    // Direction Tests
    // =============================================================================

    mod direction_tests {
        use super::*;

        #[test]
        fn test_compile_rehearsal() {
            let sexpr = parse("(rehearsal \"A\")").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            assert_eq!(dir.placement, Some(AboveBelow::Above));
            if let DirectionTypeContent::Rehearsal(r) = &dir.direction_types[0].content {
                assert_eq!(r[0].value, "A");
            } else {
                panic!("Expected Rehearsal content");
            }
        }

        #[test]
        fn test_compile_words() {
            let sexpr = parse("(words \"dolce\")").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            if let DirectionTypeContent::Words(w) = &dir.direction_types[0].content {
                assert_eq!(w[0].value, "dolce");
            } else {
                panic!("Expected Words content");
            }
        }

        #[test]
        fn test_compile_segno() {
            let sexpr = parse("(segno)").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            if let DirectionTypeContent::Segno(s) = &dir.direction_types[0].content {
                assert_eq!(s.len(), 1);
            } else {
                panic!("Expected Segno content");
            }
        }

        #[test]
        fn test_compile_coda() {
            let sexpr = parse("(coda)").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            if let DirectionTypeContent::Coda(c) = &dir.direction_types[0].content {
                assert_eq!(c.len(), 1);
            } else {
                panic!("Expected Coda content");
            }
        }

        #[test]
        fn test_compile_pedal_start() {
            let sexpr = parse("(pedal :start)").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            assert_eq!(dir.placement, Some(AboveBelow::Below));
            if let DirectionTypeContent::Pedal(p) = &dir.direction_types[0].content {
                assert_eq!(p.r#type, PedalType::Start);
            } else {
                panic!("Expected Pedal content");
            }
        }

        #[test]
        fn test_compile_pedal_stop() {
            let sexpr = parse("(pedal :stop)").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            if let DirectionTypeContent::Pedal(p) = &dir.direction_types[0].content {
                assert_eq!(p.r#type, PedalType::Stop);
            } else {
                panic!("Expected Pedal content");
            }
        }

        #[test]
        fn test_compile_pedal_change() {
            let sexpr = parse("(pedal :change)").unwrap();
            let dir = compile_direction(&sexpr).unwrap();
            if let DirectionTypeContent::Pedal(p) = &dir.direction_types[0].content {
                assert_eq!(p.r#type, PedalType::Change);
            } else {
                panic!("Expected Pedal content");
            }
        }

        #[test]
        fn test_compile_direction_invalid() {
            let sexpr = parse("(unknown)").unwrap();
            assert!(compile_direction(&sexpr).is_err());
        }

        #[test]
        fn test_compile_rehearsal_missing_text() {
            let sexpr = parse("(rehearsal)").unwrap();
            assert!(compile_direction(&sexpr).is_err());
        }

        #[test]
        fn test_compile_words_missing_text() {
            let sexpr = parse("(words)").unwrap();
            assert!(compile_direction(&sexpr).is_err());
        }

        #[test]
        fn test_compile_pedal_missing_type() {
            let sexpr = parse("(pedal)").unwrap();
            assert!(compile_direction(&sexpr).is_err());
        }

        // FermataDirection compilation tests
        #[test]
        fn test_compile_fermata_direction_words() {
            let dir =
                compile_fermata_direction(&FermataDirection::Words("test".to_string())).unwrap();
            if let DirectionTypeContent::Words(w) = &dir.direction_types[0].content {
                assert_eq!(w[0].value, "test");
            } else {
                panic!("Expected Words content");
            }
        }

        #[test]
        fn test_compile_fermata_direction_rehearsal() {
            let dir =
                compile_fermata_direction(&FermataDirection::Rehearsal("B".to_string())).unwrap();
            if let DirectionTypeContent::Rehearsal(r) = &dir.direction_types[0].content {
                assert_eq!(r[0].value, "B");
            } else {
                panic!("Expected Rehearsal content");
            }
        }

        #[test]
        fn test_compile_fermata_direction_segno() {
            let dir = compile_fermata_direction(&FermataDirection::Segno).unwrap();
            assert!(matches!(
                &dir.direction_types[0].content,
                DirectionTypeContent::Segno(_)
            ));
        }

        #[test]
        fn test_compile_fermata_direction_coda() {
            let dir = compile_fermata_direction(&FermataDirection::Coda).unwrap();
            assert!(matches!(
                &dir.direction_types[0].content,
                DirectionTypeContent::Coda(_)
            ));
        }

        #[test]
        fn test_compile_fermata_direction_pedal_start() {
            let dir = compile_fermata_direction(&FermataDirection::PedalStart).unwrap();
            if let DirectionTypeContent::Pedal(p) = &dir.direction_types[0].content {
                assert_eq!(p.r#type, PedalType::Start);
            } else {
                panic!("Expected Pedal content");
            }
        }

        #[test]
        fn test_compile_fermata_direction_pedal_stop() {
            let dir = compile_fermata_direction(&FermataDirection::PedalStop).unwrap();
            if let DirectionTypeContent::Pedal(p) = &dir.direction_types[0].content {
                assert_eq!(p.r#type, PedalType::Stop);
            } else {
                panic!("Expected Pedal content");
            }
        }
    }
}
