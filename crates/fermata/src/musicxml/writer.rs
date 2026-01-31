//! XML writer helpers for MusicXML emission.
//!
//! This module provides a wrapper around `quick_xml::Writer` with helper methods
//! for common XML writing patterns used in MusicXML emission.

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::io::Cursor;

/// A wrapper around `quick_xml::Writer` with helper methods for MusicXML emission.
pub struct XmlWriter {
    writer: Writer<Cursor<Vec<u8>>>,
}

#[allow(dead_code)]
impl XmlWriter {
    /// Create a new XmlWriter with 2-space indentation.
    pub fn new() -> Self {
        let writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        Self { writer }
    }

    /// Write XML declaration and DOCTYPE for MusicXML 4.0 partwise.
    ///
    /// Writes:
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
    /// ```
    pub fn write_header(&mut self) -> Result<(), std::io::Error> {
        use std::io::Write;

        // <?xml version="1.0" encoding="UTF-8"?>
        self.writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

        // DOCTYPE for MusicXML 4.0 partwise
        // Use Write trait to properly update cursor position
        self.writer.get_mut().write_all(
            b"\n<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 4.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\">\n"
        )?;
        Ok(())
    }

    /// Start an element with no attributes.
    pub fn start_element(&mut self, name: &str) -> Result<(), std::io::Error> {
        self.writer.write_event(Event::Start(BytesStart::new(name)))
    }

    /// Write a start tag with attributes from an `ElementBuilder`.
    pub fn write_start(&mut self, builder: ElementBuilder) -> Result<(), std::io::Error> {
        self.writer
            .write_event(Event::Start(builder.into_bytes_start()))
    }

    /// End the current element.
    pub fn end_element(&mut self, name: &str) -> Result<(), std::io::Error> {
        self.writer.write_event(Event::End(BytesEnd::new(name)))
    }

    /// Write an empty element `<name/>`.
    pub fn empty_element(&mut self, name: &str) -> Result<(), std::io::Error> {
        self.writer.write_event(Event::Empty(BytesStart::new(name)))
    }

    /// Write an empty element with attributes.
    pub fn empty_element_with_attrs(
        &mut self,
        builder: ElementBuilder,
    ) -> Result<(), std::io::Error> {
        self.writer
            .write_event(Event::Empty(builder.into_bytes_start()))
    }

    /// Write a simple element: `<name>text</name>`.
    pub fn text_element(&mut self, name: &str, text: &str) -> Result<(), std::io::Error> {
        self.start_element(name)?;
        self.writer.write_event(Event::Text(BytesText::new(text)))?;
        self.end_element(name)
    }

    /// Write element only if value is `Some`.
    pub fn optional_text_element<T: std::fmt::Display>(
        &mut self,
        name: &str,
        value: &Option<T>,
    ) -> Result<(), std::io::Error> {
        if let Some(v) = value {
            self.text_element(name, &v.to_string())?;
        }
        Ok(())
    }

    /// Write raw text content (for use inside open elements).
    pub fn write_text(&mut self, text: &str) -> Result<(), std::io::Error> {
        self.writer.write_event(Event::Text(BytesText::new(text)))
    }

    /// Consume the writer and return the XML string.
    pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.writer.into_inner().into_inner())
    }
}

impl Default for XmlWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for elements with attributes.
pub struct ElementBuilder {
    start: BytesStart<'static>,
}

#[allow(dead_code)]
impl ElementBuilder {
    /// Create a new element builder with the given element name.
    pub fn new(name: &str) -> Self {
        Self {
            start: BytesStart::new(name.to_string()),
        }
    }

    /// Add an attribute to the element.
    pub fn attr(mut self, key: &str, value: &str) -> Self {
        self.start.push_attribute((key, value));
        self
    }

    /// Add an optional attribute to the element.
    ///
    /// If the value is `None`, no attribute is added.
    pub fn optional_attr<T: std::fmt::Display>(self, key: &str, value: &Option<T>) -> Self {
        match value {
            Some(v) => self.attr(key, &v.to_string()),
            None => self,
        }
    }

