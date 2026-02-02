//! S-expression conversions for `ir::direction` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for direction-related
//! types that represent musical directions, dynamics, tempo, and other
//! performance instructions:
//!
//! - Direction (main container)
//! - DirectionType and DirectionTypeContent
//! - Dynamics, DynamicElement
//! - Wedge, WedgeType
//! - Metronome, MetronomeContent, PerMinute
//! - Words, Rehearsal, Segno, Coda
//! - Pedal, PedalType, Dashes, Bracket
//! - OctaveShift, Offset, Sound
//! - Various percussion types

use crate::ir::common::{FormattedText, Position, PrintStyle};
use crate::ir::direction::{
    Accord, AccordionRegistration, Beater, Bracket, Coda, Dashes, Direction, DirectionType,
    DirectionTypeContent, DynamicElement, Dynamics, Effect, EmptyPrintStyle, FormattedSymbol,
    Glass, HarpPedals, Image, LineEnd, Membrane, Metal, Metronome, MetronomeContent, MetronomeNote,
    MetronomeTuplet, OctaveShift, Offset, OnOff, OtherDirection, Pedal, PedalTuning, PedalType,
    PerMinute, Percussion, PercussionContent, Pitched, PrincipalVoice, PrincipalVoiceSymbol,
    Scordatura, Segno, Sound, StaffDivide, StaffDivideSymbol, Stick, StickLocation, StringMute,
    UpDownStopContinue, Wedge, WedgeType, Wood, Words,
};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, get_head, optional_kwarg, require_kwarg};

// ============================================================================
// WedgeType
// ============================================================================

impl ToSexpr for WedgeType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            WedgeType::Crescendo => "crescendo",
            WedgeType::Diminuendo => "diminuendo",
            WedgeType::Stop => "stop",
            WedgeType::Continue => "continue",
        })
    }
}

impl FromSexpr for WedgeType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("crescendo") => Ok(WedgeType::Crescendo),
            Some("diminuendo") | Some("decrescendo") => Ok(WedgeType::Diminuendo),
            Some("stop") => Ok(WedgeType::Stop),
            Some("continue") => Ok(WedgeType::Continue),
            _ => Err(ConvertError::type_mismatch("wedge-type", sexpr)),
        }
    }
}

// ============================================================================
// PedalType
// ============================================================================

impl ToSexpr for PedalType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            PedalType::Start => "start",
            PedalType::Stop => "stop",
            PedalType::Sostenuto => "sostenuto",
            PedalType::Change => "change",
            PedalType::Continue => "continue",
            PedalType::Discontinue => "discontinue",
            PedalType::Resume => "resume",
        })
    }
}

impl FromSexpr for PedalType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(PedalType::Start),
            Some("stop") => Ok(PedalType::Stop),
            Some("sostenuto") => Ok(PedalType::Sostenuto),
            Some("change") => Ok(PedalType::Change),
            Some("continue") => Ok(PedalType::Continue),
            Some("discontinue") => Ok(PedalType::Discontinue),
            Some("resume") => Ok(PedalType::Resume),
            _ => Err(ConvertError::type_mismatch("pedal-type", sexpr)),
        }
    }
}

// ============================================================================
// LineEnd
// ============================================================================

impl ToSexpr for LineEnd {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            LineEnd::Up => "up",
            LineEnd::Down => "down",
            LineEnd::Both => "both",
            LineEnd::Arrow => "arrow",
            LineEnd::None => "none",
        })
    }
}

impl FromSexpr for LineEnd {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("up") => Ok(LineEnd::Up),
            Some("down") => Ok(LineEnd::Down),
            Some("both") => Ok(LineEnd::Both),
            Some("arrow") => Ok(LineEnd::Arrow),
            Some("none") => Ok(LineEnd::None),
            _ => Err(ConvertError::type_mismatch("line-end", sexpr)),
        }
    }
}

// ============================================================================
// OnOff
// ============================================================================

impl ToSexpr for OnOff {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            OnOff::On => "on",
            OnOff::Off => "off",
        })
    }
}

impl FromSexpr for OnOff {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("on") => Ok(OnOff::On),
            Some("off") => Ok(OnOff::Off),
            _ => Err(ConvertError::type_mismatch("on-off", sexpr)),
        }
    }
}

// ============================================================================
// UpDownStopContinue
// ============================================================================

impl ToSexpr for UpDownStopContinue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            UpDownStopContinue::Up => "up",
            UpDownStopContinue::Down => "down",
            UpDownStopContinue::Stop => "stop",
            UpDownStopContinue::Continue => "continue",
        })
    }
}

impl FromSexpr for UpDownStopContinue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("up") => Ok(UpDownStopContinue::Up),
            Some("down") => Ok(UpDownStopContinue::Down),
            Some("stop") => Ok(UpDownStopContinue::Stop),
            Some("continue") => Ok(UpDownStopContinue::Continue),
            _ => Err(ConvertError::type_mismatch("up-down-stop-continue", sexpr)),
        }
    }
}

// ============================================================================
// StaffDivideSymbol
// ============================================================================

impl ToSexpr for StaffDivideSymbol {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StaffDivideSymbol::Down => "down",
            StaffDivideSymbol::Up => "up",
            StaffDivideSymbol::UpDown => "up-down",
        })
    }
}

impl FromSexpr for StaffDivideSymbol {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("down") => Ok(StaffDivideSymbol::Down),
            Some("up") => Ok(StaffDivideSymbol::Up),
            Some("up-down") => Ok(StaffDivideSymbol::UpDown),
            _ => Err(ConvertError::type_mismatch("staff-divide-symbol", sexpr)),
        }
    }
}

// ============================================================================
// PrincipalVoiceSymbol
// ============================================================================

impl ToSexpr for PrincipalVoiceSymbol {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            PrincipalVoiceSymbol::Hauptstimme => "hauptstimme",
            PrincipalVoiceSymbol::Nebenstimme => "nebenstimme",
            PrincipalVoiceSymbol::Plain => "plain",
            PrincipalVoiceSymbol::None => "none",
        })
    }
}

impl FromSexpr for PrincipalVoiceSymbol {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("hauptstimme") => Ok(PrincipalVoiceSymbol::Hauptstimme),
            Some("nebenstimme") => Ok(PrincipalVoiceSymbol::Nebenstimme),
            Some("plain") => Ok(PrincipalVoiceSymbol::Plain),
            Some("none") => Ok(PrincipalVoiceSymbol::None),
            _ => Err(ConvertError::type_mismatch("principal-voice-symbol", sexpr)),
        }
    }
}

// ============================================================================
// DynamicElement
// ============================================================================

impl ToSexpr for DynamicElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            DynamicElement::P => Sexpr::symbol("p"),
            DynamicElement::PP => Sexpr::symbol("pp"),
            DynamicElement::PPP => Sexpr::symbol("ppp"),
            DynamicElement::PPPP => Sexpr::symbol("pppp"),
            DynamicElement::PPPPP => Sexpr::symbol("ppppp"),
            DynamicElement::PPPPPP => Sexpr::symbol("pppppp"),
            DynamicElement::F => Sexpr::symbol("f"),
            DynamicElement::FF => Sexpr::symbol("ff"),
            DynamicElement::FFF => Sexpr::symbol("fff"),
            DynamicElement::FFFF => Sexpr::symbol("ffff"),
            DynamicElement::FFFFF => Sexpr::symbol("fffff"),
            DynamicElement::FFFFFF => Sexpr::symbol("ffffff"),
            DynamicElement::MP => Sexpr::symbol("mp"),
            DynamicElement::MF => Sexpr::symbol("mf"),
            DynamicElement::SF => Sexpr::symbol("sf"),
            DynamicElement::SFP => Sexpr::symbol("sfp"),
            DynamicElement::SFPP => Sexpr::symbol("sfpp"),
            DynamicElement::FP => Sexpr::symbol("fp"),
            DynamicElement::RF => Sexpr::symbol("rf"),
            DynamicElement::RFZ => Sexpr::symbol("rfz"),
            DynamicElement::SFZ => Sexpr::symbol("sfz"),
            DynamicElement::SFFZ => Sexpr::symbol("sffz"),
            DynamicElement::FZ => Sexpr::symbol("fz"),
            DynamicElement::N => Sexpr::symbol("n"),
            DynamicElement::PF => Sexpr::symbol("pf"),
            DynamicElement::SFZP => Sexpr::symbol("sfzp"),
            DynamicElement::OtherDynamics(s) => {
                ListBuilder::new("other-dynamics").kwarg("value", s).build()
            }
        }
    }
}

impl FromSexpr for DynamicElement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        if let Some(sym) = sexpr.as_symbol() {
            return match sym {
                "p" => Ok(DynamicElement::P),
                "pp" => Ok(DynamicElement::PP),
                "ppp" => Ok(DynamicElement::PPP),
                "pppp" => Ok(DynamicElement::PPPP),
                "ppppp" => Ok(DynamicElement::PPPPP),
                "pppppp" => Ok(DynamicElement::PPPPPP),
                "f" => Ok(DynamicElement::F),
                "ff" => Ok(DynamicElement::FF),
                "fff" => Ok(DynamicElement::FFF),
                "ffff" => Ok(DynamicElement::FFFF),
                "fffff" => Ok(DynamicElement::FFFFF),
                "ffffff" => Ok(DynamicElement::FFFFFF),
                "mp" => Ok(DynamicElement::MP),
                "mf" => Ok(DynamicElement::MF),
                "sf" => Ok(DynamicElement::SF),
                "sfp" => Ok(DynamicElement::SFP),
                "sfpp" => Ok(DynamicElement::SFPP),
                "fp" => Ok(DynamicElement::FP),
                "rf" => Ok(DynamicElement::RF),
                "rfz" => Ok(DynamicElement::RFZ),
                "sfz" => Ok(DynamicElement::SFZ),
                "sffz" => Ok(DynamicElement::SFFZ),
                "fz" => Ok(DynamicElement::FZ),
                "n" => Ok(DynamicElement::N),
                "pf" => Ok(DynamicElement::PF),
                "sfzp" => Ok(DynamicElement::SFZP),
                _ => Err(ConvertError::type_mismatch("dynamic-element symbol", sexpr)),
            };
        }

        // Check for other-dynamics list form
        if let Some(list) = sexpr.as_list() {
            if list.first().is_some_and(|h| h.is_symbol("other-dynamics")) {
                let value: String = require_kwarg(list, "value")?;
                return Ok(DynamicElement::OtherDynamics(value));
            }
        }

        Err(ConvertError::type_mismatch("dynamic-element", sexpr))
    }
}

// ============================================================================
// Dynamics
// ============================================================================

impl ToSexpr for Dynamics {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("dynamics");

        // Content (list of dynamic elements)
        if !self.content.is_empty() {
            builder = builder.kwarg_list("content", &self.content);
        }

        // Print style (inline flattening)
        let ps = &self.print_style;
        let pos = &ps.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = ps.color {
            builder = builder.kwarg("color", color);
        }

        builder = builder.kwarg_opt("placement", &self.placement);

        builder.build()
    }
}

impl FromSexpr for Dynamics {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("dynamics list", sexpr))?;

        expect_head(list, "dynamics")?;

        let content = optional_kwarg::<Vec<DynamicElement>>(list, "content")?.unwrap_or_default();

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        let print_style = PrintStyle {
            position,
            font: Default::default(),
            color,
        };

        Ok(Dynamics {
            content,
            print_style,
            placement: optional_kwarg(list, "placement")?,
        })
    }
}

// ============================================================================
// Wedge
// ============================================================================

impl ToSexpr for Wedge {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("wedge")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("spread", &self.spread)
            .kwarg_opt("niente", &self.niente)
            .kwarg_opt("line-type", &self.line_type);

        // Position
        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for Wedge {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("wedge list", sexpr))?;

