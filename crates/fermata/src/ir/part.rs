//! Part and part-list types.

use super::common::{
    AccidentalValue, Color, Editorial, FormattedText, Identification, LeftCenterRight, Position,
    PrintStyle, StartStop, YesNo,
};
use super::measure::Measure;

/// A musical part containing measures.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    /// Part ID (must match a score-part ID)
    pub id: String,
    /// Measures in this part
    pub measures: Vec<Measure>,
}

/// The part-list element.
#[derive(Debug, Clone, PartialEq)]
pub struct PartList {
    /// Part list content
    pub content: Vec<PartListElement>,
}

/// Elements within part-list.
#[derive(Debug, Clone, PartialEq)]
pub enum PartListElement {
    /// Score part definition
    ScorePart(ScorePart),
    /// Part group
    PartGroup(PartGroup),
}

/// Score-part definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ScorePart {
    /// Part ID
    pub id: String,
    /// Part identification
    pub identification: Option<Identification>,
    /// Part name
    pub part_name: PartName,
    /// Part name display
    pub part_name_display: Option<NameDisplay>,
    /// Part abbreviation
    pub part_abbreviation: Option<PartName>,
    /// Part abbreviation display
    pub part_abbreviation_display: Option<NameDisplay>,
    /// Group memberships
    pub group: Vec<String>,
    /// Score instruments
    pub score_instruments: Vec<ScoreInstrument>,
    /// MIDI devices
    pub midi_devices: Vec<MidiDevice>,
    /// MIDI instruments
    pub midi_instruments: Vec<MidiInstrument>,
}

/// Part name.
#[derive(Debug, Clone, PartialEq)]
pub struct PartName {
    /// The name value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// Text justification
    pub justify: Option<LeftCenterRight>,
}

/// Name display for alternative formatting.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NameDisplay {
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// Display content
    pub content: Vec<NameDisplayContent>,
}

/// Name display content.
#[derive(Debug, Clone, PartialEq)]
pub enum NameDisplayContent {
    /// Display text
    DisplayText(FormattedText),
    /// Accidental text
    AccidentalText(AccidentalText),
}

/// Accidental text in name display.
#[derive(Debug, Clone, PartialEq)]
pub struct AccidentalText {
    /// The accidental value
    pub value: AccidentalValue,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Score instrument (for playback).
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreInstrument {
    /// Instrument ID
    pub id: String,
    /// Instrument name
    pub instrument_name: String,
    /// Instrument abbreviation
    pub instrument_abbreviation: Option<String>,
    /// Standard instrument sound
    pub instrument_sound: Option<String>,
    /// Solo or ensemble
    pub solo_or_ensemble: Option<SoloOrEnsemble>,
    /// Virtual instrument
    pub virtual_instrument: Option<VirtualInstrument>,
}

/// Solo or ensemble.
#[derive(Debug, Clone, PartialEq)]
pub enum SoloOrEnsemble {
    /// Solo instrument
    Solo,
    /// Ensemble (with size)
    Ensemble(u32),
}

/// Virtual instrument settings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VirtualInstrument {
    /// Virtual library name
    pub virtual_library: Option<String>,
    /// Virtual instrument name
    pub virtual_name: Option<String>,
}

/// MIDI device assignment.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MidiDevice {
    /// Device name
    pub value: String,
    /// MIDI port
    pub port: Option<u16>,
    /// Instrument ID reference
    pub id: Option<String>,
}

/// MIDI instrument settings.
#[derive(Debug, Clone, PartialEq)]
pub struct MidiInstrument {
    /// Instrument ID reference
    pub id: String,
    /// MIDI channel (1-16)
    pub midi_channel: Option<u8>,
    /// MIDI name
    pub midi_name: Option<String>,
    /// MIDI bank
    pub midi_bank: Option<u16>,
    /// MIDI program (1-128)
    pub midi_program: Option<u8>,
    /// MIDI unpitched note
    pub midi_unpitched: Option<u8>,
    /// Volume (0-100)
    pub volume: Option<f64>,
    /// Pan (-180 to 180)
    pub pan: Option<f64>,
    /// Elevation (-180 to 180)
    pub elevation: Option<f64>,
}

/// Part group (for grouping parts in the score).
#[derive(Debug, Clone, PartialEq)]
pub struct PartGroup {
    /// Start or stop
    pub r#type: StartStop,
    /// Group number
    pub number: Option<String>,
    /// Group name
    pub group_name: Option<GroupName>,
    /// Group name display
    pub group_name_display: Option<NameDisplay>,
    /// Group abbreviation
    pub group_abbreviation: Option<GroupName>,
    /// Group abbreviation display
    pub group_abbreviation_display: Option<NameDisplay>,
    /// Group symbol
    pub group_symbol: Option<GroupSymbol>,
    /// Group barline
    pub group_barline: Option<GroupBarline>,
    /// Group time (empty element indicating shared time signature)
    pub group_time: Option<()>,
    /// Editorial information
    pub editorial: Editorial,
}

