//! Direction emission functions for MusicXML.
//!
//! This module handles the emission of direction elements including dynamics,
//! wedges (hairpins), metronome marks, pedal markings, and other direction types.

use crate::ir::direction::{
    Dashes, Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics,
    MetronomeContent, OctaveShift, Offset, Pedal, Sound, Wedge, Words,
};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::helpers::{
    above_below_to_string, line_type_to_string, note_type_value_to_string, pedal_type_to_string,
    start_stop_continue_to_string, up_down_stop_continue_to_string, wedge_type_to_string,
    yes_no_to_string,
};

/// Emit a direction element.
///
/// Direction elements contain direction-type+ (required), offset?, voice?, staff?, sound?
pub(crate) fn emit_direction(w: &mut XmlWriter, dir: &Direction) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("direction");

    // placement attribute: above, below
    if let Some(ref placement) = dir.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    // directive attribute: yes/no
    if let Some(ref directive) = dir.directive {
        elem = elem.attr("directive", yes_no_to_string(directive));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // direction-type+ (one or more)
    for direction_type in &dir.direction_types {
        emit_direction_type(w, direction_type)?;
    }

    // offset?
    if let Some(ref offset) = dir.offset {
        emit_offset(w, offset)?;
    }

    // voice?
    if let Some(ref voice) = dir.voice {
        w.text_element("voice", voice)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // staff?
    if let Some(staff) = dir.staff {
        w.text_element("staff", &staff.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // sound?
    if let Some(ref sound) = dir.sound {
        emit_sound(w, sound)?;
    }

    w.end_element("direction")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an offset element.
pub(crate) fn emit_offset(w: &mut XmlWriter, offset: &Offset) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("offset");
    if let Some(ref sound) = offset.sound {
        elem = elem.attr("sound", yes_no_to_string(sound));
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&offset.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("offset")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a sound element.
pub(crate) fn emit_sound(w: &mut XmlWriter, sound: &Sound) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("sound");

    if let Some(tempo) = sound.tempo {
        elem = elem.attr("tempo", &tempo.to_string());
    }
    if let Some(dynamics) = sound.dynamics {
        elem = elem.attr("dynamics", &dynamics.to_string());
    }
    if let Some(ref dacapo) = sound.dacapo {
        elem = elem.attr("dacapo", yes_no_to_string(dacapo));
    }
    if let Some(ref segno) = sound.segno {
        elem = elem.attr("segno", segno);
    }
    if let Some(ref dalsegno) = sound.dalsegno {
        elem = elem.attr("dalsegno", dalsegno);
    }
    if let Some(ref coda) = sound.coda {
        elem = elem.attr("coda", coda);
    }
    if let Some(ref tocoda) = sound.tocoda {
        elem = elem.attr("tocoda", tocoda);
    }
    if let Some(divisions) = sound.divisions {
        elem = elem.attr("divisions", &divisions.to_string());
    }
    if let Some(ref forward_repeat) = sound.forward_repeat {
        elem = elem.attr("forward-repeat", yes_no_to_string(forward_repeat));
    }
    if let Some(ref fine) = sound.fine {
        elem = elem.attr("fine", fine);
    }
    if let Some(ref time_only) = sound.time_only {
        elem = elem.attr("time-only", time_only);
    }
    if let Some(ref pizzicato) = sound.pizzicato {
        elem = elem.attr("pizzicato", yes_no_to_string(pizzicato));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a direction-type element.
pub(crate) fn emit_direction_type(w: &mut XmlWriter, dt: &DirectionType) -> Result<(), EmitError> {
    w.start_element("direction-type")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    match &dt.content {
        DirectionTypeContent::Rehearsal(texts) => {
            for text in texts {
                w.text_element("rehearsal", &text.value)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
        DirectionTypeContent::Segno(segnos) => {
            for _segno in segnos {
                emit_segno(w)?;
            }
        }
        DirectionTypeContent::Coda(codas) => {
            for _coda in codas {
                emit_coda(w)?;
            }
        }
        DirectionTypeContent::Words(words_list) => {
            for words in words_list {
                emit_words(w, words)?;
            }
        }
        DirectionTypeContent::Symbol(symbols) => {
            for symbol in symbols {
                w.text_element("symbol", &symbol.value)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
        DirectionTypeContent::Wedge(wedge) => {
            emit_wedge(w, wedge)?;
        }
        DirectionTypeContent::Dynamics(dynamics) => {
            emit_dynamics(w, dynamics)?;
        }
        DirectionTypeContent::Dashes(dashes) => {
            emit_dashes(w, dashes)?;
        }
        DirectionTypeContent::Bracket(_bracket) => {
            // TODO: Implement bracket emission
        }
        DirectionTypeContent::Pedal(pedal) => {
            emit_pedal(w, pedal)?;
        }
        DirectionTypeContent::Metronome(metronome) => {
            emit_metronome(w, metronome)?;
        }
        DirectionTypeContent::OctaveShift(octave_shift) => {
            emit_octave_shift(w, octave_shift)?;
        }
        DirectionTypeContent::HarpPedals(_) => {
            // TODO: Implement harp-pedals emission
        }
        DirectionTypeContent::Damp(_) => {
            w.empty_element("damp")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        DirectionTypeContent::DampAll(_) => {
            w.empty_element("damp-all")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        DirectionTypeContent::Eyeglasses(_) => {
            w.empty_element("eyeglasses")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        DirectionTypeContent::StringMute(_) => {
            // TODO: Implement string-mute emission
        }
        DirectionTypeContent::Scordatura(_) => {
            // TODO: Implement scordatura emission
        }
        DirectionTypeContent::Image(_) => {
            // TODO: Implement image emission
        }
        DirectionTypeContent::PrincipalVoice(_) => {
            // TODO: Implement principal-voice emission
        }
        DirectionTypeContent::Percussion(_) => {
            // TODO: Implement percussion emission
        }
        DirectionTypeContent::AccordionRegistration(_) => {
            // TODO: Implement accordion-registration emission
        }
        DirectionTypeContent::StaffDivide(_) => {
            // TODO: Implement staff-divide emission
        }
        DirectionTypeContent::OtherDirection(other) => {
            let mut elem = ElementBuilder::new("other-direction");
            if let Some(ref po) = other.print_object {
                elem = elem.attr("print-object", yes_no_to_string(po));
            }
            w.write_start(elem)
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            w.write_text(&other.value)
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            w.end_element("other-direction")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
    }

    w.end_element("direction-type")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a segno element.
pub(crate) fn emit_segno(w: &mut XmlWriter) -> Result<(), EmitError> {
    w.empty_element("segno")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a coda element.
pub(crate) fn emit_coda(w: &mut XmlWriter) -> Result<(), EmitError> {
    w.empty_element("coda")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a words element.
pub(crate) fn emit_words(w: &mut XmlWriter, words: &Words) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("words");
    if let Some(ref lang) = words.lang {
        elem = elem.attr("xml:lang", lang);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&words.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("words")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a dynamics element (Task 4.2).
///
/// Contains empty elements for standard dynamics and other-dynamics for custom.
pub(crate) fn emit_dynamics(w: &mut XmlWriter, dynamics: &Dynamics) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("dynamics");

    if let Some(ref placement) = dynamics.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for dynamic in &dynamics.content {
        match dynamic {
            DynamicElement::P => {
                w.empty_element("p")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::PP => {
                w.empty_element("pp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::PPP => {
                w.empty_element("ppp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::PPPP => {
                w.empty_element("pppp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::PPPPP => {
                w.empty_element("ppppp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::PPPPPP => {
                w.empty_element("pppppp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::F => {
                w.empty_element("f")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FF => {
                w.empty_element("ff")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FFF => {
                w.empty_element("fff")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FFFF => {
                w.empty_element("ffff")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FFFFF => {
                w.empty_element("fffff")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FFFFFF => {
                w.empty_element("ffffff")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::MP => {
                w.empty_element("mp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::MF => {
                w.empty_element("mf")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::SF => {
                w.empty_element("sf")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::SFP => {
                w.empty_element("sfp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::SFPP => {
                w.empty_element("sfpp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FP => {
                w.empty_element("fp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::RF => {
                w.empty_element("rf")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::RFZ => {
                w.empty_element("rfz")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::SFZ => {
                w.empty_element("sfz")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::SFFZ => {
                w.empty_element("sffz")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::FZ => {
                w.empty_element("fz")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::N => {
                w.empty_element("n")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::PF => {
                w.empty_element("pf")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::SFZP => {
                w.empty_element("sfzp")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
            DynamicElement::OtherDynamics(text) => {
                w.text_element("other-dynamics", text)
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
    }

    w.end_element("dynamics")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a wedge element (Task 4.3).
///
/// Hairpin dynamics: crescendo, diminuendo, stop, continue.
pub(crate) fn emit_wedge(w: &mut XmlWriter, wedge: &Wedge) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("wedge").attr("type", wedge_type_to_string(&wedge.r#type));

    if let Some(number) = wedge.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(spread) = wedge.spread {
        elem = elem.attr("spread", &spread.to_string());
    }
    if let Some(ref niente) = wedge.niente {
        elem = elem.attr("niente", yes_no_to_string(niente));
    }
    if let Some(ref line_type) = wedge.line_type {
        elem = elem.attr("line-type", line_type_to_string(line_type));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a dashes element.
pub(crate) fn emit_dashes(w: &mut XmlWriter, dashes: &Dashes) -> Result<(), EmitError> {
    let mut elem =
        ElementBuilder::new("dashes").attr("type", start_stop_continue_to_string(&dashes.r#type));

    if let Some(number) = dashes.number {
        elem = elem.attr("number", &number.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a pedal element.
pub(crate) fn emit_pedal(w: &mut XmlWriter, pedal: &Pedal) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("pedal").attr("type", pedal_type_to_string(&pedal.r#type));

    if let Some(number) = pedal.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref line) = pedal.line {
        elem = elem.attr("line", yes_no_to_string(line));
    }
    if let Some(ref sign) = pedal.sign {
        elem = elem.attr("sign", yes_no_to_string(sign));
    }
    if let Some(ref abbreviated) = pedal.abbreviated {
        elem = elem.attr("abbreviated", yes_no_to_string(abbreviated));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a metronome element (Task 4.4).
pub(crate) fn emit_metronome(
    w: &mut XmlWriter,
    metronome: &crate::ir::direction::Metronome,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("metronome");

    if let Some(ref parentheses) = metronome.parentheses {
        elem = elem.attr("parentheses", yes_no_to_string(parentheses));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    match &metronome.content {
        MetronomeContent::PerMinute {
            beat_unit,
            beat_unit_dots,
            per_minute,
        } => {
            // beat-unit
            w.text_element("beat-unit", note_type_value_to_string(beat_unit))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

            // beat-unit-dot*
            for _ in 0..*beat_unit_dots {
                w.empty_element("beat-unit-dot")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }

            // per-minute
            w.text_element("per-minute", &per_minute.value)
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        MetronomeContent::BeatEquation {
            left_unit,
            left_dots,
            right_unit,
            right_dots,
        } => {
            // Left beat unit
            w.text_element("beat-unit", note_type_value_to_string(left_unit))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            for _ in 0..*left_dots {
                w.empty_element("beat-unit-dot")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }

            // Right beat unit
            w.text_element("beat-unit", note_type_value_to_string(right_unit))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            for _ in 0..*right_dots {
                w.empty_element("beat-unit-dot")
                    .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            }
        }
        MetronomeContent::MetricModulation { .. } => {
            // TODO: Implement metric modulation
        }
    }

    w.end_element("metronome")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an octave-shift element.
pub(crate) fn emit_octave_shift(
    w: &mut XmlWriter,
    octave_shift: &OctaveShift,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("octave-shift").attr(
        "type",
        up_down_stop_continue_to_string(&octave_shift.r#type),
    );

    if let Some(number) = octave_shift.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(size) = octave_shift.size {
        elem = elem.attr("size", &size.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::NoteTypeValue;
    use crate::ir::PrintStyle;
    use crate::ir::common::{
        AboveBelow, Font, FormattedText, LineType, Position, StartStopContinue, YesNo,
    };
    use crate::ir::direction::{
        Coda, EmptyPrintStyle, FormattedSymbol, MetricRelation, Metronome, OtherDirection,
        PedalType, PerMinute, Segno, UpDownStopContinue, WedgeType,
    };

    // ==================== emit_direction Tests ====================

    #[test]
    fn test_emit_direction_basic() {
        let mut w = XmlWriter::new();
        let dir = Direction {
            placement: Some(AboveBelow::Above),
            directive: None,
            direction_types: vec![DirectionType {
                content: DirectionTypeContent::Words(vec![Words {
                    value: "cresc.".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    lang: None,
                }]),
            }],
            offset: None,
            voice: None,
            staff: None,
            sound: None,
        };

        emit_direction(&mut w, &dir).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<direction placement=\"above\">"));
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("<words>cresc.</words>"));
        assert!(xml.contains("</direction-type>"));
        assert!(xml.contains("</direction>"));
    }

    #[test]
    fn test_emit_direction_with_voice_and_staff() {
        let mut w = XmlWriter::new();
        let dir = Direction {
            placement: Some(AboveBelow::Below),
            directive: Some(YesNo::Yes),
            direction_types: vec![DirectionType {
                content: DirectionTypeContent::Words(vec![Words {
                    value: "dolce".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    lang: Some("it".to_string()),
                }]),
            }],
            offset: None,
            voice: Some("1".to_string()),
            staff: Some(1),
            sound: None,
        };

        emit_direction(&mut w, &dir).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<direction placement=\"below\" directive=\"yes\">"));
        assert!(xml.contains("<voice>1</voice>"));
        assert!(xml.contains("<staff>1</staff>"));
    }

    #[test]
    fn test_emit_direction_with_sound() {
        let mut w = XmlWriter::new();
        let dir = Direction {
            placement: Some(AboveBelow::Above),
            directive: None,
            direction_types: vec![DirectionType {
                content: DirectionTypeContent::Words(vec![Words {
                    value: "Allegro".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    lang: None,
                }]),
            }],
            offset: None,
            voice: None,
            staff: None,
            sound: Some(Sound {
                tempo: Some(120.0),
                dynamics: Some(80.0),
                ..Default::default()
            }),
        };

        emit_direction(&mut w, &dir).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sound tempo=\"120\" dynamics=\"80\"/>"));
    }

    #[test]
    fn test_emit_direction_with_offset() {
        let mut w = XmlWriter::new();
        let dir = Direction {
            placement: None,
            directive: None,
            direction_types: vec![DirectionType {
                content: DirectionTypeContent::Words(vec![Words {
                    value: "test".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    lang: None,
                }]),
            }],
            offset: Some(Offset {
                value: 10,
                sound: Some(YesNo::Yes),
            }),
            voice: None,
            staff: None,
            sound: None,
        };

        emit_direction(&mut w, &dir).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<offset sound=\"yes\">10</offset>"));
    }

    #[test]
    fn test_emit_direction_no_placement_no_directive() {
        let mut w = XmlWriter::new();
        let dir = Direction {
            placement: None,
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
        };

        emit_direction(&mut w, &dir).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<direction>"));
        assert!(!xml.contains("placement="));
        assert!(!xml.contains("directive="));
    }

    #[test]
    fn test_emit_direction_multiple_direction_types() {
        let mut w = XmlWriter::new();
        let dir = Direction {
            placement: Some(AboveBelow::Above),
            directive: None,
            direction_types: vec![
                DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![DynamicElement::F],
                        print_style: PrintStyle::default(),
                        placement: None,
                    }),
                },
                DirectionType {
                    content: DirectionTypeContent::Words(vec![Words {
                        value: "espr.".to_string(),
                        print_style: PrintStyle::default(),
                        justify: None,
                        lang: None,
                    }]),
                },
            ],
            offset: None,
            voice: None,
            staff: None,
            sound: None,
        };

        emit_direction(&mut w, &dir).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<f/>"));
        assert!(xml.contains("<words>espr.</words>"));
    }

    // ==================== emit_offset Tests ====================

    #[test]
    fn test_emit_offset_basic() {
        let mut w = XmlWriter::new();
        let offset = Offset {
            value: 5,
            sound: None,
        };

        emit_offset(&mut w, &offset).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<offset>5</offset>"));
    }

    #[test]
    fn test_emit_offset_with_sound_yes() {
        let mut w = XmlWriter::new();
        let offset = Offset {
            value: -10,
            sound: Some(YesNo::Yes),
        };

        emit_offset(&mut w, &offset).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<offset sound=\"yes\">-10</offset>"));
    }

    #[test]
    fn test_emit_offset_with_sound_no() {
        let mut w = XmlWriter::new();
        let offset = Offset {
            value: 0,
            sound: Some(YesNo::No),
        };

        emit_offset(&mut w, &offset).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<offset sound=\"no\">0</offset>"));
    }

    // ==================== emit_sound Tests ====================

    #[test]
    fn test_emit_sound_basic_tempo_dynamics() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            tempo: Some(120.0),
            dynamics: Some(80.0),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sound tempo=\"120\" dynamics=\"80\"/>"));
    }

    #[test]
    fn test_emit_sound_dacapo() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            dacapo: Some(YesNo::Yes),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("dacapo=\"yes\""));
    }

    #[test]
    fn test_emit_sound_segno() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            segno: Some("segno1".to_string()),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("segno=\"segno1\""));
    }

    #[test]
    fn test_emit_sound_dalsegno() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            dalsegno: Some("segno1".to_string()),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("dalsegno=\"segno1\""));
    }

    #[test]
    fn test_emit_sound_coda() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            coda: Some("coda1".to_string()),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("coda=\"coda1\""));
    }

    #[test]
    fn test_emit_sound_tocoda() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            tocoda: Some("coda1".to_string()),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("tocoda=\"coda1\""));
    }

    #[test]
    fn test_emit_sound_divisions() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            divisions: Some(4),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("divisions=\"4\""));
    }

    #[test]
    fn test_emit_sound_forward_repeat() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            forward_repeat: Some(YesNo::Yes),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("forward-repeat=\"yes\""));
    }

    #[test]
    fn test_emit_sound_fine() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            fine: Some("yes".to_string()),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("fine=\"yes\""));
    }

    #[test]
    fn test_emit_sound_time_only() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            time_only: Some("1".to_string()),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("time-only=\"1\""));
    }

    #[test]
    fn test_emit_sound_pizzicato() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            pizzicato: Some(YesNo::Yes),
            ..Default::default()
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("pizzicato=\"yes\""));
    }

    #[test]
    fn test_emit_sound_all_attributes() {
        let mut w = XmlWriter::new();
        let sound = Sound {
            tempo: Some(100.0),
            dynamics: Some(90.0),
            dacapo: Some(YesNo::Yes),
            segno: Some("s1".to_string()),
            dalsegno: Some("s1".to_string()),
            coda: Some("c1".to_string()),
            tocoda: Some("c1".to_string()),
            divisions: Some(2),
            forward_repeat: Some(YesNo::No),
            fine: Some("yes".to_string()),
            time_only: Some("2".to_string()),
            pizzicato: Some(YesNo::No),
        };

        emit_sound(&mut w, &sound).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("tempo=\"100\""));
        assert!(xml.contains("dynamics=\"90\""));
        assert!(xml.contains("dacapo=\"yes\""));
        assert!(xml.contains("segno=\"s1\""));
        assert!(xml.contains("dalsegno=\"s1\""));
        assert!(xml.contains("coda=\"c1\""));
        assert!(xml.contains("tocoda=\"c1\""));
        assert!(xml.contains("divisions=\"2\""));
        assert!(xml.contains("forward-repeat=\"no\""));
        assert!(xml.contains("fine=\"yes\""));
        assert!(xml.contains("time-only=\"2\""));
        assert!(xml.contains("pizzicato=\"no\""));
    }

    // ==================== emit_direction_type Tests ====================

    #[test]
    fn test_emit_direction_type_rehearsal() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Rehearsal(vec![FormattedText {
                value: "A".to_string(),
                print_style: PrintStyle::default(),
                lang: None,
            }]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("<rehearsal>A</rehearsal>"));
        assert!(xml.contains("</direction-type>"));
    }

    #[test]
    fn test_emit_direction_type_rehearsal_multiple() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Rehearsal(vec![
                FormattedText {
                    value: "A".to_string(),
                    print_style: PrintStyle::default(),
                    lang: None,
                },
                FormattedText {
                    value: "B".to_string(),
                    print_style: PrintStyle::default(),
                    lang: None,
                },
            ]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<rehearsal>A</rehearsal>"));
        assert!(xml.contains("<rehearsal>B</rehearsal>"));
    }

    #[test]
    fn test_emit_direction_type_segno() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Segno(vec![Segno::default()]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<segno/>"));
    }

    #[test]
    fn test_emit_direction_type_segno_multiple() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Segno(vec![Segno::default(), Segno::default()]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // Should contain two segno elements
        let count = xml.matches("<segno/>").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_emit_direction_type_coda() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Coda(vec![Coda::default()]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<coda/>"));
    }

    #[test]
    fn test_emit_direction_type_coda_multiple() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Coda(vec![Coda::default(), Coda::default()]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        let count = xml.matches("<coda/>").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_emit_direction_type_words_single() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Words(vec![Words {
                value: "rit.".to_string(),
                print_style: PrintStyle::default(),
                justify: None,
                lang: None,
            }]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<words>rit.</words>"));
    }

    #[test]
    fn test_emit_direction_type_words_multiple() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Words(vec![
                Words {
                    value: "rit.".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    lang: None,
                },
                Words {
                    value: "e dim.".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    lang: Some("it".to_string()),
                },
            ]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<words>rit.</words>"));
        assert!(xml.contains("<words xml:lang=\"it\">e dim.</words>"));
    }

    #[test]
    fn test_emit_direction_type_symbol() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Symbol(vec![FormattedSymbol {
                value: "\u{1D10F}".to_string(), // musical symbol
                print_style: PrintStyle::default(),
                justify: None,
            }]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<symbol>"));
        assert!(xml.contains("</symbol>"));
    }

    #[test]
    fn test_emit_direction_type_symbol_multiple() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Symbol(vec![
                FormattedSymbol {
                    value: "sym1".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                },
                FormattedSymbol {
                    value: "sym2".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                },
            ]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<symbol>sym1</symbol>"));
        assert!(xml.contains("<symbol>sym2</symbol>"));
    }

    #[test]
    fn test_emit_direction_type_wedge() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Wedge(Wedge {
                r#type: WedgeType::Crescendo,
                number: None,
                spread: None,
                niente: None,
                line_type: None,
                position: Position::default(),
                color: None,
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wedge type=\"crescendo\"/>"));
    }

    #[test]
    fn test_emit_direction_type_dynamics() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Dynamics(Dynamics {
                content: vec![DynamicElement::FF],
                print_style: PrintStyle::default(),
                placement: Some(AboveBelow::Below),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dynamics placement=\"below\">"));
        assert!(xml.contains("<ff/>"));
    }

    #[test]
    fn test_emit_direction_type_dashes() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Dashes(Dashes {
                r#type: StartStopContinue::Start,
                number: Some(1),
                position: Position::default(),
                color: None,
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dashes type=\"start\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_direction_type_pedal() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Pedal(Pedal {
                r#type: PedalType::Start,
                number: None,
                line: Some(YesNo::Yes),
                sign: Some(YesNo::Yes),
                abbreviated: None,
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"start\" line=\"yes\" sign=\"yes\"/>"));
    }

    #[test]
    fn test_emit_direction_type_metronome() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Metronome(Metronome {
                parentheses: None,
                content: MetronomeContent::PerMinute {
                    beat_unit: NoteTypeValue::Quarter,
                    beat_unit_dots: 0,
                    per_minute: PerMinute {
                        value: "120".to_string(),
                        font: Font::default(),
                    },
                },
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<metronome>"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
    }

    #[test]
    fn test_emit_direction_type_octave_shift() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::OctaveShift(OctaveShift {
                r#type: UpDownStopContinue::Up,
                number: None,
                size: Some(8),
                position: Position::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-shift type=\"up\" size=\"8\"/>"));
    }

    #[test]
    fn test_emit_direction_type_damp() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Damp(EmptyPrintStyle {
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<damp/>"));
    }

    #[test]
    fn test_emit_direction_type_damp_all() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::DampAll(EmptyPrintStyle {
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<damp-all/>"));
    }

    #[test]
    fn test_emit_direction_type_eyeglasses() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Eyeglasses(EmptyPrintStyle {
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<eyeglasses/>"));
    }

    #[test]
    fn test_emit_direction_type_other_direction() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::OtherDirection(OtherDirection {
                value: "custom direction".to_string(),
                print_object: Some(YesNo::Yes),
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(
            xml.contains(
                "<other-direction print-object=\"yes\">custom direction</other-direction>"
            )
        );
    }

    #[test]
    fn test_emit_direction_type_other_direction_no_print_object() {
        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::OtherDirection(OtherDirection {
                value: "custom".to_string(),
                print_object: None,
                print_style: PrintStyle::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<other-direction>custom</other-direction>"));
    }

    // ==================== emit_segno Tests ====================

    #[test]
    fn test_emit_segno() {
        let mut w = XmlWriter::new();
        emit_segno(&mut w).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<segno/>"));
    }

    // ==================== emit_coda Tests ====================

    #[test]
    fn test_emit_coda() {
        let mut w = XmlWriter::new();
        emit_coda(&mut w).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<coda/>"));
    }

    // ==================== emit_words Tests ====================

    #[test]
    fn test_emit_words_basic() {
        let mut w = XmlWriter::new();
        let words = Words {
            value: "crescendo".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            lang: None,
        };

        emit_words(&mut w, &words).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<words>crescendo</words>"));
    }

    #[test]
    fn test_emit_words_with_lang() {
        let mut w = XmlWriter::new();
        let words = Words {
            value: "dolce".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            lang: Some("it".to_string()),
        };

        emit_words(&mut w, &words).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<words xml:lang=\"it\">dolce</words>"));
    }

    #[test]
    fn test_emit_words_with_lang_en() {
        let mut w = XmlWriter::new();
        let words = Words {
            value: "gradually louder".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            lang: Some("en".to_string()),
        };

        emit_words(&mut w, &words).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<words xml:lang=\"en\">gradually louder</words>"));
    }

    // ==================== emit_dynamics Tests ====================

    #[test]
    fn test_emit_dynamics_forte() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![DynamicElement::F],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dynamics>"));
        assert!(xml.contains("<f/>"));
        assert!(xml.contains("</dynamics>"));
    }

    #[test]
    fn test_emit_dynamics_all_piano_levels() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![
                DynamicElement::P,
                DynamicElement::PP,
                DynamicElement::PPP,
                DynamicElement::PPPP,
                DynamicElement::PPPPP,
                DynamicElement::PPPPPP,
            ],
            print_style: PrintStyle::default(),
            placement: Some(AboveBelow::Below),
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<p/>"));
        assert!(xml.contains("<pp/>"));
        assert!(xml.contains("<ppp/>"));
        assert!(xml.contains("<pppp/>"));
        assert!(xml.contains("<ppppp/>"));
        assert!(xml.contains("<pppppp/>"));
    }

    #[test]
    fn test_emit_dynamics_all_forte_levels() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![
                DynamicElement::F,
                DynamicElement::FF,
                DynamicElement::FFF,
                DynamicElement::FFFF,
                DynamicElement::FFFFF,
                DynamicElement::FFFFFF,
            ],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<f/>"));
        assert!(xml.contains("<ff/>"));
        assert!(xml.contains("<fff/>"));
        assert!(xml.contains("<ffff/>"));
        assert!(xml.contains("<fffff/>"));
        assert!(xml.contains("<ffffff/>"));
    }

    #[test]
    fn test_emit_dynamics_mezzo() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![DynamicElement::MP, DynamicElement::MF],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mp/>"));
        assert!(xml.contains("<mf/>"));
    }

    #[test]
    fn test_emit_dynamics_sforzando_variants() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![
                DynamicElement::SF,
                DynamicElement::SFP,
                DynamicElement::SFPP,
                DynamicElement::SFZ,
                DynamicElement::SFFZ,
                DynamicElement::SFZP,
            ],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<sf/>"));
        assert!(xml.contains("<sfp/>"));
        assert!(xml.contains("<sfpp/>"));
        assert!(xml.contains("<sfz/>"));
        assert!(xml.contains("<sffz/>"));
        assert!(xml.contains("<sfzp/>"));
    }

    #[test]
    fn test_emit_dynamics_special_variants() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![
                DynamicElement::FP,
                DynamicElement::RF,
                DynamicElement::RFZ,
                DynamicElement::FZ,
                DynamicElement::N,
                DynamicElement::PF,
            ],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fp/>"));
        assert!(xml.contains("<rf/>"));
        assert!(xml.contains("<rfz/>"));
        assert!(xml.contains("<fz/>"));
        assert!(xml.contains("<n/>"));
        assert!(xml.contains("<pf/>"));
    }

    #[test]
    fn test_emit_dynamics_other() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![DynamicElement::OtherDynamics("custom".to_string())],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<other-dynamics>custom</other-dynamics>"));
    }

    #[test]
    fn test_emit_dynamics_with_placement_above() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![DynamicElement::F],
            print_style: PrintStyle::default(),
            placement: Some(AboveBelow::Above),
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dynamics placement=\"above\">"));
    }

    #[test]
    fn test_emit_dynamics_empty_content() {
        let mut w = XmlWriter::new();
        let dynamics = Dynamics {
            content: vec![],
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_dynamics(&mut w, &dynamics).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dynamics>"));
        assert!(xml.contains("</dynamics>"));
    }

    // ==================== emit_wedge Tests ====================

    #[test]
    fn test_emit_wedge_crescendo() {
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: Some(1),
            spread: Some(15.0),
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };

        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wedge type=\"crescendo\" number=\"1\" spread=\"15\"/>"));
    }

    #[test]
    fn test_emit_wedge_diminuendo_with_niente() {
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Diminuendo,
            number: Some(1),
            spread: None,
            niente: Some(YesNo::Yes),
            line_type: Some(LineType::Dashed),
            position: Position::default(),
            color: None,
        };

        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"diminuendo\""));
        assert!(xml.contains("niente=\"yes\""));
        assert!(xml.contains("line-type=\"dashed\""));
    }

    #[test]
    fn test_emit_wedge_stop() {
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Stop,
            number: Some(1),
            spread: None,
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };

        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wedge type=\"stop\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_wedge_continue() {
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Continue,
            number: Some(2),
            spread: None,
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };

        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wedge type=\"continue\" number=\"2\"/>"));
    }

    #[test]
    fn test_emit_wedge_minimal() {
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: None,
            spread: None,
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };

        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wedge type=\"crescendo\"/>"));
    }

    #[test]
    fn test_emit_wedge_all_line_types() {
        // Test solid
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: None,
            spread: None,
            niente: None,
            line_type: Some(LineType::Solid),
            position: Position::default(),
            color: None,
        };
        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();
        assert!(xml.contains("line-type=\"solid\""));

        // Test dotted
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: None,
            spread: None,
            niente: None,
            line_type: Some(LineType::Dotted),
            position: Position::default(),
            color: None,
        };
        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();
        assert!(xml.contains("line-type=\"dotted\""));

        // Test wavy
        let mut w = XmlWriter::new();
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: None,
            spread: None,
            niente: None,
            line_type: Some(LineType::Wavy),
            position: Position::default(),
            color: None,
        };
        emit_wedge(&mut w, &wedge).unwrap();
        let xml = w.into_string().unwrap();
        assert!(xml.contains("line-type=\"wavy\""));
    }

    // ==================== emit_dashes Tests ====================

    #[test]
    fn test_emit_dashes_start() {
        let mut w = XmlWriter::new();
        let dashes = Dashes {
            r#type: StartStopContinue::Start,
            number: Some(1),
            position: Position::default(),
            color: None,
        };

        emit_dashes(&mut w, &dashes).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dashes type=\"start\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_dashes_stop() {
        let mut w = XmlWriter::new();
        let dashes = Dashes {
            r#type: StartStopContinue::Stop,
            number: Some(1),
            position: Position::default(),
            color: None,
        };

        emit_dashes(&mut w, &dashes).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dashes type=\"stop\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_dashes_continue() {
        let mut w = XmlWriter::new();
        let dashes = Dashes {
            r#type: StartStopContinue::Continue,
            number: Some(2),
            position: Position::default(),
            color: None,
        };

        emit_dashes(&mut w, &dashes).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dashes type=\"continue\" number=\"2\"/>"));
    }

    #[test]
    fn test_emit_dashes_no_number() {
        let mut w = XmlWriter::new();
        let dashes = Dashes {
            r#type: StartStopContinue::Start,
            number: None,
            position: Position::default(),
            color: None,
        };

        emit_dashes(&mut w, &dashes).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dashes type=\"start\"/>"));
    }

    // ==================== emit_pedal Tests ====================

    #[test]
    fn test_emit_pedal_start() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"start\"/>"));
    }

    #[test]
    fn test_emit_pedal_stop() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Stop,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"stop\"/>"));
    }

    #[test]
    fn test_emit_pedal_sostenuto() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Sostenuto,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"sostenuto\"/>"));
    }

    #[test]
    fn test_emit_pedal_change() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Change,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"change\"/>"));
    }

    #[test]
    fn test_emit_pedal_continue() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Continue,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"continue\"/>"));
    }

    #[test]
    fn test_emit_pedal_discontinue() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Discontinue,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"discontinue\"/>"));
    }

    #[test]
    fn test_emit_pedal_resume() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Resume,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"resume\"/>"));
    }

    #[test]
    fn test_emit_pedal_with_number() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: Some(1),
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pedal type=\"start\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_pedal_with_line() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: Some(YesNo::Yes),
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("line=\"yes\""));
    }

    #[test]
    fn test_emit_pedal_with_sign() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: None,
            sign: Some(YesNo::Yes),
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("sign=\"yes\""));
    }

    #[test]
    fn test_emit_pedal_with_abbreviated() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: None,
            sign: None,
            abbreviated: Some(YesNo::Yes),
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("abbreviated=\"yes\""));
    }

    #[test]
    fn test_emit_pedal_all_attributes() {
        let mut w = XmlWriter::new();
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: Some(1),
            line: Some(YesNo::Yes),
            sign: Some(YesNo::Yes),
            abbreviated: Some(YesNo::No),
            print_style: PrintStyle::default(),
        };

        emit_pedal(&mut w, &pedal).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"start\""));
        assert!(xml.contains("number=\"1\""));
        assert!(xml.contains("line=\"yes\""));
        assert!(xml.contains("sign=\"yes\""));
        assert!(xml.contains("abbreviated=\"no\""));
    }

    // ==================== emit_metronome Tests ====================

    #[test]
    fn test_emit_metronome_per_minute() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Quarter,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "120".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<metronome>"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<per-minute>120</per-minute>"));
        assert!(xml.contains("</metronome>"));
    }

    #[test]
    fn test_emit_metronome_with_dots() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: Some(YesNo::Yes),
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Quarter,
                beat_unit_dots: 1,
                per_minute: PerMinute {
                    value: "80".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<metronome parentheses=\"yes\">"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<beat-unit-dot/>"));
        assert!(xml.contains("<per-minute>80</per-minute>"));
    }

    #[test]
    fn test_emit_metronome_with_multiple_dots() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Half,
                beat_unit_dots: 2,
                per_minute: PerMinute {
                    value: "60".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        let count = xml.matches("<beat-unit-dot/>").count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_emit_metronome_beat_equation() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::BeatEquation {
                left_unit: NoteTypeValue::Half,
                left_dots: 0,
                right_unit: NoteTypeValue::Quarter,
                right_dots: 1,
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<beat-unit>half</beat-unit>"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<beat-unit-dot/>"));
    }

    #[test]
    fn test_emit_metronome_beat_equation_with_left_dots() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::BeatEquation {
                left_unit: NoteTypeValue::Quarter,
                left_dots: 1,
                right_unit: NoteTypeValue::Eighth,
                right_dots: 0,
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<beat-unit-dot/>"));
        assert!(xml.contains("<beat-unit>eighth</beat-unit>"));
    }

    #[test]
    fn test_emit_metronome_metric_modulation() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::MetricModulation {
                metric_relation: vec![MetricRelation {
                    left: crate::ir::direction::MetronomeNote {
                        note_type: NoteTypeValue::Quarter,
                        dots: 0,
                        tuplet: None,
                    },
                    right: crate::ir::direction::MetronomeNote {
                        note_type: NoteTypeValue::Eighth,
                        dots: 0,
                        tuplet: None,
                    },
                }],
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        // MetricModulation is a TODO, so it should just produce empty metronome
        assert!(xml.contains("<metronome>"));
        assert!(xml.contains("</metronome>"));
    }

    #[test]
    fn test_emit_metronome_parentheses_no() {
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: Some(YesNo::No),
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Quarter,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "100".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };

        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<metronome parentheses=\"no\">"));
    }

    #[test]
    fn test_emit_metronome_different_note_types() {
        // Test eighth note
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Eighth,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "160".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };
        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();
        assert!(xml.contains("<beat-unit>eighth</beat-unit>"));

        // Test whole note
        let mut w = XmlWriter::new();
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Whole,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "30".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };
        emit_metronome(&mut w, &metronome).unwrap();
        let xml = w.into_string().unwrap();
        assert!(xml.contains("<beat-unit>whole</beat-unit>"));
    }

    // ==================== emit_octave_shift Tests ====================

    #[test]
    fn test_emit_octave_shift_up() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(8),
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-shift type=\"up\" size=\"8\"/>"));
    }

    #[test]
    fn test_emit_octave_shift_down() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Down,
            number: None,
            size: Some(8),
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-shift type=\"down\" size=\"8\"/>"));
    }

    #[test]
    fn test_emit_octave_shift_stop() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Stop,
            number: Some(1),
            size: None,
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-shift type=\"stop\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_octave_shift_continue() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Continue,
            number: Some(1),
            size: None,
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-shift type=\"continue\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_octave_shift_8va() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(8),
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"up\""));
        assert!(xml.contains("size=\"8\""));
    }

    #[test]
    fn test_emit_octave_shift_15ma() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(15),
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("size=\"15\""));
    }

    #[test]
    fn test_emit_octave_shift_22ma() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(22),
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("size=\"22\""));
    }

    #[test]
    fn test_emit_octave_shift_minimal() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: None,
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<octave-shift type=\"up\"/>"));
    }

    #[test]
    fn test_emit_octave_shift_with_number() {
        let mut w = XmlWriter::new();
        let octave_shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: Some(2),
            size: Some(8),
            position: Position::default(),
        };

        emit_octave_shift(&mut w, &octave_shift).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("number=\"2\""));
    }

    // ==================== Bracket Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_bracket_todo() {
        use crate::ir::direction::{Bracket, LineEnd};

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Bracket(Bracket {
                r#type: StartStopContinue::Start,
                number: None,
                line_end: LineEnd::Up,
                end_length: None,
                line_type: None,
                position: Position::default(),
                color: None,
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // Bracket is a TODO, so should just have direction-type wrapper
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== HarpPedals Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_harp_pedals_todo() {
        use crate::ir::direction::HarpPedals;

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::HarpPedals(HarpPedals {
                pedal_tuning: vec![],
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // HarpPedals is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== StringMute Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_string_mute_todo() {
        use crate::ir::direction::{OnOff, StringMute};

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::StringMute(StringMute { r#type: OnOff::On }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // StringMute is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== Scordatura Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_scordatura_todo() {
        use crate::ir::direction::Scordatura;

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Scordatura(Scordatura::default()),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // Scordatura is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== Image Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_image_todo() {
        use crate::ir::direction::Image;

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Image(Image {
                source: "test.png".to_string(),
                r#type: "image/png".to_string(),
                position: Position::default(),
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // Image is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== PrincipalVoice Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_principal_voice_todo() {
        use crate::ir::common::StartStop;
        use crate::ir::direction::{PrincipalVoice, PrincipalVoiceSymbol};

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::PrincipalVoice(PrincipalVoice {
                r#type: StartStop::Start,
                symbol: PrincipalVoiceSymbol::Hauptstimme,
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // PrincipalVoice is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== Percussion Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_percussion_todo() {
        use crate::ir::direction::{Percussion, PercussionContent};

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::Percussion(vec![Percussion {
                content: PercussionContent::Timpani,
            }]),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // Percussion is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== AccordionRegistration Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_accordion_registration_todo() {
        use crate::ir::direction::AccordionRegistration;

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::AccordionRegistration(AccordionRegistration::default()),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // AccordionRegistration is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }

    // ==================== StaffDivide Tests (TODO - no-op) ====================

    #[test]
    fn test_emit_direction_type_staff_divide_todo() {
        use crate::ir::direction::{StaffDivide, StaffDivideSymbol};

        let mut w = XmlWriter::new();
        let dt = DirectionType {
            content: DirectionTypeContent::StaffDivide(StaffDivide {
                r#type: StaffDivideSymbol::Down,
            }),
        };

        emit_direction_type(&mut w, &dt).unwrap();
        let xml = w.into_string().unwrap();

        // StaffDivide is a TODO
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("</direction-type>"));
    }
}