        expect_head(list, "wedge")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Wedge {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            spread: optional_kwarg(list, "spread")?,
            niente: optional_kwarg(list, "niente")?,
            line_type: optional_kwarg(list, "line-type")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// Dashes
// ============================================================================

impl ToSexpr for Dashes {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("dashes")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number);

        // Position
        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for Dashes {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("dashes list", sexpr))?;

        expect_head(list, "dashes")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Dashes {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// Bracket
// ============================================================================

impl ToSexpr for Bracket {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("bracket")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg("line-end", &self.line_end)
            .kwarg_opt("end-length", &self.end_length)
            .kwarg_opt("line-type", &self.line_type);

        // Position
        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for Bracket {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("bracket list", sexpr))?;

        expect_head(list, "bracket")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Bracket {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            line_end: require_kwarg(list, "line-end")?,
            end_length: optional_kwarg(list, "end-length")?,
            line_type: optional_kwarg(list, "line-type")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// Pedal
// ============================================================================

impl ToSexpr for Pedal {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("pedal")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("line", &self.line)
            .kwarg_opt("sign", &self.sign)
            .kwarg_opt("abbreviated", &self.abbreviated);

        // Print style (inline)
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = self.print_style.color {
            builder = builder.kwarg("color", color);
        }

        builder.build()
    }
}

impl FromSexpr for Pedal {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pedal list", sexpr))?;

        expect_head(list, "pedal")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(Pedal {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            line: optional_kwarg(list, "line")?,
            sign: optional_kwarg(list, "sign")?,
            abbreviated: optional_kwarg(list, "abbreviated")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

// ============================================================================
// PerMinute
// ============================================================================

impl ToSexpr for PerMinute {
    fn to_sexpr(&self) -> Sexpr {
        // Check if font has any content
        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();

        let mut builder = ListBuilder::new("per-minute").kwarg("value", &self.value);

        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for PerMinute {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("per-minute list", sexpr))?;

        expect_head(list, "per-minute")?;

        use crate::ir::common::Font;

        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(PerMinute {
            value: require_kwarg(list, "value")?,
            font,
        })
    }
}

// ============================================================================
// MetronomeNote
// ============================================================================

impl ToSexpr for MetronomeNote {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("metronome-note")
            .kwarg("type", &self.note_type)
            .kwarg("dots", &self.dots);

        if let Some(ref tuplet) = self.tuplet {
            builder = builder.kwarg_raw("tuplet", tuplet.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for MetronomeNote {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metronome-note list", sexpr))?;

        expect_head(list, "metronome-note")?;

        Ok(MetronomeNote {
            note_type: require_kwarg(list, "type")?,
            dots: require_kwarg(list, "dots")?,
            tuplet: optional_kwarg(list, "tuplet")?,
        })
    }
}

// ============================================================================
// MetronomeTuplet
// ============================================================================

impl ToSexpr for MetronomeTuplet {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("metronome-tuplet")
            .kwarg("type", &self.r#type)
            .kwarg("actual-notes", &self.actual_notes)
            .kwarg("normal-notes", &self.normal_notes)
            .build()
    }
}

impl FromSexpr for MetronomeTuplet {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metronome-tuplet list", sexpr))?;

        expect_head(list, "metronome-tuplet")?;

        Ok(MetronomeTuplet {
            r#type: require_kwarg(list, "type")?,
            actual_notes: require_kwarg(list, "actual-notes")?,
            normal_notes: require_kwarg(list, "normal-notes")?,
        })
    }
}

// ============================================================================
// MetronomeContent
// ============================================================================

impl ToSexpr for MetronomeContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            MetronomeContent::PerMinute {
                beat_unit,
                beat_unit_dots,
                per_minute,
            } => ListBuilder::new("per-minute-content")
                .kwarg("beat-unit", beat_unit)
                .kwarg("beat-unit-dots", beat_unit_dots)
                .kwarg_raw("per-minute", per_minute.to_sexpr())
                .build(),
            MetronomeContent::BeatEquation {
                left_unit,
                left_dots,
                right_unit,
                right_dots,
            } => ListBuilder::new("beat-equation")
                .kwarg("left-unit", left_unit)
                .kwarg("left-dots", left_dots)
                .kwarg("right-unit", right_unit)
                .kwarg("right-dots", right_dots)
                .build(),
            MetronomeContent::MetricModulation { metric_relation } => {
                ListBuilder::new("metric-modulation")
                    .kwarg_list("metric-relation", metric_relation)
                    .build()
            }
        }
    }
}

impl FromSexpr for MetronomeContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metronome-content", sexpr))?;

        match get_head(list)? {
            "per-minute-content" => Ok(MetronomeContent::PerMinute {
                beat_unit: require_kwarg(list, "beat-unit")?,
                beat_unit_dots: require_kwarg(list, "beat-unit-dots")?,
                per_minute: require_kwarg(list, "per-minute")?,
            }),
            "beat-equation" => Ok(MetronomeContent::BeatEquation {
                left_unit: require_kwarg(list, "left-unit")?,
                left_dots: require_kwarg(list, "left-dots")?,
                right_unit: require_kwarg(list, "right-unit")?,
                right_dots: require_kwarg(list, "right-dots")?,
            }),
            "metric-modulation" => Ok(MetronomeContent::MetricModulation {
                metric_relation: optional_kwarg(list, "metric-relation")?.unwrap_or_default(),
            }),
            _ => Err(ConvertError::type_mismatch(
                "metronome-content variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// MetricRelation (stub - needed for MetricModulation)
// ============================================================================

use crate::ir::direction::MetricRelation;

impl ToSexpr for MetricRelation {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("metric-relation")
            .kwarg_raw("left", self.left.to_sexpr())
            .kwarg_raw("right", self.right.to_sexpr())
            .build()
    }
}

impl FromSexpr for MetricRelation {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metric-relation list", sexpr))?;

        expect_head(list, "metric-relation")?;

        Ok(MetricRelation {
            left: require_kwarg(list, "left")?,
            right: require_kwarg(list, "right")?,
        })
    }
}

// ============================================================================
// Metronome
// ============================================================================

impl ToSexpr for Metronome {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("metronome")
            .kwarg_raw("content", self.content.to_sexpr())
            .kwarg_opt("parentheses", &self.parentheses);

        // Print style (simplified)
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for Metronome {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metronome list", sexpr))?;

        expect_head(list, "metronome")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Metronome {
            content: require_kwarg(list, "content")?,
            parentheses: optional_kwarg(list, "parentheses")?,
            print_style: PrintStyle {
                position,
                ..Default::default()
            },
        })
    }
}

// ============================================================================
// Words
// ============================================================================

impl ToSexpr for Words {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("words").kwarg("value", &self.value);

        // Print style components
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = self.print_style.color {
            builder = builder.kwarg("color", color);
        }

        builder = builder
            .kwarg_opt("justify", &self.justify)
            .kwarg_opt("lang", &self.lang);

        builder.build()
    }
}

impl FromSexpr for Words {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("words list", sexpr))?;

        expect_head(list, "words")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(Words {
            value: require_kwarg(list, "value")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
            justify: optional_kwarg(list, "justify")?,
            lang: optional_kwarg(list, "lang")?,
        })
    }
}

// ============================================================================
// FormattedSymbol
// ============================================================================

impl ToSexpr for FormattedSymbol {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("formatted-symbol").kwarg("value", &self.value);

        // Print style
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = self.print_style.color {
            builder = builder.kwarg("color", color);
        }

        builder = builder.kwarg_opt("justify", &self.justify);

        builder.build()
    }
}

impl FromSexpr for FormattedSymbol {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("formatted-symbol list", sexpr))?;

        expect_head(list, "formatted-symbol")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(FormattedSymbol {
            value: require_kwarg(list, "value")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
            justify: optional_kwarg(list, "justify")?,
        })
    }
}

// ============================================================================
// Segno
// ============================================================================

impl ToSexpr for Segno {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("segno");

        // Print style
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = self.print_style.color {
            builder = builder.kwarg("color", color);
        }

        builder = builder.kwarg_opt("smufl", &self.smufl);

        builder.build()
    }
}

impl FromSexpr for Segno {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("segno list", sexpr))?;

        expect_head(list, "segno")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(Segno {
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
            smufl: optional_kwarg(list, "smufl")?,
        })
    }
}

// ============================================================================
// Coda
// ============================================================================

impl ToSexpr for Coda {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("coda");

        // Print style
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = self.print_style.color {
            builder = builder.kwarg("color", color);
        }

        builder = builder.kwarg_opt("smufl", &self.smufl);

        builder.build()
    }
}

impl FromSexpr for Coda {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("coda list", sexpr))?;

        expect_head(list, "coda")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(Coda {
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
            smufl: optional_kwarg(list, "smufl")?,
        })
    }
}

// ============================================================================
// OctaveShift
// ============================================================================

impl ToSexpr for OctaveShift {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("octave-shift")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("size", &self.size);

        // Position
        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for OctaveShift {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("octave-shift list", sexpr))?;

        expect_head(list, "octave-shift")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(OctaveShift {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            size: optional_kwarg(list, "size")?,
            position,
        })
    }
}

// ============================================================================
// Offset
// ============================================================================

impl ToSexpr for Offset {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("offset")
            .kwarg("value", &self.value)
            .kwarg_opt("sound", &self.sound)
            .build()
    }
}

impl FromSexpr for Offset {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("offset list", sexpr))?;

        expect_head(list, "offset")?;

        Ok(Offset {
            value: require_kwarg(list, "value")?,
            sound: optional_kwarg(list, "sound")?,
        })
    }
}

// ============================================================================
// Sound
// ============================================================================

impl ToSexpr for Sound {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("sound")
            .kwarg_opt("tempo", &self.tempo)
            .kwarg_opt("dynamics", &self.dynamics)
            .kwarg_opt("dacapo", &self.dacapo)
            .kwarg_opt("segno", &self.segno)
            .kwarg_opt("dalsegno", &self.dalsegno)
            .kwarg_opt("coda", &self.coda)
            .kwarg_opt("tocoda", &self.tocoda)
            .kwarg_opt("divisions", &self.divisions)
            .kwarg_opt("forward-repeat", &self.forward_repeat)
            .kwarg_opt("fine", &self.fine)
            .kwarg_opt("time-only", &self.time_only)
            .kwarg_opt("pizzicato", &self.pizzicato)
            .build()
    }
}

impl FromSexpr for Sound {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("sound list", sexpr))?;

        expect_head(list, "sound")?;

        Ok(Sound {
            tempo: optional_kwarg(list, "tempo")?,
            dynamics: optional_kwarg(list, "dynamics")?,
            dacapo: optional_kwarg(list, "dacapo")?,
            segno: optional_kwarg(list, "segno")?,
            dalsegno: optional_kwarg(list, "dalsegno")?,
            coda: optional_kwarg(list, "coda")?,
            tocoda: optional_kwarg(list, "tocoda")?,
            divisions: optional_kwarg(list, "divisions")?,
            forward_repeat: optional_kwarg(list, "forward-repeat")?,
            fine: optional_kwarg(list, "fine")?,
            time_only: optional_kwarg(list, "time-only")?,
            pizzicato: optional_kwarg(list, "pizzicato")?,
        })
    }
}

// ============================================================================
// EmptyPrintStyle
// ============================================================================

impl ToSexpr for EmptyPrintStyle {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("empty-print-style");

        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for EmptyPrintStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("empty-print-style list", sexpr))?;

        expect_head(list, "empty-print-style")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(EmptyPrintStyle {
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color: None,
            },
        })
    }
}

// ============================================================================
// HarpPedals and PedalTuning
// ============================================================================

impl ToSexpr for PedalTuning {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("pedal-tuning")
            .kwarg("step", &self.pedal_step)
            .kwarg("alter", &self.pedal_alter)
            .build()
    }
}

impl FromSexpr for PedalTuning {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pedal-tuning list", sexpr))?;

        expect_head(list, "pedal-tuning")?;

        Ok(PedalTuning {
            pedal_step: require_kwarg(list, "step")?,
            pedal_alter: require_kwarg(list, "alter")?,
        })
    }
}

impl ToSexpr for HarpPedals {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("harp-pedals")
            .kwarg_list("tuning", &self.pedal_tuning)
            .build()
    }
}

impl FromSexpr for HarpPedals {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("harp-pedals list", sexpr))?;

        expect_head(list, "harp-pedals")?;

        Ok(HarpPedals {
            pedal_tuning: optional_kwarg::<Vec<PedalTuning>>(list, "tuning")?.unwrap_or_default(),
        })
    }
}

// ============================================================================
// StringMute
// ============================================================================

impl ToSexpr for StringMute {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("string-mute")
            .kwarg("type", &self.r#type)
            .build()
    }
}

impl FromSexpr for StringMute {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("string-mute list", sexpr))?;

        expect_head(list, "string-mute")?;

        Ok(StringMute {
            r#type: require_kwarg(list, "type")?,
        })
    }
}

// ============================================================================
// Scordatura and Accord
// ============================================================================

impl ToSexpr for Accord {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("accord")
            .kwarg("string", &self.string)
            .kwarg("tuning-step", &self.tuning_step)
            .kwarg_opt("tuning-alter", &self.tuning_alter)
            .kwarg("tuning-octave", &self.tuning_octave)
            .build()
    }
}

impl FromSexpr for Accord {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("accord list", sexpr))?;

        expect_head(list, "accord")?;

        Ok(Accord {
            string: require_kwarg(list, "string")?,
            tuning_step: require_kwarg(list, "tuning-step")?,
            tuning_alter: optional_kwarg(list, "tuning-alter")?,
            tuning_octave: require_kwarg(list, "tuning-octave")?,
        })
    }
}

