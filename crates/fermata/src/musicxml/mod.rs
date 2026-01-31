//! MusicXML parsing and emission module.
//!
//! This module provides functionality to:
//! - Parse MusicXML 4.0 documents into IR types
//! - Emit MusicXML 4.0 documents from IR types
//!
//! # Parsing Example
//!
//! ```ignore
//! use fermata::musicxml::parse;
//!
//! let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
//! <score-partwise version="4.0">
//!   <part-list>
//!     <score-part id="P1"><part-name>Piano</part-name></score-part>
//!   </part-list>
//!   <part id="P1"><measure number="1"/></part>
//! </score-partwise>"#;
//!
//! let score = parse(xml)?;
//! println!("Parsed score with {} parts", score.parts.len());
//! ```
//!
//! # Emission Example
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
mod parse;
mod reader;
mod values;
mod writer;

pub use divisions::{
    STANDARD_DIVISIONS, apply_dots, apply_time_modification, calculate_duration,
    note_type_to_divisions,
};
pub use emit::{emit_score, note_type_value_to_string};
pub use parse::parse_score;

use crate::ir::ScorePartwise;

/// Parse a MusicXML document from a string.
///
/// This is the main entry point for parsing MusicXML. It accepts a complete
/// MusicXML document string and returns the parsed score.
///
/// # Arguments
///
/// * `xml` - The MusicXML document as a string
///
/// # Returns
///
/// A `Result` containing the parsed `ScorePartwise` or a `ParseError`
///
/// # Errors
///
/// Returns `ParseError::Xml` if the XML is malformed.
/// Returns `ParseError::MissingElement` if a required element is missing.
/// Returns `ParseError::MissingAttribute` if a required attribute is missing.
/// Returns `ParseError::InvalidValue` if a value cannot be parsed.
/// Returns `ParseError::UnexpectedElement` if an unexpected element is encountered.
/// Returns `ParseError::UndefinedReference` if a reference (like part ID) is undefined.
///
/// # Example
///
/// ```ignore
/// use fermata::musicxml::parse;
///
/// let xml = include_str!("../tests/fixtures/simple.musicxml");
/// let score = parse(xml)?;
/// ```
pub fn parse(xml: &str) -> Result<ScorePartwise, ParseError> {
    parse::parse_score(xml)
}

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

/// Errors that can occur during MusicXML parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Error from the XML parser (malformed XML).
    Xml {
        /// Error message from the XML parser
        message: String,
        /// Byte position in the input where the error occurred
        position: usize,
    },
    /// A required element is missing.
    MissingElement {
        /// Name of the missing element
        element: String,
        /// Name of the parent element
        parent: String,
        /// Byte position in the input
        position: usize,
    },
    /// A required attribute is missing.
    MissingAttribute {
        /// Name of the missing attribute
        attribute: String,
        /// Name of the element
        element: String,
        /// Byte position in the input
        position: usize,
    },
    /// A value could not be parsed (invalid format or out of range).
    InvalidValue {
        /// Description of the expected value
        expected: String,
        /// The actual value that was found
        found: String,
        /// Byte position in the input
        position: usize,
    },
    /// An unexpected element was encountered.
    UnexpectedElement {
        /// Name of the unexpected element
        element: String,
        /// Name of the parent element
        parent: String,
        /// Byte position in the input
        position: usize,
    },
    /// A reference to an undefined entity (e.g., part ID not in part-list).
    UndefinedReference {
        /// Type of reference (e.g., "part", "score-part")
        reference_type: String,
        /// The ID that was not found
        id: String,
        /// Byte position in the input
        position: usize,
    },
    /// Any other error.
    Other {
        /// Error message
        message: String,
        /// Byte position in the input (if available)
        position: Option<usize>,
    },
}

