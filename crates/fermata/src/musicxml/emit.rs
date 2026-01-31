//! Main MusicXML emission logic.
//!
//! This module contains the functions for emitting MusicXML from IR types.

use crate::ir::*;
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

/// Emit a complete MusicXML document from a ScorePartwise.
///
/// This function generates a complete MusicXML 4.0 partwise document including
/// the XML declaration, DOCTYPE, and all score content.
///
/// # Arguments
///
/// * `score` - The ScorePartwise IR to emit
///
/// # Returns
///
/// A `Result` containing the complete XML string or an `EmitError`
pub fn emit_score(score: &ScorePartwise) -> Result<String, EmitError> {
    let mut w = XmlWriter::new();

    w.write_header()
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // <score-partwise version="4.0">
    let mut root = ElementBuilder::new("score-partwise");
    if let Some(ref v) = score.version {
        root = root.attr("version", v);
    }
    w.write_start(root)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Score header elements (work, identification, defaults, credits, part-list)
    emit_score_header(&mut w, score)?;

    // Parts
    for part in &score.parts {
        emit_part(&mut w, part)?;
    }

    w.end_element("score-partwise")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.into_string()
        .map_err(|e| EmitError::XmlWrite(e.to_string()))
}

/// Emit the score header elements.
///
/// This includes work, movement-number, movement-title, identification,
/// defaults, credits, and part-list.
fn emit_score_header(w: &mut XmlWriter, score: &ScorePartwise) -> Result<(), EmitError> {
    // TODO: work
    // TODO: movement-number
    // TODO: movement-title
    // TODO: identification
    // TODO: defaults
    // TODO: credits

    emit_part_list(w, &score.part_list)?;
    Ok(())
}

