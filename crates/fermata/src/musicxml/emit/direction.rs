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
    use crate::ir::common::{AboveBelow, Font, LineType, Position, YesNo};
    use crate::ir::direction::{Metronome, PerMinute, WedgeType};

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
}
