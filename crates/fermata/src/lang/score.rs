//! Score compilation for Fermata syntax.
//!
//! This module handles compiling score S-expressions into IR ScorePartwise types.
//! It assembles parts, generates the part-list, and handles score metadata.

use crate::ir::common::{Identification, LeftCenterRight, PrintStyle, TopMiddleBottom, TypedText};
use crate::ir::part::PartList;
use crate::ir::score::{Credit, CreditContent, CreditWords, ScorePartwise, Work};
use crate::lang::ast::FermataScore;
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::part::{compile_fermata_part, parse_part_from_sexpr, score_part_to_list_element};
use crate::sexpr::Sexpr;

/// Compile a score S-expression into an IR ScorePartwise.
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::score::compile_score;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse(r#"(score :title "Test" (part :piano (measure (note c4 :q))))"#)?;
/// let score = compile_score(&sexpr)?;
/// ```
pub fn compile_score(sexpr: &Sexpr) -> CompileResult<ScorePartwise> {
    let fermata_score = parse_score_from_sexpr(sexpr)?;
    compile_fermata_score(&fermata_score)
}

/// Parse a score S-expression into a FermataScore AST.
///
/// Expected format: `(score [:title "Title"] [:composer "Composer"] parts...)`
pub fn parse_score_from_sexpr(sexpr: &Sexpr) -> CompileResult<FermataScore> {
    let items = sexpr.as_list().ok_or_else(|| {
        CompileError::UnknownForm(format!("expected score list, got {:?}", sexpr))
    })?;

    if items.is_empty() {
        return Err(CompileError::UnknownForm("empty score list".to_string()));
    }

    // Check for 'score' head
    if let Some(head) = items[0].as_symbol() {
        if head != "score" {
            return Err(CompileError::UnknownForm(format!(
                "expected 'score', got '{}'",
                head
            )));
        }
    } else {
        return Err(CompileError::UnknownForm(format!(
            "expected 'score' symbol, got {:?}",
            items[0]
        )));
    }

    // Parse score attributes and content
    let mut title: Option<String> = None;
    let mut composer: Option<String> = None;
    let mut parts = Vec::new();
    let mut part_index = 0usize;

    let mut i = 1;
    while i < items.len() {
        // Check for keyword arguments
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "title" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::MissingField("score title value"));
                    }
                    title = Some(
                        items[i + 1]
                            .as_string()
                            .ok_or_else(|| {
                                CompileError::type_mismatch("string", format!("{:?}", items[i + 1]))
                            })?
                            .to_string(),
                    );
                    i += 2;
                }
                "composer" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::MissingField("score composer value"));
                    }
                    composer = Some(
                        items[i + 1]
                            .as_string()
                            .ok_or_else(|| {
                                CompileError::type_mismatch("string", format!("{:?}", items[i + 1]))
                            })?
                            .to_string(),
                    );
                    i += 2;
                }
                _ => {
                    // Unknown keyword - skip
                    i += 1;
                }
            }
        } else if let Some(list) = items[i].as_list() {
            // Check if it's a part
            if !list.is_empty() {
                if let Some(head) = list[0].as_symbol() {
                    if head == "part" {
                        let fermata_part = parse_part_from_sexpr(&items[i], part_index)?;
                        parts.push(fermata_part);
                        part_index += 1;
                        i += 1;
                        continue;
                    }
                }
            }
            // Not a part - skip unknown list
            i += 1;
        } else {
            // Skip unknown items
            i += 1;
        }
    }

    Ok(FermataScore {
        title,
        composer,
        parts,
    })
}

