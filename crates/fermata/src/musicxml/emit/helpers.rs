//! String conversion helper functions for MusicXML emission.
//!
//! This module contains all the conversion functions that transform IR enum values
//! to their MusicXML string representations.

use crate::ir::NoteTypeValue;
use crate::ir::attributes::{BarStyle, CancelLocation, ClefSign, Mode, TimeSymbol, Winged};
use crate::ir::beam::{BeamValue, Fan, NoteheadValue, StemValue};
use crate::ir::common::{
    AboveBelow, AccidentalValue, BackwardForward, LineType, OverUnder, RightLeftMiddle, StartStop,
    StartStopContinue, StartStopDiscontinue, StartStopSingle, UpDown, UprightInverted, YesNo,
};
use crate::ir::direction::{PedalType, UpDownStopContinue, WedgeType};
use crate::ir::notation::{
    BreathMarkValue, CaesuraValue, FermataShape, LineLength, LineShape, ShowTuplet, TopBottom,
};
use crate::ir::pitch::Step;

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
pub(crate) fn step_to_string(step: &Step) -> &'static str {
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
pub(crate) fn mode_to_string(mode: &Mode) -> &'static str {
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
pub(crate) fn clef_sign_to_string(sign: &ClefSign) -> &'static str {
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
pub(crate) fn time_symbol_to_string(symbol: &TimeSymbol) -> &'static str {
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
pub(crate) fn start_stop_to_string(ss: &StartStop) -> &'static str {
    match ss {
        StartStop::Start => "start",
        StartStop::Stop => "stop",
    }
}

/// Convert a BeamValue to its MusicXML string representation.
pub(crate) fn beam_value_to_string(value: &BeamValue) -> &'static str {
    match value {
        BeamValue::Begin => "begin",
        BeamValue::Continue => "continue",
        BeamValue::End => "end",
        BeamValue::ForwardHook => "forward hook",
        BeamValue::BackwardHook => "backward hook",
    }
}

/// Convert a StemValue to its MusicXML string representation.
pub(crate) fn stem_value_to_string(value: &StemValue) -> &'static str {
    match value {
        StemValue::Down => "down",
        StemValue::Up => "up",
        StemValue::Double => "double",
        StemValue::None => "none",
    }
}

/// Convert a NoteheadValue to its MusicXML string representation.
pub(crate) fn notehead_value_to_string(value: &NoteheadValue) -> &'static str {
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
pub(crate) fn accidental_value_to_string(value: &AccidentalValue) -> &'static str {
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
pub(crate) fn yes_no_to_string(yn: &YesNo) -> &'static str {
    match yn {
        YesNo::Yes => "yes",
        YesNo::No => "no",
    }
}

/// Convert a BarStyle to its MusicXML string representation.
pub(crate) fn bar_style_to_string(style: &BarStyle) -> &'static str {
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
pub(crate) fn backward_forward_to_string(bf: &BackwardForward) -> &'static str {
    match bf {
        BackwardForward::Backward => "backward",
        BackwardForward::Forward => "forward",
    }
}

/// Convert a RightLeftMiddle to its MusicXML string representation.
pub(crate) fn right_left_middle_to_string(rlm: &RightLeftMiddle) -> &'static str {
    match rlm {
        RightLeftMiddle::Right => "right",
        RightLeftMiddle::Left => "left",
        RightLeftMiddle::Middle => "middle",
    }
}

/// Convert a StartStopDiscontinue to its MusicXML string representation.
pub(crate) fn start_stop_discontinue_to_string(ssd: &StartStopDiscontinue) -> &'static str {
    match ssd {
        StartStopDiscontinue::Start => "start",
        StartStopDiscontinue::Stop => "stop",
        StartStopDiscontinue::Discontinue => "discontinue",
    }
}

/// Convert a Winged to its MusicXML string representation.
pub(crate) fn winged_to_string(winged: &Winged) -> &'static str {
    match winged {
        Winged::None => "none",
        Winged::Straight => "straight",
        Winged::Curved => "curved",
        Winged::DoubleStraight => "double-straight",
        Winged::DoubleCurved => "double-curved",
    }
}

/// Convert an UprightInverted to its MusicXML string representation.
pub(crate) fn upright_inverted_to_string(ui: &UprightInverted) -> &'static str {
    match ui {
        UprightInverted::Upright => "upright",
        UprightInverted::Inverted => "inverted",
    }
}

/// Convert a FermataShape to its MusicXML string representation.
pub(crate) fn fermata_shape_to_string(shape: &FermataShape) -> &'static str {
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
pub(crate) fn cancel_location_to_string(loc: &CancelLocation) -> &'static str {
    match loc {
        CancelLocation::Left => "left",
        CancelLocation::Right => "right",
        CancelLocation::BeforeBarline => "before-barline",
    }
}

/// Convert a Fan to its MusicXML string representation.
pub(crate) fn fan_to_string(fan: &Fan) -> &'static str {
    match fan {
        Fan::Accel => "accel",
        Fan::Rit => "rit",
        Fan::None => "none",
    }
}

/// Convert an AboveBelow to its MusicXML string representation.
pub(crate) fn above_below_to_string(ab: &AboveBelow) -> &'static str {
    match ab {
        AboveBelow::Above => "above",
        AboveBelow::Below => "below",
    }
}

/// Convert a StartStopContinue to its MusicXML string representation.
pub(crate) fn start_stop_continue_to_string(ssc: &StartStopContinue) -> &'static str {
    match ssc {
        StartStopContinue::Start => "start",
        StartStopContinue::Stop => "stop",
        StartStopContinue::Continue => "continue",
    }
}

/// Convert a WedgeType to its MusicXML string representation.
pub(crate) fn wedge_type_to_string(wt: &WedgeType) -> &'static str {
    match wt {
        WedgeType::Crescendo => "crescendo",
        WedgeType::Diminuendo => "diminuendo",
        WedgeType::Stop => "stop",
        WedgeType::Continue => "continue",
    }
}

