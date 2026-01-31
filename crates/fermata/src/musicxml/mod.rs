//! MusicXML emission module.
//!
//! This module provides functionality to emit MusicXML 4.0 documents from the IR types.
//!
//! # Example
//!
//! ```ignore
//! use fermata::ir::ScorePartwise;
//! use fermata::musicxml::emit;
//!
//! let score: ScorePartwise = // ... create or parse a score
//! let xml = emit(&score)?;
//! println!("{}", xml);
//! ```

mod divisions;
mod emit;
mod writer;

pub use divisions::{
    STANDARD_DIVISIONS, apply_dots, apply_time_modification, calculate_duration,
    note_type_to_divisions,
};
pub use emit::emit_score;

use crate::ir::ScorePartwise;

/// Emit a MusicXML document from a ScorePartwise IR.
///
/// Returns the complete XML string including declaration and DOCTYPE.
///
/// # Arguments
///
/// * `score` - The score partwise IR to emit
///
/// # Returns
///
/// A `Result` containing the XML string or an `EmitError`
///
/// # Errors
///
/// Returns `EmitError::XmlWrite` if there's an error writing XML elements.
/// Returns `EmitError::InvalidData` if the IR contains invalid data.
pub fn emit(score: &ScorePartwise) -> Result<String, EmitError> {
    emit::emit_score(score)
}

/// Errors that can occur during MusicXML emission.
#[derive(Debug, Clone, PartialEq)]
pub enum EmitError {
    /// Error writing XML content.
    XmlWrite(String),
    /// Invalid data in the IR.
    InvalidData(String),
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::XmlWrite(msg) => write!(f, "XML write error: {}", msg),
            EmitError::InvalidData(msg) => write!(f, "Invalid IR data: {}", msg),
        }
    }
}

impl std::error::Error for EmitError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::part::{PartList, PartListElement, PartName, ScorePart};
    use crate::ir::score::ScorePartwise;
    use crate::ir::{Measure, Part, PrintStyle};

    #[test]
    fn test_emit_empty_score() {
        let score = ScorePartwise {
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
        };

        let result = emit(&score);
        assert!(result.is_ok());
        let xml = result.unwrap();

        // Verify XML declaration
        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));

        // Verify DOCTYPE
        assert!(xml.contains("<!DOCTYPE score-partwise"));
        assert!(xml.contains("MusicXML 4.0"));

        // Verify root element with version
        assert!(xml.contains("<score-partwise version=\"4.0\">"));

        // Verify part-list structure
        assert!(xml.contains("<part-list>"));
        assert!(xml.contains("<score-part id=\"P1\">"));
        assert!(xml.contains("<part-name>Piano</part-name>"));
        assert!(xml.contains("</score-part>"));
        assert!(xml.contains("</part-list>"));

        // Verify part structure
        assert!(xml.contains("<part id=\"P1\">"));
        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("</measure>"));
        assert!(xml.contains("</part>"));

        // Verify closing tag
        assert!(xml.contains("</score-partwise>"));
    }

    #[test]
    fn test_emit_error_display() {
        let err = EmitError::XmlWrite("test error".to_string());
        assert_eq!(format!("{}", err), "XML write error: test error");

        let err = EmitError::InvalidData("invalid data".to_string());
        assert_eq!(format!("{}", err), "Invalid IR data: invalid data");
    }
}