/// Emit the part-list element.
fn emit_part_list(w: &mut XmlWriter, part_list: &PartList) -> Result<(), EmitError> {
    w.start_element("part-list")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for element in &part_list.content {
        match element {
            PartListElement::ScorePart(sp) => emit_score_part(w, sp)?,
            PartListElement::PartGroup(pg) => emit_part_group(w, pg)?,
        }
    }

    w.end_element("part-list")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a score-part element.
fn emit_score_part(w: &mut XmlWriter, sp: &ScorePart) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("score-part").attr("id", &sp.id);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // part-name is required
    w.text_element("part-name", &sp.part_name.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // TODO: part-name-display
    // TODO: part-abbreviation
    // TODO: part-abbreviation-display
    // TODO: group
    // TODO: score-instrument
    // TODO: midi-device
    // TODO: midi-instrument

    w.end_element("score-part")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a part-group element (stub).
fn emit_part_group(w: &mut XmlWriter, _pg: &PartGroup) -> Result<(), EmitError> {
    // TODO: implement part-group emission
    // For now, this is a stub that does nothing
    let _ = w;
    Ok(())
}

/// Emit a part element.
fn emit_part(w: &mut XmlWriter, part: &Part) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("part").attr("id", &part.id);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for measure in &part.measures {
        emit_measure(w, measure)?;
    }

    w.end_element("part")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a measure element.
fn emit_measure(w: &mut XmlWriter, measure: &Measure) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("measure").attr("number", &measure.number);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for element in &measure.content {
        emit_music_data(w, element)?;
    }

    w.end_element("measure")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a music data element.
///
/// This handles all variants of the MusicDataElement enum:
/// - Note
/// - Backup
/// - Forward
/// - Direction
/// - Attributes
/// - Barline
fn emit_music_data(w: &mut XmlWriter, element: &MusicDataElement) -> Result<(), EmitError> {
    match element {
        MusicDataElement::Note(note) => emit_note(w, note),
        MusicDataElement::Backup(backup) => emit_backup(w, backup),
        MusicDataElement::Forward(forward) => emit_forward(w, forward),
        MusicDataElement::Direction(dir) => emit_direction(w, dir),
        MusicDataElement::Attributes(attrs) => emit_attributes(w, attrs),
        MusicDataElement::Barline(barline) => emit_barline(w, barline),
    }
}

// Stub implementations - to be completed in later milestones

/// Emit a note element (stub).
fn emit_note(w: &mut XmlWriter, _note: &Note) -> Result<(), EmitError> {
    // TODO: implement note emission
    let _ = w;
    Ok(())
}

/// Emit a backup element (stub).
fn emit_backup(w: &mut XmlWriter, _backup: &Backup) -> Result<(), EmitError> {
    // TODO: implement backup emission
    let _ = w;
    Ok(())
}

/// Emit a forward element (stub).
fn emit_forward(w: &mut XmlWriter, _forward: &Forward) -> Result<(), EmitError> {
    // TODO: implement forward emission
    let _ = w;
    Ok(())
}

/// Emit a direction element (stub).
fn emit_direction(w: &mut XmlWriter, _dir: &Direction) -> Result<(), EmitError> {
    // TODO: implement direction emission
    let _ = w;
    Ok(())
}

/// Emit an attributes element (stub).
fn emit_attributes(w: &mut XmlWriter, _attrs: &Attributes) -> Result<(), EmitError> {
    // TODO: implement attributes emission
    let _ = w;
    Ok(())
}

/// Emit a barline element (stub).
fn emit_barline(w: &mut XmlWriter, _barline: &Barline) -> Result<(), EmitError> {
    // TODO: implement barline emission
    let _ = w;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::part::{PartList, PartListElement, PartName, ScorePart};

    fn create_minimal_score() -> ScorePartwise {
        ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList {
                content: vec![PartListElement::ScorePart(ScorePart {
                    id: "P1".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Test Part".to_string(),
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
                })],
            },
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![Measure {
                    number: "1".to_string(),
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    content: vec![],
                }],
            }],
        }
    }

    #[test]
    fn test_emit_score_structure() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        // Check XML declaration
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

        // Check DOCTYPE
        assert!(xml.contains("<!DOCTYPE score-partwise"));

        // Check root element
        assert!(xml.contains("<score-partwise version=\"4.0\">"));
        assert!(xml.contains("</score-partwise>"));
    }

    #[test]
    fn test_emit_score_part_list() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<part-list>"));
        assert!(xml.contains("</part-list>"));
    }

    #[test]
    fn test_emit_score_part() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<score-part id=\"P1\">"));
        assert!(xml.contains("<part-name>Test Part</part-name>"));
        assert!(xml.contains("</score-part>"));
    }

    #[test]
    fn test_emit_part() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<part id=\"P1\">"));
        assert!(xml.contains("</part>"));
    }

    #[test]
    fn test_emit_measure() {
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("</measure>"));
    }

    #[test]
    fn test_emit_score_without_version() {
        let mut score = create_minimal_score();
        score.version = None;
        let xml = emit_score(&score).unwrap();

        // Should have <score-partwise> without version attribute
        assert!(xml.contains("<score-partwise>"));
        // Verify no version attribute on score-partwise (but version= exists in XML declaration)
        assert!(!xml.contains("<score-partwise version="));
    }

    #[test]
    fn test_emit_multiple_measures() {
        let mut score = create_minimal_score();
        score.parts[0].measures.push(Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("<measure number=\"2\">"));
    }

    #[test]
    fn test_emit_multiple_parts() {
        let mut score = create_minimal_score();

        // Add second part to part-list
        score
            .part_list
            .content
            .push(PartListElement::ScorePart(ScorePart {
                id: "P2".to_string(),
                identification: None,
                part_name: PartName {
                    value: "Second Part".to_string(),
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
            }));

        // Add second part
        score.parts.push(Part {
            id: "P2".to_string(),
            measures: vec![Measure {
                number: "1".to_string(),
                implicit: None,
                non_controlling: None,
                width: None,
                content: vec![],
            }],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<score-part id=\"P1\">"));
        assert!(xml.contains("<score-part id=\"P2\">"));
        assert!(xml.contains("<part id=\"P1\">"));
        assert!(xml.contains("<part id=\"P2\">"));
    }

    #[test]
    fn test_emit_music_data_with_empty_content() {
        // Test that empty measure content works
        let score = create_minimal_score();
        let xml = emit_score(&score).unwrap();

        // Should have measure tags but no content between them
        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("</measure>"));
    }
}