/// Compile a FermataScore AST to an IR ScorePartwise.
pub fn compile_fermata_score(score: &FermataScore) -> CompileResult<ScorePartwise> {
    // Compile all parts
    let mut ir_parts = Vec::new();
    let mut part_list_elements = Vec::new();

    for (index, fermata_part) in score.parts.iter().enumerate() {
        let compiled = compile_fermata_part(fermata_part, index)?;
        part_list_elements.push(score_part_to_list_element(compiled.score_part));
        ir_parts.push(compiled.part);
    }

    // Build Work if we have a title
    let work = score.title.as_ref().map(|t| Work {
        work_number: None,
        work_title: Some(t.clone()),
        opus: None,
    });

    // Build Identification if we have a composer
    let identification = score.composer.as_ref().map(|c| Identification {
        creators: vec![TypedText {
            r#type: Some("composer".to_string()),
            value: c.clone(),
        }],
        rights: vec![],
        encoding: None,
        source: None,
        relations: vec![],
        miscellaneous: None,
    });

    // Build Credits for title and composer (for visual display)
    let mut credits = Vec::new();

    if let Some(ref t) = score.title {
        credits.push(Credit {
            page: Some(1),
            content: vec![
                CreditContent::CreditType("title".to_string()),
                CreditContent::CreditWords(CreditWords {
                    value: t.clone(),
                    print_style: PrintStyle::default(),
                    justify: Some(LeftCenterRight::Center),
                    halign: Some(LeftCenterRight::Center),
                    valign: Some(TopMiddleBottom::Top),
                    lang: None,
                }),
            ],
        });
    }

    if let Some(ref c) = score.composer {
        credits.push(Credit {
            page: Some(1),
            content: vec![
                CreditContent::CreditType("composer".to_string()),
                CreditContent::CreditWords(CreditWords {
                    value: c.clone(),
                    print_style: PrintStyle::default(),
                    justify: Some(LeftCenterRight::Right),
                    halign: Some(LeftCenterRight::Right),
                    valign: Some(TopMiddleBottom::Top),
                    lang: None,
                }),
            ],
        });
    }

    Ok(ScorePartwise {
        version: Some("4.0".to_string()),
        work,
        movement_number: None,
        movement_title: None,
        identification,
        defaults: None,
        credits,
        part_list: PartList {
            content: part_list_elements,
        },
        parts: ir_parts,
    })
}

