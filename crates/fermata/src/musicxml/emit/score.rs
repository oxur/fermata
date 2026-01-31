//! Score-level emission functions for MusicXML.
//!
//! This module contains the main entry point `emit_score` and functions for
//! emitting score headers, part lists, parts, and measures.

use crate::ir::common::{Encoding, EncodingContent, Identification, Supports, TypedText};
use crate::ir::part::{PartGroup, PartList, PartListElement, ScorePart};
use crate::ir::score::{
    Appearance, Credit, CreditContent, CreditImage, CreditSymbol, CreditWords, Defaults, Distance,
    LineWidth, LyricFont, LyricLanguage, NoteSize, Opus, PageLayout, PageMargins, Scaling,
    StaffLayout, SystemDividers, SystemLayout, SystemMargins, Work,
};
use crate::ir::{Measure, MusicDataElement, Part, ScorePartwise};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::attributes::emit_attributes;
use super::barline::emit_barline;
use super::direction::emit_direction;
use super::helpers::{
    font_size_to_string, left_center_right_to_string, margin_type_to_string,
    note_size_type_to_string, top_middle_bottom_to_string, yes_no_to_string,
};
use super::note::emit_note;
use super::voice::{emit_backup, emit_forward};

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
pub(crate) fn emit_score_header(w: &mut XmlWriter, score: &ScorePartwise) -> Result<(), EmitError> {
    // work
    if let Some(ref work) = score.work {
        emit_work(w, work)?;
    }

    // movement-number
    if let Some(ref mn) = score.movement_number {
        w.text_element("movement-number", mn)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // movement-title
    if let Some(ref mt) = score.movement_title {
        w.text_element("movement-title", mt)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // identification
    if let Some(ref id) = score.identification {
        emit_identification(w, id)?;
    }

    // defaults
    if let Some(ref defaults) = score.defaults {
        emit_defaults(w, defaults)?;
    }

    // credits
    for credit in &score.credits {
        emit_credit(w, credit)?;
    }

    emit_part_list(w, &score.part_list)?;
    Ok(())
}

/// Emit the work element.
pub(crate) fn emit_work(w: &mut XmlWriter, work: &Work) -> Result<(), EmitError> {
    let has_content =
        work.work_number.is_some() || work.work_title.is_some() || work.opus.is_some();

    if !has_content {
        return Ok(());
    }

    w.start_element("work")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if let Some(ref wn) = work.work_number {
        w.text_element("work-number", wn)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    if let Some(ref wt) = work.work_title {
        w.text_element("work-title", wt)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    if let Some(ref opus) = work.opus {
        emit_opus(w, opus)?;
    }

    w.end_element("work")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the opus element.
fn emit_opus(w: &mut XmlWriter, opus: &Opus) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("opus").attr("xlink:href", &opus.href);
    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the identification element.
pub(crate) fn emit_identification(w: &mut XmlWriter, id: &Identification) -> Result<(), EmitError> {
    w.start_element("identification")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // creators
    for creator in &id.creators {
        emit_typed_text(w, "creator", creator)?;
    }

    // rights
    for right in &id.rights {
        emit_typed_text(w, "rights", right)?;
    }

    // encoding
    if let Some(ref encoding) = id.encoding {
        emit_encoding(w, encoding)?;
    }

    // source
    if let Some(ref source) = id.source {
        w.text_element("source", source)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // relations
    for relation in &id.relations {
        emit_typed_text(w, "relation", relation)?;
    }

    // miscellaneous - not implemented yet (rarely used)

    w.end_element("identification")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a typed text element (creator, rights, relation).
fn emit_typed_text(w: &mut XmlWriter, name: &str, text: &TypedText) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref t) = text.r#type {
        elem = elem.attr("type", t);
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&text.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element(name)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the encoding element.
fn emit_encoding(w: &mut XmlWriter, encoding: &Encoding) -> Result<(), EmitError> {
    w.start_element("encoding")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for content in &encoding.content {
        match content {
            EncodingContent::EncodingDate(date) => {
                w.text_element("encoding-date", date)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            EncodingContent::Encoder(encoder) => {
                emit_typed_text(w, "encoder", encoder)?;
            }
            EncodingContent::Software(software) => {
                w.text_element("software", software)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            EncodingContent::EncodingDescription(desc) => {
                w.text_element("encoding-description", desc)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            EncodingContent::Supports(supports) => {
                emit_supports(w, supports)?;
            }
        }
    }

    w.end_element("encoding")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the supports element.
fn emit_supports(w: &mut XmlWriter, supports: &Supports) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("supports")
        .attr("type", yes_no_to_string(&supports.r#type))
        .attr("element", &supports.element);

    if let Some(ref attr) = supports.attribute {
        elem = elem.attr("attribute", attr);
    }
    if let Some(ref val) = supports.value {
        elem = elem.attr("value", val);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the defaults element.
pub(crate) fn emit_defaults(w: &mut XmlWriter, defaults: &Defaults) -> Result<(), EmitError> {
    w.start_element("defaults")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // scaling
    if let Some(ref scaling) = defaults.scaling {
        emit_scaling(w, scaling)?;
    }

    // page-layout
    if let Some(ref page_layout) = defaults.page_layout {
        emit_page_layout(w, page_layout)?;
    }

    // system-layout
    if let Some(ref system_layout) = defaults.system_layout {
        emit_system_layout(w, system_layout)?;
    }

    // staff-layout
    for staff_layout in &defaults.staff_layout {
        emit_staff_layout(w, staff_layout)?;
    }

    // appearance
    if let Some(ref appearance) = defaults.appearance {
        emit_appearance(w, appearance)?;
    }

    // music-font
    if let Some(ref font) = defaults.music_font {
        emit_font_element(w, "music-font", font)?;
    }

    // word-font
    if let Some(ref font) = defaults.word_font {
        emit_font_element(w, "word-font", font)?;
    }

    // lyric-font
    for lf in &defaults.lyric_fonts {
        emit_lyric_font(w, lf)?;
    }

    // lyric-language
    for ll in &defaults.lyric_languages {
        emit_lyric_language(w, ll)?;
    }

    w.end_element("defaults")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the scaling element.
fn emit_scaling(w: &mut XmlWriter, scaling: &Scaling) -> Result<(), EmitError> {
    w.start_element("scaling")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.text_element("millimeters", &scaling.millimeters.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.text_element("tenths", &scaling.tenths.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.end_element("scaling")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the page-layout element.
fn emit_page_layout(w: &mut XmlWriter, pl: &PageLayout) -> Result<(), EmitError> {
    w.start_element("page-layout")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if let Some(height) = pl.page_height {
        w.text_element("page-height", &height.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    if let Some(width) = pl.page_width {
        w.text_element("page-width", &width.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    for margins in &pl.page_margins {
        emit_page_margins(w, margins)?;
    }

    w.end_element("page-layout")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the page-margins element.
fn emit_page_margins(w: &mut XmlWriter, pm: &PageMargins) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("page-margins");

    if let Some(ref mt) = pm.r#type {
        elem = elem.attr("type", margin_type_to_string(mt));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.text_element("left-margin", &pm.left.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.text_element("right-margin", &pm.right.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.text_element("top-margin", &pm.top.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.text_element("bottom-margin", &pm.bottom.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.end_element("page-margins")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the system-layout element.
fn emit_system_layout(w: &mut XmlWriter, sl: &SystemLayout) -> Result<(), EmitError> {
    w.start_element("system-layout")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if let Some(ref margins) = sl.system_margins {
        emit_system_margins(w, margins)?;
    }
    if let Some(dist) = sl.system_distance {
        w.text_element("system-distance", &dist.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    if let Some(dist) = sl.top_system_distance {
        w.text_element("top-system-distance", &dist.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    if let Some(ref dividers) = sl.system_dividers {
        emit_system_dividers(w, dividers)?;
    }

    w.end_element("system-layout")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the system-margins element.
fn emit_system_margins(w: &mut XmlWriter, sm: &SystemMargins) -> Result<(), EmitError> {
    w.start_element("system-margins")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.text_element("left-margin", &sm.left.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.text_element("right-margin", &sm.right.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.end_element("system-margins")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the system-dividers element.
fn emit_system_dividers(w: &mut XmlWriter, sd: &SystemDividers) -> Result<(), EmitError> {
    w.start_element("system-dividers")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if let Some(ref left) = sd.left_divider {
        let mut elem = ElementBuilder::new("left-divider");
        if let Some(ref po) = left.print_object {
            elem = elem.attr("print-object", yes_no_to_string(po));
        }
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    if let Some(ref right) = sd.right_divider {
        let mut elem = ElementBuilder::new("right-divider");
        if let Some(ref po) = right.print_object {
            elem = elem.attr("print-object", yes_no_to_string(po));
        }
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("system-dividers")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the staff-layout element.
fn emit_staff_layout(w: &mut XmlWriter, sl: &StaffLayout) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("staff-layout");

    if let Some(number) = sl.number {
        elem = elem.attr("number", &number.to_string());
    }

    if let Some(dist) = sl.staff_distance {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.text_element("staff-distance", &dist.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("staff-layout")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit the appearance element.
fn emit_appearance(w: &mut XmlWriter, app: &Appearance) -> Result<(), EmitError> {
    w.start_element("appearance")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for lw in &app.line_widths {
        emit_line_width(w, lw)?;
    }
    for ns in &app.note_sizes {
        emit_note_size(w, ns)?;
    }
    for dist in &app.distances {
        emit_distance(w, dist)?;
    }
    // other-appearance elements not implemented (rarely used)

    w.end_element("appearance")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a line-width element.
fn emit_line_width(w: &mut XmlWriter, lw: &LineWidth) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("line-width").attr("type", &lw.r#type);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&lw.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("line-width")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a note-size element.
fn emit_note_size(w: &mut XmlWriter, ns: &NoteSize) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("note-size").attr("type", note_size_type_to_string(&ns.r#type));
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&ns.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("note-size")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a distance element.
fn emit_distance(w: &mut XmlWriter, dist: &Distance) -> Result<(), EmitError> {
    let elem = ElementBuilder::new("distance").attr("type", &dist.r#type);
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&dist.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("distance")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a font element (music-font, word-font).
fn emit_font_element(
    w: &mut XmlWriter,
    name: &str,
    font: &crate::ir::common::Font,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref family) = font.font_family {
        elem = elem.attr("font-family", family);
    }
    if let Some(ref style) = font.font_style {
        let style_str = match style {
            crate::ir::common::FontStyle::Normal => "normal",
            crate::ir::common::FontStyle::Italic => "italic",
        };
        elem = elem.attr("font-style", style_str);
    }
    if let Some(ref size) = font.font_size {
        elem = elem.attr("font-size", &font_size_to_string(size));
    }
    if let Some(ref weight) = font.font_weight {
        let weight_str = match weight {
            crate::ir::common::FontWeight::Normal => "normal",
            crate::ir::common::FontWeight::Bold => "bold",
        };
        elem = elem.attr("font-weight", weight_str);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a lyric-font element.
fn emit_lyric_font(w: &mut XmlWriter, lf: &LyricFont) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("lyric-font");

    if let Some(ref number) = lf.number {
        elem = elem.attr("number", number);
    }
    if let Some(ref name) = lf.name {
        elem = elem.attr("name", name);
    }
    if let Some(ref family) = lf.font.font_family {
        elem = elem.attr("font-family", family);
    }
    if let Some(ref size) = lf.font.font_size {
        elem = elem.attr("font-size", &font_size_to_string(size));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a lyric-language element.
fn emit_lyric_language(w: &mut XmlWriter, ll: &LyricLanguage) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("lyric-language");

    if let Some(ref number) = ll.number {
        elem = elem.attr("number", number);
    }
    if let Some(ref name) = ll.name {
        elem = elem.attr("name", name);
    }
    elem = elem.attr("xml:lang", &ll.lang);

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a credit element.
pub(crate) fn emit_credit(w: &mut XmlWriter, credit: &Credit) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("credit");

    if let Some(page) = credit.page {
        elem = elem.attr("page", &page.to_string());
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for content in &credit.content {
        emit_credit_content(w, content)?;
    }

    w.end_element("credit")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a credit content element.
fn emit_credit_content(w: &mut XmlWriter, content: &CreditContent) -> Result<(), EmitError> {
    match content {
        CreditContent::CreditType(ct) => {
            w.text_element("credit-type", ct)
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        CreditContent::Link(_) | CreditContent::Bookmark(_) => {
            // Links and bookmarks are rarely used in MusicXML credits
            // Not implementing for now
        }
        CreditContent::CreditImage(ci) => {
            emit_credit_image(w, ci)?;
        }
        CreditContent::CreditWords(cw) => {
            emit_credit_words(w, cw)?;
        }
        CreditContent::CreditSymbol(cs) => {
            emit_credit_symbol(w, cs)?;
        }
    }
    Ok(())
}

/// Emit a credit-image element.
fn emit_credit_image(w: &mut XmlWriter, ci: &CreditImage) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("credit-image")
        .attr("source", &ci.source)
        .attr("type", &ci.r#type);

    if let Some(dx) = ci.position.default_x {
        elem = elem.attr("default-x", &dx.to_string());
    }
    if let Some(dy) = ci.position.default_y {
        elem = elem.attr("default-y", &dy.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a credit-words element.
fn emit_credit_words(w: &mut XmlWriter, cw: &CreditWords) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("credit-words");

    if let Some(dx) = cw.print_style.position.default_x {
        elem = elem.attr("default-x", &dx.to_string());
    }
    if let Some(dy) = cw.print_style.position.default_y {
        elem = elem.attr("default-y", &dy.to_string());
    }
    if let Some(ref justify) = cw.justify {
        elem = elem.attr("justify", left_center_right_to_string(justify));
    }
    if let Some(ref halign) = cw.halign {
        elem = elem.attr("halign", left_center_right_to_string(halign));
    }
    if let Some(ref valign) = cw.valign {
        elem = elem.attr("valign", top_middle_bottom_to_string(valign));
    }
    // Font attributes from print_style.font
    if let Some(ref family) = cw.print_style.font.font_family {
        elem = elem.attr("font-family", family);
    }
    if let Some(ref size) = cw.print_style.font.font_size {
        elem = elem.attr("font-size", &font_size_to_string(size));
    }
    if let Some(ref lang) = cw.lang {
        elem = elem.attr("xml:lang", lang);
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&cw.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("credit-words")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a credit-symbol element.
fn emit_credit_symbol(w: &mut XmlWriter, cs: &CreditSymbol) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("credit-symbol");

    if let Some(dx) = cs.print_style.position.default_x {
        elem = elem.attr("default-x", &dx.to_string());
    }
    if let Some(dy) = cs.print_style.position.default_y {
        elem = elem.attr("default-y", &dy.to_string());
    }
    if let Some(ref justify) = cs.justify {
        elem = elem.attr("justify", left_center_right_to_string(justify));
    }
    if let Some(ref halign) = cs.halign {
        elem = elem.attr("halign", left_center_right_to_string(halign));
    }
    if let Some(ref valign) = cs.valign {
        elem = elem.attr("valign", top_middle_bottom_to_string(valign));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&cs.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("credit-symbol")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the part-list element.
pub(crate) fn emit_part_list(w: &mut XmlWriter, part_list: &PartList) -> Result<(), EmitError> {
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
pub(crate) fn emit_score_part(w: &mut XmlWriter, sp: &ScorePart) -> Result<(), EmitError> {
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
pub(crate) fn emit_part_group(w: &mut XmlWriter, _pg: &PartGroup) -> Result<(), EmitError> {
    // TODO: implement part-group emission
    // For now, this is a stub that does nothing
    let _ = w;
    Ok(())
}

/// Emit a part element.
pub(crate) fn emit_part(w: &mut XmlWriter, part: &Part) -> Result<(), EmitError> {
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
pub(crate) fn emit_measure(w: &mut XmlWriter, measure: &Measure) -> Result<(), EmitError> {
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
pub(crate) fn emit_music_data(
    w: &mut XmlWriter,
    element: &MusicDataElement,
) -> Result<(), EmitError> {
    match element {
        MusicDataElement::Note(note) => emit_note(w, note),
        MusicDataElement::Backup(backup) => emit_backup(w, backup),
        MusicDataElement::Forward(forward) => emit_forward(w, forward),
        MusicDataElement::Direction(dir) => emit_direction(w, dir),
        MusicDataElement::Attributes(attrs) => emit_attributes(w, attrs),
        MusicDataElement::Barline(barline) => emit_barline(w, barline),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::PrintStyle;
    use crate::ir::attributes::{
        Attributes, Clef, ClefSign, Key, KeyContent, Mode, Time, TimeContent, TimeSignature,
        TraditionalKey,
    };
    use crate::ir::common::{Editorial, FontSize};
    use crate::ir::part::PartName;

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

    #[test]
    fn test_emit_score_with_attributes() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
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
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<attributes>"));
        assert!(xml.contains("<divisions>4</divisions>"));
        assert!(xml.contains("<fifths>0</fifths>"));
        assert!(xml.contains("<mode>major</mode>"));
        assert!(xml.contains("</attributes>"));
    }

    // =======================================================================
    // Integration Tests: Full Score Scenarios
    // =======================================================================

    #[test]
    fn test_emit_twinkle_twinkle_first_phrase() {
        use crate::ir::common::Position;
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        // "Twinkle Twinkle Little Star" first phrase: C C G G A A G (half)
        let mut score = create_minimal_score();

        // Add attributes
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
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
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        // Helper to create a quarter note
        let make_quarter = |step: Step| -> MusicDataElement {
            MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            }))
        };

        // Helper to create a half note
        let make_half = |step: Step| -> MusicDataElement {
            MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            }))
        };

        // Measure 1: C C G G
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::C));
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::C));
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::G));
        score.parts[0].measures[0]
            .content
            .push(make_quarter(Step::G));

        // Measure 2: A A G (half)
        score.parts[0].measures.push(Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                make_quarter(Step::A),
                make_quarter(Step::A),
                make_half(Step::G),
            ],
        });

        let xml = emit_score(&score).unwrap();

        // Verify structure
        assert!(xml.contains("<measure number=\"1\">"));
        assert!(xml.contains("<measure number=\"2\">"));
        assert!(xml.contains("<divisions>4</divisions>"));
        assert!(xml.contains("<fifths>0</fifths>"));
        assert!(xml.contains("<mode>major</mode>"));
        assert!(xml.contains("<sign>G</sign>"));

        // Verify notes (should have C, C, G, G in measure 1)
        let c_count = xml.matches("<step>C</step>").count();
        let g_count = xml.matches("<step>G</step>").count();
        let a_count = xml.matches("<step>A</step>").count();

        assert_eq!(c_count, 2, "Should have 2 C notes");
        assert_eq!(g_count, 3, "Should have 3 G notes (2 quarters + 1 half)");
        assert_eq!(a_count, 2, "Should have 2 A notes");

        // Verify we have quarter and half notes
        assert!(xml.contains("<type>quarter</type>"));
        assert!(xml.contains("<type>half</type>"));
    }

    #[test]
    fn test_emit_multi_voice_with_backup() {
        use crate::ir::beam::{Stem, StemValue};
        use crate::ir::common::Position;
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};
        use crate::ir::voice::Backup;

        // Two voices: Voice 1 has C4 half, D4 half; Voice 2 has E3 half, F3 half
        let mut score = create_minimal_score();

        // Add attributes
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
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
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        // Voice 1: C4 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Up,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Voice 1: D4 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::D,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Up,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Backup to start of measure for voice 2
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Backup(Backup {
                duration: 16, // Full measure
                editorial: Editorial::default(),
            }));

        // Voice 2: E3 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::E,
                            alter: None,
                            octave: 3,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("2".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Down,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Voice 2: F3 half
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::F,
                            alter: None,
                            octave: 3,
                        }),
                    },
                    duration: 8,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("2".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Half,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: Some(Stem {
                    value: StemValue::Down,
                    default_y: None,
                    color: None,
                }),
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify two voices
        assert!(xml.contains("<voice>1</voice>"));
        assert!(xml.contains("<voice>2</voice>"));

        // Verify backup
        assert!(xml.contains("<backup>"));
        assert!(xml.contains("<duration>16</duration>"));

        // Verify stem directions
        assert!(xml.contains("<stem>up</stem>"));
        assert!(xml.contains("<stem>down</stem>"));

        // Verify all pitches are present
        assert!(xml.contains("<step>C</step>"));
        assert!(xml.contains("<step>D</step>"));
        assert!(xml.contains("<step>E</step>"));
        assert!(xml.contains("<step>F</step>"));
    }

    #[test]
    fn test_emit_repeat_with_volta_brackets() {
        use crate::ir::attributes::{BarStyle, Barline, Ending, Repeat};
        use crate::ir::common::{BackwardForward, Position, RightLeftMiddle, StartStopDiscontinue};
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();

        // Measure 1: Start repeat
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Barline(Box::new(Barline {
                location: Some(RightLeftMiddle::Left),
                bar_style: Some(BarStyle::HeavyLight),
                editorial: Editorial::default(),
                wavy_line: None,
                segno: None,
                coda: None,
                fermatas: vec![],
                ending: None,
                repeat: Some(Repeat {
                    direction: BackwardForward::Forward,
                    times: None,
                    winged: None,
                }),
            })));

        // Add a note to measure 1
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 16,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Whole,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        // Measure 2: First ending with backward repeat
        score.parts[0].measures.push(Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                // First ending start
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Left),
                    bar_style: None,
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Start,
                        number: "1".to_string(),
                        text: Some("1.".to_string()),
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: None,
                })),
                // A whole note
                MusicDataElement::Note(Box::new(Note {
                    position: Position::default(),
                    dynamics: None,
                    end_dynamics: None,
                    attack: None,
                    release: None,
                    pizzicato: None,
                    print_object: None,
                    content: NoteContent::Regular {
                        full_note: FullNote {
                            chord: false,
                            content: PitchRestUnpitched::Pitch(Pitch {
                                step: Step::D,
                                alter: None,
                                octave: 4,
                            }),
                        },
                        duration: 16,
                        ties: vec![],
                    },
                    instrument: vec![],
                    voice: Some("1".to_string()),
                    r#type: Some(NoteType {
                        value: NoteTypeValue::Whole,
                        size: None,
                    }),
                    dots: vec![],
                    accidental: None,
                    time_modification: None,
                    stem: None,
                    notehead: None,
                    staff: None,
                    beams: vec![],
                    notations: vec![],
                    lyrics: vec![],
                })),
                // End of first ending with backward repeat
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Right),
                    bar_style: Some(BarStyle::LightHeavy),
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Stop,
                        number: "1".to_string(),
                        text: None,
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: Some(Repeat {
                        direction: BackwardForward::Backward,
                        times: None,
                        winged: None,
                    }),
                })),
            ],
        });

        // Measure 3: Second ending
        score.parts[0].measures.push(Measure {
            number: "3".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                // Second ending start
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Left),
                    bar_style: None,
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Start,
                        number: "2".to_string(),
                        text: Some("2.".to_string()),
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: None,
                })),
                // E whole note
                MusicDataElement::Note(Box::new(Note {
                    position: Position::default(),
                    dynamics: None,
                    end_dynamics: None,
                    attack: None,
                    release: None,
                    pizzicato: None,
                    print_object: None,
                    content: NoteContent::Regular {
                        full_note: FullNote {
                            chord: false,
                            content: PitchRestUnpitched::Pitch(Pitch {
                                step: Step::E,
                                alter: None,
                                octave: 4,
                            }),
                        },
                        duration: 16,
                        ties: vec![],
                    },
                    instrument: vec![],
                    voice: Some("1".to_string()),
                    r#type: Some(NoteType {
                        value: NoteTypeValue::Whole,
                        size: None,
                    }),
                    dots: vec![],
                    accidental: None,
                    time_modification: None,
                    stem: None,
                    notehead: None,
                    staff: None,
                    beams: vec![],
                    notations: vec![],
                    lyrics: vec![],
                })),
                // End of second ending (discontinue - no line at end)
                MusicDataElement::Barline(Box::new(Barline {
                    location: Some(RightLeftMiddle::Right),
                    bar_style: Some(BarStyle::LightHeavy),
                    editorial: Editorial::default(),
                    wavy_line: None,
                    segno: None,
                    coda: None,
                    fermatas: vec![],
                    ending: Some(Ending {
                        r#type: StartStopDiscontinue::Discontinue,
                        number: "2".to_string(),
                        text: None,
                        print_object: None,
                        end_length: None,
                        text_x: None,
                        text_y: None,
                    }),
                    repeat: None,
                })),
            ],
        });

        let xml = emit_score(&score).unwrap();

        // Verify forward repeat
        assert!(xml.contains("<repeat direction=\"forward\"/>"));

        // Verify backward repeat
        assert!(xml.contains("<repeat direction=\"backward\"/>"));

        // Verify first ending
        assert!(xml.contains("<ending number=\"1\" type=\"start\">1.</ending>"));
        assert!(xml.contains("<ending number=\"1\" type=\"stop\"/>"));

        // Verify second ending
        assert!(xml.contains("<ending number=\"2\" type=\"start\">2.</ending>"));
        assert!(xml.contains("<ending number=\"2\" type=\"discontinue\"/>"));

        // Verify bar styles
        assert!(xml.contains("<bar-style>heavy-light</bar-style>"));
        assert!(xml.contains("<bar-style>light-heavy</bar-style>"));
    }

    #[test]
    fn test_emit_direction_and_notations_integration() {
        use crate::ir::common::AboveBelow;
        use crate::ir::common::{EmptyPlacement, Position, StartStopContinue};
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics, Wedge,
            WedgeType,
        };
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::notation::{
            ArticulationElement, Articulations, NotationContent, Notations, Slur,
        };
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();

        // Add attributes
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0,
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
                    line: Some(2),
                    octave_change: None,
                    number: None,
                    size: None,
                    print_object: None,
                }],
                staff_details: vec![],
                transpose: vec![],
                measure_styles: vec![],
            })));

        // Add forte direction
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![DynamicElement::F],
                        print_style: PrintStyle::default(),
                        placement: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        // Add crescendo start
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: WedgeType::Crescendo,
                        number: Some(1),
                        spread: None,
                        niente: None,
                        line_type: None,
                        position: Position::default(),
                        color: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        // Add a note with slur and staccato
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![Notations {
                    print_object: None,
                    content: vec![
                        NotationContent::Slur(Slur {
                            r#type: StartStopContinue::Start,
                            number: 1,
                            line_type: None,
                            position: Position::default(),
                            placement: Some(AboveBelow::Above),
                            orientation: None,
                            color: None,
                        }),
                        NotationContent::Articulations(Box::new(Articulations {
                            content: vec![ArticulationElement::Staccato(EmptyPlacement {
                                placement: Some(AboveBelow::Above),
                                position: Position::default(),
                            })],
                        })),
                    ],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify structure
        assert!(xml.contains("<direction placement=\"below\">"));
        assert!(xml.contains("<dynamics>"));
        assert!(xml.contains("<f/>"));
        assert!(xml.contains("<wedge type=\"crescendo\""));
        assert!(xml.contains("<notations>"));
        assert!(xml.contains("<slur type=\"start\""));
        assert!(xml.contains("<articulations>"));
        assert!(xml.contains("<staccato"));
    }

    // =======================================================================
    // Milestone 5 Integration Tests
    // =======================================================================

    #[test]
    fn test_emit_score_with_work_and_identification() {
        use crate::ir::common::{
            Encoding, EncodingContent, Identification, Supports, TypedText, YesNo,
        };
        use crate::ir::score::Work;

        let mut score = create_minimal_score();
        score.work = Some(Work {
            work_number: Some("Op. 1".to_string()),
            work_title: Some("Symphony No. 1".to_string()),
            opus: None,
        });
        score.movement_number = Some("1".to_string());
        score.movement_title = Some("Allegro con brio".to_string());
        score.identification = Some(Identification {
            creators: vec![
                TypedText {
                    value: "Ludwig van Beethoven".to_string(),
                    r#type: Some("composer".to_string()),
                },
                TypedText {
                    value: "Anonymous".to_string(),
                    r#type: Some("lyricist".to_string()),
                },
            ],
            rights: vec![TypedText {
                value: "Public Domain".to_string(),
                r#type: Some("copyright".to_string()),
            }],
            encoding: Some(Encoding {
                content: vec![
                    EncodingContent::Software("Fermata 1.0".to_string()),
                    EncodingContent::EncodingDate("2024-01-15".to_string()),
                    EncodingContent::Supports(Supports {
                        r#type: YesNo::Yes,
                        element: "accidental".to_string(),
                        attribute: None,
                        value: None,
                    }),
                ],
            }),
            source: Some("Original manuscript".to_string()),
            relations: vec![],
            miscellaneous: None,
        });

        let xml = emit_score(&score).unwrap();

        // Verify work
        assert!(xml.contains("<work>"));
        assert!(xml.contains("<work-number>Op. 1</work-number>"));
        assert!(xml.contains("<work-title>Symphony No. 1</work-title>"));
        assert!(xml.contains("</work>"));

        // Verify movement
        assert!(xml.contains("<movement-number>1</movement-number>"));
        assert!(xml.contains("<movement-title>Allegro con brio</movement-title>"));

        // Verify identification
        assert!(xml.contains("<identification>"));
        assert!(xml.contains("<creator type=\"composer\">Ludwig van Beethoven</creator>"));
        assert!(xml.contains("<creator type=\"lyricist\">Anonymous</creator>"));
        assert!(xml.contains("<rights type=\"copyright\">Public Domain</rights>"));
        assert!(xml.contains("<software>Fermata 1.0</software>"));
        assert!(xml.contains("<encoding-date>2024-01-15</encoding-date>"));
        assert!(xml.contains("<supports type=\"yes\" element=\"accidental\"/>"));
        assert!(xml.contains("<source>Original manuscript</source>"));
        assert!(xml.contains("</identification>"));
    }

    #[test]
    fn test_emit_score_with_defaults() {
        use crate::ir::common::Font;
        use crate::ir::score::{
            Defaults, LineWidth, MarginType, NoteSize, NoteSizeType, PageLayout, PageMargins,
            Scaling, StaffLayout, SystemLayout, SystemMargins,
        };

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: Some(Scaling {
                millimeters: 7.056,
                tenths: 40.0,
            }),
            page_layout: Some(PageLayout {
                page_height: Some(1683.36),
                page_width: Some(1190.88),
                page_margins: vec![PageMargins {
                    r#type: Some(MarginType::Both),
                    left: 70.0,
                    right: 70.0,
                    top: 88.0,
                    bottom: 88.0,
                }],
            }),
            system_layout: Some(SystemLayout {
                system_margins: Some(SystemMargins {
                    left: 0.0,
                    right: 0.0,
                }),
                system_distance: Some(121.0),
                top_system_distance: Some(70.0),
                system_dividers: None,
            }),
            staff_layout: vec![StaffLayout {
                number: Some(2),
                staff_distance: Some(65.0),
            }],
            appearance: Some(crate::ir::score::Appearance {
                line_widths: vec![
                    LineWidth {
                        r#type: "stem".to_string(),
                        value: 0.8333,
                    },
                    LineWidth {
                        r#type: "beam".to_string(),
                        value: 5.0,
                    },
                ],
                note_sizes: vec![NoteSize {
                    r#type: NoteSizeType::Cue,
                    value: 60.0,
                }],
                distances: vec![],
                other_appearances: vec![],
            }),
            music_font: Some(Font {
                font_family: Some("Bravura".to_string()),
                font_style: None,
                font_size: None,
                font_weight: None,
            }),
            word_font: Some(Font {
                font_family: Some("Times New Roman".to_string()),
                font_style: None,
                font_size: Some(FontSize::Points(10.0)),
                font_weight: None,
            }),
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        // Verify defaults
        assert!(xml.contains("<defaults>"));

        // Verify scaling
        assert!(xml.contains("<scaling>"));
        assert!(xml.contains("<millimeters>7.056</millimeters>"));
        assert!(xml.contains("<tenths>40</tenths>"));
        assert!(xml.contains("</scaling>"));

        // Verify page layout
        assert!(xml.contains("<page-layout>"));
        assert!(xml.contains("<page-height>1683.36</page-height>"));
        assert!(xml.contains("<page-width>1190.88</page-width>"));
        assert!(xml.contains("<page-margins type=\"both\">"));
        assert!(xml.contains("<left-margin>70</left-margin>"));
        assert!(xml.contains("</page-layout>"));

        // Verify system layout
        assert!(xml.contains("<system-layout>"));
        assert!(xml.contains("<system-margins>"));
        assert!(xml.contains("<system-distance>121</system-distance>"));
        assert!(xml.contains("</system-layout>"));

        // Verify staff layout
        assert!(xml.contains("<staff-layout number=\"2\">"));
        assert!(xml.contains("<staff-distance>65</staff-distance>"));
        assert!(xml.contains("</staff-layout>"));

        // Verify appearance
        assert!(xml.contains("<appearance>"));
        assert!(xml.contains("<line-width type=\"stem\">"));
        assert!(xml.contains("<note-size type=\"cue\">60</note-size>"));
        assert!(xml.contains("</appearance>"));

        // Verify fonts
        assert!(xml.contains("<music-font font-family=\"Bravura\"/>"));
        assert!(xml.contains("<word-font font-family=\"Times New Roman\" font-size=\"10\"/>"));

        assert!(xml.contains("</defaults>"));
    }

    #[test]
    fn test_emit_score_with_credits() {
        use crate::ir::common::{LeftCenterRight, Position, TopMiddleBottom};
        use crate::ir::score::{Credit, CreditContent, CreditWords};

        let mut score = create_minimal_score();
        score.credits = vec![
            Credit {
                page: Some(1),
                content: vec![
                    CreditContent::CreditType("title".to_string()),
                    CreditContent::CreditWords(CreditWords {
                        value: "Symphony No. 5".to_string(),
                        print_style: PrintStyle {
                            position: Position {
                                default_x: Some(595.0),
                                default_y: Some(1560.0),
                                relative_x: None,
                                relative_y: None,
                            },
                            font: crate::ir::common::Font {
                                font_family: Some("Times New Roman".to_string()),
                                font_style: None,
                                font_size: Some(crate::ir::common::FontSize::Points(24.0)),
                                font_weight: None,
                            },
                            color: None,
                        },
                        justify: Some(LeftCenterRight::Center),
                        halign: Some(LeftCenterRight::Center),
                        valign: Some(TopMiddleBottom::Top),
                        lang: None,
                    }),
                ],
            },
            Credit {
                page: Some(1),
                content: vec![
                    CreditContent::CreditType("composer".to_string()),
                    CreditContent::CreditWords(CreditWords {
                        value: "Ludwig van Beethoven".to_string(),
                        print_style: PrintStyle {
                            position: Position {
                                default_x: Some(1100.0),
                                default_y: Some(1480.0),
                                relative_x: None,
                                relative_y: None,
                            },
                            font: crate::ir::common::Font::default(),
                            color: None,
                        },
                        justify: Some(LeftCenterRight::Right),
                        halign: None,
                        valign: None,
                        lang: None,
                    }),
                ],
            },
        ];

        let xml = emit_score(&score).unwrap();

        // Verify credits
        assert!(xml.contains("<credit page=\"1\">"));
        assert!(xml.contains("<credit-type>title</credit-type>"));
        assert!(xml.contains("<credit-words"));
        assert!(xml.contains("default-x=\"595\""));
        assert!(xml.contains("default-y=\"1560\""));
        assert!(xml.contains("justify=\"center\""));
        assert!(xml.contains("font-family=\"Times New Roman\""));
        assert!(xml.contains("font-size=\"24\""));
        assert!(xml.contains(">Symphony No. 5</credit-words>"));

        assert!(xml.contains("<credit-type>composer</credit-type>"));
        assert!(xml.contains("justify=\"right\""));
        assert!(xml.contains(">Ludwig van Beethoven</credit-words>"));
    }

    #[test]
    fn test_emit_note_with_lyrics() {
        use crate::ir::common::{AboveBelow, Position, StartStopContinue};
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::lyric::{Extend, Lyric, LyricContent, Syllabic, TextElementData};
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::C,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![
                    Lyric {
                        number: Some("1".to_string()),
                        name: None,
                        justify: None,
                        placement: Some(AboveBelow::Below),
                        print_object: None,
                        content: LyricContent::Syllable {
                            syllabic: Some(Syllabic::Begin),
                            text: TextElementData {
                                value: "Hap".to_string(),
                                font: crate::ir::common::Font::default(),
                                color: None,
                                lang: None,
                            },
                            extensions: vec![],
                            extend: None,
                        },
                        end_line: false,
                        end_paragraph: false,
                    },
                    Lyric {
                        number: Some("2".to_string()),
                        name: None,
                        justify: None,
                        placement: Some(AboveBelow::Below),
                        print_object: None,
                        content: LyricContent::Syllable {
                            syllabic: Some(Syllabic::End),
                            text: TextElementData {
                                value: "py".to_string(),
                                font: crate::ir::common::Font::default(),
                                color: None,
                                lang: None,
                            },
                            extensions: vec![],
                            extend: Some(Extend {
                                r#type: Some(StartStopContinue::Start),
                                position: Position::default(),
                                color: None,
                            }),
                        },
                        end_line: false,
                        end_paragraph: false,
                    },
                ],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify lyrics
        assert!(xml.contains("<lyric number=\"1\" placement=\"below\">"));
        assert!(xml.contains("<syllabic>begin</syllabic>"));
        assert!(xml.contains("<text>Hap</text>"));
        assert!(xml.contains("</lyric>"));

        assert!(xml.contains("<lyric number=\"2\" placement=\"below\">"));
        assert!(xml.contains("<syllabic>end</syllabic>"));
        assert!(xml.contains("<text>py</text>"));
        assert!(xml.contains("<extend type=\"start\"/>"));
    }

    #[test]
    fn test_emit_note_with_ornaments() {
        use crate::ir::common::{AboveBelow, Position, YesNo};
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::notation::{
            EmptyTrillSound, Mordent, NotationContent, Notations, OrnamentElement,
            OrnamentWithAccidentals, Ornaments, Tremolo, TremoloType,
        };
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::G,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Ornaments(Box::new(Ornaments {
                        content: vec![
                            OrnamentWithAccidentals {
                                ornament: OrnamentElement::TrillMark(EmptyTrillSound {
                                    placement: Some(AboveBelow::Above),
                                    ..Default::default()
                                }),
                                accidental_marks: vec![],
                            },
                            OrnamentWithAccidentals {
                                ornament: OrnamentElement::Mordent(Mordent {
                                    long: Some(YesNo::Yes),
                                    placement: Some(AboveBelow::Above),
                                    ..Default::default()
                                }),
                                accidental_marks: vec![],
                            },
                            OrnamentWithAccidentals {
                                ornament: OrnamentElement::Tremolo(Tremolo {
                                    value: 3,
                                    r#type: Some(TremoloType::Single),
                                    placement: Some(AboveBelow::Below),
                                    position: Position::default(),
                                }),
                                accidental_marks: vec![],
                            },
                        ],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify ornaments
        assert!(xml.contains("<ornaments>"));
        assert!(xml.contains("<trill-mark placement=\"above\"/>"));
        assert!(xml.contains("<mordent long=\"yes\" placement=\"above\"/>"));
        assert!(xml.contains("<tremolo type=\"single\" placement=\"below\">3</tremolo>"));
        assert!(xml.contains("</ornaments>"));
    }

    #[test]
    fn test_emit_note_with_technical() {
        use crate::ir::common::{AboveBelow, Font, Position};
        use crate::ir::duration::{NoteType, NoteTypeValue};
        use crate::ir::notation::{
            Fingering, Fret, NotationContent, Notations, StringNumber, Technical, TechnicalElement,
        };
        use crate::ir::note::{FullNote, Note, NoteContent, PitchRestUnpitched};
        use crate::ir::pitch::{Pitch, Step};

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Note(Box::new(Note {
                position: Position::default(),
                dynamics: None,
                end_dynamics: None,
                attack: None,
                release: None,
                pizzicato: None,
                print_object: None,
                content: NoteContent::Regular {
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::E,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    duration: 4,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Technical(Box::new(Technical {
                        content: vec![
                            TechnicalElement::Fingering(Fingering {
                                value: "2".to_string(),
                                substitution: None,
                                alternate: None,
                                placement: Some(AboveBelow::Above),
                                print_style: PrintStyle::default(),
                            }),
                            TechnicalElement::String(StringNumber {
                                value: 1,
                                placement: Some(AboveBelow::Above),
                                print_style: PrintStyle::default(),
                            }),
                            TechnicalElement::Fret(Fret {
                                value: 5,
                                font: Font::default(),
                                color: None,
                            }),
                        ],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        // Verify technical marks
        assert!(xml.contains("<technical>"));
        assert!(xml.contains("<fingering placement=\"above\">2</fingering>"));
        assert!(xml.contains("<string placement=\"above\">1</string>"));
        assert!(xml.contains("<fret>5</fret>"));
        assert!(xml.contains("</technical>"));
    }

    // =======================================================================
    // Additional tests for uncovered paths
    // =======================================================================

    #[test]
    fn test_emit_empty_work() {
        use crate::ir::score::Work;

        let mut score = create_minimal_score();
        // Empty work should not produce any output
        score.work = Some(Work {
            work_number: None,
            work_title: None,
            opus: None,
        });

        let xml = emit_score(&score).unwrap();
        // Empty work should not produce <work> element
        assert!(!xml.contains("<work>"));
    }

    #[test]
    fn test_emit_work_with_opus() {
        use crate::ir::score::{Opus, Work};

        let mut score = create_minimal_score();
        score.work = Some(Work {
            work_number: None,
            work_title: Some("Sonata".to_string()),
            opus: Some(Opus {
                href: "opus1.xml".to_string(),
            }),
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<work>"));
        assert!(xml.contains("<work-title>Sonata</work-title>"));
        assert!(xml.contains("<opus xlink:href=\"opus1.xml\"/>"));
        assert!(xml.contains("</work>"));
    }

    #[test]
    fn test_emit_identification_with_relations() {
        use crate::ir::common::{Identification, TypedText};

        let mut score = create_minimal_score();
        score.identification = Some(Identification {
            creators: vec![],
            rights: vec![],
            encoding: None,
            source: None,
            relations: vec![
                TypedText {
                    value: "http://example.com/related".to_string(),
                    r#type: Some("related-to".to_string()),
                },
                TypedText {
                    value: "Another relation".to_string(),
                    r#type: None,
                },
            ],
            miscellaneous: None,
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<identification>"));
        assert!(
            xml.contains("<relation type=\"related-to\">http://example.com/related</relation>")
        );
        assert!(xml.contains("<relation>Another relation</relation>"));
        assert!(xml.contains("</identification>"));
    }

    #[test]
    fn test_emit_encoding_with_all_content_types() {
        use crate::ir::common::{
            Encoding, EncodingContent, Identification, Supports, TypedText, YesNo,
        };

        let mut score = create_minimal_score();
        score.identification = Some(Identification {
            creators: vec![],
            rights: vec![],
            encoding: Some(Encoding {
                content: vec![
                    EncodingContent::EncodingDate("2024-06-15".to_string()),
                    EncodingContent::Encoder(TypedText {
                        value: "John Doe".to_string(),
                        r#type: Some("transcriber".to_string()),
                    }),
                    EncodingContent::Software("Test Software".to_string()),
                    EncodingContent::EncodingDescription("Test description".to_string()),
                    EncodingContent::Supports(Supports {
                        r#type: YesNo::Yes,
                        element: "print".to_string(),
                        attribute: Some("new-page".to_string()),
                        value: Some("yes".to_string()),
                    }),
                ],
            }),
            source: None,
            relations: vec![],
            miscellaneous: None,
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<encoding>"));
        assert!(xml.contains("<encoding-date>2024-06-15</encoding-date>"));
        assert!(xml.contains("<encoder type=\"transcriber\">John Doe</encoder>"));
        assert!(xml.contains("<software>Test Software</software>"));
        assert!(xml.contains("<encoding-description>Test description</encoding-description>"));
        assert!(xml.contains(
            "<supports type=\"yes\" element=\"print\" attribute=\"new-page\" value=\"yes\"/>"
        ));
        assert!(xml.contains("</encoding>"));
    }

    #[test]
    fn test_emit_credit_with_image() {
        use crate::ir::common::Position;
        use crate::ir::score::{Credit, CreditContent, CreditImage};

        let mut score = create_minimal_score();
        score.credits = vec![Credit {
            page: Some(1),
            content: vec![CreditContent::CreditImage(CreditImage {
                source: "logo.png".to_string(),
                r#type: "image/png".to_string(),
                position: Position {
                    default_x: Some(100.0),
                    default_y: Some(200.0),
                    relative_x: None,
                    relative_y: None,
                },
            })],
        }];

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<credit-image source=\"logo.png\" type=\"image/png\" default-x=\"100\" default-y=\"200\"/>"));
    }

    #[test]
    fn test_emit_credit_with_symbol() {
        use crate::ir::common::{LeftCenterRight, Position, TopMiddleBottom};
        use crate::ir::score::{Credit, CreditContent, CreditSymbol};

        let mut score = create_minimal_score();
        score.credits = vec![Credit {
            page: None,
            content: vec![CreditContent::CreditSymbol(CreditSymbol {
                value: "coda".to_string(),
                print_style: PrintStyle {
                    position: Position {
                        default_x: Some(50.0),
                        default_y: Some(100.0),
                        relative_x: None,
                        relative_y: None,
                    },
                    font: crate::ir::common::Font::default(),
                    color: None,
                },
                justify: Some(LeftCenterRight::Left),
                halign: Some(LeftCenterRight::Right),
                valign: Some(TopMiddleBottom::Bottom),
            })],
        }];

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<credit>"));
        assert!(xml.contains("<credit-symbol"));
        assert!(xml.contains("default-x=\"50\""));
        assert!(xml.contains("default-y=\"100\""));
        assert!(xml.contains("justify=\"left\""));
        assert!(xml.contains("halign=\"right\""));
        assert!(xml.contains("valign=\"bottom\""));
        assert!(xml.contains(">coda</credit-symbol>"));
    }

    #[test]
    fn test_emit_defaults_with_system_dividers() {
        use crate::ir::common::{PrintStyle, YesNo};
        use crate::ir::score::{Defaults, Divider, SystemDividers, SystemLayout};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: None,
            system_layout: Some(SystemLayout {
                system_margins: None,
                system_distance: None,
                top_system_distance: None,
                system_dividers: Some(SystemDividers {
                    left_divider: Some(Divider {
                        print_object: Some(YesNo::Yes),
                        print_style: PrintStyle::default(),
                    }),
                    right_divider: Some(Divider {
                        print_object: Some(YesNo::No),
                        print_style: PrintStyle::default(),
                    }),
                }),
            }),
            staff_layout: vec![],
            appearance: None,
            music_font: None,
            word_font: None,
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<system-layout>"));
        assert!(xml.contains("<system-dividers>"));
        assert!(xml.contains("<left-divider print-object=\"yes\"/>"));
        assert!(xml.contains("<right-divider print-object=\"no\"/>"));
        assert!(xml.contains("</system-dividers>"));
        assert!(xml.contains("</system-layout>"));
    }

    #[test]
    fn test_emit_defaults_with_lyric_fonts_and_languages() {
        use crate::ir::common::Font;
        use crate::ir::score::{Defaults, LyricFont, LyricLanguage};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: None,
            system_layout: None,
            staff_layout: vec![],
            appearance: None,
            music_font: None,
            word_font: None,
            lyric_fonts: vec![
                LyricFont {
                    number: Some("1".to_string()),
                    name: Some("verse".to_string()),
                    font: Font {
                        font_family: Some("Arial".to_string()),
                        font_style: None,
                        font_size: Some(FontSize::Points(12.0)),
                        font_weight: None,
                    },
                },
                LyricFont {
                    number: None,
                    name: None,
                    font: Font {
                        font_family: Some("Times".to_string()),
                        font_style: None,
                        font_size: None,
                        font_weight: None,
                    },
                },
            ],
            lyric_languages: vec![
                LyricLanguage {
                    number: Some("1".to_string()),
                    name: Some("verse".to_string()),
                    lang: "en".to_string(),
                },
                LyricLanguage {
                    number: None,
                    name: None,
                    lang: "de".to_string(),
                },
            ],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains(
            "<lyric-font number=\"1\" name=\"verse\" font-family=\"Arial\" font-size=\"12\"/>"
        ));
        assert!(xml.contains("<lyric-font font-family=\"Times\"/>"));
        assert!(xml.contains("<lyric-language number=\"1\" name=\"verse\" xml:lang=\"en\"/>"));
        assert!(xml.contains("<lyric-language xml:lang=\"de\"/>"));
    }

    #[test]
    fn test_emit_font_with_style_and_weight() {
        use crate::ir::common::{Font, FontStyle, FontWeight};
        use crate::ir::score::Defaults;

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: None,
            system_layout: None,
            staff_layout: vec![],
            appearance: None,
            music_font: Some(Font {
                font_family: Some("Bravura".to_string()),
                font_style: Some(FontStyle::Italic),
                font_size: Some(FontSize::Points(14.0)),
                font_weight: Some(FontWeight::Bold),
            }),
            word_font: Some(Font {
                font_family: None,
                font_style: Some(FontStyle::Normal),
                font_size: None,
                font_weight: Some(FontWeight::Normal),
            }),
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("font-style=\"italic\""));
        assert!(xml.contains("font-weight=\"bold\""));
        assert!(xml.contains("font-style=\"normal\""));
        assert!(xml.contains("font-weight=\"normal\""));
    }

    #[test]
    fn test_emit_staff_layout_empty() {
        use crate::ir::score::{Defaults, StaffLayout};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: None,
            system_layout: None,
            staff_layout: vec![
                StaffLayout {
                    number: Some(1),
                    staff_distance: None,
                },
                StaffLayout {
                    number: None,
                    staff_distance: None,
                },
            ],
            appearance: None,
            music_font: None,
            word_font: None,
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<staff-layout number=\"1\"/>"));
        assert!(xml.contains("<staff-layout/>"));
    }

    #[test]
    fn test_emit_appearance_with_distances() {
        use crate::ir::score::{Appearance, Defaults, Distance};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: None,
            system_layout: None,
            staff_layout: vec![],
            appearance: Some(Appearance {
                line_widths: vec![],
                note_sizes: vec![],
                distances: vec![
                    Distance {
                        r#type: "hyphen".to_string(),
                        value: 60.0,
                    },
                    Distance {
                        r#type: "beam".to_string(),
                        value: 7.5,
                    },
                ],
                other_appearances: vec![],
            }),
            music_font: None,
            word_font: None,
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<appearance>"));
        assert!(xml.contains("<distance type=\"hyphen\">60</distance>"));
        assert!(xml.contains("<distance type=\"beam\">7.5</distance>"));
        assert!(xml.contains("</appearance>"));
    }

    #[test]
    fn test_emit_page_layout_without_dimensions() {
        use crate::ir::score::{Defaults, PageLayout, PageMargins};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: Some(PageLayout {
                page_height: None,
                page_width: None,
                page_margins: vec![PageMargins {
                    r#type: None,
                    left: 50.0,
                    right: 50.0,
                    top: 50.0,
                    bottom: 50.0,
                }],
            }),
            system_layout: None,
            staff_layout: vec![],
            appearance: None,
            music_font: None,
            word_font: None,
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<page-layout>"));
        assert!(!xml.contains("<page-height>"));
        assert!(!xml.contains("<page-width>"));
        assert!(xml.contains("<page-margins>"));
        assert!(xml.contains("<left-margin>50</left-margin>"));
        assert!(xml.contains("</page-layout>"));
    }

    #[test]
    fn test_emit_credit_words_with_all_attributes() {
        use crate::ir::common::{LeftCenterRight, Position, TopMiddleBottom};
        use crate::ir::score::{Credit, CreditContent, CreditWords};

        let mut score = create_minimal_score();
        score.credits = vec![Credit {
            page: None,
            content: vec![CreditContent::CreditWords(CreditWords {
                value: "Test".to_string(),
                print_style: PrintStyle {
                    position: Position::default(),
                    font: crate::ir::common::Font::default(),
                    color: None,
                },
                justify: Some(LeftCenterRight::Left),
                halign: None,
                valign: Some(TopMiddleBottom::Middle),
                lang: Some("en".to_string()),
            })],
        }];

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("justify=\"left\""));
        assert!(xml.contains("valign=\"middle\""));
        assert!(xml.contains("xml:lang=\"en\""));
    }

    #[test]
    fn test_emit_typed_text_without_type() {
        use crate::ir::common::{Identification, TypedText};

        let mut score = create_minimal_score();
        score.identification = Some(Identification {
            creators: vec![TypedText {
                value: "Unknown Creator".to_string(),
                r#type: None,
            }],
            rights: vec![TypedText {
                value: "All rights reserved".to_string(),
                r#type: None,
            }],
            encoding: None,
            source: None,
            relations: vec![],
            miscellaneous: None,
        });

        let xml = emit_score(&score).unwrap();

        // Without type attribute, element should just have the value
        assert!(xml.contains("<creator>Unknown Creator</creator>"));
        assert!(xml.contains("<rights>All rights reserved</rights>"));
    }

    #[test]
    fn test_emit_credit_without_page() {
        use crate::ir::score::{Credit, CreditContent};

        let mut score = create_minimal_score();
        score.credits = vec![Credit {
            page: None,
            content: vec![CreditContent::CreditType("footer".to_string())],
        }];

        let xml = emit_score(&score).unwrap();

        // Credit without page shouldn't have page attribute
        assert!(xml.contains("<credit>"));
        assert!(!xml.contains("page="));
        assert!(xml.contains("<credit-type>footer</credit-type>"));
    }

    #[test]
    fn test_emit_note_size_types() {
        use crate::ir::score::{Appearance, Defaults, NoteSize, NoteSizeType};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: None,
            system_layout: None,
            staff_layout: vec![],
            appearance: Some(Appearance {
                line_widths: vec![],
                note_sizes: vec![
                    NoteSize {
                        r#type: NoteSizeType::Cue,
                        value: 60.0,
                    },
                    NoteSize {
                        r#type: NoteSizeType::Grace,
                        value: 70.0,
                    },
                    NoteSize {
                        r#type: NoteSizeType::GraceCue,
                        value: 50.0,
                    },
                    NoteSize {
                        r#type: NoteSizeType::Large,
                        value: 100.0,
                    },
                ],
                distances: vec![],
                other_appearances: vec![],
            }),
            music_font: None,
            word_font: None,
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<note-size type=\"cue\">60</note-size>"));
        assert!(xml.contains("<note-size type=\"grace\">70</note-size>"));
        assert!(xml.contains("<note-size type=\"grace-cue\">50</note-size>"));
        assert!(xml.contains("<note-size type=\"large\">100</note-size>"));
    }

    #[test]
    fn test_emit_margin_types() {
        use crate::ir::score::{Defaults, MarginType, PageLayout, PageMargins};

        let mut score = create_minimal_score();
        score.defaults = Some(Defaults {
            scaling: None,
            page_layout: Some(PageLayout {
                page_height: None,
                page_width: None,
                page_margins: vec![
                    PageMargins {
                        r#type: Some(MarginType::Odd),
                        left: 60.0,
                        right: 40.0,
                        top: 50.0,
                        bottom: 50.0,
                    },
                    PageMargins {
                        r#type: Some(MarginType::Even),
                        left: 40.0,
                        right: 60.0,
                        top: 50.0,
                        bottom: 50.0,
                    },
                ],
            }),
            system_layout: None,
            staff_layout: vec![],
            appearance: None,
            music_font: None,
            word_font: None,
            lyric_fonts: vec![],
            lyric_languages: vec![],
        });

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<page-margins type=\"odd\">"));
        assert!(xml.contains("<page-margins type=\"even\">"));
    }
}
