//! Main MusicXML emission logic.
//!
//! This module contains the functions for emitting MusicXML from IR types.

use crate::ir::attributes::{
    BarStyle, Cancel, CancelLocation, ClefSign, Ending, KeyContent, Mode, Repeat, TimeContent,
    TimeSymbol, Transpose, Winged,
};
use crate::ir::beam::{BeamValue, Fan, NoteheadValue, StemValue};
use crate::ir::common::{
    AboveBelow, AccidentalValue, BackwardForward, LineType, OverUnder, RightLeftMiddle, StartStop,
    StartStopContinue, StartStopDiscontinue, StartStopSingle, UpDown, UprightInverted, YesNo,
};
use crate::ir::direction::{
    Dashes, DirectionType, DirectionTypeContent, DynamicElement, Dynamics, MetronomeContent,
    OctaveShift, Offset, Pedal, PedalType, Sound, UpDownStopContinue, Wedge, WedgeType, Words,
};
use crate::ir::notation::{
    Arpeggiate, ArticulationElement, Articulations, BreathMark, BreathMarkValue, Caesura,
    CaesuraValue, EmptyLine, Fermata, FermataShape, LineLength, LineShape, NonArpeggiate,
    NotationContent, Notations, OtherArticulation, OtherNotation, ShowTuplet, Slur, StrongAccent,
    Tied, TopBottom, Tuplet, TupletPortion,
};
pub use crate::ir::note::{FullNote, Grace, NoteContent, PitchRestUnpitched, Rest, Tie};
use crate::ir::pitch::{Pitch, Step, Unpitched};
use crate::ir::*;
use crate::musicxml::EmitError;
use crate::musicxml::writer::{ElementBuilder, XmlWriter};

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
fn emit_score_header(w: &mut XmlWriter, score: &ScorePartwise) -> Result<(), EmitError> {
    // TODO: work
    // TODO: movement-number
    // TODO: movement-title
    // TODO: identification
    // TODO: defaults
    // TODO: credits

    emit_part_list(w, &score.part_list)?;
    Ok(())
}

/// Emit the part-list element.
fn emit_part_list(w: &mut XmlWriter, part_list: &PartList) -> Result<(), EmitError> {
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
fn emit_score_part(w: &mut XmlWriter, sp: &ScorePart) -> Result<(), EmitError> {
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
fn emit_part_group(w: &mut XmlWriter, _pg: &PartGroup) -> Result<(), EmitError> {
    // TODO: implement part-group emission
    // For now, this is a stub that does nothing
    let _ = w;
    Ok(())
}

/// Emit a part element.
fn emit_part(w: &mut XmlWriter, part: &Part) -> Result<(), EmitError> {
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
fn emit_measure(w: &mut XmlWriter, measure: &Measure) -> Result<(), EmitError> {
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
fn emit_music_data(w: &mut XmlWriter, element: &MusicDataElement) -> Result<(), EmitError> {
    match element {
        MusicDataElement::Note(note) => emit_note(w, note),
        MusicDataElement::Backup(backup) => emit_backup(w, backup),
        MusicDataElement::Forward(forward) => emit_forward(w, forward),
        MusicDataElement::Direction(dir) => emit_direction(w, dir),
        MusicDataElement::Attributes(attrs) => emit_attributes(w, attrs),
        MusicDataElement::Barline(barline) => emit_barline(w, barline),
    }
}

// =============================================================================
// String Converters (public for use in other modules)
// =============================================================================

/// Convert a NoteTypeValue to its MusicXML string representation.
///
/// This is exported publicly as it may be useful for other modules.
pub fn note_type_value_to_string(value: &NoteTypeValue) -> &'static str {
    match value {
        NoteTypeValue::N1024th => "1024th",
        NoteTypeValue::N512th => "512th",
        NoteTypeValue::N256th => "256th",
        NoteTypeValue::N128th => "128th",
        NoteTypeValue::N64th => "64th",
        NoteTypeValue::N32nd => "32nd",
        NoteTypeValue::N16th => "16th",
        NoteTypeValue::Eighth => "eighth",
        NoteTypeValue::Quarter => "quarter",
        NoteTypeValue::Half => "half",
        NoteTypeValue::Whole => "whole",
        NoteTypeValue::Breve => "breve",
        NoteTypeValue::Long => "long",
        NoteTypeValue::Maxima => "maxima",
    }
}

/// Convert a Step to its MusicXML string representation.
fn step_to_string(step: &Step) -> &'static str {
    match step {
        Step::A => "A",
        Step::B => "B",
        Step::C => "C",
        Step::D => "D",
        Step::E => "E",
        Step::F => "F",
        Step::G => "G",
    }
}

/// Convert a Mode to its MusicXML string representation.
fn mode_to_string(mode: &Mode) -> &'static str {
    match mode {
        Mode::Major => "major",
        Mode::Minor => "minor",
        Mode::Dorian => "dorian",
        Mode::Phrygian => "phrygian",
        Mode::Lydian => "lydian",
        Mode::Mixolydian => "mixolydian",
        Mode::Aeolian => "aeolian",
        Mode::Locrian => "locrian",
        Mode::Ionian => "ionian",
        Mode::None => "none",
    }
}

/// Convert a ClefSign to its MusicXML string representation.
fn clef_sign_to_string(sign: &ClefSign) -> &'static str {
    match sign {
        ClefSign::G => "G",
        ClefSign::F => "F",
        ClefSign::C => "C",
        ClefSign::Percussion => "percussion",
        ClefSign::Tab => "TAB",
        ClefSign::Jianpu => "jianpu",
        ClefSign::None => "none",
    }
}

/// Convert a TimeSymbol to its MusicXML string representation.
fn time_symbol_to_string(symbol: &TimeSymbol) -> &'static str {
    match symbol {
        TimeSymbol::Common => "common",
        TimeSymbol::Cut => "cut",
        TimeSymbol::SingleNumber => "single-number",
        TimeSymbol::Note => "note",
        TimeSymbol::DottedNote => "dotted-note",
        TimeSymbol::Normal => "normal",
    }
}

/// Convert a StartStop to its MusicXML string representation.
fn start_stop_to_string(ss: &StartStop) -> &'static str {
    match ss {
        StartStop::Start => "start",
        StartStop::Stop => "stop",
    }
}

/// Convert a BeamValue to its MusicXML string representation.
fn beam_value_to_string(value: &BeamValue) -> &'static str {
    match value {
        BeamValue::Begin => "begin",
        BeamValue::Continue => "continue",
        BeamValue::End => "end",
        BeamValue::ForwardHook => "forward hook",
        BeamValue::BackwardHook => "backward hook",
    }
}

/// Convert a StemValue to its MusicXML string representation.
fn stem_value_to_string(value: &StemValue) -> &'static str {
    match value {
        StemValue::Down => "down",
        StemValue::Up => "up",
        StemValue::Double => "double",
        StemValue::None => "none",
    }
}

