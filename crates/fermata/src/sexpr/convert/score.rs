//! S-expression conversions for score structure types.
//!
//! This module implements [`ToSexpr`] for high-level score structure types:
//! - [`ScorePartwise`] - The root score element
//! - [`Part`] - A musical part
//! - [`Measure`] - A measure within a part
//! - [`MusicDataElement`] - Elements within a measure
//! - Part-list types (`PartList`, `ScorePart`, etc.)

use crate::ir::measure::{Measure, MusicDataElement};
use crate::ir::part::{
    GroupBarline, GroupBarlineValue, GroupName, GroupSymbol, MidiDevice, MidiInstrument,
    NameDisplay, NameDisplayContent, Part, PartGroup, PartList, PartListElement, PartName,
    ScoreInstrument, ScorePart, SoloOrEnsemble, VirtualInstrument,
};
use crate::ir::score::{Credit, CreditContent, CreditWords, ScorePartwise, Work};
use crate::sexpr::{ListBuilder, Sexpr, ToSexpr};

// ============================================================================
// ScorePartwise (root element)
// ============================================================================

impl ToSexpr for ScorePartwise {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("score");

        // Metadata
        if let Some(ref work) = self.work {
            if let Some(ref title) = work.work_title {
                builder = builder.kwarg("title", title);
            }
            if let Some(ref number) = work.work_number {
                builder = builder.kwarg("work-number", number);
            }
        }
        if let Some(ref title) = self.movement_title {
            builder = builder.kwarg("movement-title", title);
        }
        if let Some(ref number) = self.movement_number {
            builder = builder.kwarg("movement-number", number);
        }

        // Credits (composer, lyricist, etc.)
        for credit in &self.credits {
            builder = builder.arg(credit.to_sexpr());
        }

        // Part list (instrument definitions)
        if !self.part_list.content.is_empty() {
            builder = builder.arg(self.part_list.to_sexpr());
        }

        // Parts
        for part in &self.parts {
            builder = builder.arg(part.to_sexpr());
        }

        builder.build()
    }
}

// ============================================================================
// Work
// ============================================================================

impl ToSexpr for Work {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("work")
            .kwarg_opt("number", &self.work_number)
            .kwarg_opt("title", &self.work_title)
            .build()
    }
}

// ============================================================================
// Credit
// ============================================================================

impl ToSexpr for Credit {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("credit");

        if let Some(page) = self.page {
            builder = builder.kwarg("page", &page);
        }

        for content in &self.content {
            match content {
                CreditContent::CreditType(t) => {
                    builder = builder.kwarg("type", t);
                }
                CreditContent::CreditWords(words) => {
                    builder = builder.arg(words.to_sexpr());
                }
                _ => {} // Skip other credit content for now
            }
        }

        builder.build()
    }
}

impl ToSexpr for CreditWords {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("credit-words")
            .kwarg("text", &self.value)
            .build()
    }
}

// ============================================================================
// PartList
// ============================================================================

impl ToSexpr for PartList {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("part-list");

        for element in &self.content {
            builder = builder.arg(element.to_sexpr());
        }

        builder.build()
    }
}

impl ToSexpr for PartListElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            PartListElement::ScorePart(sp) => sp.to_sexpr(),
            PartListElement::PartGroup(pg) => pg.to_sexpr(),
        }
    }
}

impl ToSexpr for ScorePart {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("score-part")
            .kwarg("id", &self.id)
            .kwarg("name", &self.part_name.value);

        if let Some(ref abbrev) = self.part_abbreviation {
            builder = builder.kwarg("abbreviation", &abbrev.value);
        }

        for instrument in &self.score_instruments {
            builder = builder.arg(instrument.to_sexpr());
        }

        for midi in &self.midi_instruments {
            builder = builder.arg(midi.to_sexpr());
        }

        builder.build()
    }
}

impl ToSexpr for PartName {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("part-name")
            .kwarg("value", &self.value)
            .build()
    }
}

impl ToSexpr for NameDisplay {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("name-display");

        for content in &self.content {
            builder = builder.arg(content.to_sexpr());
        }

        builder.build()
    }
}

impl ToSexpr for NameDisplayContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            NameDisplayContent::DisplayText(ft) => ListBuilder::new("display-text")
                .kwarg("text", &ft.value)
                .build(),
            NameDisplayContent::AccidentalText(at) => ListBuilder::new("accidental-text")
                .kwarg("value", &at.value)
                .build(),
        }
    }
}

impl ToSexpr for ScoreInstrument {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("score-instrument")
            .kwarg("id", &self.id)
            .kwarg("name", &self.instrument_name);

        if let Some(ref abbrev) = self.instrument_abbreviation {
            builder = builder.kwarg("abbreviation", abbrev);
        }

