//! Part compilation for Fermata syntax.
//!
//! This module handles compiling part S-expressions into IR Part types.
//! It generates both the Part (containing measures) and ScorePart (metadata)
//! for use in the part-list.

use crate::ir::attributes::{
    Attributes, Clef, ClefSign, Key, KeyContent, Mode, Time, TimeContent, TimeSignature,
    TraditionalKey,
};
use crate::ir::common::{Editorial, PrintStyle};
use crate::ir::measure::{Measure, MusicDataElement};
use crate::ir::part::{Part, PartListElement, PartName, ScorePart};
use crate::lang::ast::{FermataMeasure, FermataPart};
use crate::lang::defaults::{generate_part_id, DEFAULT_DIVISIONS};
use crate::lang::error::{CompileError, CompileResult};
use crate::lang::measure::{compile_fermata_measure, parse_measure_from_sexpr};
use crate::sexpr::Sexpr;

/// Result of compiling a part, containing both the Part and its ScorePart metadata.
#[derive(Debug, Clone)]
pub struct CompiledPart {
    /// The Part containing measures
    pub part: Part,
    /// The ScorePart for the part-list
    pub score_part: ScorePart,
}

/// Compile a part S-expression into IR Part and ScorePart.
///
/// # Arguments
///
/// * `sexpr` - The S-expression representing a part
/// * `index` - The zero-based index of this part (used to generate ID if not provided)
///
/// # Examples
///
/// ```rust,ignore
/// use fermata::lang::part::compile_part;
/// use fermata::sexpr::parse;
///
/// let sexpr = parse("(part :name \"Piano\" (measure (note c4 :q)))")?;
/// let compiled = compile_part(&sexpr, 0)?;
/// ```
pub fn compile_part(sexpr: &Sexpr, index: usize) -> CompileResult<CompiledPart> {
    let fermata_part = parse_part_from_sexpr(sexpr, index)?;
    compile_fermata_part(&fermata_part, index)
}

/// Parse a part S-expression into a FermataPart AST.
///
/// Expected format: `(part :name "Name" [:id "P1"] [:abbreviation "Abbr."] content...)`
pub fn parse_part_from_sexpr(sexpr: &Sexpr, index: usize) -> CompileResult<FermataPart> {
    let items = sexpr
        .as_list()
        .ok_or_else(|| CompileError::UnknownForm(format!("expected part list, got {:?}", sexpr)))?;

    if items.is_empty() {
        return Err(CompileError::UnknownForm("empty part list".to_string()));
    }

    // Check for 'part' head
    if let Some(head) = items[0].as_symbol() {
        if head != "part" {
            return Err(CompileError::UnknownForm(format!(
                "expected 'part', got '{}'",
                head
            )));
        }
    } else {
        return Err(CompileError::UnknownForm(format!(
            "expected 'part' symbol, got {:?}",
            items[0]
        )));
    }

    // Parse part attributes and content
    let mut name: Option<String> = None;
    let mut id: Option<String> = None;
    let mut abbreviation: Option<String> = None;
    let mut measures: Vec<FermataMeasure> = Vec::new();
    let mut measure_number = 1u32;

    let mut i = 1;
    while i < items.len() {
        // Check for keyword arguments
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "name" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::MissingField("part name value"));
                    }
                    name = Some(
                        items[i + 1]
                            .as_string()
                            .ok_or_else(|| {
                                CompileError::type_mismatch("string", format!("{:?}", items[i + 1]))
                            })?
                            .to_string(),
                    );
                    i += 2;
                }
                "id" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::MissingField("part id value"));
                    }
                    id = Some(
                        items[i + 1]
                            .as_string()
                            .or_else(|| items[i + 1].as_symbol())
                            .ok_or_else(|| {
                                CompileError::type_mismatch("string", format!("{:?}", items[i + 1]))
                            })?
                            .to_string(),
                    );
                    i += 2;
                }
                "abbreviation" | "abbr" => {
                    if i + 1 >= items.len() {
                        return Err(CompileError::MissingField("part abbreviation value"));
                    }
                    abbreviation = Some(
                        items[i + 1]
                            .as_string()
                            .ok_or_else(|| {
                                CompileError::type_mismatch("string", format!("{:?}", items[i + 1]))
                            })?
                            .to_string(),
                    );
                    i += 2;
                }
                // Check for instrument shortcuts (e.g., :piano)
                _ => {
                    // Instrument shortcuts set name
                    if name.is_none() {
                        name = Some(instrument_from_keyword(kw));
                    }
                    i += 1;
                }
            }
        } else if let Some(list) = items[i].as_list() {
            // Check if it's a measure
            if !list.is_empty() {
                if let Some(head) = list[0].as_symbol() {
                    if head == "measure" {
                        let measure = parse_measure_from_sexpr(&items[i], measure_number)?;
                        measures.push(measure);
                        measure_number += 1;
                        i += 1;
                        continue;
                    }
                }
            }
            // Not a measure - skip unknown list
            i += 1;
        } else {
            // Skip unknown items
            i += 1;
        }
    }

    // Use default name if not provided
    let part_name = name.unwrap_or_else(|| format!("Part {}", index + 1));

    Ok(FermataPart {
        name: part_name,
        id,
        abbreviation,
        measures,
    })
}

