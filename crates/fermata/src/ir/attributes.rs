//! Measure attributes: time, key, clef, barline.

use super::common::{
    AccidentalValue, BackwardForward, Color, Editorial, Octave, Position, PositiveDivisions,
    RightLeftMiddle, Semitones, StaffNumber, StartStop, StartStopDiscontinue, SymbolSize, Tenths,
    WavyLine, YesNo,
};
use super::direction::{Coda, Segno};
use super::notation::Fermata;

/// Attributes element containing key, time, clef, etc.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Attributes {
    /// Editorial information
    pub editorial: Editorial,
    /// Divisions per quarter note
    pub divisions: Option<PositiveDivisions>,
    /// Key signatures
    pub keys: Vec<Key>,
    /// Time signatures
    pub times: Vec<Time>,
    /// Number of staves
    pub staves: Option<u32>,
    /// Part symbol for grouping
    pub part_symbol: Option<PartSymbol>,
    /// Number of instruments
    pub instruments: Option<u32>,
    /// Clefs
    pub clefs: Vec<Clef>,
    /// Staff details
    pub staff_details: Vec<StaffDetails>,
    /// Transposition
    pub transpose: Vec<Transpose>,
    /// Measure styles
    pub measure_styles: Vec<MeasureStyle>,
}

/// Key signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    /// Key content (traditional or non-traditional)
    pub content: KeyContent,
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Whether to print
    pub print_object: Option<YesNo>,
}

/// Key content - traditional (fifths) or non-traditional (explicit steps).
#[derive(Debug, Clone, PartialEq)]
pub enum KeyContent {
    /// Traditional key signature based on circle of fifths
    Traditional(TraditionalKey),
    /// Non-traditional key with explicit steps
    NonTraditional(Vec<KeyStep>),
}

/// Traditional key signature.
#[derive(Debug, Clone, PartialEq)]
pub struct TraditionalKey {
    /// Cancel previous key
    pub cancel: Option<Cancel>,
    /// Number of fifths (-7 to 7)
    pub fifths: i8,
    /// Key mode
    pub mode: Option<Mode>,
}

/// Key step for non-traditional keys.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyStep {
    /// The pitch step
    pub step: super::pitch::Step,
    /// Alteration in semitones
    pub alter: Semitones,
    /// Accidental to display
    pub accidental: Option<AccidentalValue>,
}

/// Cancel previous key signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Cancel {
    /// Number of fifths to cancel
    pub fifths: i8,
    /// Cancel location
    pub location: Option<CancelLocation>,
}

/// Cancel location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelLocation {
    /// Left of new key
    Left,
    /// Right of new key
    Right,
    /// Before barline
    BeforeBarline,
}

/// Mode for key signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Major mode
    Major,
    /// Minor mode
    Minor,
    /// Dorian mode
    Dorian,
    /// Phrygian mode
    Phrygian,
    /// Lydian mode
    Lydian,
    /// Mixolydian mode
    Mixolydian,
    /// Aeolian mode
    Aeolian,
    /// Locrian mode
    Locrian,
    /// Ionian mode
    Ionian,
    /// No mode
    None,
}

/// Time signature.
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    /// Time signature content
    pub content: TimeContent,
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Time symbol
    pub symbol: Option<TimeSymbol>,
    /// Whether to print
    pub print_object: Option<YesNo>,
}

/// Time signature content.
#[derive(Debug, Clone, PartialEq)]
pub enum TimeContent {
    /// Measured time with beat signatures
    Measured {
        /// Time signatures
        signatures: Vec<TimeSignature>,
    },
    /// Senza misura (free time)
    SenzaMisura(String),
}

/// A single time signature (beats / beat-type).
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSignature {
    /// Beats (numerator) - can be compound like "3+2"
    pub beats: String,
    /// Beat type (denominator)
    pub beat_type: String,
}

/// Time signature symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSymbol {
    /// Common time (C)
    Common,
    /// Cut time (C with line)
    Cut,
    /// Single number
    SingleNumber,
    /// Note symbol
    Note,
    /// Dotted note symbol
    DottedNote,
    /// Normal numeric display
    Normal,
}

