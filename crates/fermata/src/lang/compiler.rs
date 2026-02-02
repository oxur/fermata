//! Fermata -> IR compiler.
//!
//! This module orchestrates the compilation of Fermata syntax to Music IR.

use crate::ir::score::ScorePartwise;
use crate::sexpr::Sexpr;
use crate::sexpr::parser::parse as parse_sexpr;

use super::ast::FermataScore;
use super::error::{CompileError, CompileResult};
use super::score::{compile_fermata_score, parse_score_from_sexpr};

/// Compile Fermata source text to Music IR.
///
/// # Example
///
/// ```rust,ignore
/// use fermata::lang::compile;
///
/// let source = r#"
///     (score :title "Test"
///       (part :piano
///         (measure
///           (note c4 :q))))
/// "#;
/// let score = compile(source)?;
/// ```
pub fn compile(source: &str) -> CompileResult<ScorePartwise> {
    // Step 1: Parse S-expression
    let sexpr = parse_sexpr(source)?;

    // Step 2: Interpret as Fermata AST
    let fermata_ast = interpret_sexpr(&sexpr)?;

    // Step 3: Compile to IR
    compile_to_ir(&fermata_ast)
}

/// Interpret an S-expression as Fermata AST
fn interpret_sexpr(sexpr: &Sexpr) -> CompileResult<FermataScore> {
    parse_score_from_sexpr(sexpr)
}

/// Compile Fermata AST to Music IR
fn compile_to_ir(ast: &FermataScore) -> CompileResult<ScorePartwise> {
    compile_fermata_score(ast)
}

/// Compile a single note (for testing/incremental development)
pub fn compile_note_str(source: &str) -> CompileResult<crate::ir::note::Note> {
    let sexpr = parse_sexpr(source)?;
    super::note::compile_note(&sexpr)
}

/// Compile a single pitch (for testing)
pub fn compile_pitch_str(source: &str) -> CompileResult<crate::ir::pitch::Pitch> {
    let sexpr = parse_sexpr(source)?;
    match &sexpr {
        Sexpr::Symbol(s) => super::pitch::parse_pitch(s),
        _ => Err(CompileError::type_mismatch(
            "pitch symbol",
            format!("{:?}", sexpr),
        )),
    }
}

/// Compile a single measure (for testing/incremental development)
pub fn compile_measure_str(source: &str) -> CompileResult<crate::ir::measure::Measure> {
    let sexpr = parse_sexpr(source)?;
    super::measure::compile_measure(&sexpr, 1)
}

/// Compile a single part (for testing/incremental development)
pub fn compile_part_str(source: &str) -> CompileResult<super::part::CompiledPart> {
    let sexpr = parse_sexpr(source)?;
    super::part::compile_part(&sexpr, 0)
}

