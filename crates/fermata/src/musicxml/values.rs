//! Value parsers for MusicXML string values.
//!
//! This module provides parsing functions that convert MusicXML string values
//! to their corresponding IR enum types. These are the inverse of the
//! `*_to_string` functions in the emit module.
//!
//! Each parser takes a string slice and a position (for error reporting) and
//! returns a Result with the parsed value or a ParseError.
//!
//! Note: These parsers are currently unused but will be used in Phase 3
//! Milestones 2-5 for parsing notes, attributes, directions, and barlines.
#![allow(dead_code)]

use super::ParseError;
use crate::ir::attributes::{BarStyle, CancelLocation, ClefSign, Mode, TimeSymbol, Winged};
use crate::ir::beam::{BeamValue, Fan, NoteheadValue, StemValue};
use crate::ir::common::{
    AboveBelow, AccidentalValue, BackwardForward, CssFontSize, FontSize, LeftCenterRight, LineType,
    OverUnder, RightLeftMiddle, StartStop, StartStopContinue, StartStopDiscontinue,
    StartStopSingle, TopMiddleBottom, UpDown, UprightInverted, YesNo,
};
use crate::ir::direction::{PedalType, UpDownStopContinue, WedgeType};
use crate::ir::duration::NoteTypeValue;
use crate::ir::lyric::Syllabic;
use crate::ir::notation::{
    ArrowDirection, ArrowStyle, BreathMarkValue, CaesuraValue, FermataShape, HandbellValue,
    HoleClosedLocation, HoleClosedValue, LineLength, LineShape, ShowTuplet, StartNote, TapHand,
    TopBottom, TremoloType, TrillStep, TwoNoteTurn,
};
use crate::ir::pitch::Step;
use crate::ir::score::{MarginType, NoteSizeType};

// === Note Type Value ===

/// Parse a note-type-value string.
///
/// Valid values: "1024th", "512th", "256th", "128th", "64th", "32nd", "16th",
/// "eighth", "quarter", "half", "whole", "breve", "long", "maxima"
pub(crate) fn parse_note_type_value(s: &str, position: usize) -> Result<NoteTypeValue, ParseError> {
    match s {
        "1024th" => Ok(NoteTypeValue::N1024th),
        "512th" => Ok(NoteTypeValue::N512th),
        "256th" => Ok(NoteTypeValue::N256th),
        "128th" => Ok(NoteTypeValue::N128th),
        "64th" => Ok(NoteTypeValue::N64th),
        "32nd" => Ok(NoteTypeValue::N32nd),
        "16th" => Ok(NoteTypeValue::N16th),
        "eighth" => Ok(NoteTypeValue::Eighth),
        "quarter" => Ok(NoteTypeValue::Quarter),
        "half" => Ok(NoteTypeValue::Half),
        "whole" => Ok(NoteTypeValue::Whole),
        "breve" => Ok(NoteTypeValue::Breve),
        "long" => Ok(NoteTypeValue::Long),
        "maxima" => Ok(NoteTypeValue::Maxima),
        _ => Err(ParseError::invalid_value("note-type-value", s, position)),
    }
}

// === Step ===

/// Parse a step value (pitch letter).
///
/// Valid values: "A", "B", "C", "D", "E", "F", "G"
pub(crate) fn parse_step(s: &str, position: usize) -> Result<Step, ParseError> {
    match s {
        "A" => Ok(Step::A),
        "B" => Ok(Step::B),
        "C" => Ok(Step::C),
        "D" => Ok(Step::D),
        "E" => Ok(Step::E),
        "F" => Ok(Step::F),
        "G" => Ok(Step::G),
        _ => Err(ParseError::invalid_value("step (A-G)", s, position)),
    }
}

// === Mode ===

/// Parse a mode value.
///
/// Valid values: "major", "minor", "dorian", "phrygian", "lydian",
/// "mixolydian", "aeolian", "locrian", "ionian", "none"
pub(crate) fn parse_mode(s: &str, position: usize) -> Result<Mode, ParseError> {
    match s {
        "major" => Ok(Mode::Major),
        "minor" => Ok(Mode::Minor),
        "dorian" => Ok(Mode::Dorian),
        "phrygian" => Ok(Mode::Phrygian),
        "lydian" => Ok(Mode::Lydian),
        "mixolydian" => Ok(Mode::Mixolydian),
        "aeolian" => Ok(Mode::Aeolian),
        "locrian" => Ok(Mode::Locrian),
        "ionian" => Ok(Mode::Ionian),
        "none" => Ok(Mode::None),
        _ => Err(ParseError::invalid_value("mode", s, position)),
    }
}

// === Clef Sign ===

/// Parse a clef-sign value.
///
/// Valid values: "G", "F", "C", "percussion", "TAB", "jianpu", "none"
pub(crate) fn parse_clef_sign(s: &str, position: usize) -> Result<ClefSign, ParseError> {
    match s {
        "G" => Ok(ClefSign::G),
        "F" => Ok(ClefSign::F),
        "C" => Ok(ClefSign::C),
        "percussion" => Ok(ClefSign::Percussion),
        "TAB" => Ok(ClefSign::Tab),
        "jianpu" => Ok(ClefSign::Jianpu),
        "none" => Ok(ClefSign::None),
        _ => Err(ParseError::invalid_value("clef-sign", s, position)),
    }
}

// === Time Symbol ===

/// Parse a time-symbol value.
///
/// Valid values: "common", "cut", "single-number", "note", "dotted-note", "normal"
pub(crate) fn parse_time_symbol(s: &str, position: usize) -> Result<TimeSymbol, ParseError> {
    match s {
        "common" => Ok(TimeSymbol::Common),
        "cut" => Ok(TimeSymbol::Cut),
        "single-number" => Ok(TimeSymbol::SingleNumber),
        "note" => Ok(TimeSymbol::Note),
        "dotted-note" => Ok(TimeSymbol::DottedNote),
        "normal" => Ok(TimeSymbol::Normal),
        _ => Err(ParseError::invalid_value("time-symbol", s, position)),
    }
}

// === Start/Stop Variants ===

/// Parse a start-stop value.
///
/// Valid values: "start", "stop"
pub(crate) fn parse_start_stop(s: &str, position: usize) -> Result<StartStop, ParseError> {
    match s {
        "start" => Ok(StartStop::Start),
        "stop" => Ok(StartStop::Stop),
        _ => Err(ParseError::invalid_value("start-stop", s, position)),
    }
}

/// Parse a start-stop-continue value.
///
/// Valid values: "start", "stop", "continue"
pub(crate) fn parse_start_stop_continue(
    s: &str,
    position: usize,
) -> Result<StartStopContinue, ParseError> {
    match s {
        "start" => Ok(StartStopContinue::Start),
        "stop" => Ok(StartStopContinue::Stop),
        "continue" => Ok(StartStopContinue::Continue),
        _ => Err(ParseError::invalid_value(
            "start-stop-continue",
            s,
            position,
        )),
    }
}

/// Parse a start-stop-single value.
///
/// Valid values: "start", "stop", "single"
pub(crate) fn parse_start_stop_single(
    s: &str,
    position: usize,
) -> Result<StartStopSingle, ParseError> {
    match s {
        "start" => Ok(StartStopSingle::Start),
        "stop" => Ok(StartStopSingle::Stop),
        "single" => Ok(StartStopSingle::Single),
        _ => Err(ParseError::invalid_value("start-stop-single", s, position)),
    }
}

/// Parse a start-stop-discontinue value.
///
/// Valid values: "start", "stop", "discontinue"
pub(crate) fn parse_start_stop_discontinue(
    s: &str,
    position: usize,
) -> Result<StartStopDiscontinue, ParseError> {
    match s {
        "start" => Ok(StartStopDiscontinue::Start),
        "stop" => Ok(StartStopDiscontinue::Stop),
        "discontinue" => Ok(StartStopDiscontinue::Discontinue),
        _ => Err(ParseError::invalid_value(
            "start-stop-discontinue",
            s,
            position,
        )),
    }
}

// === Yes/No ===

/// Parse a yes-no value.
///
/// Valid values: "yes", "no"
pub(crate) fn parse_yes_no(s: &str, position: usize) -> Result<YesNo, ParseError> {
    match s {
        "yes" => Ok(YesNo::Yes),
        "no" => Ok(YesNo::No),
        _ => Err(ParseError::invalid_value("yes-no", s, position)),
    }
}

// === Direction/Position Enums ===

/// Parse an above-below value.
///
/// Valid values: "above", "below"
pub(crate) fn parse_above_below(s: &str, position: usize) -> Result<AboveBelow, ParseError> {
    match s {
        "above" => Ok(AboveBelow::Above),
        "below" => Ok(AboveBelow::Below),
        _ => Err(ParseError::invalid_value("above-below", s, position)),
    }
}

/// Parse an up-down value.
///
/// Valid values: "up", "down"
pub(crate) fn parse_up_down(s: &str, position: usize) -> Result<UpDown, ParseError> {
    match s {
        "up" => Ok(UpDown::Up),
        "down" => Ok(UpDown::Down),
        _ => Err(ParseError::invalid_value("up-down", s, position)),
    }
}

/// Parse an over-under value.
///
/// Valid values: "over", "under"
pub(crate) fn parse_over_under(s: &str, position: usize) -> Result<OverUnder, ParseError> {
    match s {
        "over" => Ok(OverUnder::Over),
        "under" => Ok(OverUnder::Under),
        _ => Err(ParseError::invalid_value("over-under", s, position)),
    }
}

/// Parse a backward-forward value.
///
/// Valid values: "backward", "forward"
pub(crate) fn parse_backward_forward(
    s: &str,
    position: usize,
) -> Result<BackwardForward, ParseError> {
    match s {
        "backward" => Ok(BackwardForward::Backward),
        "forward" => Ok(BackwardForward::Forward),
        _ => Err(ParseError::invalid_value("backward-forward", s, position)),
    }
}