/// Clef.
#[derive(Debug, Clone, PartialEq)]
pub struct Clef {
    /// Clef sign
    pub sign: ClefSign,
    /// Staff line (1 = bottom)
    pub line: Option<u8>,
    /// Octave change (-2 to 2)
    pub octave_change: Option<i8>,
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Symbol size
    pub size: Option<SymbolSize>,
    /// Whether to print
    pub print_object: Option<YesNo>,
}

/// Clef sign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClefSign {
    /// G clef (treble)
    G,
    /// F clef (bass)
    F,
    /// C clef (alto/tenor)
    C,
    /// Percussion clef
    Percussion,
    /// Tab clef
    Tab,
    /// Jianpu clef
    Jianpu,
    /// No clef
    None,
}

/// Part symbol for grouping.
#[derive(Debug, Clone, PartialEq)]
pub struct PartSymbol {
    /// Symbol value
    pub value: GroupSymbolValue,
    /// Top staff
    pub top_staff: Option<StaffNumber>,
    /// Bottom staff
    pub bottom_staff: Option<StaffNumber>,
    /// Position attributes
    pub position: Position,
    /// Color
    pub color: Option<Color>,
}

/// Group symbol values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupSymbolValue {
    /// No symbol
    None,
    /// Brace
    Brace,
    /// Line
    Line,
    /// Bracket
    Bracket,
    /// Square bracket
    Square,
}

/// Staff details.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StaffDetails {
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Staff type
    pub staff_type: Option<StaffType>,
    /// Number of staff lines
    pub staff_lines: Option<u8>,
    /// Staff tuning for tablature
    pub staff_tuning: Vec<StaffTuning>,
    /// Capo position
    pub capo: Option<u8>,
    /// Staff size percentage
    pub staff_size: Option<f64>,
}

/// Staff types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffType {
    /// Ossia staff
    Ossia,
    /// Editorial staff
    Editorial,
    /// Cue staff
    Cue,
    /// Regular staff
    Regular,
    /// Alternate staff
    Alternate,
}

/// Staff tuning for tablature.
#[derive(Debug, Clone, PartialEq)]
pub struct StaffTuning {
    /// Line number (1 = bottom)
    pub line: u8,
    /// Tuning step
    pub tuning_step: super::pitch::Step,
    /// Tuning alteration
    pub tuning_alter: Option<Semitones>,
    /// Tuning octave
    pub tuning_octave: Octave,
}

/// Transposition.
#[derive(Debug, Clone, PartialEq)]
pub struct Transpose {
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Diatonic steps
    pub diatonic: Option<i32>,
    /// Chromatic steps
    pub chromatic: i32,
    /// Octave change
    pub octave_change: Option<i32>,
    /// Double transposition
    pub double: Option<YesNo>,
}

/// Measure style (multimeasure rests, slashes, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureStyle {
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Measure style content
    pub content: MeasureStyleContent,
}

/// Measure style content.
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureStyleContent {
    /// Multiple measure rest
    MultipleRest {
        /// Number of measures
        count: u32,
        /// Whether to use symbols
        use_symbols: Option<YesNo>,
    },
    /// Measure repeat
    MeasureRepeat {
        /// Start or stop
        r#type: StartStop,
        /// Number of slashes
        slashes: Option<u32>,
    },
    /// Beat repeat
    BeatRepeat {
        /// Start or stop
        r#type: StartStop,
        /// Number of slashes
        slashes: Option<u32>,
    },
    /// Slash notation
    Slash {
        /// Start or stop
        r#type: StartStop,
        /// Whether to use stems
        use_stems: Option<YesNo>,
    },
}

// === Barlines ===

/// Barline.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Barline {
    /// Location (right, left, or middle)
    pub location: Option<RightLeftMiddle>,
    /// Bar style
    pub bar_style: Option<BarStyle>,
    /// Editorial information
    pub editorial: Editorial,
    /// Wavy line
    pub wavy_line: Option<WavyLine>,
    /// Segno sign
    pub segno: Option<Segno>,
    /// Coda sign
    pub coda: Option<Coda>,
    /// Fermatas
    pub fermatas: Vec<Fermata>,
    /// Ending (volta)
    pub ending: Option<Ending>,
    /// Repeat
    pub repeat: Option<Repeat>,
}