/// Convert a NoteheadValue to its MusicXML string representation.
fn notehead_value_to_string(value: &NoteheadValue) -> &'static str {
    match value {
        NoteheadValue::Slash => "slash",
        NoteheadValue::Triangle => "triangle",
        NoteheadValue::Diamond => "diamond",
        NoteheadValue::Square => "square",
        NoteheadValue::Cross => "cross",
        NoteheadValue::X => "x",
        NoteheadValue::CircleX => "circle-x",
        NoteheadValue::InvertedTriangle => "inverted triangle",
        NoteheadValue::ArrowDown => "arrow down",
        NoteheadValue::ArrowUp => "arrow up",
        NoteheadValue::Circled => "circled",
        NoteheadValue::Slashed => "slashed",
        NoteheadValue::BackSlashed => "back slashed",
        NoteheadValue::Normal => "normal",
        NoteheadValue::Cluster => "cluster",
        NoteheadValue::CircleDot => "circle dot",
        NoteheadValue::LeftTriangle => "left triangle",
        NoteheadValue::Rectangle => "rectangle",
        NoteheadValue::None => "none",
        NoteheadValue::Do => "do",
        NoteheadValue::Re => "re",
        NoteheadValue::Mi => "mi",
        NoteheadValue::Fa => "fa",
        NoteheadValue::FaUp => "fa up",
        NoteheadValue::So => "so",
        NoteheadValue::La => "la",
        NoteheadValue::Ti => "ti",
        NoteheadValue::Other => "other",
    }
}

/// Convert an AccidentalValue to its MusicXML string representation.
fn accidental_value_to_string(value: &AccidentalValue) -> &'static str {
    match value {
        AccidentalValue::Sharp => "sharp",
        AccidentalValue::Natural => "natural",
        AccidentalValue::Flat => "flat",
        AccidentalValue::DoubleSharp => "double-sharp",
        AccidentalValue::SharpSharp => "sharp-sharp",
        AccidentalValue::FlatFlat => "flat-flat",
        AccidentalValue::DoubleFlat => "double-flat",
        AccidentalValue::NaturalSharp => "natural-sharp",
        AccidentalValue::NaturalFlat => "natural-flat",
        AccidentalValue::QuarterFlat => "quarter-flat",
        AccidentalValue::QuarterSharp => "quarter-sharp",
        AccidentalValue::ThreeQuartersFlat => "three-quarters-flat",
        AccidentalValue::ThreeQuartersSharp => "three-quarters-sharp",
        AccidentalValue::SharpDown => "sharp-down",
        AccidentalValue::SharpUp => "sharp-up",
        AccidentalValue::NaturalDown => "natural-down",
        AccidentalValue::NaturalUp => "natural-up",
        AccidentalValue::FlatDown => "flat-down",
        AccidentalValue::FlatUp => "flat-up",
        AccidentalValue::TripleSharp => "triple-sharp",
        AccidentalValue::TripleFlat => "triple-flat",
        AccidentalValue::SlashQuarterSharp => "slash-quarter-sharp",
        AccidentalValue::SlashSharp => "slash-sharp",
        AccidentalValue::SlashFlat => "slash-flat",
        AccidentalValue::DoubleSlashFlat => "double-slash-flat",
        AccidentalValue::Sharp1 => "sharp-1",
        AccidentalValue::Sharp2 => "sharp-2",
        AccidentalValue::Sharp3 => "sharp-3",
        AccidentalValue::Sharp5 => "sharp-5",
        AccidentalValue::Flat1 => "flat-1",
        AccidentalValue::Flat2 => "flat-2",
        AccidentalValue::Flat3 => "flat-3",
        AccidentalValue::Flat4 => "flat-4",
        AccidentalValue::Sori => "sori",
        AccidentalValue::Koron => "koron",
        AccidentalValue::Other => "other",
    }
}

/// Convert a YesNo to its MusicXML string representation.
fn yes_no_to_string(yn: &YesNo) -> &'static str {
    match yn {
        YesNo::Yes => "yes",
        YesNo::No => "no",
    }
}

/// Convert a BarStyle to its MusicXML string representation.
fn bar_style_to_string(style: &BarStyle) -> &'static str {
    match style {
        BarStyle::Regular => "regular",
        BarStyle::Dotted => "dotted",
        BarStyle::Dashed => "dashed",
        BarStyle::Heavy => "heavy",
        BarStyle::LightLight => "light-light",
        BarStyle::LightHeavy => "light-heavy",
        BarStyle::HeavyLight => "heavy-light",
        BarStyle::HeavyHeavy => "heavy-heavy",
        BarStyle::Tick => "tick",
        BarStyle::Short => "short",
        BarStyle::None => "none",
    }
}

/// Convert a BackwardForward to its MusicXML string representation.
fn backward_forward_to_string(bf: &BackwardForward) -> &'static str {
    match bf {
        BackwardForward::Backward => "backward",
        BackwardForward::Forward => "forward",
    }
}

/// Convert a RightLeftMiddle to its MusicXML string representation.
fn right_left_middle_to_string(rlm: &RightLeftMiddle) -> &'static str {
    match rlm {
        RightLeftMiddle::Right => "right",
        RightLeftMiddle::Left => "left",
        RightLeftMiddle::Middle => "middle",
    }
}

/// Convert a StartStopDiscontinue to its MusicXML string representation.
fn start_stop_discontinue_to_string(ssd: &StartStopDiscontinue) -> &'static str {
    match ssd {
        StartStopDiscontinue::Start => "start",
        StartStopDiscontinue::Stop => "stop",
        StartStopDiscontinue::Discontinue => "discontinue",
    }
}

/// Convert a Winged to its MusicXML string representation.
fn winged_to_string(winged: &Winged) -> &'static str {
    match winged {
        Winged::None => "none",
        Winged::Straight => "straight",
        Winged::Curved => "curved",
        Winged::DoubleStraight => "double-straight",
        Winged::DoubleCurved => "double-curved",
    }
}

/// Convert an UprightInverted to its MusicXML string representation.
fn upright_inverted_to_string(ui: &UprightInverted) -> &'static str {
    match ui {
        UprightInverted::Upright => "upright",
        UprightInverted::Inverted => "inverted",
    }
}

/// Convert a FermataShape to its MusicXML string representation.
fn fermata_shape_to_string(shape: &FermataShape) -> &'static str {
    match shape {
        FermataShape::Normal => "normal",
        FermataShape::Angled => "angled",
        FermataShape::Square => "square",
        FermataShape::DoubleAngled => "double-angled",
        FermataShape::DoubleSquare => "double-square",
        FermataShape::DoubleDot => "double-dot",
        FermataShape::HalfCurve => "half-curve",
        FermataShape::Curlew => "curlew",
    }
}

/// Convert a CancelLocation to its MusicXML string representation.
fn cancel_location_to_string(loc: &CancelLocation) -> &'static str {
    match loc {
        CancelLocation::Left => "left",
        CancelLocation::Right => "right",
        CancelLocation::BeforeBarline => "before-barline",
    }
}

/// Convert a Fan to its MusicXML string representation.
fn fan_to_string(fan: &Fan) -> &'static str {
    match fan {
        Fan::Accel => "accel",
        Fan::Rit => "rit",
        Fan::None => "none",
    }
}

/// Convert an AboveBelow to its MusicXML string representation.
fn above_below_to_string(ab: &AboveBelow) -> &'static str {
    match ab {
        AboveBelow::Above => "above",
        AboveBelow::Below => "below",
    }
}