/// Parse score to AST (public helper).
///
/// This is a convenience function for parsing just the score structure
/// without compiling to IR.
pub fn parse_score_to_ast(sexpr: &Sexpr) -> CompileResult<FermataScore> {
    parse_score_from_sexpr(sexpr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::part::PartListElement;
    use crate::lang::ast::{
        FermataDuration, FermataMeasure, FermataNote, FermataPart, FermataPitch, MeasureElement,
        PitchStep,
    };
    use crate::sexpr::parse;

    // === parse_score_from_sexpr tests ===

    #[test]
    fn test_parse_score_from_sexpr_simple() {
        let sexpr = parse("(score)").unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert!(score.title.is_none());
        assert!(score.composer.is_none());
        assert!(score.parts.is_empty());
    }

    #[test]
    fn test_parse_score_from_sexpr_with_title() {
        let sexpr = parse(r#"(score :title "Test Score")"#).unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert_eq!(score.title, Some("Test Score".to_string()));
    }

    #[test]
    fn test_parse_score_from_sexpr_with_composer() {
        let sexpr = parse(r#"(score :composer "J.S. Bach")"#).unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert_eq!(score.composer, Some("J.S. Bach".to_string()));
    }

    #[test]
    fn test_parse_score_from_sexpr_with_title_and_composer() {
        let sexpr = parse(r#"(score :title "Fugue in G minor" :composer "J.S. Bach")"#).unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert_eq!(score.title, Some("Fugue in G minor".to_string()));
        assert_eq!(score.composer, Some("J.S. Bach".to_string()));
    }

    #[test]
    fn test_parse_score_from_sexpr_with_part() {
        let sexpr = parse(r#"(score :title "Test" (part :piano))"#).unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].name, "Piano");
    }

    #[test]
    fn test_parse_score_from_sexpr_with_multiple_parts() {
        let sexpr = parse(r#"(score (part :violin) (part :cello))"#).unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert_eq!(score.parts.len(), 2);
        assert_eq!(score.parts[0].name, "Violin");
        assert_eq!(score.parts[1].name, "Cello");
    }

    #[test]
    fn test_parse_score_from_sexpr_with_full_part() {
        let sexpr = parse(r#"(score (part :name "Piano" (measure (note c4 :q))))"#).unwrap();
        let score = parse_score_from_sexpr(&sexpr).unwrap();
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].name, "Piano");
        assert_eq!(score.parts[0].measures.len(), 1);
    }

    #[test]
    fn test_parse_score_from_sexpr_not_list() {
        let sexpr = Sexpr::symbol("score");
        assert!(parse_score_from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_score_from_sexpr_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(parse_score_from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_parse_score_from_sexpr_wrong_head() {
        let sexpr = parse("(part :piano)").unwrap();
        assert!(parse_score_from_sexpr(&sexpr).is_err());
    }

    // === compile_score tests ===

    #[test]
    fn test_compile_score_empty() {
        let sexpr = parse("(score)").unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert_eq!(score.version, Some("4.0".to_string()));
        assert!(score.work.is_none());
        assert!(score.identification.is_none());
        assert!(score.parts.is_empty());
        assert!(score.part_list.content.is_empty());
    }

    #[test]
    fn test_compile_score_with_title() {
        let sexpr = parse(r#"(score :title "Symphony No. 5")"#).unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert!(score.work.is_some());
        let work = score.work.unwrap();
        assert_eq!(work.work_title, Some("Symphony No. 5".to_string()));

        // Should have title credit
        assert!(!score.credits.is_empty());
    }

    #[test]
    fn test_compile_score_with_composer() {
        let sexpr = parse(r#"(score :composer "Beethoven")"#).unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert!(score.identification.is_some());
        let ident = score.identification.unwrap();
        assert_eq!(ident.creators.len(), 1);
        assert_eq!(ident.creators[0].value, "Beethoven");
        assert_eq!(ident.creators[0].r#type, Some("composer".to_string()));
    }

    #[test]
    fn test_compile_score_with_part() {
        let sexpr = parse(r#"(score (part :name "Piano"))"#).unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].id, "P1");
        assert_eq!(score.part_list.content.len(), 1);

        if let PartListElement::ScorePart(sp) = &score.part_list.content[0] {
            assert_eq!(sp.id, "P1");
            assert_eq!(sp.part_name.value, "Piano");
        } else {
            panic!("Expected ScorePart");
        }
    }

    #[test]
    fn test_compile_score_with_multiple_parts() {
        let sexpr = parse(r#"(score (part :violin) (part :cello))"#).unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert_eq!(score.parts.len(), 2);
        assert_eq!(score.parts[0].id, "P1");
        assert_eq!(score.parts[1].id, "P2");

        assert_eq!(score.part_list.content.len(), 2);
    }

    #[test]
    fn test_compile_score_with_measures() {
        let sexpr = parse(r#"(score (part :piano (measure (note c4 :q)) (measure (note d4 :q))))"#)
            .unwrap();
        let score = compile_score(&sexpr).unwrap();

        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 2);
        assert_eq!(score.parts[0].measures[0].number, "1");
        assert_eq!(score.parts[0].measures[1].number, "2");
    }

    // === compile_fermata_score tests ===

    #[test]
    fn test_compile_fermata_score_basic() {
        let fermata_score = FermataScore {
            title: None,
            composer: None,
            parts: vec![],
        };

        let score = compile_fermata_score(&fermata_score).unwrap();

        assert_eq!(score.version, Some("4.0".to_string()));
        assert!(score.work.is_none());
        assert!(score.identification.is_none());
    }

    #[test]
    fn test_compile_fermata_score_with_title() {
        let fermata_score = FermataScore {
            title: Some("Test Title".to_string()),
            composer: None,
            parts: vec![],
        };

        let score = compile_fermata_score(&fermata_score).unwrap();

        assert!(score.work.is_some());
        assert_eq!(
            score.work.unwrap().work_title,
            Some("Test Title".to_string())
        );

        // Should have title credit
        assert_eq!(score.credits.len(), 1);
    }

    #[test]
    fn test_compile_fermata_score_with_composer() {
        let fermata_score = FermataScore {
            title: None,
            composer: Some("Test Composer".to_string()),
            parts: vec![],
        };

        let score = compile_fermata_score(&fermata_score).unwrap();

        assert!(score.identification.is_some());

        // Should have composer credit
        assert_eq!(score.credits.len(), 1);
    }

    #[test]
    fn test_compile_fermata_score_with_title_and_composer() {
        let fermata_score = FermataScore {
            title: Some("Title".to_string()),
            composer: Some("Composer".to_string()),
            parts: vec![],
        };

        let score = compile_fermata_score(&fermata_score).unwrap();

        // Should have both title and composer credits
        assert_eq!(score.credits.len(), 2);
    }

    #[test]
    fn test_compile_fermata_score_with_parts() {
        let fermata_score = FermataScore {
            title: None,
            composer: None,
            parts: vec![
                FermataPart {
                    name: "Violin".to_string(),
                    id: None,
                    abbreviation: None,
                    measures: vec![],
                },
                FermataPart {
                    name: "Cello".to_string(),
                    id: None,
                    abbreviation: None,
                    measures: vec![],
                },
            ],
        };

        let score = compile_fermata_score(&fermata_score).unwrap();

        assert_eq!(score.parts.len(), 2);
        assert_eq!(score.part_list.content.len(), 2);

        // Part IDs should be P1, P2
        assert_eq!(score.parts[0].id, "P1");
        assert_eq!(score.parts[1].id, "P2");
    }

    #[test]
    fn test_compile_fermata_score_with_measures() {
        let fermata_score = FermataScore {
            title: None,
            composer: None,
            parts: vec![FermataPart {
                name: "Piano".to_string(),
                id: None,
                abbreviation: None,
                measures: vec![FermataMeasure {
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
                }],
            }],
        };

        let score = compile_fermata_score(&fermata_score).unwrap();

        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 1);
    }

    // === parse_score_to_ast tests ===

    #[test]
    fn test_parse_score_to_ast_simple() {
        let sexpr = parse(r#"(score :title "Test")"#).unwrap();
        let ast = parse_score_to_ast(&sexpr).unwrap();
        assert_eq!(ast.title, Some("Test".to_string()));
    }

    #[test]
    fn test_parse_score_to_ast_with_parts() {
        let sexpr = parse("(score (part :piano) (part :violin))").unwrap();
        let ast = parse_score_to_ast(&sexpr).unwrap();
        assert_eq!(ast.parts.len(), 2);
    }

    // === Credit generation tests ===

    #[test]
    fn test_title_credit_has_center_alignment() {
        let sexpr = parse(r#"(score :title "Test")"#).unwrap();
        let score = compile_score(&sexpr).unwrap();

        let title_credit = &score.credits[0];
        if let CreditContent::CreditWords(cw) = &title_credit.content[1] {
            assert_eq!(cw.justify, Some(LeftCenterRight::Center));
            assert_eq!(cw.halign, Some(LeftCenterRight::Center));
        } else {
            panic!("Expected CreditWords");
        }
    }

    #[test]
    fn test_composer_credit_has_right_alignment() {
        let sexpr = parse(r#"(score :composer "Test")"#).unwrap();
        let score = compile_score(&sexpr).unwrap();

        let composer_credit = &score.credits[0];
        if let CreditContent::CreditWords(cw) = &composer_credit.content[1] {
            assert_eq!(cw.justify, Some(LeftCenterRight::Right));
            assert_eq!(cw.halign, Some(LeftCenterRight::Right));
        } else {
            panic!("Expected CreditWords");
        }
    }
}