/// Parse a right-left-middle value.
///
/// Valid values: "right", "left", "middle"
pub(crate) fn parse_right_left_middle(
    s: &str,
    position: usize,
) -> Result<RightLeftMiddle, ParseError> {
    match s {
        "right" => Ok(RightLeftMiddle::Right),
        "left" => Ok(RightLeftMiddle::Left),
        "middle" => Ok(RightLeftMiddle::Middle),
        _ => Err(ParseError::invalid_value("right-left-middle", s, position)),
    }
}

/// Parse an upright-inverted value.
///
/// Valid values: "upright", "inverted"
pub(crate) fn parse_upright_inverted(
    s: &str,
    position: usize,
) -> Result<UprightInverted, ParseError> {
    match s {
        "upright" => Ok(UprightInverted::Upright),
        "inverted" => Ok(UprightInverted::Inverted),
        _ => Err(ParseError::invalid_value("upright-inverted", s, position)),
    }
}

/// Parse a left-center-right value.
///
/// Valid values: "left", "center", "right"
pub(crate) fn parse_left_center_right(
    s: &str,
    position: usize,
) -> Result<LeftCenterRight, ParseError> {
    match s {
        "left" => Ok(LeftCenterRight::Left),
        "center" => Ok(LeftCenterRight::Center),
        "right" => Ok(LeftCenterRight::Right),
        _ => Err(ParseError::invalid_value("left-center-right", s, position)),
    }
}

/// Parse a top-middle-bottom value.
///
/// Valid values: "top", "middle", "bottom"
pub(crate) fn parse_top_middle_bottom(
    s: &str,
    position: usize,
) -> Result<TopMiddleBottom, ParseError> {
    match s {
        "top" => Ok(TopMiddleBottom::Top),
        "middle" => Ok(TopMiddleBottom::Middle),
        "bottom" => Ok(TopMiddleBottom::Bottom),
        _ => Err(ParseError::invalid_value("top-middle-bottom", s, position)),
    }
}

/// Parse a top-bottom value.
///
/// Valid values: "top", "bottom"
pub(crate) fn parse_top_bottom(s: &str, position: usize) -> Result<TopBottom, ParseError> {
    match s {
        "top" => Ok(TopBottom::Top),
        "bottom" => Ok(TopBottom::Bottom),
        _ => Err(ParseError::invalid_value("top-bottom", s, position)),
    }
}

// === Beam/Stem/Notehead ===

/// Parse a beam-value.
///
/// Valid values: "begin", "continue", "end", "forward hook", "backward hook"
pub(crate) fn parse_beam_value(s: &str, position: usize) -> Result<BeamValue, ParseError> {
    match s {
        "begin" => Ok(BeamValue::Begin),
        "continue" => Ok(BeamValue::Continue),
        "end" => Ok(BeamValue::End),
        "forward hook" => Ok(BeamValue::ForwardHook),
        "backward hook" => Ok(BeamValue::BackwardHook),
        _ => Err(ParseError::invalid_value("beam-value", s, position)),
    }
}

/// Parse a stem-value.
///
/// Valid values: "down", "up", "double", "none"
pub(crate) fn parse_stem_value(s: &str, position: usize) -> Result<StemValue, ParseError> {
    match s {
        "down" => Ok(StemValue::Down),
        "up" => Ok(StemValue::Up),
        "double" => Ok(StemValue::Double),
        "none" => Ok(StemValue::None),
        _ => Err(ParseError::invalid_value("stem-value", s, position)),
    }
}

/// Parse a notehead-value.
///
/// Valid values include: "slash", "triangle", "diamond", "square", "cross", "x",
/// "circle-x", "inverted triangle", "arrow down", "arrow up", "circled", "slashed",
/// "back slashed", "normal", "cluster", "circle dot", "left triangle", "rectangle",
/// "none", "do", "re", "mi", "fa", "fa up", "so", "la", "ti", "other"
pub(crate) fn parse_notehead_value(s: &str, position: usize) -> Result<NoteheadValue, ParseError> {
    match s {
        "slash" => Ok(NoteheadValue::Slash),
        "triangle" => Ok(NoteheadValue::Triangle),
        "diamond" => Ok(NoteheadValue::Diamond),
        "square" => Ok(NoteheadValue::Square),
        "cross" => Ok(NoteheadValue::Cross),
        "x" => Ok(NoteheadValue::X),
        "circle-x" => Ok(NoteheadValue::CircleX),
        "inverted triangle" => Ok(NoteheadValue::InvertedTriangle),
        "arrow down" => Ok(NoteheadValue::ArrowDown),
        "arrow up" => Ok(NoteheadValue::ArrowUp),
        "circled" => Ok(NoteheadValue::Circled),
        "slashed" => Ok(NoteheadValue::Slashed),
        "back slashed" => Ok(NoteheadValue::BackSlashed),
        "normal" => Ok(NoteheadValue::Normal),
        "cluster" => Ok(NoteheadValue::Cluster),
        "circle dot" => Ok(NoteheadValue::CircleDot),
        "left triangle" => Ok(NoteheadValue::LeftTriangle),
        "rectangle" => Ok(NoteheadValue::Rectangle),
        "none" => Ok(NoteheadValue::None),
        "do" => Ok(NoteheadValue::Do),
        "re" => Ok(NoteheadValue::Re),
        "mi" => Ok(NoteheadValue::Mi),
        "fa" => Ok(NoteheadValue::Fa),
        "fa up" => Ok(NoteheadValue::FaUp),
        "so" => Ok(NoteheadValue::So),
        "la" => Ok(NoteheadValue::La),
        "ti" => Ok(NoteheadValue::Ti),
        "other" => Ok(NoteheadValue::Other),
        _ => Err(ParseError::invalid_value("notehead-value", s, position)),
    }
}

/// Parse a fan value.
///
/// Valid values: "accel", "rit", "none"
pub(crate) fn parse_fan(s: &str, position: usize) -> Result<Fan, ParseError> {
    match s {
        "accel" => Ok(Fan::Accel),
        "rit" => Ok(Fan::Rit),
        "none" => Ok(Fan::None),
        _ => Err(ParseError::invalid_value("fan", s, position)),
    }
}

// === Accidentals ===

/// Parse an accidental-value.
///
/// This handles all MusicXML accidental types including microtonal variants.
pub(crate) fn parse_accidental_value(
    s: &str,
    position: usize,
) -> Result<AccidentalValue, ParseError> {
    match s {
        "sharp" => Ok(AccidentalValue::Sharp),
        "natural" => Ok(AccidentalValue::Natural),
        "flat" => Ok(AccidentalValue::Flat),
        "double-sharp" => Ok(AccidentalValue::DoubleSharp),
        "sharp-sharp" => Ok(AccidentalValue::SharpSharp),
        "flat-flat" => Ok(AccidentalValue::FlatFlat),
        "double-flat" => Ok(AccidentalValue::DoubleFlat),
        "natural-sharp" => Ok(AccidentalValue::NaturalSharp),
        "natural-flat" => Ok(AccidentalValue::NaturalFlat),
        "quarter-flat" => Ok(AccidentalValue::QuarterFlat),
        "quarter-sharp" => Ok(AccidentalValue::QuarterSharp),
        "three-quarters-flat" => Ok(AccidentalValue::ThreeQuartersFlat),
        "three-quarters-sharp" => Ok(AccidentalValue::ThreeQuartersSharp),
        "sharp-down" => Ok(AccidentalValue::SharpDown),
        "sharp-up" => Ok(AccidentalValue::SharpUp),
        "natural-down" => Ok(AccidentalValue::NaturalDown),
        "natural-up" => Ok(AccidentalValue::NaturalUp),
        "flat-down" => Ok(AccidentalValue::FlatDown),
        "flat-up" => Ok(AccidentalValue::FlatUp),
        "triple-sharp" => Ok(AccidentalValue::TripleSharp),
        "triple-flat" => Ok(AccidentalValue::TripleFlat),
        "slash-quarter-sharp" => Ok(AccidentalValue::SlashQuarterSharp),
        "slash-sharp" => Ok(AccidentalValue::SlashSharp),
        "slash-flat" => Ok(AccidentalValue::SlashFlat),
        "double-slash-flat" => Ok(AccidentalValue::DoubleSlashFlat),
        "sharp-1" => Ok(AccidentalValue::Sharp1),
        "sharp-2" => Ok(AccidentalValue::Sharp2),
        "sharp-3" => Ok(AccidentalValue::Sharp3),
        "sharp-5" => Ok(AccidentalValue::Sharp5),
        "flat-1" => Ok(AccidentalValue::Flat1),
        "flat-2" => Ok(AccidentalValue::Flat2),
        "flat-3" => Ok(AccidentalValue::Flat3),
        "flat-4" => Ok(AccidentalValue::Flat4),
        "sori" => Ok(AccidentalValue::Sori),
        "koron" => Ok(AccidentalValue::Koron),
        "other" => Ok(AccidentalValue::Other),
        _ => Err(ParseError::invalid_value("accidental-value", s, position)),
    }
}

// === Bar Style ===

/// Parse a bar-style value.
///
/// Valid values: "regular", "dotted", "dashed", "heavy", "light-light",
/// "light-heavy", "heavy-light", "heavy-heavy", "tick", "short", "none"
pub(crate) fn parse_bar_style(s: &str, position: usize) -> Result<BarStyle, ParseError> {
    match s {
        "regular" => Ok(BarStyle::Regular),
        "dotted" => Ok(BarStyle::Dotted),
        "dashed" => Ok(BarStyle::Dashed),
        "heavy" => Ok(BarStyle::Heavy),
        "light-light" => Ok(BarStyle::LightLight),
        "light-heavy" => Ok(BarStyle::LightHeavy),
        "heavy-light" => Ok(BarStyle::HeavyLight),
        "heavy-heavy" => Ok(BarStyle::HeavyHeavy),
        "tick" => Ok(BarStyle::Tick),
        "short" => Ok(BarStyle::Short),
        "none" => Ok(BarStyle::None),
        _ => Err(ParseError::invalid_value("bar-style", s, position)),
    }
}

