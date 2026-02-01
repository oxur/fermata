//! S-expression conversions for `ir::notation` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for notation-related
//! types including articulations, ornaments, technical markings, slurs, ties,
//! tuplets, and other notation elements attached to notes.

use crate::ir::common::{EmptyPlacement, Position, PrintStyle};
use crate::ir::notation::{
    AccidentalMark, Arpeggiate, ArticulationElement, Articulations, Arrow, ArrowDirection,
    ArrowStyle, Bend, BendRelease, BreathMark, BreathMarkValue, Caesura, CaesuraValue,
    EmptyLine, EmptyTrillSound, Fingering, Fret, Glissando, HammerPull, Handbell, HandbellValue,
    HarmonMute, Harmonic, HeelToe, Hole, HoleClosed, HoleClosedLocation, HoleClosedValue,
    LineLength, LineShape, Mordent, NonArpeggiate, NotationContent, Notations, OrnamentElement,
    OrnamentWithAccidentals, Ornaments, OtherArticulation, OtherNotation, OtherOrnament,
    OtherTechnical, Pluck, ShowTuplet, Slide, Slur, StartNote, StringNumber, StrongAccent, Tap,
    TapHand, Technical, TechnicalElement, TopBottom, Tremolo, TremoloType, TrillStep, Tuplet,
    TupletDot, TupletNumber, TupletPortion, TupletType, Turn, Tied, TwoNoteTurn,
};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, get_head, optional_kwarg, require_kwarg};

// ============================================================================
// EmptyPlacement (needed for articulations/technical)
// ============================================================================

impl ToSexpr for EmptyPlacement {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("empty-placement");

        builder = builder.kwarg_opt("placement", &self.placement);

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

impl FromSexpr for EmptyPlacement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("empty-placement list", sexpr))?;

        expect_head(list, "empty-placement")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(EmptyPlacement {
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

// ============================================================================
// Simple Enums
// ============================================================================

impl ToSexpr for ShowTuplet {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            ShowTuplet::Actual => "actual",
            ShowTuplet::Both => "both",
            ShowTuplet::None => "none",
        })
    }
}

impl FromSexpr for ShowTuplet {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("actual") => Ok(ShowTuplet::Actual),
            Some("both") => Ok(ShowTuplet::Both),
            Some("none") => Ok(ShowTuplet::None),
            _ => Err(ConvertError::type_mismatch("show-tuplet", sexpr)),
        }
    }
}

impl ToSexpr for LineShape {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            LineShape::Straight => "straight",
            LineShape::Curved => "curved",
        })
    }
}

impl FromSexpr for LineShape {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("straight") => Ok(LineShape::Straight),
            Some("curved") => Ok(LineShape::Curved),
            _ => Err(ConvertError::type_mismatch("line-shape", sexpr)),
        }
    }
}

impl ToSexpr for TopBottom {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TopBottom::Top => "top",
            TopBottom::Bottom => "bottom",
        })
    }
}

impl FromSexpr for TopBottom {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("top") => Ok(TopBottom::Top),
            Some("bottom") => Ok(TopBottom::Bottom),
            _ => Err(ConvertError::type_mismatch("top-bottom", sexpr)),
        }
    }
}

impl ToSexpr for LineLength {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            LineLength::Short => "short",
            LineLength::Medium => "medium",
            LineLength::Long => "long",
        })
    }
}

impl FromSexpr for LineLength {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("short") => Ok(LineLength::Short),
            Some("medium") => Ok(LineLength::Medium),
            Some("long") => Ok(LineLength::Long),
            _ => Err(ConvertError::type_mismatch("line-length", sexpr)),
        }
    }
}

impl ToSexpr for BreathMarkValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BreathMarkValue::Empty => "empty",
            BreathMarkValue::Comma => "comma",
            BreathMarkValue::Tick => "tick",
            BreathMarkValue::Upbow => "upbow",
            BreathMarkValue::Salzedo => "salzedo",
        })
    }
}

impl FromSexpr for BreathMarkValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("empty") => Ok(BreathMarkValue::Empty),
            Some("comma") => Ok(BreathMarkValue::Comma),
            Some("tick") => Ok(BreathMarkValue::Tick),
            Some("upbow") => Ok(BreathMarkValue::Upbow),
            Some("salzedo") => Ok(BreathMarkValue::Salzedo),
            _ => Err(ConvertError::type_mismatch("breath-mark-value", sexpr)),
        }
    }
}

impl ToSexpr for CaesuraValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            CaesuraValue::Normal => "normal",
            CaesuraValue::Thick => "thick",
            CaesuraValue::Short => "short",
            CaesuraValue::Curved => "curved",
            CaesuraValue::Single => "single",
        })
    }
}

impl FromSexpr for CaesuraValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("normal") => Ok(CaesuraValue::Normal),
            Some("thick") => Ok(CaesuraValue::Thick),
            Some("short") => Ok(CaesuraValue::Short),
            Some("curved") => Ok(CaesuraValue::Curved),
            Some("single") => Ok(CaesuraValue::Single),
            _ => Err(ConvertError::type_mismatch("caesura-value", sexpr)),
        }
    }
}

impl ToSexpr for StartNote {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartNote::Upper => "upper",
            StartNote::Main => "main",
            StartNote::Below => "below",
        })
    }
}

impl FromSexpr for StartNote {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("upper") => Ok(StartNote::Upper),
            Some("main") => Ok(StartNote::Main),
            Some("below") => Ok(StartNote::Below),
            _ => Err(ConvertError::type_mismatch("start-note", sexpr)),
        }
    }
}

impl ToSexpr for TrillStep {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TrillStep::Whole => "whole",
            TrillStep::Half => "half",
            TrillStep::Unison => "unison",
        })
    }
}

impl FromSexpr for TrillStep {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("whole") => Ok(TrillStep::Whole),
            Some("half") => Ok(TrillStep::Half),
            Some("unison") => Ok(TrillStep::Unison),
            _ => Err(ConvertError::type_mismatch("trill-step", sexpr)),
        }
    }
}

impl ToSexpr for TwoNoteTurn {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TwoNoteTurn::Whole => "whole",
            TwoNoteTurn::Half => "half",
            TwoNoteTurn::None => "none",
        })
    }
}

impl FromSexpr for TwoNoteTurn {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("whole") => Ok(TwoNoteTurn::Whole),
            Some("half") => Ok(TwoNoteTurn::Half),
            Some("none") => Ok(TwoNoteTurn::None),
            _ => Err(ConvertError::type_mismatch("two-note-turn", sexpr)),
        }
    }
}

impl ToSexpr for TremoloType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TremoloType::Start => "start",
            TremoloType::Stop => "stop",
            TremoloType::Single => "single",
            TremoloType::Unmeasured => "unmeasured",
        })
    }
}

impl FromSexpr for TremoloType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(TremoloType::Start),
            Some("stop") => Ok(TremoloType::Stop),
            Some("single") => Ok(TremoloType::Single),
            Some("unmeasured") => Ok(TremoloType::Unmeasured),
            _ => Err(ConvertError::type_mismatch("tremolo-type", sexpr)),
        }
    }
}

impl ToSexpr for BendRelease {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BendRelease::Early => "early",
            BendRelease::Late => "late",
        })
    }
}