impl ParseError {
    /// Get the byte position in the input where the error occurred, if available.
    ///
    /// This can be used to provide better error messages with line/column information.
    #[must_use]
    pub fn position(&self) -> Option<usize> {
        match self {
            ParseError::Xml { position, .. } => Some(*position),
            ParseError::MissingElement { position, .. } => Some(*position),
            ParseError::MissingAttribute { position, .. } => Some(*position),
            ParseError::InvalidValue { position, .. } => Some(*position),
            ParseError::UnexpectedElement { position, .. } => Some(*position),
            ParseError::UndefinedReference { position, .. } => Some(*position),
            ParseError::Other { position, .. } => *position,
        }
    }

    /// Create a new Xml error.
    pub(crate) fn xml(message: impl Into<String>, position: usize) -> Self {
        ParseError::Xml {
            message: message.into(),
            position,
        }
    }

    /// Create a new MissingElement error.
    pub(crate) fn missing_element(
        element: impl Into<String>,
        parent: impl Into<String>,
        position: usize,
    ) -> Self {
        ParseError::MissingElement {
            element: element.into(),
            parent: parent.into(),
            position,
        }
    }

    /// Create a new MissingAttribute error.
    pub(crate) fn missing_attribute(
        attribute: impl Into<String>,
        element: impl Into<String>,
        position: usize,
    ) -> Self {
        ParseError::MissingAttribute {
            attribute: attribute.into(),
            element: element.into(),
            position,
        }
    }

    /// Create a new InvalidValue error.
    pub(crate) fn invalid_value(
        expected: impl Into<String>,
        found: impl Into<String>,
        position: usize,
    ) -> Self {
        ParseError::InvalidValue {
            expected: expected.into(),
            found: found.into(),
            position,
        }
    }

    /// Create a new UnexpectedElement error.
    pub(crate) fn unexpected_element(
        element: impl Into<String>,
        parent: impl Into<String>,
        position: usize,
    ) -> Self {
        ParseError::UnexpectedElement {
            element: element.into(),
            parent: parent.into(),
            position,
        }
    }

    /// Create a new UndefinedReference error.
    pub(crate) fn undefined_reference(
        reference_type: impl Into<String>,
        id: impl Into<String>,
        position: usize,
    ) -> Self {
        ParseError::UndefinedReference {
            reference_type: reference_type.into(),
            id: id.into(),
            position,
        }
    }