impl ToSexpr for Scordatura {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("scordatura")
            .kwarg_list("accord", &self.accord)
            .build()
    }
}

impl FromSexpr for Scordatura {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("scordatura list", sexpr))?;

        expect_head(list, "scordatura")?;

        Ok(Scordatura {
            accord: optional_kwarg::<Vec<Accord>>(list, "accord")?.unwrap_or_default(),
        })
    }
}

// ============================================================================
// Image
// ============================================================================

impl ToSexpr for Image {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("image")
            .kwarg("source", &self.source)
            .kwarg("type", &self.r#type);

        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for Image {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("image list", sexpr))?;

        expect_head(list, "image")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Image {
            source: require_kwarg(list, "source")?,
            r#type: require_kwarg(list, "type")?,
            position,
        })
    }
}

// ============================================================================
// PrincipalVoice
// ============================================================================

impl ToSexpr for PrincipalVoice {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("principal-voice")
            .kwarg("type", &self.r#type)
            .kwarg("symbol", &self.symbol)
            .build()
    }
}

impl FromSexpr for PrincipalVoice {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("principal-voice list", sexpr))?;

        expect_head(list, "principal-voice")?;

        Ok(PrincipalVoice {
            r#type: require_kwarg(list, "type")?,
            symbol: require_kwarg(list, "symbol")?,
        })
    }
}

// ============================================================================
// AccordionRegistration
// ============================================================================

impl ToSexpr for AccordionRegistration {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("accordion-registration")
            .kwarg("high", &self.accordion_high)
            .kwarg_opt("middle", &self.accordion_middle)
            .kwarg("low", &self.accordion_low)
            .build()
    }
}

impl FromSexpr for AccordionRegistration {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("accordion-registration list", sexpr))?;

        expect_head(list, "accordion-registration")?;

        Ok(AccordionRegistration {
            accordion_high: optional_kwarg(list, "high")?.unwrap_or(false),
            accordion_middle: optional_kwarg(list, "middle")?,
            accordion_low: optional_kwarg(list, "low")?.unwrap_or(false),
        })
    }
}

// ============================================================================
// StaffDivide
// ============================================================================

impl ToSexpr for StaffDivide {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("staff-divide")
            .kwarg("type", &self.r#type)
            .build()
    }
}

impl FromSexpr for StaffDivide {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("staff-divide list", sexpr))?;

        expect_head(list, "staff-divide")?;

        Ok(StaffDivide {
            r#type: require_kwarg(list, "type")?,
        })
    }
}

// ============================================================================
// OtherDirection
// ============================================================================

impl ToSexpr for OtherDirection {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("other-direction")
            .kwarg("value", &self.value)
            .kwarg_opt("print-object", &self.print_object);

        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for OtherDirection {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("other-direction list", sexpr))?;

        expect_head(list, "other-direction")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(OtherDirection {
            value: require_kwarg(list, "value")?,
            print_object: optional_kwarg(list, "print-object")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color: None,
            },
        })
    }
}

// ============================================================================
// Percussion Types (simplified implementations - IR uses simple value: String)
// ============================================================================

impl ToSexpr for Glass {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("glass")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Glass {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("glass list", sexpr))?;

        expect_head(list, "glass")?;