impl FromSexpr for BendRelease {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("early") => Ok(BendRelease::Early),
            Some("late") => Ok(BendRelease::Late),
            _ => Err(ConvertError::type_mismatch("bend-release", sexpr)),
        }
    }
}

impl ToSexpr for TapHand {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TapHand::Left => "left",
            TapHand::Right => "right",
        })
    }
}

impl FromSexpr for TapHand {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("left") => Ok(TapHand::Left),
            Some("right") => Ok(TapHand::Right),
            _ => Err(ConvertError::type_mismatch("tap-hand", sexpr)),
        }
    }
}

impl ToSexpr for HoleClosedValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            HoleClosedValue::Yes => "yes",
            HoleClosedValue::No => "no",
            HoleClosedValue::Half => "half",
        })
    }
}

impl FromSexpr for HoleClosedValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("yes") => Ok(HoleClosedValue::Yes),
            Some("no") => Ok(HoleClosedValue::No),
            Some("half") => Ok(HoleClosedValue::Half),
            _ => Err(ConvertError::type_mismatch("hole-closed-value", sexpr)),
        }
    }
}

impl ToSexpr for HoleClosedLocation {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            HoleClosedLocation::Right => "right",
            HoleClosedLocation::Bottom => "bottom",
            HoleClosedLocation::Left => "left",
            HoleClosedLocation::Top => "top",
        })
    }
}

impl FromSexpr for HoleClosedLocation {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("right") => Ok(HoleClosedLocation::Right),
            Some("bottom") => Ok(HoleClosedLocation::Bottom),
            Some("left") => Ok(HoleClosedLocation::Left),
            Some("top") => Ok(HoleClosedLocation::Top),
            _ => Err(ConvertError::type_mismatch("hole-closed-location", sexpr)),
        }
    }
}

impl ToSexpr for ArrowDirection {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            ArrowDirection::Left => "left",
            ArrowDirection::Up => "up",
            ArrowDirection::Right => "right",
            ArrowDirection::Down => "down",
            ArrowDirection::Northwest => "northwest",
            ArrowDirection::Northeast => "northeast",
            ArrowDirection::Southeast => "southeast",
            ArrowDirection::Southwest => "southwest",
            ArrowDirection::LeftRight => "left-right",
            ArrowDirection::UpDown => "up-down",
            ArrowDirection::NorthwestSoutheast => "northwest-southeast",
            ArrowDirection::NortheastSouthwest => "northeast-southwest",
            ArrowDirection::Other => "other",
        })
    }
}

impl FromSexpr for ArrowDirection {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("left") => Ok(ArrowDirection::Left),
            Some("up") => Ok(ArrowDirection::Up),
            Some("right") => Ok(ArrowDirection::Right),
            Some("down") => Ok(ArrowDirection::Down),
            Some("northwest") => Ok(ArrowDirection::Northwest),
            Some("northeast") => Ok(ArrowDirection::Northeast),
            Some("southeast") => Ok(ArrowDirection::Southeast),
            Some("southwest") => Ok(ArrowDirection::Southwest),
            Some("left-right") => Ok(ArrowDirection::LeftRight),
            Some("up-down") => Ok(ArrowDirection::UpDown),
            Some("northwest-southeast") => Ok(ArrowDirection::NorthwestSoutheast),
            Some("northeast-southwest") => Ok(ArrowDirection::NortheastSouthwest),
            Some("other") => Ok(ArrowDirection::Other),
            _ => Err(ConvertError::type_mismatch("arrow-direction", sexpr)),
        }
    }
}

impl ToSexpr for ArrowStyle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            ArrowStyle::Single => "single",
            ArrowStyle::Double => "double",
            ArrowStyle::Filled => "filled",
            ArrowStyle::Hollow => "hollow",
            ArrowStyle::Paired => "paired",
            ArrowStyle::Combined => "combined",
            ArrowStyle::Other => "other",
        })
    }
}

impl FromSexpr for ArrowStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("single") => Ok(ArrowStyle::Single),
            Some("double") => Ok(ArrowStyle::Double),
            Some("filled") => Ok(ArrowStyle::Filled),
            Some("hollow") => Ok(ArrowStyle::Hollow),
            Some("paired") => Ok(ArrowStyle::Paired),
            Some("combined") => Ok(ArrowStyle::Combined),
            Some("other") => Ok(ArrowStyle::Other),
            _ => Err(ConvertError::type_mismatch("arrow-style", sexpr)),
        }
    }
}

impl ToSexpr for HandbellValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            HandbellValue::Belltree => "belltree",
            HandbellValue::Damp => "damp",
            HandbellValue::Echo => "echo",
            HandbellValue::Gyro => "gyro",
            HandbellValue::HandMartellato => "hand-martellato",
            HandbellValue::MalletLift => "mallet-lift",
            HandbellValue::MalletTable => "mallet-table",
            HandbellValue::Martellato => "martellato",
            HandbellValue::MartellatoLift => "martellato-lift",
            HandbellValue::MutedMartellato => "muted-martellato",
            HandbellValue::PluckLift => "pluck-lift",
            HandbellValue::Swing => "swing",
        })
    }
}

impl FromSexpr for HandbellValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("belltree") => Ok(HandbellValue::Belltree),
            Some("damp") => Ok(HandbellValue::Damp),
            Some("echo") => Ok(HandbellValue::Echo),
            Some("gyro") => Ok(HandbellValue::Gyro),
            Some("hand-martellato") => Ok(HandbellValue::HandMartellato),
            Some("mallet-lift") => Ok(HandbellValue::MalletLift),
            Some("mallet-table") => Ok(HandbellValue::MalletTable),
            Some("martellato") => Ok(HandbellValue::Martellato),
            Some("martellato-lift") => Ok(HandbellValue::MartellatoLift),
            Some("muted-martellato") => Ok(HandbellValue::MutedMartellato),
            Some("pluck-lift") => Ok(HandbellValue::PluckLift),
            Some("swing") => Ok(HandbellValue::Swing),
            _ => Err(ConvertError::type_mismatch("handbell-value", sexpr)),
        }
    }
}

// ============================================================================
// Tied, Slur, Glissando, Slide
// ============================================================================

impl ToSexpr for Tied {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tied")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("line-type", &self.line_type)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("orientation", &self.orientation)
            .kwarg_opt("color", &self.color);

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

impl FromSexpr for Tied {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tied list", sexpr))?;

        expect_head(list, "tied")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Tied {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            line_type: optional_kwarg(list, "line-type")?,
            position,
            placement: optional_kwarg(list, "placement")?,
            orientation: optional_kwarg(list, "orientation")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for Slur {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("slur")
            .kwarg("type", &self.r#type)
            .kwarg("number", &self.number)
            .kwarg_opt("line-type", &self.line_type)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("orientation", &self.orientation)
            .kwarg_opt("color", &self.color);

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

impl FromSexpr for Slur {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("slur list", sexpr))?;

        expect_head(list, "slur")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Slur {
            r#type: require_kwarg(list, "type")?,
            number: require_kwarg(list, "number")?,
            line_type: optional_kwarg(list, "line-type")?,
            position,
            placement: optional_kwarg(list, "placement")?,
            orientation: optional_kwarg(list, "orientation")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for Glissando {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("glissando")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("text", &self.text)
            .kwarg_opt("line-type", &self.line_type);

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

impl FromSexpr for Glissando {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("glissando list", sexpr))?;

        expect_head(list, "glissando")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Glissando {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            text: optional_kwarg(list, "text")?,
            line_type: optional_kwarg(list, "line-type")?,
            position,
        })
    }
}

impl ToSexpr for Slide {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("slide")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("text", &self.text)
            .kwarg_opt("line-type", &self.line_type);

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

impl FromSexpr for Slide {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("slide list", sexpr))?;

        expect_head(list, "slide")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Slide {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            text: optional_kwarg(list, "text")?,
            line_type: optional_kwarg(list, "line-type")?,
            position,
        })
    }
}

// ============================================================================
// Tuplet and related types
// ============================================================================

impl ToSexpr for TupletNumber {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tuplet-number").kwarg("value", &self.value);

        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();
        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);
        builder.build()
    }
}