/// Convert an instrument keyword to a display name.
fn instrument_from_keyword(kw: &str) -> String {
    match kw.to_lowercase().as_str() {
        "piano" => "Piano".to_string(),
        "violin" => "Violin".to_string(),
        "viola" => "Viola".to_string(),
        "cello" | "violoncello" => "Cello".to_string(),
        "bass" | "contrabass" | "double-bass" => "Double Bass".to_string(),
        "flute" => "Flute".to_string(),
        "oboe" => "Oboe".to_string(),
        "clarinet" => "Clarinet".to_string(),
        "bassoon" => "Bassoon".to_string(),
        "horn" | "french-horn" => "Horn".to_string(),
        "trumpet" => "Trumpet".to_string(),
        "trombone" => "Trombone".to_string(),
        "tuba" => "Tuba".to_string(),
        "voice" | "vocal" => "Voice".to_string(),
        "soprano" => "Soprano".to_string(),
        "alto" => "Alto".to_string(),
        "tenor" => "Tenor".to_string(),
        "baritone" => "Baritone".to_string(),
        "guitar" => "Guitar".to_string(),
        "harp" => "Harp".to_string(),
        "timpani" => "Timpani".to_string(),
        "percussion" | "perc" => "Percussion".to_string(),
        _ => {
            // Capitalize first letter
            let mut chars = kw.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().chain(chars).collect(),
                None => kw.to_string(),
            }
        }
    }
}

/// Compile a FermataPart AST to IR Part and ScorePart.
pub fn compile_fermata_part(part: &FermataPart, index: usize) -> CompileResult<CompiledPart> {
    // Determine part ID
    let part_id = part.id.clone().unwrap_or_else(|| generate_part_id(index));

    // Compile measures
    let mut ir_measures: Vec<Measure> = part
        .measures
        .iter()
        .map(compile_fermata_measure)
        .collect::<CompileResult<Vec<_>>>()?;

    // Ensure the first measure has attributes (required by Verovio)
    ensure_first_measure_has_attributes(&mut ir_measures);

    // Build Part
    let ir_part = Part {
        id: part_id.clone(),
        measures: ir_measures,
    };

    // Build ScorePart
    let score_part = ScorePart {
        id: part_id,
        identification: None,
        part_name: PartName {
            value: part.name.clone(),
            print_style: PrintStyle::default(),
            print_object: None,
            justify: None,
        },
        part_name_display: None,
        part_abbreviation: part.abbreviation.as_ref().map(|abbr| PartName {
            value: abbr.clone(),
            print_style: PrintStyle::default(),
            print_object: None,
            justify: None,
        }),
        part_abbreviation_display: None,
        group: vec![],
        score_instruments: vec![],
        midi_devices: vec![],
        midi_instruments: vec![],
    };

    Ok(CompiledPart {
        part: ir_part,
        score_part,
    })
}