/// Convert a LineType to its MusicXML string representation.
pub(crate) fn line_type_to_string(lt: &LineType) -> &'static str {
    match lt {
        LineType::Solid => "solid",
        LineType::Dashed => "dashed",
        LineType::Dotted => "dotted",
        LineType::Wavy => "wavy",
    }
}

/// Convert an UpDown to its MusicXML string representation.
pub(crate) fn up_down_to_string(ud: &UpDown) -> &'static str {
    match ud {
        UpDown::Up => "up",
        UpDown::Down => "down",
    }
}

/// Convert a TopBottom to its MusicXML string representation.
pub(crate) fn top_bottom_to_string(tb: &TopBottom) -> &'static str {
    match tb {
        TopBottom::Top => "top",
        TopBottom::Bottom => "bottom",
    }
}

/// Convert a ShowTuplet to its MusicXML string representation.
pub(crate) fn show_tuplet_to_string(st: &ShowTuplet) -> &'static str {
    match st {
        ShowTuplet::Actual => "actual",
        ShowTuplet::Both => "both",
        ShowTuplet::None => "none",
    }
}

/// Convert a LineShape to its MusicXML string representation.
pub(crate) fn line_shape_to_string(ls: &LineShape) -> &'static str {
    match ls {
        LineShape::Straight => "straight",
        LineShape::Curved => "curved",
    }
}

/// Convert a PedalType to its MusicXML string representation.
pub(crate) fn pedal_type_to_string(pt: &PedalType) -> &'static str {
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
pub(crate) fn up_down_stop_continue_to_string(udsc: &UpDownStopContinue) -> &'static str {
    match udsc {
        UpDownStopContinue::Up => "up",
        UpDownStopContinue::Down => "down",
        UpDownStopContinue::Stop => "stop",
        UpDownStopContinue::Continue => "continue",
    }
}

/// Convert a BreathMarkValue to its MusicXML string representation.
pub(crate) fn breath_mark_value_to_string(bmv: &BreathMarkValue) -> &'static str {
    match bmv {
        BreathMarkValue::Empty => "",
        BreathMarkValue::Comma => "comma",
        BreathMarkValue::Tick => "tick",
        BreathMarkValue::Upbow => "upbow",
        BreathMarkValue::Salzedo => "salzedo",
    }
}

/// Convert a CaesuraValue to its MusicXML string representation.
pub(crate) fn caesura_value_to_string(cv: &CaesuraValue) -> &'static str {
    match cv {
        CaesuraValue::Normal => "normal",
        CaesuraValue::Thick => "thick",
        CaesuraValue::Short => "short",
        CaesuraValue::Curved => "curved",
        CaesuraValue::Single => "single",
    }
}

/// Convert a LineLength to its MusicXML string representation.
pub(crate) fn line_length_to_string(ll: &LineLength) -> &'static str {
    match ll {
        LineLength::Short => "short",
        LineLength::Medium => "medium",
        LineLength::Long => "long",
    }
}

/// Convert a StartStopSingle to its MusicXML string representation.
pub(crate) fn start_stop_single_to_string(sss: &StartStopSingle) -> &'static str {
    match sss {
        StartStopSingle::Start => "start",
        StartStopSingle::Stop => "stop",
        StartStopSingle::Single => "single",
    }
}

/// Convert an OverUnder to its MusicXML string representation.
pub(crate) fn over_under_to_string(ou: &OverUnder) -> &'static str {
    match ou {
        OverUnder::Over => "over",
        OverUnder::Under => "under",
    }
}

/// Convert a LeftCenterRight to its MusicXML string representation.
pub(crate) fn left_center_right_to_string(
    lcr: &crate::ir::common::LeftCenterRight,
) -> &'static str {
    match lcr {
        crate::ir::common::LeftCenterRight::Left => "left",
        crate::ir::common::LeftCenterRight::Center => "center",
        crate::ir::common::LeftCenterRight::Right => "right",
    }
}

/// Convert a Syllabic to its MusicXML string representation.
pub(crate) fn syllabic_to_string(syllabic: &crate::ir::lyric::Syllabic) -> &'static str {
    match syllabic {
        crate::ir::lyric::Syllabic::Single => "single",
        crate::ir::lyric::Syllabic::Begin => "begin",
        crate::ir::lyric::Syllabic::End => "end",
        crate::ir::lyric::Syllabic::Middle => "middle",
    }
}

/// Convert a StartNote to its MusicXML string representation.
pub(crate) fn start_note_to_string(start_note: &crate::ir::notation::StartNote) -> &'static str {
    match start_note {
        crate::ir::notation::StartNote::Upper => "upper",
        crate::ir::notation::StartNote::Main => "main",
        crate::ir::notation::StartNote::Below => "below",
    }
}

/// Convert a TrillStep to its MusicXML string representation.
pub(crate) fn trill_step_to_string(trill_step: &crate::ir::notation::TrillStep) -> &'static str {
    match trill_step {
        crate::ir::notation::TrillStep::Whole => "whole",
        crate::ir::notation::TrillStep::Half => "half",
        crate::ir::notation::TrillStep::Unison => "unison",
    }
}

/// Convert a TwoNoteTurn to its MusicXML string representation.
pub(crate) fn two_note_turn_to_string(
    two_note_turn: &crate::ir::notation::TwoNoteTurn,
) -> &'static str {
    match two_note_turn {
        crate::ir::notation::TwoNoteTurn::Whole => "whole",
        crate::ir::notation::TwoNoteTurn::Half => "half",
        crate::ir::notation::TwoNoteTurn::None => "none",
    }
}

/// Convert a TremoloType to its MusicXML string representation.
pub(crate) fn tremolo_type_to_string(
    tremolo_type: &crate::ir::notation::TremoloType,
) -> &'static str {
    match tremolo_type {
        crate::ir::notation::TremoloType::Start => "start",
        crate::ir::notation::TremoloType::Stop => "stop",
        crate::ir::notation::TremoloType::Single => "single",
        crate::ir::notation::TremoloType::Unmeasured => "unmeasured",
    }
}

