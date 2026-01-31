# Phase 3, Milestone 1: Foundation

> **Document Series:** 2 of 7
> **Tasks:** 1.1–1.4
> **Focus:** Module structure, XmlReader helper, value parsers, parsing skeleton

---

## Overview

This milestone establishes the parsing infrastructure:

- Error types and module organization
- `XmlReader` wrapper for convenient event processing
- Value parsers (XML strings → Rust enums)
- Top-level parsing skeleton

---

## Task 1.1: Create Module Structure

### Files to Create

```
src/musicxml/
├── mod.rs      # Update: add parse exports
├── parse.rs    # NEW: main parsing logic
├── reader.rs   # NEW: XmlReader helper
└── values.rs   # NEW: string → enum parsers
```

### Update `src/musicxml/mod.rs`

```rust
mod emit;
mod writer;
mod divisions;
mod parse;     // NEW
mod reader;    // NEW
mod values;    // NEW

pub use emit::emit_score;
pub use parse::parse_score;  // NEW

use crate::ir::ScorePartwise;

// Existing EmitError...

/// Parse a MusicXML document into a ScorePartwise IR.
pub fn parse(xml: &str) -> Result<ScorePartwise, ParseError> {
    parse::parse_score(xml)
}

/// Error type for MusicXML parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// XML syntax error from quick-xml
    Xml(String),

    /// Missing required element
    MissingElement {
        parent: String,
        element: String,
        position: usize
    },

    /// Missing required attribute
    MissingAttribute {
        element: String,
        attribute: String,
        position: usize
    },

    /// Invalid value for element or attribute
    InvalidValue {
        context: String,
        value: String,
        expected: String,
        position: usize
    },

    /// Unexpected element in context
    UnexpectedElement {
        context: String,
        element: String,
        position: usize
    },

    /// Reference to undefined ID
    UndefinedReference {
        ref_type: String,
        id: String,
        position: usize
    },

    /// Generic parse error
    Other(String),
}

impl ParseError {
    /// Get the byte position where the error occurred, if available
    pub fn position(&self) -> Option<usize> {
        match self {
            ParseError::Xml(_) => None,
            ParseError::MissingElement { position, .. } => Some(*position),
            ParseError::MissingAttribute { position, .. } => Some(*position),
            ParseError::InvalidValue { position, .. } => Some(*position),
            ParseError::UnexpectedElement { position, .. } => Some(*position),
            ParseError::UndefinedReference { position, .. } => Some(*position),
            ParseError::Other(_) => None,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Xml(msg) => write!(f, "XML error: {}", msg),
            ParseError::MissingElement { parent, element, position } => {
                write!(f, "Missing required <{}> in <{}> at byte {}", element, parent, position)
            }
            ParseError::MissingAttribute { element, attribute, position } => {
                write!(f, "Missing required '{}' attribute on <{}> at byte {}", attribute, element, position)
            }
            ParseError::InvalidValue { context, value, expected, position } => {
                write!(f, "Invalid value '{}' for {}: expected {} (at byte {})", value, context, expected, position)
            }
            ParseError::UnexpectedElement { context, element, position } => {
                write!(f, "Unexpected <{}> in {} at byte {}", element, context, position)
            }
            ParseError::UndefinedReference { ref_type, id, position } => {
                write!(f, "Undefined {} reference '{}' at byte {}", ref_type, id, position)
            }
            ParseError::Other(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}
```

**Acceptance Criteria:**

- [ ] Module compiles
- [ ] `fermata::musicxml::parse()` is callable (can return placeholder error)
- [ ] `ParseError` has all variants with Display impl

---

## Task 1.2: Implement XmlReader Helper

The `XmlReader` wraps `quick-xml::Reader` with convenience methods that handle common patterns.

### `src/musicxml/reader.rs`