/// Parse a winged value.
///
/// Valid values: "none", "straight", "curved", "double-straight", "double-curved"
pub(crate) fn parse_winged(s: &str, position: usize) -> Result<Winged, ParseError> {
    match s {
        "none" => Ok(Winged::None),
        "straight" => Ok(Winged::Straight),
        "curved" => Ok(Winged::Curved),
        "double-straight" => Ok(Winged::DoubleStraight),
        "double-curved" => Ok(Winged::DoubleCurved),
        _ => Err(ParseError::invalid_value("winged", s, position)),
    }
}

/// Parse a cancel-location value.
///
/// Valid values: "left", "right", "before-barline"
pub(crate) fn parse_cancel_location(
    s: &str,
    position: usize,
) -> Result<CancelLocation, ParseError> {
    match s {
        "left" => Ok(CancelLocation::Left),
        "right" => Ok(CancelLocation::Right),
        "before-barline" => Ok(CancelLocation::BeforeBarline),
        _ => Err(ParseError::invalid_value("cancel-location", s, position)),
    }
}

// === Notation Types ===

/// Parse a fermata-shape value.
///
/// Valid values: "normal", "angled", "square", "double-angled", "double-square",
/// "double-dot", "half-curve", "curlew"
pub(crate) fn parse_fermata_shape(s: &str, position: usize) -> Result<FermataShape, ParseError> {
    match s {
        "normal" | "" => Ok(FermataShape::Normal),
        "angled" => Ok(FermataShape::Angled),
        "square" => Ok(FermataShape::Square),
        "double-angled" => Ok(FermataShape::DoubleAngled),
        "double-square" => Ok(FermataShape::DoubleSquare),
        "double-dot" => Ok(FermataShape::DoubleDot),
        "half-curve" => Ok(FermataShape::HalfCurve),
        "curlew" => Ok(FermataShape::Curlew),
        _ => Err(ParseError::invalid_value("fermata-shape", s, position)),
    }
}

/// Parse a breath-mark-value.
///
/// Valid values: "", "comma", "tick", "upbow", "salzedo"
pub(crate) fn parse_breath_mark_value(
    s: &str,
    position: usize,
) -> Result<BreathMarkValue, ParseError> {
    match s {
        "" => Ok(BreathMarkValue::Empty),
        "comma" => Ok(BreathMarkValue::Comma),
        "tick" => Ok(BreathMarkValue::Tick),
        "upbow" => Ok(BreathMarkValue::Upbow),
        "salzedo" => Ok(BreathMarkValue::Salzedo),
        _ => Err(ParseError::invalid_value("breath-mark-value", s, position)),
    }
}

/// Parse a caesura-value.
///
/// Valid values: "normal", "thick", "short", "curved", "single"
pub(crate) fn parse_caesura_value(s: &str, position: usize) -> Result<CaesuraValue, ParseError> {
    match s {
        "normal" => Ok(CaesuraValue::Normal),
        "thick" => Ok(CaesuraValue::Thick),
        "short" => Ok(CaesuraValue::Short),
        "curved" => Ok(CaesuraValue::Curved),
        "single" => Ok(CaesuraValue::Single),
        _ => Err(ParseError::invalid_value("caesura-value", s, position)),
    }
}

/// Parse a line-length value.
///
/// Valid values: "short", "medium", "long"
pub(crate) fn parse_line_length(s: &str, position: usize) -> Result<LineLength, ParseError> {
    match s {
        "short" => Ok(LineLength::Short),
        "medium" => Ok(LineLength::Medium),
        "long" => Ok(LineLength::Long),
        _ => Err(ParseError::invalid_value("line-length", s, position)),
    }
}

/// Parse a line-shape value.
///
/// Valid values: "straight", "curved"
pub(crate) fn parse_line_shape(s: &str, position: usize) -> Result<LineShape, ParseError> {
    match s {
        "straight" => Ok(LineShape::Straight),
        "curved" => Ok(LineShape::Curved),
        _ => Err(ParseError::invalid_value("line-shape", s, position)),
    }
}

/// Parse a line-type value.
///
/// Valid values: "solid", "dashed", "dotted", "wavy"
pub(crate) fn parse_line_type(s: &str, position: usize) -> Result<LineType, ParseError> {
    match s {
        "solid" => Ok(LineType::Solid),
        "dashed" => Ok(LineType::Dashed),
        "dotted" => Ok(LineType::Dotted),
        "wavy" => Ok(LineType::Wavy),
        _ => Err(ParseError::invalid_value("line-type", s, position)),
    }
}

/// Parse a show-tuplet value.
///
/// Valid values: "actual", "both", "none"
pub(crate) fn parse_show_tuplet(s: &str, position: usize) -> Result<ShowTuplet, ParseError> {
    match s {
        "actual" => Ok(ShowTuplet::Actual),
        "both" => Ok(ShowTuplet::Both),
        "none" => Ok(ShowTuplet::None),
        _ => Err(ParseError::invalid_value("show-tuplet", s, position)),
    }
}

// === Ornament Types ===

/// Parse a start-note value.
///
/// Valid values: "upper", "main", "below"
pub(crate) fn parse_start_note(s: &str, position: usize) -> Result<StartNote, ParseError> {
    match s {
        "upper" => Ok(StartNote::Upper),
        "main" => Ok(StartNote::Main),
        "below" => Ok(StartNote::Below),
        _ => Err(ParseError::invalid_value("start-note", s, position)),
    }
}

/// Parse a trill-step value.
///
/// Valid values: "whole", "half", "unison"
pub(crate) fn parse_trill_step(s: &str, position: usize) -> Result<TrillStep, ParseError> {
    match s {
        "whole" => Ok(TrillStep::Whole),
        "half" => Ok(TrillStep::Half),
        "unison" => Ok(TrillStep::Unison),
        _ => Err(ParseError::invalid_value("trill-step", s, position)),
    }
}

/// Parse a two-note-turn value.
///
/// Valid values: "whole", "half", "none"
pub(crate) fn parse_two_note_turn(s: &str, position: usize) -> Result<TwoNoteTurn, ParseError> {
    match s {
        "whole" => Ok(TwoNoteTurn::Whole),
        "half" => Ok(TwoNoteTurn::Half),
        "none" => Ok(TwoNoteTurn::None),
        _ => Err(ParseError::invalid_value("two-note-turn", s, position)),
    }
}

/// Parse a tremolo-type value.
///
/// Valid values: "start", "stop", "single", "unmeasured"
pub(crate) fn parse_tremolo_type(s: &str, position: usize) -> Result<TremoloType, ParseError> {
    match s {
        "start" => Ok(TremoloType::Start),
        "stop" => Ok(TremoloType::Stop),
        "single" => Ok(TremoloType::Single),
        "unmeasured" => Ok(TremoloType::Unmeasured),
        _ => Err(ParseError::invalid_value("tremolo-type", s, position)),
    }
}

// === Technical Types ===

/// Parse a handbell-value.
///
/// Valid values include: "belltree", "damp", "echo", "gyro", "hand martellato",
/// "mallet lift", "mallet table", "martellato", "martellato lift",
/// "muted martellato", "pluck lift", "swing"
pub(crate) fn parse_handbell_value(s: &str, position: usize) -> Result<HandbellValue, ParseError> {
    match s {
        "belltree" => Ok(HandbellValue::Belltree),
        "damp" => Ok(HandbellValue::Damp),
        "echo" => Ok(HandbellValue::Echo),
        "gyro" => Ok(HandbellValue::Gyro),
        "hand martellato" => Ok(HandbellValue::HandMartellato),
        "mallet lift" => Ok(HandbellValue::MalletLift),
        "mallet table" => Ok(HandbellValue::MalletTable),
        "martellato" => Ok(HandbellValue::Martellato),
        "martellato lift" => Ok(HandbellValue::MartellatoLift),
        "muted martellato" => Ok(HandbellValue::MutedMartellato),
        "pluck lift" => Ok(HandbellValue::PluckLift),
        "swing" => Ok(HandbellValue::Swing),
        _ => Err(ParseError::invalid_value("handbell-value", s, position)),
    }
}

/// Parse an arrow-direction value.
pub(crate) fn parse_arrow_direction(
    s: &str,
    position: usize,
) -> Result<ArrowDirection, ParseError> {
    match s {
        "left" => Ok(ArrowDirection::Left),
        "up" => Ok(ArrowDirection::Up),
        "right" => Ok(ArrowDirection::Right),
        "down" => Ok(ArrowDirection::Down),
        "northwest" => Ok(ArrowDirection::Northwest),
        "northeast" => Ok(ArrowDirection::Northeast),
        "southeast" => Ok(ArrowDirection::Southeast),
        "southwest" => Ok(ArrowDirection::Southwest),
        "left right" => Ok(ArrowDirection::LeftRight),
        "up down" => Ok(ArrowDirection::UpDown),
        "northwest southeast" => Ok(ArrowDirection::NorthwestSoutheast),
        "northeast southwest" => Ok(ArrowDirection::NortheastSouthwest),
        "other" => Ok(ArrowDirection::Other),
        _ => Err(ParseError::invalid_value("arrow-direction", s, position)),
    }
}

/// Parse an arrow-style value.
///
/// Valid values: "single", "double", "filled", "hollow", "paired", "combined", "other"
pub(crate) fn parse_arrow_style(s: &str, position: usize) -> Result<ArrowStyle, ParseError> {
    match s {
        "single" => Ok(ArrowStyle::Single),
        "double" => Ok(ArrowStyle::Double),
        "filled" => Ok(ArrowStyle::Filled),
        "hollow" => Ok(ArrowStyle::Hollow),
        "paired" => Ok(ArrowStyle::Paired),
        "combined" => Ok(ArrowStyle::Combined),
        "other" => Ok(ArrowStyle::Other),
        _ => Err(ParseError::invalid_value("arrow-style", s, position)),
    }
}

/// Parse a hole-closed-value.
///
/// Valid values: "yes", "no", "half"
pub(crate) fn parse_hole_closed_value(
    s: &str,
    position: usize,
) -> Result<HoleClosedValue, ParseError> {
    match s {
        "yes" => Ok(HoleClosedValue::Yes),
        "no" => Ok(HoleClosedValue::No),
        "half" => Ok(HoleClosedValue::Half),
        _ => Err(ParseError::invalid_value("hole-closed-value", s, position)),
    }
}