/// Convert a StartStopContinue to its MusicXML string representation.
fn start_stop_continue_to_string(ssc: &StartStopContinue) -> &'static str {
    match ssc {
        StartStopContinue::Start => "start",
        StartStopContinue::Stop => "stop",
        StartStopContinue::Continue => "continue",
    }
}

/// Convert a WedgeType to its MusicXML string representation.
fn wedge_type_to_string(wt: &WedgeType) -> &'static str {
    match wt {
        WedgeType::Crescendo => "crescendo",
        WedgeType::Diminuendo => "diminuendo",
        WedgeType::Stop => "stop",
        WedgeType::Continue => "continue",
    }
}

/// Convert a LineType to its MusicXML string representation.
fn line_type_to_string(lt: &LineType) -> &'static str {
    match lt {
        LineType::Solid => "solid",
        LineType::Dashed => "dashed",
        LineType::Dotted => "dotted",
        LineType::Wavy => "wavy",
    }
}

/// Convert an UpDown to its MusicXML string representation.
fn up_down_to_string(ud: &UpDown) -> &'static str {
    match ud {
        UpDown::Up => "up",
        UpDown::Down => "down",
    }
}

/// Convert a TopBottom to its MusicXML string representation.
fn top_bottom_to_string(tb: &TopBottom) -> &'static str {
    match tb {
        TopBottom::Top => "top",
        TopBottom::Bottom => "bottom",
    }
}

/// Convert a ShowTuplet to its MusicXML string representation.
fn show_tuplet_to_string(st: &ShowTuplet) -> &'static str {
    match st {
        ShowTuplet::Actual => "actual",
        ShowTuplet::Both => "both",
        ShowTuplet::None => "none",
    }
}

/// Convert a LineShape to its MusicXML string representation.
fn line_shape_to_string(ls: &LineShape) -> &'static str {
    match ls {
        LineShape::Straight => "straight",
        LineShape::Curved => "curved",
    }
}

/// Convert a PedalType to its MusicXML string representation.
fn pedal_type_to_string(pt: &PedalType) -> &'static str {
    match pt {
        PedalType::Start => "start",
        PedalType::Stop => "stop",
        PedalType::Sostenuto => "sostenuto",
        PedalType::Change => "change",
        PedalType::Continue => "continue",
        PedalType::Discontinue => "discontinue",
        PedalType::Resume => "resume",
    }
}

/// Convert an UpDownStopContinue to its MusicXML string representation.
fn up_down_stop_continue_to_string(udsc: &UpDownStopContinue) -> &'static str {
    match udsc {
        UpDownStopContinue::Up => "up",
        UpDownStopContinue::Down => "down",
        UpDownStopContinue::Stop => "stop",
        UpDownStopContinue::Continue => "continue",
    }
}

/// Convert a BreathMarkValue to its MusicXML string representation.
fn breath_mark_value_to_string(bmv: &BreathMarkValue) -> &'static str {
    match bmv {
        BreathMarkValue::Empty => "",
        BreathMarkValue::Comma => "comma",
        BreathMarkValue::Tick => "tick",
        BreathMarkValue::Upbow => "upbow",
        BreathMarkValue::Salzedo => "salzedo",
    }
}

/// Convert a CaesuraValue to its MusicXML string representation.
fn caesura_value_to_string(cv: &CaesuraValue) -> &'static str {
    match cv {
        CaesuraValue::Normal => "normal",
        CaesuraValue::Thick => "thick",
        CaesuraValue::Short => "short",
        CaesuraValue::Curved => "curved",
        CaesuraValue::Single => "single",
    }
}

/// Convert a LineLength to its MusicXML string representation.
fn line_length_to_string(ll: &LineLength) -> &'static str {
    match ll {
        LineLength::Short => "short",
        LineLength::Medium => "medium",
        LineLength::Long => "long",
    }
}

/// Convert a StartStopSingle to its MusicXML string representation.
fn start_stop_single_to_string(sss: &StartStopSingle) -> &'static str {
    match sss {
        StartStopSingle::Start => "start",
        StartStopSingle::Stop => "stop",
        StartStopSingle::Single => "single",
    }
}

