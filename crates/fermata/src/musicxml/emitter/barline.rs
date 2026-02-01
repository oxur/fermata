//! Barline emission functions for MusicXML.
//!
//! This module handles the emission of barline elements including repeats,
//! endings (volta brackets), and fermatas on barlines.

use crate::ir::attributes::{Barline, Ending, Repeat};
use crate::ir::notation::Fermata;
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::helpers::{
    backward_forward_to_string, bar_style_to_string, fermata_shape_to_string,
    right_left_middle_to_string, start_stop_discontinue_to_string, upright_inverted_to_string,
    winged_to_string, yes_no_to_string,
};

/// Emit a barline element.
pub(crate) fn emit_barline(w: &mut XmlWriter, barline: &Barline) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("barline");
    if let Some(ref loc) = barline.location {
        elem = elem.attr("location", right_left_middle_to_string(loc));
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // bar-style
    if let Some(ref style) = barline.bar_style {
        w.text_element("bar-style", bar_style_to_string(style))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // editorial elements (footnote, level) - skipped for now

    // wavy-line - skipped for now

    // segno - skipped for now

    // coda - skipped for now

    // fermata*
    for fermata in &barline.fermatas {
        emit_fermata(w, fermata)?;
    }

    // ending
    if let Some(ref ending) = barline.ending {
        emit_ending(w, ending)?;
    }

    // repeat
    if let Some(ref repeat) = barline.repeat {
        emit_repeat(w, repeat)?;
    }

    w.end_element("barline")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a fermata element.
pub(crate) fn emit_fermata(w: &mut XmlWriter, fermata: &Fermata) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("fermata");
    if let Some(ref ui) = fermata.r#type {
        elem = elem.attr("type", upright_inverted_to_string(ui));
    }

    if let Some(ref shape) = fermata.shape {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(fermata_shape_to_string(shape))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("fermata")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit an ending element (volta bracket).
pub(crate) fn emit_ending(w: &mut XmlWriter, ending: &Ending) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("ending")
        .attr("number", &ending.number)
        .attr("type", start_stop_discontinue_to_string(&ending.r#type));

    if let Some(ref po) = ending.print_object {
        elem = elem.attr("print-object", yes_no_to_string(po));
    }
    if let Some(el) = ending.end_length {
        elem = elem.attr("end-length", &el.to_string());
    }
    if let Some(tx) = ending.text_x {
        elem = elem.attr("text-x", &tx.to_string());
    }
    if let Some(ty) = ending.text_y {
        elem = elem.attr("text-y", &ty.to_string());
    }

    if let Some(ref text) = ending.text {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(text)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("ending")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit a repeat element.
pub(crate) fn emit_repeat(w: &mut XmlWriter, repeat: &Repeat) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("repeat")
        .attr("direction", backward_forward_to_string(&repeat.direction));
    if let Some(times) = repeat.times {
        elem = elem.attr("times", &times.to_string());
    }
    if let Some(ref winged) = repeat.winged {
        elem = elem.attr("winged", winged_to_string(winged));
    }
    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::PrintStyle;
    use crate::ir::attributes::BarStyle;
    use crate::ir::common::{
        BackwardForward, Editorial, RightLeftMiddle, StartStopDiscontinue, UprightInverted,
    };
    use crate::ir::notation::FermataShape;

    #[test]
    fn test_emit_barline_with_repeat() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: Some(Repeat {
                direction: BackwardForward::Backward,
                times: Some(2),
                winged: None,
            }),
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline location=\"right\">"));
        assert!(xml.contains("<bar-style>light-heavy</bar-style>"));
        assert!(xml.contains("<repeat direction=\"backward\" times=\"2\"/>"));
        assert!(xml.contains("</barline>"));
    }

    #[test]
    fn test_emit_barline_with_ending() {
        let mut w = XmlWriter::new();
        let barline = Barline {
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
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline location=\"left\">"));
        assert!(xml.contains("<ending number=\"1\" type=\"start\">1.</ending>"));
        assert!(xml.contains("</barline>"));
    }

    #[test]
    fn test_emit_barline_with_fermata() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![Fermata {
                shape: Some(FermataShape::Normal),
                r#type: Some(UprightInverted::Upright),
                print_style: PrintStyle::default(),
            }],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"upright\">normal</fermata>"));
    }

    #[test]
    fn test_emit_repeat_forward() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Forward,
            times: None,
            winged: None,
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<repeat direction=\"forward\"/>"));
    }

    #[test]
    fn test_emit_ending_stop() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Stop,
            number: "1".to_string(),
            text: None,
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<ending number=\"1\" type=\"stop\"/>"));
    }

    #[test]
    fn test_emit_fermata_empty() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: None,
            r#type: Some(UprightInverted::Upright),
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"upright\"/>"));
    }
}
