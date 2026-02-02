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
    use crate::ir::attributes::{BarStyle, Winged};
    use crate::ir::common::{
        BackwardForward, Editorial, RightLeftMiddle, StartStopDiscontinue, UprightInverted, YesNo,
    };
    use crate::ir::notation::FermataShape;

    // ==========================================================================
    // emit_barline tests
    // ==========================================================================

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
    fn test_emit_barline_no_location() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Regular),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline>"));
        assert!(!xml.contains("location="));
        assert!(xml.contains("<bar-style>regular</bar-style>"));
    }

    #[test]
    fn test_emit_barline_no_bar_style() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: None,
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline location=\"right\">"));
        assert!(!xml.contains("<bar-style>"));
    }

    #[test]
    fn test_emit_barline_location_middle() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: Some(RightLeftMiddle::Middle),
            bar_style: Some(BarStyle::Dashed),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline location=\"middle\">"));
        assert!(xml.contains("<bar-style>dashed</bar-style>"));
    }

    #[test]
    fn test_emit_barline_multiple_fermatas() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![
                Fermata {
                    shape: Some(FermataShape::Normal),
                    r#type: Some(UprightInverted::Upright),
                    print_style: PrintStyle::default(),
                },
                Fermata {
                    shape: Some(FermataShape::Normal),
                    r#type: Some(UprightInverted::Inverted),
                    print_style: PrintStyle::default(),
                },
            ],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"upright\">normal</fermata>"));
        assert!(xml.contains("<fermata type=\"inverted\">normal</fermata>"));
    }

    #[test]
    fn test_emit_barline_with_ending_and_repeat() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: Some(RightLeftMiddle::Left),
            bar_style: Some(BarStyle::HeavyLight),
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
            repeat: Some(Repeat {
                direction: BackwardForward::Forward,
                times: None,
                winged: None,
            }),
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline location=\"left\">"));
        assert!(xml.contains("<bar-style>heavy-light</bar-style>"));
        assert!(xml.contains("<ending"));
        assert!(xml.contains("<repeat direction=\"forward\"/>"));
    }

    #[test]
    fn test_emit_barline_minimal() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: None,
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<barline>"));
        assert!(xml.contains("</barline>"));
    }

    // ==========================================================================
    // emit_repeat tests
    // ==========================================================================

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
    fn test_emit_repeat_backward_with_times() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: Some(3),
            winged: None,
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<repeat direction=\"backward\" times=\"3\"/>"));
    }

    #[test]
    fn test_emit_repeat_with_winged_none() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: None,
            winged: Some(Winged::None),
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<repeat direction=\"backward\" winged=\"none\"/>"));
    }

    #[test]
    fn test_emit_repeat_with_winged_straight() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Forward,
            times: None,
            winged: Some(Winged::Straight),
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<repeat direction=\"forward\" winged=\"straight\"/>"));
    }

    #[test]
    fn test_emit_repeat_with_winged_curved() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: Some(2),
            winged: Some(Winged::Curved),
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<repeat direction=\"backward\" times=\"2\" winged=\"curved\"/>"));
    }

    #[test]
    fn test_emit_repeat_with_winged_double_straight() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Forward,
            times: None,
            winged: Some(Winged::DoubleStraight),
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("winged=\"double-straight\""));
    }

    #[test]
    fn test_emit_repeat_with_winged_double_curved() {
        let mut w = XmlWriter::new();
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: None,
            winged: Some(Winged::DoubleCurved),
        };

        emit_repeat(&mut w, &repeat).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("winged=\"double-curved\""));
    }

    // ==========================================================================
    // emit_ending tests
    // ==========================================================================

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
    fn test_emit_ending_start_with_text() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: Some("1.".to_string()),
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<ending number=\"1\" type=\"start\">1.</ending>"));
    }

    #[test]
    fn test_emit_ending_discontinue() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Discontinue,
            number: "1, 2".to_string(),
            text: None,
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<ending number=\"1, 2\" type=\"discontinue\"/>"));
    }

    #[test]
    fn test_emit_ending_with_print_object_yes() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: None,
            print_object: Some(YesNo::Yes),
            end_length: None,
            text_x: None,
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"yes\""));
    }

    #[test]
    fn test_emit_ending_with_print_object_no() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Stop,
            number: "1".to_string(),
            text: None,
            print_object: Some(YesNo::No),
            end_length: None,
            text_x: None,
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"no\""));
    }

    #[test]
    fn test_emit_ending_with_end_length() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Stop,
            number: "1".to_string(),
            text: None,
            print_object: None,
            end_length: Some(30.0),
            text_x: None,
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("end-length=\"30\""));
    }

    #[test]
    fn test_emit_ending_with_text_x() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: Some("1.".to_string()),
            print_object: None,
            end_length: None,
            text_x: Some(5.5),
            text_y: None,
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("text-x=\"5.5\""));
    }

    #[test]
    fn test_emit_ending_with_text_y() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: Some("1.".to_string()),
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: Some(-10.0),
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("text-y=\"-10\""));
    }

    #[test]
    fn test_emit_ending_with_all_optional_attrs() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "2".to_string(),
            text: Some("2.".to_string()),
            print_object: Some(YesNo::Yes),
            end_length: Some(25.0),
            text_x: Some(3.0),
            text_y: Some(-5.0),
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"2\""));
        assert!(xml.contains("type=\"start\""));
        assert!(xml.contains("print-object=\"yes\""));
        assert!(xml.contains("end-length=\"25\""));
        assert!(xml.contains("text-x=\"3\""));
        assert!(xml.contains("text-y=\"-5\""));
        assert!(xml.contains(">2.</ending>"));
    }

    #[test]
    fn test_emit_ending_empty_with_all_attrs() {
        let mut w = XmlWriter::new();
        let ending = Ending {
            r#type: StartStopDiscontinue::Stop,
            number: "1".to_string(),
            text: None,
            print_object: Some(YesNo::No),
            end_length: Some(20.0),
            text_x: Some(1.0),
            text_y: Some(-2.0),
        };

        emit_ending(&mut w, &ending).unwrap();
        let xml = w.into_string().unwrap();

        // When no text, uses empty element
        assert!(xml.contains("/>"));
        assert!(xml.contains("print-object=\"no\""));
        assert!(xml.contains("end-length=\"20\""));
    }

    // ==========================================================================
    // emit_fermata tests
    // ==========================================================================

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

    #[test]
    fn test_emit_fermata_no_type() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::Normal),
            r#type: None,
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata>normal</fermata>"));
        assert!(!xml.contains("type="));
    }

    #[test]
    fn test_emit_fermata_inverted() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::Normal),
            r#type: Some(UprightInverted::Inverted),
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"inverted\">normal</fermata>"));
    }

    #[test]
    fn test_emit_fermata_minimal() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: None,
            r#type: None,
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata/>"));
    }

    #[test]
    fn test_emit_fermata_shape_angled() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::Angled),
            r#type: Some(UprightInverted::Upright),
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"upright\">angled</fermata>"));
    }

    #[test]
    fn test_emit_fermata_shape_square() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::Square),
            r#type: None,
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata>square</fermata>"));
    }

    #[test]
    fn test_emit_fermata_shape_double_angled() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::DoubleAngled),
            r#type: Some(UprightInverted::Inverted),
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"inverted\">double-angled</fermata>"));
    }

    #[test]
    fn test_emit_fermata_shape_double_square() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::DoubleSquare),
            r#type: None,
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata>double-square</fermata>"));
    }

    #[test]
    fn test_emit_fermata_shape_double_dot() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::DoubleDot),
            r#type: Some(UprightInverted::Upright),
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"upright\">double-dot</fermata>"));
    }

    #[test]
    fn test_emit_fermata_shape_half_curve() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::HalfCurve),
            r#type: None,
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata>half-curve</fermata>"));
    }

    #[test]
    fn test_emit_fermata_shape_curlew() {
        let mut w = XmlWriter::new();
        let fermata = Fermata {
            shape: Some(FermataShape::Curlew),
            r#type: Some(UprightInverted::Inverted),
            print_style: PrintStyle::default(),
        };

        emit_fermata(&mut w, &fermata).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fermata type=\"inverted\">curlew</fermata>"));
    }

    // ==========================================================================
    // Bar style emission tests (through emit_barline)
    // ==========================================================================

    #[test]
    fn test_emit_barline_bar_style_regular() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Regular),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>regular</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_dotted() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Dotted),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>dotted</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_dashed() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Dashed),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>dashed</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_heavy() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Heavy),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>heavy</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_light_light() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::LightLight),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>light-light</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_heavy_light() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::HeavyLight),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>heavy-light</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_heavy_heavy() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::HeavyHeavy),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>heavy-heavy</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_tick() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Tick),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>tick</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_short() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::Short),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>short</bar-style>"));
    }

    #[test]
    fn test_emit_barline_bar_style_none() {
        let mut w = XmlWriter::new();
        let barline = Barline {
            location: None,
            bar_style: Some(BarStyle::None),
            editorial: Editorial::default(),
            wavy_line: None,
            segno: None,
            coda: None,
            fermatas: vec![],
            ending: None,
            repeat: None,
        };

        emit_barline(&mut w, &barline).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bar-style>none</bar-style>"));
    }
}