/// Bar style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarStyle {
    /// Regular barline
    Regular,
    /// Dotted barline
    Dotted,
    /// Dashed barline
    Dashed,
    /// Heavy barline
    Heavy,
    /// Light-light double barline
    LightLight,
    /// Light-heavy double barline (final)
    LightHeavy,
    /// Heavy-light double barline
    HeavyLight,
    /// Heavy-heavy double barline
    HeavyHeavy,
    /// Tick barline
    Tick,
    /// Short barline
    Short,
    /// No barline
    None,
}

/// Repeat barline.
#[derive(Debug, Clone, PartialEq)]
pub struct Repeat {
    /// Backward or forward
    pub direction: BackwardForward,
    /// Times to repeat
    pub times: Option<u32>,
    /// Winged repeat
    pub winged: Option<Winged>,
}

/// Winged repeat types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winged {
    /// No wings
    None,
    /// Straight wings
    Straight,
    /// Curved wings
    Curved,
    /// Double straight wings
    DoubleStraight,
    /// Double curved wings
    DoubleCurved,
}

/// Ending (volta).
#[derive(Debug, Clone, PartialEq)]
pub struct Ending {
    /// Start, stop, or discontinue
    pub r#type: StartStopDiscontinue,
    /// Ending number(s)
    pub number: String,
    /// Text to display
    pub text: Option<String>,
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// End length in tenths
    pub end_length: Option<Tenths>,
    /// Text X position
    pub text_x: Option<Tenths>,
    /// Text Y position
    pub text_y: Option<Tenths>,
}