/// Parse a hole-closed-location.
///
/// Valid values: "right", "bottom", "left", "top"
pub(crate) fn parse_hole_closed_location(
    s: &str,
    position: usize,
) -> Result<HoleClosedLocation, ParseError> {
    match s {
        "right" => Ok(HoleClosedLocation::Right),
        "bottom" => Ok(HoleClosedLocation::Bottom),
        "left" => Ok(HoleClosedLocation::Left),
        "top" => Ok(HoleClosedLocation::Top),
        _ => Err(ParseError::invalid_value(
            "hole-closed-location",
            s,
            position,
        )),
    }
}

/// Parse a tap-hand value.
///
/// Valid values: "left", "right"
pub(crate) fn parse_tap_hand(s: &str, position: usize) -> Result<TapHand, ParseError> {
    match s {
        "left" => Ok(TapHand::Left),
        "right" => Ok(TapHand::Right),
        _ => Err(ParseError::invalid_value("tap-hand", s, position)),
    }
}

// === Direction Types ===

/// Parse a wedge-type value.
///
/// Valid values: "crescendo", "diminuendo", "stop", "continue"
pub(crate) fn parse_wedge_type(s: &str, position: usize) -> Result<WedgeType, ParseError> {
    match s {
        "crescendo" => Ok(WedgeType::Crescendo),
        "diminuendo" => Ok(WedgeType::Diminuendo),
        "stop" => Ok(WedgeType::Stop),
        "continue" => Ok(WedgeType::Continue),
        _ => Err(ParseError::invalid_value("wedge-type", s, position)),
    }
}

/// Parse a pedal-type value.
///
/// Valid values: "start", "stop", "sostenuto", "change", "continue", "discontinue", "resume"
pub(crate) fn parse_pedal_type(s: &str, position: usize) -> Result<PedalType, ParseError> {
    match s {
        "start" => Ok(PedalType::Start),
        "stop" => Ok(PedalType::Stop),
        "sostenuto" => Ok(PedalType::Sostenuto),
        "change" => Ok(PedalType::Change),
        "continue" => Ok(PedalType::Continue),
        "discontinue" => Ok(PedalType::Discontinue),
        "resume" => Ok(PedalType::Resume),
        _ => Err(ParseError::invalid_value("pedal-type", s, position)),
    }
}

/// Parse an up-down-stop-continue value.
///
/// Valid values: "up", "down", "stop", "continue"
pub(crate) fn parse_up_down_stop_continue(
    s: &str,
    position: usize,
) -> Result<UpDownStopContinue, ParseError> {
    match s {
        "up" => Ok(UpDownStopContinue::Up),
        "down" => Ok(UpDownStopContinue::Down),
        "stop" => Ok(UpDownStopContinue::Stop),
        "continue" => Ok(UpDownStopContinue::Continue),
        _ => Err(ParseError::invalid_value(
            "up-down-stop-continue",
            s,
            position,
        )),
    }
}

// === Lyric Types ===

/// Parse a syllabic value.
///
/// Valid values: "single", "begin", "end", "middle"
pub(crate) fn parse_syllabic(s: &str, position: usize) -> Result<Syllabic, ParseError> {
    match s {
        "single" => Ok(Syllabic::Single),
        "begin" => Ok(Syllabic::Begin),
        "end" => Ok(Syllabic::End),
        "middle" => Ok(Syllabic::Middle),
        _ => Err(ParseError::invalid_value("syllabic", s, position)),
    }
}

// === Score Types ===

/// Parse a margin-type value.
///
/// Valid values: "odd", "even", "both"
pub(crate) fn parse_margin_type(s: &str, position: usize) -> Result<MarginType, ParseError> {
    match s {
        "odd" => Ok(MarginType::Odd),
        "even" => Ok(MarginType::Even),
        "both" => Ok(MarginType::Both),
        _ => Err(ParseError::invalid_value("margin-type", s, position)),
    }
}

/// Parse a note-size-type value.
///
/// Valid values: "cue", "grace", "grace-cue", "large"
pub(crate) fn parse_note_size_type(s: &str, position: usize) -> Result<NoteSizeType, ParseError> {
    match s {
        "cue" => Ok(NoteSizeType::Cue),
        "grace" => Ok(NoteSizeType::Grace),
        "grace-cue" => Ok(NoteSizeType::GraceCue),
        "large" => Ok(NoteSizeType::Large),
        _ => Err(ParseError::invalid_value("note-size-type", s, position)),
    }
}

// === Font Types ===

/// Parse a CSS font-size value.
///
/// Valid values: "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large"
pub(crate) fn parse_css_font_size(s: &str, position: usize) -> Result<CssFontSize, ParseError> {
    match s {
        "xx-small" => Ok(CssFontSize::XxSmall),
        "x-small" => Ok(CssFontSize::XSmall),
        "small" => Ok(CssFontSize::Small),
        "medium" => Ok(CssFontSize::Medium),
        "large" => Ok(CssFontSize::Large),
        "x-large" => Ok(CssFontSize::XLarge),
        "xx-large" => Ok(CssFontSize::XxLarge),
        _ => Err(ParseError::invalid_value("css-font-size", s, position)),
    }
}

