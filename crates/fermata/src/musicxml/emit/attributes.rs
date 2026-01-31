//! Attributes emission functions for MusicXML.
//!
//! This module handles the emission of attributes elements including key signatures,
//! time signatures, clefs, and transpose.

use crate::ir::attributes::{
    Attributes, Cancel, Clef, Key, KeyContent, Time, TimeContent, Transpose,
};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::helpers::{
    accidental_value_to_string, cancel_location_to_string, clef_sign_to_string, mode_to_string,
    step_to_string, time_symbol_to_string, yes_no_to_string,
};

/// Emit an attributes element.
///
/// Elements are emitted in XSD order:
/// 1. divisions
/// 2. key*
/// 3. time*
/// 4. staves
/// 5. part-symbol
/// 6. instruments
/// 7. clef*
/// 8. staff-details*
/// 9. transpose*
/// 10. measure-style*
pub(crate) fn emit_attributes(w: &mut XmlWriter, attrs: &Attributes) -> Result<(), EmitError> {
    w.start_element("attributes")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Editorial elements (footnote, level) - skipped for now

    // divisions
    if let Some(div) = attrs.divisions {
        w.text_element("divisions", &div.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // key*
    for key in &attrs.keys {
        emit_key(w, key)?;
    }

    // time*
    for time in &attrs.times {
        emit_time(w, time)?;
    }

    // staves
    if let Some(staves) = attrs.staves {
        w.text_element("staves", &staves.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // part-symbol - skipped for now

    // instruments
    if let Some(instruments) = attrs.instruments {
        w.text_element("instruments", &instruments.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // clef*
    for clef in &attrs.clefs {
        emit_clef(w, clef)?;
    }

    // staff-details* - skipped for now

    // transpose*
    for transpose in &attrs.transpose {
        emit_transpose(w, transpose)?;
    }

    // measure-style* - skipped for now

    w.end_element("attributes")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a key element.
pub(crate) fn emit_key(w: &mut XmlWriter, key: &Key) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("key");
    if let Some(num) = key.number {
        elem = elem.attr("number", &num.to_string());
    }
    if let Some(ref po) = key.print_object {
        elem = elem.attr("print-object", yes_no_to_string(po));
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    match &key.content {
        KeyContent::Traditional(trad) => {
            // cancel
            if let Some(ref cancel) = trad.cancel {
                emit_cancel(w, cancel)?;
            }
            // fifths (required)
            w.text_element("fifths", &trad.fifths.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            // mode
            if let Some(ref mode) = trad.mode {
                w.text_element("mode", mode_to_string(mode))
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
        KeyContent::NonTraditional(steps) => {
            for step in steps {
                w.text_element("key-step", step_to_string(&step.step))
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
                w.text_element("key-alter", &step.alter.to_string())
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
                if let Some(ref acc) = step.accidental {
                    w.text_element("key-accidental", accidental_value_to_string(acc))
                        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
                }
            }
        }
    }

    w.end_element("key")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a cancel element.
pub(crate) fn emit_cancel(w: &mut XmlWriter, cancel: &Cancel) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("cancel");
    if let Some(ref loc) = cancel.location {
        elem = elem.attr("location", cancel_location_to_string(loc));
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&cancel.fifths.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("cancel")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a time element.
pub(crate) fn emit_time(w: &mut XmlWriter, time: &Time) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("time");
    if let Some(num) = time.number {
        elem = elem.attr("number", &num.to_string());
    }
    if let Some(ref symbol) = time.symbol {
        elem = elem.attr("symbol", time_symbol_to_string(symbol));
    }
    if let Some(ref po) = time.print_object {
        elem = elem.attr("print-object", yes_no_to_string(po));
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    match &time.content {
        TimeContent::Measured { signatures } => {
            for sig in signatures {
                w.text_element("beats", &sig.beats)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
                w.text_element("beat-type", &sig.beat_type)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
        TimeContent::SenzaMisura(text) => {
            if text.is_empty() {
                w.empty_element("senza-misura")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            } else {
                w.text_element("senza-misura", text)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
    }

    w.end_element("time")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a clef element.
pub(crate) fn emit_clef(w: &mut XmlWriter, clef: &Clef) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("clef");
    if let Some(num) = clef.number {
        elem = elem.attr("number", &num.to_string());
    }
    if let Some(ref po) = clef.print_object {
        elem = elem.attr("print-object", yes_no_to_string(po));
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // sign (required)
    w.text_element("sign", clef_sign_to_string(&clef.sign))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // line
    if let Some(line) = clef.line {
        w.text_element("line", &line.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // clef-octave-change
    if let Some(oc) = clef.octave_change {
        w.text_element("clef-octave-change", &oc.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("clef")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a transpose element.
pub(crate) fn emit_transpose(w: &mut XmlWriter, transpose: &Transpose) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("transpose");
    if let Some(num) = transpose.number {
        elem = elem.attr("number", &num.to_string());
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // diatonic
    if let Some(diatonic) = transpose.diatonic {
        w.text_element("diatonic", &diatonic.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // chromatic (required)
    w.text_element("chromatic", &transpose.chromatic.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // octave-change
    if let Some(oc) = transpose.octave_change {
        w.text_element("octave-change", &oc.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // double
    if let Some(ref double) = transpose.double {
        if *double == crate::ir::common::YesNo::Yes {
            w.empty_element("double")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
    }

    w.end_element("transpose")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::attributes::{
        ClefSign, KeyContent, Mode, TimeContent, TimeSignature, TraditionalKey,
    };
    use crate::ir::common::Editorial;

    #[test]
    fn test_emit_attributes_with_divisions_key_time_clef() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
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
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<attributes>"));
        assert!(xml.contains("<divisions>4</divisions>"));
        assert!(xml.contains("<fifths>0</fifths>"));
        assert!(xml.contains("<mode>major</mode>"));
        assert!(xml.contains("<beats>4</beats>"));
        assert!(xml.contains("<beat-type>4</beat-type>"));
        assert!(xml.contains("<sign>G</sign>"));
        assert!(xml.contains("<line>2</line>"));
        assert!(xml.contains("</attributes>"));
    }

    #[test]
    fn test_emit_key_traditional() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 2, // D major
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key>"));
        assert!(xml.contains("<fifths>2</fifths>"));
        assert!(xml.contains("<mode>major</mode>"));
        assert!(xml.contains("</key>"));
    }

    #[test]
    fn test_emit_time_4_4() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<time>"));
        assert!(xml.contains("<beats>4</beats>"));
        assert!(xml.contains("<beat-type>4</beat-type>"));
        assert!(xml.contains("</time>"));
    }

    #[test]
    fn test_emit_clef_treble() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<clef>"));
        assert!(xml.contains("<sign>G</sign>"));
        assert!(xml.contains("<line>2</line>"));
        assert!(xml.contains("</clef>"));
    }

    #[test]
    fn test_emit_clef_bass() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::F,
            line: Some(4),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>F</sign>"));
        assert!(xml.contains("<line>4</line>"));
    }

    #[test]
    fn test_emit_transpose() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: Some(0),
            chromatic: -2, // Bb instrument
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<transpose>"));
        assert!(xml.contains("<diatonic>0</diatonic>"));
        assert!(xml.contains("<chromatic>-2</chromatic>"));
        assert!(xml.contains("</transpose>"));
    }
}