```rust
//! XML reader helper for parsing MusicXML documents.

use quick_xml::events::{BytesStart, BytesEnd, BytesText, Event};
use quick_xml::Reader;
use crate::musicxml::ParseError;

/// Wrapper around quick-xml Reader with convenience methods for MusicXML parsing.
pub struct XmlReader<'a> {
    reader: Reader<&'a [u8]>,
    buf: Vec<u8>,
    /// Peeked event for lookahead
    peeked: Option<Event<'static>>,
}

impl<'a> XmlReader<'a> {
    /// Create a new XmlReader from an XML string.
    pub fn new(xml: &'a str) -> Self {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        Self {
            reader,
            buf: Vec::with_capacity(1024),
            peeked: None,
        }
    }

    /// Get current byte position for error reporting.
    pub fn position(&self) -> usize {
        self.reader.buffer_position()
    }

    /// Read next event, skipping comments and processing instructions.
    /// Returns owned events to avoid lifetime issues.
    pub fn next_event(&mut self) -> Result<Event<'static>, ParseError> {
        // Return peeked event if available
        if let Some(event) = self.peeked.take() {
            return Ok(event);
        }

        loop {
            self.buf.clear();
            match self.reader.read_event_into(&mut self.buf) {
                Ok(event) => {
                    let owned = event.into_owned();
                    match &owned {
                        Event::Comment(_) | Event::PI(_) => continue,
                        _ => return Ok(owned),
                    }
                }
                Err(e) => {
                    return Err(ParseError::Xml(format!(
                        "{} at position {}", e, self.position()
                    )));
                }
            }
        }
    }

    /// Peek at next event without consuming it.
    pub fn peek_event(&mut self) -> Result<&Event<'static>, ParseError> {
        if self.peeked.is_none() {
            self.peeked = Some(self.next_event()?);
        }
        Ok(self.peeked.as_ref().unwrap())
    }

    /// Read text content until the end tag of the current element.
    /// Assumes we just consumed a Start event.
    pub fn read_text(&mut self) -> Result<String, ParseError> {
        let mut text = String::new();
        loop {
            match self.next_event()? {
                Event::Text(e) => {
                    text.push_str(&String::from_utf8_lossy(&e));
                }
                Event::CData(e) => {
                    text.push_str(&String::from_utf8_lossy(&e));
                }
                Event::End(_) => return Ok(text),
                Event::Start(e) => {
                    return Err(ParseError::UnexpectedElement {
                        context: "text content".to_string(),
                        element: element_name(&e),
                        position: self.position(),
                    });
                }
                Event::Eof => {
                    return Err(ParseError::Other(
                        "Unexpected EOF while reading text content".to_string()
                    ));
                }
                _ => {}
            }
        }
    }

    /// Read text content and parse as type T.
    pub fn read_text_as<T>(&mut self) -> Result<T, ParseError>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let text = self.read_text()?;
        text.trim().parse().map_err(|e: T::Err| ParseError::InvalidValue {
            context: "element content".to_string(),
            value: text.clone(),
            expected: format!("{} ({})", std::any::type_name::<T>(), e),
            position: self.position(),
        })
    }

    /// Skip the current element and all its children.
    /// Call this after receiving a Start event you want to ignore.
    pub fn skip_element(&mut self) -> Result<(), ParseError> {
        let mut depth = 1;
        loop {
            match self.next_event()? {
                Event::Start(_) => depth += 1,
                Event::End(_) => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                Event::Eof => {
                    return Err(ParseError::Other(
                        "Unexpected EOF while skipping element".to_string()
                    ));
                }
                _ => {}
            }
        }
    }

    /// Get a required attribute value from a start tag.
    pub fn get_attr(&self, element: &BytesStart, name: &str) -> Result<String, ParseError> {
        get_attr(element, name, self.position())
    }

    /// Get an optional attribute value from a start tag.
    pub fn get_optional_attr(&self, element: &BytesStart, name: &str) -> Option<String> {
        get_optional_attr(element, name)
    }

    /// Get an optional attribute and parse as type T.
    pub fn get_optional_attr_as<T>(&self, element: &BytesStart, name: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        get_optional_attr(element, name).and_then(|s| s.parse().ok())
    }
}

/// Get element name as String (for error messages).
pub fn element_name(e: &BytesStart) -> String {
    String::from_utf8_lossy(e.name().as_ref()).to_string()
}

/// Get element name from end tag.
pub fn end_element_name(e: &BytesEnd) -> String {
    String::from_utf8_lossy(e.name().as_ref()).to_string()
}

/// Get required attribute from a start tag (standalone function).
pub fn get_attr(element: &BytesStart, name: &str, position: usize) -> Result<String, ParseError> {
    for attr_result in element.attributes() {
        if let Ok(attr) = attr_result {
            if attr.key.as_ref() == name.as_bytes() {
                return Ok(String::from_utf8_lossy(&attr.value).to_string());
            }
        }
    }
    Err(ParseError::MissingAttribute {
        element: element_name(element),
        attribute: name.to_string(),
        position,
    })
}

/// Get optional attribute from a start tag (standalone function).
pub fn get_optional_attr(element: &BytesStart, name: &str) -> Option<String> {
    for attr_result in element.attributes() {
        if let Ok(attr) = attr_result {
            if attr.key.as_ref() == name.as_bytes() {
                return Some(String::from_utf8_lossy(&attr.value).to_string());
            }
        }
    }
    None
}

/// Check if a start tag matches a given name.
pub fn is_start(event: &Event, name: &str) -> bool {
    matches!(event, Event::Start(e) if e.name().as_ref() == name.as_bytes())
}

/// Check if an end tag matches a given name.
pub fn is_end(event: &Event, name: &str) -> bool {
    matches!(event, Event::End(e) if e.name().as_ref() == name.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_simple_element() {
        let xml = "<root><value>42</value></root>";
        let mut reader = XmlReader::new(xml);

        // Skip to <value>
        assert!(matches!(reader.next_event().unwrap(), Event::Start(e) if e.name().as_ref() == b"root"));
        assert!(matches!(reader.next_event().unwrap(), Event::Start(e) if e.name().as_ref() == b"value"));

        let value: i32 = reader.read_text_as().unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_skip_element() {
        let xml = "<root><skip><nested>content</nested></skip><keep>data</keep></root>";
        let mut reader = XmlReader::new(xml);

        // Consume <root>
        reader.next_event().unwrap();
        // Consume <skip>
        reader.next_event().unwrap();
        // Skip entire <skip> subtree
        reader.skip_element().unwrap();

        // Next should be <keep>
        match reader.next_event().unwrap() {
            Event::Start(e) => assert_eq!(e.name().as_ref(), b"keep"),
            _ => panic!("Expected Start event"),
        }
    }

    #[test]
    fn test_get_attributes() {
        let xml = r#"<note id="n1" voice="2"><pitch/></note>"#;
        let mut reader = XmlReader::new(xml);

        match reader.next_event().unwrap() {
            Event::Start(e) => {
                assert_eq!(reader.get_attr(&e, "id").unwrap(), "n1");
                assert_eq!(reader.get_optional_attr(&e, "voice"), Some("2".to_string()));
                assert_eq!(reader.get_optional_attr(&e, "missing"), None);
            }
            _ => panic!("Expected Start event"),
        }
    }

    #[test]
    fn test_position_tracking() {
        let xml = "<root>\n  <child>text</child>\n</root>";
        let mut reader = XmlReader::new(xml);

        reader.next_event().unwrap(); // <root>
        let pos_before = reader.position();
        reader.next_event().unwrap(); // <child>
        let pos_after = reader.position();

        assert!(pos_after > pos_before);
    }
}
```

