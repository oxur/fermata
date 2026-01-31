//! XML reader helper for parsing MusicXML.
//!
//! This module provides a wrapper around quick-xml's Reader that adds
//! convenience methods for common parsing operations like reading text,
//! getting attributes, and skipping elements.
//!
//! Note: Some methods like `peek_event` and `get_attr_as` are not used yet
//! but will be used in Phase 3 Milestones 2-5.
#![allow(dead_code)]

use quick_xml::Reader;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::{BytesStart, Event};
use std::str::FromStr;

use super::ParseError;

/// A wrapper around quick-xml's Reader with helper methods for parsing MusicXML.
///
/// This wrapper maintains a peek buffer for look-ahead parsing and provides
/// convenient methods for common operations like:
/// - Reading element text content
/// - Getting required and optional attributes
/// - Skipping elements and their children
/// - Position tracking for error reporting
pub(crate) struct XmlReader<'a> {
    /// The underlying quick-xml reader
    reader: Reader<&'a [u8]>,
    /// Buffer for storing the peeked event (if any)
    peeked: Option<Event<'static>>,
    /// Buffer for reading events
    buf: Vec<u8>,
}

impl<'a> XmlReader<'a> {
    /// Create a new XmlReader from an XML string.
    ///
    /// # Arguments
    ///
    /// * `xml` - The XML string to parse
    ///
    /// # Returns
    ///
    /// A new XmlReader ready for parsing
    pub fn new(xml: &'a str) -> Self {
        let mut reader = Reader::from_reader(xml.as_bytes());
        reader.config_mut().trim_text(true);
        XmlReader {
            reader,
            peeked: None,
            buf: Vec::new(),
        }
    }

    /// Get the current byte position in the input.
    ///
    /// This is useful for error reporting to indicate where in the
    /// input an error occurred.
    #[inline]
    pub fn position(&self) -> usize {
        self.reader.buffer_position() as usize
    }