impl FromSexpr for TupletNumber {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tuplet-number list", sexpr))?;

        expect_head(list, "tuplet-number")?;

        use crate::ir::common::Font;
        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(TupletNumber {
            value: require_kwarg(list, "value")?,
            font,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for TupletType {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tuplet-type").kwarg("value", &self.value);

        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();
        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);
        builder.build()
    }
}

impl FromSexpr for TupletType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tuplet-type list", sexpr))?;

        expect_head(list, "tuplet-type")?;

        use crate::ir::common::Font;
        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(TupletType {
            value: require_kwarg(list, "value")?,
            font,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for TupletDot {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tuplet-dot");

        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();
        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);
        builder.build()
    }
}

impl FromSexpr for TupletDot {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tuplet-dot list", sexpr))?;

        expect_head(list, "tuplet-dot")?;

        use crate::ir::common::Font;
        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(TupletDot {
            font,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for TupletPortion {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tuplet-portion");

        if let Some(ref num) = self.tuplet_number {
            builder = builder.kwarg_raw("number", num.to_sexpr());
        }
        if let Some(ref typ) = self.tuplet_type {
            builder = builder.kwarg_raw("type", typ.to_sexpr());
        }
        if !self.tuplet_dots.is_empty() {
            builder = builder.kwarg_list("dots", &self.tuplet_dots);
        }

        builder.build()
    }
}

impl FromSexpr for TupletPortion {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tuplet-portion list", sexpr))?;

        expect_head(list, "tuplet-portion")?;

        Ok(TupletPortion {
            tuplet_number: optional_kwarg(list, "number")?,
            tuplet_type: optional_kwarg(list, "type")?,
            tuplet_dots: optional_kwarg::<Vec<TupletDot>>(list, "dots")?.unwrap_or_default(),
        })
    }
}

impl ToSexpr for Tuplet {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tuplet")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("bracket", &self.bracket)
            .kwarg_opt("show-number", &self.show_number)
            .kwarg_opt("show-type", &self.show_type)
            .kwarg_opt("line-shape", &self.line_shape)
            .kwarg_opt("placement", &self.placement);

        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        if let Some(ref actual) = self.tuplet_actual {
            builder = builder.kwarg_raw("actual", actual.to_sexpr());
        }
        if let Some(ref normal) = self.tuplet_normal {
            builder = builder.kwarg_raw("normal", normal.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for Tuplet {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tuplet list", sexpr))?;

        expect_head(list, "tuplet")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Tuplet {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            bracket: optional_kwarg(list, "bracket")?,
            show_number: optional_kwarg(list, "show-number")?,
            show_type: optional_kwarg(list, "show-type")?,
            line_shape: optional_kwarg(list, "line-shape")?,
            position,
            placement: optional_kwarg(list, "placement")?,
            tuplet_actual: optional_kwarg(list, "actual")?,
            tuplet_normal: optional_kwarg(list, "normal")?,
        })
    }
}

// ============================================================================
// Arpeggiate, NonArpeggiate, AccidentalMark, OtherNotation
// ============================================================================

impl ToSexpr for Arpeggiate {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("arpeggiate")
            .kwarg_opt("number", &self.number)
            .kwarg_opt("direction", &self.direction)
            .kwarg_opt("color", &self.color);

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

impl FromSexpr for Arpeggiate {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("arpeggiate list", sexpr))?;

