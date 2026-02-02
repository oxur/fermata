//! Tie and slur handling for Fermata syntax.
//!
//! This module provides functions for parsing and compiling tie/slur markers
//! that connect notes together.

use crate::ir::common::{Position, StartStop, StartStopContinue};
use crate::ir::notation::{NotationContent, Notations, Slur, Tied};
use crate::lang::ast::SlurMark;
use crate::lang::error::{CompileError, CompileResult};
use crate::sexpr::Sexpr;

/// Parse a slur action from an S-expression.
///
/// Accepts "start" or "stop" (case insensitive).
pub fn parse_slur_action(sexpr: &Sexpr) -> CompileResult<StartStop> {
    let s = sexpr.as_symbol().or_else(|| sexpr.as_keyword()).ok_or_else(|| {
        CompileError::InvalidNote(format!("expected slur action symbol, got {:?}", sexpr))
    })?;

    match s.to_lowercase().as_str() {
        "start" => Ok(StartStop::Start),
        "stop" => Ok(StartStop::Stop),
        _ => Err(CompileError::InvalidNote(
            format!("invalid slur action '{}', expected start or stop", s)
        )),
    }
}

/// Parse a u8 from an S-expression (Integer or Symbol).
///
/// This handles slur numbers which are u8 in the IR.
pub fn parse_u8(sexpr: &Sexpr) -> CompileResult<u8> {
    match sexpr {
        Sexpr::Integer(n) => {
            if *n < 0 || *n > 255 {
                return Err(CompileError::InvalidNote(
                    format!("slur number {} out of range (0-255)", n)
                ));
            }
            Ok(*n as u8)
        }
        Sexpr::Symbol(s) => s.parse().map_err(|_| {
            CompileError::InvalidNote(format!("invalid slur number '{}'", s))
        }),
        _ => Err(CompileError::InvalidNote(
            format!("expected integer for slur number, got {:?}", sexpr)
        )),
    }
}

/// Compile a SlurMark to an IR Slur notation.
///
/// The Slur.number field is required (u8), not optional.
pub fn compile_slur_marker(mark: &SlurMark) -> Slur {
    let r#type = match mark.action {
        StartStop::Start => StartStopContinue::Start,
        StartStop::Stop => StartStopContinue::Stop,
    };

    Slur {
        r#type,
        number: mark.number, // Required field, defaults to 1
        line_type: None,
        position: Position::default(),
        placement: None,
        orientation: None,
        color: None,
    }
}

/// Convert a StartStop to StartStopContinue for tied notation.
pub fn start_stop_to_continue(ss: StartStop) -> StartStopContinue {
    match ss {
        StartStop::Start => StartStopContinue::Start,
        StartStop::Stop => StartStopContinue::Stop,
    }
}

/// Create a Tied notation element.
pub fn create_tied(action: StartStop) -> Tied {
    Tied {
        r#type: start_stop_to_continue(action),
        number: None,
        line_type: None,
        position: Position::default(),
        placement: None,
        orientation: None,
        color: None,
    }
}

/// Create a Slur notation element from an action and optional number.
pub fn create_slur(action: StartStop, number: u8) -> Slur {
    Slur {
        r#type: start_stop_to_continue(action),
        number, // Required field
        line_type: None,
        position: Position::default(),
        placement: None,
        orientation: None,
        color: None,
    }
}

/// Parse a slur form from S-expression items.
///
/// Expected format: `action [number]`
/// - action: start or stop
/// - number: optional slur number (defaults to 1)
pub fn parse_slur_form(items: &[Sexpr]) -> CompileResult<SlurMark> {
    if items.is_empty() {
        return Err(CompileError::InvalidNote("slur requires action (start/stop)".to_string()));
    }

    let action = parse_slur_action(&items[0])?;

    let number = if items.len() > 1 {
        parse_u8(&items[1])?
    } else {
        1 // Default slur number
    };

    Ok(SlurMark { action, number })
}