        Ok(Glass {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Metal {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("metal")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Metal {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metal list", sexpr))?;

        expect_head(list, "metal")?;

        Ok(Metal {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Wood {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("wood").kwarg("value", &self.value).build()
    }
}

impl FromSexpr for Wood {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("wood list", sexpr))?;

        expect_head(list, "wood")?;

        Ok(Wood {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Pitched {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("pitched")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Pitched {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pitched list", sexpr))?;

        expect_head(list, "pitched")?;

        Ok(Pitched {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Membrane {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("membrane")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Membrane {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("membrane list", sexpr))?;

        expect_head(list, "membrane")?;

        Ok(Membrane {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Effect {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("effect")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Effect {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("effect list", sexpr))?;

        expect_head(list, "effect")?;

        Ok(Effect {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Beater {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("beater")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Beater {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("beater list", sexpr))?;

        expect_head(list, "beater")?;

        Ok(Beater {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for Stick {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("stick")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Stick {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("stick list", sexpr))?;

        expect_head(list, "stick")?;

        Ok(Stick {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for StickLocation {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("stick-location")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for StickLocation {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("stick-location list", sexpr))?;

        expect_head(list, "stick-location")?;

        Ok(StickLocation {
            value: require_kwarg(list, "value")?,
        })
    }
}

// ============================================================================
// PercussionContent
// ============================================================================

impl ToSexpr for PercussionContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            PercussionContent::Glass(g) => g.to_sexpr(),
            PercussionContent::Metal(m) => m.to_sexpr(),
            PercussionContent::Wood(w) => w.to_sexpr(),
            PercussionContent::Pitched(p) => p.to_sexpr(),
            PercussionContent::Membrane(m) => m.to_sexpr(),
            PercussionContent::Effect(e) => e.to_sexpr(),
            PercussionContent::Timpani => ListBuilder::new("timpani").build(),
            PercussionContent::Beater(b) => b.to_sexpr(),
            PercussionContent::Stick(s) => s.to_sexpr(),
            PercussionContent::StickLocation(sl) => sl.to_sexpr(),
            PercussionContent::OtherPercussion(s) => ListBuilder::new("other-percussion")
                .kwarg("value", s)
                .build(),
        }
    }
}

impl FromSexpr for PercussionContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("percussion-content", sexpr))?;

        match get_head(list)? {
            "glass" => Ok(PercussionContent::Glass(Glass::from_sexpr(sexpr)?)),
            "metal" => Ok(PercussionContent::Metal(Metal::from_sexpr(sexpr)?)),
            "wood" => Ok(PercussionContent::Wood(Wood::from_sexpr(sexpr)?)),
            "pitched" => Ok(PercussionContent::Pitched(Pitched::from_sexpr(sexpr)?)),
            "membrane" => Ok(PercussionContent::Membrane(Membrane::from_sexpr(sexpr)?)),
            "effect" => Ok(PercussionContent::Effect(Effect::from_sexpr(sexpr)?)),
            "timpani" => Ok(PercussionContent::Timpani),
            "beater" => Ok(PercussionContent::Beater(Beater::from_sexpr(sexpr)?)),
            "stick" => Ok(PercussionContent::Stick(Stick::from_sexpr(sexpr)?)),
            "stick-location" => Ok(PercussionContent::StickLocation(StickLocation::from_sexpr(
                sexpr,
            )?)),
            "other-percussion" => {
                let value: String = require_kwarg(list, "value")?;
                Ok(PercussionContent::OtherPercussion(value))
            }
            _ => Err(ConvertError::type_mismatch(
                "percussion-content variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Percussion
// ============================================================================

impl ToSexpr for Percussion {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("percussion")
            .kwarg_raw("content", self.content.to_sexpr())
            .build()
    }
}

impl FromSexpr for Percussion {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("percussion list", sexpr))?;

        expect_head(list, "percussion")?;

        Ok(Percussion {
            content: require_kwarg(list, "content")?,
        })
    }
}

// ============================================================================
// DirectionTypeContent
// ============================================================================

impl ToSexpr for DirectionTypeContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            DirectionTypeContent::Rehearsal(texts) => ListBuilder::new("rehearsal")
                .kwarg_list("texts", texts)
                .build(),
            DirectionTypeContent::Segno(segnos) => ListBuilder::new("segnos")
                .kwarg_list("items", segnos)
                .build(),
            DirectionTypeContent::Coda(codas) => {
                ListBuilder::new("codas").kwarg_list("items", codas).build()
            }
            DirectionTypeContent::Words(words) => ListBuilder::new("words-list")
                .kwarg_list("items", words)
                .build(),
            DirectionTypeContent::Symbol(symbols) => ListBuilder::new("symbols")
                .kwarg_list("items", symbols)
                .build(),
            DirectionTypeContent::Dynamics(d) => d.to_sexpr(),
            DirectionTypeContent::Wedge(w) => w.to_sexpr(),
            DirectionTypeContent::Dashes(d) => d.to_sexpr(),
            DirectionTypeContent::Bracket(b) => b.to_sexpr(),
            DirectionTypeContent::Pedal(p) => p.to_sexpr(),
            DirectionTypeContent::Metronome(m) => m.to_sexpr(),
            DirectionTypeContent::OctaveShift(o) => o.to_sexpr(),
            DirectionTypeContent::HarpPedals(h) => h.to_sexpr(),
            DirectionTypeContent::Damp(e) => {
                let mut builder = ListBuilder::new("damp");
                let pos = &e.print_style.position;
                if pos.default_x.is_some()
                    || pos.default_y.is_some()
                    || pos.relative_x.is_some()
                    || pos.relative_y.is_some()
                {
                    builder = builder.kwarg_raw("position", pos.to_sexpr());
                }
                builder.build()
            }
            DirectionTypeContent::DampAll(e) => {
                let mut builder = ListBuilder::new("damp-all");
                let pos = &e.print_style.position;
                if pos.default_x.is_some()
                    || pos.default_y.is_some()
                    || pos.relative_x.is_some()
                    || pos.relative_y.is_some()
                {
                    builder = builder.kwarg_raw("position", pos.to_sexpr());
                }
                builder.build()
            }
            DirectionTypeContent::Eyeglasses(e) => {
                let mut builder = ListBuilder::new("eyeglasses");
                let pos = &e.print_style.position;
                if pos.default_x.is_some()
                    || pos.default_y.is_some()
                    || pos.relative_x.is_some()
                    || pos.relative_y.is_some()
                {
                    builder = builder.kwarg_raw("position", pos.to_sexpr());
                }
                builder.build()
            }
            DirectionTypeContent::StringMute(s) => s.to_sexpr(),
            DirectionTypeContent::Scordatura(s) => s.to_sexpr(),
            DirectionTypeContent::Image(i) => i.to_sexpr(),
            DirectionTypeContent::PrincipalVoice(p) => p.to_sexpr(),
            DirectionTypeContent::Percussion(percs) => ListBuilder::new("percussion-list")
                .kwarg_list("items", percs)
                .build(),
            DirectionTypeContent::AccordionRegistration(a) => a.to_sexpr(),
            DirectionTypeContent::StaffDivide(s) => s.to_sexpr(),
            DirectionTypeContent::OtherDirection(o) => o.to_sexpr(),
        }
    }
}

impl FromSexpr for DirectionTypeContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("direction-type-content", sexpr))?;

        match get_head(list)? {
            "rehearsal" => {
                let texts =
                    optional_kwarg::<Vec<FormattedText>>(list, "texts")?.unwrap_or_default();
                Ok(DirectionTypeContent::Rehearsal(texts))
            }
            "segnos" => {
                let items = optional_kwarg::<Vec<Segno>>(list, "items")?.unwrap_or_default();
                Ok(DirectionTypeContent::Segno(items))
            }
            "codas" => {
                let items = optional_kwarg::<Vec<Coda>>(list, "items")?.unwrap_or_default();
                Ok(DirectionTypeContent::Coda(items))
            }
            "words-list" => {
                let items = optional_kwarg::<Vec<Words>>(list, "items")?.unwrap_or_default();
                Ok(DirectionTypeContent::Words(items))
            }
            "symbols" => {
                let items =
                    optional_kwarg::<Vec<FormattedSymbol>>(list, "items")?.unwrap_or_default();
                Ok(DirectionTypeContent::Symbol(items))
            }
            "dynamics" => Ok(DirectionTypeContent::Dynamics(Dynamics::from_sexpr(sexpr)?)),
            "wedge" => Ok(DirectionTypeContent::Wedge(Wedge::from_sexpr(sexpr)?)),
            "dashes" => Ok(DirectionTypeContent::Dashes(Dashes::from_sexpr(sexpr)?)),
            "bracket" => Ok(DirectionTypeContent::Bracket(Bracket::from_sexpr(sexpr)?)),
            "pedal" => Ok(DirectionTypeContent::Pedal(Pedal::from_sexpr(sexpr)?)),
            "metronome" => Ok(DirectionTypeContent::Metronome(Metronome::from_sexpr(
                sexpr,
            )?)),
            "octave-shift" => Ok(DirectionTypeContent::OctaveShift(OctaveShift::from_sexpr(
                sexpr,
            )?)),
            "harp-pedals" => Ok(DirectionTypeContent::HarpPedals(HarpPedals::from_sexpr(
                sexpr,
            )?)),
            "damp" => {
                let position = match find_kwarg(list, "position") {
                    Some(ps) => Position::from_sexpr(ps)?,
                    None => Position::default(),
                };
                Ok(DirectionTypeContent::Damp(EmptyPrintStyle {
                    print_style: PrintStyle {
                        position,
                        ..Default::default()
                    },
                }))
            }
            "damp-all" => {
                let position = match find_kwarg(list, "position") {
                    Some(ps) => Position::from_sexpr(ps)?,
                    None => Position::default(),
                };
                Ok(DirectionTypeContent::DampAll(EmptyPrintStyle {
                    print_style: PrintStyle {
                        position,
                        ..Default::default()
                    },
                }))
            }
            "eyeglasses" => {
                let position = match find_kwarg(list, "position") {
                    Some(ps) => Position::from_sexpr(ps)?,
                    None => Position::default(),
                };
                Ok(DirectionTypeContent::Eyeglasses(EmptyPrintStyle {
                    print_style: PrintStyle {
                        position,
                        ..Default::default()
                    },
                }))
            }
            "string-mute" => Ok(DirectionTypeContent::StringMute(StringMute::from_sexpr(
                sexpr,
            )?)),
            "scordatura" => Ok(DirectionTypeContent::Scordatura(Scordatura::from_sexpr(
                sexpr,
            )?)),
            "image" => Ok(DirectionTypeContent::Image(Image::from_sexpr(sexpr)?)),
            "principal-voice" => Ok(DirectionTypeContent::PrincipalVoice(
                PrincipalVoice::from_sexpr(sexpr)?,
            )),
            "percussion-list" => {
                let items = optional_kwarg::<Vec<Percussion>>(list, "items")?.unwrap_or_default();
                Ok(DirectionTypeContent::Percussion(items))
            }
            "accordion-registration" => Ok(DirectionTypeContent::AccordionRegistration(
                AccordionRegistration::from_sexpr(sexpr)?,
            )),
            "staff-divide" => Ok(DirectionTypeContent::StaffDivide(StaffDivide::from_sexpr(
                sexpr,
            )?)),
            "other-direction" => Ok(DirectionTypeContent::OtherDirection(
                OtherDirection::from_sexpr(sexpr)?,
            )),
            _ => Err(ConvertError::type_mismatch(
                "direction-type-content variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// DirectionType
// ============================================================================

impl ToSexpr for DirectionType {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("direction-type")
            .kwarg_raw("content", self.content.to_sexpr())
            .build()
    }
}

impl FromSexpr for DirectionType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("direction-type list", sexpr))?;

        expect_head(list, "direction-type")?;

        Ok(DirectionType {
            content: require_kwarg(list, "content")?,
        })
    }
}

// ============================================================================
// Direction
// ============================================================================

impl ToSexpr for Direction {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("direction");

        // Direction types
        if !self.direction_types.is_empty() {
            builder = builder.kwarg_list("types", &self.direction_types);
        }

        // Offset
        if let Some(ref offset) = self.offset {
            builder = builder.kwarg_raw("offset", offset.to_sexpr());
        }

        // Sound
        if let Some(ref sound) = self.sound {
            builder = builder.kwarg_raw("sound", sound.to_sexpr());
        }

        // Other attributes
        builder = builder
            .kwarg_opt("staff", &self.staff)
            .kwarg_opt("voice", &self.voice)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("directive", &self.directive);

        builder.build()
    }
}

impl FromSexpr for Direction {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("direction list", sexpr))?;

        expect_head(list, "direction")?;

        Ok(Direction {
            direction_types: optional_kwarg::<Vec<DirectionType>>(list, "types")?
                .unwrap_or_default(),
            offset: optional_kwarg(list, "offset")?,
            sound: optional_kwarg(list, "sound")?,
            staff: optional_kwarg(list, "staff")?,
            voice: optional_kwarg(list, "voice")?,
            placement: optional_kwarg(list, "placement")?,
            directive: optional_kwarg(list, "directive")?,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{AboveBelow, Font, LeftCenterRight, LineType, StartStop, YesNo};
    use crate::ir::duration::NoteTypeValue;
    use crate::ir::pitch::Step;
    use crate::sexpr::print_sexpr;

    // ========================================================================
    // WedgeType Tests
    // ========================================================================

    #[test]
    fn test_wedge_type_crescendo_round_trip() {
        let wt = WedgeType::Crescendo;
        let sexpr = wt.to_sexpr();
        let parsed = WedgeType::from_sexpr(&sexpr).unwrap();
        assert_eq!(wt, parsed);
    }

    #[test]
    fn test_wedge_type_diminuendo_round_trip() {
        let wt = WedgeType::Diminuendo;
        let sexpr = wt.to_sexpr();
        let parsed = WedgeType::from_sexpr(&sexpr).unwrap();
        assert_eq!(wt, parsed);
    }

    #[test]
    fn test_wedge_type_decrescendo_alias() {
        // "decrescendo" should be parsed as Diminuendo
        let sexpr = Sexpr::symbol("decrescendo");
        let parsed = WedgeType::from_sexpr(&sexpr).unwrap();
        assert_eq!(WedgeType::Diminuendo, parsed);
    }

    #[test]
    fn test_wedge_type_stop_round_trip() {
        let wt = WedgeType::Stop;
        let sexpr = wt.to_sexpr();
        let parsed = WedgeType::from_sexpr(&sexpr).unwrap();
        assert_eq!(wt, parsed);
    }

    #[test]
    fn test_wedge_type_continue_round_trip() {
        let wt = WedgeType::Continue;
        let sexpr = wt.to_sexpr();
        let parsed = WedgeType::from_sexpr(&sexpr).unwrap();
        assert_eq!(wt, parsed);
    }

    #[test]
    fn test_wedge_type_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = WedgeType::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // PedalType Tests
    // ========================================================================

    #[test]
    fn test_pedal_type_all_variants_round_trip() {
        let variants = [
            PedalType::Start,
            PedalType::Stop,
            PedalType::Sostenuto,
            PedalType::Change,
            PedalType::Continue,
            PedalType::Discontinue,
            PedalType::Resume,
        ];
        for pt in variants {
            let sexpr = pt.to_sexpr();
            let parsed = PedalType::from_sexpr(&sexpr).unwrap();
            assert_eq!(pt, parsed);
        }
    }

    #[test]
    fn test_pedal_type_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = PedalType::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // LineEnd Tests
    // ========================================================================

    #[test]
    fn test_line_end_all_variants_round_trip() {
        let variants = [
            LineEnd::Up,
            LineEnd::Down,
            LineEnd::Both,
            LineEnd::Arrow,
            LineEnd::None,
        ];
        for le in variants {
            let sexpr = le.to_sexpr();
            let parsed = LineEnd::from_sexpr(&sexpr).unwrap();
            assert_eq!(le, parsed);
        }
    }

    #[test]
    fn test_line_end_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = LineEnd::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // OnOff Tests
    // ========================================================================

    #[test]
    fn test_on_off_on_round_trip() {
        let oo = OnOff::On;
        let sexpr = oo.to_sexpr();
        let parsed = OnOff::from_sexpr(&sexpr).unwrap();
        assert_eq!(oo, parsed);
    }

    #[test]
    fn test_on_off_off_round_trip() {
        let oo = OnOff::Off;
        let sexpr = oo.to_sexpr();
        let parsed = OnOff::from_sexpr(&sexpr).unwrap();
        assert_eq!(oo, parsed);
    }

    #[test]
    fn test_on_off_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = OnOff::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // UpDownStopContinue Tests
    // ========================================================================

    #[test]
    fn test_up_down_stop_continue_all_variants_round_trip() {
        let variants = [
            UpDownStopContinue::Up,
            UpDownStopContinue::Down,
            UpDownStopContinue::Stop,
            UpDownStopContinue::Continue,
        ];
        for udsc in variants {
            let sexpr = udsc.to_sexpr();
            let parsed = UpDownStopContinue::from_sexpr(&sexpr).unwrap();
            assert_eq!(udsc, parsed);
        }
    }

    #[test]
    fn test_up_down_stop_continue_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = UpDownStopContinue::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // StaffDivideSymbol Tests
    // ========================================================================

    #[test]
    fn test_staff_divide_symbol_all_variants_round_trip() {
        let variants = [
            StaffDivideSymbol::Down,
            StaffDivideSymbol::Up,
            StaffDivideSymbol::UpDown,
        ];
        for sds in variants {
            let sexpr = sds.to_sexpr();
            let parsed = StaffDivideSymbol::from_sexpr(&sexpr).unwrap();
            assert_eq!(sds, parsed);
        }
    }

    #[test]
    fn test_staff_divide_symbol_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = StaffDivideSymbol::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // PrincipalVoiceSymbol Tests
    // ========================================================================

    #[test]
    fn test_principal_voice_symbol_all_variants_round_trip() {
        let variants = [
            PrincipalVoiceSymbol::Hauptstimme,
            PrincipalVoiceSymbol::Nebenstimme,
            PrincipalVoiceSymbol::Plain,
            PrincipalVoiceSymbol::None,
        ];
        for pvs in variants {
            let sexpr = pvs.to_sexpr();
            let parsed = PrincipalVoiceSymbol::from_sexpr(&sexpr).unwrap();
            assert_eq!(pvs, parsed);
        }
    }

    #[test]
    fn test_principal_voice_symbol_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        let result = PrincipalVoiceSymbol::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // DynamicElement Tests - All Variants
    // ========================================================================

    #[test]
    fn test_dynamic_element_all_piano_variants() {
        let variants = [
            DynamicElement::P,
            DynamicElement::PP,
            DynamicElement::PPP,
            DynamicElement::PPPP,
            DynamicElement::PPPPP,
            DynamicElement::PPPPPP,
        ];
        for elem in variants {
            let sexpr = elem.to_sexpr();
            let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
            assert_eq!(elem, parsed);
        }
    }

    #[test]
    fn test_dynamic_element_all_forte_variants() {
        let variants = [
            DynamicElement::F,
            DynamicElement::FF,
            DynamicElement::FFF,
            DynamicElement::FFFF,
            DynamicElement::FFFFF,
            DynamicElement::FFFFFF,
        ];
        for elem in variants {
            let sexpr = elem.to_sexpr();
            let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
            assert_eq!(elem, parsed);
        }
    }

    #[test]
    fn test_dynamic_element_mezzo_variants() {
        let variants = [DynamicElement::MP, DynamicElement::MF];
        for elem in variants {
            let sexpr = elem.to_sexpr();
            let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
            assert_eq!(elem, parsed);
        }
    }

    #[test]
    fn test_dynamic_element_sforzando_variants() {
        let variants = [
            DynamicElement::SF,
            DynamicElement::SFP,
            DynamicElement::SFPP,
            DynamicElement::SFZ,
            DynamicElement::SFFZ,
            DynamicElement::SFZP,
        ];
        for elem in variants {
            let sexpr = elem.to_sexpr();
            let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
            assert_eq!(elem, parsed);
        }
    }

    #[test]
    fn test_dynamic_element_other_variants() {
        let variants = [
            DynamicElement::FP,
            DynamicElement::RF,
            DynamicElement::RFZ,
            DynamicElement::FZ,
            DynamicElement::N,
            DynamicElement::PF,
        ];
        for elem in variants {
            let sexpr = elem.to_sexpr();
            let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
            assert_eq!(elem, parsed);
        }
    }

    #[test]
    fn test_dynamic_element_other_dynamics_round_trip() {
        let elem = DynamicElement::OtherDynamics("custom-dynamic".to_string());
        let sexpr = elem.to_sexpr();
        let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
        assert_eq!(elem, parsed);
    }

    #[test]
    fn test_dynamic_element_invalid_symbol() {
        let sexpr = Sexpr::symbol("invalid-dynamic");
        let result = DynamicElement::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_dynamic_element_not_a_list_or_symbol() {
        let sexpr = Sexpr::Integer(42);
        let result = DynamicElement::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Dynamics Tests
    // ========================================================================

    #[test]
    fn test_dynamics_empty_content() {
        let dynamics = Dynamics {
            content: vec![],
            print_style: PrintStyle::default(),
            placement: None,
        };
        let sexpr = dynamics.to_sexpr();
        let parsed = Dynamics::from_sexpr(&sexpr).unwrap();
        assert_eq!(dynamics.content, parsed.content);
    }

    #[test]
    fn test_dynamics_with_position() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::F],
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#FF0000".to_string()),
            },
            placement: Some(AboveBelow::Below),
        };
        let sexpr = dynamics.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        assert!(text.contains("color"));
        let parsed = Dynamics::from_sexpr(&sexpr).unwrap();
        assert_eq!(dynamics.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_dynamics_not_a_list() {
        let sexpr = Sexpr::symbol("dynamics");
        let result = Dynamics::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_dynamics_wrong_head() {
        let sexpr = ListBuilder::new("wrong").build();
        let result = Dynamics::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Wedge Tests
    // ========================================================================

    #[test]
    fn test_wedge_crescendo_full() {
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: Some(1),
            spread: Some(15.0),
            niente: Some(YesNo::Yes),
            line_type: Some(LineType::Dashed),
            position: Position {
                default_x: Some(5.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(-10.0),
            },
            color: Some("#000000".to_string()),
        };
        let sexpr = wedge.to_sexpr();
        let parsed = Wedge::from_sexpr(&sexpr).unwrap();
        assert_eq!(wedge.r#type, parsed.r#type);
        assert_eq!(wedge.number, parsed.number);
        assert_eq!(wedge.spread, parsed.spread);
        assert_eq!(wedge.niente, parsed.niente);
        assert_eq!(wedge.line_type, parsed.line_type);
        assert_eq!(wedge.color, parsed.color);
    }

    #[test]
    fn test_wedge_diminuendo_minimal() {
        let wedge = Wedge {
            r#type: WedgeType::Diminuendo,
            number: None,
            spread: None,
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };
        let sexpr = wedge.to_sexpr();
        let parsed = Wedge::from_sexpr(&sexpr).unwrap();
        assert_eq!(wedge.r#type, parsed.r#type);
    }

    #[test]
    fn test_wedge_not_a_list() {
        let sexpr = Sexpr::symbol("wedge");
        let result = Wedge::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_wedge_wrong_head() {
        let sexpr = ListBuilder::new("wrong")
            .kwarg("type", &WedgeType::Crescendo)
            .build();
        let result = Wedge::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Dashes Tests
    // ========================================================================

    #[test]
    fn test_dashes_round_trip() {
        use crate::ir::common::StartStopContinue;
        let dashes = Dashes {
            r#type: StartStopContinue::Start,
            number: Some(1),
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: None,
                relative_y: None,
            },
            color: Some("#333333".to_string()),
        };
        let sexpr = dashes.to_sexpr();
        let parsed = Dashes::from_sexpr(&sexpr).unwrap();
        assert_eq!(dashes.r#type, parsed.r#type);
        assert_eq!(dashes.number, parsed.number);
        assert_eq!(dashes.color, parsed.color);
    }

    #[test]
    fn test_dashes_minimal() {
        use crate::ir::common::StartStopContinue;
        let dashes = Dashes {
            r#type: StartStopContinue::Stop,
            number: None,
            position: Position::default(),
            color: None,
        };
        let sexpr = dashes.to_sexpr();
        let parsed = Dashes::from_sexpr(&sexpr).unwrap();
        assert_eq!(dashes.r#type, parsed.r#type);
    }

    #[test]
    fn test_dashes_not_a_list() {
        let sexpr = Sexpr::symbol("dashes");
        let result = Dashes::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Bracket Tests
    // ========================================================================

    #[test]
    fn test_bracket_round_trip() {
        use crate::ir::common::StartStopContinue;
        let bracket = Bracket {
            r#type: StartStopContinue::Start,
            number: Some(1),
            line_end: LineEnd::Up,
            end_length: Some(10.0),
            line_type: Some(LineType::Solid),
            position: Position {
                default_x: Some(5.0),
                default_y: None,
                relative_x: Some(2.0),
                relative_y: None,
            },
            color: Some("#444444".to_string()),
        };
        let sexpr = bracket.to_sexpr();
        let parsed = Bracket::from_sexpr(&sexpr).unwrap();
        assert_eq!(bracket.r#type, parsed.r#type);
        assert_eq!(bracket.line_end, parsed.line_end);
        assert_eq!(bracket.end_length, parsed.end_length);
        assert_eq!(bracket.line_type, parsed.line_type);
        assert_eq!(bracket.color, parsed.color);
    }

    #[test]
    fn test_bracket_not_a_list() {
        let sexpr = Sexpr::symbol("bracket");
        let result = Bracket::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Pedal Tests
    // ========================================================================

    #[test]
    fn test_pedal_all_types() {
        let types = [
            PedalType::Start,
            PedalType::Stop,
            PedalType::Sostenuto,
            PedalType::Change,
            PedalType::Continue,
            PedalType::Discontinue,
            PedalType::Resume,
        ];
        for pt in types {
            let pedal = Pedal {
                r#type: pt,
                number: Some(1),
                line: Some(YesNo::Yes),
                sign: Some(YesNo::No),
                abbreviated: Some(YesNo::Yes),
                print_style: PrintStyle::default(),
            };
            let sexpr = pedal.to_sexpr();
            let parsed = Pedal::from_sexpr(&sexpr).unwrap();
            assert_eq!(pedal.r#type, parsed.r#type);
        }
    }

    #[test]
    fn test_pedal_with_position_and_color() {
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#555555".to_string()),
            },
        };
        let sexpr = pedal.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        assert!(text.contains("color"));
        let parsed = Pedal::from_sexpr(&sexpr).unwrap();
        assert_eq!(pedal.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_pedal_not_a_list() {
        let sexpr = Sexpr::symbol("pedal");
        let result = Pedal::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // PerMinute Tests
    // ========================================================================

    #[test]
    fn test_per_minute_minimal() {
        let pm = PerMinute {
            value: "120".to_string(),
            font: Font::default(),
        };
        let sexpr = pm.to_sexpr();
        let parsed = PerMinute::from_sexpr(&sexpr).unwrap();
        assert_eq!(pm.value, parsed.value);
    }

    #[test]
    fn test_per_minute_with_font() {
        let pm = PerMinute {
            value: "120-132".to_string(),
            font: Font {
                font_family: Some("Times".to_string()),
                font_style: None,
                font_size: None,
                font_weight: None,
            },
        };
        let sexpr = pm.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("font"));
        let parsed = PerMinute::from_sexpr(&sexpr).unwrap();
        assert_eq!(pm.value, parsed.value);
    }

    #[test]
    fn test_per_minute_not_a_list() {
        let sexpr = Sexpr::symbol("per-minute");
        let result = PerMinute::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // MetronomeNote Tests
    // ========================================================================

    #[test]
    fn test_metronome_note_round_trip() {
        let mn = MetronomeNote {
            note_type: NoteTypeValue::Quarter,
            dots: 1,
            tuplet: None,
        };
        let sexpr = mn.to_sexpr();
        let parsed = MetronomeNote::from_sexpr(&sexpr).unwrap();
        assert_eq!(mn.note_type, parsed.note_type);
        assert_eq!(mn.dots, parsed.dots);
    }

    #[test]
    fn test_metronome_note_with_tuplet() {
        let mn = MetronomeNote {
            note_type: NoteTypeValue::Eighth,
            dots: 0,
            tuplet: Some(MetronomeTuplet {
                r#type: StartStop::Start,
                actual_notes: 3,
                normal_notes: 2,
            }),
        };
        let sexpr = mn.to_sexpr();
        let parsed = MetronomeNote::from_sexpr(&sexpr).unwrap();
        assert_eq!(mn.tuplet.as_ref().unwrap().actual_notes, 3);
        assert_eq!(parsed.tuplet.as_ref().unwrap().actual_notes, 3);
    }

    #[test]
    fn test_metronome_note_not_a_list() {
        let sexpr = Sexpr::symbol("metronome-note");
        let result = MetronomeNote::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // MetronomeTuplet Tests
    // ========================================================================

    #[test]
    fn test_metronome_tuplet_round_trip() {
        let mt = MetronomeTuplet {
            r#type: StartStop::Start,
            actual_notes: 3,
            normal_notes: 2,
        };
        let sexpr = mt.to_sexpr();
        let parsed = MetronomeTuplet::from_sexpr(&sexpr).unwrap();
        assert_eq!(mt.r#type, parsed.r#type);
        assert_eq!(mt.actual_notes, parsed.actual_notes);
        assert_eq!(mt.normal_notes, parsed.normal_notes);
    }

    #[test]
    fn test_metronome_tuplet_not_a_list() {
        let sexpr = Sexpr::symbol("metronome-tuplet");
        let result = MetronomeTuplet::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // MetronomeContent Tests
    // ========================================================================

    #[test]
    fn test_metronome_content_per_minute() {
        let mc = MetronomeContent::PerMinute {
            beat_unit: NoteTypeValue::Quarter,
            beat_unit_dots: 0,
            per_minute: PerMinute {
                value: "120".to_string(),
                font: Font::default(),
            },
        };
        let sexpr = mc.to_sexpr();
        let parsed = MetronomeContent::from_sexpr(&sexpr).unwrap();
        if let MetronomeContent::PerMinute { beat_unit, .. } = parsed {
            assert_eq!(beat_unit, NoteTypeValue::Quarter);
        } else {
            panic!("Expected PerMinute variant");
        }
    }

    #[test]
    fn test_metronome_content_beat_equation() {
        let mc = MetronomeContent::BeatEquation {
            left_unit: NoteTypeValue::Half,
            left_dots: 1,
            right_unit: NoteTypeValue::Quarter,
            right_dots: 0,
        };
        let sexpr = mc.to_sexpr();
        let parsed = MetronomeContent::from_sexpr(&sexpr).unwrap();
        if let MetronomeContent::BeatEquation {
            left_unit,
            right_unit,
            ..
        } = parsed
        {
            assert_eq!(left_unit, NoteTypeValue::Half);
            assert_eq!(right_unit, NoteTypeValue::Quarter);
        } else {
            panic!("Expected BeatEquation variant");
        }
    }

    #[test]
    fn test_metronome_content_metric_modulation() {
        let mc = MetronomeContent::MetricModulation {
            metric_relation: vec![MetricRelation {
                left: MetronomeNote {
                    note_type: NoteTypeValue::Quarter,
                    dots: 0,
                    tuplet: None,
                },
                right: MetronomeNote {
                    note_type: NoteTypeValue::Half,
                    dots: 0,
                    tuplet: None,
                },
            }],
        };
        let sexpr = mc.to_sexpr();
        let parsed = MetronomeContent::from_sexpr(&sexpr).unwrap();
        if let MetronomeContent::MetricModulation { metric_relation } = parsed {
            assert_eq!(metric_relation.len(), 1);
        } else {
            panic!("Expected MetricModulation variant");
        }
    }

    #[test]
    fn test_metronome_content_not_a_list() {
        let sexpr = Sexpr::symbol("per-minute-content");
        let result = MetronomeContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_metronome_content_invalid_variant() {
        let sexpr = ListBuilder::new("invalid-variant").build();
        let result = MetronomeContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // MetricRelation Tests
    // ========================================================================

    #[test]
    fn test_metric_relation_round_trip() {
        let mr = MetricRelation {
            left: MetronomeNote {
                note_type: NoteTypeValue::Quarter,
                dots: 0,
                tuplet: None,
            },
            right: MetronomeNote {
                note_type: NoteTypeValue::Half,
                dots: 1,
                tuplet: None,
            },
        };
        let sexpr = mr.to_sexpr();
        let parsed = MetricRelation::from_sexpr(&sexpr).unwrap();
        assert_eq!(mr.left.note_type, parsed.left.note_type);
        assert_eq!(mr.right.dots, parsed.right.dots);
    }

    #[test]
    fn test_metric_relation_not_a_list() {
        let sexpr = Sexpr::symbol("metric-relation");
        let result = MetricRelation::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Metronome Tests
    // ========================================================================

    #[test]
    fn test_metronome_per_minute_round_trip() {
        let metronome = Metronome {
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Quarter,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "120".to_string(),
                    font: Default::default(),
                },
            },
            parentheses: Some(YesNo::Yes),
            print_style: PrintStyle::default(),
        };
        let sexpr = metronome.to_sexpr();
        let parsed = Metronome::from_sexpr(&sexpr).unwrap();
        assert_eq!(metronome.parentheses, parsed.parentheses);
    }

    #[test]
    fn test_metronome_with_position() {
        let metronome = Metronome {
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Half,
                beat_unit_dots: 1,
                per_minute: PerMinute {
                    value: "60".to_string(),
                    font: Default::default(),
                },
            },
            parentheses: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        };
        let sexpr = metronome.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        let parsed = Metronome::from_sexpr(&sexpr).unwrap();
        assert_eq!(
            metronome.print_style.position.default_x,
            parsed.print_style.position.default_x
        );
    }

    #[test]
    fn test_metronome_not_a_list() {
        let sexpr = Sexpr::symbol("metronome");
        let result = Metronome::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Words Tests
    // ========================================================================

    #[test]
    fn test_words_full() {
        let words = Words {
            value: "dolce".to_string(),
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(5.0),
                    default_y: Some(10.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#666666".to_string()),
            },
            justify: Some(LeftCenterRight::Center),
            lang: Some("it".to_string()),
        };
        let sexpr = words.to_sexpr();
        let parsed = Words::from_sexpr(&sexpr).unwrap();
        assert_eq!(words.value, parsed.value);
        assert_eq!(words.justify, parsed.justify);
        assert_eq!(words.lang, parsed.lang);
        assert_eq!(words.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_words_minimal() {
        let words = Words {
            value: "cresc.".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            lang: None,
        };
        let sexpr = words.to_sexpr();
        let parsed = Words::from_sexpr(&sexpr).unwrap();
        assert_eq!(words.value, parsed.value);
    }

    #[test]
    fn test_words_not_a_list() {
        let sexpr = Sexpr::symbol("words");
        let result = Words::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // FormattedSymbol Tests
    // ========================================================================

    #[test]
    fn test_formatted_symbol_round_trip() {
        let fs = FormattedSymbol {
            value: "segno".to_string(),
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(5.0),
                    default_y: None,
                    relative_x: None,
                    relative_y: Some(10.0),
                },
                font: Default::default(),
                color: Some("#777777".to_string()),
            },
            justify: Some(LeftCenterRight::Right),
        };
        let sexpr = fs.to_sexpr();
        let parsed = FormattedSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(fs.value, parsed.value);
        assert_eq!(fs.justify, parsed.justify);
        assert_eq!(fs.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_formatted_symbol_minimal() {
        let fs = FormattedSymbol {
            value: "coda".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
        };
        let sexpr = fs.to_sexpr();
        let parsed = FormattedSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(fs.value, parsed.value);
    }

    #[test]
    fn test_formatted_symbol_not_a_list() {
        let sexpr = Sexpr::symbol("formatted-symbol");
        let result = FormattedSymbol::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Segno Tests
    // ========================================================================

    #[test]
    fn test_segno_with_all_fields() {
        let segno = Segno {
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#888888".to_string()),
            },
            smufl: Some("segno".to_string()),
        };
        let sexpr = segno.to_sexpr();
        let parsed = Segno::from_sexpr(&sexpr).unwrap();
        assert_eq!(segno.smufl, parsed.smufl);
        assert_eq!(segno.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_segno_minimal() {
        let segno = Segno {
            print_style: PrintStyle::default(),
            smufl: None,
        };
        let sexpr = segno.to_sexpr();
        let parsed = Segno::from_sexpr(&sexpr).unwrap();
        assert!(parsed.smufl.is_none());
    }

    #[test]
    fn test_segno_not_a_list() {
        let sexpr = Sexpr::symbol("segno");
        let result = Segno::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Coda Tests
    // ========================================================================

    #[test]
    fn test_coda_with_all_fields() {
        let coda = Coda {
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(15.0),
                    default_y: Some(25.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#999999".to_string()),
            },
            smufl: Some("coda".to_string()),
        };
        let sexpr = coda.to_sexpr();
        let parsed = Coda::from_sexpr(&sexpr).unwrap();
        assert_eq!(coda.smufl, parsed.smufl);
        assert_eq!(coda.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_coda_minimal() {
        let coda = Coda {
            print_style: PrintStyle::default(),
            smufl: None,
        };
        let sexpr = coda.to_sexpr();
        let parsed = Coda::from_sexpr(&sexpr).unwrap();
        assert!(parsed.smufl.is_none());
    }

    #[test]
    fn test_coda_not_a_list() {
        let sexpr = Sexpr::symbol("coda");
        let result = Coda::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // OctaveShift Tests
    // ========================================================================

    #[test]
    fn test_octave_shift_all_types() {
        let types = [
            UpDownStopContinue::Up,
            UpDownStopContinue::Down,
            UpDownStopContinue::Stop,
            UpDownStopContinue::Continue,
        ];
        for t in types {
            let os = OctaveShift {
                r#type: t,
                number: Some(1),
                size: Some(8),
                position: Position::default(),
            };
            let sexpr = os.to_sexpr();
            let parsed = OctaveShift::from_sexpr(&sexpr).unwrap();
            assert_eq!(os.r#type, parsed.r#type);
        }
    }

    #[test]
    fn test_octave_shift_with_position() {
        let os = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(15),
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: None,
                relative_y: None,
            },
        };
        let sexpr = os.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        let parsed = OctaveShift::from_sexpr(&sexpr).unwrap();
        assert_eq!(os.size, parsed.size);
    }

    #[test]
    fn test_octave_shift_not_a_list() {
        let sexpr = Sexpr::symbol("octave-shift");
        let result = OctaveShift::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Offset Tests
    // ========================================================================

    #[test]
    fn test_offset_round_trip() {
        let offset = Offset {
            value: 10,
            sound: Some(YesNo::Yes),
        };
        let sexpr = offset.to_sexpr();
        let parsed = Offset::from_sexpr(&sexpr).unwrap();
        assert_eq!(offset.value, parsed.value);
        assert_eq!(offset.sound, parsed.sound);
    }

    #[test]
    fn test_offset_minimal() {
        let offset = Offset {
            value: -5,
            sound: None,
        };
        let sexpr = offset.to_sexpr();
        let parsed = Offset::from_sexpr(&sexpr).unwrap();
        assert_eq!(offset.value, parsed.value);
        assert!(parsed.sound.is_none());
    }

    #[test]
    fn test_offset_not_a_list() {
        let sexpr = Sexpr::symbol("offset");
        let result = Offset::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Sound Tests
    // ========================================================================

    #[test]
    fn test_sound_all_fields() {
        let sound = Sound {
            tempo: Some(120.0),
            dynamics: Some(80.0),
            dacapo: Some(YesNo::Yes),
            segno: Some("segno1".to_string()),
            dalsegno: Some("dalsegno1".to_string()),
            coda: Some("coda1".to_string()),
            tocoda: Some("tocoda1".to_string()),
            divisions: Some(4),
            forward_repeat: Some(YesNo::Yes),
            fine: Some("fine".to_string()),
            time_only: Some("1".to_string()),
            pizzicato: Some(YesNo::Yes),
        };
        let sexpr = sound.to_sexpr();
        let parsed = Sound::from_sexpr(&sexpr).unwrap();
        assert_eq!(sound.tempo, parsed.tempo);
        assert_eq!(sound.dynamics, parsed.dynamics);
        assert_eq!(sound.dacapo, parsed.dacapo);
        assert_eq!(sound.segno, parsed.segno);
        assert_eq!(sound.dalsegno, parsed.dalsegno);
        assert_eq!(sound.coda, parsed.coda);
        assert_eq!(sound.tocoda, parsed.tocoda);
        assert_eq!(sound.divisions, parsed.divisions);
        assert_eq!(sound.forward_repeat, parsed.forward_repeat);
        assert_eq!(sound.fine, parsed.fine);
        assert_eq!(sound.time_only, parsed.time_only);
        assert_eq!(sound.pizzicato, parsed.pizzicato);
    }

    #[test]
    fn test_sound_empty() {
        let sound = Sound::default();
        let sexpr = sound.to_sexpr();
        let parsed = Sound::from_sexpr(&sexpr).unwrap();
        assert!(parsed.tempo.is_none());
        assert!(parsed.dynamics.is_none());
    }

    #[test]
    fn test_sound_not_a_list() {
        let sexpr = Sexpr::symbol("sound");
        let result = Sound::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // EmptyPrintStyle Tests
    // ========================================================================

    #[test]
    fn test_empty_print_style_with_position() {
        let eps = EmptyPrintStyle {
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(5.0),
                    default_y: Some(10.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        };
        let sexpr = eps.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        let parsed = EmptyPrintStyle::from_sexpr(&sexpr).unwrap();
        assert_eq!(
            eps.print_style.position.default_x,
            parsed.print_style.position.default_x
        );
    }

    #[test]
    fn test_empty_print_style_default() {
        let eps = EmptyPrintStyle::default();
        let sexpr = eps.to_sexpr();
        let parsed = EmptyPrintStyle::from_sexpr(&sexpr).unwrap();
        assert!(parsed.print_style.position.default_x.is_none());
    }

    #[test]
    fn test_empty_print_style_not_a_list() {
        let sexpr = Sexpr::symbol("empty-print-style");
        let result = EmptyPrintStyle::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // HarpPedals and PedalTuning Tests
    // ========================================================================

    #[test]
    fn test_pedal_tuning_round_trip() {
        let pt = PedalTuning {
            pedal_step: Step::D,
            pedal_alter: 1.0,
        };
        let sexpr = pt.to_sexpr();
        let parsed = PedalTuning::from_sexpr(&sexpr).unwrap();
        assert_eq!(pt.pedal_step, parsed.pedal_step);
        assert_eq!(pt.pedal_alter, parsed.pedal_alter);
    }

    #[test]
    fn test_pedal_tuning_not_a_list() {
        let sexpr = Sexpr::symbol("pedal-tuning");
        let result = PedalTuning::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_harp_pedals_round_trip() {
        let hp = HarpPedals {
            pedal_tuning: vec![
                PedalTuning {
                    pedal_step: Step::D,
                    pedal_alter: 0.0,
                },
                PedalTuning {
                    pedal_step: Step::C,
                    pedal_alter: 1.0,
                },
            ],
        };
        let sexpr = hp.to_sexpr();
        let parsed = HarpPedals::from_sexpr(&sexpr).unwrap();
        assert_eq!(hp.pedal_tuning.len(), parsed.pedal_tuning.len());
    }

    #[test]
    fn test_harp_pedals_empty() {
        let hp = HarpPedals {
            pedal_tuning: vec![],
        };
        let sexpr = hp.to_sexpr();
        let parsed = HarpPedals::from_sexpr(&sexpr).unwrap();
        assert!(parsed.pedal_tuning.is_empty());
    }

    #[test]
    fn test_harp_pedals_not_a_list() {
        let sexpr = Sexpr::symbol("harp-pedals");
        let result = HarpPedals::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // StringMute Tests
    // ========================================================================

    #[test]
    fn test_string_mute_on() {
        let sm = StringMute { r#type: OnOff::On };
        let sexpr = sm.to_sexpr();
        let parsed = StringMute::from_sexpr(&sexpr).unwrap();
        assert_eq!(sm.r#type, parsed.r#type);
    }

    #[test]
    fn test_string_mute_off() {
        let sm = StringMute { r#type: OnOff::Off };
        let sexpr = sm.to_sexpr();
        let parsed = StringMute::from_sexpr(&sexpr).unwrap();
        assert_eq!(sm.r#type, parsed.r#type);
    }

    #[test]
    fn test_string_mute_not_a_list() {
        let sexpr = Sexpr::symbol("string-mute");
        let result = StringMute::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Scordatura and Accord Tests
    // ========================================================================

    #[test]
    fn test_accord_round_trip() {
        let accord = Accord {
            string: 6,
            tuning_step: Step::D,
            tuning_alter: Some(-1.0),
            tuning_octave: 2,
        };
        let sexpr = accord.to_sexpr();
        let parsed = Accord::from_sexpr(&sexpr).unwrap();
        assert_eq!(accord.string, parsed.string);
        assert_eq!(accord.tuning_step, parsed.tuning_step);
        assert_eq!(accord.tuning_alter, parsed.tuning_alter);
        assert_eq!(accord.tuning_octave, parsed.tuning_octave);
    }

    #[test]
    fn test_accord_no_alter() {
        let accord = Accord {
            string: 1,
            tuning_step: Step::E,
            tuning_alter: None,
            tuning_octave: 4,
        };
        let sexpr = accord.to_sexpr();
        let parsed = Accord::from_sexpr(&sexpr).unwrap();
        assert!(parsed.tuning_alter.is_none());
    }

    #[test]
    fn test_accord_not_a_list() {
        let sexpr = Sexpr::symbol("accord");
        let result = Accord::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_scordatura_round_trip() {
        let scordatura = Scordatura {
            accord: vec![
                Accord {
                    string: 6,
                    tuning_step: Step::D,
                    tuning_alter: None,
                    tuning_octave: 2,
                },
                Accord {
                    string: 1,
                    tuning_step: Step::E,
                    tuning_alter: None,
                    tuning_octave: 4,
                },
            ],
        };
        let sexpr = scordatura.to_sexpr();
        let parsed = Scordatura::from_sexpr(&sexpr).unwrap();
        assert_eq!(scordatura.accord.len(), parsed.accord.len());
    }

    #[test]
    fn test_scordatura_empty() {
        let scordatura = Scordatura::default();
        let sexpr = scordatura.to_sexpr();
        let parsed = Scordatura::from_sexpr(&sexpr).unwrap();
        assert!(parsed.accord.is_empty());
    }

    #[test]
    fn test_scordatura_not_a_list() {
        let sexpr = Sexpr::symbol("scordatura");
        let result = Scordatura::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Image Tests
    // ========================================================================

    #[test]
    fn test_image_round_trip() {
        let image = Image {
            source: "image.png".to_string(),
            r#type: "image/png".to_string(),
            position: Position {
                default_x: Some(10.0),
                default_y: Some(20.0),
                relative_x: None,
                relative_y: None,
            },
        };
        let sexpr = image.to_sexpr();
        let parsed = Image::from_sexpr(&sexpr).unwrap();
        assert_eq!(image.source, parsed.source);
        assert_eq!(image.r#type, parsed.r#type);
    }

    #[test]
    fn test_image_no_position() {
        let image = Image {
            source: "test.jpg".to_string(),
            r#type: "image/jpeg".to_string(),
            position: Position::default(),
        };
        let sexpr = image.to_sexpr();
        let parsed = Image::from_sexpr(&sexpr).unwrap();
        assert_eq!(image.source, parsed.source);
    }

    #[test]
    fn test_image_not_a_list() {
        let sexpr = Sexpr::symbol("image");
        let result = Image::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // PrincipalVoice Tests
    // ========================================================================

    #[test]
    fn test_principal_voice_all_symbols() {
        let symbols = [
            PrincipalVoiceSymbol::Hauptstimme,
            PrincipalVoiceSymbol::Nebenstimme,
            PrincipalVoiceSymbol::Plain,
            PrincipalVoiceSymbol::None,
        ];
        for symbol in symbols {
            let pv = PrincipalVoice {
                r#type: StartStop::Start,
                symbol,
            };
            let sexpr = pv.to_sexpr();
            let parsed = PrincipalVoice::from_sexpr(&sexpr).unwrap();
            assert_eq!(pv.r#type, parsed.r#type);
            assert_eq!(pv.symbol, parsed.symbol);
        }
    }

    #[test]
    fn test_principal_voice_not_a_list() {
        let sexpr = Sexpr::symbol("principal-voice");
        let result = PrincipalVoice::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // AccordionRegistration Tests
    // ========================================================================

    #[test]
    fn test_accordion_registration_full() {
        let ar = AccordionRegistration {
            accordion_high: true,
            accordion_middle: Some(2),
            accordion_low: true,
        };
        let sexpr = ar.to_sexpr();
        let parsed = AccordionRegistration::from_sexpr(&sexpr).unwrap();
        assert_eq!(ar.accordion_high, parsed.accordion_high);
        assert_eq!(ar.accordion_middle, parsed.accordion_middle);
        assert_eq!(ar.accordion_low, parsed.accordion_low);
    }

    #[test]
    fn test_accordion_registration_default() {
        let ar = AccordionRegistration::default();
        let sexpr = ar.to_sexpr();
        let parsed = AccordionRegistration::from_sexpr(&sexpr).unwrap();
        assert!(!parsed.accordion_high);
        assert!(parsed.accordion_middle.is_none());
        assert!(!parsed.accordion_low);
    }

    #[test]
    fn test_accordion_registration_not_a_list() {
        let sexpr = Sexpr::symbol("accordion-registration");
        let result = AccordionRegistration::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // StaffDivide Tests
    // ========================================================================

    #[test]
    fn test_staff_divide_all_types() {
        let types = [
            StaffDivideSymbol::Down,
            StaffDivideSymbol::Up,
            StaffDivideSymbol::UpDown,
        ];
        for t in types {
            let sd = StaffDivide { r#type: t };
            let sexpr = sd.to_sexpr();
            let parsed = StaffDivide::from_sexpr(&sexpr).unwrap();
            assert_eq!(sd.r#type, parsed.r#type);
        }
    }

    #[test]
    fn test_staff_divide_not_a_list() {
        let sexpr = Sexpr::symbol("staff-divide");
        let result = StaffDivide::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // OtherDirection Tests
    // ========================================================================

    #[test]
    fn test_other_direction_full() {
        let od = OtherDirection {
            value: "custom-direction".to_string(),
            print_object: Some(YesNo::Yes),
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(5.0),
                    default_y: Some(10.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        };
        let sexpr = od.to_sexpr();
        let parsed = OtherDirection::from_sexpr(&sexpr).unwrap();
        assert_eq!(od.value, parsed.value);
        assert_eq!(od.print_object, parsed.print_object);
    }

    #[test]
    fn test_other_direction_minimal() {
        let od = OtherDirection {
            value: "simple".to_string(),
            print_object: None,
            print_style: PrintStyle::default(),
        };
        let sexpr = od.to_sexpr();
        let parsed = OtherDirection::from_sexpr(&sexpr).unwrap();
        assert_eq!(od.value, parsed.value);
    }

    #[test]
    fn test_other_direction_not_a_list() {
        let sexpr = Sexpr::symbol("other-direction");
        let result = OtherDirection::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Percussion Types Tests
    // ========================================================================

    #[test]
    fn test_glass_round_trip() {
        let g = Glass {
            value: "wind-chimes".to_string(),
        };
        let sexpr = g.to_sexpr();
        let parsed = Glass::from_sexpr(&sexpr).unwrap();
        assert_eq!(g.value, parsed.value);
    }

    #[test]
    fn test_glass_not_a_list() {
        let sexpr = Sexpr::symbol("glass");
        let result = Glass::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_metal_round_trip() {
        let m = Metal {
            value: "triangle".to_string(),
        };
        let sexpr = m.to_sexpr();
        let parsed = Metal::from_sexpr(&sexpr).unwrap();
        assert_eq!(m.value, parsed.value);
    }

    #[test]
    fn test_metal_not_a_list() {
        let sexpr = Sexpr::symbol("metal");
        let result = Metal::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_wood_round_trip() {
        let w = Wood {
            value: "claves".to_string(),
        };
        let sexpr = w.to_sexpr();
        let parsed = Wood::from_sexpr(&sexpr).unwrap();
        assert_eq!(w.value, parsed.value);
    }

    #[test]
    fn test_wood_not_a_list() {
        let sexpr = Sexpr::symbol("wood");
        let result = Wood::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_pitched_round_trip() {
        let p = Pitched {
            value: "vibraphone".to_string(),
        };
        let sexpr = p.to_sexpr();
        let parsed = Pitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(p.value, parsed.value);
    }

    #[test]
    fn test_pitched_not_a_list() {
        let sexpr = Sexpr::symbol("pitched");
        let result = Pitched::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_membrane_round_trip() {
        let m = Membrane {
            value: "snare-drum".to_string(),
        };
        let sexpr = m.to_sexpr();
        let parsed = Membrane::from_sexpr(&sexpr).unwrap();
        assert_eq!(m.value, parsed.value);
    }

    #[test]
    fn test_membrane_not_a_list() {
        let sexpr = Sexpr::symbol("membrane");
        let result = Membrane::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_effect_round_trip() {
        let e = Effect {
            value: "siren".to_string(),
        };
        let sexpr = e.to_sexpr();
        let parsed = Effect::from_sexpr(&sexpr).unwrap();
        assert_eq!(e.value, parsed.value);
    }

    #[test]
    fn test_effect_not_a_list() {
        let sexpr = Sexpr::symbol("effect");
        let result = Effect::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_beater_round_trip() {
        let b = Beater {
            value: "soft".to_string(),
        };
        let sexpr = b.to_sexpr();
        let parsed = Beater::from_sexpr(&sexpr).unwrap();
        assert_eq!(b.value, parsed.value);
    }

    #[test]
    fn test_beater_not_a_list() {
        let sexpr = Sexpr::symbol("beater");
        let result = Beater::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_stick_round_trip() {
        let s = Stick {
            value: "yarn".to_string(),
        };
        let sexpr = s.to_sexpr();
        let parsed = Stick::from_sexpr(&sexpr).unwrap();
        assert_eq!(s.value, parsed.value);
    }

    #[test]
    fn test_stick_not_a_list() {
        let sexpr = Sexpr::symbol("stick");
        let result = Stick::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_stick_location_round_trip() {
        let sl = StickLocation {
            value: "center".to_string(),
        };
        let sexpr = sl.to_sexpr();
        let parsed = StickLocation::from_sexpr(&sexpr).unwrap();
        assert_eq!(sl.value, parsed.value);
    }

    #[test]
    fn test_stick_location_not_a_list() {
        let sexpr = Sexpr::symbol("stick-location");
        let result = StickLocation::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // PercussionContent Tests
    // ========================================================================

    #[test]
    fn test_percussion_content_glass() {
        let pc = PercussionContent::Glass(Glass {
            value: "wind-chimes".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Glass(g) = parsed {
            assert_eq!(g.value, "wind-chimes");
        } else {
            panic!("Expected Glass variant");
        }
    }

    #[test]
    fn test_percussion_content_metal() {
        let pc = PercussionContent::Metal(Metal {
            value: "triangle".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Metal(m) = parsed {
            assert_eq!(m.value, "triangle");
        } else {
            panic!("Expected Metal variant");
        }
    }

    #[test]
    fn test_percussion_content_wood() {
        let pc = PercussionContent::Wood(Wood {
            value: "claves".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Wood(w) = parsed {
            assert_eq!(w.value, "claves");
        } else {
            panic!("Expected Wood variant");
        }
    }

    #[test]
    fn test_percussion_content_pitched() {
        let pc = PercussionContent::Pitched(Pitched {
            value: "marimba".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Pitched(p) = parsed {
            assert_eq!(p.value, "marimba");
        } else {
            panic!("Expected Pitched variant");
        }
    }

    #[test]
    fn test_percussion_content_membrane() {
        let pc = PercussionContent::Membrane(Membrane {
            value: "bass-drum".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Membrane(m) = parsed {
            assert_eq!(m.value, "bass-drum");
        } else {
            panic!("Expected Membrane variant");
        }
    }

    #[test]
    fn test_percussion_content_effect() {
        let pc = PercussionContent::Effect(Effect {
            value: "siren".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Effect(e) = parsed {
            assert_eq!(e.value, "siren");
        } else {
            panic!("Expected Effect variant");
        }
    }

    #[test]
    fn test_percussion_content_timpani() {
        let pc = PercussionContent::Timpani;
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, PercussionContent::Timpani));
    }

    #[test]
    fn test_percussion_content_beater() {
        let pc = PercussionContent::Beater(Beater {
            value: "hard".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Beater(b) = parsed {
            assert_eq!(b.value, "hard");
        } else {
            panic!("Expected Beater variant");
        }
    }

    #[test]
    fn test_percussion_content_stick() {
        let pc = PercussionContent::Stick(Stick {
            value: "felt".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::Stick(s) = parsed {
            assert_eq!(s.value, "felt");
        } else {
            panic!("Expected Stick variant");
        }
    }

    #[test]
    fn test_percussion_content_stick_location() {
        let pc = PercussionContent::StickLocation(StickLocation {
            value: "rim".to_string(),
        });
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::StickLocation(sl) = parsed {
            assert_eq!(sl.value, "rim");
        } else {
            panic!("Expected StickLocation variant");
        }
    }

    #[test]
    fn test_percussion_content_other() {
        let pc = PercussionContent::OtherPercussion("custom".to_string());
        let sexpr = pc.to_sexpr();
        let parsed = PercussionContent::from_sexpr(&sexpr).unwrap();
        if let PercussionContent::OtherPercussion(s) = parsed {
            assert_eq!(s, "custom");
        } else {
            panic!("Expected OtherPercussion variant");
        }
    }

    #[test]
    fn test_percussion_content_not_a_list() {
        let sexpr = Sexpr::symbol("glass");
        let result = PercussionContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_percussion_content_invalid_variant() {
        let sexpr = ListBuilder::new("invalid-percussion").build();
        let result = PercussionContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Percussion Tests
    // ========================================================================

    #[test]
    fn test_percussion_round_trip() {
        let perc = Percussion {
            content: PercussionContent::Timpani,
        };
        let sexpr = perc.to_sexpr();
        let parsed = Percussion::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed.content, PercussionContent::Timpani));
    }

    #[test]
    fn test_percussion_not_a_list() {
        let sexpr = Sexpr::symbol("percussion");
        let result = Percussion::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // DirectionTypeContent Tests
    // ========================================================================

    #[test]
    fn test_direction_type_content_rehearsal() {
        use crate::ir::common::FormattedText;
        let dtc = DirectionTypeContent::Rehearsal(vec![FormattedText {
            value: "A".to_string(),
            print_style: PrintStyle::default(),
            lang: None,
        }]);
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Rehearsal(texts) = parsed {
            assert_eq!(texts.len(), 1);
            assert_eq!(texts[0].value, "A");
        } else {
            panic!("Expected Rehearsal variant");
        }
    }

    #[test]
    fn test_direction_type_content_segno() {
        let dtc = DirectionTypeContent::Segno(vec![Segno {
            print_style: PrintStyle::default(),
            smufl: None,
        }]);
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Segno(segnos) = parsed {
            assert_eq!(segnos.len(), 1);
        } else {
            panic!("Expected Segno variant");
        }
    }

    #[test]
    fn test_direction_type_content_coda() {
        let dtc = DirectionTypeContent::Coda(vec![Coda {
            print_style: PrintStyle::default(),
            smufl: None,
        }]);
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Coda(codas) = parsed {
            assert_eq!(codas.len(), 1);
        } else {
            panic!("Expected Coda variant");
        }
    }

    #[test]
    fn test_direction_type_content_words() {
        let dtc = DirectionTypeContent::Words(vec![Words {
            value: "dolce".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            lang: None,
        }]);
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Words(words) = parsed {
            assert_eq!(words.len(), 1);
            assert_eq!(words[0].value, "dolce");
        } else {
            panic!("Expected Words variant");
        }
    }

    #[test]
    fn test_direction_type_content_symbol() {
        let dtc = DirectionTypeContent::Symbol(vec![FormattedSymbol {
            value: "segno".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
        }]);
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Symbol(symbols) = parsed {
            assert_eq!(symbols.len(), 1);
        } else {
            panic!("Expected Symbol variant");
        }
    }

    #[test]
    fn test_direction_type_content_dynamics() {
        let dtc = DirectionTypeContent::Dynamics(Dynamics {
            content: vec![DynamicElement::F],
            print_style: PrintStyle::default(),
            placement: None,
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Dynamics(_)));
    }

    #[test]
    fn test_direction_type_content_wedge() {
        let dtc = DirectionTypeContent::Wedge(Wedge {
            r#type: WedgeType::Crescendo,
            number: None,
            spread: None,
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Wedge(_)));
    }

    #[test]
    fn test_direction_type_content_dashes() {
        use crate::ir::common::StartStopContinue;
        let dtc = DirectionTypeContent::Dashes(Dashes {
            r#type: StartStopContinue::Start,
            number: None,
            position: Position::default(),
            color: None,
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Dashes(_)));
    }

    #[test]
    fn test_direction_type_content_bracket() {
        use crate::ir::common::StartStopContinue;
        let dtc = DirectionTypeContent::Bracket(Bracket {
            r#type: StartStopContinue::Start,
            number: None,
            line_end: LineEnd::Up,
            end_length: None,
            line_type: None,
            position: Position::default(),
            color: None,
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Bracket(_)));
    }

    #[test]
    fn test_direction_type_content_pedal() {
        let dtc = DirectionTypeContent::Pedal(Pedal {
            r#type: PedalType::Start,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Pedal(_)));
    }

    #[test]
    fn test_direction_type_content_metronome() {
        let dtc = DirectionTypeContent::Metronome(Metronome {
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Quarter,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "120".to_string(),
                    font: Font::default(),
                },
            },
            parentheses: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Metronome(_)));
    }

    #[test]
    fn test_direction_type_content_octave_shift() {
        let dtc = DirectionTypeContent::OctaveShift(OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(8),
            position: Position::default(),
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::OctaveShift(_)));
    }

    #[test]
    fn test_direction_type_content_harp_pedals() {
        let dtc = DirectionTypeContent::HarpPedals(HarpPedals {
            pedal_tuning: vec![],
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::HarpPedals(_)));
    }

    #[test]
    fn test_direction_type_content_damp() {
        let dtc = DirectionTypeContent::Damp(EmptyPrintStyle::default());
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Damp(_)));
    }

    #[test]
    fn test_direction_type_content_damp_with_position() {
        let dtc = DirectionTypeContent::Damp(EmptyPrintStyle {
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Damp(eps) = parsed {
            assert_eq!(eps.print_style.position.default_x, Some(10.0));
        } else {
            panic!("Expected Damp variant");
        }
    }

    #[test]
    fn test_direction_type_content_damp_all() {
        let dtc = DirectionTypeContent::DampAll(EmptyPrintStyle::default());
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::DampAll(_)));
    }

    #[test]
    fn test_direction_type_content_damp_all_with_position() {
        let dtc = DirectionTypeContent::DampAll(EmptyPrintStyle {
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(15.0),
                    default_y: Some(25.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::DampAll(eps) = parsed {
            assert_eq!(eps.print_style.position.default_x, Some(15.0));
        } else {
            panic!("Expected DampAll variant");
        }
    }

    #[test]
    fn test_direction_type_content_eyeglasses() {
        let dtc = DirectionTypeContent::Eyeglasses(EmptyPrintStyle::default());
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Eyeglasses(_)));
    }

    #[test]
    fn test_direction_type_content_eyeglasses_with_position() {
        let dtc = DirectionTypeContent::Eyeglasses(EmptyPrintStyle {
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(20.0),
                    default_y: Some(30.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Eyeglasses(eps) = parsed {
            assert_eq!(eps.print_style.position.default_x, Some(20.0));
        } else {
            panic!("Expected Eyeglasses variant");
        }
    }

    #[test]
    fn test_direction_type_content_string_mute() {
        let dtc = DirectionTypeContent::StringMute(StringMute { r#type: OnOff::On });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::StringMute(_)));
    }

    #[test]
    fn test_direction_type_content_scordatura() {
        let dtc = DirectionTypeContent::Scordatura(Scordatura::default());
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Scordatura(_)));
    }

    #[test]
    fn test_direction_type_content_image() {
        let dtc = DirectionTypeContent::Image(Image {
            source: "test.png".to_string(),
            r#type: "image/png".to_string(),
            position: Position::default(),
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::Image(_)));
    }

    #[test]
    fn test_direction_type_content_principal_voice() {
        let dtc = DirectionTypeContent::PrincipalVoice(PrincipalVoice {
            r#type: StartStop::Start,
            symbol: PrincipalVoiceSymbol::Hauptstimme,
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::PrincipalVoice(_)));
    }

    #[test]
    fn test_direction_type_content_percussion() {
        let dtc = DirectionTypeContent::Percussion(vec![Percussion {
            content: PercussionContent::Timpani,
        }]);
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Percussion(percs) = parsed {
            assert_eq!(percs.len(), 1);
        } else {
            panic!("Expected Percussion variant");
        }
    }

    #[test]
    fn test_direction_type_content_accordion_registration() {
        let dtc = DirectionTypeContent::AccordionRegistration(AccordionRegistration::default());
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(
            parsed,
            DirectionTypeContent::AccordionRegistration(_)
        ));
    }

    #[test]
    fn test_direction_type_content_staff_divide() {
        let dtc = DirectionTypeContent::StaffDivide(StaffDivide {
            r#type: StaffDivideSymbol::Down,
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::StaffDivide(_)));
    }

    #[test]
    fn test_direction_type_content_other_direction() {
        let dtc = DirectionTypeContent::OtherDirection(OtherDirection {
            value: "custom".to_string(),
            print_object: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = dtc.to_sexpr();
        let parsed = DirectionTypeContent::from_sexpr(&sexpr).unwrap();
        assert!(matches!(parsed, DirectionTypeContent::OtherDirection(_)));
    }

    #[test]
    fn test_direction_type_content_not_a_list() {
        let sexpr = Sexpr::symbol("dynamics");
        let result = DirectionTypeContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_direction_type_content_invalid_variant() {
        let sexpr = ListBuilder::new("invalid-direction-type").build();
        let result = DirectionTypeContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // DirectionType Tests
    // ========================================================================

    #[test]
    fn test_direction_type_round_trip() {
        let dt = DirectionType {
            content: DirectionTypeContent::Dynamics(Dynamics {
                content: vec![DynamicElement::F],
                print_style: PrintStyle::default(),
                placement: None,
            }),
        };
        let sexpr = dt.to_sexpr();
        let parsed = DirectionType::from_sexpr(&sexpr).unwrap();
        if let DirectionTypeContent::Dynamics(d) = parsed.content {
            assert_eq!(d.content.len(), 1);
        } else {
            panic!("Expected Dynamics content");
        }
    }

    #[test]
    fn test_direction_type_not_a_list() {
        let sexpr = Sexpr::symbol("direction-type");
        let result = DirectionType::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Direction Tests
    // ========================================================================

    #[test]
    fn test_direction_full() {
        let direction = Direction {
            direction_types: vec![DirectionType {
                content: DirectionTypeContent::Dynamics(Dynamics {
                    content: vec![DynamicElement::MF],
                    print_style: PrintStyle::default(),
                    placement: None,
                }),
            }],
            offset: Some(Offset {
                value: 2,
                sound: Some(YesNo::Yes),
            }),
            sound: Some(Sound {
                tempo: Some(120.0),
                ..Default::default()
            }),
            staff: Some(1),
            voice: Some("1".to_string()),
            placement: Some(AboveBelow::Below),
            directive: Some(YesNo::Yes),
        };
        let sexpr = direction.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("direction"));
        assert!(text.contains(":staff 1"));
        assert!(text.contains(":voice"));
        assert!(text.contains("offset"));
        assert!(text.contains("sound"));
        let parsed = Direction::from_sexpr(&sexpr).unwrap();
        assert_eq!(direction.staff, parsed.staff);
        assert_eq!(direction.voice, parsed.voice);
        assert_eq!(direction.placement, parsed.placement);
        assert_eq!(direction.directive, parsed.directive);
        assert!(parsed.offset.is_some());
        assert!(parsed.sound.is_some());
    }

    #[test]
    fn test_direction_minimal() {
        let direction = Direction {
            direction_types: vec![],
            offset: None,
            sound: None,
            staff: None,
            voice: None,
            placement: None,
            directive: None,
        };
        let sexpr = direction.to_sexpr();
        let parsed = Direction::from_sexpr(&sexpr).unwrap();
        assert!(parsed.direction_types.is_empty());
        assert!(parsed.offset.is_none());
        assert!(parsed.sound.is_none());
    }

    #[test]
    fn test_direction_not_a_list() {
        let sexpr = Sexpr::symbol("direction");
        let result = Direction::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_direction_wrong_head() {
        let sexpr = ListBuilder::new("wrong").build();
        let result = Direction::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Edge Cases and Error Handling
    // ========================================================================

    #[test]
    fn test_dynamics_with_relative_position() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::P],
            print_style: PrintStyle {
                position: Position {
                    default_x: None,
                    default_y: None,
                    relative_x: Some(5.0),
                    relative_y: Some(-10.0),
                },
                font: Default::default(),
                color: None,
            },
            placement: None,
        };
        let sexpr = dynamics.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        let parsed = Dynamics::from_sexpr(&sexpr).unwrap();
        assert_eq!(
            dynamics.print_style.position.relative_x,
            parsed.print_style.position.relative_x
        );
    }

    #[test]
    fn test_wedge_with_all_position_fields() {
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: None,
            spread: None,
            niente: None,
            line_type: None,
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: Some(3.0),
                relative_y: Some(4.0),
            },
            color: None,
        };
        let sexpr = wedge.to_sexpr();
        let parsed = Wedge::from_sexpr(&sexpr).unwrap();
        assert_eq!(wedge.position.default_x, parsed.position.default_x);
        assert_eq!(wedge.position.default_y, parsed.position.default_y);
        assert_eq!(wedge.position.relative_x, parsed.position.relative_x);
        assert_eq!(wedge.position.relative_y, parsed.position.relative_y);
    }

    #[test]
    fn test_multiple_direction_types() {
        let direction = Direction {
            direction_types: vec![
                DirectionType {
                    content: DirectionTypeContent::Dynamics(Dynamics {
                        content: vec![DynamicElement::F],
                        print_style: PrintStyle::default(),
                        placement: None,
                    }),
                },
                DirectionType {
                    content: DirectionTypeContent::Wedge(Wedge {
                        r#type: WedgeType::Crescendo,
                        number: None,
                        spread: None,
                        niente: None,
                        line_type: None,
                        position: Position::default(),
                        color: None,
                    }),
                },
            ],
            offset: None,
            sound: None,
            staff: None,
            voice: None,
            placement: None,
            directive: None,
        };
        let sexpr = direction.to_sexpr();
        let parsed = Direction::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.direction_types.len(), 2);
    }

    #[test]
    fn test_dynamics_multiple_elements() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::SF, DynamicElement::P],
            print_style: PrintStyle::default(),
            placement: None,
        };
        let sexpr = dynamics.to_sexpr();
        let parsed = Dynamics::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.content.len(), 2);
        assert_eq!(parsed.content[0], DynamicElement::SF);
        assert_eq!(parsed.content[1], DynamicElement::P);
    }
}