    /// Create a new Other error.
    pub(crate) fn other(message: impl Into<String>, position: Option<usize>) -> Self {
        ParseError::Other {
            message: message.into(),
            position,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Xml { message, position } => {
                write!(f, "XML parse error at byte {}: {}", position, message)
            }
            ParseError::MissingElement {
                element,
                parent,
                position,
            } => {
                write!(
                    f,
                    "Missing required element <{}> in <{}> at byte {}",
                    element, parent, position
                )
            }
            ParseError::MissingAttribute {
                attribute,
                element,
                position,
            } => {
                write!(
                    f,
                    "Missing required attribute '{}' on <{}> at byte {}",
                    attribute, element, position
                )
            }
            ParseError::InvalidValue {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Invalid value at byte {}: expected {}, found '{}'",
                    position, expected, found
                )
            }
            ParseError::UnexpectedElement {
                element,
                parent,
                position,
            } => {
                write!(
                    f,
                    "Unexpected element <{}> in <{}> at byte {}",
                    element, parent, position
                )
            }
            ParseError::UndefinedReference {
                reference_type,
                id,
                position,
            } => {
                write!(
                    f,
                    "Undefined {} reference '{}' at byte {}",
                    reference_type, id, position
                )
            }
            ParseError::Other { message, position } => {
                if let Some(pos) = position {
                    write!(f, "Parse error at byte {}: {}", pos, message)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<quick_xml::Error> for ParseError {
    fn from(err: quick_xml::Error) -> Self {
        ParseError::Xml {
            message: err.to_string(),
            position: 0,
        }
    }
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

    // === ParseError Tests ===

    #[test]
    fn test_parse_error_xml_display() {
        let err = ParseError::xml("unexpected end of input", 42);
        assert_eq!(
            format!("{}", err),
            "XML parse error at byte 42: unexpected end of input"
        );
    }

    #[test]
    fn test_parse_error_xml_position() {
        let err = ParseError::xml("test", 100);
        assert_eq!(err.position(), Some(100));
    }

    #[test]
    fn test_parse_error_missing_element_display() {
        let err = ParseError::missing_element("pitch", "note", 50);
        assert_eq!(
            format!("{}", err),
            "Missing required element <pitch> in <note> at byte 50"
        );
    }

    #[test]
    fn test_parse_error_missing_element_position() {
        let err = ParseError::missing_element("step", "pitch", 75);
        assert_eq!(err.position(), Some(75));
    }

    #[test]
    fn test_parse_error_missing_attribute_display() {
        let err = ParseError::missing_attribute("id", "part", 30);
        assert_eq!(
            format!("{}", err),
            "Missing required attribute 'id' on <part> at byte 30"
        );
    }

    #[test]
    fn test_parse_error_missing_attribute_position() {
        let err = ParseError::missing_attribute("number", "measure", 120);
        assert_eq!(err.position(), Some(120));
    }

    #[test]
    fn test_parse_error_invalid_value_display() {
        let err = ParseError::invalid_value("note-type-value", "invalid", 200);
        assert_eq!(
            format!("{}", err),
            "Invalid value at byte 200: expected note-type-value, found 'invalid'"
        );
    }

    #[test]
    fn test_parse_error_invalid_value_position() {
        let err = ParseError::invalid_value("integer", "abc", 150);
        assert_eq!(err.position(), Some(150));
    }

    #[test]
    fn test_parse_error_unexpected_element_display() {
        let err = ParseError::unexpected_element("foo", "measure", 80);
        assert_eq!(
            format!("{}", err),
            "Unexpected element <foo> in <measure> at byte 80"
        );
    }

    #[test]
    fn test_parse_error_unexpected_element_position() {
        let err = ParseError::unexpected_element("bar", "part", 90);
        assert_eq!(err.position(), Some(90));
    }

    #[test]
    fn test_parse_error_undefined_reference_display() {
        let err = ParseError::undefined_reference("part", "P2", 300);
        assert_eq!(
            format!("{}", err),
            "Undefined part reference 'P2' at byte 300"
        );
    }

    #[test]
    fn test_parse_error_undefined_reference_position() {
        let err = ParseError::undefined_reference("score-part", "P1", 250);
        assert_eq!(err.position(), Some(250));
    }

    #[test]
    fn test_parse_error_other_with_position_display() {
        let err = ParseError::other("something went wrong", Some(400));
        assert_eq!(
            format!("{}", err),
            "Parse error at byte 400: something went wrong"
        );
    }

    #[test]
    fn test_parse_error_other_without_position_display() {
        let err = ParseError::other("generic error", None);
        assert_eq!(format!("{}", err), "Parse error: generic error");
    }

    #[test]
    fn test_parse_error_other_position() {
        let err = ParseError::other("test", Some(500));
        assert_eq!(err.position(), Some(500));

        let err = ParseError::other("test", None);
        assert_eq!(err.position(), None);
    }

    #[test]
    fn test_parse_error_clone() {
        let err = ParseError::xml("test", 10);
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_parse_error_debug() {
        let err = ParseError::xml("debug test", 5);
        let debug = format!("{:?}", err);
        assert!(debug.contains("Xml"));
        assert!(debug.contains("debug test"));
        assert!(debug.contains("5"));
    }

    #[test]
    fn test_parse_error_is_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<ParseError>();
    }

    // === EmitError Tests ===

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

    #[test]
    fn test_emit_error_clone() {
        let err = EmitError::XmlWrite("clone test".to_string());
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }

    #[test]
    fn test_emit_error_debug() {
        let err = EmitError::XmlWrite("debug test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("XmlWrite"));
        assert!(debug.contains("debug test"));
    }

    #[test]
    fn test_emit_error_is_error_trait() {
        fn assert_error<T: std::error::Error>() {}
        assert_error::<EmitError>();
    }
}