/// Check if a Fermata source is valid without fully compiling.
///
/// Returns Ok(()) if the source can be parsed and validated,
/// or an error describing what's wrong.
pub fn check(source: &str) -> CompileResult<()> {
    let sexpr = parse_sexpr(source)?;
    let _ast = interpret_sexpr(&sexpr)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::measure::MusicDataElement;
    use crate::ir::note::{NoteContent, PitchRestUnpitched};
    use crate::ir::part::PartListElement;
    use crate::ir::pitch::Step;

    // === compile tests ===

    #[test]
    fn test_compile_empty_score() {
        let source = "(score)";
        let score = compile(source).unwrap();
        assert!(score.parts.is_empty());
    }

    #[test]
    fn test_compile_score_with_title() {
        let source = r#"(score :title "Test Score")"#;
        let score = compile(source).unwrap();
        assert!(score.work.is_some());
        assert_eq!(
            score.work.unwrap().work_title,
            Some("Test Score".to_string())
        );
    }

    #[test]
    fn test_compile_score_with_composer() {
        let source = r#"(score :composer "Test Composer")"#;
        let score = compile(source).unwrap();
        assert!(score.identification.is_some());
    }

    #[test]
    fn test_compile_score_with_part() {
        let source = r#"(score (part :name "Piano"))"#;
        let score = compile(source).unwrap();
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].id, "P1");
    }

    #[test]
    fn test_compile_score_with_measure() {
        let source = r#"(score (part :piano (measure (note c4 :q))))"#;
        let score = compile(source).unwrap();
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 1);
    }

    #[test]
    fn test_compile_full_score() {
        let source = r#"
            (score
              :title "Simple Song"
              :composer "Test Composer"
              (part :piano
                (measure
                  (key c :major)
                  (time 4 4)
                  (clef :treble)
                  (note c4 :q)
                  (note d4 :q)
                  (note e4 :q)
                  (note f4 :q))
                (measure
                  (note g4 :h)
                  (note a4 :h))
                (measure
                  (note b4 :w)
                  (barline :final))))
        "#;
        let score = compile(source).unwrap();

        // Check metadata
        assert!(score.work.is_some());
        assert!(score.identification.is_some());

        // Check part
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 3);

        // Check part list
        assert_eq!(score.part_list.content.len(), 1);
        if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
            assert_eq!(sp.part_name.value, "Piano");
        }
    }

    #[test]
    fn test_compile_parse_error() {
        let source = "(score (";
        assert!(compile(source).is_err());
    }

    #[test]
    fn test_compile_invalid_form() {
        let source = "(invalid-form)";
        assert!(compile(source).is_err());
    }

    // === compile_note_str tests ===

    #[test]
    fn test_compile_note_str_simple() {
        let note = compile_note_str("(note c4 :q)").unwrap();
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, Step::C);
                assert_eq!(p.octave, 4);
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Regular");
        }
    }

    #[test]
    fn test_compile_note_str_with_sharp() {
        let note = compile_note_str("(note f#4 :q)").unwrap();
        if let NoteContent::Regular { full_note, .. } = &note.content {
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, Step::F);
                assert_eq!(p.alter, Some(1.0));
            } else {
                panic!("Expected Pitch");
            }
        } else {
            panic!("Expected Regular");
        }
    }

    // === compile_pitch_str tests ===

    #[test]
    fn test_compile_pitch_str_simple() {
        let pitch = compile_pitch_str("c4").unwrap();
        assert_eq!(pitch.step, Step::C);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_compile_pitch_str_with_flat() {
        let pitch = compile_pitch_str("bb3").unwrap();
        assert_eq!(pitch.step, Step::B);
        assert_eq!(pitch.alter, Some(-1.0));
        assert_eq!(pitch.octave, 3);
    }

    #[test]
    fn test_compile_pitch_str_not_symbol() {
        assert!(compile_pitch_str("(c4)").is_err());
    }

    // === compile_measure_str tests ===

    #[test]
    fn test_compile_measure_str_simple() {
        let measure = compile_measure_str("(measure (note c4 :q))").unwrap();
        assert_eq!(measure.number, "1");
        assert_eq!(measure.content.len(), 1);
    }

    #[test]
    fn test_compile_measure_str_with_attributes() {
        let measure =
            compile_measure_str("(measure (key c :major) (time 4 4) (note c4 :q))").unwrap();

        // First element should be attributes
        assert!(matches!(
            measure.content[0],
            MusicDataElement::Attributes(_)
        ));
    }

    // === compile_part_str tests ===

    #[test]
    fn test_compile_part_str_simple() {
        let compiled = compile_part_str("(part :name \"Piano\")").unwrap();
        assert_eq!(compiled.part.id, "P1");
        assert_eq!(compiled.score_part.part_name.value, "Piano");
    }

    #[test]
    fn test_compile_part_str_with_measure() {
        let compiled = compile_part_str("(part :piano (measure (note c4 :q)))").unwrap();
        assert_eq!(compiled.part.measures.len(), 1);
    }

    // === check tests ===

    #[test]
    fn test_check_valid_score() {
        let source = "(score :title \"Test\" (part :piano))";
        assert!(check(source).is_ok());
    }

    #[test]
    fn test_check_invalid_syntax() {
        let source = "(score (";
        assert!(check(source).is_err());
    }

    #[test]
    fn test_check_invalid_form() {
        let source = "(invalid)";
        assert!(check(source).is_err());
    }

    // === Integration tests ===

    #[test]
    fn test_round_trip_simple_score() {
        let source = r#"(score :title "Test" (part :piano (measure (note c4 :q))))"#;
        let score = compile(source).unwrap();

        // Verify structure
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 1);
        assert!(!score.parts[0].measures[0].content.is_empty());
    }

    #[test]
    fn test_compile_multi_part_score() {
        let source = r#"
            (score
              :title "Duet"
              (part :violin
                (measure (note c5 :q)))
              (part :cello
                (measure (note c3 :q))))
        "#;
        let score = compile(source).unwrap();

        assert_eq!(score.parts.len(), 2);
        assert_eq!(score.parts[0].id, "P1");
        assert_eq!(score.parts[1].id, "P2");

        if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
            assert_eq!(sp.part_name.value, "Violin");
        }
        if let PartListElement::ScorePart(sp) = &score.part_list.content[1] {
            assert_eq!(sp.part_name.value, "Cello");
        }
    }

    #[test]
    fn test_compile_score_with_dynamics() {
        let source = r#"
            (score
              (part :piano
                (measure
                  (ff)
                  (note c4 :q))))
        "#;
        let score = compile(source).unwrap();

        // First measure should have direction (dynamics) and note
        let measure = &score.parts[0].measures[0];
        assert!(measure.content.len() >= 2);
        assert!(matches!(measure.content[0], MusicDataElement::Direction(_)));
    }

    #[test]
    fn test_compile_score_with_chord() {
        let source = r#"
            (score
              (part :piano
                (measure
                  (chord (c4 e4 g4) :q))))
        "#;
        let score = compile(source).unwrap();

        // Chord should expand to 3 notes
        let measure = &score.parts[0].measures[0];
        assert_eq!(measure.content.len(), 3);
    }
}