/// Wrap a Slur in a NotationContent.
pub fn slur_to_notation_content(slur: Slur) -> NotationContent {
    NotationContent::Slur(slur)
}

/// Wrap a Tied in a NotationContent.
pub fn tied_to_notation_content(tied: Tied) -> NotationContent {
    NotationContent::Tied(tied)
}

/// Create a Notations container with a single slur.
pub fn create_slur_notations(action: StartStop, number: u8) -> Notations {
    Notations {
        print_object: None,
        content: vec![NotationContent::Slur(create_slur(action, number))],
        editorial: Default::default(),
    }
}

/// Create a Notations container with a single tie.
pub fn create_tied_notations(action: StartStop) -> Notations {
    Notations {
        print_object: None,
        content: vec![NotationContent::Tied(create_tied(action))],
        editorial: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_slur_action tests ===

    #[test]
    fn test_parse_slur_action_start() {
        assert_eq!(parse_slur_action(&Sexpr::symbol("start")).unwrap(), StartStop::Start);
        assert_eq!(parse_slur_action(&Sexpr::symbol("START")).unwrap(), StartStop::Start);
        assert_eq!(parse_slur_action(&Sexpr::symbol("Start")).unwrap(), StartStop::Start);
    }

    #[test]
    fn test_parse_slur_action_stop() {
        assert_eq!(parse_slur_action(&Sexpr::symbol("stop")).unwrap(), StartStop::Stop);
        assert_eq!(parse_slur_action(&Sexpr::symbol("STOP")).unwrap(), StartStop::Stop);
    }

    #[test]
    fn test_parse_slur_action_keyword() {
        assert_eq!(parse_slur_action(&Sexpr::keyword("start")).unwrap(), StartStop::Start);
        assert_eq!(parse_slur_action(&Sexpr::keyword("stop")).unwrap(), StartStop::Stop);
    }

    #[test]
    fn test_parse_slur_action_invalid() {
        assert!(parse_slur_action(&Sexpr::symbol("continue")).is_err());
        assert!(parse_slur_action(&Sexpr::symbol("pause")).is_err());
    }

    #[test]
    fn test_parse_slur_action_not_symbol() {
        assert!(parse_slur_action(&Sexpr::Integer(1)).is_err());
    }

    // === parse_u8 tests ===

    #[test]
    fn test_parse_u8_integer() {
        assert_eq!(parse_u8(&Sexpr::Integer(1)).unwrap(), 1);
        assert_eq!(parse_u8(&Sexpr::Integer(0)).unwrap(), 0);
        assert_eq!(parse_u8(&Sexpr::Integer(255)).unwrap(), 255);
    }

    #[test]
    fn test_parse_u8_symbol() {
        assert_eq!(parse_u8(&Sexpr::symbol("1")).unwrap(), 1);
        assert_eq!(parse_u8(&Sexpr::symbol("123")).unwrap(), 123);
    }

    #[test]
    fn test_parse_u8_negative() {
        assert!(parse_u8(&Sexpr::Integer(-1)).is_err());
    }

    #[test]
    fn test_parse_u8_too_large() {
        assert!(parse_u8(&Sexpr::Integer(256)).is_err());
    }

    #[test]
    fn test_parse_u8_invalid_symbol() {
        assert!(parse_u8(&Sexpr::symbol("abc")).is_err());
    }

    // === compile_slur_marker tests ===

    #[test]
    fn test_compile_slur_marker_start() {
        let mark = SlurMark { action: StartStop::Start, number: 1 };
        let slur = compile_slur_marker(&mark);
        assert_eq!(slur.r#type, StartStopContinue::Start);
        assert_eq!(slur.number, 1);
    }

    #[test]
    fn test_compile_slur_marker_stop() {
        let mark = SlurMark { action: StartStop::Stop, number: 2 };
        let slur = compile_slur_marker(&mark);
        assert_eq!(slur.r#type, StartStopContinue::Stop);
        assert_eq!(slur.number, 2);
    }

    // === start_stop_to_continue tests ===

    #[test]
    fn test_start_stop_to_continue_start() {
        assert_eq!(start_stop_to_continue(StartStop::Start), StartStopContinue::Start);
    }

    #[test]
    fn test_start_stop_to_continue_stop() {
        assert_eq!(start_stop_to_continue(StartStop::Stop), StartStopContinue::Stop);
    }

    // === create_tied tests ===

    #[test]
    fn test_create_tied_start() {
        let tied = create_tied(StartStop::Start);
        assert_eq!(tied.r#type, StartStopContinue::Start);
        assert!(tied.number.is_none());
    }

    #[test]
    fn test_create_tied_stop() {
        let tied = create_tied(StartStop::Stop);
        assert_eq!(tied.r#type, StartStopContinue::Stop);
    }

    // === create_slur tests ===

    #[test]
    fn test_create_slur_start() {
        let slur = create_slur(StartStop::Start, 1);
        assert_eq!(slur.r#type, StartStopContinue::Start);
        assert_eq!(slur.number, 1);
    }

    #[test]
    fn test_create_slur_with_number() {
        let slur = create_slur(StartStop::Stop, 3);
        assert_eq!(slur.r#type, StartStopContinue::Stop);
        assert_eq!(slur.number, 3);
    }

    // === parse_slur_form tests ===

    #[test]
    fn test_parse_slur_form_start_only() {
        let items = vec![Sexpr::symbol("start")];
        let mark = parse_slur_form(&items).unwrap();
        assert_eq!(mark.action, StartStop::Start);
        assert_eq!(mark.number, 1); // Default
    }

    #[test]
    fn test_parse_slur_form_stop_only() {
        let items = vec![Sexpr::symbol("stop")];
        let mark = parse_slur_form(&items).unwrap();
        assert_eq!(mark.action, StartStop::Stop);
        assert_eq!(mark.number, 1); // Default
    }

    #[test]
    fn test_parse_slur_form_with_number() {
        let items = vec![Sexpr::symbol("start"), Sexpr::Integer(2)];
        let mark = parse_slur_form(&items).unwrap();
        assert_eq!(mark.action, StartStop::Start);
        assert_eq!(mark.number, 2);
    }

    #[test]
    fn test_parse_slur_form_empty() {
        let items: Vec<Sexpr> = vec![];
        assert!(parse_slur_form(&items).is_err());
    }

    // === slur_to_notation_content tests ===

    #[test]
    fn test_slur_to_notation_content() {
        let slur = create_slur(StartStop::Start, 1);
        let content = slur_to_notation_content(slur);
        if let NotationContent::Slur(s) = content {
            assert_eq!(s.r#type, StartStopContinue::Start);
        } else {
            panic!("Expected Slur content");
        }
    }

    // === tied_to_notation_content tests ===

    #[test]
    fn test_tied_to_notation_content() {
        let tied = create_tied(StartStop::Stop);
        let content = tied_to_notation_content(tied);
        if let NotationContent::Tied(t) = content {
            assert_eq!(t.r#type, StartStopContinue::Stop);
        } else {
            panic!("Expected Tied content");
        }
    }

    // === create_slur_notations tests ===

    #[test]
    fn test_create_slur_notations() {
        let notations = create_slur_notations(StartStop::Start, 1);
        assert_eq!(notations.content.len(), 1);
        if let NotationContent::Slur(s) = &notations.content[0] {
            assert_eq!(s.number, 1);
        } else {
            panic!("Expected Slur content");
        }
    }

    // === create_tied_notations tests ===

    #[test]
    fn test_create_tied_notations() {
        let notations = create_tied_notations(StartStop::Start);
        assert_eq!(notations.content.len(), 1);
        assert!(matches!(&notations.content[0], NotationContent::Tied(_)));
    }
}