/// Convert a HandbellValue to its MusicXML string representation.
pub(crate) fn handbell_value_to_string(value: &crate::ir::notation::HandbellValue) -> &'static str {
    match value {
        crate::ir::notation::HandbellValue::Belltree => "belltree",
        crate::ir::notation::HandbellValue::Damp => "damp",
        crate::ir::notation::HandbellValue::Echo => "echo",
        crate::ir::notation::HandbellValue::Gyro => "gyro",
        crate::ir::notation::HandbellValue::HandMartellato => "hand martellato",
        crate::ir::notation::HandbellValue::MalletLift => "mallet lift",
        crate::ir::notation::HandbellValue::MalletTable => "mallet table",
        crate::ir::notation::HandbellValue::Martellato => "martellato",
        crate::ir::notation::HandbellValue::MartellatoLift => "martellato lift",
        crate::ir::notation::HandbellValue::MutedMartellato => "muted martellato",
        crate::ir::notation::HandbellValue::PluckLift => "pluck lift",
        crate::ir::notation::HandbellValue::Swing => "swing",
    }
}

/// Convert an ArrowDirection to its MusicXML string representation.
pub(crate) fn arrow_direction_to_string(
    direction: &crate::ir::notation::ArrowDirection,
) -> &'static str {
    match direction {
        crate::ir::notation::ArrowDirection::Left => "left",
        crate::ir::notation::ArrowDirection::Up => "up",
        crate::ir::notation::ArrowDirection::Right => "right",
        crate::ir::notation::ArrowDirection::Down => "down",
        crate::ir::notation::ArrowDirection::Northwest => "northwest",
        crate::ir::notation::ArrowDirection::Northeast => "northeast",
        crate::ir::notation::ArrowDirection::Southeast => "southeast",
        crate::ir::notation::ArrowDirection::Southwest => "southwest",
        crate::ir::notation::ArrowDirection::LeftRight => "left right",
        crate::ir::notation::ArrowDirection::UpDown => "up down",
        crate::ir::notation::ArrowDirection::NorthwestSoutheast => "northwest southeast",
        crate::ir::notation::ArrowDirection::NortheastSouthwest => "northeast southwest",
        crate::ir::notation::ArrowDirection::Other => "other",
    }
}

/// Convert an ArrowStyle to its MusicXML string representation.
pub(crate) fn arrow_style_to_string(style: &crate::ir::notation::ArrowStyle) -> &'static str {
    match style {
        crate::ir::notation::ArrowStyle::Single => "single",
        crate::ir::notation::ArrowStyle::Double => "double",
        crate::ir::notation::ArrowStyle::Filled => "filled",
        crate::ir::notation::ArrowStyle::Hollow => "hollow",
        crate::ir::notation::ArrowStyle::Paired => "paired",
        crate::ir::notation::ArrowStyle::Combined => "combined",
        crate::ir::notation::ArrowStyle::Other => "other",
    }
}

/// Convert a HoleClosedValue to its MusicXML string representation.
pub(crate) fn hole_closed_value_to_string(
    value: &crate::ir::notation::HoleClosedValue,
) -> &'static str {
    match value {
        crate::ir::notation::HoleClosedValue::Yes => "yes",
        crate::ir::notation::HoleClosedValue::No => "no",
        crate::ir::notation::HoleClosedValue::Half => "half",
    }
}

/// Convert a HoleClosedLocation to its MusicXML string representation.
pub(crate) fn hole_closed_location_to_string(
    location: &crate::ir::notation::HoleClosedLocation,
) -> &'static str {
    match location {
        crate::ir::notation::HoleClosedLocation::Right => "right",
        crate::ir::notation::HoleClosedLocation::Bottom => "bottom",
        crate::ir::notation::HoleClosedLocation::Left => "left",
        crate::ir::notation::HoleClosedLocation::Top => "top",
    }
}

/// Convert a TapHand to its MusicXML string representation.
pub(crate) fn tap_hand_to_string(hand: &crate::ir::notation::TapHand) -> &'static str {
    match hand {
        crate::ir::notation::TapHand::Left => "left",
        crate::ir::notation::TapHand::Right => "right",
    }
}

/// Convert a TopMiddleBottom to its MusicXML string representation.
pub(crate) fn top_middle_bottom_to_string(
    tmb: &crate::ir::common::TopMiddleBottom,
) -> &'static str {
    match tmb {
        crate::ir::common::TopMiddleBottom::Top => "top",
        crate::ir::common::TopMiddleBottom::Middle => "middle",
        crate::ir::common::TopMiddleBottom::Bottom => "bottom",
    }
}

/// Convert a MarginType to its MusicXML string representation.
pub(crate) fn margin_type_to_string(margin_type: &crate::ir::score::MarginType) -> &'static str {
    match margin_type {
        crate::ir::score::MarginType::Odd => "odd",
        crate::ir::score::MarginType::Even => "even",
        crate::ir::score::MarginType::Both => "both",
    }
}

/// Convert a NoteSizeType to its MusicXML string representation.
pub(crate) fn note_size_type_to_string(
    note_size_type: &crate::ir::score::NoteSizeType,
) -> &'static str {
    match note_size_type {
        crate::ir::score::NoteSizeType::Cue => "cue",
        crate::ir::score::NoteSizeType::Grace => "grace",
        crate::ir::score::NoteSizeType::GraceCue => "grace-cue",
        crate::ir::score::NoteSizeType::Large => "large",
    }
}

/// Convert a FontSize to its MusicXML string representation.
pub(crate) fn font_size_to_string(font_size: &crate::ir::common::FontSize) -> String {
    match font_size {
        crate::ir::common::FontSize::Css(css) => css_font_size_to_string(css).to_string(),
        crate::ir::common::FontSize::Points(pts) => pts.to_string(),
    }
}