// =============================================================================
// Attributes Emission (Task 2.1)
// =============================================================================

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
fn emit_attributes(w: &mut XmlWriter, attrs: &Attributes) -> Result<(), EmitError> {
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
fn emit_key(w: &mut XmlWriter, key: &Key) -> Result<(), EmitError> {
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
fn emit_cancel(w: &mut XmlWriter, cancel: &Cancel) -> Result<(), EmitError> {
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
fn emit_time(w: &mut XmlWriter, time: &Time) -> Result<(), EmitError> {
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
fn emit_clef(w: &mut XmlWriter, clef: &Clef) -> Result<(), EmitError> {
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
fn emit_transpose(w: &mut XmlWriter, transpose: &Transpose) -> Result<(), EmitError> {
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
        if *double == YesNo::Yes {
            w.empty_element("double")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
    }

    w.end_element("transpose")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

// =============================================================================
// Note Emission (Tasks 2.2, 2.3, 2.4)
// =============================================================================

/// Emit a note element.
///
/// Elements are emitted in XSD order:
/// 1. grace OR cue OR (chord?, pitch/rest/unpitched)
/// 2. duration (for regular and cue notes)
/// 3. tie*
/// 4. instrument*
/// 5. editorial-voice (footnote, level, voice)
/// 6. type
/// 7. dot*
/// 8. accidental
/// 9. time-modification
/// 10. stem
/// 11. notehead
/// 12. staff
/// 13. beam* (0-8)
/// 14. notations*
/// 15. lyric*
fn emit_note(w: &mut XmlWriter, note: &Note) -> Result<(), EmitError> {
    w.start_element("note")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // Handle the three content variants
    match &note.content {
        NoteContent::Grace {
            grace,
            full_note,
            ties,
        } => {
            emit_grace(w, grace)?;
            emit_full_note(w, full_note)?;
            for tie in ties {
                emit_tie(w, tie)?;
            }
        }
        NoteContent::Cue {
            full_note,
            duration,
        } => {
            w.empty_element("cue")
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            emit_full_note(w, full_note)?;
            w.text_element("duration", &duration.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        NoteContent::Regular {
            full_note,
            duration,
            ties,
        } => {
            emit_full_note(w, full_note)?;
            w.text_element("duration", &duration.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
            for tie in ties {
                emit_tie(w, tie)?;
            }
        }
    }

    // instrument*
    for inst in &note.instrument {
        let elem = ElementBuilder::new("instrument").attr("id", &inst.id);
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // voice
    if let Some(ref voice) = note.voice {
        w.text_element("voice", voice)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // type
    if let Some(ref note_type) = note.r#type {
        let elem = ElementBuilder::new("type");
        // size attribute if present
        if let Some(ref _size) = note_type.size {
            // size attribute would go here if needed
        }
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.write_text(note_type_value_to_string(&note_type.value))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        w.end_element("type")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // dot*
    for _dot in &note.dots {
        w.empty_element("dot")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // accidental
    if let Some(ref acc) = note.accidental {
        emit_accidental(w, acc)?;
    }

    // time-modification
    if let Some(ref tm) = note.time_modification {
        emit_time_modification(w, tm)?;
    }

    // stem
    if let Some(ref stem) = note.stem {
        emit_stem(w, stem)?;
    }

    // notehead
    if let Some(ref notehead) = note.notehead {
        emit_notehead(w, notehead)?;
    }

    // staff
    if let Some(staff) = note.staff {
        w.text_element("staff", &staff.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // beam* (up to 8 levels)
    for beam in &note.beams {
        emit_beam(w, beam)?;
    }

    // notations* - emit each Notations container
    for notations in &note.notations {
        emit_notations(w, notations)?;
    }

    // lyric* - skipped for now (Milestone 5)

    w.end_element("note")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a grace element.
fn emit_grace(w: &mut XmlWriter, grace: &Grace) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("grace");

    if let Some(ref slash) = grace.slash {
        elem = elem.attr("slash", yes_no_to_string(slash));
    }
    if let Some(stp) = grace.steal_time_previous {
        elem = elem.attr("steal-time-previous", &stp.to_string());
    }
    if let Some(stf) = grace.steal_time_following {
        elem = elem.attr("steal-time-following", &stf.to_string());
    }
    if let Some(mt) = grace.make_time {
        elem = elem.attr("make-time", &mt.to_string());
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit the full note content (chord flag + pitch/rest/unpitched).
fn emit_full_note(w: &mut XmlWriter, full_note: &FullNote) -> Result<(), EmitError> {
    // chord flag
    if full_note.chord {
        w.empty_element("chord")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // pitch, rest, or unpitched
    match &full_note.content {
        PitchRestUnpitched::Pitch(pitch) => emit_pitch(w, pitch)?,
        PitchRestUnpitched::Rest(rest) => emit_rest(w, rest)?,
        PitchRestUnpitched::Unpitched(unpitched) => emit_unpitched(w, unpitched)?,
    }

    Ok(())
}

/// Emit a pitch element.
fn emit_pitch(w: &mut XmlWriter, pitch: &Pitch) -> Result<(), EmitError> {
    w.start_element("pitch")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // step (required)
    w.text_element("step", step_to_string(&pitch.step))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // alter
    if let Some(alter) = pitch.alter {
        w.text_element("alter", &alter.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // octave (required)
    w.text_element("octave", &pitch.octave.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    w.end_element("pitch")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a rest element.
fn emit_rest(w: &mut XmlWriter, rest: &Rest) -> Result<(), EmitError> {
    let has_content = rest.display_step.is_some() || rest.display_octave.is_some();

    let mut elem = ElementBuilder::new("rest");
    if let Some(ref measure) = rest.measure {
        elem = elem.attr("measure", yes_no_to_string(measure));
    }

    if has_content {
        w.write_start(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        if let Some(ref step) = rest.display_step {
            w.text_element("display-step", step_to_string(step))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if let Some(octave) = rest.display_octave {
            w.text_element("display-octave", &octave.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        w.end_element("rest")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element_with_attrs(elem)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    Ok(())
}

/// Emit an unpitched element.
fn emit_unpitched(w: &mut XmlWriter, unpitched: &Unpitched) -> Result<(), EmitError> {
    let has_content = unpitched.display_step.is_some() || unpitched.display_octave.is_some();

    if has_content {
        w.start_element("unpitched")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        if let Some(ref step) = unpitched.display_step {
            w.text_element("display-step", step_to_string(step))
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        if let Some(octave) = unpitched.display_octave {
            w.text_element("display-octave", &octave.to_string())
                .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
        }
        w.end_element("unpitched")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    } else {
        w.empty_element("unpitched")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    Ok(())
}

/// Emit a tie element (playback, not visual).
fn emit_tie(w: &mut XmlWriter, tie: &Tie) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("tie").attr("type", start_stop_to_string(&tie.r#type));
    if let Some(ref time_only) = tie.time_only {
        elem = elem.attr("time-only", time_only);
    }
    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit an accidental element.
fn emit_accidental(w: &mut XmlWriter, acc: &Accidental) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("accidental");
    if let Some(ref cautionary) = acc.cautionary {
        elem = elem.attr("cautionary", yes_no_to_string(cautionary));
    }
    if let Some(ref editorial) = acc.editorial {
        elem = elem.attr("editorial", yes_no_to_string(editorial));
    }
    if let Some(ref parentheses) = acc.parentheses {
        elem = elem.attr("parentheses", yes_no_to_string(parentheses));
    }
    if let Some(ref bracket) = acc.bracket {
        elem = elem.attr("bracket", yes_no_to_string(bracket));
    }

    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(accidental_value_to_string(&acc.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("accidental")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a time-modification element (for tuplets).
fn emit_time_modification(w: &mut XmlWriter, tm: &TimeModification) -> Result<(), EmitError> {
    w.start_element("time-modification")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // actual-notes (required)
    w.text_element("actual-notes", &tm.actual_notes.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // normal-notes (required)
    w.text_element("normal-notes", &tm.normal_notes.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // normal-type
    if let Some(ref nt) = tm.normal_type {
        w.text_element("normal-type", note_type_value_to_string(nt))
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // normal-dot*
    for _ in 0..tm.normal_dots {
        w.empty_element("normal-dot")
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("time-modification")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a stem element.
fn emit_stem(w: &mut XmlWriter, stem: &Stem) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("stem");
    if let Some(dy) = stem.default_y {
        elem = elem.attr("default-y", &dy.to_string());
    }
    if let Some(ref color) = stem.color {
        elem = elem.attr("color", color);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(stem_value_to_string(&stem.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("stem")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a notehead element.
fn emit_notehead(w: &mut XmlWriter, notehead: &Notehead) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("notehead");
    if let Some(ref filled) = notehead.filled {
        elem = elem.attr("filled", yes_no_to_string(filled));
    }
    if let Some(ref parentheses) = notehead.parentheses {
        elem = elem.attr("parentheses", yes_no_to_string(parentheses));
    }
    if let Some(ref color) = notehead.color {
        elem = elem.attr("color", color);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(notehead_value_to_string(&notehead.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("notehead")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a beam element.
fn emit_beam(w: &mut XmlWriter, beam: &Beam) -> Result<(), EmitError> {
    let mut elem = ElementBuilder::new("beam").attr("number", &beam.number.to_string());
    if let Some(ref fan) = beam.fan {
        elem = elem.attr("fan", fan_to_string(fan));
    }
    if let Some(ref color) = beam.color {
        elem = elem.attr("color", color);
    }
    w.write_start(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.write_text(beam_value_to_string(&beam.value))
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    w.end_element("beam")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

// =============================================================================
// Backup and Forward (Task 3.1)
// =============================================================================

/// Emit a backup element.
fn emit_backup(w: &mut XmlWriter, backup: &Backup) -> Result<(), EmitError> {
    w.start_element("backup")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // duration (required)
    w.text_element("duration", &backup.duration.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // editorial elements (footnote, level) - skipped for now

    w.end_element("backup")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a forward element.
fn emit_forward(w: &mut XmlWriter, forward: &Forward) -> Result<(), EmitError> {
    w.start_element("forward")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // duration (required)
    w.text_element("duration", &forward.duration.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // editorial elements (footnote, level) - skipped for now

    // voice
    if let Some(ref voice) = forward.voice {
        w.text_element("voice", voice)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // staff
    if let Some(staff) = forward.staff {
        w.text_element("staff", &staff.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("forward")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

// =============================================================================
// Barline (Task 3.2)
// =============================================================================

/// Emit a barline element.
fn emit_barline(w: &mut XmlWriter, barline: &Barline) -> Result<(), EmitError> {
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
fn emit_fermata(w: &mut XmlWriter, fermata: &Fermata) -> Result<(), EmitError> {
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
fn emit_ending(w: &mut XmlWriter, ending: &Ending) -> Result<(), EmitError> {
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
fn emit_repeat(w: &mut XmlWriter, repeat: &Repeat) -> Result<(), EmitError> {
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

// =============================================================================
// Direction (Milestone 4 - Tasks 4.1-4.4)
// =============================================================================

/// Emit a direction element.
///
/// Direction elements contain direction-type+ (required), offset?, voice?, staff?, sound?
fn emit_direction(w: &mut XmlWriter, dir: &Direction) -> Result<(), EmitError> {
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
fn emit_offset(w: &mut XmlWriter, offset: &Offset) -> Result<(), EmitError> {
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
fn emit_sound(w: &mut XmlWriter, sound: &Sound) -> Result<(), EmitError> {
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
fn emit_direction_type(w: &mut XmlWriter, dt: &DirectionType) -> Result<(), EmitError> {
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
fn emit_segno(w: &mut XmlWriter) -> Result<(), EmitError> {
    w.empty_element("segno")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a coda element.
fn emit_coda(w: &mut XmlWriter) -> Result<(), EmitError> {
    w.empty_element("coda")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a words element.
fn emit_words(w: &mut XmlWriter, words: &Words) -> Result<(), EmitError> {
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
fn emit_dynamics(w: &mut XmlWriter, dynamics: &Dynamics) -> Result<(), EmitError> {
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
fn emit_wedge(w: &mut XmlWriter, wedge: &Wedge) -> Result<(), EmitError> {
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
fn emit_dashes(w: &mut XmlWriter, dashes: &Dashes) -> Result<(), EmitError> {
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
fn emit_pedal(w: &mut XmlWriter, pedal: &Pedal) -> Result<(), EmitError> {
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
fn emit_metronome(
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
fn emit_octave_shift(w: &mut XmlWriter, octave_shift: &OctaveShift) -> Result<(), EmitError> {
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

// =============================================================================
// Notations (Milestone 4 - Tasks 4.5-4.6)
// =============================================================================

/// Emit a notations element (Task 4.5).
///
/// Container for note-attached notations: tied, slur, tuplet, articulations,
/// ornaments, technical, dynamics, fermata, arpeggiate, etc.
fn emit_notations(w: &mut XmlWriter, notations: &Notations) -> Result<(), EmitError> {
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
fn emit_tied(w: &mut XmlWriter, tied: &Tied) -> Result<(), EmitError> {
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
        elem = elem.attr(
            "orientation",
            match orientation {
                OverUnder::Over => "over",
                OverUnder::Under => "under",
            },
        );
    }
    if let Some(ref color) = tied.color {
        elem = elem.attr("color", color);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a slur element.
fn emit_slur(w: &mut XmlWriter, slur: &Slur) -> Result<(), EmitError> {
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
        elem = elem.attr(
            "orientation",
            match orientation {
                OverUnder::Over => "over",
                OverUnder::Under => "under",
            },
        );
    }
    if let Some(ref color) = slur.color {
        elem = elem.attr("color", color);
    }

    w.empty_element_with_attrs(elem)
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a tuplet element.
fn emit_tuplet(w: &mut XmlWriter, tuplet: &Tuplet) -> Result<(), EmitError> {
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
fn emit_tuplet_portion(
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
fn emit_arpeggiate(w: &mut XmlWriter, arpeggiate: &Arpeggiate) -> Result<(), EmitError> {
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
fn emit_non_arpeggiate(w: &mut XmlWriter, non_arpeggiate: &NonArpeggiate) -> Result<(), EmitError> {
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
fn emit_other_notation(w: &mut XmlWriter, other: &OtherNotation) -> Result<(), EmitError> {
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
fn emit_articulations(w: &mut XmlWriter, articulations: &Articulations) -> Result<(), EmitError> {
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
fn emit_empty_placement(
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
fn emit_strong_accent(w: &mut XmlWriter, sa: &StrongAccent) -> Result<(), EmitError> {
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
fn emit_empty_line(w: &mut XmlWriter, name: &str, el: &EmptyLine) -> Result<(), EmitError> {
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
fn emit_breath_mark(w: &mut XmlWriter, bm: &BreathMark) -> Result<(), EmitError> {
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
fn emit_caesura(w: &mut XmlWriter, c: &Caesura) -> Result<(), EmitError> {
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
fn emit_other_articulation(w: &mut XmlWriter, oa: &OtherArticulation) -> Result<(), EmitError> {
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
    use crate::ir::attributes::{
        Barline, ClefSign, KeyContent, TimeContent, TimeSignature, TraditionalKey,
    };
    use crate::ir::beam::{Beam, BeamValue, Notehead, NoteheadValue, Stem, StemValue};
    use crate::ir::common::{
        AboveBelow, Editorial, LineType, Position, StartStop, StartStopContinue,
        StartStopDiscontinue, UpDown, YesNo,
    };
    use crate::ir::duration::{Dot, NoteType, NoteTypeValue, TimeModification};
    use crate::ir::note::{
        Accidental, FullNote, NoteContent, PitchRestUnpitched, Rest as NoteRest, Tie,
    };
    use crate::ir::part::{PartList, PartListElement, PartName, ScorePart};
    use crate::ir::pitch::{Pitch, Step};
    use crate::ir::voice::{Backup, Forward};

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

    // =======================================================================
    // Milestone 2 Tests: Core Note & Attributes
    // =======================================================================

    #[test]
    fn test_emit_attributes_with_divisions_key_time_clef() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Attributes(Box::new(Attributes {
                editorial: Editorial::default(),
                divisions: Some(4),
                keys: vec![Key {
                    content: KeyContent::Traditional(TraditionalKey {
                        cancel: None,
                        fifths: 0, // C major
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
        assert!(xml.contains("<beats>4</beats>"));
        assert!(xml.contains("<beat-type>4</beat-type>"));
        assert!(xml.contains("<sign>G</sign>"));
        assert!(xml.contains("<line>2</line>"));
        assert!(xml.contains("</attributes>"));
    }

    #[test]
    fn test_emit_note_c4_quarter() {
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
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<note>"));
        assert!(xml.contains("<pitch>"));
        assert!(xml.contains("<step>C</step>"));
        assert!(xml.contains("<octave>4</octave>"));
        assert!(xml.contains("</pitch>"));
        assert!(xml.contains("<duration>4</duration>"));
        assert!(xml.contains("<voice>1</voice>"));
        assert!(xml.contains("<type>quarter</type>"));
        assert!(xml.contains("</note>"));
    }

    #[test]
    fn test_emit_note_with_accidental() {
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
                            step: Step::F,
                            alter: Some(1.0),
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
                accidental: Some(Accidental {
                    value: AccidentalValue::Sharp,
                    cautionary: None,
                    editorial: None,
                    parentheses: None,
                    bracket: None,
                    size: None,
                }),
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<step>F</step>"));
        assert!(xml.contains("<alter>1</alter>"));
        assert!(xml.contains("<accidental>sharp</accidental>"));
    }

    #[test]
    fn test_emit_note_with_tie() {
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
                    ties: vec![Tie {
                        r#type: StartStop::Start,
                        time_only: None,
                    }],
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<tie type=\"start\"/>"));
    }

    #[test]
    fn test_emit_note_with_triplet_time_modification() {
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
                    duration: 2, // triplet eighth in 4/4
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Eighth,
                    size: None,
                }),
                dots: vec![],
                accidental: None,
                time_modification: Some(TimeModification {
                    actual_notes: 3,
                    normal_notes: 2,
                    normal_type: Some(NoteTypeValue::Eighth),
                    normal_dots: 0,
                }),
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<time-modification>"));
        assert!(xml.contains("<actual-notes>3</actual-notes>"));
        assert!(xml.contains("<normal-notes>2</normal-notes>"));
        assert!(xml.contains("<normal-type>eighth</normal-type>"));
        assert!(xml.contains("</time-modification>"));
    }

    #[test]
    fn test_emit_note_with_beam() {
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
                    duration: 2,
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Eighth,
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
                beams: vec![Beam {
                    value: BeamValue::Begin,
                    number: 1,
                    fan: None,
                    color: None,
                }],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<stem>up</stem>"));
        assert!(xml.contains("<beam number=\"1\">begin</beam>"));
    }

    #[test]
    fn test_emit_note_with_notehead() {
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
                notehead: Some(Notehead {
                    value: NoteheadValue::Diamond,
                    filled: Some(YesNo::Yes),
                    parentheses: None,
                    font: crate::ir::common::Font::default(),
                    color: None,
                }),
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<notehead filled=\"yes\">diamond</notehead>"));
    }

    #[test]
    fn test_emit_rest() {
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
                        content: PitchRestUnpitched::Rest(NoteRest {
                            measure: None,
                            display_step: None,
                            display_octave: None,
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<rest/>"));
        assert!(xml.contains("<duration>4</duration>"));
        assert!(xml.contains("<type>quarter</type>"));
    }

    #[test]
    fn test_emit_dotted_note() {
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
                    duration: 6, // dotted quarter in 4/4 with divisions=4
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Quarter,
                    size: None,
                }),
                dots: vec![Dot::default()],
                accidental: None,
                time_modification: None,
                stem: None,
                notehead: None,
                staff: None,
                beams: vec![],
                notations: vec![],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<type>quarter</type>"));
        assert!(xml.contains("<dot/>"));
    }

    #[test]
    fn test_emit_grace_note() {
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
                content: NoteContent::Grace {
                    grace: Grace {
                        steal_time_previous: None,
                        steal_time_following: None,
                        make_time: None,
                        slash: Some(YesNo::Yes),
                    },
                    full_note: FullNote {
                        chord: false,
                        content: PitchRestUnpitched::Pitch(Pitch {
                            step: Step::D,
                            alter: None,
                            octave: 4,
                        }),
                    },
                    ties: vec![],
                },
                instrument: vec![],
                voice: Some("1".to_string()),
                r#type: Some(NoteType {
                    value: NoteTypeValue::Eighth,
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

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<grace slash=\"yes\"/>"));
        assert!(xml.contains("<step>D</step>"));
        // Grace notes have no duration
        assert!(!xml.contains("<duration>"));
    }

    // =======================================================================
    // Milestone 2.5 Test: "Twinkle Twinkle" Integration Test
    // =======================================================================

    #[test]
    fn test_emit_twinkle_twinkle_first_phrase() {
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

    // =======================================================================
    // Milestone 3 Tests: Voice, Barlines, Navigation
    // =======================================================================

    #[test]
    fn test_emit_backup() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Backup(Backup {
                duration: 4,
                editorial: Editorial::default(),
            }));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<backup>"));
        assert!(xml.contains("<duration>4</duration>"));
        assert!(xml.contains("</backup>"));
    }

    #[test]
    fn test_emit_forward() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Forward(Forward {
                duration: 2,
                voice: Some("2".to_string()),
                staff: Some(1),
                editorial: Editorial::default(),
            }));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<forward>"));
        assert!(xml.contains("<duration>2</duration>"));
        assert!(xml.contains("<voice>2</voice>"));
        assert!(xml.contains("<staff>1</staff>"));
        assert!(xml.contains("</forward>"));
    }

    #[test]
    fn test_emit_barline_with_repeat() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Barline(Box::new(Barline {
                location: Some(RightLeftMiddle::Right),
                bar_style: Some(BarStyle::LightHeavy),
                editorial: Editorial::default(),
                wavy_line: None,
                segno: None,
                coda: None,
                fermatas: vec![],
                ending: None,
                repeat: Some(crate::ir::attributes::Repeat {
                    direction: BackwardForward::Backward,
                    times: Some(2),
                    winged: None,
                }),
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<barline location=\"right\">"));
        assert!(xml.contains("<bar-style>light-heavy</bar-style>"));
        assert!(xml.contains("<repeat direction=\"backward\" times=\"2\"/>"));
        assert!(xml.contains("</barline>"));
    }

    #[test]
    fn test_emit_barline_with_ending() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Barline(Box::new(Barline {
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<barline location=\"left\">"));
        assert!(xml.contains("<ending number=\"1\" type=\"start\">1.</ending>"));
        assert!(xml.contains("</barline>"));
    }

    #[test]
    fn test_emit_barline_with_fermata() {
        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Barline(Box::new(Barline {
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<fermata type=\"upright\">normal</fermata>"));
    }

    // =======================================================================
    // Milestone 3.3 Test: Multi-Voice Test
    // =======================================================================

    #[test]
    fn test_emit_multi_voice_with_backup() {
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

    // =======================================================================
    // Milestone 3.4 Test: Repeat/Volta Test
    // =======================================================================

    #[test]
    fn test_emit_repeat_with_volta_brackets() {
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
                repeat: Some(crate::ir::attributes::Repeat {
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

        // Measure 2: First ending
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
                    repeat: Some(crate::ir::attributes::Repeat {
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

    // =======================================================================
    // String Converter Tests
    // =======================================================================

    #[test]
    fn test_note_type_value_to_string() {
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Whole), "whole");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Half), "half");
        assert_eq!(
            note_type_value_to_string(&NoteTypeValue::Quarter),
            "quarter"
        );
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Eighth), "eighth");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N16th), "16th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N32nd), "32nd");
    }

    #[test]
    fn test_step_to_string() {
        assert_eq!(step_to_string(&Step::C), "C");
        assert_eq!(step_to_string(&Step::D), "D");
        assert_eq!(step_to_string(&Step::E), "E");
        assert_eq!(step_to_string(&Step::F), "F");
        assert_eq!(step_to_string(&Step::G), "G");
        assert_eq!(step_to_string(&Step::A), "A");
        assert_eq!(step_to_string(&Step::B), "B");
    }

    #[test]
    fn test_mode_to_string() {
        assert_eq!(mode_to_string(&Mode::Major), "major");
        assert_eq!(mode_to_string(&Mode::Minor), "minor");
        assert_eq!(mode_to_string(&Mode::Dorian), "dorian");
    }

    #[test]
    fn test_clef_sign_to_string() {
        assert_eq!(clef_sign_to_string(&ClefSign::G), "G");
        assert_eq!(clef_sign_to_string(&ClefSign::F), "F");
        assert_eq!(clef_sign_to_string(&ClefSign::C), "C");
        assert_eq!(clef_sign_to_string(&ClefSign::Percussion), "percussion");
        assert_eq!(clef_sign_to_string(&ClefSign::Tab), "TAB");
    }

    #[test]
    fn test_bar_style_to_string() {
        assert_eq!(bar_style_to_string(&BarStyle::Regular), "regular");
        assert_eq!(bar_style_to_string(&BarStyle::LightHeavy), "light-heavy");
        assert_eq!(bar_style_to_string(&BarStyle::HeavyLight), "heavy-light");
    }

    #[test]
    fn test_accidental_value_to_string() {
        assert_eq!(accidental_value_to_string(&AccidentalValue::Sharp), "sharp");
        assert_eq!(accidental_value_to_string(&AccidentalValue::Flat), "flat");
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Natural),
            "natural"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::DoubleSharp),
            "double-sharp"
        );
    }

    // =======================================================================
    // Milestone 4 Tests: Directions and Notations
    // =======================================================================

    // === Task 4.1: Direction Container Tests ===

    #[test]
    fn test_emit_direction_basic() {
        use crate::ir::direction::{Direction, DirectionType, DirectionTypeContent, Words};

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<direction placement=\"above\">"));
        assert!(xml.contains("<direction-type>"));
        assert!(xml.contains("<words>cresc.</words>"));
        assert!(xml.contains("</direction-type>"));
        assert!(xml.contains("</direction>"));
    }

    #[test]
    fn test_emit_direction_with_voice_and_staff() {
        use crate::ir::direction::{Direction, DirectionType, DirectionTypeContent, Words};

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<direction placement=\"below\" directive=\"yes\">"));
        assert!(xml.contains("<voice>1</voice>"));
        assert!(xml.contains("<staff>1</staff>"));
    }

    #[test]
    fn test_emit_direction_with_sound() {
        use crate::ir::direction::{Direction, DirectionType, DirectionTypeContent, Sound, Words};

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
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
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<sound tempo=\"120\" dynamics=\"80\"/>"));
    }

    // === Task 4.2: Dynamics Tests ===

    #[test]
    fn test_emit_dynamics_forte() {
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics,
        };

        let mut score = create_minimal_score();
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

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<dynamics>"));
        assert!(xml.contains("<f/>"));
        assert!(xml.contains("</dynamics>"));
    }

    #[test]
    fn test_emit_dynamics_all_piano_levels() {
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
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
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<p/>"));
        assert!(xml.contains("<pp/>"));
        assert!(xml.contains("<ppp/>"));
        assert!(xml.contains("<pppp/>"));
        assert!(xml.contains("<ppppp/>"));
        assert!(xml.contains("<pppppp/>"));
    }

    #[test]
    fn test_emit_dynamics_all_forte_levels() {
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
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
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<f/>"));
        assert!(xml.contains("<ff/>"));
        assert!(xml.contains("<fff/>"));
        assert!(xml.contains("<ffff/>"));
        assert!(xml.contains("<fffff/>"));
        assert!(xml.contains("<ffffff/>"));
    }

    #[test]
    fn test_emit_dynamics_compound() {
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![
                            DynamicElement::MP,
                            DynamicElement::MF,
                            DynamicElement::SF,
                            DynamicElement::SFP,
                            DynamicElement::SFPP,
                            DynamicElement::FP,
                            DynamicElement::RF,
                            DynamicElement::RFZ,
                            DynamicElement::SFZ,
                            DynamicElement::SFFZ,
                            DynamicElement::FZ,
                            DynamicElement::N,
                            DynamicElement::PF,
                            DynamicElement::SFZP,
                        ],
                        print_style: PrintStyle::default(),
                        placement: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<mp/>"));
        assert!(xml.contains("<mf/>"));
        assert!(xml.contains("<sf/>"));
        assert!(xml.contains("<sfp/>"));
        assert!(xml.contains("<sfpp/>"));
        assert!(xml.contains("<fp/>"));
        assert!(xml.contains("<rf/>"));
        assert!(xml.contains("<rfz/>"));
        assert!(xml.contains("<sfz/>"));
        assert!(xml.contains("<sffz/>"));
        assert!(xml.contains("<fz/>"));
        assert!(xml.contains("<n/>"));
        assert!(xml.contains("<pf/>"));
        assert!(xml.contains("<sfzp/>"));
    }

    #[test]
    fn test_emit_dynamics_other() {
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![DynamicElement::OtherDynamics("custom".to_string())],
                        print_style: PrintStyle::default(),
                        placement: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<other-dynamics>custom</other-dynamics>"));
    }

    // === Task 4.3: Wedge Tests ===

    #[test]
    fn test_emit_wedge_crescendo() {
        use crate::ir::common::Position;
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, Wedge, WedgeType,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: Some(AboveBelow::Below),
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: WedgeType::Crescendo,
                        number: Some(1),
                        spread: Some(15.0),
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

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<wedge type=\"crescendo\" number=\"1\" spread=\"15\"/>"));
    }

    #[test]
    fn test_emit_wedge_diminuendo_with_niente() {
        use crate::ir::common::Position;
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, Wedge, WedgeType,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: WedgeType::Diminuendo,
                        number: Some(1),
                        spread: None,
                        niente: Some(YesNo::Yes),
                        line_type: Some(LineType::Dashed),
                        position: Position::default(),
                        color: None,
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("type=\"diminuendo\""));
        assert!(xml.contains("niente=\"yes\""));
        assert!(xml.contains("line-type=\"dashed\""));
    }

    #[test]
    fn test_emit_wedge_stop() {
        use crate::ir::common::Position;
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, Wedge, WedgeType,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: WedgeType::Stop,
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

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<wedge type=\"stop\" number=\"1\"/>"));
    }

    // === Task 4.4: Metronome Tests ===

    #[test]
    fn test_emit_metronome_per_minute() {
        use crate::ir::common::Font;
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, Metronome, MetronomeContent, PerMinute,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: Some(AboveBelow::Above),
                directive: None,
                direction_types: vec![DirectionType {
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
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<metronome>"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<per-minute>120</per-minute>"));
        assert!(xml.contains("</metronome>"));
    }

    #[test]
    fn test_emit_metronome_with_dots() {
        use crate::ir::common::Font;
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, Metronome, MetronomeContent, PerMinute,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Metronome(Metronome {
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
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<metronome parentheses=\"yes\">"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<beat-unit-dot/>"));
        assert!(xml.contains("<per-minute>80</per-minute>"));
    }

    #[test]
    fn test_emit_metronome_beat_equation() {
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, Metronome, MetronomeContent,
        };

        let mut score = create_minimal_score();
        score.parts[0].measures[0]
            .content
            .push(MusicDataElement::Direction(Box::new(Direction {
                placement: None,
                directive: None,
                direction_types: vec![DirectionType {
                    content: DirectionTypeContent::Metronome(Metronome {
                        parentheses: None,
                        content: MetronomeContent::BeatEquation {
                            left_unit: NoteTypeValue::Half,
                            left_dots: 0,
                            right_unit: NoteTypeValue::Quarter,
                            right_dots: 1,
                        },
                        print_style: PrintStyle::default(),
                    }),
                }],
                offset: None,
                voice: None,
                staff: None,
                sound: None,
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<beat-unit>half</beat-unit>"));
        assert!(xml.contains("<beat-unit>quarter</beat-unit>"));
        assert!(xml.contains("<beat-unit-dot/>"));
    }

    // === Task 4.5: Notations Container Tests ===

    #[test]
    fn test_emit_notations_with_tied() {
        use crate::ir::notation::{NotationContent, Notations, Tied};

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
                    ties: vec![Tie {
                        r#type: StartStop::Start,
                        time_only: None,
                    }],
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
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<notations>"));
        assert!(xml.contains("<tied type=\"start\" number=\"1\" placement=\"above\"/>"));
        assert!(xml.contains("</notations>"));
    }

    #[test]
    fn test_emit_notations_with_slur() {
        use crate::ir::notation::{NotationContent, Notations, Slur};

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
                notations: vec![Notations {
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
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<notations>"));
        assert!(xml.contains(
            "<slur type=\"start\" number=\"1\" line-type=\"solid\" placement=\"above\"/>"
        ));
        assert!(xml.contains("</notations>"));
    }

    #[test]
    fn test_emit_notations_with_fermata() {
        use crate::ir::notation::{Fermata, FermataShape, NotationContent, Notations};

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Fermata(Fermata {
                        shape: Some(FermataShape::Normal),
                        r#type: Some(UprightInverted::Upright),
                        print_style: PrintStyle::default(),
                    })],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<notations>"));
        assert!(xml.contains("<fermata type=\"upright\">normal</fermata>"));
        assert!(xml.contains("</notations>"));
    }

    // === Task 4.6: Articulations Tests ===

    #[test]
    fn test_emit_articulations_staccato() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::{ArticulationElement, Articulations, NotationContent, Notations};

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Articulations(Box::new(Articulations {
                        content: vec![ArticulationElement::Staccato(EmptyPlacement {
                            placement: Some(AboveBelow::Above),
                            position: Position::default(),
                        })],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<articulations>"));
        assert!(xml.contains("<staccato placement=\"above\"/>"));
        assert!(xml.contains("</articulations>"));
    }

    #[test]
    fn test_emit_articulations_accent_tenuto() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::notation::{ArticulationElement, Articulations, NotationContent, Notations};

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Articulations(Box::new(Articulations {
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
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<accent placement=\"above\"/>"));
        assert!(xml.contains("<tenuto placement=\"below\"/>"));
    }

    #[test]
    fn test_emit_articulations_strong_accent() {
        use crate::ir::notation::{
            ArticulationElement, Articulations, NotationContent, Notations, StrongAccent,
        };

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Articulations(Box::new(Articulations {
                        content: vec![ArticulationElement::StrongAccent(StrongAccent {
                            r#type: Some(UpDown::Up),
                            placement: Some(AboveBelow::Above),
                            position: Position::default(),
                        })],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<strong-accent type=\"up\" placement=\"above\"/>"));
    }

    #[test]
    fn test_emit_articulations_jazz() {
        use crate::ir::notation::{
            ArticulationElement, Articulations, EmptyLine, NotationContent, Notations,
        };

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Articulations(Box::new(Articulations {
                        content: vec![
                            ArticulationElement::Scoop(EmptyLine::default()),
                            ArticulationElement::Plop(EmptyLine::default()),
                            ArticulationElement::Doit(EmptyLine::default()),
                            ArticulationElement::Falloff(EmptyLine::default()),
                        ],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<scoop/>"));
        assert!(xml.contains("<plop/>"));
        assert!(xml.contains("<doit/>"));
        assert!(xml.contains("<falloff/>"));
    }

    #[test]
    fn test_emit_articulations_breath_mark() {
        use crate::ir::notation::{
            ArticulationElement, Articulations, BreathMark, BreathMarkValue, NotationContent,
            Notations,
        };

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Articulations(Box::new(Articulations {
                        content: vec![ArticulationElement::BreathMark(BreathMark {
                            value: BreathMarkValue::Comma,
                            placement: Some(AboveBelow::Above),
                            position: Position::default(),
                        })],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<breath-mark placement=\"above\">comma</breath-mark>"));
    }

    #[test]
    fn test_emit_articulations_caesura() {
        use crate::ir::notation::{
            ArticulationElement, Articulations, Caesura, CaesuraValue, NotationContent, Notations,
        };

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
                notations: vec![Notations {
                    print_object: None,
                    content: vec![NotationContent::Articulations(Box::new(Articulations {
                        content: vec![ArticulationElement::Caesura(Caesura {
                            value: CaesuraValue::Normal,
                            placement: Some(AboveBelow::Above),
                            position: Position::default(),
                        })],
                    }))],
                    editorial: Editorial::default(),
                }],
                lyrics: vec![],
            })));

        let xml = emit_score(&score).unwrap();

        assert!(xml.contains("<caesura placement=\"above\">normal</caesura>"));
    }

    // === Integration Test: Direction with Dynamics and Notes with Notations ===

    #[test]
    fn test_emit_direction_and_notations_integration() {
        use crate::ir::common::EmptyPlacement;
        use crate::ir::direction::{
            Direction, DirectionType, DirectionTypeContent, DynamicElement, Dynamics, Wedge,
            WedgeType,
        };
        use crate::ir::notation::{
            ArticulationElement, Articulations, NotationContent, Notations, Slur,
        };

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

    // === New String Converter Tests for Milestone 4 ===

    #[test]
    fn test_above_below_to_string() {
        assert_eq!(above_below_to_string(&AboveBelow::Above), "above");
        assert_eq!(above_below_to_string(&AboveBelow::Below), "below");
    }

    #[test]
    fn test_start_stop_continue_to_string() {
        assert_eq!(
            start_stop_continue_to_string(&StartStopContinue::Start),
            "start"
        );
        assert_eq!(
            start_stop_continue_to_string(&StartStopContinue::Stop),
            "stop"
        );
        assert_eq!(
            start_stop_continue_to_string(&StartStopContinue::Continue),
            "continue"
        );
    }

    #[test]
    fn test_wedge_type_to_string() {
        use crate::ir::direction::WedgeType;
        assert_eq!(wedge_type_to_string(&WedgeType::Crescendo), "crescendo");
        assert_eq!(wedge_type_to_string(&WedgeType::Diminuendo), "diminuendo");
        assert_eq!(wedge_type_to_string(&WedgeType::Stop), "stop");
        assert_eq!(wedge_type_to_string(&WedgeType::Continue), "continue");
    }

    #[test]
    fn test_line_type_to_string() {
        assert_eq!(line_type_to_string(&LineType::Solid), "solid");
        assert_eq!(line_type_to_string(&LineType::Dashed), "dashed");
        assert_eq!(line_type_to_string(&LineType::Dotted), "dotted");
        assert_eq!(line_type_to_string(&LineType::Wavy), "wavy");
    }

    #[test]
    fn test_up_down_to_string() {
        assert_eq!(up_down_to_string(&UpDown::Up), "up");
        assert_eq!(up_down_to_string(&UpDown::Down), "down");
    }
}