**Acceptance Criteria:**

- [ ] All unit tests pass
- [ ] `read_text()` correctly handles nested elements (error)
- [ ] `skip_element()` handles arbitrary nesting depth
- [ ] Position tracking works for error messages

---

## Task 1.3: Implement Value Parsers

These functions convert XML string values to Rust enum variants. They're the inverse of the `*_to_string()` functions in the emitter.

### `src/musicxml/values.rs`

```rust
//! Value parsers for converting MusicXML string values to Rust types.

use crate::ir::*;
use crate::musicxml::ParseError;

/// Helper to create InvalidValue error
fn invalid_value(context: &str, value: &str, expected: &str) -> ParseError {
    ParseError::InvalidValue {
        context: context.to_string(),
        value: value.to_string(),
        expected: expected.to_string(),
        position: 0, // Caller should update position if needed
    }
}

// ============================================================================
// Note-related values
// ============================================================================

/// Parse note type: "whole", "half", "quarter", "eighth", "16th", etc.
pub fn parse_note_type_value(s: &str) -> Result<NoteTypeValue, ParseError> {
    match s.trim() {
        "maxima" => Ok(NoteTypeValue::Maxima),
        "long" => Ok(NoteTypeValue::Long),
        "breve" => Ok(NoteTypeValue::Breve),
        "whole" => Ok(NoteTypeValue::Whole),
        "half" => Ok(NoteTypeValue::Half),
        "quarter" => Ok(NoteTypeValue::Quarter),
        "eighth" => Ok(NoteTypeValue::Eighth),
        "16th" => Ok(NoteTypeValue::N16th),
        "32nd" => Ok(NoteTypeValue::N32nd),
        "64th" => Ok(NoteTypeValue::N64th),
        "128th" => Ok(NoteTypeValue::N128th),
        "256th" => Ok(NoteTypeValue::N256th),
        "512th" => Ok(NoteTypeValue::N512th),
        "1024th" => Ok(NoteTypeValue::N1024th),
        _ => Err(invalid_value(
            "note type",
            s,
            "whole, half, quarter, eighth, 16th, 32nd, 64th, 128th, 256th, 512th, or 1024th",
        )),
    }
}

/// Parse pitch step: C, D, E, F, G, A, B
pub fn parse_step(s: &str) -> Result<Step, ParseError> {
    match s.trim() {
        "C" => Ok(Step::C),
        "D" => Ok(Step::D),
        "E" => Ok(Step::E),
        "F" => Ok(Step::F),
        "G" => Ok(Step::G),
        "A" => Ok(Step::A),
        "B" => Ok(Step::B),
        _ => Err(invalid_value("step", s, "C, D, E, F, G, A, or B")),
    }
}

/// Parse accidental value
pub fn parse_accidental_value(s: &str) -> Result<AccidentalValue, ParseError> {
    match s.trim() {
        "sharp" => Ok(AccidentalValue::Sharp),
        "natural" => Ok(AccidentalValue::Natural),
        "flat" => Ok(AccidentalValue::Flat),
        "double-sharp" => Ok(AccidentalValue::DoubleSharp),
        "sharp-sharp" => Ok(AccidentalValue::SharpSharp),
        "flat-flat" => Ok(AccidentalValue::FlatFlat),
        "double-flat" => Ok(AccidentalValue::DoubleFlat),
        "natural-sharp" => Ok(AccidentalValue::NaturalSharp),
        "natural-flat" => Ok(AccidentalValue::NaturalFlat),
        "quarter-flat" => Ok(AccidentalValue::QuarterFlat),
        "quarter-sharp" => Ok(AccidentalValue::QuarterSharp),
        "three-quarters-flat" => Ok(AccidentalValue::ThreeQuartersFlat),
        "three-quarters-sharp" => Ok(AccidentalValue::ThreeQuartersSharp),
        "sharp-down" => Ok(AccidentalValue::SharpDown),
        "sharp-up" => Ok(AccidentalValue::SharpUp),
        "natural-down" => Ok(AccidentalValue::NaturalDown),
        "natural-up" => Ok(AccidentalValue::NaturalUp),
        "flat-down" => Ok(AccidentalValue::FlatDown),
        "flat-up" => Ok(AccidentalValue::FlatUp),
        "triple-sharp" => Ok(AccidentalValue::TripleSharp),
        "triple-flat" => Ok(AccidentalValue::TripleFlat),
        // Add more as defined in IR
        _ => Err(invalid_value(
            "accidental",
            s,
            "sharp, flat, natural, double-sharp, double-flat, etc.",
        )),
    }
}

/// Parse stem value
pub fn parse_stem_value(s: &str) -> Result<StemValue, ParseError> {
    match s.trim() {
        "up" => Ok(StemValue::Up),
        "down" => Ok(StemValue::Down),
        "none" => Ok(StemValue::None),
        "double" => Ok(StemValue::Double),
        _ => Err(invalid_value("stem", s, "up, down, none, or double")),
    }
}

/// Parse beam value
pub fn parse_beam_value(s: &str) -> Result<BeamValue, ParseError> {
    match s.trim() {
        "begin" => Ok(BeamValue::Begin),
        "continue" => Ok(BeamValue::Continue),
        "end" => Ok(BeamValue::End),
        "forward hook" => Ok(BeamValue::ForwardHook),
        "backward hook" => Ok(BeamValue::BackwardHook),
        _ => Err(invalid_value(
            "beam",
            s,
            "begin, continue, end, forward hook, or backward hook",
        )),
    }
}

/// Parse notehead value
pub fn parse_notehead_value(s: &str) -> Result<NoteheadValue, ParseError> {
    match s.trim() {
        "slash" => Ok(NoteheadValue::Slash),
        "triangle" => Ok(NoteheadValue::Triangle),
        "diamond" => Ok(NoteheadValue::Diamond),
        "square" => Ok(NoteheadValue::Square),
        "cross" => Ok(NoteheadValue::Cross),
        "x" => Ok(NoteheadValue::X),
        "circle-x" => Ok(NoteheadValue::CircleX),
        "inverted triangle" => Ok(NoteheadValue::InvertedTriangle),
        "arrow down" => Ok(NoteheadValue::ArrowDown),
        "arrow up" => Ok(NoteheadValue::ArrowUp),
        "circled" => Ok(NoteheadValue::Circled),
        "slashed" => Ok(NoteheadValue::Slashed),
        "back slashed" => Ok(NoteheadValue::BackSlashed),
        "normal" => Ok(NoteheadValue::Normal),
        "cluster" => Ok(NoteheadValue::Cluster),
        "circle dot" => Ok(NoteheadValue::CircleDot),
        "left triangle" => Ok(NoteheadValue::LeftTriangle),
        "rectangle" => Ok(NoteheadValue::Rectangle),
        "none" => Ok(NoteheadValue::None),
        "do" => Ok(NoteheadValue::Do),
        "re" => Ok(NoteheadValue::Re),
        "mi" => Ok(NoteheadValue::Mi),
        "fa" => Ok(NoteheadValue::Fa),
        "fa up" => Ok(NoteheadValue::FaUp),
        "so" => Ok(NoteheadValue::So),
        "la" => Ok(NoteheadValue::La),
        "ti" => Ok(NoteheadValue::Ti),
        _ => Ok(NoteheadValue::Normal), // Default for unknown
    }
}

// ============================================================================
// Attributes-related values
// ============================================================================

/// Parse clef sign
pub fn parse_clef_sign(s: &str) -> Result<ClefSign, ParseError> {
    match s.trim() {
        "G" => Ok(ClefSign::G),
        "F" => Ok(ClefSign::F),
        "C" => Ok(ClefSign::C),
        "percussion" => Ok(ClefSign::Percussion),
        "TAB" => Ok(ClefSign::Tab),
        "jianpu" => Ok(ClefSign::Jianpu),
        "none" => Ok(ClefSign::None),
        _ => Err(invalid_value(
            "clef sign",
            s,
            "G, F, C, percussion, TAB, jianpu, or none",
        )),
    }
}

/// Parse mode
pub fn parse_mode(s: &str) -> Result<Mode, ParseError> {
    match s.trim().to_lowercase().as_str() {
        "major" => Ok(Mode::Major),
        "minor" => Ok(Mode::Minor),
        "dorian" => Ok(Mode::Dorian),
        "phrygian" => Ok(Mode::Phrygian),
        "lydian" => Ok(Mode::Lydian),
        "mixolydian" => Ok(Mode::Mixolydian),
        "aeolian" => Ok(Mode::Aeolian),
        "ionian" => Ok(Mode::Ionian),
        "locrian" => Ok(Mode::Locrian),
        "none" => Ok(Mode::None),
        _ => Err(invalid_value(
            "mode",
            s,
            "major, minor, dorian, phrygian, lydian, mixolydian, aeolian, ionian, locrian, or none",
        )),
    }
}

/// Parse time symbol
pub fn parse_time_symbol(s: &str) -> Result<TimeSymbol, ParseError> {
    match s.trim() {
        "common" => Ok(TimeSymbol::Common),
        "cut" => Ok(TimeSymbol::Cut),
        "single-number" => Ok(TimeSymbol::SingleNumber),
        "half-note" | "note" => Ok(TimeSymbol::Note),
        "dotted-note" => Ok(TimeSymbol::DottedNote),
        "normal" => Ok(TimeSymbol::Normal),
        _ => Err(invalid_value(
            "time symbol",
            s,
            "common, cut, single-number, note, dotted-note, or normal",
        )),
    }
}

// ============================================================================
// Common enumeration values
// ============================================================================

/// Parse yes/no to bool
pub fn parse_yes_no(s: &str) -> Result<bool, ParseError> {
    match s.trim() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(invalid_value("yes-no", s, "yes or no")),
    }
}

/// Parse start-stop
pub fn parse_start_stop(s: &str) -> Result<StartStop, ParseError> {
    match s.trim() {
        "start" => Ok(StartStop::Start),
        "stop" => Ok(StartStop::Stop),
        _ => Err(invalid_value("start-stop", s, "start or stop")),
    }
}

/// Parse start-stop-continue
pub fn parse_start_stop_continue(s: &str) -> Result<StartStopContinue, ParseError> {
    match s.trim() {
        "start" => Ok(StartStopContinue::Start),
        "stop" => Ok(StartStopContinue::Stop),
        "continue" => Ok(StartStopContinue::Continue),
        _ => Err(invalid_value("start-stop-continue", s, "start, stop, or continue")),
    }
}

/// Parse start-stop-discontinue (for endings)
pub fn parse_start_stop_discontinue(s: &str) -> Result<StartStopDiscontinue, ParseError> {
    match s.trim() {
        "start" => Ok(StartStopDiscontinue::Start),
        "stop" => Ok(StartStopDiscontinue::Stop),
        "discontinue" => Ok(StartStopDiscontinue::Discontinue),
        _ => Err(invalid_value("start-stop-discontinue", s, "start, stop, or discontinue")),
    }
}

/// Parse backward-forward (for repeats)
pub fn parse_backward_forward(s: &str) -> Result<BackwardForward, ParseError> {
    match s.trim() {
        "backward" => Ok(BackwardForward::Backward),
        "forward" => Ok(BackwardForward::Forward),
        _ => Err(invalid_value("backward-forward", s, "backward or forward")),
    }
}

/// Parse above-below (placement)
pub fn parse_above_below(s: &str) -> Result<AboveBelow, ParseError> {
    match s.trim() {
        "above" => Ok(AboveBelow::Above),
        "below" => Ok(AboveBelow::Below),
        _ => Err(invalid_value("above-below", s, "above or below")),
    }
}

/// Parse up-down (for stem, etc.)
pub fn parse_up_down(s: &str) -> Result<UpDown, ParseError> {
    match s.trim() {
        "up" => Ok(UpDown::Up),
        "down" => Ok(UpDown::Down),
        _ => Err(invalid_value("up-down", s, "up or down")),
    }
}

/// Parse over-under (for slur orientation)
pub fn parse_over_under(s: &str) -> Result<OverUnder, ParseError> {
    match s.trim() {
        "over" => Ok(OverUnder::Over),
        "under" => Ok(OverUnder::Under),
        _ => Err(invalid_value("over-under", s, "over or under")),
    }
}

// ============================================================================
// Barline-related values
// ============================================================================

/// Parse bar style
pub fn parse_bar_style(s: &str) -> Result<BarStyleValue, ParseError> {
    match s.trim() {
        "regular" => Ok(BarStyleValue::Regular),
        "dotted" => Ok(BarStyleValue::Dotted),
        "dashed" => Ok(BarStyleValue::Dashed),
        "heavy" => Ok(BarStyleValue::Heavy),
        "light-light" => Ok(BarStyleValue::LightLight),
        "light-heavy" => Ok(BarStyleValue::LightHeavy),
        "heavy-light" => Ok(BarStyleValue::HeavyLight),
        "heavy-heavy" => Ok(BarStyleValue::HeavyHeavy),
        "tick" => Ok(BarStyleValue::Tick),
        "short" => Ok(BarStyleValue::Short),
        "none" => Ok(BarStyleValue::None),
        _ => Err(invalid_value(
            "bar-style",
            s,
            "regular, dotted, dashed, heavy, light-light, light-heavy, heavy-light, heavy-heavy, tick, short, or none",
        )),
    }
}

/// Parse right-left-middle (barline location)
pub fn parse_right_left_middle(s: &str) -> Result<RightLeftMiddle, ParseError> {
    match s.trim() {
        "right" => Ok(RightLeftMiddle::Right),
        "left" => Ok(RightLeftMiddle::Left),
        "middle" => Ok(RightLeftMiddle::Middle),
        _ => Err(invalid_value("barline location", s, "right, left, or middle")),
    }
}

// ============================================================================
// Direction-related values
// ============================================================================

/// Parse wedge type
pub fn parse_wedge_type(s: &str) -> Result<WedgeType, ParseError> {
    match s.trim() {
        "crescendo" => Ok(WedgeType::Crescendo),
        "diminuendo" => Ok(WedgeType::Diminuendo),
        "stop" => Ok(WedgeType::Stop),
        "continue" => Ok(WedgeType::Continue),
        _ => Err(invalid_value("wedge type", s, "crescendo, diminuendo, stop, or continue")),
    }
}

/// Parse line type
pub fn parse_line_type(s: &str) -> Result<LineType, ParseError> {
    match s.trim() {
        "solid" => Ok(LineType::Solid),
        "dashed" => Ok(LineType::Dashed),
        "dotted" => Ok(LineType::Dotted),
        "wavy" => Ok(LineType::Wavy),
        _ => Err(invalid_value("line-type", s, "solid, dashed, dotted, or wavy")),
    }
}

// ============================================================================
// Notation-related values
// ============================================================================

/// Parse tied type
pub fn parse_tied_type(s: &str) -> Result<TiedType, ParseError> {
    match s.trim() {
        "start" => Ok(TiedType::Start),
        "stop" => Ok(TiedType::Stop),
        "continue" => Ok(TiedType::Continue),
        "let-ring" => Ok(TiedType::LetRing),
        _ => Err(invalid_value("tied type", s, "start, stop, continue, or let-ring")),
    }
}

/// Parse fermata shape
pub fn parse_fermata_shape(s: &str) -> Result<FermataShape, ParseError> {
    match s.trim() {
        "" | "normal" => Ok(FermataShape::Normal),
        "angled" => Ok(FermataShape::Angled),
        "square" => Ok(FermataShape::Square),
        "double-angled" => Ok(FermataShape::DoubleAngled),
        "double-square" => Ok(FermataShape::DoubleSquare),
        "double-dot" => Ok(FermataShape::DoubleDot),
        "half-curve" => Ok(FermataShape::HalfCurve),
        "curlew" => Ok(FermataShape::Curlew),
        _ => Ok(FermataShape::Normal), // Default for unknown
    }
}

/// Parse upright-inverted (fermata type)
pub fn parse_upright_inverted(s: &str) -> Result<UprightInverted, ParseError> {
    match s.trim() {
        "upright" => Ok(UprightInverted::Upright),
        "inverted" => Ok(UprightInverted::Inverted),
        _ => Err(invalid_value("upright-inverted", s, "upright or inverted")),
    }
}

/// Parse syllabic
pub fn parse_syllabic(s: &str) -> Result<Syllabic, ParseError> {
    match s.trim() {
        "single" => Ok(Syllabic::Single),
        "begin" => Ok(Syllabic::Begin),
        "middle" => Ok(Syllabic::Middle),
        "end" => Ok(Syllabic::End),
        _ => Err(invalid_value("syllabic", s, "single, begin, middle, or end")),
    }
}

/// Parse tremolo type
pub fn parse_tremolo_type(s: &str) -> Result<TremoloType, ParseError> {
    match s.trim() {
        "start" => Ok(TremoloType::Start),
        "stop" => Ok(TremoloType::Stop),
        "single" => Ok(TremoloType::Single),
        "unmeasured" => Ok(TremoloType::Unmeasured),
        _ => Err(invalid_value("tremolo type", s, "start, stop, single, or unmeasured")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note_type() {
        assert_eq!(parse_note_type_value("quarter").unwrap(), NoteTypeValue::Quarter);
        assert_eq!(parse_note_type_value("16th").unwrap(), NoteTypeValue::N16th);
        assert!(parse_note_type_value("invalid").is_err());
    }

    #[test]
    fn test_parse_step() {
        assert_eq!(parse_step("C").unwrap(), Step::C);
        assert_eq!(parse_step("G").unwrap(), Step::G);
        assert!(parse_step("X").is_err());
    }

    #[test]
    fn test_parse_yes_no() {
        assert!(parse_yes_no("yes").unwrap());
        assert!(!parse_yes_no("no").unwrap());
        assert!(parse_yes_no("maybe").is_err());
    }

    #[test]
    fn test_parse_mode_case_insensitive() {
        assert_eq!(parse_mode("Major").unwrap(), Mode::Major);
        assert_eq!(parse_mode("MINOR").unwrap(), Mode::Minor);
        assert_eq!(parse_mode("dorian").unwrap(), Mode::Dorian);
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(parse_step("  C  ").unwrap(), Step::C);
        assert_eq!(parse_note_type_value(" quarter ").unwrap(), NoteTypeValue::Quarter);
    }
}
```