/// Convert a CssFontSize to its MusicXML string representation.
pub(crate) fn css_font_size_to_string(css: &crate::ir::common::CssFontSize) -> &'static str {
    match css {
        crate::ir::common::CssFontSize::XxSmall => "xx-small",
        crate::ir::common::CssFontSize::XSmall => "x-small",
        crate::ir::common::CssFontSize::Small => "small",
        crate::ir::common::CssFontSize::Medium => "medium",
        crate::ir::common::CssFontSize::Large => "large",
        crate::ir::common::CssFontSize::XLarge => "x-large",
        crate::ir::common::CssFontSize::XxLarge => "xx-large",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{CssFontSize, FontSize, LeftCenterRight, TopMiddleBottom};
    use crate::ir::lyric::Syllabic;
    use crate::ir::notation::{
        ArrowDirection, ArrowStyle, HandbellValue, HoleClosedLocation, HoleClosedValue, StartNote,
        TapHand, TremoloType, TrillStep, TwoNoteTurn,
    };
    use crate::ir::score::{MarginType, NoteSizeType};

    // ==================== note_type_value_to_string ====================

    #[test]
    fn test_note_type_value_to_string_all_variants() {
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N1024th), "1024th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N512th), "512th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N256th), "256th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N128th), "128th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N64th), "64th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N32nd), "32nd");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::N16th), "16th");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Eighth), "eighth");
        assert_eq!(
            note_type_value_to_string(&NoteTypeValue::Quarter),
            "quarter"
        );
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Half), "half");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Whole), "whole");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Breve), "breve");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Long), "long");
        assert_eq!(note_type_value_to_string(&NoteTypeValue::Maxima), "maxima");
    }

    // ==================== step_to_string ====================

    #[test]
    fn test_step_to_string_all_variants() {
        assert_eq!(step_to_string(&Step::A), "A");
        assert_eq!(step_to_string(&Step::B), "B");
        assert_eq!(step_to_string(&Step::C), "C");
        assert_eq!(step_to_string(&Step::D), "D");
        assert_eq!(step_to_string(&Step::E), "E");
        assert_eq!(step_to_string(&Step::F), "F");
        assert_eq!(step_to_string(&Step::G), "G");
    }

    // ==================== mode_to_string ====================

    #[test]
    fn test_mode_to_string_all_variants() {
        assert_eq!(mode_to_string(&Mode::Major), "major");
        assert_eq!(mode_to_string(&Mode::Minor), "minor");
        assert_eq!(mode_to_string(&Mode::Dorian), "dorian");
        assert_eq!(mode_to_string(&Mode::Phrygian), "phrygian");
        assert_eq!(mode_to_string(&Mode::Lydian), "lydian");
        assert_eq!(mode_to_string(&Mode::Mixolydian), "mixolydian");
        assert_eq!(mode_to_string(&Mode::Aeolian), "aeolian");
        assert_eq!(mode_to_string(&Mode::Locrian), "locrian");
        assert_eq!(mode_to_string(&Mode::Ionian), "ionian");
        assert_eq!(mode_to_string(&Mode::None), "none");
    }

    // ==================== clef_sign_to_string ====================

    #[test]
    fn test_clef_sign_to_string_all_variants() {
        assert_eq!(clef_sign_to_string(&ClefSign::G), "G");
        assert_eq!(clef_sign_to_string(&ClefSign::F), "F");
        assert_eq!(clef_sign_to_string(&ClefSign::C), "C");
        assert_eq!(clef_sign_to_string(&ClefSign::Percussion), "percussion");
        assert_eq!(clef_sign_to_string(&ClefSign::Tab), "TAB");
        assert_eq!(clef_sign_to_string(&ClefSign::Jianpu), "jianpu");
        assert_eq!(clef_sign_to_string(&ClefSign::None), "none");
    }

    // ==================== time_symbol_to_string ====================

    #[test]
    fn test_time_symbol_to_string_all_variants() {
        assert_eq!(time_symbol_to_string(&TimeSymbol::Common), "common");
        assert_eq!(time_symbol_to_string(&TimeSymbol::Cut), "cut");
        assert_eq!(
            time_symbol_to_string(&TimeSymbol::SingleNumber),
            "single-number"
        );
        assert_eq!(time_symbol_to_string(&TimeSymbol::Note), "note");
        assert_eq!(
            time_symbol_to_string(&TimeSymbol::DottedNote),
            "dotted-note"
        );
        assert_eq!(time_symbol_to_string(&TimeSymbol::Normal), "normal");
    }

    // ==================== start_stop_to_string ====================

    #[test]
    fn test_start_stop_to_string_all_variants() {
        assert_eq!(start_stop_to_string(&StartStop::Start), "start");
        assert_eq!(start_stop_to_string(&StartStop::Stop), "stop");
    }

    // ==================== beam_value_to_string ====================

    #[test]
    fn test_beam_value_to_string_all_variants() {
        assert_eq!(beam_value_to_string(&BeamValue::Begin), "begin");
        assert_eq!(beam_value_to_string(&BeamValue::Continue), "continue");
        assert_eq!(beam_value_to_string(&BeamValue::End), "end");
        assert_eq!(
            beam_value_to_string(&BeamValue::ForwardHook),
            "forward hook"
        );
        assert_eq!(
            beam_value_to_string(&BeamValue::BackwardHook),
            "backward hook"
        );
    }

    // ==================== stem_value_to_string ====================

    #[test]
    fn test_stem_value_to_string_all_variants() {
        assert_eq!(stem_value_to_string(&StemValue::Down), "down");
        assert_eq!(stem_value_to_string(&StemValue::Up), "up");
        assert_eq!(stem_value_to_string(&StemValue::Double), "double");
        assert_eq!(stem_value_to_string(&StemValue::None), "none");
    }

    // ==================== notehead_value_to_string ====================

    #[test]
    fn test_notehead_value_to_string_all_variants() {
        assert_eq!(notehead_value_to_string(&NoteheadValue::Slash), "slash");
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::Triangle),
            "triangle"
        );
        assert_eq!(notehead_value_to_string(&NoteheadValue::Diamond), "diamond");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Square), "square");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Cross), "cross");
        assert_eq!(notehead_value_to_string(&NoteheadValue::X), "x");
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::CircleX),
            "circle-x"
        );
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::InvertedTriangle),
            "inverted triangle"
        );
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::ArrowDown),
            "arrow down"
        );
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::ArrowUp),
            "arrow up"
        );
        assert_eq!(notehead_value_to_string(&NoteheadValue::Circled), "circled");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Slashed), "slashed");
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::BackSlashed),
            "back slashed"
        );
        assert_eq!(notehead_value_to_string(&NoteheadValue::Normal), "normal");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Cluster), "cluster");
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::CircleDot),
            "circle dot"
        );
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::LeftTriangle),
            "left triangle"
        );
        assert_eq!(
            notehead_value_to_string(&NoteheadValue::Rectangle),
            "rectangle"
        );
        assert_eq!(notehead_value_to_string(&NoteheadValue::None), "none");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Do), "do");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Re), "re");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Mi), "mi");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Fa), "fa");
        assert_eq!(notehead_value_to_string(&NoteheadValue::FaUp), "fa up");
        assert_eq!(notehead_value_to_string(&NoteheadValue::So), "so");
        assert_eq!(notehead_value_to_string(&NoteheadValue::La), "la");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Ti), "ti");
        assert_eq!(notehead_value_to_string(&NoteheadValue::Other), "other");
    }

    // ==================== accidental_value_to_string ====================

    #[test]
    fn test_accidental_value_to_string_all_variants() {
        assert_eq!(accidental_value_to_string(&AccidentalValue::Sharp), "sharp");
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Natural),
            "natural"
        );
        assert_eq!(accidental_value_to_string(&AccidentalValue::Flat), "flat");
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::DoubleSharp),
            "double-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::SharpSharp),
            "sharp-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::FlatFlat),
            "flat-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::DoubleFlat),
            "double-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::NaturalSharp),
            "natural-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::NaturalFlat),
            "natural-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::QuarterFlat),
            "quarter-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::QuarterSharp),
            "quarter-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::ThreeQuartersFlat),
            "three-quarters-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::ThreeQuartersSharp),
            "three-quarters-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::SharpDown),
            "sharp-down"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::SharpUp),
            "sharp-up"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::NaturalDown),
            "natural-down"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::NaturalUp),
            "natural-up"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::FlatDown),
            "flat-down"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::FlatUp),
            "flat-up"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::TripleSharp),
            "triple-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::TripleFlat),
            "triple-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::SlashQuarterSharp),
            "slash-quarter-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::SlashSharp),
            "slash-sharp"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::SlashFlat),
            "slash-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::DoubleSlashFlat),
            "double-slash-flat"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Sharp1),
            "sharp-1"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Sharp2),
            "sharp-2"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Sharp3),
            "sharp-3"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Sharp5),
            "sharp-5"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Flat1),
            "flat-1"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Flat2),
            "flat-2"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Flat3),
            "flat-3"
        );
        assert_eq!(
            accidental_value_to_string(&AccidentalValue::Flat4),
            "flat-4"
        );
        assert_eq!(accidental_value_to_string(&AccidentalValue::Sori), "sori");
        assert_eq!(accidental_value_to_string(&AccidentalValue::Koron), "koron");
        assert_eq!(accidental_value_to_string(&AccidentalValue::Other), "other");
    }

    // ==================== yes_no_to_string ====================

    #[test]
    fn test_yes_no_to_string_all_variants() {
        assert_eq!(yes_no_to_string(&YesNo::Yes), "yes");
        assert_eq!(yes_no_to_string(&YesNo::No), "no");
    }

    // ==================== bar_style_to_string ====================

    #[test]
    fn test_bar_style_to_string_all_variants() {
        assert_eq!(bar_style_to_string(&BarStyle::Regular), "regular");
        assert_eq!(bar_style_to_string(&BarStyle::Dotted), "dotted");
        assert_eq!(bar_style_to_string(&BarStyle::Dashed), "dashed");
        assert_eq!(bar_style_to_string(&BarStyle::Heavy), "heavy");
        assert_eq!(bar_style_to_string(&BarStyle::LightLight), "light-light");
        assert_eq!(bar_style_to_string(&BarStyle::LightHeavy), "light-heavy");
        assert_eq!(bar_style_to_string(&BarStyle::HeavyLight), "heavy-light");
        assert_eq!(bar_style_to_string(&BarStyle::HeavyHeavy), "heavy-heavy");
        assert_eq!(bar_style_to_string(&BarStyle::Tick), "tick");
        assert_eq!(bar_style_to_string(&BarStyle::Short), "short");
        assert_eq!(bar_style_to_string(&BarStyle::None), "none");
    }

    // ==================== backward_forward_to_string ====================

    #[test]
    fn test_backward_forward_to_string_all_variants() {
        assert_eq!(
            backward_forward_to_string(&BackwardForward::Backward),
            "backward"
        );
        assert_eq!(
            backward_forward_to_string(&BackwardForward::Forward),
            "forward"
        );
    }

    // ==================== right_left_middle_to_string ====================

    #[test]
    fn test_right_left_middle_to_string_all_variants() {
        assert_eq!(
            right_left_middle_to_string(&RightLeftMiddle::Right),
            "right"
        );
        assert_eq!(right_left_middle_to_string(&RightLeftMiddle::Left), "left");
        assert_eq!(
            right_left_middle_to_string(&RightLeftMiddle::Middle),
            "middle"
        );
    }

    // ==================== start_stop_discontinue_to_string ====================

    #[test]
    fn test_start_stop_discontinue_to_string_all_variants() {
        assert_eq!(
            start_stop_discontinue_to_string(&StartStopDiscontinue::Start),
            "start"
        );
        assert_eq!(
            start_stop_discontinue_to_string(&StartStopDiscontinue::Stop),
            "stop"
        );
        assert_eq!(
            start_stop_discontinue_to_string(&StartStopDiscontinue::Discontinue),
            "discontinue"
        );
    }

    // ==================== winged_to_string ====================

    #[test]
    fn test_winged_to_string_all_variants() {
        assert_eq!(winged_to_string(&Winged::None), "none");
        assert_eq!(winged_to_string(&Winged::Straight), "straight");
        assert_eq!(winged_to_string(&Winged::Curved), "curved");
        assert_eq!(winged_to_string(&Winged::DoubleStraight), "double-straight");
        assert_eq!(winged_to_string(&Winged::DoubleCurved), "double-curved");
    }

    // ==================== upright_inverted_to_string ====================

    #[test]
    fn test_upright_inverted_to_string_all_variants() {
        assert_eq!(
            upright_inverted_to_string(&UprightInverted::Upright),
            "upright"
        );
        assert_eq!(
            upright_inverted_to_string(&UprightInverted::Inverted),
            "inverted"
        );
    }

    // ==================== fermata_shape_to_string ====================

    #[test]
    fn test_fermata_shape_to_string_all_variants() {
        assert_eq!(fermata_shape_to_string(&FermataShape::Normal), "normal");
        assert_eq!(fermata_shape_to_string(&FermataShape::Angled), "angled");
        assert_eq!(fermata_shape_to_string(&FermataShape::Square), "square");
        assert_eq!(
            fermata_shape_to_string(&FermataShape::DoubleAngled),
            "double-angled"
        );
        assert_eq!(
            fermata_shape_to_string(&FermataShape::DoubleSquare),
            "double-square"
        );
        assert_eq!(
            fermata_shape_to_string(&FermataShape::DoubleDot),
            "double-dot"
        );
        assert_eq!(
            fermata_shape_to_string(&FermataShape::HalfCurve),
            "half-curve"
        );
        assert_eq!(fermata_shape_to_string(&FermataShape::Curlew), "curlew");
    }

    // ==================== cancel_location_to_string ====================

    #[test]
    fn test_cancel_location_to_string_all_variants() {
        assert_eq!(cancel_location_to_string(&CancelLocation::Left), "left");
        assert_eq!(cancel_location_to_string(&CancelLocation::Right), "right");
        assert_eq!(
            cancel_location_to_string(&CancelLocation::BeforeBarline),
            "before-barline"
        );
    }

    // ==================== fan_to_string ====================

    #[test]
    fn test_fan_to_string_all_variants() {
        assert_eq!(fan_to_string(&Fan::Accel), "accel");
        assert_eq!(fan_to_string(&Fan::Rit), "rit");
        assert_eq!(fan_to_string(&Fan::None), "none");
    }

    // ==================== above_below_to_string ====================

    #[test]
    fn test_above_below_to_string_all_variants() {
        assert_eq!(above_below_to_string(&AboveBelow::Above), "above");
        assert_eq!(above_below_to_string(&AboveBelow::Below), "below");
    }

    // ==================== start_stop_continue_to_string ====================

    #[test]
    fn test_start_stop_continue_to_string_all_variants() {
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

    // ==================== wedge_type_to_string ====================

    #[test]
    fn test_wedge_type_to_string_all_variants() {
        assert_eq!(wedge_type_to_string(&WedgeType::Crescendo), "crescendo");
        assert_eq!(wedge_type_to_string(&WedgeType::Diminuendo), "diminuendo");
        assert_eq!(wedge_type_to_string(&WedgeType::Stop), "stop");
        assert_eq!(wedge_type_to_string(&WedgeType::Continue), "continue");
    }

    // ==================== line_type_to_string ====================

    #[test]
    fn test_line_type_to_string_all_variants() {
        assert_eq!(line_type_to_string(&LineType::Solid), "solid");
        assert_eq!(line_type_to_string(&LineType::Dashed), "dashed");
        assert_eq!(line_type_to_string(&LineType::Dotted), "dotted");
        assert_eq!(line_type_to_string(&LineType::Wavy), "wavy");
    }

    // ==================== up_down_to_string ====================

    #[test]
    fn test_up_down_to_string_all_variants() {
        assert_eq!(up_down_to_string(&UpDown::Up), "up");
        assert_eq!(up_down_to_string(&UpDown::Down), "down");
    }

    // ==================== top_bottom_to_string ====================

    #[test]
    fn test_top_bottom_to_string_all_variants() {
        assert_eq!(top_bottom_to_string(&TopBottom::Top), "top");
        assert_eq!(top_bottom_to_string(&TopBottom::Bottom), "bottom");
    }

    // ==================== show_tuplet_to_string ====================

    #[test]
    fn test_show_tuplet_to_string_all_variants() {
        assert_eq!(show_tuplet_to_string(&ShowTuplet::Actual), "actual");
        assert_eq!(show_tuplet_to_string(&ShowTuplet::Both), "both");
        assert_eq!(show_tuplet_to_string(&ShowTuplet::None), "none");
    }

    // ==================== line_shape_to_string ====================

    #[test]
    fn test_line_shape_to_string_all_variants() {
        assert_eq!(line_shape_to_string(&LineShape::Straight), "straight");
        assert_eq!(line_shape_to_string(&LineShape::Curved), "curved");
    }

    // ==================== pedal_type_to_string ====================

    #[test]
    fn test_pedal_type_to_string_all_variants() {
        assert_eq!(pedal_type_to_string(&PedalType::Start), "start");
        assert_eq!(pedal_type_to_string(&PedalType::Stop), "stop");
        assert_eq!(pedal_type_to_string(&PedalType::Sostenuto), "sostenuto");
        assert_eq!(pedal_type_to_string(&PedalType::Change), "change");
        assert_eq!(pedal_type_to_string(&PedalType::Continue), "continue");
        assert_eq!(pedal_type_to_string(&PedalType::Discontinue), "discontinue");
        assert_eq!(pedal_type_to_string(&PedalType::Resume), "resume");
    }

    // ==================== up_down_stop_continue_to_string ====================

    #[test]
    fn test_up_down_stop_continue_to_string_all_variants() {
        assert_eq!(
            up_down_stop_continue_to_string(&UpDownStopContinue::Up),
            "up"
        );
        assert_eq!(
            up_down_stop_continue_to_string(&UpDownStopContinue::Down),
            "down"
        );
        assert_eq!(
            up_down_stop_continue_to_string(&UpDownStopContinue::Stop),
            "stop"
        );
        assert_eq!(
            up_down_stop_continue_to_string(&UpDownStopContinue::Continue),
            "continue"
        );
    }

    // ==================== breath_mark_value_to_string ====================

    #[test]
    fn test_breath_mark_value_to_string_all_variants() {
        assert_eq!(breath_mark_value_to_string(&BreathMarkValue::Empty), "");
        assert_eq!(
            breath_mark_value_to_string(&BreathMarkValue::Comma),
            "comma"
        );
        assert_eq!(breath_mark_value_to_string(&BreathMarkValue::Tick), "tick");
        assert_eq!(
            breath_mark_value_to_string(&BreathMarkValue::Upbow),
            "upbow"
        );
        assert_eq!(
            breath_mark_value_to_string(&BreathMarkValue::Salzedo),
            "salzedo"
        );
    }

    // ==================== caesura_value_to_string ====================

    #[test]
    fn test_caesura_value_to_string_all_variants() {
        assert_eq!(caesura_value_to_string(&CaesuraValue::Normal), "normal");
        assert_eq!(caesura_value_to_string(&CaesuraValue::Thick), "thick");
        assert_eq!(caesura_value_to_string(&CaesuraValue::Short), "short");
        assert_eq!(caesura_value_to_string(&CaesuraValue::Curved), "curved");
        assert_eq!(caesura_value_to_string(&CaesuraValue::Single), "single");
    }

    // ==================== line_length_to_string ====================

    #[test]
    fn test_line_length_to_string_all_variants() {
        assert_eq!(line_length_to_string(&LineLength::Short), "short");
        assert_eq!(line_length_to_string(&LineLength::Medium), "medium");
        assert_eq!(line_length_to_string(&LineLength::Long), "long");
    }

    // ==================== start_stop_single_to_string ====================

    #[test]
    fn test_start_stop_single_to_string_all_variants() {
        assert_eq!(
            start_stop_single_to_string(&StartStopSingle::Start),
            "start"
        );
        assert_eq!(start_stop_single_to_string(&StartStopSingle::Stop), "stop");
        assert_eq!(
            start_stop_single_to_string(&StartStopSingle::Single),
            "single"
        );
    }

    // ==================== over_under_to_string ====================

    #[test]
    fn test_over_under_to_string_all_variants() {
        assert_eq!(over_under_to_string(&OverUnder::Over), "over");
        assert_eq!(over_under_to_string(&OverUnder::Under), "under");
    }

    // ==================== left_center_right_to_string ====================

    #[test]
    fn test_left_center_right_to_string_all_variants() {
        assert_eq!(left_center_right_to_string(&LeftCenterRight::Left), "left");
        assert_eq!(
            left_center_right_to_string(&LeftCenterRight::Center),
            "center"
        );
        assert_eq!(
            left_center_right_to_string(&LeftCenterRight::Right),
            "right"
        );
    }

    // ==================== syllabic_to_string ====================

    #[test]
    fn test_syllabic_to_string_all_variants() {
        assert_eq!(syllabic_to_string(&Syllabic::Single), "single");
        assert_eq!(syllabic_to_string(&Syllabic::Begin), "begin");
        assert_eq!(syllabic_to_string(&Syllabic::End), "end");
        assert_eq!(syllabic_to_string(&Syllabic::Middle), "middle");
    }

    // ==================== start_note_to_string ====================

    #[test]
    fn test_start_note_to_string_all_variants() {
        assert_eq!(start_note_to_string(&StartNote::Upper), "upper");
        assert_eq!(start_note_to_string(&StartNote::Main), "main");
        assert_eq!(start_note_to_string(&StartNote::Below), "below");
    }

    // ==================== trill_step_to_string ====================

    #[test]
    fn test_trill_step_to_string_all_variants() {
        assert_eq!(trill_step_to_string(&TrillStep::Whole), "whole");
        assert_eq!(trill_step_to_string(&TrillStep::Half), "half");
        assert_eq!(trill_step_to_string(&TrillStep::Unison), "unison");
    }

    // ==================== two_note_turn_to_string ====================

    #[test]
    fn test_two_note_turn_to_string_all_variants() {
        assert_eq!(two_note_turn_to_string(&TwoNoteTurn::Whole), "whole");
        assert_eq!(two_note_turn_to_string(&TwoNoteTurn::Half), "half");
        assert_eq!(two_note_turn_to_string(&TwoNoteTurn::None), "none");
    }

    // ==================== tremolo_type_to_string ====================

    #[test]
    fn test_tremolo_type_to_string_all_variants() {
        assert_eq!(tremolo_type_to_string(&TremoloType::Start), "start");
        assert_eq!(tremolo_type_to_string(&TremoloType::Stop), "stop");
        assert_eq!(tremolo_type_to_string(&TremoloType::Single), "single");
        assert_eq!(
            tremolo_type_to_string(&TremoloType::Unmeasured),
            "unmeasured"
        );
    }

    // ==================== handbell_value_to_string ====================

    #[test]
    fn test_handbell_value_to_string_all_variants() {
        assert_eq!(
            handbell_value_to_string(&HandbellValue::Belltree),
            "belltree"
        );
        assert_eq!(handbell_value_to_string(&HandbellValue::Damp), "damp");
        assert_eq!(handbell_value_to_string(&HandbellValue::Echo), "echo");
        assert_eq!(handbell_value_to_string(&HandbellValue::Gyro), "gyro");
        assert_eq!(
            handbell_value_to_string(&HandbellValue::HandMartellato),
            "hand martellato"
        );
        assert_eq!(
            handbell_value_to_string(&HandbellValue::MalletLift),
            "mallet lift"
        );
        assert_eq!(
            handbell_value_to_string(&HandbellValue::MalletTable),
            "mallet table"
        );
        assert_eq!(
            handbell_value_to_string(&HandbellValue::Martellato),
            "martellato"
        );
        assert_eq!(
            handbell_value_to_string(&HandbellValue::MartellatoLift),
            "martellato lift"
        );
        assert_eq!(
            handbell_value_to_string(&HandbellValue::MutedMartellato),
            "muted martellato"
        );
        assert_eq!(
            handbell_value_to_string(&HandbellValue::PluckLift),
            "pluck lift"
        );
        assert_eq!(handbell_value_to_string(&HandbellValue::Swing), "swing");
    }

    // ==================== arrow_direction_to_string ====================

    #[test]
    fn test_arrow_direction_to_string_all_variants() {
        assert_eq!(arrow_direction_to_string(&ArrowDirection::Left), "left");
        assert_eq!(arrow_direction_to_string(&ArrowDirection::Up), "up");
        assert_eq!(arrow_direction_to_string(&ArrowDirection::Right), "right");
        assert_eq!(arrow_direction_to_string(&ArrowDirection::Down), "down");
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::Northwest),
            "northwest"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::Northeast),
            "northeast"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::Southeast),
            "southeast"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::Southwest),
            "southwest"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::LeftRight),
            "left right"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::UpDown),
            "up down"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::NorthwestSoutheast),
            "northwest southeast"
        );
        assert_eq!(
            arrow_direction_to_string(&ArrowDirection::NortheastSouthwest),
            "northeast southwest"
        );
        assert_eq!(arrow_direction_to_string(&ArrowDirection::Other), "other");
    }

    // ==================== arrow_style_to_string ====================

    #[test]
    fn test_arrow_style_to_string_all_variants() {
        assert_eq!(arrow_style_to_string(&ArrowStyle::Single), "single");
        assert_eq!(arrow_style_to_string(&ArrowStyle::Double), "double");
        assert_eq!(arrow_style_to_string(&ArrowStyle::Filled), "filled");
        assert_eq!(arrow_style_to_string(&ArrowStyle::Hollow), "hollow");
        assert_eq!(arrow_style_to_string(&ArrowStyle::Paired), "paired");
        assert_eq!(arrow_style_to_string(&ArrowStyle::Combined), "combined");
        assert_eq!(arrow_style_to_string(&ArrowStyle::Other), "other");
    }

    // ==================== hole_closed_value_to_string ====================

    #[test]
    fn test_hole_closed_value_to_string_all_variants() {
        assert_eq!(hole_closed_value_to_string(&HoleClosedValue::Yes), "yes");
        assert_eq!(hole_closed_value_to_string(&HoleClosedValue::No), "no");
        assert_eq!(hole_closed_value_to_string(&HoleClosedValue::Half), "half");
    }

    // ==================== hole_closed_location_to_string ====================

    #[test]
    fn test_hole_closed_location_to_string_all_variants() {
        assert_eq!(
            hole_closed_location_to_string(&HoleClosedLocation::Right),
            "right"
        );
        assert_eq!(
            hole_closed_location_to_string(&HoleClosedLocation::Bottom),
            "bottom"
        );
        assert_eq!(
            hole_closed_location_to_string(&HoleClosedLocation::Left),
            "left"
        );
        assert_eq!(
            hole_closed_location_to_string(&HoleClosedLocation::Top),
            "top"
        );
    }

    // ==================== tap_hand_to_string ====================

    #[test]
    fn test_tap_hand_to_string_all_variants() {
        assert_eq!(tap_hand_to_string(&TapHand::Left), "left");
        assert_eq!(tap_hand_to_string(&TapHand::Right), "right");
    }

    // ==================== top_middle_bottom_to_string ====================

    #[test]
    fn test_top_middle_bottom_to_string_all_variants() {
        assert_eq!(top_middle_bottom_to_string(&TopMiddleBottom::Top), "top");
        assert_eq!(
            top_middle_bottom_to_string(&TopMiddleBottom::Middle),
            "middle"
        );
        assert_eq!(
            top_middle_bottom_to_string(&TopMiddleBottom::Bottom),
            "bottom"
        );
    }

    // ==================== margin_type_to_string ====================

    #[test]
    fn test_margin_type_to_string_all_variants() {
        assert_eq!(margin_type_to_string(&MarginType::Odd), "odd");
        assert_eq!(margin_type_to_string(&MarginType::Even), "even");
        assert_eq!(margin_type_to_string(&MarginType::Both), "both");
    }

    // ==================== note_size_type_to_string ====================

    #[test]
    fn test_note_size_type_to_string_all_variants() {
        assert_eq!(note_size_type_to_string(&NoteSizeType::Cue), "cue");
        assert_eq!(note_size_type_to_string(&NoteSizeType::Grace), "grace");
        assert_eq!(
            note_size_type_to_string(&NoteSizeType::GraceCue),
            "grace-cue"
        );
        assert_eq!(note_size_type_to_string(&NoteSizeType::Large), "large");
    }

    // ==================== font_size_to_string ====================

    #[test]
    fn test_font_size_to_string_css_variants() {
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::XxSmall)),
            "xx-small"
        );
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::XSmall)),
            "x-small"
        );
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::Small)),
            "small"
        );
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::Medium)),
            "medium"
        );
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::Large)),
            "large"
        );
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::XLarge)),
            "x-large"
        );
        assert_eq!(
            font_size_to_string(&FontSize::Css(CssFontSize::XxLarge)),
            "xx-large"
        );
    }

    #[test]
    fn test_font_size_to_string_points_variants() {
        assert_eq!(font_size_to_string(&FontSize::Points(12.0)), "12");
        assert_eq!(font_size_to_string(&FontSize::Points(10.5)), "10.5");
        assert_eq!(font_size_to_string(&FontSize::Points(24.0)), "24");
        assert_eq!(font_size_to_string(&FontSize::Points(8.25)), "8.25");
    }

    // ==================== css_font_size_to_string ====================

    #[test]
    fn test_css_font_size_to_string_all_variants() {
        assert_eq!(css_font_size_to_string(&CssFontSize::XxSmall), "xx-small");
        assert_eq!(css_font_size_to_string(&CssFontSize::XSmall), "x-small");
        assert_eq!(css_font_size_to_string(&CssFontSize::Small), "small");
        assert_eq!(css_font_size_to_string(&CssFontSize::Medium), "medium");
        assert_eq!(css_font_size_to_string(&CssFontSize::Large), "large");
        assert_eq!(css_font_size_to_string(&CssFontSize::XLarge), "x-large");
        assert_eq!(css_font_size_to_string(&CssFontSize::XxLarge), "xx-large");
    }
}