    /// Convert the builder into a `BytesStart` for use with quick-xml.
    pub fn into_bytes_start(self) -> BytesStart<'static> {
        self.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmlwriter_new() {
        let writer = XmlWriter::new();
        let result = writer.into_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_xmlwriter_default() {
        let writer = XmlWriter::default();
        let result = writer.into_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_xmlwriter_write_header() {
        let mut writer = XmlWriter::new();
        writer.write_header().unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(result.contains("<!DOCTYPE score-partwise"));
        assert!(result.contains("MusicXML 4.0"));
    }

    #[test]
    fn test_xmlwriter_start_end_element() {
        let mut writer = XmlWriter::new();
        writer.start_element("note").unwrap();
        writer.end_element("note").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<note>"));
        assert!(result.contains("</note>"));
    }

    #[test]
    fn test_xmlwriter_empty_element() {
        let mut writer = XmlWriter::new();
        writer.empty_element("rest").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<rest/>"));
    }

    #[test]
    fn test_xmlwriter_text_element() {
        let mut writer = XmlWriter::new();
        writer.text_element("step", "C").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<step>C</step>"));
    }

    #[test]
    fn test_xmlwriter_optional_text_element_some() {
        let mut writer = XmlWriter::new();
        writer.optional_text_element("octave", &Some(4)).unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<octave>4</octave>"));
    }

    #[test]
    fn test_xmlwriter_optional_text_element_none() {
        let mut writer = XmlWriter::new();
        writer
            .optional_text_element::<i32>("octave", &None)
            .unwrap();
        let result = writer.into_string().unwrap();

        assert!(!result.contains("<octave>"));
    }

    #[test]
    fn test_xmlwriter_write_text() {
        let mut writer = XmlWriter::new();
        writer.start_element("words").unwrap();
        writer.write_text("Hello").unwrap();
        writer.end_element("words").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<words>Hello</words>"));
    }

    #[test]
    fn test_xmlwriter_write_start_with_attrs() {
        let mut writer = XmlWriter::new();
        let builder = ElementBuilder::new("note").attr("default-x", "10");
        writer.write_start(builder).unwrap();
        writer.end_element("note").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<note default-x=\"10\">"));
    }

    #[test]
    fn test_xmlwriter_empty_element_with_attrs() {
        let mut writer = XmlWriter::new();
        let builder = ElementBuilder::new("tie").attr("type", "start");
        writer.empty_element_with_attrs(builder).unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<tie type=\"start\"/>"));
    }

    #[test]
    fn test_elementbuilder_new() {
        let builder = ElementBuilder::new("note");
        let start = builder.into_bytes_start();
        assert_eq!(start.name().as_ref(), b"note");
    }

    #[test]
    fn test_elementbuilder_attr() {
        let builder = ElementBuilder::new("note").attr("id", "n1");
        let start = builder.into_bytes_start();
        assert_eq!(start.name().as_ref(), b"note");
        // Attributes are encoded in the start tag
    }

    #[test]
    fn test_elementbuilder_multiple_attrs() {
        let builder = ElementBuilder::new("note")
            .attr("default-x", "10")
            .attr("default-y", "20");
        let start = builder.into_bytes_start();
        assert_eq!(start.name().as_ref(), b"note");
    }

    #[test]
    fn test_elementbuilder_optional_attr_some() {
        let mut writer = XmlWriter::new();
        let builder = ElementBuilder::new("clef").optional_attr("number", &Some(1));
        writer.write_start(builder).unwrap();
        writer.end_element("clef").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<clef number=\"1\">"));
    }

    #[test]
    fn test_elementbuilder_optional_attr_none() {
        let mut writer = XmlWriter::new();
        let builder = ElementBuilder::new("clef").optional_attr::<i32>("number", &None);
        writer.write_start(builder).unwrap();
        writer.end_element("clef").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<clef>"));
        assert!(!result.contains("number"));
    }

    #[test]
    fn test_nested_elements() {
        let mut writer = XmlWriter::new();
        writer.start_element("pitch").unwrap();
        writer.text_element("step", "C").unwrap();
        writer.text_element("octave", "4").unwrap();
        writer.end_element("pitch").unwrap();
        let result = writer.into_string().unwrap();

        assert!(result.contains("<pitch>"));
        assert!(result.contains("<step>C</step>"));
        assert!(result.contains("<octave>4</octave>"));
        assert!(result.contains("</pitch>"));
    }

    #[test]
    fn test_text_escaping() {
        let mut writer = XmlWriter::new();
        writer.text_element("words", "A < B & C > D").unwrap();
        let result = writer.into_string().unwrap();

        // quick-xml should escape special characters
        assert!(result.contains("&lt;"));
        assert!(result.contains("&amp;"));
        assert!(result.contains("&gt;"));
    }
}
