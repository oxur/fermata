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
    Glass, HarpPedals, Image, LineEnd, Membrane, Metal, Metronome, MetronomeContent,
    MetronomeNote, MetronomeTuplet, OctaveShift, Offset, OnOff, OtherDirection, Pedal,
    PedalTuning, PedalType, Percussion, PercussionContent, PerMinute, Pitched, PrincipalVoice,
    PrincipalVoiceSymbol, Scordatura, Segno, Sound, StaffDivide, StaffDivideSymbol, Stick,
    StickLocation, StringMute, UpDownStopContinue, Wedge, WedgeType, Wood, Words,
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
                ListBuilder::new("other-dynamics")
                    .kwarg("value", s)
                    .build()
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
            if list
                .first()
                .map_or(false, |h| h.is_symbol("other-dynamics"))
            {
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
            DirectionTypeContent::Segno(segnos) => {
                ListBuilder::new("segnos").kwarg_list("items", segnos).build()
            }
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
    use crate::ir::common::{AboveBelow, YesNo};
    use crate::ir::duration::NoteTypeValue;
    use crate::sexpr::print_sexpr;

    // === DynamicElement Tests ===

    #[test]
    fn test_dynamic_element_round_trip() {
        for elem in [
            DynamicElement::P,
            DynamicElement::PP,
            DynamicElement::PPP,
            DynamicElement::F,
            DynamicElement::FF,
            DynamicElement::FFF,
            DynamicElement::MF,
            DynamicElement::MP,
            DynamicElement::SF,
            DynamicElement::SFZ,
            DynamicElement::FP,
        ] {
            let sexpr = elem.to_sexpr();
            let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
            assert_eq!(elem, parsed);
        }
    }

    #[test]
    fn test_dynamic_element_other() {
        let elem = DynamicElement::OtherDynamics("custom".to_string());
        let sexpr = elem.to_sexpr();
        let parsed = DynamicElement::from_sexpr(&sexpr).unwrap();
        assert_eq!(elem, parsed);
    }

    // === Dynamics Tests ===

    #[test]
    fn test_dynamics_forte() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::F],
            print_style: PrintStyle::default(),
            placement: Some(AboveBelow::Below),
        };

        let sexpr = dynamics.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("dynamics"));
        assert!(text.contains(":placement below"));

        let parsed = Dynamics::from_sexpr(&sexpr).unwrap();
        assert_eq!(dynamics.content, parsed.content);
        assert_eq!(dynamics.placement, parsed.placement);
    }

    // === Wedge Tests ===

    #[test]
    fn test_wedge_crescendo() {
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: Some(1),
            spread: Some(15.0),
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };

        let sexpr = wedge.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("wedge"));
        assert!(text.contains("crescendo"));

        let parsed = Wedge::from_sexpr(&sexpr).unwrap();
        assert_eq!(wedge.r#type, parsed.r#type);
        assert_eq!(wedge.number, parsed.number);
    }

    // === Metronome Tests ===

    #[test]
    fn test_metronome_per_minute() {
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
        let text = print_sexpr(&sexpr);
        assert!(text.contains("metronome"));
        assert!(text.contains("quarter"));
        assert!(text.contains("120"));

        let parsed = Metronome::from_sexpr(&sexpr).unwrap();
        assert_eq!(metronome.parentheses, parsed.parentheses);
    }

    // === Words Tests ===

    #[test]
    fn test_words_round_trip() {
        use crate::ir::common::LeftCenterRight;

        let words = Words {
            value: "dolce".to_string(),
            print_style: PrintStyle::default(),
            justify: Some(LeftCenterRight::Left),
            lang: Some("it".to_string()),
        };

        let sexpr = words.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("dolce"));

        let parsed = Words::from_sexpr(&sexpr).unwrap();
        assert_eq!(words.value, parsed.value);
        assert_eq!(words.justify, parsed.justify);
        assert_eq!(words.lang, parsed.lang);
    }

    // === Pedal Tests ===

    #[test]
    fn test_pedal_start() {
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: Some(YesNo::Yes),
            sign: Some(YesNo::No),
            abbreviated: None,
            print_style: PrintStyle::default(),
        };

        let sexpr = pedal.to_sexpr();
        let parsed = Pedal::from_sexpr(&sexpr).unwrap();
        assert_eq!(pedal.r#type, parsed.r#type);
        assert_eq!(pedal.line, parsed.line);
    }

    // === Direction Tests ===

    #[test]
    fn test_direction_with_dynamics() {
        let direction = Direction {
            direction_types: vec![DirectionType {
                content: DirectionTypeContent::Dynamics(Dynamics {
                    content: vec![DynamicElement::MF],
                    print_style: PrintStyle::default(),
                    placement: None,
                }),
            }],
            offset: None,
            sound: None,
            staff: Some(1),
            voice: Some("1".to_string()),
            placement: Some(AboveBelow::Below),
            directive: None,
        };

        let sexpr = direction.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("direction"));
        assert!(text.contains(":staff 1"));

        let parsed = Direction::from_sexpr(&sexpr).unwrap();
        assert_eq!(direction.staff, parsed.staff);
        assert_eq!(direction.placement, parsed.placement);
        assert_eq!(
            direction.direction_types.len(),
            parsed.direction_types.len()
        );
    }

    // === Sound Tests ===

    #[test]
    fn test_sound_with_tempo() {
        let sound = Sound {
            tempo: Some(120.0),
            dynamics: Some(80.0),
            dacapo: None,
            segno: None,
            dalsegno: None,
            coda: None,
            tocoda: None,
            divisions: None,
            forward_repeat: None,
            fine: None,
            time_only: None,
            pizzicato: None,
        };

        let sexpr = sound.to_sexpr();
        let parsed = Sound::from_sexpr(&sexpr).unwrap();
        assert_eq!(sound.tempo, parsed.tempo);
        assert_eq!(sound.dynamics, parsed.dynamics);
    }

    // === OctaveShift Tests ===

    #[test]
    fn test_octave_shift_round_trip() {
        let shift = OctaveShift {
            r#type: UpDownStopContinue::Down,
            number: Some(1),
            size: Some(8),
            position: Position::default(),
        };

        let sexpr = shift.to_sexpr();
        let parsed = OctaveShift::from_sexpr(&sexpr).unwrap();
        assert_eq!(shift.r#type, parsed.r#type);
        assert_eq!(shift.size, parsed.size);
    }

    // === Segno/Coda Tests ===

    #[test]
    fn test_segno_round_trip() {
        let segno = Segno {
            print_style: PrintStyle::default(),
            smufl: Some("segno".to_string()),
        };

        let sexpr = segno.to_sexpr();
        let parsed = Segno::from_sexpr(&sexpr).unwrap();
        assert_eq!(segno.smufl, parsed.smufl);
    }

    #[test]
    fn test_coda_round_trip() {
        let coda = Coda {
            print_style: PrintStyle::default(),
            smufl: None,
        };

        let sexpr = coda.to_sexpr();
        let _parsed = Coda::from_sexpr(&sexpr).unwrap();
    }
}