        if let Some(ref sound) = self.instrument_sound {
            builder = builder.kwarg("sound", sound);
        }

        if let Some(ref soe) = self.solo_or_ensemble {
            builder = builder.arg(soe.to_sexpr());
        }

        if let Some(ref vi) = self.virtual_instrument {
            builder = builder.arg(vi.to_sexpr());
        }

        builder.build()
    }
}

impl ToSexpr for SoloOrEnsemble {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            SoloOrEnsemble::Solo => Sexpr::keyword("solo"),
            SoloOrEnsemble::Ensemble(size) => {
                ListBuilder::new("ensemble").kwarg("size", size).build()
            }
        }
    }
}

impl ToSexpr for VirtualInstrument {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("virtual-instrument")
            .kwarg_opt("library", &self.virtual_library)
            .kwarg_opt("name", &self.virtual_name)
            .build()
    }
}

impl ToSexpr for MidiDevice {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("midi-device")
            .kwarg("name", &self.value)
            .kwarg_opt("port", &self.port)
            .kwarg_opt("id", &self.id)
            .build()
    }
}

impl ToSexpr for MidiInstrument {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("midi-instrument")
            .kwarg("id", &self.id)
            .kwarg_opt("channel", &self.midi_channel)
            .kwarg_opt("name", &self.midi_name)
            .kwarg_opt("bank", &self.midi_bank)
            .kwarg_opt("program", &self.midi_program)
            .kwarg_opt("unpitched", &self.midi_unpitched)
            .kwarg_opt("volume", &self.volume)
            .kwarg_opt("pan", &self.pan)
            .kwarg_opt("elevation", &self.elevation)
            .build()
    }
}

impl ToSexpr for PartGroup {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("part-group")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number);

        if let Some(ref name) = self.group_name {
            builder = builder.arg(name.to_sexpr());
        }

        if let Some(ref symbol) = self.group_symbol {
            builder = builder.arg(symbol.to_sexpr());
        }

        if let Some(ref barline) = self.group_barline {
            builder = builder.arg(barline.to_sexpr());
        }

        builder.build()
    }
}

impl ToSexpr for GroupName {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("group-name")
            .kwarg("value", &self.value)
            .build()
    }
}

impl ToSexpr for GroupSymbol {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("group-symbol")
            .kwarg("value", &self.value)
            .build()
    }
}

impl ToSexpr for GroupBarline {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("group-barline")
            .kwarg("value", &self.value)
            .build()
    }
}

impl ToSexpr for GroupBarlineValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::keyword(match self {
            GroupBarlineValue::Yes => "yes",
            GroupBarlineValue::No => "no",
            GroupBarlineValue::Mensurstrich => "mensurstrich",
        })
    }
}

// ============================================================================
// Part
// ============================================================================

impl ToSexpr for Part {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("part").kwarg("id", &self.id);

        for measure in &self.measures {
            builder = builder.arg(measure.to_sexpr());
        }

        builder.build()
    }
}

// ============================================================================
// Measure
// ============================================================================

impl ToSexpr for Measure {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("measure")
            .kwarg("number", &self.number)
            .kwarg_opt("implicit", &self.implicit)
            .kwarg_opt("width", &self.width);

        for element in &self.content {
            builder = builder.arg(element.to_sexpr());
        }

        builder.build()
    }
}

// ============================================================================
// MusicDataElement
// ============================================================================

