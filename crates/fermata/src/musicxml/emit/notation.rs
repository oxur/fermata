//! Notation emission functions for MusicXML.
//!
//! This module handles the emission of notation elements including tied, slur,
//! tuplet, articulations, and other note-attached notations.

use crate::ir::notation::{
    Arpeggiate, ArticulationElement, Articulations, BreathMark, Caesura, EmptyLine, NonArpeggiate,
    NotationContent, Notations, OtherNotation, Slur, StrongAccent, Tied, Tuplet, TupletPortion,
};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::barline::emit_fermata;
use super::direction::emit_dynamics;
use super::helpers::{
    above_below_to_string, accidental_value_to_string, breath_mark_value_to_string,
    caesura_value_to_string, line_length_to_string, line_shape_to_string, line_type_to_string,
    note_type_value_to_string, over_under_to_string, show_tuplet_to_string,
    start_stop_continue_to_string, start_stop_single_to_string, start_stop_to_string,
    top_bottom_to_string, up_down_to_string, yes_no_to_string,
};

/// Emit a notations element (Task 4.5).
///
/// Container for note-attached notations: tied, slur, tuplet, articulations,
/// ornaments, technical, dynamics, fermata, arpeggiate, etc.
pub(crate) fn emit_notations(w: &mut XmlWriter, notations: &Notations) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("notations");

    if let Some(ref po) = notations.print_object {
        elem = elem.attr("print-object", yes_no_to_string(po));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Emit each notation content element
    for content in &notations.content {
        match content {
            NotationContent::Tied(tied) => emit_tied(w, tied)?,
            NotationContent::Slur(slur) => emit_slur(w, slur)?,
            NotationContent::Tuplet(tuplet) => emit_tuplet(w, tuplet)?,
            NotationContent::Glissando(_glissando) => {
                // TODO: Implement glissando
            }
            NotationContent::Slide(_slide) => {
                // TODO: Implement slide
            }
            NotationContent::Ornaments(_ornaments) => {
                // TODO: Implement ornaments (Milestone 5)
            }
            NotationContent::Technical(_technical) => {
                // TODO: Implement technical (Milestone 5)
            }
            NotationContent::Articulations(articulations) => {
                emit_articulations(w, articulations)?;
            }
            NotationContent::Dynamics(dynamics) => {
                emit_dynamics(w, dynamics)?;
            }
            NotationContent::Fermata(fermata) => {
                emit_fermata(w, fermata)?;
            }
            NotationContent::Arpeggiate(arpeggiate) => {
                emit_arpeggiate(w, arpeggiate)?;
            }
            NotationContent::NonArpeggiate(non_arpeggiate) => {
                emit_non_arpeggiate(w, non_arpeggiate)?;
            }
            NotationContent::AccidentalMark(acc_mark) => {
                let mut elem = ElementBuilder::new("accidental-mark");
                if let Some(ref placement) = acc_mark.placement {
                    elem = elem.attr("placement", above_below_to_string(placement));
                }
                w.write_start(elem)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
                w.write_text(accidental_value_to_string(&acc_mark.value))
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
                w.end_element("accidental-mark")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            NotationContent::OtherNotation(other) => {
                emit_other_notation(w, other)?;
            }
        }
    }

    w.end_element("notations")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a tied element (visual tie, not playback tie).
pub(crate) fn emit_tied(w: &mut XmlWriter, tied: &Tied) -> Result<(), EmitError> {
    let mut elem =
        ElementBuilder::new("tied").attr("type", start_stop_continue_to_string(&tied.r#type));

    if let Some(number) = tied.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref line_type) = tied.line_type {
        elem = elem.attr("line-type", line_type_to_string(line_type));
    }
    if let Some(ref placement) = tied.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref orientation) = tied.orientation {
        elem = elem.attr("orientation", over_under_to_string(orientation));
    }
    if let Some(ref color) = tied.color {
        elem = elem.attr("color", color);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a slur element.
pub(crate) fn emit_slur(w: &mut XmlWriter, slur: &Slur) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("slur")
        .attr("type", start_stop_continue_to_string(&slur.r#type))
        .attr("number", &slur.number.to_string());

    if let Some(ref line_type) = slur.line_type {
        elem = elem.attr("line-type", line_type_to_string(line_type));
    }
    if let Some(ref placement) = slur.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref orientation) = slur.orientation {
        elem = elem.attr("orientation", over_under_to_string(orientation));
    }
    if let Some(ref color) = slur.color {
        elem = elem.attr("color", color);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a tuplet element.
pub(crate) fn emit_tuplet(w: &mut XmlWriter, tuplet: &Tuplet) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("tuplet").attr("type", start_stop_to_string(&tuplet.r#type));

    if let Some(number) = tuplet.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref bracket) = tuplet.bracket {
        elem = elem.attr("bracket", yes_no_to_string(bracket));
    }
    if let Some(ref show_number) = tuplet.show_number {
        elem = elem.attr("show-number", show_tuplet_to_string(show_number));
    }
    if let Some(ref show_type) = tuplet.show_type {
        elem = elem.attr("show-type", show_tuplet_to_string(show_type));
    }
    if let Some(ref line_shape) = tuplet.line_shape {
        elem = elem.attr("line-shape", line_shape_to_string(line_shape));
    }
    if let Some(ref placement) = tuplet.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    let has_content = tuplet.tuplet_actual.is_some() || tuplet.tuplet_normal.is_some();

    if has_content {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

        if let Some(ref actual) = tuplet.tuplet_actual {
            emit_tuplet_portion(w, "tuplet-actual", actual)?;
        }
        if let Some(ref normal) = tuplet.tuplet_normal {
            emit_tuplet_portion(w, "tuplet-normal", normal)?;
        }

        w.end_element("tuplet")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    Ok(())
}

/// Emit a tuplet-actual or tuplet-normal element.
pub(crate) fn emit_tuplet_portion(
    w: &mut XmlWriter,
    name: &str,
    portion: &TupletPortion,
) -> Result<(), EmitError> {
    w.start_element(name)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if let Some(ref number) = portion.tuplet_number {
        w.text_element("tuplet-number", &number.value.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    if let Some(ref tuplet_type) = portion.tuplet_type {
        w.text_element("tuplet-type", note_type_value_to_string(&tuplet_type.value))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    for _dot in &portion.tuplet_dots {
        w.empty_element("tuplet-dot")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element(name)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an arpeggiate element.
pub(crate) fn emit_arpeggiate(w: &mut XmlWriter, arpeggiate: &Arpeggiate) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("arpeggiate");

    if let Some(number) = arpeggiate.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref direction) = arpeggiate.direction {
        elem = elem.attr("direction", up_down_to_string(direction));
    }
    if let Some(ref color) = arpeggiate.color {
        elem = elem.attr("color", color);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a non-arpeggiate element.
pub(crate) fn emit_non_arpeggiate(
    w: &mut XmlWriter,
    non_arpeggiate: &NonArpeggiate,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("non-arpeggiate")
        .attr("type", top_bottom_to_string(&non_arpeggiate.r#type));

    if let Some(number) = non_arpeggiate.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref color) = non_arpeggiate.color {
        elem = elem.attr("color", color);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an other-notation element.
pub(crate) fn emit_other_notation(
    w: &mut XmlWriter,
    other: &OtherNotation,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("other-notation")
        .attr("type", start_stop_single_to_string(&other.r#type));

    if let Some(number) = other.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref po) = other.print_object {
        elem = elem.attr("print-object", yes_no_to_string(po));
    }
    if let Some(ref placement) = other.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&other.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("other-notation")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an articulations element (Task 4.6).
pub(crate) fn emit_articulations(
    w: &mut XmlWriter,
    articulations: &Articulations,
) -> Result<(), EmitError> {
    w.start_element("articulations")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for articulation in &articulations.content {
        match articulation {
            ArticulationElement::Accent(ep) => {
                emit_empty_placement(w, "accent", ep)?;
            }
            ArticulationElement::StrongAccent(sa) => {
                emit_strong_accent(w, sa)?;
            }
            ArticulationElement::Staccato(ep) => {
                emit_empty_placement(w, "staccato", ep)?;
            }
            ArticulationElement::Tenuto(ep) => {
                emit_empty_placement(w, "tenuto", ep)?;
            }
            ArticulationElement::DetachedLegato(ep) => {
                emit_empty_placement(w, "detached-legato", ep)?;
            }
            ArticulationElement::Staccatissimo(ep) => {
                emit_empty_placement(w, "staccatissimo", ep)?;
            }
            ArticulationElement::Spiccato(ep) => {
                emit_empty_placement(w, "spiccato", ep)?;
            }
            ArticulationElement::Scoop(el) => {
                emit_empty_line(w, "scoop", el)?;
            }
            ArticulationElement::Plop(el) => {
                emit_empty_line(w, "plop", el)?;
            }
            ArticulationElement::Doit(el) => {
                emit_empty_line(w, "doit", el)?;
            }
            ArticulationElement::Falloff(el) => {
                emit_empty_line(w, "falloff", el)?;
            }
            ArticulationElement::BreathMark(bm) => {
                emit_breath_mark(w, bm)?;
            }
            ArticulationElement::Caesura(c) => {
                emit_caesura(w, c)?;
            }
            ArticulationElement::Stress(ep) => {
                emit_empty_placement(w, "stress", ep)?;
            }
            ArticulationElement::Unstress(ep) => {
                emit_empty_placement(w, "unstress", ep)?;
            }
            ArticulationElement::SoftAccent(ep) => {
                emit_empty_placement(w, "soft-accent", ep)?;
            }
            ArticulationElement::OtherArticulation(oa) => {
                emit_other_articulation(w, oa)?;
            }
        }
    }

    w.end_element("articulations")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an empty-placement element.
pub(crate) fn emit_empty_placement(
    w: &mut XmlWriter,
    name: &str,
    ep: &crate::ir::common::EmptyPlacement,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref placement) = ep.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a strong-accent element.
pub(crate) fn emit_strong_accent(w: &mut XmlWriter, sa: &StrongAccent) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("strong-accent");

    if let Some(ref t) = sa.r#type {
        elem = elem.attr("type", up_down_to_string(t));
    }
    if let Some(ref placement) = sa.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an empty-line element (for jazz articulations).
pub(crate) fn emit_empty_line(
    w: &mut XmlWriter,
    name: &str,
    el: &EmptyLine,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref line_shape) = el.line_shape {
        elem = elem.attr("line-shape", line_shape_to_string(line_shape));
    }
    if let Some(ref line_type) = el.line_type {
        elem = elem.attr("line-type", line_type_to_string(line_type));
    }
    if let Some(ref line_length) = el.line_length {
        elem = elem.attr("line-length", line_length_to_string(line_length));
    }
    if let Some(ref placement) = el.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a breath-mark element.
pub(crate) fn emit_breath_mark(w: &mut XmlWriter, bm: &BreathMark) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("breath-mark");

    if let Some(ref placement) = bm.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    let value = breath_mark_value_to_string(&bm.value);
    if value.is_empty() {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(value)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("breath-mark")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit a caesura element.
pub(crate) fn emit_caesura(w: &mut XmlWriter, c: &Caesura) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("caesura");

    if let Some(ref placement) = c.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(caesura_value_to_string(&c.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("caesura")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an other-articulation element.
pub(crate) fn emit_other_articulation(
    w: &mut XmlWriter,
    oa: &crate::ir::notation::OtherArticulation,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("other-articulation");

    if let Some(ref placement) = oa.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&oa.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("other-articulation")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{AboveBelow, Editorial, LineType, Position, StartStopContinue, UpDown};
    use crate::ir::notation::{BreathMarkValue, CaesuraValue, FermataShape};

    #[test]
    fn test_emit_notations_with_tied() {
        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Tied(Tied {
                r#type: StartStopContinue::Start,
                number: Some(1),
                line_type: None,
                position: Position::default(),
                placement: Some(AboveBelow::Above),
                orientation: None,
                color: None,
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<notations>"));
        assert!(xml.contains("<tied type=\"start\" number=\"1\" placement=\"above\"/>"));
        assert!(xml.contains("</notations>"));
    }

    #[test]
    fn test_emit_notations_with_slur() {
        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Slur(Slur {
                r#type: StartStopContinue::Start,
                number: 1,
                line_type: Some(LineType::Solid),
                position: Position::default(),
                placement: Some(AboveBelow::Above),
                orientation: None,
                color: None,
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<notations>"));
        assert!(xml.contains(
            "<slur type=\"start\" number=\"1\" line-type=\"solid\" placement=\"above\"/>"
        ));
        assert!(xml.contains("</notations>"));
    }

    #[test]
    fn test_emit_notations_with_fermata() {
        use crate::ir::PrintStyle;
        use crate::ir::common::UprightInverted;
        use crate::ir::notation::Fermata;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Fermata(Fermata {
                shape: Some(FermataShape::Normal),
                r#type: Some(UprightInverted::Upright),
                print_style: PrintStyle::default(),
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<notations>"));
        assert!(xml.contains("<fermata type=\"upright\">normal</fermata>"));
        assert!(xml.contains("</notations>"));
    }

    #[test]
    fn test_emit_articulations_staccato() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::Staccato(EmptyPlacement {
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<articulations>"));
        assert!(xml.contains("<staccato placement=\"above\"/>"));
        assert!(xml.contains("</articulations>"));
    }

    #[test]
    fn test_emit_articulations_accent_tenuto() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![
                ArticulationElement::Accent(EmptyPlacement {
                    placement: Some(AboveBelow::Above),
                    position: Position::default(),
                }),
                ArticulationElement::Tenuto(EmptyPlacement {
                    placement: Some(AboveBelow::Below),
                    position: Position::default(),
                }),
            ],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<accent placement=\"above\"/>"));
        assert!(xml.contains("<tenuto placement=\"below\"/>"));
    }

    #[test]
    fn test_emit_articulations_strong_accent() {
        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::StrongAccent(StrongAccent {
                r#type: Some(UpDown::Up),
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<strong-accent type=\"up\" placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_articulations_jazz() {
        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![
                ArticulationElement::Scoop(EmptyLine::default()),
                ArticulationElement::Plop(EmptyLine::default()),
                ArticulationElement::Doit(EmptyLine::default()),
                ArticulationElement::Falloff(EmptyLine::default()),
            ],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<scoop/>"));
        assert!(xml.contains("<plop/>"));
        assert!(xml.contains("<doit/>"));
        assert!(xml.contains("<falloff/>"));
    }

    #[test]
    fn test_emit_articulations_breath_mark() {
        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::BreathMark(BreathMark {
                value: BreathMarkValue::Comma,
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<breath-mark placement=\"above\">comma</breath-mark>"));
    }

    #[test]
    fn test_emit_articulations_caesura() {
        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::Caesura(Caesura {
                value: CaesuraValue::Normal,
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<caesura placement=\"above\">normal</caesura>"));
    }

    #[test]
    fn test_emit_tied_with_all_attrs() {
        let mut w = XmlWriter::new();
        let tied = Tied {
            r#type: StartStopContinue::Start,
            number: Some(1),
            line_type: Some(LineType::Dashed),
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            orientation: Some(crate::ir::common::OverUnder::Over),
            color: Some("#FF0000".to_string()),
        };

        emit_tied(&mut w, &tied).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"start\""));
        assert!(xml.contains("number=\"1\""));
        assert!(xml.contains("line-type=\"dashed\""));
        assert!(xml.contains("placement=\"above\""));
        assert!(xml.contains("orientation=\"over\""));
        assert!(xml.contains("color=\"#FF0000\""));
    }
}