        expect_head(list, "arpeggiate")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Arpeggiate {
            number: optional_kwarg(list, "number")?,
            direction: optional_kwarg(list, "direction")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for NonArpeggiate {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("non-arpeggiate")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("color", &self.color);

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

impl FromSexpr for NonArpeggiate {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("non-arpeggiate list", sexpr))?;

        expect_head(list, "non-arpeggiate")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(NonArpeggiate {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for AccidentalMark {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("accidental-mark")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for AccidentalMark {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("accidental-mark list", sexpr))?;

        expect_head(list, "accidental-mark")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(AccidentalMark {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

impl ToSexpr for OtherNotation {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("other-notation")
            .kwarg("value", &self.value)
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("print-object", &self.print_object)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for OtherNotation {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("other-notation list", sexpr))?;

        expect_head(list, "other-notation")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(OtherNotation {
            value: require_kwarg(list, "value")?,
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            print_object: optional_kwarg(list, "print-object")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
            placement: optional_kwarg(list, "placement")?,
        })
    }
}

// ============================================================================
// Articulation structs
// ============================================================================

impl ToSexpr for StrongAccent {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("strong-accent")
            .kwarg_opt("type", &self.r#type)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for StrongAccent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("strong-accent list", sexpr))?;

        expect_head(list, "strong-accent")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(StrongAccent {
            r#type: optional_kwarg(list, "type")?,
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

impl ToSexpr for EmptyLine {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("empty-line")
            .kwarg_opt("line-shape", &self.line_shape)
            .kwarg_opt("line-type", &self.line_type)
            .kwarg_opt("line-length", &self.line_length)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for EmptyLine {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("empty-line list", sexpr))?;

        expect_head(list, "empty-line")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(EmptyLine {
            line_shape: optional_kwarg(list, "line-shape")?,
            line_type: optional_kwarg(list, "line-type")?,
            line_length: optional_kwarg(list, "line-length")?,
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

impl ToSexpr for BreathMark {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("breath-mark")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for BreathMark {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("breath-mark list", sexpr))?;

        expect_head(list, "breath-mark")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(BreathMark {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

impl ToSexpr for Caesura {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("caesura")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for Caesura {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("caesura list", sexpr))?;

        expect_head(list, "caesura")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Caesura {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

impl ToSexpr for OtherArticulation {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("other-articulation")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for OtherArticulation {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("other-articulation list", sexpr))?;

        expect_head(list, "other-articulation")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(OtherArticulation {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

// ============================================================================
// ArticulationElement enum
// ============================================================================

impl ToSexpr for ArticulationElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            ArticulationElement::Accent(ep) => {
                ListBuilder::new("accent")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::StrongAccent(sa) => sa.to_sexpr(),
            ArticulationElement::Staccato(ep) => {
                ListBuilder::new("staccato")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::Tenuto(ep) => {
                ListBuilder::new("tenuto")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::DetachedLegato(ep) => {
                ListBuilder::new("detached-legato")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::Staccatissimo(ep) => {
                ListBuilder::new("staccatissimo")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::Spiccato(ep) => {
                ListBuilder::new("spiccato")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::Scoop(el) => {
                ListBuilder::new("scoop")
                    .kwarg_raw("data", el.to_sexpr())
                    .build()
            }
            ArticulationElement::Plop(el) => {
                ListBuilder::new("plop")
                    .kwarg_raw("data", el.to_sexpr())
                    .build()
            }
            ArticulationElement::Doit(el) => {
                ListBuilder::new("doit")
                    .kwarg_raw("data", el.to_sexpr())
                    .build()
            }
            ArticulationElement::Falloff(el) => {
                ListBuilder::new("falloff")
                    .kwarg_raw("data", el.to_sexpr())
                    .build()
            }
            ArticulationElement::BreathMark(bm) => bm.to_sexpr(),
            ArticulationElement::Caesura(c) => c.to_sexpr(),
            ArticulationElement::Stress(ep) => {
                ListBuilder::new("stress")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::Unstress(ep) => {
                ListBuilder::new("unstress")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::SoftAccent(ep) => {
                ListBuilder::new("soft-accent")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            ArticulationElement::OtherArticulation(oa) => oa.to_sexpr(),
        }
    }
}

impl FromSexpr for ArticulationElement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("articulation-element", sexpr))?;

        match get_head(list)? {
            "accent" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Accent(data))
            }
            "strong-accent" => Ok(ArticulationElement::StrongAccent(StrongAccent::from_sexpr(
                sexpr,
            )?)),
            "staccato" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Staccato(data))
            }
            "tenuto" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Tenuto(data))
            }
            "detached-legato" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::DetachedLegato(data))
            }
            "staccatissimo" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Staccatissimo(data))
            }
            "spiccato" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Spiccato(data))
            }
            "scoop" => {
                let data: EmptyLine = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Scoop(data))
            }
            "plop" => {
                let data: EmptyLine = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Plop(data))
            }
            "doit" => {
                let data: EmptyLine = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Doit(data))
            }
            "falloff" => {
                let data: EmptyLine = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Falloff(data))
            }
            "breath-mark" => Ok(ArticulationElement::BreathMark(BreathMark::from_sexpr(
                sexpr,
            )?)),
            "caesura" => Ok(ArticulationElement::Caesura(Caesura::from_sexpr(sexpr)?)),
            "stress" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Stress(data))
            }
            "unstress" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::Unstress(data))
            }
            "soft-accent" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(ArticulationElement::SoftAccent(data))
            }
            "other-articulation" => Ok(ArticulationElement::OtherArticulation(
                OtherArticulation::from_sexpr(sexpr)?,
            )),
            _ => Err(ConvertError::type_mismatch(
                "articulation-element variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Articulations container
// ============================================================================

impl ToSexpr for Articulations {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("articulations");

        if !self.content.is_empty() {
            builder = builder.kwarg_list("content", &self.content);
        }

        builder.build()
    }
}

impl FromSexpr for Articulations {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("articulations list", sexpr))?;

        expect_head(list, "articulations")?;

        Ok(Articulations {
            content: optional_kwarg::<Vec<ArticulationElement>>(list, "content")?.unwrap_or_default(),
        })
    }
}

// ============================================================================
// Ornament structs
// ============================================================================

impl ToSexpr for EmptyTrillSound {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("empty-trill-sound")
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("start-note", &self.start_note)
            .kwarg_opt("trill-step", &self.trill_step)
            .kwarg_opt("two-note-turn", &self.two_note_turn)
            .kwarg_opt("accelerate", &self.accelerate)
            .kwarg_opt("beats", &self.beats)
            .kwarg_opt("second-beat", &self.second_beat)
            .kwarg_opt("last-beat", &self.last_beat);

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

impl FromSexpr for EmptyTrillSound {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("empty-trill-sound list", sexpr))?;

        expect_head(list, "empty-trill-sound")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(EmptyTrillSound {
            placement: optional_kwarg(list, "placement")?,
            position,
            start_note: optional_kwarg(list, "start-note")?,
            trill_step: optional_kwarg(list, "trill-step")?,
            two_note_turn: optional_kwarg(list, "two-note-turn")?,
            accelerate: optional_kwarg(list, "accelerate")?,
            beats: optional_kwarg(list, "beats")?,
            second_beat: optional_kwarg(list, "second-beat")?,
            last_beat: optional_kwarg(list, "last-beat")?,
        })
    }
}

impl ToSexpr for Turn {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("turn")
            .kwarg_opt("slash", &self.slash)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("start-note", &self.start_note)
            .kwarg_opt("trill-step", &self.trill_step)
            .kwarg_opt("two-note-turn", &self.two_note_turn)
            .kwarg_opt("accelerate", &self.accelerate)
            .kwarg_opt("beats", &self.beats)
            .kwarg_opt("second-beat", &self.second_beat)
            .kwarg_opt("last-beat", &self.last_beat);

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

impl FromSexpr for Turn {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("turn list", sexpr))?;

        expect_head(list, "turn")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Turn {
            slash: optional_kwarg(list, "slash")?,
            placement: optional_kwarg(list, "placement")?,
            position,
            start_note: optional_kwarg(list, "start-note")?,
            trill_step: optional_kwarg(list, "trill-step")?,
            two_note_turn: optional_kwarg(list, "two-note-turn")?,
            accelerate: optional_kwarg(list, "accelerate")?,
            beats: optional_kwarg(list, "beats")?,
            second_beat: optional_kwarg(list, "second-beat")?,
            last_beat: optional_kwarg(list, "last-beat")?,
        })
    }
}

impl ToSexpr for Mordent {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("mordent")
            .kwarg_opt("long", &self.long)
            .kwarg_opt("approach", &self.approach)
            .kwarg_opt("departure", &self.departure)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("start-note", &self.start_note)
            .kwarg_opt("trill-step", &self.trill_step)
            .kwarg_opt("two-note-turn", &self.two_note_turn)
            .kwarg_opt("accelerate", &self.accelerate)
            .kwarg_opt("beats", &self.beats)
            .kwarg_opt("second-beat", &self.second_beat)
            .kwarg_opt("last-beat", &self.last_beat);

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

impl FromSexpr for Mordent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("mordent list", sexpr))?;

        expect_head(list, "mordent")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Mordent {
            long: optional_kwarg(list, "long")?,
            approach: optional_kwarg(list, "approach")?,
            departure: optional_kwarg(list, "departure")?,
            placement: optional_kwarg(list, "placement")?,
            position,
            start_note: optional_kwarg(list, "start-note")?,
            trill_step: optional_kwarg(list, "trill-step")?,
            two_note_turn: optional_kwarg(list, "two-note-turn")?,
            accelerate: optional_kwarg(list, "accelerate")?,
            beats: optional_kwarg(list, "beats")?,
            second_beat: optional_kwarg(list, "second-beat")?,
            last_beat: optional_kwarg(list, "last-beat")?,
        })
    }
}

impl ToSexpr for Tremolo {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("tremolo")
            .kwarg("value", &self.value)
            .kwarg_opt("type", &self.r#type)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for Tremolo {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tremolo list", sexpr))?;

        expect_head(list, "tremolo")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Tremolo {
            value: require_kwarg(list, "value")?,
            r#type: optional_kwarg(list, "type")?,
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

impl ToSexpr for OtherOrnament {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("other-ornament")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for OtherOrnament {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("other-ornament list", sexpr))?;

        expect_head(list, "other-ornament")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(OtherOrnament {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

// ============================================================================
// OrnamentElement enum
// ============================================================================

impl ToSexpr for OrnamentElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            OrnamentElement::TrillMark(ets) => {
                ListBuilder::new("trill-mark")
                    .kwarg_raw("data", ets.to_sexpr())
                    .build()
            }
            OrnamentElement::Turn(t) => t.to_sexpr(),
            OrnamentElement::DelayedTurn(t) => {
                ListBuilder::new("delayed-turn")
                    .kwarg_raw("data", t.to_sexpr())
                    .build()
            }
            OrnamentElement::InvertedTurn(t) => {
                ListBuilder::new("inverted-turn")
                    .kwarg_raw("data", t.to_sexpr())
                    .build()
            }
            OrnamentElement::DelayedInvertedTurn(t) => {
                ListBuilder::new("delayed-inverted-turn")
                    .kwarg_raw("data", t.to_sexpr())
                    .build()
            }
            OrnamentElement::VerticalTurn(ets) => {
                ListBuilder::new("vertical-turn")
                    .kwarg_raw("data", ets.to_sexpr())
                    .build()
            }
            OrnamentElement::InvertedVerticalTurn(ets) => {
                ListBuilder::new("inverted-vertical-turn")
                    .kwarg_raw("data", ets.to_sexpr())
                    .build()
            }
            OrnamentElement::Shake(ets) => {
                ListBuilder::new("shake")
                    .kwarg_raw("data", ets.to_sexpr())
                    .build()
            }
            OrnamentElement::WavyLine(wl) => {
                ListBuilder::new("wavy-line")
                    .kwarg_raw("data", wl.to_sexpr())
                    .build()
            }
            OrnamentElement::Mordent(m) => m.to_sexpr(),
            OrnamentElement::InvertedMordent(m) => {
                ListBuilder::new("inverted-mordent")
                    .kwarg_raw("data", m.to_sexpr())
                    .build()
            }
            OrnamentElement::Schleifer(ep) => {
                ListBuilder::new("schleifer")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            OrnamentElement::Tremolo(t) => t.to_sexpr(),
            OrnamentElement::Haydn(ets) => {
                ListBuilder::new("haydn")
                    .kwarg_raw("data", ets.to_sexpr())
                    .build()
            }
            OrnamentElement::OtherOrnament(oo) => oo.to_sexpr(),
        }
    }
}

impl FromSexpr for OrnamentElement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("ornament-element", sexpr))?;

        match get_head(list)? {
            "trill-mark" => {
                let data: EmptyTrillSound = require_kwarg(list, "data")?;
                Ok(OrnamentElement::TrillMark(data))
            }
            "turn" => Ok(OrnamentElement::Turn(Turn::from_sexpr(sexpr)?)),
            "delayed-turn" => {
                let data: Turn = require_kwarg(list, "data")?;
                Ok(OrnamentElement::DelayedTurn(data))
            }
            "inverted-turn" => {
                let data: Turn = require_kwarg(list, "data")?;
                Ok(OrnamentElement::InvertedTurn(data))
            }
            "delayed-inverted-turn" => {
                let data: Turn = require_kwarg(list, "data")?;
                Ok(OrnamentElement::DelayedInvertedTurn(data))
            }
            "vertical-turn" => {
                let data: EmptyTrillSound = require_kwarg(list, "data")?;
                Ok(OrnamentElement::VerticalTurn(data))
            }
            "inverted-vertical-turn" => {
                let data: EmptyTrillSound = require_kwarg(list, "data")?;
                Ok(OrnamentElement::InvertedVerticalTurn(data))
            }
            "shake" => {
                let data: EmptyTrillSound = require_kwarg(list, "data")?;
                Ok(OrnamentElement::Shake(data))
            }
            "wavy-line" => {
                use crate::ir::common::WavyLine;
                let data: WavyLine = require_kwarg(list, "data")?;
                Ok(OrnamentElement::WavyLine(data))
            }
            "mordent" => Ok(OrnamentElement::Mordent(Mordent::from_sexpr(sexpr)?)),
            "inverted-mordent" => {
                let data: Mordent = require_kwarg(list, "data")?;
                Ok(OrnamentElement::InvertedMordent(data))
            }
            "schleifer" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(OrnamentElement::Schleifer(data))
            }
            "tremolo" => Ok(OrnamentElement::Tremolo(Tremolo::from_sexpr(sexpr)?)),
            "haydn" => {
                let data: EmptyTrillSound = require_kwarg(list, "data")?;
                Ok(OrnamentElement::Haydn(data))
            }
            "other-ornament" => Ok(OrnamentElement::OtherOrnament(OtherOrnament::from_sexpr(
                sexpr,
            )?)),
            _ => Err(ConvertError::type_mismatch("ornament-element variant", sexpr)),
        }
    }
}

// ============================================================================
// OrnamentWithAccidentals and Ornaments
// ============================================================================

impl ToSexpr for OrnamentWithAccidentals {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("ornament-with-accidentals")
            .kwarg_raw("ornament", self.ornament.to_sexpr());

