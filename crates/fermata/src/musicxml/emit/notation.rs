//! Notation emission functions for MusicXML.
//!
//! This module handles the emission of notation elements including tied, slur,
//! tuplet, articulations, and other note-attached notations.

use crate::ir::common::WavyLine;
use crate::ir::notation::{
    AccidentalMark, Arpeggiate, Arrow, ArticulationElement, Articulations, Bend, BreathMark,
    Caesura, EmptyLine, EmptyTrillSound, Fingering, Fret, HammerPull, Handbell, HarmonMute,
    Harmonic, HeelToe, Hole, HoleClosed, Mordent, NonArpeggiate, NotationContent, Notations,
    OrnamentElement, OrnamentWithAccidentals, Ornaments, OtherNotation, OtherOrnament,
    OtherTechnical, Pluck, Slur, StringNumber, StrongAccent, Tap, Technical, TechnicalElement,
    Tied, Tremolo, Tuplet, TupletPortion, Turn,
};
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

use super::barline::emit_fermata;
use super::direction::emit_dynamics;
use super::helpers::{
    above_below_to_string, accidental_value_to_string, arrow_direction_to_string,
    arrow_style_to_string, breath_mark_value_to_string, caesura_value_to_string,
    handbell_value_to_string, hole_closed_location_to_string, hole_closed_value_to_string,
    line_length_to_string, line_shape_to_string, line_type_to_string, note_type_value_to_string,
    over_under_to_string, show_tuplet_to_string, start_note_to_string,
    start_stop_continue_to_string, start_stop_single_to_string, start_stop_to_string,
    tap_hand_to_string, top_bottom_to_string, tremolo_type_to_string, trill_step_to_string,
    two_note_turn_to_string, up_down_to_string, yes_no_to_string,
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
            NotationContent::Ornaments(ornaments) => {
                emit_ornaments(w, ornaments)?;
            }
            NotationContent::Technical(technical) => {
                emit_technical(w, technical)?;
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

// === Ornaments Emission (Task 5.2) ===

/// Emit an ornaments element.
pub(crate) fn emit_ornaments(w: &mut XmlWriter, ornaments: &Ornaments) -> Result<(), EmitError> {
    w.start_element("ornaments")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for owa in &ornaments.content {
        emit_ornament_with_accidentals(w, owa)?;
    }

    w.end_element("ornaments")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an ornament with its accidental marks.
pub(crate) fn emit_ornament_with_accidentals(
    w: &mut XmlWriter,
    owa: &OrnamentWithAccidentals,
) -> Result<(), EmitError> {
    emit_ornament_element(w, &owa.ornament)?;

    for acc_mark in &owa.accidental_marks {
        emit_accidental_mark_in_ornaments(w, acc_mark)?;
    }

    Ok(())
}

/// Emit an accidental mark within ornaments.
fn emit_accidental_mark_in_ornaments(
    w: &mut XmlWriter,
    acc_mark: &AccidentalMark,
) -> Result<(), EmitError> {
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
    Ok(())
}

/// Emit an individual ornament element.
pub(crate) fn emit_ornament_element(
    w: &mut XmlWriter,
    ornament: &OrnamentElement,
) -> Result<(), EmitError> {
    match ornament {
        OrnamentElement::TrillMark(ets) => emit_empty_trill_sound(w, "trill-mark", ets),
        OrnamentElement::Turn(turn) => emit_turn(w, "turn", turn),
        OrnamentElement::DelayedTurn(turn) => emit_turn(w, "delayed-turn", turn),
        OrnamentElement::InvertedTurn(turn) => emit_turn(w, "inverted-turn", turn),
        OrnamentElement::DelayedInvertedTurn(turn) => emit_turn(w, "delayed-inverted-turn", turn),
        OrnamentElement::VerticalTurn(ets) => emit_empty_trill_sound(w, "vertical-turn", ets),
        OrnamentElement::InvertedVerticalTurn(ets) => {
            emit_empty_trill_sound(w, "inverted-vertical-turn", ets)
        }
        OrnamentElement::Shake(ets) => emit_empty_trill_sound(w, "shake", ets),
        OrnamentElement::WavyLine(wl) => emit_wavy_line(w, wl),
        OrnamentElement::Mordent(m) => emit_mordent(w, "mordent", m),
        OrnamentElement::InvertedMordent(m) => emit_mordent(w, "inverted-mordent", m),
        OrnamentElement::Schleifer(ep) => emit_empty_placement(w, "schleifer", ep),
        OrnamentElement::Tremolo(t) => emit_tremolo(w, t),
        OrnamentElement::Haydn(ets) => emit_empty_trill_sound(w, "haydn", ets),
        OrnamentElement::OtherOrnament(oo) => emit_other_ornament(w, oo),
    }
}

/// Emit an empty-trill-sound element.
fn emit_empty_trill_sound(
    w: &mut XmlWriter,
    name: &str,
    ets: &EmptyTrillSound,
) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref placement) = ets.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref start_note) = ets.start_note {
        elem = elem.attr("start-note", start_note_to_string(start_note));
    }
    if let Some(ref trill_step) = ets.trill_step {
        elem = elem.attr("trill-step", trill_step_to_string(trill_step));
    }
    if let Some(ref two_note_turn) = ets.two_note_turn {
        elem = elem.attr("two-note-turn", two_note_turn_to_string(two_note_turn));
    }
    if let Some(ref accelerate) = ets.accelerate {
        elem = elem.attr("accelerate", yes_no_to_string(accelerate));
    }
    if let Some(beats) = ets.beats {
        elem = elem.attr("beats", &beats.to_string());
    }
    if let Some(second_beat) = ets.second_beat {
        elem = elem.attr("second-beat", &second_beat.to_string());
    }
    if let Some(last_beat) = ets.last_beat {
        elem = elem.attr("last-beat", &last_beat.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a turn ornament element.
fn emit_turn(w: &mut XmlWriter, name: &str, turn: &Turn) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref slash) = turn.slash {
        elem = elem.attr("slash", yes_no_to_string(slash));
    }
    if let Some(ref placement) = turn.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref start_note) = turn.start_note {
        elem = elem.attr("start-note", start_note_to_string(start_note));
    }
    if let Some(ref trill_step) = turn.trill_step {
        elem = elem.attr("trill-step", trill_step_to_string(trill_step));
    }
    if let Some(ref two_note_turn) = turn.two_note_turn {
        elem = elem.attr("two-note-turn", two_note_turn_to_string(two_note_turn));
    }
    if let Some(ref accelerate) = turn.accelerate {
        elem = elem.attr("accelerate", yes_no_to_string(accelerate));
    }
    if let Some(beats) = turn.beats {
        elem = elem.attr("beats", &beats.to_string());
    }
    if let Some(second_beat) = turn.second_beat {
        elem = elem.attr("second-beat", &second_beat.to_string());
    }
    if let Some(last_beat) = turn.last_beat {
        elem = elem.attr("last-beat", &last_beat.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a wavy-line element.
fn emit_wavy_line(w: &mut XmlWriter, wl: &WavyLine) -> Result<(), EmitError> {
    let mut elem =
        ElementBuilder::new("wavy-line").attr("type", start_stop_continue_to_string(&wl.r#type));

    if let Some(number) = wl.number {
        elem = elem.attr("number", &number.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a mordent element.
fn emit_mordent(w: &mut XmlWriter, name: &str, mordent: &Mordent) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref long) = mordent.long {
        elem = elem.attr("long", yes_no_to_string(long));
    }
    if let Some(ref approach) = mordent.approach {
        elem = elem.attr("approach", above_below_to_string(approach));
    }
    if let Some(ref departure) = mordent.departure {
        elem = elem.attr("departure", above_below_to_string(departure));
    }
    if let Some(ref placement) = mordent.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref start_note) = mordent.start_note {
        elem = elem.attr("start-note", start_note_to_string(start_note));
    }
    if let Some(ref trill_step) = mordent.trill_step {
        elem = elem.attr("trill-step", trill_step_to_string(trill_step));
    }
    if let Some(ref two_note_turn) = mordent.two_note_turn {
        elem = elem.attr("two-note-turn", two_note_turn_to_string(two_note_turn));
    }
    if let Some(ref accelerate) = mordent.accelerate {
        elem = elem.attr("accelerate", yes_no_to_string(accelerate));
    }
    if let Some(beats) = mordent.beats {
        elem = elem.attr("beats", &beats.to_string());
    }
    if let Some(second_beat) = mordent.second_beat {
        elem = elem.attr("second-beat", &second_beat.to_string());
    }
    if let Some(last_beat) = mordent.last_beat {
        elem = elem.attr("last-beat", &last_beat.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a tremolo element.
fn emit_tremolo(w: &mut XmlWriter, tremolo: &Tremolo) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("tremolo");

    if let Some(ref tremolo_type) = tremolo.r#type {
        elem = elem.attr("type", tremolo_type_to_string(tremolo_type));
    }
    if let Some(ref placement) = tremolo.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&tremolo.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("tremolo")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an other-ornament element.
fn emit_other_ornament(w: &mut XmlWriter, oo: &OtherOrnament) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("other-ornament");

    if let Some(ref placement) = oo.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&oo.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("other-ornament")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

// === Technical Emission (Task 5.3) ===

/// Emit a technical element.
pub(crate) fn emit_technical(w: &mut XmlWriter, technical: &Technical) -> Result<(), EmitError> {
    w.start_element("technical")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    for tech_elem in &technical.content {
        emit_technical_element(w, tech_elem)?;
    }

    w.end_element("technical")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an individual technical element.
pub(crate) fn emit_technical_element(
    w: &mut XmlWriter,
    tech: &TechnicalElement,
) -> Result<(), EmitError> {
    match tech {
        TechnicalElement::UpBow(ep) => emit_empty_placement(w, "up-bow", ep),
        TechnicalElement::DownBow(ep) => emit_empty_placement(w, "down-bow", ep),
        TechnicalElement::Harmonic(h) => emit_harmonic(w, h),
        TechnicalElement::OpenString(ep) => emit_empty_placement(w, "open-string", ep),
        TechnicalElement::ThumbPosition(ep) => emit_empty_placement(w, "thumb-position", ep),
        TechnicalElement::Fingering(f) => emit_fingering(w, f),
        TechnicalElement::Pluck(p) => emit_pluck(w, p),
        TechnicalElement::DoubleTongue(ep) => emit_empty_placement(w, "double-tongue", ep),
        TechnicalElement::TripleTongue(ep) => emit_empty_placement(w, "triple-tongue", ep),
        TechnicalElement::Stopped(ep) => emit_empty_placement(w, "stopped", ep),
        TechnicalElement::SnapPizzicato(ep) => emit_empty_placement(w, "snap-pizzicato", ep),
        TechnicalElement::Fret(f) => emit_fret(w, f),
        TechnicalElement::String(s) => emit_string_number(w, s),
        TechnicalElement::HammerOn(hp) => emit_hammer_pull(w, "hammer-on", hp),
        TechnicalElement::PullOff(hp) => emit_hammer_pull(w, "pull-off", hp),
        TechnicalElement::Bend(b) => emit_bend(w, b),
        TechnicalElement::Tap(t) => emit_tap(w, t),
        TechnicalElement::Heel(ht) => emit_heel_toe(w, "heel", ht),
        TechnicalElement::Toe(ht) => emit_heel_toe(w, "toe", ht),
        TechnicalElement::Fingernails(ep) => emit_empty_placement(w, "fingernails", ep),
        TechnicalElement::Hole(h) => emit_hole(w, h),
        TechnicalElement::Arrow(a) => emit_arrow(w, a),
        TechnicalElement::Handbell(hb) => emit_handbell(w, hb),
        TechnicalElement::BrassBend(ep) => emit_empty_placement(w, "brass-bend", ep),
        TechnicalElement::Flip(ep) => emit_empty_placement(w, "flip", ep),
        TechnicalElement::Smear(ep) => emit_empty_placement(w, "smear", ep),
        TechnicalElement::Open(ep) => emit_empty_placement(w, "open", ep),
        TechnicalElement::HalfMuted(ep) => emit_empty_placement(w, "half-muted", ep),
        TechnicalElement::HarmonMute(hm) => emit_harmon_mute(w, hm),
        TechnicalElement::Golpe(ep) => emit_empty_placement(w, "golpe", ep),
        TechnicalElement::OtherTechnical(ot) => emit_other_technical(w, ot),
    }
}

/// Emit a harmonic element.
fn emit_harmonic(w: &mut XmlWriter, harmonic: &Harmonic) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("harmonic");

    if let Some(ref placement) = harmonic.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }
    if let Some(ref print_object) = harmonic.print_object {
        elem = elem.attr("print-object", yes_no_to_string(print_object));
    }

    let has_content = harmonic.natural
        || harmonic.artificial
        || harmonic.base_pitch
        || harmonic.touching_pitch
        || harmonic.sounding_pitch;

    if has_content {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

        if harmonic.natural {
            w.empty_element("natural")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if harmonic.artificial {
            w.empty_element("artificial")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if harmonic.base_pitch {
            w.empty_element("base-pitch")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if harmonic.touching_pitch {
            w.empty_element("touching-pitch")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if harmonic.sounding_pitch {
            w.empty_element("sounding-pitch")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }

        w.end_element("harmonic")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit a fingering element.
fn emit_fingering(w: &mut XmlWriter, fingering: &Fingering) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("fingering");

    if let Some(ref substitution) = fingering.substitution {
        elem = elem.attr("substitution", yes_no_to_string(substitution));
    }
    if let Some(ref alternate) = fingering.alternate {
        elem = elem.attr("alternate", yes_no_to_string(alternate));
    }
    if let Some(ref placement) = fingering.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&fingering.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("fingering")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a pluck element.
fn emit_pluck(w: &mut XmlWriter, pluck: &Pluck) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("pluck");

    if let Some(ref placement) = pluck.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&pluck.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("pluck")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a fret element.
fn emit_fret(w: &mut XmlWriter, fret: &Fret) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("fret");

    if let Some(ref color) = fret.color {
        elem = elem.attr("color", color);
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&fret.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("fret")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a string element (for string number).
fn emit_string_number(w: &mut XmlWriter, string: &StringNumber) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("string");

    if let Some(ref placement) = string.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&string.value.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("string")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a hammer-on or pull-off element.
fn emit_hammer_pull(w: &mut XmlWriter, name: &str, hp: &HammerPull) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name).attr("type", start_stop_to_string(&hp.r#type));

    if let Some(number) = hp.number {
        elem = elem.attr("number", &number.to_string());
    }
    if let Some(ref placement) = hp.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    if hp.value.is_empty() {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(&hp.value)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element(name)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit a bend element.
fn emit_bend(w: &mut XmlWriter, bend: &Bend) -> Result<(), EmitError> {
    w.start_element("bend")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.text_element("bend-alter", &bend.bend_alter.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if bend.pre_bend {
        w.empty_element("pre-bend")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    if let Some(ref release) = bend.release {
        let release_str = match release {
            crate::ir::notation::BendRelease::Early => "early",
            crate::ir::notation::BendRelease::Late => "late",
        };
        let elem = ElementBuilder::new("release").attr("offset", release_str);
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    if let Some(ref with_bar) = bend.with_bar {
        w.text_element("with-bar", with_bar)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("bend")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a tap element.
fn emit_tap(w: &mut XmlWriter, tap: &Tap) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("tap");

    if let Some(ref hand) = tap.hand {
        elem = elem.attr("hand", tap_hand_to_string(hand));
    }

    if tap.value.is_empty() {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(&tap.value)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("tap")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit a heel or toe element.
fn emit_heel_toe(w: &mut XmlWriter, name: &str, ht: &HeelToe) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new(name);

    if let Some(ref substitution) = ht.substitution {
        elem = elem.attr("substitution", yes_no_to_string(substitution));
    }
    if let Some(ref placement) = ht.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a hole element.
fn emit_hole(w: &mut XmlWriter, hole: &Hole) -> Result<(), EmitError> {
    w.start_element("hole")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    if let Some(ref hole_type) = hole.hole_type {
        w.text_element("hole-type", hole_type)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    emit_hole_closed(w, &hole.hole_closed)?;

    if let Some(ref hole_shape) = hole.hole_shape {
        w.text_element("hole-shape", hole_shape)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("hole")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a hole-closed element.
fn emit_hole_closed(w: &mut XmlWriter, hc: &HoleClosed) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("hole-closed");

    if let Some(ref location) = hc.location {
        elem = elem.attr("location", hole_closed_location_to_string(location));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(hole_closed_value_to_string(&hc.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("hole-closed")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an arrow element.
fn emit_arrow(w: &mut XmlWriter, arrow: &Arrow) -> Result<(), EmitError> {
    let has_content = arrow.direction.is_some() || arrow.style.is_some() || arrow.smufl.is_some();

    if has_content {
        w.start_element("arrow")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

        if let Some(ref direction) = arrow.direction {
            w.text_element("arrow-direction", arrow_direction_to_string(direction))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if let Some(ref style) = arrow.style {
            w.text_element("arrow-style", arrow_style_to_string(style))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        // Note: circular-arrow and smufl handling would go here

        w.end_element("arrow")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element("arrow")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit a handbell element.
fn emit_handbell(w: &mut XmlWriter, handbell: &Handbell) -> Result<(), EmitError> {
    w.text_element("handbell", handbell_value_to_string(&handbell.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a harmon-mute element.
fn emit_harmon_mute(w: &mut XmlWriter, hm: &HarmonMute) -> Result<(), EmitError> {
    let has_content = hm.open || hm.half;

    if has_content {
        w.start_element("harmon-mute")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

        if hm.open {
            w.text_element("harmon-closed", "open")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        } else if hm.half {
            w.text_element("harmon-closed", "half")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }

        w.end_element("harmon-mute")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element("harmon-mute")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }
    Ok(())
}

/// Emit an other-technical element.
fn emit_other_technical(w: &mut XmlWriter, ot: &OtherTechnical) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("other-technical");

    if let Some(ref placement) = ot.placement {
        elem = elem.attr("placement", above_below_to_string(placement));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(&ot.value)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("other-technical")
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

    // === Ornaments Tests (Milestone 5) ===

    #[test]
    fn test_emit_ornaments_trill() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::TrillMark(EmptyTrillSound {
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<ornaments>"));
        assert!(xml.contains("<trill-mark placement=\"above\"/>"));
        assert!(xml.contains("</ornaments>"));
    }

    #[test]
    fn test_emit_ornaments_mordent() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Mordent(Mordent {
                    long: Some(YesNo::Yes),
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<mordent long=\"yes\" placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_ornaments_tremolo() {
        use crate::ir::notation::{
            OrnamentElement, OrnamentWithAccidentals, Ornaments, TremoloType,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Tremolo(Tremolo {
                    value: 3,
                    r#type: Some(TremoloType::Single),
                    placement: Some(AboveBelow::Below),
                    position: Position::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tremolo type=\"single\" placement=\"below\">3</tremolo>"));
    }

    #[test]
    fn test_emit_ornaments_turn() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Turn(Turn {
                    slash: Some(YesNo::Yes),
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<turn slash=\"yes\" placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_ornaments_with_accidental_mark() {
        use crate::ir::PrintStyle;
        use crate::ir::common::AccidentalValue;
        use crate::ir::notation::{
            AccidentalMark, OrnamentElement, OrnamentWithAccidentals, Ornaments,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::TrillMark(EmptyTrillSound {
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![AccidentalMark {
                    value: AccidentalValue::Sharp,
                    placement: Some(AboveBelow::Above),
                    print_style: PrintStyle::default(),
                }],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<trill-mark placement=\"above\"/>"));
        assert!(xml.contains("<accidental-mark placement=\"above\">sharp</accidental-mark>"));
    }

    // === Technical Tests (Milestone 5) ===

    #[test]
    fn test_emit_technical_fingering() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Fingering(Fingering {
                value: "1".to_string(),
                substitution: None,
                alternate: None,
                placement: Some(AboveBelow::Above),
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<technical>"));
        assert!(xml.contains("<fingering placement=\"above\">1</fingering>"));
        assert!(xml.contains("</technical>"));
    }

    #[test]
    fn test_emit_technical_string_number() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::String(StringNumber {
                value: 1,
                placement: Some(AboveBelow::Above),
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<string placement=\"above\">1</string>"));
    }

    #[test]
    fn test_emit_technical_fret() {
        use crate::ir::common::Font;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Fret(Fret {
                value: 5,
                font: Font::default(),
                color: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fret>5</fret>"));
    }

    #[test]
    fn test_emit_technical_upbow_downbow() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![
                TechnicalElement::UpBow(EmptyPlacement {
                    placement: Some(AboveBelow::Above),
                    position: Position::default(),
                }),
                TechnicalElement::DownBow(EmptyPlacement {
                    placement: Some(AboveBelow::Below),
                    position: Position::default(),
                }),
            ],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<up-bow placement=\"above\"/>"));
        assert!(xml.contains("<down-bow placement=\"below\"/>"));
    }

    #[test]
    fn test_emit_technical_harmonic() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Harmonic(Harmonic {
                natural: true,
                artificial: false,
                base_pitch: false,
                touching_pitch: false,
                sounding_pitch: false,
                placement: Some(AboveBelow::Above),
                print_object: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<harmonic placement=\"above\">"));
        assert!(xml.contains("<natural/>"));
        assert!(xml.contains("</harmonic>"));
    }

    #[test]
    fn test_emit_technical_hammer_on() {
        use crate::ir::common::StartStop;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::HammerOn(HammerPull {
                value: "H".to_string(),
                r#type: StartStop::Start,
                number: Some(1),
                placement: Some(AboveBelow::Above),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(
            xml.contains(
                "<hammer-on type=\"start\" number=\"1\" placement=\"above\">H</hammer-on>"
            )
        );
    }

    // === Additional Notations Tests for comprehensive coverage ===

    #[test]
    fn test_emit_notations_with_print_object() {
        use crate::ir::common::YesNo;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: Some(YesNo::No),
            content: vec![],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<notations print-object=\"no\">"));
    }

    #[test]
    fn test_emit_notations_with_tuplet() {
        use crate::ir::common::StartStop;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Tuplet(Box::new(Tuplet {
                r#type: StartStop::Start,
                number: Some(1),
                bracket: None,
                show_number: None,
                show_type: None,
                line_shape: None,
                position: Position::default(),
                placement: None,
                tuplet_actual: None,
                tuplet_normal: None,
            }))],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tuplet type=\"start\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_notations_with_arpeggiate() {
        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Arpeggiate(Arpeggiate {
                number: Some(1),
                direction: Some(UpDown::Up),
                position: Position::default(),
                color: Some("#0000FF".to_string()),
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arpeggiate number=\"1\" direction=\"up\" color=\"#0000FF\"/>"));
    }

    #[test]
    fn test_emit_notations_with_non_arpeggiate() {
        use crate::ir::notation::TopBottom;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::NonArpeggiate(NonArpeggiate {
                r#type: TopBottom::Bottom,
                number: Some(1),
                position: Position::default(),
                color: Some("#FF0000".to_string()),
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<non-arpeggiate type=\"bottom\" number=\"1\" color=\"#FF0000\"/>"));
    }

    #[test]
    fn test_emit_notations_with_accidental_mark() {
        use crate::ir::PrintStyle;
        use crate::ir::common::AccidentalValue;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::AccidentalMark(AccidentalMark {
                value: AccidentalValue::Flat,
                placement: Some(AboveBelow::Below),
                print_style: PrintStyle::default(),
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<accidental-mark placement=\"below\">flat</accidental-mark>"));
    }

    #[test]
    fn test_emit_notations_with_other_notation() {
        use crate::ir::PrintStyle;
        use crate::ir::common::{StartStopSingle, YesNo};

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::OtherNotation(OtherNotation {
                value: "custom".to_string(),
                r#type: StartStopSingle::Single,
                number: Some(1),
                print_object: Some(YesNo::Yes),
                print_style: PrintStyle::default(),
                placement: Some(AboveBelow::Above),
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"single\""));
        assert!(xml.contains("number=\"1\""));
        assert!(xml.contains("print-object=\"yes\""));
        assert!(xml.contains("placement=\"above\""));
        assert!(xml.contains(">custom</other-notation>"));
    }

    #[test]
    fn test_emit_notations_with_ornaments() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Ornaments(Box::new(Ornaments {
                content: vec![OrnamentWithAccidentals {
                    ornament: OrnamentElement::TrillMark(EmptyTrillSound::default()),
                    accidental_marks: vec![],
                }],
            }))],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<ornaments>"));
        assert!(xml.contains("<trill-mark/>"));
        assert!(xml.contains("</ornaments>"));
    }

    #[test]
    fn test_emit_notations_with_technical() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Technical(Box::new(Technical {
                content: vec![TechnicalElement::OpenString(EmptyPlacement::default())],
            }))],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<technical>"));
        assert!(xml.contains("<open-string/>"));
        assert!(xml.contains("</technical>"));
    }

    #[test]
    fn test_emit_notations_with_articulations() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Articulations(Box::new(Articulations {
                content: vec![ArticulationElement::Accent(EmptyPlacement::default())],
            }))],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<articulations>"));
        assert!(xml.contains("<accent/>"));
        assert!(xml.contains("</articulations>"));
    }

    #[test]
    fn test_emit_notations_with_dynamics() {
        use crate::ir::direction::{DynamicElement, Dynamics};

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::Dynamics(Box::new(Dynamics {
                content: vec![DynamicElement::F],
                ..Default::default()
            }))],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<dynamics>"));
        assert!(xml.contains("<f/>"));
        assert!(xml.contains("</dynamics>"));
    }

    // === Tuplet Tests ===

    #[test]
    fn test_emit_tuplet_minimal() {
        use crate::ir::common::StartStop;

        let mut w = XmlWriter::new();
        let tuplet = Tuplet {
            r#type: StartStop::Stop,
            number: None,
            bracket: None,
            show_number: None,
            show_type: None,
            line_shape: None,
            position: Position::default(),
            placement: None,
            tuplet_actual: None,
            tuplet_normal: None,
        };

        emit_tuplet(&mut w, &tuplet).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tuplet type=\"stop\"/>"));
    }

    #[test]
    fn test_emit_tuplet_with_all_attrs() {
        use crate::ir::common::{StartStop, YesNo};
        use crate::ir::notation::{LineShape, ShowTuplet};

        let mut w = XmlWriter::new();
        let tuplet = Tuplet {
            r#type: StartStop::Start,
            number: Some(1),
            bracket: Some(YesNo::Yes),
            show_number: Some(ShowTuplet::Actual),
            show_type: Some(ShowTuplet::Both),
            line_shape: Some(LineShape::Curved),
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            tuplet_actual: None,
            tuplet_normal: None,
        };

        emit_tuplet(&mut w, &tuplet).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"start\""));
        assert!(xml.contains("number=\"1\""));
        assert!(xml.contains("bracket=\"yes\""));
        assert!(xml.contains("show-number=\"actual\""));
        assert!(xml.contains("show-type=\"both\""));
        assert!(xml.contains("line-shape=\"curved\""));
        assert!(xml.contains("placement=\"above\""));
    }

    #[test]
    fn test_emit_tuplet_with_portions() {
        use crate::ir::common::{Font, StartStop};
        use crate::ir::duration::NoteTypeValue;
        use crate::ir::notation::{TupletDot, TupletNumber, TupletPortion, TupletType};

        let mut w = XmlWriter::new();
        let tuplet = Tuplet {
            r#type: StartStop::Start,
            number: Some(1),
            bracket: None,
            show_number: None,
            show_type: None,
            line_shape: None,
            position: Position::default(),
            placement: None,
            tuplet_actual: Some(TupletPortion {
                tuplet_number: Some(TupletNumber {
                    value: 3,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_type: Some(TupletType {
                    value: NoteTypeValue::Eighth,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_dots: vec![TupletDot::default()],
            }),
            tuplet_normal: Some(TupletPortion {
                tuplet_number: Some(TupletNumber {
                    value: 2,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_type: None,
                tuplet_dots: vec![],
            }),
        };

        emit_tuplet(&mut w, &tuplet).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tuplet type=\"start\" number=\"1\">"));
        assert!(xml.contains("<tuplet-actual>"));
        assert!(xml.contains("<tuplet-number>3</tuplet-number>"));
        assert!(xml.contains("<tuplet-type>eighth</tuplet-type>"));
        assert!(xml.contains("<tuplet-dot/>"));
        assert!(xml.contains("</tuplet-actual>"));
        assert!(xml.contains("<tuplet-normal>"));
        assert!(xml.contains("<tuplet-number>2</tuplet-number>"));
        assert!(xml.contains("</tuplet-normal>"));
        assert!(xml.contains("</tuplet>"));
    }

    // === Slur Tests ===

    #[test]
    fn test_emit_slur_with_all_attrs() {
        use crate::ir::common::OverUnder;

        let mut w = XmlWriter::new();
        let slur = Slur {
            r#type: StartStopContinue::Continue,
            number: 2,
            line_type: Some(LineType::Dashed),
            position: Position::default(),
            placement: Some(AboveBelow::Below),
            orientation: Some(OverUnder::Under),
            color: Some("#00FF00".to_string()),
        };

        emit_slur(&mut w, &slur).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("type=\"continue\""));
        assert!(xml.contains("number=\"2\""));
        assert!(xml.contains("line-type=\"dashed\""));
        assert!(xml.contains("placement=\"below\""));
        assert!(xml.contains("orientation=\"under\""));
        assert!(xml.contains("color=\"#00FF00\""));
    }

    // === Arpeggiate Tests ===

    #[test]
    fn test_emit_arpeggiate_minimal() {
        let mut w = XmlWriter::new();
        let arpeggiate = Arpeggiate::default();

        emit_arpeggiate(&mut w, &arpeggiate).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arpeggiate/>"));
    }

    #[test]
    fn test_emit_arpeggiate_down() {
        let mut w = XmlWriter::new();
        let arpeggiate = Arpeggiate {
            number: None,
            direction: Some(UpDown::Down),
            position: Position::default(),
            color: None,
        };

        emit_arpeggiate(&mut w, &arpeggiate).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arpeggiate direction=\"down\"/>"));
    }

    // === NonArpeggiate Tests ===

    #[test]
    fn test_emit_non_arpeggiate_top() {
        use crate::ir::notation::TopBottom;

        let mut w = XmlWriter::new();
        let non_arp = NonArpeggiate {
            r#type: TopBottom::Top,
            number: None,
            position: Position::default(),
            color: None,
        };

        emit_non_arpeggiate(&mut w, &non_arp).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<non-arpeggiate type=\"top\"/>"));
    }

    // === Other Notation Tests ===

    #[test]
    fn test_emit_other_notation_minimal() {
        use crate::ir::PrintStyle;
        use crate::ir::common::StartStopSingle;

        let mut w = XmlWriter::new();
        let other = OtherNotation {
            value: "custom-notation".to_string(),
            r#type: StartStopSingle::Start,
            number: None,
            print_object: None,
            print_style: PrintStyle::default(),
            placement: None,
        };

        emit_other_notation(&mut w, &other).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<other-notation type=\"start\">custom-notation</other-notation>"));
    }

    // === Additional Articulation Tests ===

    #[test]
    fn test_emit_articulations_detached_legato() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::DetachedLegato(EmptyPlacement {
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<detached-legato placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_articulations_staccatissimo() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::Staccatissimo(EmptyPlacement {
                placement: Some(AboveBelow::Below),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<staccatissimo placement=\"below\"/>"));
    }

    #[test]
    fn test_emit_articulations_spiccato() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::Spiccato(EmptyPlacement::default())],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<spiccato/>"));
    }

    #[test]
    fn test_emit_articulations_stress_unstress() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![
                ArticulationElement::Stress(EmptyPlacement::default()),
                ArticulationElement::Unstress(EmptyPlacement::default()),
            ],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<stress/>"));
        assert!(xml.contains("<unstress/>"));
    }

    #[test]
    fn test_emit_articulations_soft_accent() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::SoftAccent(EmptyPlacement {
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<soft-accent placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_articulations_other_articulation() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::OtherArticulation;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::OtherArticulation(OtherArticulation {
                value: "custom-articulation".to_string(),
                placement: Some(AboveBelow::Below),
                print_style: PrintStyle::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains(
            "<other-articulation placement=\"below\">custom-articulation</other-articulation>"
        ));
    }

    #[test]
    fn test_emit_articulations_jazz_with_attrs() {
        use crate::ir::notation::LineLength;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::Scoop(EmptyLine {
                line_shape: Some(crate::ir::notation::LineShape::Curved),
                line_type: Some(LineType::Solid),
                line_length: Some(LineLength::Medium),
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("line-shape=\"curved\""));
        assert!(xml.contains("line-type=\"solid\""));
        assert!(xml.contains("line-length=\"medium\""));
        assert!(xml.contains("placement=\"above\""));
    }

    #[test]
    fn test_emit_breath_mark_empty() {
        let mut w = XmlWriter::new();
        let bm = BreathMark {
            value: BreathMarkValue::Empty,
            placement: None,
            position: Position::default(),
        };

        emit_breath_mark(&mut w, &bm).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<breath-mark/>"));
    }

    #[test]
    fn test_emit_breath_mark_tick() {
        let mut w = XmlWriter::new();
        let bm = BreathMark {
            value: BreathMarkValue::Tick,
            placement: None,
            position: Position::default(),
        };

        emit_breath_mark(&mut w, &bm).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<breath-mark>tick</breath-mark>"));
    }

    #[test]
    fn test_emit_breath_mark_upbow() {
        let mut w = XmlWriter::new();
        let bm = BreathMark {
            value: BreathMarkValue::Upbow,
            placement: None,
            position: Position::default(),
        };

        emit_breath_mark(&mut w, &bm).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<breath-mark>upbow</breath-mark>"));
    }

    #[test]
    fn test_emit_breath_mark_salzedo() {
        let mut w = XmlWriter::new();
        let bm = BreathMark {
            value: BreathMarkValue::Salzedo,
            placement: None,
            position: Position::default(),
        };

        emit_breath_mark(&mut w, &bm).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<breath-mark>salzedo</breath-mark>"));
    }

    #[test]
    fn test_emit_caesura_thick() {
        let mut w = XmlWriter::new();
        let c = Caesura {
            value: CaesuraValue::Thick,
            placement: None,
            position: Position::default(),
        };

        emit_caesura(&mut w, &c).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<caesura>thick</caesura>"));
    }

    #[test]
    fn test_emit_caesura_short() {
        let mut w = XmlWriter::new();
        let c = Caesura {
            value: CaesuraValue::Short,
            placement: None,
            position: Position::default(),
        };

        emit_caesura(&mut w, &c).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<caesura>short</caesura>"));
    }

    #[test]
    fn test_emit_caesura_curved() {
        let mut w = XmlWriter::new();
        let c = Caesura {
            value: CaesuraValue::Curved,
            placement: None,
            position: Position::default(),
        };

        emit_caesura(&mut w, &c).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<caesura>curved</caesura>"));
    }

    #[test]
    fn test_emit_caesura_single() {
        let mut w = XmlWriter::new();
        let c = Caesura {
            value: CaesuraValue::Single,
            placement: None,
            position: Position::default(),
        };

        emit_caesura(&mut w, &c).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<caesura>single</caesura>"));
    }

    #[test]
    fn test_emit_strong_accent_down() {
        let mut w = XmlWriter::new();
        let sa = StrongAccent {
            r#type: Some(UpDown::Down),
            placement: None,
            position: Position::default(),
        };

        emit_strong_accent(&mut w, &sa).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<strong-accent type=\"down\"/>"));
    }

    #[test]
    fn test_emit_strong_accent_minimal() {
        let mut w = XmlWriter::new();
        let sa = StrongAccent::default();

        emit_strong_accent(&mut w, &sa).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<strong-accent/>"));
    }

    // === Additional Ornament Tests ===

    #[test]
    fn test_emit_ornament_delayed_turn() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::DelayedTurn(Turn::default()),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<delayed-turn/>"));
    }

    #[test]
    fn test_emit_ornament_inverted_turn() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::InvertedTurn(Turn {
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<inverted-turn placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_ornament_delayed_inverted_turn() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::DelayedInvertedTurn(Turn::default()),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<delayed-inverted-turn/>"));
    }

    #[test]
    fn test_emit_ornament_vertical_turn() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::VerticalTurn(EmptyTrillSound::default()),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<vertical-turn/>"));
    }

    #[test]
    fn test_emit_ornament_inverted_vertical_turn() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::InvertedVerticalTurn(EmptyTrillSound::default()),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<inverted-vertical-turn/>"));
    }

    #[test]
    fn test_emit_ornament_shake() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Shake(EmptyTrillSound {
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<shake placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_ornament_wavy_line() {
        use crate::ir::common::WavyLine;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::WavyLine(WavyLine {
                    r#type: StartStopContinue::Start,
                    number: Some(1),
                    position: Position::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wavy-line type=\"start\" number=\"1\"/>"));
    }

    #[test]
    fn test_emit_ornament_wavy_line_continue() {
        use crate::ir::common::WavyLine;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::WavyLine(WavyLine {
                    r#type: StartStopContinue::Continue,
                    number: None,
                    position: Position::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<wavy-line type=\"continue\"/>"));
    }

    #[test]
    fn test_emit_ornament_inverted_mordent() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::InvertedMordent(Mordent {
                    placement: Some(AboveBelow::Above),
                    ..Default::default()
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<inverted-mordent placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_ornament_schleifer() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Schleifer(EmptyPlacement {
                    placement: Some(AboveBelow::Above),
                    position: Position::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<schleifer placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_ornament_haydn() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Haydn(EmptyTrillSound::default()),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<haydn/>"));
    }

    #[test]
    fn test_emit_ornament_other_ornament() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::OtherOrnament(OtherOrnament {
                    value: "custom-ornament".to_string(),
                    placement: Some(AboveBelow::Above),
                    print_style: PrintStyle::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(
            xml.contains("<other-ornament placement=\"above\">custom-ornament</other-ornament>")
        );
    }

    #[test]
    fn test_emit_empty_trill_sound_with_all_attrs() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::{
            OrnamentElement, OrnamentWithAccidentals, Ornaments, StartNote, TrillStep, TwoNoteTurn,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::TrillMark(EmptyTrillSound {
                    placement: Some(AboveBelow::Above),
                    position: Position::default(),
                    start_note: Some(StartNote::Upper),
                    trill_step: Some(TrillStep::Half),
                    two_note_turn: Some(TwoNoteTurn::Whole),
                    accelerate: Some(YesNo::Yes),
                    beats: Some(4.0),
                    second_beat: Some(25.0),
                    last_beat: Some(75.0),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("placement=\"above\""));
        assert!(xml.contains("start-note=\"upper\""));
        assert!(xml.contains("trill-step=\"half\""));
        assert!(xml.contains("two-note-turn=\"whole\""));
        assert!(xml.contains("accelerate=\"yes\""));
        assert!(xml.contains("beats=\"4\""));
        assert!(xml.contains("second-beat=\"25\""));
        assert!(xml.contains("last-beat=\"75\""));
    }

    #[test]
    fn test_emit_turn_with_all_attrs() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::{
            OrnamentElement, OrnamentWithAccidentals, Ornaments, StartNote, TrillStep, TwoNoteTurn,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Turn(Turn {
                    slash: Some(YesNo::No),
                    placement: Some(AboveBelow::Below),
                    position: Position::default(),
                    start_note: Some(StartNote::Main),
                    trill_step: Some(TrillStep::Whole),
                    two_note_turn: Some(TwoNoteTurn::Half),
                    accelerate: Some(YesNo::No),
                    beats: Some(2.5),
                    second_beat: Some(33.0),
                    last_beat: Some(66.0),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("slash=\"no\""));
        assert!(xml.contains("placement=\"below\""));
        assert!(xml.contains("start-note=\"main\""));
        assert!(xml.contains("trill-step=\"whole\""));
        assert!(xml.contains("two-note-turn=\"half\""));
        assert!(xml.contains("accelerate=\"no\""));
        assert!(xml.contains("beats=\"2.5\""));
    }

    #[test]
    fn test_emit_mordent_with_all_attrs() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::{
            OrnamentElement, OrnamentWithAccidentals, Ornaments, StartNote, TrillStep, TwoNoteTurn,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Mordent(Mordent {
                    long: Some(YesNo::Yes),
                    approach: Some(AboveBelow::Above),
                    departure: Some(AboveBelow::Below),
                    placement: Some(AboveBelow::Above),
                    position: Position::default(),
                    start_note: Some(StartNote::Below),
                    trill_step: Some(TrillStep::Unison),
                    two_note_turn: Some(TwoNoteTurn::None),
                    accelerate: Some(YesNo::Yes),
                    beats: Some(3.0),
                    second_beat: Some(20.0),
                    last_beat: Some(80.0),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("long=\"yes\""));
        assert!(xml.contains("approach=\"above\""));
        assert!(xml.contains("departure=\"below\""));
        assert!(xml.contains("placement=\"above\""));
        assert!(xml.contains("start-note=\"below\""));
        assert!(xml.contains("trill-step=\"unison\""));
        assert!(xml.contains("two-note-turn=\"none\""));
        assert!(xml.contains("accelerate=\"yes\""));
        assert!(xml.contains("beats=\"3\""));
    }

    #[test]
    fn test_emit_tremolo_without_type() {
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Tremolo(Tremolo {
                    value: 2,
                    r#type: None,
                    placement: None,
                    position: Position::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tremolo>2</tremolo>"));
    }

    #[test]
    fn test_emit_tremolo_start_stop() {
        use crate::ir::notation::{
            OrnamentElement, OrnamentWithAccidentals, Ornaments, TremoloType,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![
                OrnamentWithAccidentals {
                    ornament: OrnamentElement::Tremolo(Tremolo {
                        value: 3,
                        r#type: Some(TremoloType::Start),
                        placement: None,
                        position: Position::default(),
                    }),
                    accidental_marks: vec![],
                },
                OrnamentWithAccidentals {
                    ornament: OrnamentElement::Tremolo(Tremolo {
                        value: 3,
                        r#type: Some(TremoloType::Stop),
                        placement: None,
                        position: Position::default(),
                    }),
                    accidental_marks: vec![],
                },
            ],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tremolo type=\"start\">3</tremolo>"));
        assert!(xml.contains("<tremolo type=\"stop\">3</tremolo>"));
    }

    #[test]
    fn test_emit_tremolo_unmeasured() {
        use crate::ir::notation::{
            OrnamentElement, OrnamentWithAccidentals, Ornaments, TremoloType,
        };

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::Tremolo(Tremolo {
                    value: 0,
                    r#type: Some(TremoloType::Unmeasured),
                    placement: None,
                    position: Position::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tremolo type=\"unmeasured\">0</tremolo>"));
    }

    // === Additional Technical Tests ===

    #[test]
    fn test_emit_technical_pluck() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Pluck(Pluck {
                value: "p".to_string(),
                placement: Some(AboveBelow::Below),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pluck placement=\"below\">p</pluck>"));
    }

    #[test]
    fn test_emit_technical_pluck_no_placement() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Pluck(Pluck {
                value: "i".to_string(),
                placement: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<pluck>i</pluck>"));
    }

    #[test]
    fn test_emit_technical_pull_off() {
        use crate::ir::common::StartStop;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::PullOff(HammerPull {
                value: "P".to_string(),
                r#type: StartStop::Stop,
                number: Some(1),
                placement: Some(AboveBelow::Below),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(
            xml.contains("<pull-off type=\"stop\" number=\"1\" placement=\"below\">P</pull-off>")
        );
    }

    #[test]
    fn test_emit_technical_hammer_on_empty_value() {
        use crate::ir::common::StartStop;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::HammerOn(HammerPull {
                value: "".to_string(),
                r#type: StartStop::Start,
                number: None,
                placement: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<hammer-on type=\"start\"/>"));
    }

    #[test]
    fn test_emit_technical_fingering_with_substitution() {
        use crate::ir::PrintStyle;
        use crate::ir::common::YesNo;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Fingering(Fingering {
                value: "2".to_string(),
                substitution: Some(YesNo::Yes),
                alternate: None,
                placement: None,
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fingering substitution=\"yes\">2</fingering>"));
    }

    #[test]
    fn test_emit_technical_fingering_with_alternate() {
        use crate::ir::PrintStyle;
        use crate::ir::common::YesNo;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Fingering(Fingering {
                value: "3".to_string(),
                substitution: None,
                alternate: Some(YesNo::Yes),
                placement: None,
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fingering alternate=\"yes\">3</fingering>"));
    }

    #[test]
    fn test_emit_technical_fingering_all_attrs() {
        use crate::ir::PrintStyle;
        use crate::ir::common::YesNo;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Fingering(Fingering {
                value: "1".to_string(),
                substitution: Some(YesNo::Yes),
                alternate: Some(YesNo::No),
                placement: Some(AboveBelow::Above),
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("substitution=\"yes\""));
        assert!(xml.contains("alternate=\"no\""));
        assert!(xml.contains("placement=\"above\""));
        assert!(xml.contains(">1</fingering>"));
    }

    #[test]
    fn test_emit_technical_fret_with_color() {
        use crate::ir::common::Font;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Fret(Fret {
                value: 7,
                font: Font::default(),
                color: Some("#FF0000".to_string()),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<fret color=\"#FF0000\">7</fret>"));
    }

    #[test]
    fn test_emit_technical_string_no_placement() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::String(StringNumber {
                value: 6,
                placement: None,
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<string>6</string>"));
    }

    #[test]
    fn test_emit_technical_bend_minimal() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Bend(Bend {
                bend_alter: 1.0,
                pre_bend: false,
                release: None,
                with_bar: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bend>"));
        assert!(xml.contains("<bend-alter>1</bend-alter>"));
        assert!(xml.contains("</bend>"));
    }

    #[test]
    fn test_emit_technical_bend_with_pre_bend() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Bend(Bend {
                bend_alter: 2.0,
                pre_bend: true,
                release: None,
                with_bar: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bend-alter>2</bend-alter>"));
        assert!(xml.contains("<pre-bend/>"));
    }

    #[test]
    fn test_emit_technical_bend_with_release_early() {
        use crate::ir::notation::{BendRelease, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Bend(Bend {
                bend_alter: 1.5,
                pre_bend: false,
                release: Some(BendRelease::Early),
                with_bar: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<release offset=\"early\"/>"));
    }

    #[test]
    fn test_emit_technical_bend_with_release_late() {
        use crate::ir::notation::{BendRelease, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Bend(Bend {
                bend_alter: 1.0,
                pre_bend: false,
                release: Some(BendRelease::Late),
                with_bar: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<release offset=\"late\"/>"));
    }

    #[test]
    fn test_emit_technical_bend_with_bar() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Bend(Bend {
                bend_alter: 1.0,
                pre_bend: false,
                release: None,
                with_bar: Some("whammy".to_string()),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<with-bar>whammy</with-bar>"));
    }

    #[test]
    fn test_emit_technical_bend_all_options() {
        use crate::ir::notation::{BendRelease, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Bend(Bend {
                bend_alter: 2.5,
                pre_bend: true,
                release: Some(BendRelease::Early),
                with_bar: Some("bar".to_string()),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<bend-alter>2.5</bend-alter>"));
        assert!(xml.contains("<pre-bend/>"));
        assert!(xml.contains("<release offset=\"early\"/>"));
        assert!(xml.contains("<with-bar>bar</with-bar>"));
    }

    #[test]
    fn test_emit_technical_tap_minimal() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Tap(Tap {
                value: "".to_string(),
                hand: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tap/>"));
    }

    #[test]
    fn test_emit_technical_tap_with_value() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Tap(Tap {
                value: "T".to_string(),
                hand: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tap>T</tap>"));
    }

    #[test]
    fn test_emit_technical_tap_with_hand_left() {
        use crate::ir::notation::{TapHand, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Tap(Tap {
                value: "".to_string(),
                hand: Some(TapHand::Left),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tap hand=\"left\"/>"));
    }

    #[test]
    fn test_emit_technical_tap_with_hand_right() {
        use crate::ir::notation::{TapHand, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Tap(Tap {
                value: "tap text".to_string(),
                hand: Some(TapHand::Right),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tap hand=\"right\">tap text</tap>"));
    }

    #[test]
    fn test_emit_technical_heel() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Heel(HeelToe {
                substitution: Some(YesNo::Yes),
                placement: Some(AboveBelow::Below),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<heel substitution=\"yes\" placement=\"below\"/>"));
    }

    #[test]
    fn test_emit_technical_toe() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Toe(HeelToe {
                substitution: None,
                placement: Some(AboveBelow::Above),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<toe placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_technical_heel_toe_minimal() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![
                TechnicalElement::Heel(HeelToe::default()),
                TechnicalElement::Toe(HeelToe::default()),
            ],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<heel/>"));
        assert!(xml.contains("<toe/>"));
    }

    #[test]
    fn test_emit_technical_hole_minimal() {
        use crate::ir::notation::{HoleClosed, HoleClosedValue, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Hole(Hole {
                hole_type: None,
                hole_closed: HoleClosed {
                    value: HoleClosedValue::Yes,
                    location: None,
                },
                hole_shape: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<hole>"));
        assert!(xml.contains("<hole-closed>yes</hole-closed>"));
        assert!(xml.contains("</hole>"));
    }

    #[test]
    fn test_emit_technical_hole_with_type() {
        use crate::ir::notation::{HoleClosed, HoleClosedValue, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Hole(Hole {
                hole_type: Some("finger".to_string()),
                hole_closed: HoleClosed {
                    value: HoleClosedValue::No,
                    location: None,
                },
                hole_shape: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<hole-type>finger</hole-type>"));
        assert!(xml.contains("<hole-closed>no</hole-closed>"));
    }

    #[test]
    fn test_emit_technical_hole_half_with_location() {
        use crate::ir::notation::{
            HoleClosed, HoleClosedLocation, HoleClosedValue, TechnicalElement,
        };

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Hole(Hole {
                hole_type: None,
                hole_closed: HoleClosed {
                    value: HoleClosedValue::Half,
                    location: Some(HoleClosedLocation::Right),
                },
                hole_shape: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<hole-closed location=\"right\">half</hole-closed>"));
    }

    #[test]
    fn test_emit_technical_hole_with_shape() {
        use crate::ir::notation::{HoleClosed, HoleClosedValue, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Hole(Hole {
                hole_type: None,
                hole_closed: HoleClosed {
                    value: HoleClosedValue::Yes,
                    location: None,
                },
                hole_shape: Some("round".to_string()),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<hole-shape>round</hole-shape>"));
    }

    #[test]
    fn test_emit_technical_hole_all_attrs() {
        use crate::ir::notation::{
            HoleClosed, HoleClosedLocation, HoleClosedValue, TechnicalElement,
        };

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Hole(Hole {
                hole_type: Some("trill".to_string()),
                hole_closed: HoleClosed {
                    value: HoleClosedValue::Half,
                    location: Some(HoleClosedLocation::Bottom),
                },
                hole_shape: Some("oval".to_string()),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<hole-type>trill</hole-type>"));
        assert!(xml.contains("<hole-closed location=\"bottom\">half</hole-closed>"));
        assert!(xml.contains("<hole-shape>oval</hole-shape>"));
    }

    #[test]
    fn test_emit_technical_hole_closed_locations() {
        use crate::ir::notation::{
            HoleClosed, HoleClosedLocation, HoleClosedValue, TechnicalElement,
        };

        for (location, expected) in [
            (HoleClosedLocation::Right, "right"),
            (HoleClosedLocation::Bottom, "bottom"),
            (HoleClosedLocation::Left, "left"),
            (HoleClosedLocation::Top, "top"),
        ] {
            let mut w = XmlWriter::new();
            let technical = Technical {
                content: vec![TechnicalElement::Hole(Hole {
                    hole_type: None,
                    hole_closed: HoleClosed {
                        value: HoleClosedValue::Half,
                        location: Some(location),
                    },
                    hole_shape: None,
                })],
            };

            emit_technical(&mut w, &technical).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!("location=\"{}\"", expected)),
                "Expected location=\"{}\" in XML: {}",
                expected,
                xml
            );
        }
    }

    #[test]
    fn test_emit_technical_arrow_minimal() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Arrow(Arrow::default())],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arrow/>"));
    }

    #[test]
    fn test_emit_technical_arrow_with_direction() {
        use crate::ir::notation::{ArrowDirection, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Arrow(Arrow {
                direction: Some(ArrowDirection::Up),
                style: None,
                smufl: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arrow>"));
        assert!(xml.contains("<arrow-direction>up</arrow-direction>"));
        assert!(xml.contains("</arrow>"));
    }

    #[test]
    fn test_emit_technical_arrow_with_style() {
        use crate::ir::notation::{ArrowStyle, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Arrow(Arrow {
                direction: None,
                style: Some(ArrowStyle::Double),
                smufl: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arrow-style>double</arrow-style>"));
    }

    #[test]
    fn test_emit_technical_arrow_with_direction_and_style() {
        use crate::ir::notation::{ArrowDirection, ArrowStyle, TechnicalElement};

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Arrow(Arrow {
                direction: Some(ArrowDirection::LeftRight),
                style: Some(ArrowStyle::Filled),
                smufl: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<arrow-direction>left right</arrow-direction>"));
        assert!(xml.contains("<arrow-style>filled</arrow-style>"));
    }

    #[test]
    fn test_emit_technical_arrow_directions() {
        use crate::ir::notation::{ArrowDirection, TechnicalElement};

        let directions = [
            (ArrowDirection::Left, "left"),
            (ArrowDirection::Up, "up"),
            (ArrowDirection::Right, "right"),
            (ArrowDirection::Down, "down"),
            (ArrowDirection::Northwest, "northwest"),
            (ArrowDirection::Northeast, "northeast"),
            (ArrowDirection::Southeast, "southeast"),
            (ArrowDirection::Southwest, "southwest"),
            (ArrowDirection::LeftRight, "left right"),
            (ArrowDirection::UpDown, "up down"),
            (ArrowDirection::NorthwestSoutheast, "northwest southeast"),
            (ArrowDirection::NortheastSouthwest, "northeast southwest"),
            (ArrowDirection::Other, "other"),
        ];

        for (direction, expected) in directions {
            let mut w = XmlWriter::new();
            let technical = Technical {
                content: vec![TechnicalElement::Arrow(Arrow {
                    direction: Some(direction),
                    style: None,
                    smufl: None,
                })],
            };

            emit_technical(&mut w, &technical).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!("<arrow-direction>{}</arrow-direction>", expected)),
                "Expected direction {} in XML: {}",
                expected,
                xml
            );
        }
    }

    #[test]
    fn test_emit_technical_arrow_styles() {
        use crate::ir::notation::{ArrowDirection, ArrowStyle, TechnicalElement};

        let styles = [
            (ArrowStyle::Single, "single"),
            (ArrowStyle::Double, "double"),
            (ArrowStyle::Filled, "filled"),
            (ArrowStyle::Hollow, "hollow"),
            (ArrowStyle::Paired, "paired"),
            (ArrowStyle::Combined, "combined"),
            (ArrowStyle::Other, "other"),
        ];

        for (style, expected) in styles {
            let mut w = XmlWriter::new();
            let technical = Technical {
                content: vec![TechnicalElement::Arrow(Arrow {
                    direction: Some(ArrowDirection::Up),
                    style: Some(style),
                    smufl: None,
                })],
            };

            emit_technical(&mut w, &technical).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!("<arrow-style>{}</arrow-style>", expected)),
                "Expected style {} in XML: {}",
                expected,
                xml
            );
        }
    }

    #[test]
    fn test_emit_technical_handbell_values() {
        use crate::ir::notation::{HandbellValue, TechnicalElement};

        let values = [
            (HandbellValue::Belltree, "belltree"),
            (HandbellValue::Damp, "damp"),
            (HandbellValue::Echo, "echo"),
            (HandbellValue::Gyro, "gyro"),
            (HandbellValue::HandMartellato, "hand martellato"),
            (HandbellValue::MalletLift, "mallet lift"),
            (HandbellValue::MalletTable, "mallet table"),
            (HandbellValue::Martellato, "martellato"),
            (HandbellValue::MartellatoLift, "martellato lift"),
            (HandbellValue::MutedMartellato, "muted martellato"),
            (HandbellValue::PluckLift, "pluck lift"),
            (HandbellValue::Swing, "swing"),
        ];

        for (value, expected) in values {
            let mut w = XmlWriter::new();
            let technical = Technical {
                content: vec![TechnicalElement::Handbell(Handbell { value })],
            };

            emit_technical(&mut w, &technical).unwrap();
            let xml = w.into_string().unwrap();

            assert!(
                xml.contains(&format!("<handbell>{}</handbell>", expected)),
                "Expected handbell {} in XML: {}",
                expected,
                xml
            );
        }
    }

    #[test]
    fn test_emit_technical_harmon_mute_empty() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::HarmonMute(HarmonMute::default())],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<harmon-mute/>"));
    }

    #[test]
    fn test_emit_technical_harmon_mute_open() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::HarmonMute(HarmonMute {
                open: true,
                half: false,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<harmon-mute>"));
        assert!(xml.contains("<harmon-closed>open</harmon-closed>"));
        assert!(xml.contains("</harmon-mute>"));
    }

    #[test]
    fn test_emit_technical_harmon_mute_half() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::HarmonMute(HarmonMute {
                open: false,
                half: true,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<harmon-closed>half</harmon-closed>"));
    }

    #[test]
    fn test_emit_technical_other_technical() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::OtherTechnical(OtherTechnical {
                value: "custom-technical".to_string(),
                placement: Some(AboveBelow::Above),
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(
            xml.contains("<other-technical placement=\"above\">custom-technical</other-technical>")
        );
    }

    #[test]
    fn test_emit_technical_other_technical_no_placement() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::OtherTechnical(OtherTechnical {
                value: "special".to_string(),
                placement: None,
                print_style: PrintStyle::default(),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<other-technical>special</other-technical>"));
    }

    #[test]
    fn test_emit_technical_harmonic_artificial() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Harmonic(Harmonic {
                natural: false,
                artificial: true,
                base_pitch: false,
                touching_pitch: false,
                sounding_pitch: false,
                placement: None,
                print_object: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<harmonic>"));
        assert!(xml.contains("<artificial/>"));
        assert!(xml.contains("</harmonic>"));
    }

    #[test]
    fn test_emit_technical_harmonic_with_pitches() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Harmonic(Harmonic {
                natural: true,
                artificial: false,
                base_pitch: true,
                touching_pitch: true,
                sounding_pitch: true,
                placement: None,
                print_object: None,
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<natural/>"));
        assert!(xml.contains("<base-pitch/>"));
        assert!(xml.contains("<touching-pitch/>"));
        assert!(xml.contains("<sounding-pitch/>"));
    }

    #[test]
    fn test_emit_technical_harmonic_with_print_object() {
        use crate::ir::common::YesNo;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Harmonic(Harmonic {
                natural: true,
                artificial: false,
                base_pitch: false,
                touching_pitch: false,
                sounding_pitch: false,
                placement: Some(AboveBelow::Above),
                print_object: Some(YesNo::No),
            })],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("placement=\"above\""));
        assert!(xml.contains("print-object=\"no\""));
    }

    #[test]
    fn test_emit_technical_harmonic_empty() {
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![TechnicalElement::Harmonic(Harmonic::default())],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<harmonic/>"));
    }

    #[test]
    fn test_emit_technical_many_empty_placements() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![
                TechnicalElement::OpenString(EmptyPlacement::default()),
                TechnicalElement::ThumbPosition(EmptyPlacement::default()),
                TechnicalElement::DoubleTongue(EmptyPlacement::default()),
                TechnicalElement::TripleTongue(EmptyPlacement::default()),
                TechnicalElement::Stopped(EmptyPlacement::default()),
                TechnicalElement::SnapPizzicato(EmptyPlacement::default()),
                TechnicalElement::Fingernails(EmptyPlacement::default()),
                TechnicalElement::BrassBend(EmptyPlacement::default()),
                TechnicalElement::Flip(EmptyPlacement::default()),
                TechnicalElement::Smear(EmptyPlacement::default()),
                TechnicalElement::Open(EmptyPlacement::default()),
                TechnicalElement::HalfMuted(EmptyPlacement::default()),
                TechnicalElement::Golpe(EmptyPlacement::default()),
            ],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<open-string/>"));
        assert!(xml.contains("<thumb-position/>"));
        assert!(xml.contains("<double-tongue/>"));
        assert!(xml.contains("<triple-tongue/>"));
        assert!(xml.contains("<stopped/>"));
        assert!(xml.contains("<snap-pizzicato/>"));
        assert!(xml.contains("<fingernails/>"));
        assert!(xml.contains("<brass-bend/>"));
        assert!(xml.contains("<flip/>"));
        assert!(xml.contains("<smear/>"));
        assert!(xml.contains("<open/>"));
        assert!(xml.contains("<half-muted/>"));
        assert!(xml.contains("<golpe/>"));
    }

    #[test]
    fn test_emit_technical_empty_placements_with_placement() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::TechnicalElement;

        let mut w = XmlWriter::new();
        let technical = Technical {
            content: vec![
                TechnicalElement::OpenString(EmptyPlacement {
                    placement: Some(AboveBelow::Above),
                    position: Position::default(),
                }),
                TechnicalElement::Stopped(EmptyPlacement {
                    placement: Some(AboveBelow::Below),
                    position: Position::default(),
                }),
            ],
        };

        emit_technical(&mut w, &technical).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<open-string placement=\"above\"/>"));
        assert!(xml.contains("<stopped placement=\"below\"/>"));
    }

    // === Accidental mark without placement test ===

    #[test]
    fn test_emit_accidental_mark_no_placement() {
        use crate::ir::PrintStyle;
        use crate::ir::common::AccidentalValue;

        let mut w = XmlWriter::new();
        let notations = Notations {
            print_object: None,
            content: vec![NotationContent::AccidentalMark(AccidentalMark {
                value: AccidentalValue::Natural,
                placement: None,
                print_style: PrintStyle::default(),
            })],
            editorial: Editorial::default(),
        };

        emit_notations(&mut w, &notations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<accidental-mark>natural</accidental-mark>"));
    }

    // === Empty placement without placement test ===

    #[test]
    fn test_emit_empty_placement_no_placement() {
        use crate::ir::common::EmptyPlacement;

        let mut w = XmlWriter::new();
        let ep = EmptyPlacement::default();

        emit_empty_placement(&mut w, "test-element", &ep).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<test-element/>"));
    }

    // === Tied minimal test ===

    #[test]
    fn test_emit_tied_minimal() {
        let mut w = XmlWriter::new();
        let tied = Tied {
            r#type: StartStopContinue::Stop,
            number: None,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        };

        emit_tied(&mut w, &tied).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<tied type=\"stop\"/>"));
    }

    // === Slur minimal test ===

    #[test]
    fn test_emit_slur_minimal() {
        let mut w = XmlWriter::new();
        let slur = Slur {
            r#type: StartStopContinue::Stop,
            number: 1,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        };

        emit_slur(&mut w, &slur).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<slur type=\"stop\" number=\"1\"/>"));
    }

    // === Other ornament without placement ===

    #[test]
    fn test_emit_other_ornament_no_placement() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::OtherOrnament(OtherOrnament {
                    value: "test".to_string(),
                    placement: None,
                    print_style: PrintStyle::default(),
                }),
                accidental_marks: vec![],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<other-ornament>test</other-ornament>"));
    }

    // === Accidental mark in ornaments without placement ===

    #[test]
    fn test_emit_accidental_mark_in_ornaments_no_placement() {
        use crate::ir::PrintStyle;
        use crate::ir::common::AccidentalValue;
        use crate::ir::notation::{OrnamentElement, OrnamentWithAccidentals, Ornaments};

        let mut w = XmlWriter::new();
        let ornaments = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::TrillMark(EmptyTrillSound::default()),
                accidental_marks: vec![AccidentalMark {
                    value: AccidentalValue::DoubleSharp,
                    placement: None,
                    print_style: PrintStyle::default(),
                }],
            }],
        };

        emit_ornaments(&mut w, &ornaments).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<accidental-mark>double-sharp</accidental-mark>"));
    }

    // === Other articulation without placement ===

    #[test]
    fn test_emit_other_articulation_no_placement() {
        use crate::ir::PrintStyle;
        use crate::ir::notation::OtherArticulation;

        let mut w = XmlWriter::new();
        let articulations = Articulations {
            content: vec![ArticulationElement::OtherArticulation(OtherArticulation {
                value: "test-artic".to_string(),
                placement: None,
                print_style: PrintStyle::default(),
            })],
        };

        emit_articulations(&mut w, &articulations).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<other-articulation>test-artic</other-articulation>"));
    }
}
