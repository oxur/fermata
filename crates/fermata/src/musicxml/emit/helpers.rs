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

#[cfg(test)]
mod tests {
    use super::*;

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