        if !self.accidental_marks.is_empty() {
            builder = builder.kwarg_list("accidental-marks", &self.accidental_marks);
        }

        builder.build()
    }
}

impl FromSexpr for OrnamentWithAccidentals {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("ornament-with-accidentals list", sexpr))?;

        expect_head(list, "ornament-with-accidentals")?;

        Ok(OrnamentWithAccidentals {
            ornament: require_kwarg(list, "ornament")?,
            accidental_marks: optional_kwarg::<Vec<AccidentalMark>>(list, "accidental-marks")?
                .unwrap_or_default(),
        })
    }
}

impl ToSexpr for Ornaments {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("ornaments");

        if !self.content.is_empty() {
            builder = builder.kwarg_list("content", &self.content);
        }

        builder.build()
    }
}

impl FromSexpr for Ornaments {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("ornaments list", sexpr))?;

        expect_head(list, "ornaments")?;

        Ok(Ornaments {
            content: optional_kwarg::<Vec<OrnamentWithAccidentals>>(list, "content")?
                .unwrap_or_default(),
        })
    }
}

// ============================================================================
// Technical structs (Part 1 - basic types)
// ============================================================================

impl ToSexpr for Harmonic {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("harmonic");

        if self.natural {
            builder = builder.kwarg("natural", &true);
        }
        if self.artificial {
            builder = builder.kwarg("artificial", &true);
        }
        if self.base_pitch {
            builder = builder.kwarg("base-pitch", &true);
        }
        if self.touching_pitch {
            builder = builder.kwarg("touching-pitch", &true);
        }
        if self.sounding_pitch {
            builder = builder.kwarg("sounding-pitch", &true);
        }

        builder = builder
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("print-object", &self.print_object);

        builder.build()
    }
}