**Acceptance Criteria:**

- [ ] All unit tests pass
- [ ] Every enum used in Phase 2 emitter has a corresponding parser
- [ ] Parsers handle leading/trailing whitespace
- [ ] Parsers return descriptive error messages

---

## Task 1.4: Implement Top-Level Parsing Skeleton

### `src/musicxml/parse.rs` (initial structure)

```rust
//! MusicXML parser - converts MusicXML documents to Fermata IR.

use quick_xml::events::Event;
use crate::ir::*;
use crate::musicxml::reader::{XmlReader, element_name, get_optional_attr};
use crate::musicxml::values::*;
use crate::musicxml::ParseError;

/// Parse a MusicXML document into a ScorePartwise IR.
pub fn parse_score(xml: &str) -> Result<ScorePartwise, ParseError> {
    let mut reader = XmlReader::new(xml);

    // Skip XML declaration, DOCTYPE, comments until we hit the root element
    loop {
        match reader.next_event()? {
            Event::Decl(_) | Event::DocType(_) => continue,
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"score-partwise" => {
                        return parse_score_partwise(&mut reader, &e);
                    }
                    b"score-timewise" => {
                        return Err(ParseError::Other(
                            "score-timewise format is not supported. \
                             Please convert to score-partwise using XSLT or notation software."
                                .to_string(),
                        ));
                    }
                    _ => {
                        return Err(ParseError::UnexpectedElement {
                            context: "document root".to_string(),
                            element: element_name(&e),
                            position: reader.position(),
                        });
                    }
                }
            }
            Event::Eof => {
                return Err(ParseError::MissingElement {
                    parent: "document".to_string(),
                    element: "score-partwise".to_string(),
                    position: reader.position(),
                });
            }
            _ => continue,
        }
    }
}

fn parse_score_partwise(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<ScorePartwise, ParseError> {
    let version = reader.get_optional_attr(start, "version");

    let mut work = None;
    let mut movement_number = None;
    let mut movement_title = None;
    let mut identification = None;
    let mut defaults = None;
    let mut credits = Vec::new();
    let mut part_list = None;
    let mut parts = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"work" => work = Some(parse_work(reader)?),
                    b"movement-number" => movement_number = Some(reader.read_text()?),
                    b"movement-title" => movement_title = Some(reader.read_text()?),
                    b"identification" => identification = Some(parse_identification(reader)?),
                    b"defaults" => defaults = Some(parse_defaults(reader)?),
                    b"credit" => credits.push(parse_credit(reader, &e)?),
                    b"part-list" => part_list = Some(parse_part_list(reader)?),
                    b"part" => parts.push(parse_part(reader, &e)?),
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"score-partwise" => break,
            Event::Eof => {
                return Err(ParseError::Other(
                    "Unexpected EOF in score-partwise".to_string()
                ));
            }
            _ => {}
        }
    }

    // part-list is required
    let part_list = part_list.ok_or_else(|| ParseError::MissingElement {
        parent: "score-partwise".to_string(),
        element: "part-list".to_string(),
        position: reader.position(),
    })?;

    Ok(ScorePartwise {
        version,
        work,
        movement_number,
        movement_title,
        identification,
        defaults,
        credits,
        part_list,
        parts,
    })
}

// ============================================================================
// Part List parsing
// ============================================================================

fn parse_part_list(reader: &mut XmlReader) -> Result<PartList, ParseError> {
    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"score-part" => {
                        content.push(PartListElement::ScorePart(parse_score_part(reader, &e)?));
                    }
                    b"part-group" => {
                        content.push(PartListElement::PartGroup(parse_part_group(reader, &e)?));
                    }
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"part-list" => break,
            _ => {}
        }
    }

    Ok(PartList { content })
}

fn parse_score_part(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<ScorePart, ParseError> {
    let id = reader.get_attr(start, "id")?;

    let mut part_name = None;
    let mut part_name_display = None;
    let mut part_abbreviation = None;
    let mut part_abbreviation_display = None;
    let mut groups = Vec::new();
    let mut score_instruments = Vec::new();
    let mut player = None;
    let mut midi_devices = Vec::new();
    let mut midi_instruments = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"part-name" => {
                        let value = reader.read_text()?;
                        part_name = Some(PartName {
                            value,
                            print_style: None,
                            print_object: None,
                            justify: None,
                        });
                    }
                    b"part-abbreviation" => {
                        let value = reader.read_text()?;
                        part_abbreviation = Some(PartAbbreviation {
                            value,
                            print_style: None,
                            print_object: None,
                            justify: None,
                        });
                    }
                    b"group" => groups.push(reader.read_text()?),
                    b"score-instrument" => {
                        score_instruments.push(parse_score_instrument(reader, &e)?);
                    }
                    b"midi-device" => {
                        midi_devices.push(parse_midi_device(reader, &e)?);
                    }
                    b"midi-instrument" => {
                        midi_instruments.push(parse_midi_instrument(reader, &e)?);
                    }
                    // TODO: part-name-display, part-abbreviation-display, player
                    _ => reader.skip_element()?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"score-part" => break,
            _ => {}
        }
    }

    let part_name = part_name.ok_or_else(|| ParseError::MissingElement {
        parent: "score-part".to_string(),
        element: "part-name".to_string(),
        position: reader.position(),
    })?;

    Ok(ScorePart {
        id,
        part_name,
        part_name_display,
        part_abbreviation,
        part_abbreviation_display,
        groups,
        score_instruments,
        player,
        midi_devices,
        midi_instruments,
    })
}

fn parse_part_group(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<PartGroup, ParseError> {
    let r#type = reader.get_attr(start, "type")?;
    let number = reader.get_optional_attr(start, "number").unwrap_or_else(|| "1".to_string());

    // TODO: parse group contents
    reader.skip_element()?;

    Ok(PartGroup {
        r#type: parse_start_stop(&r#type)?,
        number,
        group_name: None,
        group_name_display: None,
        group_abbreviation: None,
        group_abbreviation_display: None,
        group_symbol: None,
        group_barline: None,
        group_time: None,
        editorial: None,
    })
}

// ============================================================================
// Part and Measure parsing
// ============================================================================

fn parse_part(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Part, ParseError> {
    let id = reader.get_attr(start, "id")?;
    let mut measures = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) if e.name().as_ref() == b"measure" => {
                measures.push(parse_measure(reader, &e)?);
            }
            Event::End(e) if e.name().as_ref() == b"part" => break,
            _ => {}
        }
    }

    Ok(Part { id, measures })
}

fn parse_measure(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Measure, ParseError> {
    let number = reader.get_attr(start, "number")?;
    let text = reader.get_optional_attr(start, "text");
    let implicit = reader.get_optional_attr(start, "implicit")
        .and_then(|s| parse_yes_no(&s).ok());
    let non_controlling = reader.get_optional_attr(start, "non-controlling")
        .and_then(|s| parse_yes_no(&s).ok());
    let width = reader.get_optional_attr_as(start, "width");
    let id = reader.get_optional_attr(start, "id");

    let mut content = Vec::new();

    loop {
        match reader.next_event()? {
            Event::Start(e) => {
                let element = match e.name().as_ref() {
                    b"note" => Some(MusicDataElement::Note(parse_note(reader, &e)?)),
                    b"backup" => Some(MusicDataElement::Backup(parse_backup(reader)?)),
                    b"forward" => Some(MusicDataElement::Forward(parse_forward(reader)?)),
                    b"direction" => Some(MusicDataElement::Direction(parse_direction(reader, &e)?)),
                    b"attributes" => Some(MusicDataElement::Attributes(parse_attributes(reader)?)),
                    b"harmony" => Some(MusicDataElement::Harmony(parse_harmony(reader, &e)?)),
                    b"figured-bass" => Some(MusicDataElement::FiguredBass(parse_figured_bass(reader, &e)?)),
                    b"print" => Some(MusicDataElement::Print(parse_print(reader, &e)?)),
                    b"sound" => Some(MusicDataElement::Sound(parse_sound(reader, &e)?)),
                    b"listening" => {
                        reader.skip_element()?; // Deferred
                        None
                    }
                    b"barline" => Some(MusicDataElement::Barline(parse_barline(reader, &e)?)),
                    b"grouping" => {
                        reader.skip_element()?; // Deferred
                        None
                    }
                    b"link" => {
                        reader.skip_element()?; // Deferred
                        None
                    }
                    b"bookmark" => {
                        reader.skip_element()?; // Deferred
                        None
                    }
                    _ => {
                        reader.skip_element()?;
                        None
                    }
                };
                if let Some(el) = element {
                    content.push(el);
                }
            }
            Event::End(e) if e.name().as_ref() == b"measure" => break,
            _ => {}
        }
    }

    Ok(Measure {
        number,
        text,
        implicit,
        non_controlling,
        width,
        optional_unique_id: id,
        content,
    })
}

// ============================================================================
// Stub implementations - to be completed in Milestones 2-5
// ============================================================================

fn parse_work(reader: &mut XmlReader) -> Result<Work, ParseError> {
    // TODO: Milestone 5
    reader.skip_element()?;
    Ok(Work::default())
}

fn parse_identification(reader: &mut XmlReader) -> Result<Identification, ParseError> {
    // TODO: Milestone 5
    reader.skip_element()?;
    Ok(Identification::default())
}

fn parse_defaults(reader: &mut XmlReader) -> Result<Defaults, ParseError> {
    // Deferred - layout element
    reader.skip_element()?;
    Ok(Defaults::default())
}

fn parse_credit(
    reader: &mut XmlReader,
    _start: &quick_xml::events::BytesStart,
) -> Result<Credit, ParseError> {
    // TODO: Milestone 5
    reader.skip_element()?;
    Ok(Credit::default())
}

fn parse_score_instrument(
    reader: &mut XmlReader,
    _start: &quick_xml::events::BytesStart,
) -> Result<ScoreInstrument, ParseError> {
    reader.skip_element()?;
    Ok(ScoreInstrument::default())
}

fn parse_midi_device(
    reader: &mut XmlReader,
    _start: &quick_xml::events::BytesStart,
) -> Result<MidiDevice, ParseError> {
    reader.skip_element()?;
    Ok(MidiDevice::default())
}

fn parse_midi_instrument(
    reader: &mut XmlReader,
    _start: &quick_xml::events::BytesStart,
) -> Result<MidiInstrument, ParseError> {
    reader.skip_element()?;
    Ok(MidiInstrument::default())
}

// Music data stubs - Milestone 2
fn parse_note(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Note, ParseError> {
    todo!("Implement in Milestone 2, Task 2.2")
}

fn parse_attributes(reader: &mut XmlReader) -> Result<Attributes, ParseError> {
    todo!("Implement in Milestone 2, Task 2.1")
}

// Voice/navigation stubs - Milestone 3
fn parse_backup(reader: &mut XmlReader) -> Result<Backup, ParseError> {
    todo!("Implement in Milestone 3, Task 3.1")
}

fn parse_forward(reader: &mut XmlReader) -> Result<Forward, ParseError> {
    todo!("Implement in Milestone 3, Task 3.1")
}

fn parse_barline(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Barline, ParseError> {
    todo!("Implement in Milestone 3, Task 3.2")
}

// Direction stubs - Milestone 4
fn parse_direction(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Direction, ParseError> {
    todo!("Implement in Milestone 4, Task 4.1")
}

// Extended stubs - Milestone 5
fn parse_harmony(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Harmony, ParseError> {
    reader.skip_element()?;
    Ok(Harmony::default())
}

fn parse_figured_bass(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<FiguredBass, ParseError> {
    reader.skip_element()?;
    Ok(FiguredBass::default())
}

fn parse_print(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Print, ParseError> {
    reader.skip_element()?;
    Ok(Print::default())
}

fn parse_sound(
    reader: &mut XmlReader,
    start: &quick_xml::events::BytesStart,
) -> Result<Sound, ParseError> {
    reader.skip_element()?;
    Ok(Sound::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_score() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1">
      <part-name>Music</part-name>
    </score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
    </measure>
  </part>
</score-partwise>"#;

        let score = parse_score(xml).expect("should parse minimal score");

        assert_eq!(score.version, Some("4.0".to_string()));
        assert_eq!(score.part_list.content.len(), 1);
        assert_eq!(score.parts.len(), 1);
        assert_eq!(score.parts[0].measures.len(), 1);
        assert_eq!(score.parts[0].measures[0].number, "1");
    }

    #[test]
    fn test_reject_score_timewise() {
        let xml = r#"<?xml version="1.0"?>
<score-timewise version="4.0">
</score-timewise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("timewise"));
    }

    #[test]
    fn test_missing_part_list() {
        let xml = r#"<?xml version="1.0"?>
<score-partwise version="4.0">
  <part id="P1">
    <measure number="1"/>
  </part>
</score-partwise>"#;

        let result = parse_score(xml);
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::MissingElement { element, .. } => {
                assert_eq!(element, "part-list");
            }
            e => panic!("Expected MissingElement, got {:?}", e),
        }
    }
}
```

**Acceptance Criteria:**

- [ ] All unit tests pass
- [ ] Can parse minimal valid MusicXML (part-list + empty measure)
- [ ] Rejects score-timewise with helpful message
- [ ] Returns MissingElement error for missing part-list
- [ ] Skips unknown elements gracefully

---

## Milestone 1 Summary

After completing Milestone 1, you have:

1. **Module structure** with ParseError type
2. **XmlReader** helper with tested convenience methods
3. **Value parsers** for all IR enums
4. **Parsing skeleton** that handles document structure

The parser can now consume minimal valid MusicXML files. The `todo!()` markers indicate where Milestones 2-5 will fill in the actual parsing logic.

---

*Next document: Milestone 2 — Core Note & Attributes*