    /// Get the next event from the XML stream.
    ///
    /// If an event was previously peeked, it is returned and consumed.
    /// Otherwise, reads the next event from the underlying reader.
    ///
    /// # Returns
    ///
    /// The next XML event, or an error if parsing fails.
    pub fn next_event(&mut self) -> Result<Event<'static>, ParseError> {
        if let Some(event) = self.peeked.take() {
            return Ok(event);
        }
        self.buf.clear();
        let event = self.reader.read_event_into(&mut self.buf)?;
        Ok(event.into_owned())
    }

    /// Peek at the next event without consuming it.
    ///
    /// Subsequent calls to peek_event() will return the same event.
    /// The event is consumed when next_event() is called.
    ///
    /// # Returns
    ///
    /// A reference to the next XML event, or an error if parsing fails.
    pub fn peek_event(&mut self) -> Result<&Event<'static>, ParseError> {
        if self.peeked.is_none() {
            self.buf.clear();
            let event = self.reader.read_event_into(&mut self.buf)?;
            self.peeked = Some(event.into_owned());
        }
        Ok(self.peeked.as_ref().unwrap())
    }

    /// Read the text content of the current element.
    ///
    /// This should be called after reading a Start event. It reads text
    /// until the matching End event is encountered.
    ///
    /// # Arguments
    ///
    /// * `element_name` - The name of the element (for error messages)
    ///
    /// # Returns
    ///
    /// The text content as a String, or an error.
    pub fn read_text(&mut self, element_name: &str) -> Result<String, ParseError> {
        let mut content = String::new();
        loop {
            let event = self.next_event()?;
            match event {
                Event::Text(e) => {
                    let text = String::from_utf8_lossy(e.as_ref()).to_string();
                    content.push_str(&text);
                }
                Event::End(_) => break,
                Event::Eof => {
                    return Err(ParseError::xml(
                        format!("unexpected EOF while reading <{}>", element_name),
                        self.position(),
                    ));
                }
                _ => {
                    // Skip nested elements (shouldn't happen for text-only elements)
                }
            }
        }
        Ok(content)
    }

    /// Read the text content of the current element and parse it as type T.
    ///
    /// # Arguments
    ///
    /// * `element_name` - The name of the element (for error messages)
    ///
    /// # Returns
    ///
    /// The parsed value, or an error if parsing fails.
    pub fn read_text_as<T>(&mut self, element_name: &str) -> Result<T, ParseError>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let text = self.read_text(element_name)?;
        text.parse::<T>().map_err(|e| {
            ParseError::invalid_value(
                std::any::type_name::<T>(),
                format!("{} ({})", text, e),
                self.position(),
            )
        })
    }

    /// Skip the current element and all its children.
    ///
    /// This should be called after reading a Start event. It reads
    /// and discards all events until the matching End event.
    ///
    /// # Arguments
    ///
    /// * `element_name` - The name of the element to skip (for validation)
    ///
    /// # Returns
    ///
    /// Ok(()) on success, or an error if parsing fails.
    pub fn skip_element(&mut self, element_name: &str) -> Result<(), ParseError> {
        let mut depth = 1usize;
        loop {
            let event = self.next_event()?;
            match event {
                Event::Start(_) => depth += 1,
                Event::End(_) => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Event::Empty(_) => {
                    // Empty elements don't affect depth
                }
                Event::Eof => {
                    return Err(ParseError::xml(
                        format!("unexpected EOF while skipping <{}>", element_name),
                        self.position(),
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Get a required attribute value from a start tag.
    ///
    /// # Arguments
    ///
    /// * `attrs` - The attributes from the start tag
    /// * `name` - The name of the attribute to find
    /// * `element` - The element name (for error messages)
    ///
    /// # Returns
    ///
    /// The attribute value as a String, or an error if not found.
    pub fn get_attr(
        &self,
        attrs: Attributes<'_>,
        name: &str,
        element: &str,
    ) -> Result<String, ParseError> {
        for attr in attrs {
            let attr = attr.map_err(|e| {
                ParseError::xml(format!("invalid attribute: {}", e), self.position())
            })?;
            if attr.key.as_ref() == name.as_bytes() {
                return decode_attr_value(&attr.value, self.position());
            }
        }
        Err(ParseError::missing_attribute(
            name,
            element,
            self.position(),
        ))
    }

    /// Get an optional attribute value from a start tag.
    ///
    /// # Arguments
    ///
    /// * `attrs` - The attributes from the start tag
    /// * `name` - The name of the attribute to find
    ///
    /// # Returns
    ///
    /// Some(value) if the attribute exists, None otherwise.
    pub fn get_optional_attr(
        &self,
        attrs: Attributes<'_>,
        name: &str,
    ) -> Result<Option<String>, ParseError> {
        for attr in attrs {
            let attr = attr.map_err(|e| {
                ParseError::xml(format!("invalid attribute: {}", e), self.position())
            })?;
            if attr.key.as_ref() == name.as_bytes() {
                return Ok(Some(decode_attr_value(&attr.value, self.position())?));
            }
        }
        Ok(None)
    }

    /// Get an optional attribute value and parse it as type T.
    ///
    /// # Arguments
    ///
    /// * `attrs` - The attributes from the start tag
    /// * `name` - The name of the attribute to find
    ///
    /// # Returns
    ///
    /// Some(parsed_value) if the attribute exists, None otherwise.
    pub fn get_optional_attr_as<T>(
        &self,
        attrs: Attributes<'_>,
        name: &str,
    ) -> Result<Option<T>, ParseError>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        match self.get_optional_attr(attrs, name)? {
            Some(value) => {
                let parsed = value.parse::<T>().map_err(|e| {
                    ParseError::invalid_value(
                        std::any::type_name::<T>(),
                        format!("{} ({})", value, e),
                        self.position(),
                    )
                })?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    /// Get a required attribute value and parse it as type T.
    ///
    /// # Arguments
    ///
    /// * `attrs` - The attributes from the start tag
    /// * `name` - The name of the attribute to find
    /// * `element` - The element name (for error messages)
    ///
    /// # Returns
    ///
    /// The parsed attribute value, or an error if not found or invalid.
    pub fn get_attr_as<T>(
        &self,
        attrs: Attributes<'_>,
        name: &str,
        element: &str,
    ) -> Result<T, ParseError>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let value = self.get_attr(attrs, name, element)?;
        value.parse::<T>().map_err(|e| {
            ParseError::invalid_value(
                std::any::type_name::<T>(),
                format!("{} ({})", value, e),
                self.position(),
            )
        })
    }
}

/// Decode an attribute value, unescaping XML entities.
fn decode_attr_value(value: &[u8], position: usize) -> Result<String, ParseError> {
    // Convert bytes to string
    let s = std::str::from_utf8(value)
        .map_err(|e| ParseError::xml(format!("invalid UTF-8 in attribute: {}", e), position))?;

    // Unescape common XML entities
    let unescaped = unescape_xml(s);
    Ok(unescaped)
}

/// Unescape common XML entities in a string.
fn unescape_xml(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// Extract the local name from a potentially prefixed element name.
///
/// # Arguments
///
/// * `name` - The element name (may include namespace prefix like "xml:foo")
///
/// # Returns
///
/// The local name without any namespace prefix.
pub(crate) fn local_name(name: &[u8]) -> &[u8] {
    match name.iter().position(|&b| b == b':') {
        Some(pos) => &name[pos + 1..],
        None => name,
    }
}

/// Get the element name from a BytesStart as a string.
///
/// # Arguments
///
/// * `tag` - The start tag to extract the name from
///
/// # Returns
///
/// The element name as a String.
pub(crate) fn element_name(tag: &BytesStart<'_>) -> String {
    String::from_utf8_lossy(local_name(tag.name().as_ref())).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::Event;

    // === XmlReader basic tests ===

    #[test]
    fn test_xml_reader_new() {
        let xml = "<root></root>";
        let reader = XmlReader::new(xml);
        assert_eq!(reader.position(), 0);
    }

    #[test]
    fn test_xml_reader_position() {
        let xml = "<root>content</root>";
        let mut reader = XmlReader::new(xml);

        // Read first event
        let _ = reader.next_event();
        // Position should have advanced
        assert!(reader.position() > 0);
    }

    #[test]
    fn test_xml_reader_next_event_start() {
        let xml = "<root></root>";
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        match event {
            Event::Start(e) => {
                assert_eq!(element_name(&e), "root");
            }
            _ => panic!("Expected Start event"),
        }
    }

    #[test]
    fn test_xml_reader_next_event_end() {
        let xml = "<root></root>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event(); // Start
        let event = reader.next_event().unwrap();
        match event {
            Event::End(_) => {}
            _ => panic!("Expected End event"),
        }
    }

    #[test]
    fn test_xml_reader_next_event_empty() {
        let xml = "<empty/>";
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        match event {
            Event::Empty(e) => {
                assert_eq!(element_name(&e), "empty");
            }
            _ => panic!("Expected Empty event"),
        }
    }

    #[test]
    fn test_xml_reader_next_event_eof() {
        let xml = "";
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        assert!(matches!(event, Event::Eof));
    }

    // === Peek tests ===

    #[test]
    fn test_xml_reader_peek_event() {
        let xml = "<root></root>";
        let mut reader = XmlReader::new(xml);

        let peeked = reader.peek_event().unwrap();
        match peeked {
            Event::Start(_) => {}
            _ => panic!("Expected Start event"),
        }

        // Peeking again should return the same event
        let peeked2 = reader.peek_event().unwrap();
        match peeked2 {
            Event::Start(_) => {}
            _ => panic!("Expected Start event"),
        }
    }

    #[test]
    fn test_xml_reader_peek_then_next() {
        let xml = "<root></root>";
        let mut reader = XmlReader::new(xml);

        // Peek first
        let _ = reader.peek_event();

        // Next should return the peeked event
        let event = reader.next_event().unwrap();
        match event {
            Event::Start(e) => {
                assert_eq!(element_name(&e), "root");
            }
            _ => panic!("Expected Start event"),
        }

        // Next should now return End
        let event = reader.next_event().unwrap();
        match event {
            Event::End(_) => {}
            _ => panic!("Expected End event"),
        }
    }

    // === read_text tests ===

    #[test]
    fn test_xml_reader_read_text_simple() {
        let xml = "<elem>Hello World</elem>";
        let mut reader = XmlReader::new(xml);

        // Read start tag
        let _ = reader.next_event();

        // Read text
        let text = reader.read_text("elem").unwrap();
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_xml_reader_read_text_empty() {
        let xml = "<elem></elem>";
        let mut reader = XmlReader::new(xml);

        // Read start tag
        let _ = reader.next_event();

        // Read text (empty)
        let text = reader.read_text("elem").unwrap();
        assert_eq!(text, "");
    }

    #[test]
    fn test_xml_reader_read_text_with_whitespace() {
        let xml = "<elem>  trimmed  </elem>";
        let mut reader = XmlReader::new(xml);

        // Read start tag
        let _ = reader.next_event();

        // Read text - note: quick-xml trims text when trim_text is enabled
        let text = reader.read_text("elem").unwrap();
        assert_eq!(text, "trimmed");
    }

    // === read_text_as tests ===

    #[test]
    fn test_xml_reader_read_text_as_i32() {
        let xml = "<num>42</num>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event();
        let value: i32 = reader.read_text_as("num").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_xml_reader_read_text_as_f64() {
        let xml = "<num>3.14</num>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event();
        let value: f64 = reader.read_text_as("num").unwrap();
        assert!((value - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn test_xml_reader_read_text_as_invalid() {
        let xml = "<num>not_a_number</num>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event();
        let result: Result<i32, _> = reader.read_text_as("num");
        assert!(result.is_err());
    }

    // === skip_element tests ===

    #[test]
    fn test_xml_reader_skip_element_empty() {
        let xml = "<root><skip></skip><next/></root>";
        let mut reader = XmlReader::new(xml);

        // Read <root>
        let _ = reader.next_event();
        // Read <skip>
        let _ = reader.next_event();
        // Skip it
        reader.skip_element("skip").unwrap();

        // Next should be <next/>
        let event = reader.next_event().unwrap();
        match event {
            Event::Empty(e) => {
                assert_eq!(element_name(&e), "next");
            }
            _ => panic!("Expected Empty event for <next/>"),
        }
    }

    #[test]
    fn test_xml_reader_skip_element_with_children() {
        let xml = "<root><skip><a><b/></a><c/></skip><next/></root>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event(); // <root>
        let _ = reader.next_event(); // <skip>
        reader.skip_element("skip").unwrap();

        let event = reader.next_event().unwrap();
        match event {
            Event::Empty(e) => {
                assert_eq!(element_name(&e), "next");
            }
            _ => panic!("Expected Empty event for <next/>"),
        }
    }

    #[test]
    fn test_xml_reader_skip_element_with_text() {
        let xml = "<root><skip>some text</skip><next/></root>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event(); // <root>
        let _ = reader.next_event(); // <skip>
        reader.skip_element("skip").unwrap();

        let event = reader.next_event().unwrap();
        match event {
            Event::Empty(e) => {
                assert_eq!(element_name(&e), "next");
            }
            _ => panic!("Expected Empty event for <next/>"),
        }
    }

    // === Attribute tests ===

    #[test]
    fn test_xml_reader_get_attr() {
        let xml = r#"<elem id="test" value="123"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let id = reader.get_attr(attrs, "id", "elem").unwrap();
            assert_eq!(id, "test");
        } else {
            panic!("Expected Empty event");
        }
    }

    #[test]
    fn test_xml_reader_get_attr_missing() {
        let xml = r#"<elem id="test"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let result = reader.get_attr(attrs, "missing", "elem");
            assert!(result.is_err());
            if let Err(ParseError::MissingAttribute {
                attribute, element, ..
            }) = result
            {
                assert_eq!(attribute, "missing");
                assert_eq!(element, "elem");
            } else {
                panic!("Expected MissingAttribute error");
            }
        }
    }

    #[test]
    fn test_xml_reader_get_optional_attr_present() {
        let xml = r#"<elem optional="value"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let opt = reader.get_optional_attr(attrs, "optional").unwrap();
            assert_eq!(opt, Some("value".to_string()));
        }
    }

    #[test]
    fn test_xml_reader_get_optional_attr_missing() {
        let xml = r#"<elem id="test"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let opt = reader.get_optional_attr(attrs, "optional").unwrap();
            assert!(opt.is_none());
        }
    }

    #[test]
    fn test_xml_reader_get_optional_attr_as_i32() {
        let xml = r#"<elem num="42"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let opt: Option<i32> = reader.get_optional_attr_as(attrs, "num").unwrap();
            assert_eq!(opt, Some(42));
        }
    }

    #[test]
    fn test_xml_reader_get_optional_attr_as_missing() {
        let xml = r#"<elem id="test"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let opt: Option<i32> = reader.get_optional_attr_as(attrs, "num").unwrap();
            assert!(opt.is_none());
        }
    }

    #[test]
    fn test_xml_reader_get_attr_as_i32() {
        let xml = r#"<elem num="99"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let num: i32 = reader.get_attr_as(attrs, "num", "elem").unwrap();
            assert_eq!(num, 99);
        }
    }

    #[test]
    fn test_xml_reader_get_attr_as_f64() {
        let xml = r#"<elem val="1.5"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let val: f64 = reader.get_attr_as(attrs, "val", "elem").unwrap();
            assert!((val - 1.5).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_xml_reader_get_attr_as_invalid() {
        let xml = r#"<elem num="not_a_number"/>"#;
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            let attrs = e.attributes();
            let result: Result<i32, _> = reader.get_attr_as(attrs, "num", "elem");
            assert!(result.is_err());
        }
    }

    // === XML entity unescaping tests ===

    #[test]
    fn test_unescape_xml_amp() {
        assert_eq!(unescape_xml("a &amp; b"), "a & b");
    }

    #[test]
    fn test_unescape_xml_lt() {
        assert_eq!(unescape_xml("a &lt; b"), "a < b");
    }

    #[test]
    fn test_unescape_xml_gt() {
        assert_eq!(unescape_xml("a &gt; b"), "a > b");
    }

    #[test]
    fn test_unescape_xml_quot() {
        assert_eq!(unescape_xml("&quot;hello&quot;"), "\"hello\"");
    }

    #[test]
    fn test_unescape_xml_apos() {
        assert_eq!(unescape_xml("&apos;hello&apos;"), "'hello'");
    }

    #[test]
    fn test_unescape_xml_multiple() {
        assert_eq!(unescape_xml("&lt;a&gt; &amp; &lt;b&gt;"), "<a> & <b>");
    }

    #[test]
    fn test_unescape_xml_no_entities() {
        assert_eq!(unescape_xml("plain text"), "plain text");
    }

    // === local_name tests ===

    #[test]
    fn test_local_name_no_prefix() {
        assert_eq!(local_name(b"element"), b"element");
    }

    #[test]
    fn test_local_name_with_prefix() {
        assert_eq!(local_name(b"xml:lang"), b"lang");
    }

    #[test]
    fn test_local_name_with_namespace() {
        assert_eq!(local_name(b"xlink:href"), b"href");
    }

    // === element_name tests ===

    #[test]
    fn test_element_name_simple() {
        let xml = "<test/>";
        let mut reader = XmlReader::new(xml);

        let event = reader.next_event().unwrap();
        if let Event::Empty(e) = event {
            assert_eq!(element_name(&e), "test");
        }
    }

    // === Error case tests ===

    #[test]
    fn test_xml_reader_malformed_xml() {
        let xml = "<unclosed";
        let mut reader = XmlReader::new(xml);

        // Attempt to read past the malformed content
        let result = reader.next_event();
        // quick-xml may error immediately on malformed XML, or return an event
        // Either behavior is acceptable - we just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_xml_reader_read_text_eof() {
        let xml = "<elem>unterminated";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event(); // <elem>
        let result = reader.read_text("elem");
        // Should error due to EOF
        assert!(result.is_err());
    }

    #[test]
    fn test_xml_reader_skip_element_eof() {
        let xml = "<root><skip>";
        let mut reader = XmlReader::new(xml);

        let _ = reader.next_event(); // <root>
        let _ = reader.next_event(); // <skip>
        let result = reader.skip_element("skip");
        // Should error due to EOF
        assert!(result.is_err());
    }
}