// WavyLine is defined in common.rs and imported at the top of this file

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::pitch::Step;

    // === Attributes Tests ===

    #[test]
    fn test_attributes_default() {
        let attrs = Attributes::default();
        assert!(attrs.divisions.is_none());
        assert!(attrs.keys.is_empty());
        assert!(attrs.times.is_empty());
        assert!(attrs.staves.is_none());
        assert!(attrs.clefs.is_empty());
    }

    #[test]
    fn test_attributes_with_divisions() {
        let attrs = Attributes {
            divisions: Some(4),
            ..Default::default()
        };
        assert_eq!(attrs.divisions, Some(4));
    }

    #[test]
    fn test_attributes_clone() {
        let attrs = Attributes {
            divisions: Some(8),
            staves: Some(2),
            ..Default::default()
        };
        let cloned = attrs.clone();
        assert_eq!(attrs, cloned);
    }

    // === Key Tests ===

    #[test]
    fn test_key_c_major() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };
        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, 0);
            assert_eq!(tk.mode, Some(Mode::Major));
        }
    }

    #[test]
    fn test_key_g_major() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 1,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };
        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, 1);
        }
    }

    #[test]
    fn test_key_f_major() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: -1,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };
        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.fifths, -1);
        }
    }

    #[test]
    fn test_key_a_minor() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Minor),
            }),
            number: None,
            print_object: None,
        };
        if let KeyContent::Traditional(tk) = &key.content {
            assert_eq!(tk.mode, Some(Mode::Minor));
        }
    }

    #[test]
    fn test_key_with_cancel() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: Some(Cancel {
                    fifths: -2,
                    location: Some(CancelLocation::Left),
                }),
                fifths: 3,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };
        if let KeyContent::Traditional(tk) = &key.content {
            assert!(tk.cancel.is_some());
            assert_eq!(tk.cancel.as_ref().unwrap().fifths, -2);
        }
    }

    #[test]
    fn test_key_non_traditional() {
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
            ]),
            number: None,
            print_object: None,
        };
        if let KeyContent::NonTraditional(steps) = &key.content {
            assert_eq!(steps.len(), 2);
        }
    }

    // === Mode Tests ===

    #[test]
    fn test_mode_all_variants() {
        assert_eq!(Mode::Major, Mode::Major);
        assert_eq!(Mode::Minor, Mode::Minor);
        assert_eq!(Mode::Dorian, Mode::Dorian);
        assert_eq!(Mode::Phrygian, Mode::Phrygian);
        assert_eq!(Mode::Lydian, Mode::Lydian);
        assert_eq!(Mode::Mixolydian, Mode::Mixolydian);
        assert_eq!(Mode::Aeolian, Mode::Aeolian);
        assert_eq!(Mode::Locrian, Mode::Locrian);
        assert_eq!(Mode::Ionian, Mode::Ionian);
        assert_eq!(Mode::None, Mode::None);
    }

    // === CancelLocation Tests ===

    #[test]
    fn test_cancellocation_all_variants() {
        assert_eq!(CancelLocation::Left, CancelLocation::Left);
        assert_eq!(CancelLocation::Right, CancelLocation::Right);
        assert_eq!(CancelLocation::BeforeBarline, CancelLocation::BeforeBarline);
    }

    // === Time Tests ===

    #[test]
    fn test_time_4_4() {
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
        if let TimeContent::Measured { signatures } = &time.content {
            assert_eq!(signatures.len(), 1);
            assert_eq!(signatures[0].beats, "4");
            assert_eq!(signatures[0].beat_type, "4");
        }
    }

    #[test]
    fn test_time_3_4() {
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
        if let TimeContent::Measured { signatures } = &time.content {
            assert_eq!(signatures[0].beats, "3");
        }
    }

    #[test]
    fn test_time_common_symbol() {
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
        assert_eq!(time.symbol, Some(TimeSymbol::Common));
    }

    #[test]
    fn test_time_cut_symbol() {
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
        assert_eq!(time.symbol, Some(TimeSymbol::Cut));
    }

    #[test]
    fn test_time_compound() {
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
        if let TimeContent::Measured { signatures } = &time.content {
            assert_eq!(signatures[0].beats, "3+2");
        }
    }

    #[test]
    fn test_time_senza_misura() {
        let time = Time {
            content: TimeContent::SenzaMisura("".to_string()),
            number: None,
            symbol: None,
            print_object: None,
        };
        if let TimeContent::SenzaMisura(text) = &time.content {
            assert_eq!(text, "");
        }
    }

    // === TimeSymbol Tests ===

    #[test]
    fn test_timesymbol_all_variants() {
        assert_eq!(TimeSymbol::Common, TimeSymbol::Common);
        assert_eq!(TimeSymbol::Cut, TimeSymbol::Cut);
        assert_eq!(TimeSymbol::SingleNumber, TimeSymbol::SingleNumber);
        assert_eq!(TimeSymbol::Note, TimeSymbol::Note);
        assert_eq!(TimeSymbol::DottedNote, TimeSymbol::DottedNote);
        assert_eq!(TimeSymbol::Normal, TimeSymbol::Normal);
    }

    // === Clef Tests ===

    #[test]
    fn test_clef_treble() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.sign, ClefSign::G);
        assert_eq!(clef.line, Some(2));
    }

    #[test]
    fn test_clef_bass() {
        let clef = Clef {
            sign: ClefSign::F,
            line: Some(4),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.sign, ClefSign::F);
        assert_eq!(clef.line, Some(4));
    }

    #[test]
    fn test_clef_alto() {
        let clef = Clef {
            sign: ClefSign::C,
            line: Some(3),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.sign, ClefSign::C);
        assert_eq!(clef.line, Some(3));
    }

    #[test]
    fn test_clef_tenor() {
        let clef = Clef {
            sign: ClefSign::C,
            line: Some(4),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.sign, ClefSign::C);
        assert_eq!(clef.line, Some(4));
    }

    #[test]
    fn test_clef_percussion() {
        let clef = Clef {
            sign: ClefSign::Percussion,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.sign, ClefSign::Percussion);
    }

    #[test]
    fn test_clef_with_octave_change() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(-1),
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.octave_change, Some(-1));
    }

    #[test]
    fn test_clef_8va() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(1),
            number: None,
            size: None,
            print_object: None,
        };
        assert_eq!(clef.octave_change, Some(1));
    }

    // === ClefSign Tests ===

    #[test]
    fn test_clefsign_all_variants() {
        assert_eq!(ClefSign::G, ClefSign::G);
        assert_eq!(ClefSign::F, ClefSign::F);
        assert_eq!(ClefSign::C, ClefSign::C);
        assert_eq!(ClefSign::Percussion, ClefSign::Percussion);
        assert_eq!(ClefSign::Tab, ClefSign::Tab);
        assert_eq!(ClefSign::Jianpu, ClefSign::Jianpu);
        assert_eq!(ClefSign::None, ClefSign::None);
    }

    // === PartSymbol Tests ===

    #[test]
    fn test_partsymbol_brace() {
        let symbol = PartSymbol {
            value: GroupSymbolValue::Brace,
            top_staff: Some(1),
            bottom_staff: Some(2),
            position: Position::default(),
            color: None,
        };
        assert_eq!(symbol.value, GroupSymbolValue::Brace);
    }

    // === GroupSymbolValue Tests ===

    #[test]
    fn test_groupsymbolvalue_all_variants() {
        assert_eq!(GroupSymbolValue::None, GroupSymbolValue::None);
        assert_eq!(GroupSymbolValue::Brace, GroupSymbolValue::Brace);
        assert_eq!(GroupSymbolValue::Line, GroupSymbolValue::Line);
        assert_eq!(GroupSymbolValue::Bracket, GroupSymbolValue::Bracket);
        assert_eq!(GroupSymbolValue::Square, GroupSymbolValue::Square);
    }

    // === StaffDetails Tests ===

    #[test]
    fn test_staffdetails_default() {
        let details = StaffDetails::default();
        assert!(details.number.is_none());
        assert!(details.staff_type.is_none());
        assert!(details.staff_lines.is_none());
    }

    #[test]
    fn test_staffdetails_with_lines() {
        let details = StaffDetails {
            staff_lines: Some(5),
            ..Default::default()
        };
        assert_eq!(details.staff_lines, Some(5));
    }

    #[test]
    fn test_staffdetails_tab() {
        let details = StaffDetails {
            staff_type: Some(StaffType::Regular),
            staff_lines: Some(6),
            ..Default::default()
        };
        assert_eq!(details.staff_lines, Some(6));
    }

    // === StaffType Tests ===

    #[test]
    fn test_stafftype_all_variants() {
        assert_eq!(StaffType::Ossia, StaffType::Ossia);
        assert_eq!(StaffType::Editorial, StaffType::Editorial);
        assert_eq!(StaffType::Cue, StaffType::Cue);
        assert_eq!(StaffType::Regular, StaffType::Regular);
        assert_eq!(StaffType::Alternate, StaffType::Alternate);
    }

    // === StaffTuning Tests ===

    #[test]
    fn test_stafftuning_guitar_e() {
        let tuning = StaffTuning {
            line: 1,
            tuning_step: Step::E,
            tuning_alter: None,
            tuning_octave: 2,
        };
        assert_eq!(tuning.line, 1);
        assert_eq!(tuning.tuning_step, Step::E);
        assert_eq!(tuning.tuning_octave, 2);
    }

    // === Transpose Tests ===

    #[test]
    fn test_transpose_clarinet() {
        let transpose = Transpose {
            number: None,
            diatonic: Some(-1),
            chromatic: -2,
            octave_change: None,
            double: None,
        };
        assert_eq!(transpose.chromatic, -2);
        assert_eq!(transpose.diatonic, Some(-1));
    }

    // === MeasureStyle Tests ===

    #[test]
    fn test_measurestyle_multiple_rest() {
        let style = MeasureStyle {
            number: None,
            content: MeasureStyleContent::MultipleRest {
                count: 4,
                use_symbols: Some(YesNo::Yes),
            },
        };
        if let MeasureStyleContent::MultipleRest { count, .. } = style.content {
            assert_eq!(count, 4);
        }
    }

    #[test]
    fn test_measurestyle_measure_repeat() {
        let style = MeasureStyle {
            number: None,
            content: MeasureStyleContent::MeasureRepeat {
                r#type: StartStop::Start,
                slashes: Some(1),
            },
        };
        if let MeasureStyleContent::MeasureRepeat { slashes, .. } = style.content {
            assert_eq!(slashes, Some(1));
        }
    }

    #[test]
    fn test_measurestyle_slash() {
        let style = MeasureStyle {
            number: None,
            content: MeasureStyleContent::Slash {
                r#type: StartStop::Start,
                use_stems: Some(YesNo::No),
            },
        };
        if let MeasureStyleContent::Slash { use_stems, .. } = style.content {
            assert_eq!(use_stems, Some(YesNo::No));
        }
    }

    // === Barline Tests ===

    #[test]
    fn test_barline_default() {
        let barline = Barline::default();
        assert!(barline.location.is_none());
        assert!(barline.bar_style.is_none());
    }

    #[test]
    fn test_barline_right_regular() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::Regular),
            ..Default::default()
        };
        assert_eq!(barline.location, Some(RightLeftMiddle::Right));
        assert_eq!(barline.bar_style, Some(BarStyle::Regular));
    }

    #[test]
    fn test_barline_double() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightLight),
            ..Default::default()
        };
        assert_eq!(barline.bar_style, Some(BarStyle::LightLight));
    }

    #[test]
    fn test_barline_final() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            ..Default::default()
        };
        assert_eq!(barline.bar_style, Some(BarStyle::LightHeavy));
    }

    // === BarStyle Tests ===

    #[test]
    fn test_barstyle_all_variants() {
        assert_eq!(BarStyle::Regular, BarStyle::Regular);
        assert_eq!(BarStyle::Dotted, BarStyle::Dotted);
        assert_eq!(BarStyle::Dashed, BarStyle::Dashed);
        assert_eq!(BarStyle::Heavy, BarStyle::Heavy);
        assert_eq!(BarStyle::LightLight, BarStyle::LightLight);
        assert_eq!(BarStyle::LightHeavy, BarStyle::LightHeavy);
        assert_eq!(BarStyle::HeavyLight, BarStyle::HeavyLight);
        assert_eq!(BarStyle::HeavyHeavy, BarStyle::HeavyHeavy);
        assert_eq!(BarStyle::Tick, BarStyle::Tick);
        assert_eq!(BarStyle::Short, BarStyle::Short);
        assert_eq!(BarStyle::None, BarStyle::None);
    }

    // === Repeat Tests ===

    #[test]
    fn test_repeat_backward() {
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: Some(2),
            winged: None,
        };
        assert_eq!(repeat.direction, BackwardForward::Backward);
        assert_eq!(repeat.times, Some(2));
    }

    #[test]
    fn test_repeat_forward() {
        let repeat = Repeat {
            direction: BackwardForward::Forward,
            times: None,
            winged: Some(Winged::Curved),
        };
        assert_eq!(repeat.direction, BackwardForward::Forward);
        assert_eq!(repeat.winged, Some(Winged::Curved));
    }

    // === Winged Tests ===

    #[test]
    fn test_winged_all_variants() {
        assert_eq!(Winged::None, Winged::None);
        assert_eq!(Winged::Straight, Winged::Straight);
        assert_eq!(Winged::Curved, Winged::Curved);
        assert_eq!(Winged::DoubleStraight, Winged::DoubleStraight);
        assert_eq!(Winged::DoubleCurved, Winged::DoubleCurved);
    }

    // === Ending Tests ===

    #[test]
    fn test_ending_first() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: Some("1.".to_string()),
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };
        assert_eq!(ending.number, "1");
        assert_eq!(ending.text, Some("1.".to_string()));
    }

    #[test]
    fn test_ending_second() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "2".to_string(),
            text: Some("2.".to_string()),
            print_object: None,
            end_length: Some(30.0),
            text_x: Some(5.0),
            text_y: Some(-10.0),
        };
        assert_eq!(ending.end_length, Some(30.0));
    }

    #[test]
    fn test_ending_stop() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Stop,
            number: "1".to_string(),
            text: None,
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };
        assert_eq!(ending.r#type, StartStopDiscontinue::Stop);
    }

    #[test]
    fn test_ending_discontinue() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Discontinue,
            number: "1, 2".to_string(),
            text: Some("1, 2.".to_string()),
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };
        assert_eq!(ending.r#type, StartStopDiscontinue::Discontinue);
    }
}