/// Group name.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupName {
    /// The name value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Text justification
    pub justify: Option<LeftCenterRight>,
}

/// Group symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupSymbol {
    /// Symbol value
    pub value: super::attributes::GroupSymbolValue,
    /// Position attributes
    pub position: Position,
    /// Color
    pub color: Option<Color>,
}

/// Group barline.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupBarline {
    /// Barline value
    pub value: GroupBarlineValue,
    /// Color
    pub color: Option<Color>,
}

/// Group barline values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBarlineValue {
    /// Yes (connect barlines)
    Yes,
    /// No (don't connect)
    No,
    /// Mensurstrich (between staves only)
    Mensurstrich,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::attributes::GroupSymbolValue;

    // === Part Tests ===

    #[test]
    fn test_part_basic() {
        let part = Part {
            id: "P1".to_string(),
            measures: vec![],
        };
        assert_eq!(part.id, "P1");
        assert!(part.measures.is_empty());
    }

    #[test]
    fn test_part_with_measures() {
        let part = Part {
            id: "P1".to_string(),
            measures: vec![Measure {
                number: "1".to_string(),
                implicit: None,
                non_controlling: None,
                width: None,
                content: vec![],
            }],
        };
        assert_eq!(part.measures.len(), 1);
    }

    #[test]
    fn test_part_clone() {
        let part = Part {
            id: "P2".to_string(),
            measures: vec![],
        };
        let cloned = part.clone();
        assert_eq!(part, cloned);
    }

    // === PartList Tests ===

    #[test]
    fn test_partlist_basic() {
        let part_list = PartList { content: vec![] };
        assert!(part_list.content.is_empty());
    }

    #[test]
    fn test_partlist_with_score_part() {
        let part_list = PartList {
            content: vec![PartListElement::ScorePart(ScorePart {
                id: "P1".to_string(),
                identification: None,
                part_name: PartName {
                    value: "Piano".to_string(),
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
        };
        assert_eq!(part_list.content.len(), 1);
    }

    // === PartListElement Tests ===

    #[test]
    fn test_partlistelement_score_part() {
        let elem = PartListElement::ScorePart(ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Violin".to_string(),
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
        });
        if let PartListElement::ScorePart(sp) = elem {
            assert_eq!(sp.id, "P1");
        }
    }

    #[test]
    fn test_partlistelement_part_group() {
        let elem = PartListElement::PartGroup(PartGroup {
            r#type: StartStop::Start,
            number: Some("1".to_string()),
            group_name: None,
            group_name_display: None,
            group_abbreviation: None,
            group_abbreviation_display: None,
            group_symbol: None,
            group_barline: None,
            group_time: None,
            editorial: Editorial::default(),
        });
        if let PartListElement::PartGroup(pg) = elem {
            assert_eq!(pg.r#type, StartStop::Start);
        }
    }

    // === ScorePart Tests ===

    #[test]
    fn test_scorepart_basic() {
        let sp = ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Flute".to_string(),
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
        };
        assert_eq!(sp.id, "P1");
        assert_eq!(sp.part_name.value, "Flute");
    }

    #[test]
    fn test_scorepart_with_abbreviation() {
        let sp = ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Clarinet in B-flat".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            },
            part_name_display: None,
            part_abbreviation: Some(PartName {
                value: "Cl.".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            }),
            part_abbreviation_display: None,
            group: vec![],
            score_instruments: vec![],
            midi_devices: vec![],
            midi_instruments: vec![],
        };
        assert!(sp.part_abbreviation.is_some());
        assert_eq!(sp.part_abbreviation.unwrap().value, "Cl.");
    }

    #[test]
    fn test_scorepart_with_instruments() {
        let sp = ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Piano".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            },
            part_name_display: None,
            part_abbreviation: None,
            part_abbreviation_display: None,
            group: vec![],
            score_instruments: vec![ScoreInstrument {
                id: "P1-I1".to_string(),
                instrument_name: "Piano".to_string(),
                instrument_abbreviation: Some("Pno.".to_string()),
                instrument_sound: Some("keyboard.piano".to_string()),
                solo_or_ensemble: None,
                virtual_instrument: None,
            }],
            midi_devices: vec![],
            midi_instruments: vec![MidiInstrument {
                id: "P1-I1".to_string(),
                midi_channel: Some(1),
                midi_name: None,
                midi_bank: None,
                midi_program: Some(1),
                midi_unpitched: None,
                volume: Some(80.0),
                pan: Some(0.0),
                elevation: None,
            }],
        };
        assert_eq!(sp.score_instruments.len(), 1);
        assert_eq!(sp.midi_instruments.len(), 1);
    }

    // === PartName Tests ===

    #[test]
    fn test_partname_basic() {
        let pn = PartName {
            value: "Voice".to_string(),
            print_style: PrintStyle::default(),
            print_object: None,
            justify: None,
        };
        assert_eq!(pn.value, "Voice");
    }

    #[test]
    fn test_partname_with_justify() {
        let pn = PartName {
            value: "Soprano".to_string(),
            print_style: PrintStyle::default(),
            print_object: Some(YesNo::Yes),
            justify: Some(LeftCenterRight::Left),
        };
        assert_eq!(pn.justify, Some(LeftCenterRight::Left));
    }

    // === NameDisplay Tests ===

    #[test]
    fn test_namedisplay_default() {
        let nd = NameDisplay::default();
        assert!(nd.print_object.is_none());
        assert!(nd.content.is_empty());
    }

    #[test]
    fn test_namedisplay_with_text() {
        let nd = NameDisplay {
            print_object: Some(YesNo::Yes),
            content: vec![NameDisplayContent::DisplayText(FormattedText {
                value: "Violin I".to_string(),
                print_style: PrintStyle::default(),
                lang: None,
            })],
        };
        assert_eq!(nd.content.len(), 1);
    }

    // === NameDisplayContent Tests ===

    #[test]
    fn test_namedisplaycontent_display_text() {
        let content = NameDisplayContent::DisplayText(FormattedText {
            value: "Test".to_string(),
            print_style: PrintStyle::default(),
            lang: Some("en".to_string()),
        });
        if let NameDisplayContent::DisplayText(ft) = content {
            assert_eq!(ft.value, "Test");
        }
    }

    #[test]
    fn test_namedisplaycontent_accidental_text() {
        let content = NameDisplayContent::AccidentalText(AccidentalText {
            value: AccidentalValue::Flat,
            print_style: PrintStyle::default(),
        });
        if let NameDisplayContent::AccidentalText(at) = content {
            assert_eq!(at.value, AccidentalValue::Flat);
        }
    }

    // === ScoreInstrument Tests ===

    #[test]
    fn test_scoreinstrument_basic() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Violin".to_string(),
            instrument_abbreviation: None,
            instrument_sound: None,
            solo_or_ensemble: None,
            virtual_instrument: None,
        };
        assert_eq!(si.id, "P1-I1");
        assert_eq!(si.instrument_name, "Violin");
    }

    #[test]
    fn test_scoreinstrument_with_sound() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Acoustic Grand Piano".to_string(),
            instrument_abbreviation: Some("A.Pno".to_string()),
            instrument_sound: Some("keyboard.piano.grand".to_string()),
            solo_or_ensemble: Some(SoloOrEnsemble::Solo),
            virtual_instrument: None,
        };
        assert_eq!(
            si.instrument_sound,
            Some("keyboard.piano.grand".to_string())
        );
    }

    // === SoloOrEnsemble Tests ===

    #[test]
    fn test_soloorensemble_solo() {
        let soe = SoloOrEnsemble::Solo;
        assert_eq!(soe, SoloOrEnsemble::Solo);
    }

    #[test]
    fn test_soloorensemble_ensemble() {
        let soe = SoloOrEnsemble::Ensemble(16);
        if let SoloOrEnsemble::Ensemble(size) = soe {
            assert_eq!(size, 16);
        }
    }

    // === VirtualInstrument Tests ===

    #[test]
    fn test_virtualinstrument_default() {
        let vi = VirtualInstrument::default();
        assert!(vi.virtual_library.is_none());
        assert!(vi.virtual_name.is_none());
    }

    #[test]
    fn test_virtualinstrument_with_values() {
        let vi = VirtualInstrument {
            virtual_library: Some("Vienna Symphonic Library".to_string()),
            virtual_name: Some("Solo Violin".to_string()),
        };
        assert!(vi.virtual_library.is_some());
        assert!(vi.virtual_name.is_some());
    }

    // === MidiDevice Tests ===

    #[test]
    fn test_mididevice_default() {
        let md = MidiDevice::default();
        assert_eq!(md.value, "");
        assert!(md.port.is_none());
        assert!(md.id.is_none());
    }

    #[test]
    fn test_mididevice_with_values() {
        let md = MidiDevice {
            value: "MIDI Out".to_string(),
            port: Some(1),
            id: Some("P1-I1".to_string()),
        };
        assert_eq!(md.value, "MIDI Out");
        assert_eq!(md.port, Some(1));
    }

    // === MidiInstrument Tests ===

    #[test]
    fn test_midiinstrument_basic() {
        let mi = MidiInstrument {
            id: "P1-I1".to_string(),
            midi_channel: Some(1),
            midi_name: None,
            midi_bank: None,
            midi_program: Some(1),
            midi_unpitched: None,
            volume: None,
            pan: None,
            elevation: None,
        };
        assert_eq!(mi.id, "P1-I1");
        assert_eq!(mi.midi_channel, Some(1));
        assert_eq!(mi.midi_program, Some(1));
    }

    #[test]
    fn test_midiinstrument_full() {
        let mi = MidiInstrument {
            id: "P1-I1".to_string(),
            midi_channel: Some(2),
            midi_name: Some("Strings".to_string()),
            midi_bank: Some(0),
            midi_program: Some(49),
            midi_unpitched: None,
            volume: Some(100.0),
            pan: Some(-45.0),
            elevation: Some(0.0),
        };
        assert_eq!(mi.volume, Some(100.0));
        assert_eq!(mi.pan, Some(-45.0));
    }

    #[test]
    fn test_midiinstrument_percussion() {
        let mi = MidiInstrument {
            id: "P2-I1".to_string(),
            midi_channel: Some(10),
            midi_name: None,
            midi_bank: None,
            midi_program: None,
            midi_unpitched: Some(38),
            volume: Some(80.0),
            pan: Some(0.0),
            elevation: None,
        };
        assert_eq!(mi.midi_unpitched, Some(38));
    }

    // === PartGroup Tests ===

    #[test]
    fn test_partgroup_start() {
        let pg = PartGroup {
            r#type: StartStop::Start,
            number: Some("1".to_string()),
            group_name: Some(GroupName {
                value: "Strings".to_string(),
                print_style: PrintStyle::default(),
                justify: None,
            }),
            group_name_display: None,
            group_abbreviation: None,
            group_abbreviation_display: None,
            group_symbol: Some(GroupSymbol {
                value: GroupSymbolValue::Bracket,
                position: Position::default(),
                color: None,
            }),
            group_barline: Some(GroupBarline {
                value: GroupBarlineValue::Yes,
                color: None,
            }),
            group_time: None,
            editorial: Editorial::default(),
        };
        assert_eq!(pg.r#type, StartStop::Start);
        assert!(pg.group_name.is_some());
    }

    #[test]
    fn test_partgroup_stop() {
        let pg = PartGroup {
            r#type: StartStop::Stop,
            number: Some("1".to_string()),
            group_name: None,
            group_name_display: None,
            group_abbreviation: None,
            group_abbreviation_display: None,
            group_symbol: None,
            group_barline: None,
            group_time: None,
            editorial: Editorial::default(),
        };
        assert_eq!(pg.r#type, StartStop::Stop);
    }

    // === GroupName Tests ===

    #[test]
    fn test_groupname_basic() {
        let gn = GroupName {
            value: "Woodwinds".to_string(),
            print_style: PrintStyle::default(),
            justify: Some(LeftCenterRight::Left),
        };
        assert_eq!(gn.value, "Woodwinds");
    }

    // === GroupSymbol Tests ===

    #[test]
    fn test_groupsymbol_brace() {
        let gs = GroupSymbol {
            value: GroupSymbolValue::Brace,
            position: Position::default(),
            color: None,
        };
        assert_eq!(gs.value, GroupSymbolValue::Brace);
    }

    // === GroupBarline Tests ===

    #[test]
    fn test_groupbarline_yes() {
        let gb = GroupBarline {
            value: GroupBarlineValue::Yes,
            color: None,
        };
        assert_eq!(gb.value, GroupBarlineValue::Yes);
    }

    #[test]
    fn test_groupbarline_mensurstrich() {
        let gb = GroupBarline {
            value: GroupBarlineValue::Mensurstrich,
            color: Some("#000000".to_string()),
        };
        assert_eq!(gb.value, GroupBarlineValue::Mensurstrich);
    }

    // === GroupBarlineValue Tests ===

    #[test]
    fn test_groupbarlinevalue_all_variants() {
        assert_eq!(GroupBarlineValue::Yes, GroupBarlineValue::Yes);
        assert_eq!(GroupBarlineValue::No, GroupBarlineValue::No);
        assert_eq!(
            GroupBarlineValue::Mensurstrich,
            GroupBarlineValue::Mensurstrich
        );
    }
}