impl FromSexpr for Harmonic {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("harmonic list", sexpr))?;

        expect_head(list, "harmonic")?;

        Ok(Harmonic {
            natural: optional_kwarg::<bool>(list, "natural")?.unwrap_or(false),
            artificial: optional_kwarg::<bool>(list, "artificial")?.unwrap_or(false),
            base_pitch: optional_kwarg::<bool>(list, "base-pitch")?.unwrap_or(false),
            touching_pitch: optional_kwarg::<bool>(list, "touching-pitch")?.unwrap_or(false),
            sounding_pitch: optional_kwarg::<bool>(list, "sounding-pitch")?.unwrap_or(false),
            placement: optional_kwarg(list, "placement")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}

impl ToSexpr for Fingering {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("fingering")
            .kwarg("value", &self.value)
            .kwarg_opt("substitution", &self.substitution)
            .kwarg_opt("alternate", &self.alternate)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for Fingering {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("fingering list", sexpr))?;

        expect_head(list, "fingering")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(Fingering {
            value: require_kwarg(list, "value")?,
            substitution: optional_kwarg(list, "substitution")?,
            alternate: optional_kwarg(list, "alternate")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

impl ToSexpr for Pluck {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("pluck")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement)
            .build()
    }
}

impl FromSexpr for Pluck {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pluck list", sexpr))?;

        expect_head(list, "pluck")?;

        Ok(Pluck {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
        })
    }
}

impl ToSexpr for Fret {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("fret").kwarg("value", &self.value);

        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();
        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);
        builder.build()
    }
}

impl FromSexpr for Fret {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("fret list", sexpr))?;

        expect_head(list, "fret")?;

        use crate::ir::common::Font;
        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(Fret {
            value: require_kwarg(list, "value")?,
            font,
            color: optional_kwarg(list, "color")?,
        })
    }
}

impl ToSexpr for StringNumber {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("string-number")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for StringNumber {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("string-number list", sexpr))?;

        expect_head(list, "string-number")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(StringNumber {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

impl ToSexpr for HammerPull {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("hammer-pull")
            .kwarg("value", &self.value)
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("placement", &self.placement)
            .build()
    }
}

impl FromSexpr for HammerPull {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("hammer-pull list", sexpr))?;

        expect_head(list, "hammer-pull")?;

        Ok(HammerPull {
            value: require_kwarg(list, "value")?,
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            placement: optional_kwarg(list, "placement")?,
        })
    }
}

impl ToSexpr for Bend {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("bend").kwarg("bend-alter", &self.bend_alter);

        if self.pre_bend {
            builder = builder.kwarg("pre-bend", &true);
        }

        builder = builder
            .kwarg_opt("release", &self.release)
            .kwarg_opt("with-bar", &self.with_bar);

        builder.build()
    }
}

impl FromSexpr for Bend {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("bend list", sexpr))?;

        expect_head(list, "bend")?;

        Ok(Bend {
            bend_alter: require_kwarg(list, "bend-alter")?,
            pre_bend: optional_kwarg::<bool>(list, "pre-bend")?.unwrap_or(false),
            release: optional_kwarg(list, "release")?,
            with_bar: optional_kwarg(list, "with-bar")?,
        })
    }
}

impl ToSexpr for Tap {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("tap")
            .kwarg("value", &self.value)
            .kwarg_opt("hand", &self.hand)
            .build()
    }
}

impl FromSexpr for Tap {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tap list", sexpr))?;

        expect_head(list, "tap")?;

        Ok(Tap {
            value: require_kwarg(list, "value")?,
            hand: optional_kwarg(list, "hand")?,
        })
    }
}

impl ToSexpr for HeelToe {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("heel-toe")
            .kwarg_opt("substitution", &self.substitution)
            .kwarg_opt("placement", &self.placement)
            .build()
    }
}

impl FromSexpr for HeelToe {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("heel-toe list", sexpr))?;

        expect_head(list, "heel-toe")?;

        Ok(HeelToe {
            substitution: optional_kwarg(list, "substitution")?,
            placement: optional_kwarg(list, "placement")?,
        })
    }
}

impl ToSexpr for HoleClosed {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("hole-closed")
            .kwarg("value", &self.value)
            .kwarg_opt("location", &self.location)
            .build()
    }
}

impl FromSexpr for HoleClosed {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("hole-closed list", sexpr))?;

        expect_head(list, "hole-closed")?;

        Ok(HoleClosed {
            value: require_kwarg(list, "value")?,
            location: optional_kwarg(list, "location")?,
        })
    }
}

impl ToSexpr for Hole {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("hole")
            .kwarg_opt("hole-type", &self.hole_type)
            .kwarg_raw("hole-closed", self.hole_closed.to_sexpr())
            .kwarg_opt("hole-shape", &self.hole_shape)
            .build()
    }
}

impl FromSexpr for Hole {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("hole list", sexpr))?;

        expect_head(list, "hole")?;

        Ok(Hole {
            hole_type: optional_kwarg(list, "hole-type")?,
            hole_closed: require_kwarg(list, "hole-closed")?,
            hole_shape: optional_kwarg(list, "hole-shape")?,
        })
    }
}

impl ToSexpr for Arrow {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("arrow")
            .kwarg_opt("direction", &self.direction)
            .kwarg_opt("style", &self.style)
            .kwarg_opt("smufl", &self.smufl)
            .build()
    }
}

impl FromSexpr for Arrow {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("arrow list", sexpr))?;

        expect_head(list, "arrow")?;

        Ok(Arrow {
            direction: optional_kwarg(list, "direction")?,
            style: optional_kwarg(list, "style")?,
            smufl: optional_kwarg(list, "smufl")?,
        })
    }
}

impl ToSexpr for Handbell {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("handbell")
            .kwarg("value", &self.value)
            .build()
    }
}

impl FromSexpr for Handbell {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("handbell list", sexpr))?;

        expect_head(list, "handbell")?;

        Ok(Handbell {
            value: require_kwarg(list, "value")?,
        })
    }
}

impl ToSexpr for HarmonMute {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("harmon-mute");

        if self.open {
            builder = builder.kwarg("open", &true);
        }
        if self.half {
            builder = builder.kwarg("half", &true);
        }

        builder.build()
    }
}

impl FromSexpr for HarmonMute {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("harmon-mute list", sexpr))?;

        expect_head(list, "harmon-mute")?;

        Ok(HarmonMute {
            open: optional_kwarg::<bool>(list, "open")?.unwrap_or(false),
            half: optional_kwarg::<bool>(list, "half")?.unwrap_or(false),
        })
    }
}

impl ToSexpr for OtherTechnical {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("other-technical")
            .kwarg("value", &self.value)
            .kwarg_opt("placement", &self.placement);

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

impl FromSexpr for OtherTechnical {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("other-technical list", sexpr))?;

        expect_head(list, "other-technical")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg(list, "color")?;