impl ToSexpr for MusicDataElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            MusicDataElement::Note(note) => note.to_sexpr(),
            MusicDataElement::Backup(backup) => backup.to_sexpr(),
            MusicDataElement::Forward(forward) => forward.to_sexpr(),
            MusicDataElement::Direction(direction) => direction.to_sexpr(),
            MusicDataElement::Attributes(attributes) => attributes.to_sexpr(),
            MusicDataElement::Barline(barline) => barline.to_sexpr(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::attributes::GroupSymbolValue;
    use crate::ir::common::{
        AccidentalValue, Editorial, FormattedText, Position, PrintStyle, StartStop, YesNo,
    };
    use crate::sexpr::print_sexpr;

    // ============================================================================
    // ScorePartwise Tests
    // ============================================================================

    #[test]
    fn test_score_partwise_basic() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_number: None,
                work_title: Some("Test Score".to_string()),
                opus: None,
            }),
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        };

        let sexpr = score.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score"));
        assert!(output.contains("Test Score"));
    }

    #[test]
    fn test_score_partwise_with_movement_info() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_number: Some("Op. 21".to_string()),
                work_title: Some("Symphony No. 1".to_string()),
                opus: None,
            }),
            movement_number: Some("1".to_string()),
            movement_title: Some("Allegro con brio".to_string()),
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        };

        let sexpr = score.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score"));
        assert!(output.contains("Symphony No. 1"));
        assert!(output.contains("Op. 21"));
        assert!(output.contains("movement-title"));
        assert!(output.contains("Allegro con brio"));
        assert!(output.contains("movement-number"));
    }

    #[test]
    fn test_score_partwise_with_credits() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![
                Credit {
                    page: Some(1),
                    content: vec![
                        CreditContent::CreditType("title".to_string()),
                        CreditContent::CreditWords(CreditWords {
                            value: "Symphony No. 5".to_string(),
                            print_style: PrintStyle::default(),
                            justify: None,
                            halign: None,
                            valign: None,
                            lang: None,
                        }),
                    ],
                },
                Credit {
                    page: Some(1),
                    content: vec![
                        CreditContent::CreditType("composer".to_string()),
                        CreditContent::CreditWords(CreditWords {
                            value: "Ludwig van Beethoven".to_string(),
                            print_style: PrintStyle::default(),
                            justify: None,
                            halign: None,
                            valign: None,
                            lang: None,
                        }),
                    ],
                },
            ],
            part_list: PartList { content: vec![] },
            parts: vec![],
        };

        let sexpr = score.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit"));
        assert!(output.contains("Symphony No. 5"));
        assert!(output.contains("Ludwig van Beethoven"));
    }

    // ============================================================================
    // Work Tests
    // ============================================================================

    #[test]
    fn test_work_with_number_and_title() {
        let work = Work {
            work_number: Some("Op. 27, No. 2".to_string()),
            work_title: Some("Moonlight Sonata".to_string()),
            opus: None,
        };

        let sexpr = work.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("work"));
        assert!(output.contains("number"));
        assert!(output.contains("Op. 27, No. 2"));
        assert!(output.contains("title"));
        assert!(output.contains("Moonlight Sonata"));
    }

    #[test]
    fn test_work_with_only_title() {
        let work = Work {
            work_number: None,
            work_title: Some("Fur Elise".to_string()),
            opus: None,
        };

        let sexpr = work.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("work"));
        assert!(output.contains("Fur Elise"));
        assert!(!output.contains("number"));
    }

    #[test]
    fn test_work_with_only_number() {
        let work = Work {
            work_number: Some("BWV 846".to_string()),
            work_title: None,
            opus: None,
        };

        let sexpr = work.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("work"));
        assert!(output.contains("BWV 846"));
    }

    #[test]
    fn test_work_empty() {
        let work = Work {
            work_number: None,
            work_title: None,
            opus: None,
        };

        let sexpr = work.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("work"));
    }

    // ============================================================================
    // Credit Tests
    // ============================================================================

    #[test]
    fn test_credit_with_page_and_type() {
        let credit = Credit {
            page: Some(1),
            content: vec![CreditContent::CreditType("title".to_string())],
        };

        let sexpr = credit.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit"));
        assert!(output.contains("page"));
        assert!(output.contains("type"));
        assert!(output.contains("title"));
    }

    #[test]
    fn test_credit_with_credit_words() {
        let credit = Credit {
            page: Some(2),
            content: vec![CreditContent::CreditWords(CreditWords {
                value: "Composer: J.S. Bach".to_string(),
                print_style: PrintStyle::default(),
                justify: None,
                halign: None,
                valign: None,
                lang: None,
            })],
        };

        let sexpr = credit.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit"));
        assert!(output.contains("credit-words"));
        assert!(output.contains("Composer: J.S. Bach"));
    }

    #[test]
    fn test_credit_with_multiple_content() {
        let credit = Credit {
            page: Some(1),
            content: vec![
                CreditContent::CreditType("composer".to_string()),
                CreditContent::CreditWords(CreditWords {
                    value: "Wolfgang Amadeus Mozart".to_string(),
                    print_style: PrintStyle::default(),
                    justify: None,
                    halign: None,
                    valign: None,
                    lang: None,
                }),
            ],
        };

        let sexpr = credit.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit"));
        assert!(output.contains("composer"));
        assert!(output.contains("Wolfgang Amadeus Mozart"));
    }

    #[test]
    fn test_credit_without_page() {
        let credit = Credit {
            page: None,
            content: vec![CreditContent::CreditType("lyricist".to_string())],
        };

        let sexpr = credit.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit"));
        assert!(!output.contains("page"));
        assert!(output.contains("lyricist"));
    }

    // ============================================================================
    // CreditWords Tests
    // ============================================================================

    #[test]
    fn test_credit_words_basic() {
        let cw = CreditWords {
            value: "Test Credit Text".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            halign: None,
            valign: None,
            lang: None,
        };

        let sexpr = cw.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit-words"));
        assert!(output.contains("text"));
        assert!(output.contains("Test Credit Text"));
    }

    #[test]
    fn test_credit_words_with_special_characters() {
        let cw = CreditWords {
            value: "Title with \"quotes\" and more".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            halign: None,
            valign: None,
            lang: None,
        };

        let sexpr = cw.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("credit-words"));
    }

    // ============================================================================
    // PartGroup Tests
    // ============================================================================

    #[test]
    fn test_part_group_start_with_name_symbol_barline() {
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

        let sexpr = pg.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-group"));
        assert!(output.contains("type"));
        assert!(output.contains("group-name"));
        assert!(output.contains("Strings"));
        assert!(output.contains("group-symbol"));
        assert!(output.contains("group-barline"));
    }

    #[test]
    fn test_part_group_stop() {
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

        let sexpr = pg.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-group"));
        assert!(output.contains("type"));
    }

    #[test]
    fn test_part_group_without_optional_fields() {
        let pg = PartGroup {
            r#type: StartStop::Start,
            number: None,
            group_name: None,
            group_name_display: None,
            group_abbreviation: None,
            group_abbreviation_display: None,
            group_symbol: None,
            group_barline: None,
            group_time: None,
            editorial: Editorial::default(),
        };

        let sexpr = pg.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-group"));
        assert!(!output.contains("group-name"));
        assert!(!output.contains("group-symbol"));
        assert!(!output.contains("group-barline"));
    }

    // ============================================================================
    // GroupName Tests
    // ============================================================================

    #[test]
    fn test_group_name_basic() {
        let gn = GroupName {
            value: "Woodwinds".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
        };

        let sexpr = gn.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-name"));
        assert!(output.contains("value"));
        assert!(output.contains("Woodwinds"));
    }

    #[test]
    fn test_group_name_orchestra_section() {
        let gn = GroupName {
            value: "Brass".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
        };

        let sexpr = gn.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-name"));
        assert!(output.contains("Brass"));
    }

    // ============================================================================
    // GroupSymbol Tests
    // ============================================================================

    #[test]
    fn test_group_symbol_brace() {
        let gs = GroupSymbol {
            value: GroupSymbolValue::Brace,
            position: Position::default(),
            color: None,
        };

        let sexpr = gs.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-symbol"));
        assert!(output.contains("value"));
    }

    #[test]
    fn test_group_symbol_bracket() {
        let gs = GroupSymbol {
            value: GroupSymbolValue::Bracket,
            position: Position::default(),
            color: None,
        };

        let sexpr = gs.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-symbol"));
    }

    #[test]
    fn test_group_symbol_line() {
        let gs = GroupSymbol {
            value: GroupSymbolValue::Line,
            position: Position::default(),
            color: None,
        };

        let sexpr = gs.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-symbol"));
    }

    // ============================================================================
    // GroupBarline Tests
    // ============================================================================

    #[test]
    fn test_group_barline_basic() {
        let gb = GroupBarline {
            value: GroupBarlineValue::Yes,
            color: None,
        };

        let sexpr = gb.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-barline"));
        assert!(output.contains("value"));
    }

    #[test]
    fn test_group_barline_no() {
        let gb = GroupBarline {
            value: GroupBarlineValue::No,
            color: None,
        };

        let sexpr = gb.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-barline"));
    }

    #[test]
    fn test_group_barline_mensurstrich() {
        let gb = GroupBarline {
            value: GroupBarlineValue::Mensurstrich,
            color: None,
        };

        let sexpr = gb.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("group-barline"));
    }

    // ============================================================================
    // GroupBarlineValue Tests
    // ============================================================================

    #[test]
    fn test_group_barline_value_yes() {
        let gbv = GroupBarlineValue::Yes;

        let sexpr = gbv.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("yes"));
    }

    #[test]
    fn test_group_barline_value_no() {
        let gbv = GroupBarlineValue::No;

        let sexpr = gbv.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("no"));
    }

    #[test]
    fn test_group_barline_value_mensurstrich() {
        let gbv = GroupBarlineValue::Mensurstrich;

        let sexpr = gbv.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("mensurstrich"));
    }

    // ============================================================================
    // NameDisplay Tests
    // ============================================================================

    #[test]
    fn test_name_display_with_display_text() {
        let nd = NameDisplay {
            print_object: Some(YesNo::Yes),
            content: vec![NameDisplayContent::DisplayText(FormattedText {
                value: "Violin I".to_string(),
                print_style: PrintStyle::default(),
                lang: None,
            })],
        };

        let sexpr = nd.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("name-display"));
        assert!(output.contains("display-text"));
        assert!(output.contains("Violin I"));
    }

    #[test]
    fn test_name_display_with_accidental_text() {
        let nd = NameDisplay {
            print_object: Some(YesNo::Yes),
            content: vec![NameDisplayContent::AccidentalText(
                crate::ir::part::AccidentalText {
                    value: AccidentalValue::Flat,
                    print_style: PrintStyle::default(),
                },
            )],
        };

        let sexpr = nd.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("name-display"));
        assert!(output.contains("accidental-text"));
    }

    #[test]
    fn test_name_display_empty() {
        let nd = NameDisplay {
            print_object: None,
            content: vec![],
        };

        let sexpr = nd.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("name-display"));
    }

    #[test]
    fn test_name_display_multiple_content() {
        let nd = NameDisplay {
            print_object: Some(YesNo::Yes),
            content: vec![
                NameDisplayContent::DisplayText(FormattedText {
                    value: "Clarinet in B".to_string(),
                    print_style: PrintStyle::default(),
                    lang: None,
                }),
                NameDisplayContent::AccidentalText(crate::ir::part::AccidentalText {
                    value: AccidentalValue::Flat,
                    print_style: PrintStyle::default(),
                }),
            ],
        };

        let sexpr = nd.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("name-display"));
        assert!(output.contains("display-text"));
        assert!(output.contains("Clarinet in B"));
        assert!(output.contains("accidental-text"));
    }

    // ============================================================================
    // NameDisplayContent Tests
    // ============================================================================

    #[test]
    fn test_name_display_content_display_text() {
        let content = NameDisplayContent::DisplayText(FormattedText {
            value: "Horn in F".to_string(),
            print_style: PrintStyle::default(),
            lang: Some("en".to_string()),
        });

        let sexpr = content.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("display-text"));
        assert!(output.contains("text"));
        assert!(output.contains("Horn in F"));
    }

    #[test]
    fn test_name_display_content_accidental_text() {
        let content = NameDisplayContent::AccidentalText(crate::ir::part::AccidentalText {
            value: AccidentalValue::Sharp,
            print_style: PrintStyle::default(),
        });

        let sexpr = content.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("accidental-text"));
        assert!(output.contains("value"));
    }

    // ============================================================================
    // ScoreInstrument Tests
    // ============================================================================

    #[test]
    fn test_score_instrument_basic() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Piano".to_string(),
            instrument_abbreviation: None,
            instrument_sound: None,
            solo_or_ensemble: None,
            virtual_instrument: None,
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("id"));
        assert!(output.contains("P1-I1"));
        assert!(output.contains("name"));
        assert!(output.contains("Piano"));
    }

    #[test]
    fn test_score_instrument_with_abbreviation() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Violoncello".to_string(),
            instrument_abbreviation: Some("Vc.".to_string()),
            instrument_sound: None,
            solo_or_ensemble: None,
            virtual_instrument: None,
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("Violoncello"));
        assert!(output.contains("abbreviation"));
        assert!(output.contains("Vc."));
    }

    #[test]
    fn test_score_instrument_with_sound() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Acoustic Grand Piano".to_string(),
            instrument_abbreviation: None,
            instrument_sound: Some("keyboard.piano.grand".to_string()),
            solo_or_ensemble: None,
            virtual_instrument: None,
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("sound"));
        assert!(output.contains("keyboard.piano.grand"));
    }

    #[test]
    fn test_score_instrument_with_solo() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Solo Violin".to_string(),
            instrument_abbreviation: None,
            instrument_sound: Some("strings.violin".to_string()),
            solo_or_ensemble: Some(SoloOrEnsemble::Solo),
            virtual_instrument: None,
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("solo"));
    }

    #[test]
    fn test_score_instrument_with_ensemble() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Violin Section".to_string(),
            instrument_abbreviation: None,
            instrument_sound: Some("strings.violin".to_string()),
            solo_or_ensemble: Some(SoloOrEnsemble::Ensemble(16)),
            virtual_instrument: None,
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("ensemble"));
        assert!(output.contains("size"));
    }

    #[test]
    fn test_score_instrument_with_virtual_instrument() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Strings".to_string(),
            instrument_abbreviation: None,
            instrument_sound: None,
            solo_or_ensemble: None,
            virtual_instrument: Some(VirtualInstrument {
                virtual_library: Some("Vienna Symphonic Library".to_string()),
                virtual_name: Some("Solo Violin".to_string()),
            }),
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("virtual-instrument"));
        assert!(output.contains("library"));
        assert!(output.contains("Vienna Symphonic Library"));
    }

    #[test]
    fn test_score_instrument_full() {
        let si = ScoreInstrument {
            id: "P1-I1".to_string(),
            instrument_name: "Concert Grand Piano".to_string(),
            instrument_abbreviation: Some("Pno.".to_string()),
            instrument_sound: Some("keyboard.piano.grand".to_string()),
            solo_or_ensemble: Some(SoloOrEnsemble::Solo),
            virtual_instrument: Some(VirtualInstrument {
                virtual_library: Some("Steinway Library".to_string()),
                virtual_name: Some("Model D".to_string()),
            }),
        };

        let sexpr = si.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-instrument"));
        assert!(output.contains("Concert Grand Piano"));
        assert!(output.contains("Pno."));
        assert!(output.contains("keyboard.piano.grand"));
        assert!(output.contains("solo"));
        assert!(output.contains("virtual-instrument"));
    }

    // ============================================================================
    // SoloOrEnsemble Tests
    // ============================================================================

    #[test]
    fn test_solo_or_ensemble_solo() {
        let soe = SoloOrEnsemble::Solo;

        let sexpr = soe.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("solo"));
    }

    #[test]
    fn test_solo_or_ensemble_ensemble() {
        let soe = SoloOrEnsemble::Ensemble(8);

        let sexpr = soe.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("ensemble"));
        assert!(output.contains("size"));
        assert!(output.contains("8"));
    }

    #[test]
    fn test_solo_or_ensemble_large_ensemble() {
        let soe = SoloOrEnsemble::Ensemble(60);

        let sexpr = soe.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("ensemble"));
        assert!(output.contains("60"));
    }

    // ============================================================================
    // VirtualInstrument Tests
    // ============================================================================

    #[test]
    fn test_virtual_instrument_with_library_and_name() {
        let vi = VirtualInstrument {
            virtual_library: Some("EastWest".to_string()),
            virtual_name: Some("Hollywood Strings".to_string()),
        };

        let sexpr = vi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("virtual-instrument"));
        assert!(output.contains("library"));
        assert!(output.contains("EastWest"));
        assert!(output.contains("name"));
        assert!(output.contains("Hollywood Strings"));
    }

    #[test]
    fn test_virtual_instrument_library_only() {
        let vi = VirtualInstrument {
            virtual_library: Some("Kontakt".to_string()),
            virtual_name: None,
        };

        let sexpr = vi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("virtual-instrument"));
        assert!(output.contains("library"));
        assert!(output.contains("Kontakt"));
    }

    #[test]
    fn test_virtual_instrument_name_only() {
        let vi = VirtualInstrument {
            virtual_library: None,
            virtual_name: Some("Grand Piano".to_string()),
        };

        let sexpr = vi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("virtual-instrument"));
        assert!(output.contains("name"));
        assert!(output.contains("Grand Piano"));
    }

    #[test]
    fn test_virtual_instrument_empty() {
        let vi = VirtualInstrument {
            virtual_library: None,
            virtual_name: None,
        };

        let sexpr = vi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("virtual-instrument"));
    }

    // ============================================================================
    // MidiDevice Tests
    // ============================================================================

    #[test]
    fn test_midi_device_with_port_and_id() {
        let md = MidiDevice {
            value: "MIDI Out 1".to_string(),
            port: Some(1),
            id: Some("P1-I1".to_string()),
        };

        let sexpr = md.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-device"));
        assert!(output.contains("name"));
        assert!(output.contains("MIDI Out 1"));
        assert!(output.contains("port"));
        assert!(output.contains("id"));
        assert!(output.contains("P1-I1"));
    }

    #[test]
    fn test_midi_device_name_only() {
        let md = MidiDevice {
            value: "Default MIDI".to_string(),
            port: None,
            id: None,
        };

        let sexpr = md.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-device"));
        assert!(output.contains("Default MIDI"));
    }

    #[test]
    fn test_midi_device_with_port() {
        let md = MidiDevice {
            value: "External Synth".to_string(),
            port: Some(2),
            id: None,
        };

        let sexpr = md.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-device"));
        assert!(output.contains("port"));
        assert!(output.contains("2"));
    }

    // ============================================================================
    // MidiInstrument Tests
    // ============================================================================

    #[test]
    fn test_midi_instrument_basic() {
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

        let sexpr = mi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-instrument"));
        assert!(output.contains("id"));
        assert!(output.contains("P1-I1"));
        assert!(output.contains("channel"));
        assert!(output.contains("program"));
    }

    #[test]
    fn test_midi_instrument_with_all_fields() {
        let mi = MidiInstrument {
            id: "P1-I1".to_string(),
            midi_channel: Some(2),
            midi_name: Some("Strings".to_string()),
            midi_bank: Some(0),
            midi_program: Some(49),
            midi_unpitched: None,
            volume: Some(80.0),
            pan: Some(-45.0),
            elevation: Some(15.0),
        };

        let sexpr = mi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-instrument"));
        assert!(output.contains("channel"));
        assert!(output.contains("2"));
        assert!(output.contains("name"));
        assert!(output.contains("Strings"));
        assert!(output.contains("bank"));
        assert!(output.contains("program"));
        assert!(output.contains("49"));
        assert!(output.contains("volume"));
        assert!(output.contains("80"));
        assert!(output.contains("pan"));
        assert!(output.contains("-45"));
        assert!(output.contains("elevation"));
    }

    #[test]
    fn test_midi_instrument_percussion() {
        let mi = MidiInstrument {
            id: "P2-I1".to_string(),
            midi_channel: Some(10),
            midi_name: Some("Snare Drum".to_string()),
            midi_bank: None,
            midi_program: None,
            midi_unpitched: Some(38),
            volume: Some(100.0),
            pan: Some(0.0),
            elevation: None,
        };

        let sexpr = mi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-instrument"));
        assert!(output.contains("channel"));
        assert!(output.contains("10"));
        assert!(output.contains("unpitched"));
        assert!(output.contains("38"));
    }

    #[test]
    fn test_midi_instrument_id_only() {
        let mi = MidiInstrument {
            id: "P1-I1".to_string(),
            midi_channel: None,
            midi_name: None,
            midi_bank: None,
            midi_program: None,
            midi_unpitched: None,
            volume: None,
            pan: None,
            elevation: None,
        };

        let sexpr = mi.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("midi-instrument"));
        assert!(output.contains("P1-I1"));
    }

    // ============================================================================
    // PartName Tests
    // ============================================================================

    #[test]
    fn test_part_name_basic() {
        let pn = PartName {
            value: "Flute".to_string(),
            print_style: PrintStyle::default(),
            print_object: None,
            justify: None,
        };

        let sexpr = pn.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-name"));
        assert!(output.contains("value"));
        assert!(output.contains("Flute"));
    }

    #[test]
    fn test_part_name_long_name() {
        let pn = PartName {
            value: "Clarinet in B-flat".to_string(),
            print_style: PrintStyle::default(),
            print_object: None,
            justify: None,
        };

        let sexpr = pn.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-name"));
        assert!(output.contains("Clarinet in B-flat"));
    }

    // ============================================================================
    // Part Tests
    // ============================================================================

    #[test]
    fn test_part_basic() {
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

        let sexpr = part.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part"));
        assert!(output.contains("P1"));
        assert!(output.contains("measure"));
    }

    #[test]
    fn test_part_multiple_measures() {
        let part = Part {
            id: "P1".to_string(),
            measures: vec![
                Measure {
                    number: "1".to_string(),
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    content: vec![],
                },
                Measure {
                    number: "2".to_string(),
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    content: vec![],
                },
                Measure {
                    number: "3".to_string(),
                    implicit: None,
                    non_controlling: None,
                    width: None,
                    content: vec![],
                },
            ],
        };

        let sexpr = part.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part"));
        // All measure numbers should appear
        assert!(output.contains("\"1\"") || output.contains(":number \"1\""));
        assert!(output.contains("\"2\"") || output.contains(":number \"2\""));
        assert!(output.contains("\"3\"") || output.contains(":number \"3\""));
    }

    #[test]
    fn test_part_empty_measures() {
        let part = Part {
            id: "P2".to_string(),
            measures: vec![],
        };

        let sexpr = part.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part"));
        assert!(output.contains("P2"));
    }

    // ============================================================================
    // Measure Tests
    // ============================================================================

    #[test]
    fn test_measure_basic() {
        let measure = Measure {
            number: "42".to_string(),
            implicit: None,
            non_controlling: None,
            width: Some(200.0),
            content: vec![],
        };

        let sexpr = measure.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("measure"));
        assert!(output.contains("42"));
        assert!(output.contains("200"));
    }

    #[test]
    fn test_measure_with_implicit_true() {
        let measure = Measure {
            number: "0".to_string(),
            implicit: Some(YesNo::Yes),
            non_controlling: None,
            width: None,
            content: vec![],
        };

        let sexpr = measure.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("measure"));
        assert!(output.contains("implicit"));
    }

    #[test]
    fn test_measure_with_implicit_false() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: Some(YesNo::No),
            non_controlling: None,
            width: None,
            content: vec![],
        };

        let sexpr = measure.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("measure"));
        assert!(output.contains("implicit"));
    }

    #[test]
    fn test_measure_without_width() {
        let measure = Measure {
            number: "5".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };

        let sexpr = measure.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("measure"));
        assert!(output.contains("5"));
        assert!(!output.contains("width"));
    }

    // ============================================================================
    // ScorePart Tests
    // ============================================================================

    #[test]
    fn test_score_part_basic() {
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
            part_abbreviation: Some(PartName {
                value: "Pno.".to_string(),
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

        let sexpr = sp.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-part"));
        assert!(output.contains("Piano"));
        assert!(output.contains("Pno."));
    }

    #[test]
    fn test_score_part_with_instruments() {
        let sp = ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Violin".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            },
            part_name_display: None,
            part_abbreviation: Some(PartName {
                value: "Vln.".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            }),
            part_abbreviation_display: None,
            group: vec![],
            score_instruments: vec![ScoreInstrument {
                id: "P1-I1".to_string(),
                instrument_name: "Violin".to_string(),
                instrument_abbreviation: Some("Vln.".to_string()),
                instrument_sound: Some("strings.violin".to_string()),
                solo_or_ensemble: Some(SoloOrEnsemble::Solo),
                virtual_instrument: None,
            }],
            midi_devices: vec![],
            midi_instruments: vec![MidiInstrument {
                id: "P1-I1".to_string(),
                midi_channel: Some(1),
                midi_name: None,
                midi_bank: None,
                midi_program: Some(41),
                midi_unpitched: None,
                volume: Some(80.0),
                pan: Some(0.0),
                elevation: None,
            }],
        };

        let sexpr = sp.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-part"));
        assert!(output.contains("score-instrument"));
        assert!(output.contains("midi-instrument"));
        assert!(output.contains("strings.violin"));
    }

    #[test]
    fn test_score_part_multiple_instruments() {
        let sp = ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Percussion".to_string(),
                print_style: PrintStyle::default(),
                print_object: None,
                justify: None,
            },
            part_name_display: None,
            part_abbreviation: None,
            part_abbreviation_display: None,
            group: vec![],
            score_instruments: vec![
                ScoreInstrument {
                    id: "P1-I1".to_string(),
                    instrument_name: "Snare Drum".to_string(),
                    instrument_abbreviation: None,
                    instrument_sound: Some("drum.snare-drum".to_string()),
                    solo_or_ensemble: None,
                    virtual_instrument: None,
                },
                ScoreInstrument {
                    id: "P1-I2".to_string(),
                    instrument_name: "Bass Drum".to_string(),
                    instrument_abbreviation: None,
                    instrument_sound: Some("drum.bass-drum".to_string()),
                    solo_or_ensemble: None,
                    virtual_instrument: None,
                },
            ],
            midi_devices: vec![],
            midi_instruments: vec![
                MidiInstrument {
                    id: "P1-I1".to_string(),
                    midi_channel: Some(10),
                    midi_name: None,
                    midi_bank: None,
                    midi_program: None,
                    midi_unpitched: Some(38),
                    volume: Some(100.0),
                    pan: Some(0.0),
                    elevation: None,
                },
                MidiInstrument {
                    id: "P1-I2".to_string(),
                    midi_channel: Some(10),
                    midi_name: None,
                    midi_bank: None,
                    midi_program: None,
                    midi_unpitched: Some(36),
                    volume: Some(100.0),
                    pan: Some(0.0),
                    elevation: None,
                },
            ],
        };

        let sexpr = sp.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-part"));
        assert!(output.contains("Snare Drum"));
        assert!(output.contains("Bass Drum"));
    }

    // ============================================================================
    // PartList Tests
    // ============================================================================

    #[test]
    fn test_part_list_basic() {
        let pl = PartList {
            content: vec![PartListElement::ScorePart(ScorePart {
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
            })],
        };

        let sexpr = pl.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-list"));
        assert!(output.contains("Violin"));
    }

    #[test]
    fn test_part_list_with_part_group() {
        let pl = PartList {
            content: vec![
                PartListElement::PartGroup(PartGroup {
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
                }),
                PartListElement::ScorePart(ScorePart {
                    id: "P1".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Violin I".to_string(),
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
                }),
                PartListElement::ScorePart(ScorePart {
                    id: "P2".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Violin II".to_string(),
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
                }),
                PartListElement::PartGroup(PartGroup {
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
                }),
            ],
        };

        let sexpr = pl.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-list"));
        assert!(output.contains("part-group"));
        assert!(output.contains("Strings"));
        assert!(output.contains("Violin I"));
        assert!(output.contains("Violin II"));
    }

    #[test]
    fn test_part_list_empty() {
        let pl = PartList { content: vec![] };

        let sexpr = pl.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-list"));
    }

    // ============================================================================
    // PartListElement Tests
    // ============================================================================

    #[test]
    fn test_part_list_element_score_part() {
        let elem = PartListElement::ScorePart(ScorePart {
            id: "P1".to_string(),
            identification: None,
            part_name: PartName {
                value: "Oboe".to_string(),
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

        let sexpr = elem.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("score-part"));
        assert!(output.contains("Oboe"));
    }

    #[test]
    fn test_part_list_element_part_group() {
        let elem = PartListElement::PartGroup(PartGroup {
            r#type: StartStop::Start,
            number: Some("1".to_string()),
            group_name: Some(GroupName {
                value: "Woodwinds".to_string(),
                print_style: PrintStyle::default(),
                justify: None,
            }),
            group_name_display: None,
            group_abbreviation: None,
            group_abbreviation_display: None,
            group_symbol: None,
            group_barline: None,
            group_time: None,
            editorial: Editorial::default(),
        });

        let sexpr = elem.to_sexpr();
        let output = print_sexpr(&sexpr);
        assert!(output.contains("part-group"));
        assert!(output.contains("Woodwinds"));
    }
}
