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
        CancelLocation, ClefSign, KeyContent, KeyStep, Mode, TimeContent, TimeSignature,
        TimeSymbol, TraditionalKey,
    };
    use crate::ir::common::{AccidentalValue, Editorial, YesNo};
    use crate::ir::pitch::Step;

    // ==========================================================================
    // emit_attributes tests
    // ==========================================================================

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
    fn test_emit_attributes_without_divisions() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![],
            times: vec![],
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<attributes>"));
        assert!(xml.contains("</attributes>"));
        assert!(!xml.contains("<divisions>"));
    }

    #[test]
    fn test_emit_attributes_with_staves() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![],
            times: vec![],
            staves: Some(2),
            part_symbol: None,
            instruments: None,
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<staves>2</staves>"));
    }

    #[test]
    fn test_emit_attributes_with_instruments() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![],
            times: vec![],
            staves: None,
            part_symbol: None,
            instruments: Some(3),
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<instruments>3</instruments>"));
    }

    #[test]
    fn test_emit_attributes_with_transpose() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![],
            times: vec![],
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![Transpose {
                diatonic: None,
                chromatic: 2,
                octave_change: None,
                double: None,
                number: None,
            }],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<transpose>"));
        assert!(xml.contains("<chromatic>2</chromatic>"));
        assert!(xml.contains("</transpose>"));
    }

    #[test]
    fn test_emit_attributes_with_multiple_keys() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![
                Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 1,
                        mode: Some(Mode::Major),
                    }),
                    number: Some(1),
                    print_object: None,
                },
                Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 1,
                        mode: Some(Mode::Major),
                    }),
                    number: Some(2),
                    print_object: None,
                },
            ],
            times: vec![],
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        // Count occurrences of <key
        let key_count = xml.matches("<key").count();
        assert_eq!(key_count, 2);
    }

    #[test]
    fn test_emit_attributes_with_multiple_times() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![],
            times: vec![
                Time {
                    content: TimeContent::Measured {
                        signatures: vec![TimeSignature {
                            beats: "4".to_string(),
                            beat_type: "4".to_string(),
                        }],
                    },
                    number: Some(1),
                    symbol: None,
                    print_object: None,
                },
                Time {
                    content: TimeContent::Measured {
                        signatures: vec![TimeSignature {
                            beats: "3".to_string(),
                            beat_type: "4".to_string(),
                        }],
                    },
                    number: Some(2),
                    symbol: None,
                    print_object: None,
                },
            ],
            staves: None,
            part_symbol: None,
            instruments: None,
            clefs: vec![],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        let time_count = xml.matches("<time").count();
        assert_eq!(time_count, 2);
    }

    #[test]
    fn test_emit_attributes_with_multiple_clefs() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: None,
            keys: vec![],
            times: vec![],
            staves: Some(2),
            part_symbol: None,
            instruments: None,
            clefs: vec![
                Clef {
                    sign: ClefSign::G,
                    line: Some(2),
                    octave_change: None,
                    number: Some(1),
                    size: None,
                    print_object: None,
                },
                Clef {
                    sign: ClefSign::F,
                    line: Some(4),
                    octave_change: None,
                    number: Some(2),
                    size: None,
                    print_object: None,
                },
            ],
            staff_details: vec![],
            transpose: vec![],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        let clef_count = xml.matches("<clef").count();
        assert_eq!(clef_count, 2);
        assert!(xml.contains("<sign>G</sign>"));
        assert!(xml.contains("<sign>F</sign>"));
    }

    #[test]
    fn test_emit_attributes_full() {
        let mut w = XmlWriter::new();
        let attrs = Attributes {
            editorial: Editorial::default(),
            divisions: Some(8),
            keys: vec![Key {
                content: KeyContent::Traditional(TraditionalKey {
                    cancel: None,
                    fifths: 2,
                    mode: Some(Mode::Major),
                }),
                number: None,
                print_object: None,
            }],
            times: vec![Time {
                content: TimeContent::Measured {
                    signatures: vec![TimeSignature {
                        beats: "3".to_string(),
                        beat_type: "4".to_string(),
                    }],
                },
                number: None,
                symbol: None,
                print_object: None,
            }],
            staves: Some(2),
            part_symbol: None,
            instruments: Some(1),
            clefs: vec![Clef {
                sign: ClefSign::G,
                line: Some(2),
                octave_change: None,
                number: None,
                size: None,
                print_object: None,
            }],
            staff_details: vec![],
            transpose: vec![Transpose {
                diatonic: Some(-1),
                chromatic: -2,
                octave_change: None,
                double: None,
                number: None,
            }],
            measure_styles: vec![],
        };

        emit_attributes(&mut w, &attrs).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<divisions>8</divisions>"));
        assert!(xml.contains("<fifths>2</fifths>"));
        assert!(xml.contains("<beats>3</beats>"));
        assert!(xml.contains("<staves>2</staves>"));
        assert!(xml.contains("<instruments>1</instruments>"));
        assert!(xml.contains("<sign>G</sign>"));
        assert!(xml.contains("<chromatic>-2</chromatic>"));
    }

    // ==========================================================================
    // emit_key tests
    // ==========================================================================

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
    fn test_emit_key_with_number() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: None,
            }),
            number: Some(1),
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"1\""));
    }

    #[test]
    fn test_emit_key_with_print_object_yes() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: None,
            }),
            number: None,
            print_object: Some(YesNo::Yes),
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"yes\""));
    }

    #[test]
    fn test_emit_key_with_print_object_no() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: None,
            }),
            number: None,
            print_object: Some(YesNo::No),
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"no\""));
    }

    #[test]
    fn test_emit_key_traditional_without_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 3,
                mode: None,
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fifths>3</fifths>"));
        assert!(!xml.contains("<mode>"));
    }

    #[test]
    fn test_emit_key_traditional_with_cancel() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: Some(Cancel {
                    fifths: -2,
                    location: None,
                }),
                fifths: 3,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<cancel>"));
        assert!(xml.contains("-2"));
        assert!(xml.contains("</cancel>"));
        assert!(xml.contains("<fifths>3</fifths>"));
    }

    #[test]
    fn test_emit_key_minor_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Minor),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>minor</mode>"));
    }

    #[test]
    fn test_emit_key_dorian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Dorian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>dorian</mode>"));
    }

    #[test]
    fn test_emit_key_phrygian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Phrygian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>phrygian</mode>"));
    }

    #[test]
    fn test_emit_key_lydian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Lydian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>lydian</mode>"));
    }

    #[test]
    fn test_emit_key_mixolydian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Mixolydian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>mixolydian</mode>"));
    }

    #[test]
    fn test_emit_key_aeolian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Aeolian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>aeolian</mode>"));
    }

    #[test]
    fn test_emit_key_locrian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Locrian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>locrian</mode>"));
    }

    #[test]
    fn test_emit_key_ionian_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Ionian),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>ionian</mode>"));
    }

    #[test]
    fn test_emit_key_none_mode() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::None),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mode>none</mode>"));
    }

    #[test]
    fn test_emit_key_negative_fifths() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: -4, // Ab major
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fifths>-4</fifths>"));
    }

    #[test]
    fn test_emit_key_non_traditional_single_step() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::NonTraditional(vec![KeyStep {
                step: Step::F,
                alter: 1.0,
                accidental: None,
            }]),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key-step>F</key-step>"));
        assert!(xml.contains("<key-alter>1</key-alter>"));
        assert!(!xml.contains("<key-accidental>"));
    }

    #[test]
    fn test_emit_key_non_traditional_with_accidental() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::NonTraditional(vec![KeyStep {
                step: Step::F,
                alter: 1.0,
                accidental: Some(AccidentalValue::Sharp),
            }]),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key-step>F</key-step>"));
        assert!(xml.contains("<key-alter>1</key-alter>"));
        assert!(xml.contains("<key-accidental>sharp</key-accidental>"));
    }

    #[test]
    fn test_emit_key_non_traditional_multiple_steps() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::NonTraditional(vec![
                KeyStep {
                    step: Step::F,
                    alter: 1.0,
                    accidental: Some(AccidentalValue::Sharp),
                },
                KeyStep {
                    step: Step::C,
                    alter: 1.0,
                    accidental: Some(AccidentalValue::Sharp),
                },
                KeyStep {
                    step: Step::G,
                    alter: 1.0,
                    accidental: Some(AccidentalValue::Sharp),
                },
            ]),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key-step>F</key-step>"));
        assert!(xml.contains("<key-step>C</key-step>"));
        assert!(xml.contains("<key-step>G</key-step>"));
        // Count occurrences
        let step_count = xml.matches("<key-step>").count();
        assert_eq!(step_count, 3);
    }

    #[test]
    fn test_emit_key_non_traditional_with_flat() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::NonTraditional(vec![KeyStep {
                step: Step::B,
                alter: -1.0,
                accidental: Some(AccidentalValue::Flat),
            }]),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key-step>B</key-step>"));
        assert!(xml.contains("<key-alter>-1</key-alter>"));
        assert!(xml.contains("<key-accidental>flat</key-accidental>"));
    }

    #[test]
    fn test_emit_key_non_traditional_with_natural() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::NonTraditional(vec![KeyStep {
                step: Step::A,
                alter: 0.0,
                accidental: Some(AccidentalValue::Natural),
            }]),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key-step>A</key-step>"));
        assert!(xml.contains("<key-alter>0</key-alter>"));
        assert!(xml.contains("<key-accidental>natural</key-accidental>"));
    }

    #[test]
    fn test_emit_key_non_traditional_with_double_sharp() {
        let mut w = XmlWriter::new();
        let key = Key {
            content: KeyContent::NonTraditional(vec![KeyStep {
                step: Step::D,
                alter: 2.0,
                accidental: Some(AccidentalValue::DoubleSharp),
            }]),
            number: None,
            print_object: None,
        };

        emit_key(&mut w, &key).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<key-step>D</key-step>"));
        assert!(xml.contains("<key-alter>2</key-alter>"));
        assert!(xml.contains("<key-accidental>double-sharp</key-accidental>"));
    }

    #[test]
    fn test_emit_key_non_traditional_all_steps() {
        // Test all Step variants
        for (step, expected) in [
            (Step::A, "A"),
            (Step::B, "B"),
            (Step::C, "C"),
            (Step::D, "D"),
            (Step::E, "E"),
            (Step::F, "F"),
            (Step::G, "G"),
        ] {
            let mut w = XmlWriter::new();
            let key = Key {
                content: KeyContent::NonTraditional(vec![KeyStep {
                    step,
                    alter: 0.0,
                    accidental: None,
                }]),
                number: None,
                print_object: None,
            };

            emit_key(&mut w, &key).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!("<key-step>{}</key-step>", expected)),
                "Failed for step {:?}",
                step
            );
        }
    }

    // ==========================================================================
    // emit_cancel tests
    // ==========================================================================

    #[test]
    fn test_emit_cancel_without_location() {
        let mut w = XmlWriter::new();
        let cancel = Cancel {
            fifths: -2,
            location: None,
        };

        emit_cancel(&mut w, &cancel).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<cancel>"));
        assert!(xml.contains("-2"));
        assert!(xml.contains("</cancel>"));
        assert!(!xml.contains("location"));
    }

    #[test]
    fn test_emit_cancel_with_location_left() {
        let mut w = XmlWriter::new();
        let cancel = Cancel {
            fifths: 3,
            location: Some(CancelLocation::Left),
        };

        emit_cancel(&mut w, &cancel).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("location=\"left\""));
        assert!(xml.contains("3"));
    }

    #[test]
    fn test_emit_cancel_with_location_right() {
        let mut w = XmlWriter::new();
        let cancel = Cancel {
            fifths: 2,
            location: Some(CancelLocation::Right),
        };

        emit_cancel(&mut w, &cancel).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("location=\"right\""));
    }

    #[test]
    fn test_emit_cancel_with_location_before_barline() {
        let mut w = XmlWriter::new();
        let cancel = Cancel {
            fifths: -1,
            location: Some(CancelLocation::BeforeBarline),
        };

        emit_cancel(&mut w, &cancel).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("location=\"before-barline\""));
    }

    #[test]
    fn test_emit_cancel_positive_fifths() {
        let mut w = XmlWriter::new();
        let cancel = Cancel {
            fifths: 5,
            location: None,
        };

        emit_cancel(&mut w, &cancel).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("5"));
    }

    #[test]
    fn test_emit_cancel_zero_fifths() {
        let mut w = XmlWriter::new();
        let cancel = Cancel {
            fifths: 0,
            location: None,
        };

        emit_cancel(&mut w, &cancel).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<cancel>"));
        assert!(xml.contains("0"));
        assert!(xml.contains("</cancel>"));
    }

    // ==========================================================================
    // emit_time tests
    // ==========================================================================

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
    fn test_emit_time_3_4() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "3".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<beats>3</beats>"));
    }

    #[test]
    fn test_emit_time_6_8() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "6".to_string(),
                    beat_type: "8".to_string(),
                }],
            },
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<beats>6</beats>"));
        assert!(xml.contains("<beat-type>8</beat-type>"));
    }

    #[test]
    fn test_emit_time_with_number() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: Some(1),
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"1\""));
    }

    #[test]
    fn test_emit_time_with_symbol_common() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Common),
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("symbol=\"common\""));
    }

    #[test]
    fn test_emit_time_with_symbol_cut() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "2".to_string(),
                    beat_type: "2".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Cut),
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("symbol=\"cut\""));
    }

    #[test]
    fn test_emit_time_with_symbol_single_number() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "3".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::SingleNumber),
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("symbol=\"single-number\""));
    }

    #[test]
    fn test_emit_time_with_symbol_note() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Note),
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("symbol=\"note\""));
    }

    #[test]
    fn test_emit_time_with_symbol_dotted_note() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "6".to_string(),
                    beat_type: "8".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::DottedNote),
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("symbol=\"dotted-note\""));
    }

    #[test]
    fn test_emit_time_with_symbol_normal() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Normal),
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("symbol=\"normal\""));
    }

    #[test]
    fn test_emit_time_with_print_object_yes() {
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
            print_object: Some(YesNo::Yes),
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"yes\""));
    }

    #[test]
    fn test_emit_time_with_print_object_no() {
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
            print_object: Some(YesNo::No),
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"no\""));
    }

    #[test]
    fn test_emit_time_multiple_signatures() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![
                    TimeSignature {
                        beats: "3".to_string(),
                        beat_type: "4".to_string(),
                    },
                    TimeSignature {
                        beats: "2".to_string(),
                        beat_type: "4".to_string(),
                    },
                ],
            },
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<beats>3</beats>"));
        assert!(xml.contains("<beats>2</beats>"));
        let beats_count = xml.matches("<beats>").count();
        assert_eq!(beats_count, 2);
    }

    #[test]
    fn test_emit_time_compound_beats() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "3+2".to_string(),
                    beat_type: "8".to_string(),
                }],
            },
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<beats>3+2</beats>"));
    }

    #[test]
    fn test_emit_time_senza_misura_empty() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::SenzaMisura(String::new()),
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<senza-misura/>"));
    }

    #[test]
    fn test_emit_time_senza_misura_with_text() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::SenzaMisura("free".to_string()),
            number: None,
            symbol: None,
            print_object: None,
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<senza-misura>free</senza-misura>"));
    }

    #[test]
    fn test_emit_time_full_attributes() {
        let mut w = XmlWriter::new();
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: Some(2),
            symbol: Some(TimeSymbol::Common),
            print_object: Some(YesNo::Yes),
        };

        emit_time(&mut w, &time).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"2\""));
        assert!(xml.contains("symbol=\"common\""));
        assert!(xml.contains("print-object=\"yes\""));
    }

    // ==========================================================================
    // emit_clef tests
    // ==========================================================================

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
    fn test_emit_clef_alto() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::C,
            line: Some(3),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>C</sign>"));
        assert!(xml.contains("<line>3</line>"));
    }

    #[test]
    fn test_emit_clef_tenor() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::C,
            line: Some(4),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>C</sign>"));
        assert!(xml.contains("<line>4</line>"));
    }

    #[test]
    fn test_emit_clef_percussion() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::Percussion,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>percussion</sign>"));
        assert!(!xml.contains("<line>"));
    }

    #[test]
    fn test_emit_clef_tab() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::Tab,
            line: Some(5),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>TAB</sign>"));
    }

    #[test]
    fn test_emit_clef_jianpu() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::Jianpu,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>jianpu</sign>"));
    }

    #[test]
    fn test_emit_clef_none() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::None,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sign>none</sign>"));
    }

    #[test]
    fn test_emit_clef_with_number() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: Some(1),
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"1\""));
    }

    #[test]
    fn test_emit_clef_with_print_object_yes() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: None,
            size: None,
            print_object: Some(YesNo::Yes),
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"yes\""));
    }

    #[test]
    fn test_emit_clef_with_print_object_no() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: None,
            size: None,
            print_object: Some(YesNo::No),
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("print-object=\"no\""));
    }

    #[test]
    fn test_emit_clef_with_octave_change_up_1() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(1),
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<clef-octave-change>1</clef-octave-change>"));
    }

    #[test]
    fn test_emit_clef_with_octave_change_down_1() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(-1),
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<clef-octave-change>-1</clef-octave-change>"));
    }

    #[test]
    fn test_emit_clef_with_octave_change_up_2() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(2),
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<clef-octave-change>2</clef-octave-change>"));
    }

    #[test]
    fn test_emit_clef_with_octave_change_down_2() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(-2),
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<clef-octave-change>-2</clef-octave-change>"));
    }

    #[test]
    fn test_emit_clef_without_line() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::Percussion,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(!xml.contains("<line>"));
    }

    #[test]
    fn test_emit_clef_line_1() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(1),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<line>1</line>"));
    }

    #[test]
    fn test_emit_clef_line_5() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::F,
            line: Some(5),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<line>5</line>"));
    }

    #[test]
    fn test_emit_clef_full_attributes() {
        let mut w = XmlWriter::new();
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(-1),
            number: Some(1),
            size: None,
            print_object: Some(YesNo::Yes),
        };

        emit_clef(&mut w, &clef).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"1\""));
        assert!(xml.contains("print-object=\"yes\""));
        assert!(xml.contains("<sign>G</sign>"));
        assert!(xml.contains("<line>2</line>"));
        assert!(xml.contains("<clef-octave-change>-1</clef-octave-change>"));
    }

    // ==========================================================================
    // emit_transpose tests
    // ==========================================================================

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

    #[test]
    fn test_emit_transpose_chromatic_only() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 5,
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(!xml.contains("<diatonic>"));
        assert!(xml.contains("<chromatic>5</chromatic>"));
    }

    #[test]
    fn test_emit_transpose_with_number() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: -2,
            octave_change: None,
            double: None,
            number: Some(1),
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"1\""));
    }

    #[test]
    fn test_emit_transpose_with_diatonic() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: Some(-1),
            chromatic: -2,
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<diatonic>-1</diatonic>"));
    }

    #[test]
    fn test_emit_transpose_with_octave_change_up() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 0,
            octave_change: Some(1),
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-change>1</octave-change>"));
    }

    #[test]
    fn test_emit_transpose_with_octave_change_down() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 0,
            octave_change: Some(-1),
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-change>-1</octave-change>"));
    }

    #[test]
    fn test_emit_transpose_with_double_yes() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 0,
            octave_change: None,
            double: Some(YesNo::Yes),
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<double/>"));
    }

    #[test]
    fn test_emit_transpose_with_double_no() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 0,
            octave_change: None,
            double: Some(YesNo::No),
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        // When double is No, we don't emit the element
        assert!(!xml.contains("<double"));
    }

    #[test]
    fn test_emit_transpose_clarinet_bb() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: Some(-1),
            chromatic: -2,
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<diatonic>-1</diatonic>"));
        assert!(xml.contains("<chromatic>-2</chromatic>"));
    }

    #[test]
    fn test_emit_transpose_horn_f() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: Some(-4),
            chromatic: -7,
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<diatonic>-4</diatonic>"));
        assert!(xml.contains("<chromatic>-7</chromatic>"));
    }

    #[test]
    fn test_emit_transpose_piccolo() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 0,
            octave_change: Some(1),
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<chromatic>0</chromatic>"));
        assert!(xml.contains("<octave-change>1</octave-change>"));
    }

    #[test]
    fn test_emit_transpose_full() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: Some(-1),
            chromatic: -2,
            octave_change: Some(-1),
            double: Some(YesNo::Yes),
            number: Some(1),
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"1\""));
        assert!(xml.contains("<diatonic>-1</diatonic>"));
        assert!(xml.contains("<chromatic>-2</chromatic>"));
        assert!(xml.contains("<octave-change>-1</octave-change>"));
        assert!(xml.contains("<double/>"));
    }

    #[test]
    fn test_emit_transpose_positive_chromatic() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 3,
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<chromatic>3</chromatic>"));
    }

    #[test]
    fn test_emit_transpose_zero_chromatic() {
        let mut w = XmlWriter::new();
        let transpose = Transpose {
            diatonic: None,
            chromatic: 0,
            octave_change: None,
            double: None,
            number: None,
        };

        emit_transpose(&mut w, &transpose).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<chromatic>0</chromatic>"));
    }
}