        Ok(OtherTechnical {
            value: require_kwarg(list, "value")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

// ============================================================================
// TechnicalElement enum
// ============================================================================

impl ToSexpr for TechnicalElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            TechnicalElement::UpBow(ep) => {
                ListBuilder::new("up-bow")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::DownBow(ep) => {
                ListBuilder::new("down-bow")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Harmonic(h) => h.to_sexpr(),
            TechnicalElement::OpenString(ep) => {
                ListBuilder::new("open-string")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::ThumbPosition(ep) => {
                ListBuilder::new("thumb-position")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Fingering(f) => f.to_sexpr(),
            TechnicalElement::Pluck(p) => p.to_sexpr(),
            TechnicalElement::DoubleTongue(ep) => {
                ListBuilder::new("double-tongue")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::TripleTongue(ep) => {
                ListBuilder::new("triple-tongue")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Stopped(ep) => {
                ListBuilder::new("stopped")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::SnapPizzicato(ep) => {
                ListBuilder::new("snap-pizzicato")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Fret(f) => f.to_sexpr(),
            TechnicalElement::String(s) => s.to_sexpr(),
            TechnicalElement::HammerOn(hp) => {
                ListBuilder::new("hammer-on")
                    .kwarg_raw("data", hp.to_sexpr())
                    .build()
            }
            TechnicalElement::PullOff(hp) => {
                ListBuilder::new("pull-off")
                    .kwarg_raw("data", hp.to_sexpr())
                    .build()
            }
            TechnicalElement::Bend(b) => b.to_sexpr(),
            TechnicalElement::Tap(t) => t.to_sexpr(),
            TechnicalElement::Heel(ht) => {
                ListBuilder::new("heel")
                    .kwarg_raw("data", ht.to_sexpr())
                    .build()
            }
            TechnicalElement::Toe(ht) => {
                ListBuilder::new("toe")
                    .kwarg_raw("data", ht.to_sexpr())
                    .build()
            }
            TechnicalElement::Fingernails(ep) => {
                ListBuilder::new("fingernails")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Hole(h) => h.to_sexpr(),
            TechnicalElement::Arrow(a) => a.to_sexpr(),
            TechnicalElement::Handbell(h) => h.to_sexpr(),
            TechnicalElement::BrassBend(ep) => {
                ListBuilder::new("brass-bend")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Flip(ep) => {
                ListBuilder::new("flip")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Smear(ep) => {
                ListBuilder::new("smear")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::Open(ep) => {
                ListBuilder::new("open")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::HalfMuted(ep) => {
                ListBuilder::new("half-muted")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::HarmonMute(hm) => hm.to_sexpr(),
            TechnicalElement::Golpe(ep) => {
                ListBuilder::new("golpe")
                    .kwarg_raw("data", ep.to_sexpr())
                    .build()
            }
            TechnicalElement::OtherTechnical(ot) => ot.to_sexpr(),
        }
    }
}

impl FromSexpr for TechnicalElement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("technical-element", sexpr))?;

        match get_head(list)? {
            "up-bow" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::UpBow(data))
            }
            "down-bow" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::DownBow(data))
            }
            "harmonic" => Ok(TechnicalElement::Harmonic(Harmonic::from_sexpr(sexpr)?)),
            "open-string" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::OpenString(data))
            }
            "thumb-position" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::ThumbPosition(data))
            }
            "fingering" => Ok(TechnicalElement::Fingering(Fingering::from_sexpr(sexpr)?)),
            "pluck" => Ok(TechnicalElement::Pluck(Pluck::from_sexpr(sexpr)?)),
            "double-tongue" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::DoubleTongue(data))
            }
            "triple-tongue" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::TripleTongue(data))
            }
            "stopped" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Stopped(data))
            }
            "snap-pizzicato" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::SnapPizzicato(data))
            }
            "fret" => Ok(TechnicalElement::Fret(Fret::from_sexpr(sexpr)?)),
            "string-number" => Ok(TechnicalElement::String(StringNumber::from_sexpr(sexpr)?)),
            "hammer-on" => {
                let data: HammerPull = require_kwarg(list, "data")?;
                Ok(TechnicalElement::HammerOn(data))
            }
            "pull-off" => {
                let data: HammerPull = require_kwarg(list, "data")?;
                Ok(TechnicalElement::PullOff(data))
            }
            "bend" => Ok(TechnicalElement::Bend(Bend::from_sexpr(sexpr)?)),
            "tap" => Ok(TechnicalElement::Tap(Tap::from_sexpr(sexpr)?)),
            "heel" => {
                let data: HeelToe = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Heel(data))
            }
            "toe" => {
                let data: HeelToe = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Toe(data))
            }
            "fingernails" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Fingernails(data))
            }
            "hole" => Ok(TechnicalElement::Hole(Hole::from_sexpr(sexpr)?)),
            "arrow" => Ok(TechnicalElement::Arrow(Arrow::from_sexpr(sexpr)?)),
            "handbell" => Ok(TechnicalElement::Handbell(Handbell::from_sexpr(sexpr)?)),
            "brass-bend" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::BrassBend(data))
            }
            "flip" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Flip(data))
            }
            "smear" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Smear(data))
            }
            "open" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Open(data))
            }
            "half-muted" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::HalfMuted(data))
            }
            "harmon-mute" => Ok(TechnicalElement::HarmonMute(HarmonMute::from_sexpr(sexpr)?)),
            "golpe" => {
                let data: EmptyPlacement = require_kwarg(list, "data")?;
                Ok(TechnicalElement::Golpe(data))
            }
            "other-technical" => Ok(TechnicalElement::OtherTechnical(OtherTechnical::from_sexpr(
                sexpr,
            )?)),
            _ => Err(ConvertError::type_mismatch(
                "technical-element variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Technical container
// ============================================================================

impl ToSexpr for Technical {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("technical");

        if !self.content.is_empty() {
            builder = builder.kwarg_list("content", &self.content);
        }

        builder.build()
    }
}

impl FromSexpr for Technical {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("technical list", sexpr))?;

        expect_head(list, "technical")?;

        Ok(Technical {
            content: optional_kwarg::<Vec<TechnicalElement>>(list, "content")?.unwrap_or_default(),
        })
    }
}

// ============================================================================
// NotationContent enum
// ============================================================================

impl ToSexpr for NotationContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            NotationContent::Tied(t) => t.to_sexpr(),
            NotationContent::Slur(s) => s.to_sexpr(),
            NotationContent::Tuplet(t) => t.to_sexpr(),
            NotationContent::Glissando(g) => g.to_sexpr(),
            NotationContent::Slide(s) => s.to_sexpr(),
            NotationContent::Ornaments(o) => o.to_sexpr(),
            NotationContent::Technical(t) => t.to_sexpr(),
            NotationContent::Articulations(a) => a.to_sexpr(),
            NotationContent::Dynamics(d) => d.to_sexpr(),
            NotationContent::Fermata(f) => f.to_sexpr(),
            NotationContent::Arpeggiate(a) => a.to_sexpr(),
            NotationContent::NonArpeggiate(na) => na.to_sexpr(),
            NotationContent::AccidentalMark(am) => am.to_sexpr(),
            NotationContent::OtherNotation(on) => on.to_sexpr(),
        }
    }
}