/// Parse a font-size value (either CSS keyword or numeric points).
pub(crate) fn parse_font_size(s: &str, position: usize) -> Result<FontSize, ParseError> {
    // Try parsing as CSS font size first
    if let Ok(css) = parse_css_font_size(s, position) {
        return Ok(FontSize::Css(css));
    }

    // Otherwise try parsing as numeric points
    match s.parse::<f64>() {
        Ok(pts) => Ok(FontSize::Points(pts)),
        Err(_) => Err(ParseError::invalid_value("font-size", s, position)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Note Type Value Tests ===

    #[test]
    fn test_parse_note_type_value_all_values() {
        assert_eq!(
            parse_note_type_value("1024th", 0).unwrap(),
            NoteTypeValue::N1024th
        );
        assert_eq!(
            parse_note_type_value("512th", 0).unwrap(),
            NoteTypeValue::N512th
        );
        assert_eq!(
            parse_note_type_value("256th", 0).unwrap(),
            NoteTypeValue::N256th
        );
        assert_eq!(
            parse_note_type_value("128th", 0).unwrap(),
            NoteTypeValue::N128th
        );
        assert_eq!(
            parse_note_type_value("64th", 0).unwrap(),
            NoteTypeValue::N64th
        );
        assert_eq!(
            parse_note_type_value("32nd", 0).unwrap(),
            NoteTypeValue::N32nd
        );
        assert_eq!(
            parse_note_type_value("16th", 0).unwrap(),
            NoteTypeValue::N16th
        );
        assert_eq!(
            parse_note_type_value("eighth", 0).unwrap(),
            NoteTypeValue::Eighth
        );
        assert_eq!(
            parse_note_type_value("quarter", 0).unwrap(),
            NoteTypeValue::Quarter
        );
        assert_eq!(
            parse_note_type_value("half", 0).unwrap(),
            NoteTypeValue::Half
        );
        assert_eq!(
            parse_note_type_value("whole", 0).unwrap(),
            NoteTypeValue::Whole
        );
        assert_eq!(
            parse_note_type_value("breve", 0).unwrap(),
            NoteTypeValue::Breve
        );
        assert_eq!(
            parse_note_type_value("long", 0).unwrap(),
            NoteTypeValue::Long
        );
        assert_eq!(
            parse_note_type_value("maxima", 0).unwrap(),
            NoteTypeValue::Maxima
        );
    }

    #[test]
    fn test_parse_note_type_value_invalid() {
        let result = parse_note_type_value("invalid", 42);
        assert!(result.is_err());
        if let Err(ParseError::InvalidValue {
            expected,
            found,
            position,
        }) = result
        {
            assert_eq!(expected, "note-type-value");
            assert_eq!(found, "invalid");
            assert_eq!(position, 42);
        }
    }

    // === Step Tests ===

    #[test]
    fn test_parse_step_all_values() {
        assert_eq!(parse_step("A", 0).unwrap(), Step::A);
        assert_eq!(parse_step("B", 0).unwrap(), Step::B);
        assert_eq!(parse_step("C", 0).unwrap(), Step::C);
        assert_eq!(parse_step("D", 0).unwrap(), Step::D);
        assert_eq!(parse_step("E", 0).unwrap(), Step::E);
        assert_eq!(parse_step("F", 0).unwrap(), Step::F);
        assert_eq!(parse_step("G", 0).unwrap(), Step::G);
    }

    #[test]
    fn test_parse_step_invalid() {
        assert!(parse_step("H", 0).is_err());
        assert!(parse_step("a", 0).is_err()); // lowercase
        assert!(parse_step("1", 0).is_err());
    }

    // === Mode Tests ===

    #[test]
    fn test_parse_mode_all_values() {
        assert_eq!(parse_mode("major", 0).unwrap(), Mode::Major);
        assert_eq!(parse_mode("minor", 0).unwrap(), Mode::Minor);
        assert_eq!(parse_mode("dorian", 0).unwrap(), Mode::Dorian);
        assert_eq!(parse_mode("phrygian", 0).unwrap(), Mode::Phrygian);
        assert_eq!(parse_mode("lydian", 0).unwrap(), Mode::Lydian);
        assert_eq!(parse_mode("mixolydian", 0).unwrap(), Mode::Mixolydian);
        assert_eq!(parse_mode("aeolian", 0).unwrap(), Mode::Aeolian);
        assert_eq!(parse_mode("locrian", 0).unwrap(), Mode::Locrian);
        assert_eq!(parse_mode("ionian", 0).unwrap(), Mode::Ionian);
        assert_eq!(parse_mode("none", 0).unwrap(), Mode::None);
    }

    #[test]
    fn test_parse_mode_invalid() {
        assert!(parse_mode("Major", 0).is_err()); // case sensitive
        assert!(parse_mode("", 0).is_err());
    }

    // === Clef Sign Tests ===

    #[test]
    fn test_parse_clef_sign_all_values() {
        assert_eq!(parse_clef_sign("G", 0).unwrap(), ClefSign::G);
        assert_eq!(parse_clef_sign("F", 0).unwrap(), ClefSign::F);
        assert_eq!(parse_clef_sign("C", 0).unwrap(), ClefSign::C);
        assert_eq!(
            parse_clef_sign("percussion", 0).unwrap(),
            ClefSign::Percussion
        );
        assert_eq!(parse_clef_sign("TAB", 0).unwrap(), ClefSign::Tab);
        assert_eq!(parse_clef_sign("jianpu", 0).unwrap(), ClefSign::Jianpu);
        assert_eq!(parse_clef_sign("none", 0).unwrap(), ClefSign::None);
    }

    // === Time Symbol Tests ===

    #[test]
    fn test_parse_time_symbol_all_values() {
        assert_eq!(parse_time_symbol("common", 0).unwrap(), TimeSymbol::Common);
        assert_eq!(parse_time_symbol("cut", 0).unwrap(), TimeSymbol::Cut);
        assert_eq!(
            parse_time_symbol("single-number", 0).unwrap(),
            TimeSymbol::SingleNumber
        );
        assert_eq!(parse_time_symbol("note", 0).unwrap(), TimeSymbol::Note);
        assert_eq!(
            parse_time_symbol("dotted-note", 0).unwrap(),
            TimeSymbol::DottedNote
        );
        assert_eq!(parse_time_symbol("normal", 0).unwrap(), TimeSymbol::Normal);
    }

    // === Start/Stop Tests ===

    #[test]
    fn test_parse_start_stop() {
        assert_eq!(parse_start_stop("start", 0).unwrap(), StartStop::Start);
        assert_eq!(parse_start_stop("stop", 0).unwrap(), StartStop::Stop);
        assert!(parse_start_stop("continue", 0).is_err());
    }

    #[test]
    fn test_parse_start_stop_continue() {
        assert_eq!(
            parse_start_stop_continue("start", 0).unwrap(),
            StartStopContinue::Start
        );
        assert_eq!(
            parse_start_stop_continue("stop", 0).unwrap(),
            StartStopContinue::Stop
        );
        assert_eq!(
            parse_start_stop_continue("continue", 0).unwrap(),
            StartStopContinue::Continue
        );
    }

    #[test]
    fn test_parse_start_stop_single() {
        assert_eq!(
            parse_start_stop_single("start", 0).unwrap(),
            StartStopSingle::Start
        );
        assert_eq!(
            parse_start_stop_single("stop", 0).unwrap(),
            StartStopSingle::Stop
        );
        assert_eq!(
            parse_start_stop_single("single", 0).unwrap(),
            StartStopSingle::Single
        );
    }

    #[test]
    fn test_parse_start_stop_discontinue() {
        assert_eq!(
            parse_start_stop_discontinue("start", 0).unwrap(),
            StartStopDiscontinue::Start
        );
        assert_eq!(
            parse_start_stop_discontinue("stop", 0).unwrap(),
            StartStopDiscontinue::Stop
        );
        assert_eq!(
            parse_start_stop_discontinue("discontinue", 0).unwrap(),
            StartStopDiscontinue::Discontinue
        );
    }

    // === Yes/No Tests ===

    #[test]
    fn test_parse_yes_no() {
        assert_eq!(parse_yes_no("yes", 0).unwrap(), YesNo::Yes);
        assert_eq!(parse_yes_no("no", 0).unwrap(), YesNo::No);
        assert!(parse_yes_no("true", 0).is_err());
        assert!(parse_yes_no("1", 0).is_err());
    }

    // === Direction/Position Tests ===

    #[test]
    fn test_parse_above_below() {
        assert_eq!(parse_above_below("above", 0).unwrap(), AboveBelow::Above);
        assert_eq!(parse_above_below("below", 0).unwrap(), AboveBelow::Below);
    }

    #[test]
    fn test_parse_up_down() {
        assert_eq!(parse_up_down("up", 0).unwrap(), UpDown::Up);
        assert_eq!(parse_up_down("down", 0).unwrap(), UpDown::Down);
    }

    #[test]
    fn test_parse_over_under() {
        assert_eq!(parse_over_under("over", 0).unwrap(), OverUnder::Over);
        assert_eq!(parse_over_under("under", 0).unwrap(), OverUnder::Under);
    }

    #[test]
    fn test_parse_backward_forward() {
        assert_eq!(
            parse_backward_forward("backward", 0).unwrap(),
            BackwardForward::Backward
        );
        assert_eq!(
            parse_backward_forward("forward", 0).unwrap(),
            BackwardForward::Forward
        );
    }

    #[test]
    fn test_parse_right_left_middle() {
        assert_eq!(
            parse_right_left_middle("right", 0).unwrap(),
            RightLeftMiddle::Right
        );
        assert_eq!(
            parse_right_left_middle("left", 0).unwrap(),
            RightLeftMiddle::Left
        );
        assert_eq!(
            parse_right_left_middle("middle", 0).unwrap(),
            RightLeftMiddle::Middle
        );
    }

    #[test]
    fn test_parse_upright_inverted() {
        assert_eq!(
            parse_upright_inverted("upright", 0).unwrap(),
            UprightInverted::Upright
        );
        assert_eq!(
            parse_upright_inverted("inverted", 0).unwrap(),
            UprightInverted::Inverted
        );
    }

    #[test]
    fn test_parse_left_center_right() {
        assert_eq!(
            parse_left_center_right("left", 0).unwrap(),
            LeftCenterRight::Left
        );
        assert_eq!(
            parse_left_center_right("center", 0).unwrap(),
            LeftCenterRight::Center
        );
        assert_eq!(
            parse_left_center_right("right", 0).unwrap(),
            LeftCenterRight::Right
        );
    }

    #[test]
    fn test_parse_top_middle_bottom() {
        assert_eq!(
            parse_top_middle_bottom("top", 0).unwrap(),
            TopMiddleBottom::Top
        );
        assert_eq!(
            parse_top_middle_bottom("middle", 0).unwrap(),
            TopMiddleBottom::Middle
        );
        assert_eq!(
            parse_top_middle_bottom("bottom", 0).unwrap(),
            TopMiddleBottom::Bottom
        );
    }

    #[test]
    fn test_parse_top_bottom() {
        assert_eq!(parse_top_bottom("top", 0).unwrap(), TopBottom::Top);
        assert_eq!(parse_top_bottom("bottom", 0).unwrap(), TopBottom::Bottom);
    }

    // === Beam/Stem/Notehead Tests ===

    #[test]
    fn test_parse_beam_value() {
        assert_eq!(parse_beam_value("begin", 0).unwrap(), BeamValue::Begin);
        assert_eq!(
            parse_beam_value("continue", 0).unwrap(),
            BeamValue::Continue
        );
        assert_eq!(parse_beam_value("end", 0).unwrap(), BeamValue::End);
        assert_eq!(
            parse_beam_value("forward hook", 0).unwrap(),
            BeamValue::ForwardHook
        );
        assert_eq!(
            parse_beam_value("backward hook", 0).unwrap(),
            BeamValue::BackwardHook
        );
    }

    #[test]
    fn test_parse_stem_value() {
        assert_eq!(parse_stem_value("down", 0).unwrap(), StemValue::Down);
        assert_eq!(parse_stem_value("up", 0).unwrap(), StemValue::Up);
        assert_eq!(parse_stem_value("double", 0).unwrap(), StemValue::Double);
        assert_eq!(parse_stem_value("none", 0).unwrap(), StemValue::None);
    }

    #[test]
    fn test_parse_notehead_value_common() {
        assert_eq!(
            parse_notehead_value("normal", 0).unwrap(),
            NoteheadValue::Normal
        );
        assert_eq!(
            parse_notehead_value("diamond", 0).unwrap(),
            NoteheadValue::Diamond
        );
        assert_eq!(parse_notehead_value("x", 0).unwrap(), NoteheadValue::X);
        assert_eq!(
            parse_notehead_value("slash", 0).unwrap(),
            NoteheadValue::Slash
        );
    }

    #[test]
    fn test_parse_notehead_value_solfege() {
        assert_eq!(parse_notehead_value("do", 0).unwrap(), NoteheadValue::Do);
        assert_eq!(parse_notehead_value("re", 0).unwrap(), NoteheadValue::Re);
        assert_eq!(parse_notehead_value("mi", 0).unwrap(), NoteheadValue::Mi);
        assert_eq!(parse_notehead_value("fa", 0).unwrap(), NoteheadValue::Fa);
        assert_eq!(parse_notehead_value("so", 0).unwrap(), NoteheadValue::So);
        assert_eq!(parse_notehead_value("la", 0).unwrap(), NoteheadValue::La);
        assert_eq!(parse_notehead_value("ti", 0).unwrap(), NoteheadValue::Ti);
    }

    #[test]
    fn test_parse_fan() {
        assert_eq!(parse_fan("accel", 0).unwrap(), Fan::Accel);
        assert_eq!(parse_fan("rit", 0).unwrap(), Fan::Rit);
        assert_eq!(parse_fan("none", 0).unwrap(), Fan::None);
    }

    // === Accidental Tests ===

    #[test]
    fn test_parse_accidental_value_basic() {
        assert_eq!(
            parse_accidental_value("sharp", 0).unwrap(),
            AccidentalValue::Sharp
        );
        assert_eq!(
            parse_accidental_value("natural", 0).unwrap(),
            AccidentalValue::Natural
        );
        assert_eq!(
            parse_accidental_value("flat", 0).unwrap(),
            AccidentalValue::Flat
        );
    }

    #[test]
    fn test_parse_accidental_value_double() {
        assert_eq!(
            parse_accidental_value("double-sharp", 0).unwrap(),
            AccidentalValue::DoubleSharp
        );
        assert_eq!(
            parse_accidental_value("double-flat", 0).unwrap(),
            AccidentalValue::DoubleFlat
        );
    }

    #[test]
    fn test_parse_accidental_value_microtonal() {
        assert_eq!(
            parse_accidental_value("quarter-sharp", 0).unwrap(),
            AccidentalValue::QuarterSharp
        );
        assert_eq!(
            parse_accidental_value("quarter-flat", 0).unwrap(),
            AccidentalValue::QuarterFlat
        );
        assert_eq!(
            parse_accidental_value("three-quarters-sharp", 0).unwrap(),
            AccidentalValue::ThreeQuartersSharp
        );
    }

    // === Bar Style Tests ===

    #[test]
    fn test_parse_bar_style() {
        assert_eq!(parse_bar_style("regular", 0).unwrap(), BarStyle::Regular);
        assert_eq!(
            parse_bar_style("light-heavy", 0).unwrap(),
            BarStyle::LightHeavy
        );
        assert_eq!(
            parse_bar_style("heavy-light", 0).unwrap(),
            BarStyle::HeavyLight
        );
        assert_eq!(parse_bar_style("none", 0).unwrap(), BarStyle::None);
    }

    #[test]
    fn test_parse_winged() {
        assert_eq!(parse_winged("none", 0).unwrap(), Winged::None);
        assert_eq!(parse_winged("straight", 0).unwrap(), Winged::Straight);
        assert_eq!(parse_winged("curved", 0).unwrap(), Winged::Curved);
    }

    #[test]
    fn test_parse_cancel_location() {
        assert_eq!(
            parse_cancel_location("left", 0).unwrap(),
            CancelLocation::Left
        );
        assert_eq!(
            parse_cancel_location("right", 0).unwrap(),
            CancelLocation::Right
        );
        assert_eq!(
            parse_cancel_location("before-barline", 0).unwrap(),
            CancelLocation::BeforeBarline
        );
    }

    // === Notation Type Tests ===

    #[test]
    fn test_parse_fermata_shape() {
        assert_eq!(
            parse_fermata_shape("normal", 0).unwrap(),
            FermataShape::Normal
        );
        assert_eq!(parse_fermata_shape("", 0).unwrap(), FermataShape::Normal);
        assert_eq!(
            parse_fermata_shape("angled", 0).unwrap(),
            FermataShape::Angled
        );
        assert_eq!(
            parse_fermata_shape("square", 0).unwrap(),
            FermataShape::Square
        );
    }

    #[test]
    fn test_parse_breath_mark_value() {
        assert_eq!(
            parse_breath_mark_value("", 0).unwrap(),
            BreathMarkValue::Empty
        );
        assert_eq!(
            parse_breath_mark_value("comma", 0).unwrap(),
            BreathMarkValue::Comma
        );
        assert_eq!(
            parse_breath_mark_value("tick", 0).unwrap(),
            BreathMarkValue::Tick
        );
    }

    #[test]
    fn test_parse_caesura_value() {
        assert_eq!(
            parse_caesura_value("normal", 0).unwrap(),
            CaesuraValue::Normal
        );
        assert_eq!(
            parse_caesura_value("thick", 0).unwrap(),
            CaesuraValue::Thick
        );
        assert_eq!(
            parse_caesura_value("short", 0).unwrap(),
            CaesuraValue::Short
        );
    }

    #[test]
    fn test_parse_line_length() {
        assert_eq!(parse_line_length("short", 0).unwrap(), LineLength::Short);
        assert_eq!(parse_line_length("medium", 0).unwrap(), LineLength::Medium);
        assert_eq!(parse_line_length("long", 0).unwrap(), LineLength::Long);
    }

    #[test]
    fn test_parse_line_shape() {
        assert_eq!(
            parse_line_shape("straight", 0).unwrap(),
            LineShape::Straight
        );
        assert_eq!(parse_line_shape("curved", 0).unwrap(), LineShape::Curved);
    }

    #[test]
    fn test_parse_line_type() {
        assert_eq!(parse_line_type("solid", 0).unwrap(), LineType::Solid);
        assert_eq!(parse_line_type("dashed", 0).unwrap(), LineType::Dashed);
        assert_eq!(parse_line_type("dotted", 0).unwrap(), LineType::Dotted);
        assert_eq!(parse_line_type("wavy", 0).unwrap(), LineType::Wavy);
    }

    #[test]
    fn test_parse_show_tuplet() {
        assert_eq!(parse_show_tuplet("actual", 0).unwrap(), ShowTuplet::Actual);
        assert_eq!(parse_show_tuplet("both", 0).unwrap(), ShowTuplet::Both);
        assert_eq!(parse_show_tuplet("none", 0).unwrap(), ShowTuplet::None);
    }

    // === Ornament Type Tests ===

    #[test]
    fn test_parse_start_note() {
        assert_eq!(parse_start_note("upper", 0).unwrap(), StartNote::Upper);
        assert_eq!(parse_start_note("main", 0).unwrap(), StartNote::Main);
        assert_eq!(parse_start_note("below", 0).unwrap(), StartNote::Below);
    }

    #[test]
    fn test_parse_trill_step() {
        assert_eq!(parse_trill_step("whole", 0).unwrap(), TrillStep::Whole);
        assert_eq!(parse_trill_step("half", 0).unwrap(), TrillStep::Half);
        assert_eq!(parse_trill_step("unison", 0).unwrap(), TrillStep::Unison);
    }

    #[test]
    fn test_parse_two_note_turn() {
        assert_eq!(parse_two_note_turn("whole", 0).unwrap(), TwoNoteTurn::Whole);
        assert_eq!(parse_two_note_turn("half", 0).unwrap(), TwoNoteTurn::Half);
        assert_eq!(parse_two_note_turn("none", 0).unwrap(), TwoNoteTurn::None);
    }

    #[test]
    fn test_parse_tremolo_type() {
        assert_eq!(parse_tremolo_type("start", 0).unwrap(), TremoloType::Start);
        assert_eq!(parse_tremolo_type("stop", 0).unwrap(), TremoloType::Stop);
        assert_eq!(
            parse_tremolo_type("single", 0).unwrap(),
            TremoloType::Single
        );
        assert_eq!(
            parse_tremolo_type("unmeasured", 0).unwrap(),
            TremoloType::Unmeasured
        );
    }

    // === Direction Type Tests ===

    #[test]
    fn test_parse_wedge_type() {
        assert_eq!(
            parse_wedge_type("crescendo", 0).unwrap(),
            WedgeType::Crescendo
        );
        assert_eq!(
            parse_wedge_type("diminuendo", 0).unwrap(),
            WedgeType::Diminuendo
        );
        assert_eq!(parse_wedge_type("stop", 0).unwrap(), WedgeType::Stop);
        assert_eq!(
            parse_wedge_type("continue", 0).unwrap(),
            WedgeType::Continue
        );
    }

    #[test]
    fn test_parse_pedal_type() {
        assert_eq!(parse_pedal_type("start", 0).unwrap(), PedalType::Start);
        assert_eq!(parse_pedal_type("stop", 0).unwrap(), PedalType::Stop);
        assert_eq!(
            parse_pedal_type("sostenuto", 0).unwrap(),
            PedalType::Sostenuto
        );
        assert_eq!(parse_pedal_type("change", 0).unwrap(), PedalType::Change);
    }

    #[test]
    fn test_parse_up_down_stop_continue() {
        assert_eq!(
            parse_up_down_stop_continue("up", 0).unwrap(),
            UpDownStopContinue::Up
        );
        assert_eq!(
            parse_up_down_stop_continue("down", 0).unwrap(),
            UpDownStopContinue::Down
        );
        assert_eq!(
            parse_up_down_stop_continue("stop", 0).unwrap(),
            UpDownStopContinue::Stop
        );
        assert_eq!(
            parse_up_down_stop_continue("continue", 0).unwrap(),
            UpDownStopContinue::Continue
        );
    }

    // === Lyric Tests ===

    #[test]
    fn test_parse_syllabic() {
        assert_eq!(parse_syllabic("single", 0).unwrap(), Syllabic::Single);
        assert_eq!(parse_syllabic("begin", 0).unwrap(), Syllabic::Begin);
        assert_eq!(parse_syllabic("end", 0).unwrap(), Syllabic::End);
        assert_eq!(parse_syllabic("middle", 0).unwrap(), Syllabic::Middle);
    }

    // === Score Type Tests ===

    #[test]
    fn test_parse_margin_type() {
        assert_eq!(parse_margin_type("odd", 0).unwrap(), MarginType::Odd);
        assert_eq!(parse_margin_type("even", 0).unwrap(), MarginType::Even);
        assert_eq!(parse_margin_type("both", 0).unwrap(), MarginType::Both);
    }

    #[test]
    fn test_parse_note_size_type() {
        assert_eq!(parse_note_size_type("cue", 0).unwrap(), NoteSizeType::Cue);
        assert_eq!(
            parse_note_size_type("grace", 0).unwrap(),
            NoteSizeType::Grace
        );
        assert_eq!(
            parse_note_size_type("grace-cue", 0).unwrap(),
            NoteSizeType::GraceCue
        );
        assert_eq!(
            parse_note_size_type("large", 0).unwrap(),
            NoteSizeType::Large
        );
    }

    // === Font Size Tests ===

    #[test]
    fn test_parse_css_font_size() {
        assert_eq!(
            parse_css_font_size("xx-small", 0).unwrap(),
            CssFontSize::XxSmall
        );
        assert_eq!(
            parse_css_font_size("x-small", 0).unwrap(),
            CssFontSize::XSmall
        );
        assert_eq!(parse_css_font_size("small", 0).unwrap(), CssFontSize::Small);
        assert_eq!(
            parse_css_font_size("medium", 0).unwrap(),
            CssFontSize::Medium
        );
        assert_eq!(parse_css_font_size("large", 0).unwrap(), CssFontSize::Large);
        assert_eq!(
            parse_css_font_size("x-large", 0).unwrap(),
            CssFontSize::XLarge
        );
        assert_eq!(
            parse_css_font_size("xx-large", 0).unwrap(),
            CssFontSize::XxLarge
        );
    }

    #[test]
    fn test_parse_font_size_css() {
        assert_eq!(
            parse_font_size("medium", 0).unwrap(),
            FontSize::Css(CssFontSize::Medium)
        );
    }

    #[test]
    fn test_parse_font_size_points() {
        assert_eq!(parse_font_size("12", 0).unwrap(), FontSize::Points(12.0));
        assert_eq!(parse_font_size("10.5", 0).unwrap(), FontSize::Points(10.5));
    }

    #[test]
    fn test_parse_font_size_invalid() {
        assert!(parse_font_size("huge", 0).is_err());
        assert!(parse_font_size("abc", 0).is_err());
    }

    // === Technical Type Tests ===

    #[test]
    fn test_parse_handbell_value() {
        assert_eq!(
            parse_handbell_value("belltree", 0).unwrap(),
            HandbellValue::Belltree
        );
        assert_eq!(
            parse_handbell_value("damp", 0).unwrap(),
            HandbellValue::Damp
        );
        assert_eq!(
            parse_handbell_value("martellato", 0).unwrap(),
            HandbellValue::Martellato
        );
    }

    #[test]
    fn test_parse_arrow_direction() {
        assert_eq!(
            parse_arrow_direction("left", 0).unwrap(),
            ArrowDirection::Left
        );
        assert_eq!(parse_arrow_direction("up", 0).unwrap(), ArrowDirection::Up);
        assert_eq!(
            parse_arrow_direction("northwest", 0).unwrap(),
            ArrowDirection::Northwest
        );
    }

    #[test]
    fn test_parse_arrow_style() {
        assert_eq!(parse_arrow_style("single", 0).unwrap(), ArrowStyle::Single);
        assert_eq!(parse_arrow_style("double", 0).unwrap(), ArrowStyle::Double);
        assert_eq!(parse_arrow_style("filled", 0).unwrap(), ArrowStyle::Filled);
    }

    #[test]
    fn test_parse_hole_closed_value() {
        assert_eq!(
            parse_hole_closed_value("yes", 0).unwrap(),
            HoleClosedValue::Yes
        );
        assert_eq!(
            parse_hole_closed_value("no", 0).unwrap(),
            HoleClosedValue::No
        );
        assert_eq!(
            parse_hole_closed_value("half", 0).unwrap(),
            HoleClosedValue::Half
        );
    }

    #[test]
    fn test_parse_hole_closed_location() {
        assert_eq!(
            parse_hole_closed_location("right", 0).unwrap(),
            HoleClosedLocation::Right
        );
        assert_eq!(
            parse_hole_closed_location("bottom", 0).unwrap(),
            HoleClosedLocation::Bottom
        );
        assert_eq!(
            parse_hole_closed_location("left", 0).unwrap(),
            HoleClosedLocation::Left
        );
        assert_eq!(
            parse_hole_closed_location("top", 0).unwrap(),
            HoleClosedLocation::Top
        );
    }

    #[test]
    fn test_parse_tap_hand() {
        assert_eq!(parse_tap_hand("left", 0).unwrap(), TapHand::Left);
        assert_eq!(parse_tap_hand("right", 0).unwrap(), TapHand::Right);
    }

    // =======================================================================
    // Additional tests for uncovered paths and error cases
    // =======================================================================

    // === Error case tests for all parsers ===

    #[test]
    fn test_parse_clef_sign_invalid() {
        let result = parse_clef_sign("invalid", 42);
        assert!(result.is_err());
        if let Err(ParseError::InvalidValue {
            expected,
            found,
            position,
        }) = result
        {
            assert_eq!(expected, "clef-sign");
            assert_eq!(found, "invalid");
            assert_eq!(position, 42);
        }
    }

    #[test]
    fn test_parse_time_symbol_invalid() {
        let result = parse_time_symbol("invalid", 100);
        assert!(result.is_err());
        if let Err(ParseError::InvalidValue {
            expected,
            found,
            position,
        }) = result
        {
            assert_eq!(expected, "time-symbol");
            assert_eq!(found, "invalid");
            assert_eq!(position, 100);
        }
    }

    #[test]
    fn test_parse_start_stop_continue_invalid() {
        let result = parse_start_stop_continue("pause", 0);
        assert!(result.is_err());
        if let Err(ParseError::InvalidValue { expected, .. }) = result {
            assert_eq!(expected, "start-stop-continue");
        }
    }

    #[test]
    fn test_parse_start_stop_single_invalid() {
        let result = parse_start_stop_single("double", 0);
        assert!(result.is_err());
        if let Err(ParseError::InvalidValue { expected, .. }) = result {
            assert_eq!(expected, "start-stop-single");
        }
    }

    #[test]
    fn test_parse_start_stop_discontinue_invalid() {
        let result = parse_start_stop_discontinue("pause", 0);
        assert!(result.is_err());
        if let Err(ParseError::InvalidValue { expected, .. }) = result {
            assert_eq!(expected, "start-stop-discontinue");
        }
    }

    #[test]
    fn test_parse_above_below_invalid() {
        let result = parse_above_below("middle", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_up_down_invalid() {
        let result = parse_up_down("left", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_over_under_invalid() {
        let result = parse_over_under("above", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_backward_forward_invalid() {
        let result = parse_backward_forward("up", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_right_left_middle_invalid() {
        let result = parse_right_left_middle("center", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_upright_inverted_invalid() {
        let result = parse_upright_inverted("tilted", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_left_center_right_invalid() {
        let result = parse_left_center_right("middle", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_top_middle_bottom_invalid() {
        let result = parse_top_middle_bottom("center", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_top_bottom_invalid() {
        let result = parse_top_bottom("middle", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_beam_value_invalid() {
        let result = parse_beam_value("start", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_stem_value_invalid() {
        let result = parse_stem_value("left", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fan_invalid() {
        let result = parse_fan("fast", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bar_style_invalid() {
        let result = parse_bar_style("solid", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_winged_invalid() {
        let result = parse_winged("single", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_cancel_location_invalid() {
        let result = parse_cancel_location("center", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fermata_shape_invalid() {
        let result = parse_fermata_shape("triangle", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_breath_mark_value_invalid() {
        let result = parse_breath_mark_value("downbow", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_caesura_value_invalid() {
        let result = parse_caesura_value("long", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_length_invalid() {
        let result = parse_line_length("extra-long", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_shape_invalid() {
        let result = parse_line_shape("wavy", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_type_invalid() {
        let result = parse_line_type("thick", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_show_tuplet_invalid() {
        let result = parse_show_tuplet("hidden", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_start_note_invalid() {
        let result = parse_start_note("auxiliary", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_trill_step_invalid() {
        let result = parse_trill_step("quarter", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_two_note_turn_invalid() {
        let result = parse_two_note_turn("third", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tremolo_type_invalid() {
        let result = parse_tremolo_type("double", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_wedge_type_invalid() {
        let result = parse_wedge_type("accent", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_pedal_type_invalid() {
        let result = parse_pedal_type("hold", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_up_down_stop_continue_invalid() {
        let result = parse_up_down_stop_continue("left", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_syllabic_invalid() {
        let result = parse_syllabic("start", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_margin_type_invalid() {
        let result = parse_margin_type("left", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_note_size_type_invalid() {
        let result = parse_note_size_type("small", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_css_font_size_invalid() {
        let result = parse_css_font_size("normal", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_handbell_value_invalid() {
        let result = parse_handbell_value("ring", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_arrow_direction_invalid() {
        let result = parse_arrow_direction("diagonal", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_arrow_style_invalid() {
        let result = parse_arrow_style("triple", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_hole_closed_value_invalid() {
        let result = parse_hole_closed_value("open", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_hole_closed_location_invalid() {
        let result = parse_hole_closed_location("center", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tap_hand_invalid() {
        let result = parse_tap_hand("both", 0);
        assert!(result.is_err());
    }

    // === Comprehensive coverage for all enum variants ===

    #[test]
    fn test_parse_notehead_value_all_variants() {
        // Test all remaining notehead variants not already covered
        assert_eq!(
            parse_notehead_value("triangle", 0).unwrap(),
            NoteheadValue::Triangle
        );
        assert_eq!(
            parse_notehead_value("square", 0).unwrap(),
            NoteheadValue::Square
        );
        assert_eq!(
            parse_notehead_value("cross", 0).unwrap(),
            NoteheadValue::Cross
        );
        assert_eq!(
            parse_notehead_value("circle-x", 0).unwrap(),
            NoteheadValue::CircleX
        );
        assert_eq!(
            parse_notehead_value("inverted triangle", 0).unwrap(),
            NoteheadValue::InvertedTriangle
        );
        assert_eq!(
            parse_notehead_value("arrow down", 0).unwrap(),
            NoteheadValue::ArrowDown
        );
        assert_eq!(
            parse_notehead_value("arrow up", 0).unwrap(),
            NoteheadValue::ArrowUp
        );
        assert_eq!(
            parse_notehead_value("circled", 0).unwrap(),
            NoteheadValue::Circled
        );
        assert_eq!(
            parse_notehead_value("slashed", 0).unwrap(),
            NoteheadValue::Slashed
        );
        assert_eq!(
            parse_notehead_value("back slashed", 0).unwrap(),
            NoteheadValue::BackSlashed
        );
        assert_eq!(
            parse_notehead_value("cluster", 0).unwrap(),
            NoteheadValue::Cluster
        );
        assert_eq!(
            parse_notehead_value("circle dot", 0).unwrap(),
            NoteheadValue::CircleDot
        );
        assert_eq!(
            parse_notehead_value("left triangle", 0).unwrap(),
            NoteheadValue::LeftTriangle
        );
        assert_eq!(
            parse_notehead_value("rectangle", 0).unwrap(),
            NoteheadValue::Rectangle
        );
        assert_eq!(
            parse_notehead_value("none", 0).unwrap(),
            NoteheadValue::None
        );
        assert_eq!(
            parse_notehead_value("fa up", 0).unwrap(),
            NoteheadValue::FaUp
        );
        assert_eq!(
            parse_notehead_value("other", 0).unwrap(),
            NoteheadValue::Other
        );
    }

    #[test]
    fn test_parse_notehead_value_invalid() {
        let result = parse_notehead_value("star", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_accidental_value_all_variants() {
        // Test remaining accidental variants
        assert_eq!(
            parse_accidental_value("sharp-sharp", 0).unwrap(),
            AccidentalValue::SharpSharp
        );
        assert_eq!(
            parse_accidental_value("flat-flat", 0).unwrap(),
            AccidentalValue::FlatFlat
        );
        assert_eq!(
            parse_accidental_value("natural-sharp", 0).unwrap(),
            AccidentalValue::NaturalSharp
        );
        assert_eq!(
            parse_accidental_value("natural-flat", 0).unwrap(),
            AccidentalValue::NaturalFlat
        );
        assert_eq!(
            parse_accidental_value("sharp-down", 0).unwrap(),
            AccidentalValue::SharpDown
        );
        assert_eq!(
            parse_accidental_value("sharp-up", 0).unwrap(),
            AccidentalValue::SharpUp
        );
        assert_eq!(
            parse_accidental_value("natural-down", 0).unwrap(),
            AccidentalValue::NaturalDown
        );
        assert_eq!(
            parse_accidental_value("natural-up", 0).unwrap(),
            AccidentalValue::NaturalUp
        );
        assert_eq!(
            parse_accidental_value("flat-down", 0).unwrap(),
            AccidentalValue::FlatDown
        );
        assert_eq!(
            parse_accidental_value("flat-up", 0).unwrap(),
            AccidentalValue::FlatUp
        );
        assert_eq!(
            parse_accidental_value("triple-sharp", 0).unwrap(),
            AccidentalValue::TripleSharp
        );
        assert_eq!(
            parse_accidental_value("triple-flat", 0).unwrap(),
            AccidentalValue::TripleFlat
        );
        assert_eq!(
            parse_accidental_value("slash-quarter-sharp", 0).unwrap(),
            AccidentalValue::SlashQuarterSharp
        );
        assert_eq!(
            parse_accidental_value("slash-sharp", 0).unwrap(),
            AccidentalValue::SlashSharp
        );
        assert_eq!(
            parse_accidental_value("slash-flat", 0).unwrap(),
            AccidentalValue::SlashFlat
        );
        assert_eq!(
            parse_accidental_value("double-slash-flat", 0).unwrap(),
            AccidentalValue::DoubleSlashFlat
        );
        assert_eq!(
            parse_accidental_value("sharp-1", 0).unwrap(),
            AccidentalValue::Sharp1
        );
        assert_eq!(
            parse_accidental_value("sharp-2", 0).unwrap(),
            AccidentalValue::Sharp2
        );
        assert_eq!(
            parse_accidental_value("sharp-3", 0).unwrap(),
            AccidentalValue::Sharp3
        );
        assert_eq!(
            parse_accidental_value("sharp-5", 0).unwrap(),
            AccidentalValue::Sharp5
        );
        assert_eq!(
            parse_accidental_value("flat-1", 0).unwrap(),
            AccidentalValue::Flat1
        );
        assert_eq!(
            parse_accidental_value("flat-2", 0).unwrap(),
            AccidentalValue::Flat2
        );
        assert_eq!(
            parse_accidental_value("flat-3", 0).unwrap(),
            AccidentalValue::Flat3
        );
        assert_eq!(
            parse_accidental_value("flat-4", 0).unwrap(),
            AccidentalValue::Flat4
        );
        assert_eq!(
            parse_accidental_value("sori", 0).unwrap(),
            AccidentalValue::Sori
        );
        assert_eq!(
            parse_accidental_value("koron", 0).unwrap(),
            AccidentalValue::Koron
        );
        assert_eq!(
            parse_accidental_value("other", 0).unwrap(),
            AccidentalValue::Other
        );
    }

    #[test]
    fn test_parse_accidental_value_invalid() {
        let result = parse_accidental_value("neutral", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bar_style_all_variants() {
        assert_eq!(parse_bar_style("dotted", 0).unwrap(), BarStyle::Dotted);
        assert_eq!(parse_bar_style("dashed", 0).unwrap(), BarStyle::Dashed);
        assert_eq!(parse_bar_style("heavy", 0).unwrap(), BarStyle::Heavy);
        assert_eq!(
            parse_bar_style("light-light", 0).unwrap(),
            BarStyle::LightLight
        );
        assert_eq!(
            parse_bar_style("heavy-heavy", 0).unwrap(),
            BarStyle::HeavyHeavy
        );
        assert_eq!(parse_bar_style("tick", 0).unwrap(), BarStyle::Tick);
        assert_eq!(parse_bar_style("short", 0).unwrap(), BarStyle::Short);
    }

    #[test]
    fn test_parse_winged_all_variants() {
        assert_eq!(
            parse_winged("double-straight", 0).unwrap(),
            Winged::DoubleStraight
        );
        assert_eq!(
            parse_winged("double-curved", 0).unwrap(),
            Winged::DoubleCurved
        );
    }

    #[test]
    fn test_parse_fermata_shape_all_variants() {
        assert_eq!(
            parse_fermata_shape("double-angled", 0).unwrap(),
            FermataShape::DoubleAngled
        );
        assert_eq!(
            parse_fermata_shape("double-square", 0).unwrap(),
            FermataShape::DoubleSquare
        );
        assert_eq!(
            parse_fermata_shape("double-dot", 0).unwrap(),
            FermataShape::DoubleDot
        );
        assert_eq!(
            parse_fermata_shape("half-curve", 0).unwrap(),
            FermataShape::HalfCurve
        );
        assert_eq!(
            parse_fermata_shape("curlew", 0).unwrap(),
            FermataShape::Curlew
        );
    }

    #[test]
    fn test_parse_breath_mark_value_all_variants() {
        assert_eq!(
            parse_breath_mark_value("upbow", 0).unwrap(),
            BreathMarkValue::Upbow
        );
        assert_eq!(
            parse_breath_mark_value("salzedo", 0).unwrap(),
            BreathMarkValue::Salzedo
        );
    }

    #[test]
    fn test_parse_caesura_value_all_variants() {
        assert_eq!(
            parse_caesura_value("curved", 0).unwrap(),
            CaesuraValue::Curved
        );
        assert_eq!(
            parse_caesura_value("single", 0).unwrap(),
            CaesuraValue::Single
        );
    }

    #[test]
    fn test_parse_handbell_value_all_variants() {
        assert_eq!(
            parse_handbell_value("echo", 0).unwrap(),
            HandbellValue::Echo
        );
        assert_eq!(
            parse_handbell_value("gyro", 0).unwrap(),
            HandbellValue::Gyro
        );
        assert_eq!(
            parse_handbell_value("hand martellato", 0).unwrap(),
            HandbellValue::HandMartellato
        );
        assert_eq!(
            parse_handbell_value("mallet lift", 0).unwrap(),
            HandbellValue::MalletLift
        );
        assert_eq!(
            parse_handbell_value("mallet table", 0).unwrap(),
            HandbellValue::MalletTable
        );
        assert_eq!(
            parse_handbell_value("martellato lift", 0).unwrap(),
            HandbellValue::MartellatoLift
        );
        assert_eq!(
            parse_handbell_value("muted martellato", 0).unwrap(),
            HandbellValue::MutedMartellato
        );
        assert_eq!(
            parse_handbell_value("pluck lift", 0).unwrap(),
            HandbellValue::PluckLift
        );
        assert_eq!(
            parse_handbell_value("swing", 0).unwrap(),
            HandbellValue::Swing
        );
    }

    #[test]
    fn test_parse_arrow_direction_all_variants() {
        assert_eq!(
            parse_arrow_direction("right", 0).unwrap(),
            ArrowDirection::Right
        );
        assert_eq!(
            parse_arrow_direction("down", 0).unwrap(),
            ArrowDirection::Down
        );
        assert_eq!(
            parse_arrow_direction("northeast", 0).unwrap(),
            ArrowDirection::Northeast
        );
        assert_eq!(
            parse_arrow_direction("southeast", 0).unwrap(),
            ArrowDirection::Southeast
        );
        assert_eq!(
            parse_arrow_direction("southwest", 0).unwrap(),
            ArrowDirection::Southwest
        );
        assert_eq!(
            parse_arrow_direction("left right", 0).unwrap(),
            ArrowDirection::LeftRight
        );
        assert_eq!(
            parse_arrow_direction("up down", 0).unwrap(),
            ArrowDirection::UpDown
        );
        assert_eq!(
            parse_arrow_direction("northwest southeast", 0).unwrap(),
            ArrowDirection::NorthwestSoutheast
        );
        assert_eq!(
            parse_arrow_direction("northeast southwest", 0).unwrap(),
            ArrowDirection::NortheastSouthwest
        );
        assert_eq!(
            parse_arrow_direction("other", 0).unwrap(),
            ArrowDirection::Other
        );
    }

    #[test]
    fn test_parse_arrow_style_all_variants() {
        assert_eq!(parse_arrow_style("hollow", 0).unwrap(), ArrowStyle::Hollow);
        assert_eq!(parse_arrow_style("paired", 0).unwrap(), ArrowStyle::Paired);
        assert_eq!(
            parse_arrow_style("combined", 0).unwrap(),
            ArrowStyle::Combined
        );
        assert_eq!(parse_arrow_style("other", 0).unwrap(), ArrowStyle::Other);
    }

    #[test]
    fn test_parse_pedal_type_all_variants() {
        assert_eq!(
            parse_pedal_type("continue", 0).unwrap(),
            PedalType::Continue
        );
        assert_eq!(
            parse_pedal_type("discontinue", 0).unwrap(),
            PedalType::Discontinue
        );
        assert_eq!(parse_pedal_type("resume", 0).unwrap(), PedalType::Resume);
    }

    // === Edge case tests ===

    #[test]
    fn test_parse_font_size_with_negative() {
        // Negative numbers should parse as points (valid in XML, but semantically odd)
        assert_eq!(parse_font_size("-12", 0).unwrap(), FontSize::Points(-12.0));
    }

    #[test]
    fn test_parse_font_size_with_zero() {
        assert_eq!(parse_font_size("0", 0).unwrap(), FontSize::Points(0.0));
    }

    #[test]
    fn test_error_positions_preserved() {
        // Verify that error positions are correctly preserved
        let result = parse_step("X", 12345);
        if let Err(ParseError::InvalidValue { position, .. }) = result {
            assert_eq!(position, 12345);
        } else {
            panic!("Expected InvalidValue error");
        }
    }
}