/// Create a PartListElement from a ScorePart.
pub fn score_part_to_list_element(score_part: ScorePart) -> PartListElement {
    PartListElement::ScorePart(score_part)
}

/// Ensure the first measure has an attributes element.
///
/// MusicXML renderers like Verovio require the first measure to have an `<attributes>`
/// element containing at minimum: divisions, clef, key, and time signature.
/// If the first measure doesn't have attributes, this function prepends default ones.
fn ensure_first_measure_has_attributes(measures: &mut [Measure]) {
    if measures.is_empty() {
        return;
    }

    let first_measure = &mut measures[0];

    // Check if first measure already has attributes
    let has_attributes = first_measure
        .content
        .iter()
        .any(|e| matches!(e, MusicDataElement::Attributes(_)));

    if has_attributes {
        return;
    }

    // Create default attributes: treble clef, 4/4 time, C major
    let default_attributes = Attributes {
        editorial: Editorial::default(),
        divisions: Some(DEFAULT_DIVISIONS as u64),
        keys: vec![Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0, // C major
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        }],
        times: vec![Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: None,
            print_object: None,
        }],
        staves: None,
        part_symbol: None,
        instruments: None,
        clefs: vec![Clef {
            sign: ClefSign::G,
            line: Some(2), // Treble clef
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        }],
        staff_details: vec![],
        transpose: vec![],
        measure_styles: vec![],
    };

    // Prepend attributes to measure content
    first_measure
        .content
        .insert(0, MusicDataElement::Attributes(Box::new(default_attributes)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::ast::{FermataDuration, FermataNote, FermataPitch, MeasureElement, PitchStep};
    use crate::sexpr::parse;

    // === parse_part_from_sexpr tests ===

    #[test]
    fn test_parse_part_from_sexpr_simple() {
        let sexpr = parse("(part :name \"Piano\")").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.name, "Piano");
        assert!(part.id.is_none());
        assert!(part.abbreviation.is_none());
        assert!(part.measures.is_empty());
    }

    #[test]
    fn test_parse_part_from_sexpr_with_id() {
        let sexpr = parse("(part :name \"Piano\" :id \"P1\")").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.name, "Piano");
        assert_eq!(part.id, Some("P1".to_string()));
    }

    #[test]
    fn test_parse_part_from_sexpr_with_abbreviation() {
        let sexpr = parse("(part :name \"Piano\" :abbreviation \"Pno.\")").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.abbreviation, Some("Pno.".to_string()));
    }

    #[test]
    fn test_parse_part_from_sexpr_with_abbr() {
        let sexpr = parse("(part :name \"Piano\" :abbr \"Pno.\")").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.abbreviation, Some("Pno.".to_string()));
    }

    #[test]
    fn test_parse_part_from_sexpr_with_measure() {
        let sexpr = parse("(part :name \"Piano\" (measure (note c4 :q)))").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.name, "Piano");
        assert_eq!(part.measures.len(), 1);
    }

    #[test]
    fn test_parse_part_from_sexpr_with_multiple_measures() {
        let sexpr =
            parse("(part :name \"Piano\" (measure (note c4 :q)) (measure (note d4 :q)))").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.measures.len(), 2);
        assert_eq!(part.measures[0].number, Some(1));
        assert_eq!(part.measures[1].number, Some(2));
    }

    #[test]
    fn test_parse_part_from_sexpr_instrument_shortcut() {
        let sexpr = parse("(part :piano)").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.name, "Piano");
    }

    #[test]
    fn test_parse_part_from_sexpr_default_name() {
        let sexpr = parse("(part)").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 0).unwrap();
        assert_eq!(part.name, "Part 1");
    }

    #[test]
    fn test_parse_part_from_sexpr_default_name_index() {
        let sexpr = parse("(part)").unwrap();
        let part = parse_part_from_sexpr(&sexpr, 2).unwrap();
        assert_eq!(part.name, "Part 3");
    }

    #[test]
    fn test_parse_part_from_sexpr_not_list() {
        let sexpr = Sexpr::symbol("part");
        assert!(parse_part_from_sexpr(&sexpr, 0).is_err());
    }

    #[test]
    fn test_parse_part_from_sexpr_empty_list() {
        let sexpr = Sexpr::list(vec![]);
        assert!(parse_part_from_sexpr(&sexpr, 0).is_err());
    }

    #[test]
    fn test_parse_part_from_sexpr_wrong_head() {
        let sexpr = parse("(measure (note c4 :q))").unwrap();
        assert!(parse_part_from_sexpr(&sexpr, 0).is_err());
    }

    // === instrument_from_keyword tests ===

    #[test]
    fn test_instrument_from_keyword_piano() {
        assert_eq!(instrument_from_keyword("piano"), "Piano");
    }

    #[test]
    fn test_instrument_from_keyword_violin() {
        assert_eq!(instrument_from_keyword("violin"), "Violin");
    }

    #[test]
    fn test_instrument_from_keyword_cello() {
        assert_eq!(instrument_from_keyword("cello"), "Cello");
    }

    #[test]
    fn test_instrument_from_keyword_flute() {
        assert_eq!(instrument_from_keyword("flute"), "Flute");
    }

    #[test]
    fn test_instrument_from_keyword_guitar() {
        assert_eq!(instrument_from_keyword("guitar"), "Guitar");
    }

    #[test]
    fn test_instrument_from_keyword_voice() {
        assert_eq!(instrument_from_keyword("voice"), "Voice");
    }

    #[test]
    fn test_instrument_from_keyword_unknown() {
        // Should capitalize first letter
        assert_eq!(instrument_from_keyword("xylophone"), "Xylophone");
    }

    // === compile_part tests ===

    #[test]
    fn test_compile_part_simple() {
        let sexpr = parse("(part :name \"Piano\")").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        assert_eq!(compiled.part.id, "P1");
        assert!(compiled.part.measures.is_empty());
        assert_eq!(compiled.score_part.id, "P1");
        assert_eq!(compiled.score_part.part_name.value, "Piano");
    }

    #[test]
    fn test_compile_part_with_custom_id() {
        let sexpr = parse("(part :name \"Piano\" :id \"CustomID\")").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        assert_eq!(compiled.part.id, "CustomID");
        assert_eq!(compiled.score_part.id, "CustomID");
    }

    #[test]
    fn test_compile_part_with_abbreviation() {
        let sexpr = parse("(part :name \"Piano\" :abbreviation \"Pno.\")").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        assert!(compiled.score_part.part_abbreviation.is_some());
        assert_eq!(compiled.score_part.part_abbreviation.unwrap().value, "Pno.");
    }

    #[test]
    fn test_compile_part_with_measure() {
        let sexpr = parse("(part :name \"Piano\" (measure (note c4 :q)))").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        assert_eq!(compiled.part.measures.len(), 1);
        assert_eq!(compiled.part.measures[0].number, "1");
    }

    #[test]
    fn test_compile_part_id_generation() {
        let sexpr = parse("(part :name \"Piano\")").unwrap();
        let compiled0 = compile_part(&sexpr, 0).unwrap();
        let compiled1 = compile_part(&sexpr, 1).unwrap();
        let compiled2 = compile_part(&sexpr, 2).unwrap();

        assert_eq!(compiled0.part.id, "P1");
        assert_eq!(compiled1.part.id, "P2");
        assert_eq!(compiled2.part.id, "P3");
    }

    // === compile_fermata_part tests ===

    #[test]
    fn test_compile_fermata_part_basic() {
        let fermata_part = FermataPart {
            name: "Violin".to_string(),
            id: None,
            abbreviation: None,
            measures: vec![],
        };

        let compiled = compile_fermata_part(&fermata_part, 0).unwrap();

        assert_eq!(compiled.part.id, "P1");
        assert_eq!(compiled.score_part.part_name.value, "Violin");
        assert!(compiled.score_part.part_abbreviation.is_none());
    }

    #[test]
    fn test_compile_fermata_part_with_custom_id() {
        let fermata_part = FermataPart {
            name: "Violin".to_string(),
            id: Some("VLN1".to_string()),
            abbreviation: None,
            measures: vec![],
        };

        let compiled = compile_fermata_part(&fermata_part, 0).unwrap();

        assert_eq!(compiled.part.id, "VLN1");
        assert_eq!(compiled.score_part.id, "VLN1");
    }

    #[test]
    fn test_compile_fermata_part_with_abbreviation() {
        let fermata_part = FermataPart {
            name: "Violin I".to_string(),
            id: None,
            abbreviation: Some("Vln. I".to_string()),
            measures: vec![],
        };

        let compiled = compile_fermata_part(&fermata_part, 0).unwrap();

        assert!(compiled.score_part.part_abbreviation.is_some());
        assert_eq!(
            compiled.score_part.part_abbreviation.unwrap().value,
            "Vln. I"
        );
    }

    #[test]
    fn test_compile_fermata_part_with_measures() {
        let fermata_part = FermataPart {
            name: "Piano".to_string(),
            id: None,
            abbreviation: None,
            measures: vec![
                crate::lang::ast::FermataMeasure {
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
                },
                crate::lang::ast::FermataMeasure {
                    number: Some(2),
                    content: vec![],
                },
            ],
        };

        let compiled = compile_fermata_part(&fermata_part, 0).unwrap();

        assert_eq!(compiled.part.measures.len(), 2);
        assert_eq!(compiled.part.measures[0].number, "1");
        assert_eq!(compiled.part.measures[1].number, "2");
    }

    // === score_part_to_list_element tests ===

    #[test]
    fn test_score_part_to_list_element() {
        let score_part = ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Piano".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            },
            part_name_display: None,
            part_abbreviation: None,
            part_abbreviation_display: None,
            group: vec![],
            score_instruments: vec![],
            midi_devices: vec![],
            midi_instruments: vec![],
        };

        let element = score_part_to_list_element(score_part);

        if let PartListElement::ScorePart(sp) = element {
            assert_eq!(sp.id, "P1");
        } else {
            panic!("Expected ScorePart");
        }
    }

    // === ensure_first_measure_has_attributes tests ===

    #[test]
    fn test_ensure_first_measure_has_attributes_empty_measures() {
        let mut measures: Vec<Measure> = vec![];
        ensure_first_measure_has_attributes(&mut measures);
        assert!(measures.is_empty());
    }

    #[test]
    fn test_ensure_first_measure_has_attributes_adds_default() {
        let mut measures = vec![Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![], // No attributes
        }];

        ensure_first_measure_has_attributes(&mut measures);

        // Should now have attributes as first element
        assert_eq!(measures[0].content.len(), 1);
        assert!(matches!(
            measures[0].content[0],
            MusicDataElement::Attributes(_)
        ));

        // Verify attributes content
        if let MusicDataElement::Attributes(attrs) = &measures[0].content[0] {
            assert!(attrs.divisions.is_some());
            assert!(!attrs.clefs.is_empty());
            assert!(!attrs.times.is_empty());
            assert!(!attrs.keys.is_empty());
        }
    }

    #[test]
    fn test_ensure_first_measure_has_attributes_does_not_duplicate() {
        use crate::ir::attributes::Attributes;

        // Create measure with existing attributes
        let existing_attrs = Attributes {
            editorial: Editorial::default(),
            divisions: Some(480), // Different from default
            keys: vec![],
            times: vec![],
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        let mut measures = vec![Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![MusicDataElement::Attributes(Box::new(existing_attrs))],
        }];

        ensure_first_measure_has_attributes(&mut measures);

        // Should still only have one attributes element
        assert_eq!(measures[0].content.len(), 1);

        // Divisions should still be 480, not DEFAULT_DIVISIONS
        if let MusicDataElement::Attributes(attrs) = &measures[0].content[0] {
            assert_eq!(attrs.divisions, Some(480));
        }
    }

    #[test]
    fn test_ensure_first_measure_has_attributes_prepends() {
        // Compile a real part with a note but no explicit attributes
        let sexpr = parse("(part :piano (measure (note c4 :q)))").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        let first_measure = &compiled.part.measures[0];

        // First element should be attributes (auto-added)
        assert!(matches!(
            first_measure.content[0],
            MusicDataElement::Attributes(_)
        ));

        // Second element should be the note
        assert!(matches!(
            first_measure.content[1],
            MusicDataElement::Note(_)
        ));
    }

    #[test]
    fn test_ensure_first_measure_attributes_have_required_fields() {
        // Compile a part with no explicit attributes
        let sexpr = parse("(part :piano (measure (note c4 :q)))").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        // Get the auto-generated attributes
        if let MusicDataElement::Attributes(attrs) = &compiled.part.measures[0].content[0] {
            // Verify divisions is set
            assert!(attrs.divisions.is_some());

            // Verify clef is set (treble clef)
            assert_eq!(attrs.clefs.len(), 1);
            assert_eq!(attrs.clefs[0].sign, ClefSign::G);
            assert_eq!(attrs.clefs[0].line, Some(2));

            // Verify time signature is set (4/4)
            assert_eq!(attrs.times.len(), 1);
            if let TimeContent::Measured { signatures } = &attrs.times[0].content {
                assert_eq!(signatures[0].beats, "4");
                assert_eq!(signatures[0].beat_type, "4");
            } else {
                panic!("Expected measured time");
            }

            // Verify key is set (C major)
            assert_eq!(attrs.keys.len(), 1);
            if let KeyContent::Traditional(key) = &attrs.keys[0].content {
                assert_eq!(key.fifths, 0);
                assert_eq!(key.mode, Some(Mode::Major));
            } else {
                panic!("Expected traditional key");
            }
        } else {
            panic!("Expected Attributes element");
        }
    }

    #[test]
    fn test_explicit_attributes_not_overridden() {
        // Compile a part with explicit attributes (G major, 3/4, bass clef)
        let sexpr = parse("(part :piano (measure (key g :major) (time 3 4) (clef :bass) (note c4 :q)))").unwrap();
        let compiled = compile_part(&sexpr, 0).unwrap();

        // Get the attributes (should be the explicit ones, not defaults)
        if let MusicDataElement::Attributes(attrs) = &compiled.part.measures[0].content[0] {
            // Verify key is G major (1 sharp)
            if let KeyContent::Traditional(key) = &attrs.keys[0].content {
                assert_eq!(key.fifths, 1); // G major = 1 sharp
            }

            // Verify time is 3/4
            if let TimeContent::Measured { signatures } = &attrs.times[0].content {
                assert_eq!(signatures[0].beats, "3");
                assert_eq!(signatures[0].beat_type, "4");
            }

            // Verify bass clef
            assert_eq!(attrs.clefs[0].sign, ClefSign::F);
        } else {
            panic!("Expected Attributes element");
        }

        // Should only have one Attributes element (no duplicate)
        let attr_count = compiled.part.measures[0]
            .content
            .iter()
            .filter(|e| matches!(e, MusicDataElement::Attributes(_)))
            .count();
        assert_eq!(attr_count, 1);
    }
}