impl FromSexpr for NotationContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("notation-content", sexpr))?;

        match get_head(list)? {
            "tied" => Ok(NotationContent::Tied(Tied::from_sexpr(sexpr)?)),
            "slur" => Ok(NotationContent::Slur(Slur::from_sexpr(sexpr)?)),
            "tuplet" => Ok(NotationContent::Tuplet(Box::new(Tuplet::from_sexpr(sexpr)?))),
            "glissando" => Ok(NotationContent::Glissando(Glissando::from_sexpr(sexpr)?)),
            "slide" => Ok(NotationContent::Slide(Slide::from_sexpr(sexpr)?)),
            "ornaments" => Ok(NotationContent::Ornaments(Box::new(Ornaments::from_sexpr(
                sexpr,
            )?))),
            "technical" => Ok(NotationContent::Technical(Box::new(Technical::from_sexpr(
                sexpr,
            )?))),
            "articulations" => Ok(NotationContent::Articulations(Box::new(
                Articulations::from_sexpr(sexpr)?,
            ))),
            "dynamics" => {
                use crate::ir::direction::Dynamics;
                Ok(NotationContent::Dynamics(Box::new(Dynamics::from_sexpr(
                    sexpr,
                )?)))
            }
            "fermata" => {
                use crate::ir::notation::Fermata;
                Ok(NotationContent::Fermata(Fermata::from_sexpr(sexpr)?))
            }
            "arpeggiate" => Ok(NotationContent::Arpeggiate(Arpeggiate::from_sexpr(sexpr)?)),
            "non-arpeggiate" => Ok(NotationContent::NonArpeggiate(NonArpeggiate::from_sexpr(
                sexpr,
            )?)),
            "accidental-mark" => Ok(NotationContent::AccidentalMark(AccidentalMark::from_sexpr(
                sexpr,
            )?)),
            "other-notation" => Ok(NotationContent::OtherNotation(OtherNotation::from_sexpr(
                sexpr,
            )?)),
            _ => Err(ConvertError::type_mismatch(
                "notation-content variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Notations container
// ============================================================================

impl ToSexpr for Notations {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("notations").kwarg_opt("print-object", &self.print_object);

        if !self.content.is_empty() {
            builder = builder.kwarg_list("content", &self.content);
        }

        builder.build()
    }
}

impl FromSexpr for Notations {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("notations list", sexpr))?;

        expect_head(list, "notations")?;

        use crate::ir::common::Editorial;

        Ok(Notations {
            print_object: optional_kwarg(list, "print-object")?,
            content: optional_kwarg::<Vec<NotationContent>>(list, "content")?.unwrap_or_default(),
            editorial: Editorial::default(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{
        AboveBelow, AccidentalValue, LineType, OverUnder, StartStop, StartStopContinue, UpDown,
        YesNo,
    };
    use crate::sexpr::print_sexpr;

    #[test]
    fn test_tied_round_trip() {
        let tied = Tied {
            r#type: StartStopContinue::Start,
            number: Some(1),
            line_type: None,
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            orientation: None,
            color: None,
        };

        let sexpr = tied.to_sexpr();
        let parsed = Tied::from_sexpr(&sexpr).unwrap();
        assert_eq!(tied.r#type, parsed.r#type);
        assert_eq!(tied.number, parsed.number);
    }

    #[test]
    fn test_slur_round_trip() {
        let slur = Slur {
            r#type: StartStopContinue::Start,
            number: 1,
            line_type: Some(LineType::Solid),
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            orientation: Some(OverUnder::Over),
            color: None,
        };

        let sexpr = slur.to_sexpr();
        let parsed = Slur::from_sexpr(&sexpr).unwrap();
        assert_eq!(slur.number, parsed.number);
        assert_eq!(slur.placement, parsed.placement);
    }

    #[test]
    fn test_tuplet_round_trip() {
        let tuplet = Tuplet {
            r#type: StartStop::Start,
            number: Some(1),
            bracket: Some(YesNo::Yes),
            show_number: Some(ShowTuplet::Actual),
            show_type: None,
            line_shape: Some(LineShape::Curved),
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            tuplet_actual: None,
            tuplet_normal: None,
        };

        let sexpr = tuplet.to_sexpr();
        let parsed = Tuplet::from_sexpr(&sexpr).unwrap();
        assert_eq!(tuplet.r#type, parsed.r#type);
        assert_eq!(tuplet.bracket, parsed.bracket);
    }

    #[test]
    fn test_arpeggiate_round_trip() {
        let arp = Arpeggiate {
            number: Some(1),
            direction: Some(UpDown::Up),
            position: Position::default(),
            color: None,
        };

        let sexpr = arp.to_sexpr();
        let parsed = Arpeggiate::from_sexpr(&sexpr).unwrap();
        assert_eq!(arp.direction, parsed.direction);
    }

    #[test]
    fn test_articulations_round_trip() {
        let art = Articulations {
            content: vec![
                ArticulationElement::Accent(EmptyPlacement::default()),
                ArticulationElement::Staccato(EmptyPlacement::default()),
            ],
        };

        let sexpr = art.to_sexpr();
        let parsed = Articulations::from_sexpr(&sexpr).unwrap();
        assert_eq!(art.content.len(), parsed.content.len());
    }

    #[test]
    fn test_ornaments_round_trip() {
        let orn = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::TrillMark(EmptyTrillSound::default()),
                accidental_marks: vec![],
            }],
        };

        let sexpr = orn.to_sexpr();
        let parsed = Ornaments::from_sexpr(&sexpr).unwrap();
        assert_eq!(orn.content.len(), parsed.content.len());
    }

    #[test]
    fn test_technical_round_trip() {
        let tech = Technical {
            content: vec![TechnicalElement::UpBow(EmptyPlacement::default())],
        };

        let sexpr = tech.to_sexpr();
        let parsed = Technical::from_sexpr(&sexpr).unwrap();
        assert_eq!(tech.content.len(), parsed.content.len());
    }

    #[test]
    fn test_notations_round_trip() {
        use crate::ir::common::Editorial;
        use crate::ir::notation::Fermata;

        let notations = Notations {
            print_object: Some(YesNo::Yes),
            content: vec![NotationContent::Fermata(Fermata::default())],
            editorial: Editorial::default(),
        };

        let sexpr = notations.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("notations"));

        let parsed = Notations::from_sexpr(&sexpr).unwrap();
        assert_eq!(notations.print_object, parsed.print_object);
        assert_eq!(notations.content.len(), parsed.content.len());
    }

    #[test]
    fn test_accidental_mark_round_trip() {
        let mark = AccidentalMark {
            value: AccidentalValue::Sharp,
            placement: Some(AboveBelow::Above),
            print_style: PrintStyle::default(),
        };

        let sexpr = mark.to_sexpr();
        let parsed = AccidentalMark::from_sexpr(&sexpr).unwrap();
        assert_eq!(mark.value, parsed.value);
    }

    #[test]
    fn test_breath_mark_round_trip() {
        let bm = BreathMark {
            value: BreathMarkValue::Comma,
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };

        let sexpr = bm.to_sexpr();
        let parsed = BreathMark::from_sexpr(&sexpr).unwrap();
        assert_eq!(bm.value, parsed.value);
    }

    #[test]
    fn test_tremolo_round_trip() {
        let trem = Tremolo {
            value: 3,
            r#type: Some(TremoloType::Single),
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };

        let sexpr = trem.to_sexpr();
        let parsed = Tremolo::from_sexpr(&sexpr).unwrap();
        assert_eq!(trem.value, parsed.value);
        assert_eq!(trem.r#type, parsed.r#type);
    }

    #[test]
    fn test_fingering_round_trip() {
        let fing = Fingering {
            value: "1".to_string(),
            substitution: Some(YesNo::No),
            alternate: None,
            placement: Some(AboveBelow::Above),
            print_style: PrintStyle::default(),
        };

        let sexpr = fing.to_sexpr();
        let parsed = Fingering::from_sexpr(&sexpr).unwrap();
        assert_eq!(fing.value, parsed.value);
    }
}
