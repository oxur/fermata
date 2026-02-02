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
    use crate::ir::common::PrintStyle;
    use crate::sexpr::print_sexpr;

    #[test]
    fn test_score_basic() {
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
}
