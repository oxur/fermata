//! S-expression conversions for `ir::notation` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for notation-related
//! types including articulations, ornaments, technical markings, slurs, ties,
//! tuplets, and other notation elements attached to notes.

use crate::ir::common::{EmptyPlacement, Position, PrintStyle};
use crate::ir::notation::{
    AccidentalMark, Arpeggiate, Arrow, ArrowDirection, ArrowStyle, ArticulationElement,
    Articulations, Bend, BendRelease, BreathMark, BreathMarkValue, Caesura, CaesuraValue,
    EmptyLine, EmptyTrillSound, Fingering, Fret, Glissando, HammerPull, Handbell, HandbellValue,
    HarmonMute, Harmonic, HeelToe, Hole, HoleClosed, HoleClosedLocation, HoleClosedValue,
    LineLength, LineShape, Mordent, NonArpeggiate, NotationContent, Notations, OrnamentElement,
    OrnamentWithAccidentals, Ornaments, OtherArticulation, OtherNotation, OtherOrnament,
    OtherTechnical, Pluck, ShowTuplet, Slide, Slur, StartNote, StringNumber, StrongAccent, Tap,
    TapHand, Technical, TechnicalElement, Tied, TopBottom, Tremolo, TremoloType, TrillStep, Tuplet,
    TupletDot, TupletNumber, TupletPortion, TupletType, Turn, TwoNoteTurn,
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
            ArticulationElement::Accent(ep) => ListBuilder::new("accent")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::StrongAccent(sa) => sa.to_sexpr(),
            ArticulationElement::Staccato(ep) => ListBuilder::new("staccato")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::Tenuto(ep) => ListBuilder::new("tenuto")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::DetachedLegato(ep) => ListBuilder::new("detached-legato")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::Staccatissimo(ep) => ListBuilder::new("staccatissimo")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::Spiccato(ep) => ListBuilder::new("spiccato")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::Scoop(el) => ListBuilder::new("scoop")
                .kwarg_raw("data", el.to_sexpr())
                .build(),
            ArticulationElement::Plop(el) => ListBuilder::new("plop")
                .kwarg_raw("data", el.to_sexpr())
                .build(),
            ArticulationElement::Doit(el) => ListBuilder::new("doit")
                .kwarg_raw("data", el.to_sexpr())
                .build(),
            ArticulationElement::Falloff(el) => ListBuilder::new("falloff")
                .kwarg_raw("data", el.to_sexpr())
                .build(),
            ArticulationElement::BreathMark(bm) => bm.to_sexpr(),
            ArticulationElement::Caesura(c) => c.to_sexpr(),
            ArticulationElement::Stress(ep) => ListBuilder::new("stress")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::Unstress(ep) => ListBuilder::new("unstress")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            ArticulationElement::SoftAccent(ep) => ListBuilder::new("soft-accent")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
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
            content: optional_kwarg::<Vec<ArticulationElement>>(list, "content")?
                .unwrap_or_default(),
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
            OrnamentElement::TrillMark(ets) => ListBuilder::new("trill-mark")
                .kwarg_raw("data", ets.to_sexpr())
                .build(),
            OrnamentElement::Turn(t) => t.to_sexpr(),
            OrnamentElement::DelayedTurn(t) => ListBuilder::new("delayed-turn")
                .kwarg_raw("data", t.to_sexpr())
                .build(),
            OrnamentElement::InvertedTurn(t) => ListBuilder::new("inverted-turn")
                .kwarg_raw("data", t.to_sexpr())
                .build(),
            OrnamentElement::DelayedInvertedTurn(t) => ListBuilder::new("delayed-inverted-turn")
                .kwarg_raw("data", t.to_sexpr())
                .build(),
            OrnamentElement::VerticalTurn(ets) => ListBuilder::new("vertical-turn")
                .kwarg_raw("data", ets.to_sexpr())
                .build(),
            OrnamentElement::InvertedVerticalTurn(ets) => {
                ListBuilder::new("inverted-vertical-turn")
                    .kwarg_raw("data", ets.to_sexpr())
                    .build()
            }
            OrnamentElement::Shake(ets) => ListBuilder::new("shake")
                .kwarg_raw("data", ets.to_sexpr())
                .build(),
            OrnamentElement::WavyLine(wl) => ListBuilder::new("wavy-line")
                .kwarg_raw("data", wl.to_sexpr())
                .build(),
            OrnamentElement::Mordent(m) => m.to_sexpr(),
            OrnamentElement::InvertedMordent(m) => ListBuilder::new("inverted-mordent")
                .kwarg_raw("data", m.to_sexpr())
                .build(),
            OrnamentElement::Schleifer(ep) => ListBuilder::new("schleifer")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            OrnamentElement::Tremolo(t) => t.to_sexpr(),
            OrnamentElement::Haydn(ets) => ListBuilder::new("haydn")
                .kwarg_raw("data", ets.to_sexpr())
                .build(),
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
            _ => Err(ConvertError::type_mismatch(
                "ornament-element variant",
                sexpr,
            )),
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
            TechnicalElement::UpBow(ep) => ListBuilder::new("up-bow")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::DownBow(ep) => ListBuilder::new("down-bow")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Harmonic(h) => h.to_sexpr(),
            TechnicalElement::OpenString(ep) => ListBuilder::new("open-string")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::ThumbPosition(ep) => ListBuilder::new("thumb-position")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Fingering(f) => f.to_sexpr(),
            TechnicalElement::Pluck(p) => p.to_sexpr(),
            TechnicalElement::DoubleTongue(ep) => ListBuilder::new("double-tongue")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::TripleTongue(ep) => ListBuilder::new("triple-tongue")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Stopped(ep) => ListBuilder::new("stopped")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::SnapPizzicato(ep) => ListBuilder::new("snap-pizzicato")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Fret(f) => f.to_sexpr(),
            TechnicalElement::String(s) => s.to_sexpr(),
            TechnicalElement::HammerOn(hp) => ListBuilder::new("hammer-on")
                .kwarg_raw("data", hp.to_sexpr())
                .build(),
            TechnicalElement::PullOff(hp) => ListBuilder::new("pull-off")
                .kwarg_raw("data", hp.to_sexpr())
                .build(),
            TechnicalElement::Bend(b) => b.to_sexpr(),
            TechnicalElement::Tap(t) => t.to_sexpr(),
            TechnicalElement::Heel(ht) => ListBuilder::new("heel")
                .kwarg_raw("data", ht.to_sexpr())
                .build(),
            TechnicalElement::Toe(ht) => ListBuilder::new("toe")
                .kwarg_raw("data", ht.to_sexpr())
                .build(),
            TechnicalElement::Fingernails(ep) => ListBuilder::new("fingernails")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Hole(h) => h.to_sexpr(),
            TechnicalElement::Arrow(a) => a.to_sexpr(),
            TechnicalElement::Handbell(h) => h.to_sexpr(),
            TechnicalElement::BrassBend(ep) => ListBuilder::new("brass-bend")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Flip(ep) => ListBuilder::new("flip")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Smear(ep) => ListBuilder::new("smear")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::Open(ep) => ListBuilder::new("open")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::HalfMuted(ep) => ListBuilder::new("half-muted")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
            TechnicalElement::HarmonMute(hm) => hm.to_sexpr(),
            TechnicalElement::Golpe(ep) => ListBuilder::new("golpe")
                .kwarg_raw("data", ep.to_sexpr())
                .build(),
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
            "other-technical" => Ok(TechnicalElement::OtherTechnical(
                OtherTechnical::from_sexpr(sexpr)?,
            )),
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
            "tuplet" => Ok(NotationContent::Tuplet(Box::new(Tuplet::from_sexpr(
                sexpr,
            )?))),
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
        let mut builder =
            ListBuilder::new("notations").kwarg_opt("print-object", &self.print_object);

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

    // ========================================================================
    // Simple Enum Tests - All Variants
    // ========================================================================

    #[test]
    fn test_show_tuplet_all_variants() {
        // Test all variants round-trip
        for (variant, expected_str) in [
            (ShowTuplet::Actual, "actual"),
            (ShowTuplet::Both, "both"),
            (ShowTuplet::None, "none"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = ShowTuplet::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_show_tuplet_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(ShowTuplet::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_line_shape_all_variants() {
        for (variant, expected_str) in [
            (LineShape::Straight, "straight"),
            (LineShape::Curved, "curved"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = LineShape::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_line_shape_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(LineShape::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_top_bottom_all_variants() {
        for (variant, expected_str) in [(TopBottom::Top, "top"), (TopBottom::Bottom, "bottom")] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = TopBottom::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_top_bottom_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(TopBottom::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_line_length_all_variants() {
        for (variant, expected_str) in [
            (LineLength::Short, "short"),
            (LineLength::Medium, "medium"),
            (LineLength::Long, "long"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = LineLength::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_line_length_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(LineLength::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_breath_mark_value_all_variants() {
        for (variant, expected_str) in [
            (BreathMarkValue::Empty, "empty"),
            (BreathMarkValue::Comma, "comma"),
            (BreathMarkValue::Tick, "tick"),
            (BreathMarkValue::Upbow, "upbow"),
            (BreathMarkValue::Salzedo, "salzedo"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = BreathMarkValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_breath_mark_value_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(BreathMarkValue::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_caesura_value_all_variants() {
        for (variant, expected_str) in [
            (CaesuraValue::Normal, "normal"),
            (CaesuraValue::Thick, "thick"),
            (CaesuraValue::Short, "short"),
            (CaesuraValue::Curved, "curved"),
            (CaesuraValue::Single, "single"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = CaesuraValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_caesura_value_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(CaesuraValue::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_start_note_all_variants() {
        for (variant, expected_str) in [
            (StartNote::Upper, "upper"),
            (StartNote::Main, "main"),
            (StartNote::Below, "below"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = StartNote::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_start_note_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(StartNote::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_trill_step_all_variants() {
        for (variant, expected_str) in [
            (TrillStep::Whole, "whole"),
            (TrillStep::Half, "half"),
            (TrillStep::Unison, "unison"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = TrillStep::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_trill_step_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(TrillStep::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_two_note_turn_all_variants() {
        for (variant, expected_str) in [
            (TwoNoteTurn::Whole, "whole"),
            (TwoNoteTurn::Half, "half"),
            (TwoNoteTurn::None, "none"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = TwoNoteTurn::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_two_note_turn_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(TwoNoteTurn::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_tremolo_type_all_variants() {
        for (variant, expected_str) in [
            (TremoloType::Start, "start"),
            (TremoloType::Stop, "stop"),
            (TremoloType::Single, "single"),
            (TremoloType::Unmeasured, "unmeasured"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = TremoloType::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_tremolo_type_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(TremoloType::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_bend_release_all_variants() {
        for (variant, expected_str) in [(BendRelease::Early, "early"), (BendRelease::Late, "late")]
        {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = BendRelease::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_bend_release_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(BendRelease::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_tap_hand_all_variants() {
        for (variant, expected_str) in [(TapHand::Left, "left"), (TapHand::Right, "right")] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = TapHand::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_tap_hand_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(TapHand::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_hole_closed_value_all_variants() {
        for (variant, expected_str) in [
            (HoleClosedValue::Yes, "yes"),
            (HoleClosedValue::No, "no"),
            (HoleClosedValue::Half, "half"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = HoleClosedValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_hole_closed_value_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(HoleClosedValue::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_hole_closed_location_all_variants() {
        for (variant, expected_str) in [
            (HoleClosedLocation::Right, "right"),
            (HoleClosedLocation::Bottom, "bottom"),
            (HoleClosedLocation::Left, "left"),
            (HoleClosedLocation::Top, "top"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = HoleClosedLocation::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_hole_closed_location_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(HoleClosedLocation::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_arrow_direction_all_variants() {
        for (variant, expected_str) in [
            (ArrowDirection::Left, "left"),
            (ArrowDirection::Up, "up"),
            (ArrowDirection::Right, "right"),
            (ArrowDirection::Down, "down"),
            (ArrowDirection::Northwest, "northwest"),
            (ArrowDirection::Northeast, "northeast"),
            (ArrowDirection::Southeast, "southeast"),
            (ArrowDirection::Southwest, "southwest"),
            (ArrowDirection::LeftRight, "left-right"),
            (ArrowDirection::UpDown, "up-down"),
            (ArrowDirection::NorthwestSoutheast, "northwest-southeast"),
            (ArrowDirection::NortheastSouthwest, "northeast-southwest"),
            (ArrowDirection::Other, "other"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = ArrowDirection::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_arrow_direction_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(ArrowDirection::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_arrow_style_all_variants() {
        for (variant, expected_str) in [
            (ArrowStyle::Single, "single"),
            (ArrowStyle::Double, "double"),
            (ArrowStyle::Filled, "filled"),
            (ArrowStyle::Hollow, "hollow"),
            (ArrowStyle::Paired, "paired"),
            (ArrowStyle::Combined, "combined"),
            (ArrowStyle::Other, "other"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = ArrowStyle::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_arrow_style_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(ArrowStyle::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_handbell_value_all_variants() {
        for (variant, expected_str) in [
            (HandbellValue::Belltree, "belltree"),
            (HandbellValue::Damp, "damp"),
            (HandbellValue::Echo, "echo"),
            (HandbellValue::Gyro, "gyro"),
            (HandbellValue::HandMartellato, "hand-martellato"),
            (HandbellValue::MalletLift, "mallet-lift"),
            (HandbellValue::MalletTable, "mallet-table"),
            (HandbellValue::Martellato, "martellato"),
            (HandbellValue::MartellatoLift, "martellato-lift"),
            (HandbellValue::MutedMartellato, "muted-martellato"),
            (HandbellValue::PluckLift, "pluck-lift"),
            (HandbellValue::Swing, "swing"),
        ] {
            let sexpr = variant.to_sexpr();
            assert_eq!(sexpr.as_symbol(), Some(expected_str));
            let parsed = HandbellValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_handbell_value_error_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(HandbellValue::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // EmptyPlacement Tests
    // ========================================================================

    #[test]
    fn test_empty_placement_minimal() {
        let ep = EmptyPlacement::default();
        let sexpr = ep.to_sexpr();
        let parsed = EmptyPlacement::from_sexpr(&sexpr).unwrap();
        assert_eq!(ep, parsed);
    }

    #[test]
    fn test_empty_placement_with_placement() {
        let ep = EmptyPlacement {
            placement: Some(AboveBelow::Below),
            position: Position::default(),
        };
        let sexpr = ep.to_sexpr();
        let parsed = EmptyPlacement::from_sexpr(&sexpr).unwrap();
        assert_eq!(ep.placement, parsed.placement);
    }

    #[test]
    fn test_empty_placement_with_position() {
        let ep = EmptyPlacement {
            placement: None,
            position: Position {
                default_x: Some(10.0),
                default_y: Some(20.0),
                relative_x: None,
                relative_y: None,
            },
        };
        let sexpr = ep.to_sexpr();
        let parsed = EmptyPlacement::from_sexpr(&sexpr).unwrap();
        assert_eq!(ep.position.default_x, parsed.position.default_x);
        assert_eq!(ep.position.default_y, parsed.position.default_y);
    }

    #[test]
    fn test_empty_placement_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(EmptyPlacement::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Tied Tests
    // ========================================================================

    #[test]
    fn test_tied_with_position() {
        let tied = Tied {
            r#type: StartStopContinue::Stop,
            number: None,
            line_type: Some(LineType::Dashed),
            position: Position {
                default_x: Some(5.0),
                default_y: None,
                relative_x: Some(2.0),
                relative_y: None,
            },
            placement: None,
            orientation: Some(OverUnder::Under),
            color: Some("#FF0000".to_string()),
        };
        let sexpr = tied.to_sexpr();
        let parsed = Tied::from_sexpr(&sexpr).unwrap();
        assert_eq!(tied.r#type, parsed.r#type);
        assert_eq!(tied.line_type, parsed.line_type);
        assert_eq!(tied.orientation, parsed.orientation);
        assert_eq!(tied.color, parsed.color);
    }

    #[test]
    fn test_tied_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Tied::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Slur Tests
    // ========================================================================

    #[test]
    fn test_slur_with_position() {
        let slur = Slur {
            r#type: StartStopContinue::Continue,
            number: 2,
            line_type: Some(LineType::Dotted),
            position: Position {
                default_x: None,
                default_y: Some(15.0),
                relative_x: None,
                relative_y: Some(3.0),
            },
            placement: Some(AboveBelow::Below),
            orientation: None,
            color: Some("#00FF00".to_string()),
        };
        let sexpr = slur.to_sexpr();
        let parsed = Slur::from_sexpr(&sexpr).unwrap();
        assert_eq!(slur.r#type, parsed.r#type);
        assert_eq!(slur.number, parsed.number);
        assert_eq!(slur.color, parsed.color);
    }

    #[test]
    fn test_slur_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Slur::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Glissando Tests
    // ========================================================================

    #[test]
    fn test_glissando_round_trip() {
        let gliss = Glissando {
            r#type: StartStop::Start,
            number: Some(1),
            text: Some("gliss.".to_string()),
            line_type: Some(LineType::Wavy),
            position: Position::default(),
        };
        let sexpr = gliss.to_sexpr();
        let parsed = Glissando::from_sexpr(&sexpr).unwrap();
        assert_eq!(gliss.r#type, parsed.r#type);
        assert_eq!(gliss.text, parsed.text);
    }

    #[test]
    fn test_glissando_with_position() {
        let gliss = Glissando {
            r#type: StartStop::Stop,
            number: None,
            text: None,
            line_type: None,
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: Some(3.0),
                relative_y: Some(4.0),
            },
        };
        let sexpr = gliss.to_sexpr();
        let parsed = Glissando::from_sexpr(&sexpr).unwrap();
        assert_eq!(gliss.position.default_x, parsed.position.default_x);
    }

    #[test]
    fn test_glissando_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Glissando::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Slide Tests
    // ========================================================================

    #[test]
    fn test_slide_round_trip() {
        let slide = Slide {
            r#type: StartStop::Start,
            number: Some(1),
            text: Some("slide".to_string()),
            line_type: Some(LineType::Solid),
            position: Position::default(),
        };
        let sexpr = slide.to_sexpr();
        let parsed = Slide::from_sexpr(&sexpr).unwrap();
        assert_eq!(slide.r#type, parsed.r#type);
        assert_eq!(slide.text, parsed.text);
    }

    #[test]
    fn test_slide_with_position() {
        let slide = Slide {
            r#type: StartStop::Stop,
            number: None,
            text: None,
            line_type: None,
            position: Position {
                default_x: Some(10.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(5.0),
            },
        };
        let sexpr = slide.to_sexpr();
        let parsed = Slide::from_sexpr(&sexpr).unwrap();
        assert_eq!(slide.position.relative_y, parsed.position.relative_y);
    }

    #[test]
    fn test_slide_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Slide::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // TupletNumber Tests
    // ========================================================================

    #[test]
    fn test_tuplet_number_round_trip() {
        use crate::ir::common::Font;
        let tn = TupletNumber {
            value: 3,
            font: Font::default(),
            color: None,
        };
        let sexpr = tn.to_sexpr();
        let parsed = TupletNumber::from_sexpr(&sexpr).unwrap();
        assert_eq!(tn.value, parsed.value);
    }

    #[test]
    fn test_tuplet_number_with_font() {
        use crate::ir::common::{Font, FontStyle};
        let tn = TupletNumber {
            value: 5,
            font: Font {
                font_family: Some("Times".to_string()),
                font_style: Some(FontStyle::Italic),
                font_size: None,
                font_weight: None,
            },
            color: Some("#0000FF".to_string()),
        };
        let sexpr = tn.to_sexpr();
        let parsed = TupletNumber::from_sexpr(&sexpr).unwrap();
        assert_eq!(tn.value, parsed.value);
        assert_eq!(tn.color, parsed.color);
    }

    #[test]
    fn test_tuplet_number_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(TupletNumber::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // TupletType Tests
    // ========================================================================

    #[test]
    fn test_tuplet_type_round_trip() {
        use crate::ir::common::Font;
        use crate::ir::duration::NoteTypeValue;
        let tt = TupletType {
            value: NoteTypeValue::Eighth,
            font: Font::default(),
            color: None,
        };
        let sexpr = tt.to_sexpr();
        let parsed = TupletType::from_sexpr(&sexpr).unwrap();
        assert_eq!(tt.value, parsed.value);
    }

    #[test]
    fn test_tuplet_type_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(TupletType::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // TupletDot Tests
    // ========================================================================

    #[test]
    fn test_tuplet_dot_round_trip() {
        let td = TupletDot::default();
        let sexpr = td.to_sexpr();
        let parsed = TupletDot::from_sexpr(&sexpr).unwrap();
        assert_eq!(td, parsed);
    }

    #[test]
    fn test_tuplet_dot_with_color() {
        use crate::ir::common::Font;
        let td = TupletDot {
            font: Font::default(),
            color: Some("#FF00FF".to_string()),
        };
        let sexpr = td.to_sexpr();
        let parsed = TupletDot::from_sexpr(&sexpr).unwrap();
        assert_eq!(td.color, parsed.color);
    }

    #[test]
    fn test_tuplet_dot_with_font() {
        use crate::ir::common::{Font, FontWeight};
        let td = TupletDot {
            font: Font {
                font_family: None,
                font_style: None,
                font_size: None,
                font_weight: Some(FontWeight::Bold),
            },
            color: None,
        };
        let sexpr = td.to_sexpr();
        let parsed = TupletDot::from_sexpr(&sexpr).unwrap();
        assert_eq!(td.font.font_weight, parsed.font.font_weight);
    }

    #[test]
    fn test_tuplet_dot_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(TupletDot::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // TupletPortion Tests
    // ========================================================================

    #[test]
    fn test_tuplet_portion_empty() {
        let tp = TupletPortion {
            tuplet_number: None,
            tuplet_type: None,
            tuplet_dots: vec![],
        };
        let sexpr = tp.to_sexpr();
        let parsed = TupletPortion::from_sexpr(&sexpr).unwrap();
        assert!(parsed.tuplet_number.is_none());
        assert!(parsed.tuplet_type.is_none());
        assert!(parsed.tuplet_dots.is_empty());
    }

    #[test]
    fn test_tuplet_portion_with_all() {
        use crate::ir::common::Font;
        use crate::ir::duration::NoteTypeValue;
        let tp = TupletPortion {
            tuplet_number: Some(TupletNumber {
                value: 3,
                font: Font::default(),
                color: None,
            }),
            tuplet_type: Some(TupletType {
                value: NoteTypeValue::Quarter,
                font: Font::default(),
                color: None,
            }),
            tuplet_dots: vec![TupletDot::default()],
        };
        let sexpr = tp.to_sexpr();
        let parsed = TupletPortion::from_sexpr(&sexpr).unwrap();
        assert!(parsed.tuplet_number.is_some());
        assert!(parsed.tuplet_type.is_some());
        assert_eq!(1, parsed.tuplet_dots.len());
    }

    #[test]
    fn test_tuplet_portion_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(TupletPortion::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Tuplet Tests (comprehensive)
    // ========================================================================

    #[test]
    fn test_tuplet_with_portions() {
        use crate::ir::common::Font;
        use crate::ir::duration::NoteTypeValue;
        let tuplet = Tuplet {
            r#type: StartStop::Start,
            number: Some(1),
            bracket: Some(YesNo::Yes),
            show_number: Some(ShowTuplet::Both),
            show_type: Some(ShowTuplet::Actual),
            line_shape: Some(LineShape::Straight),
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: None,
                relative_y: None,
            },
            placement: Some(AboveBelow::Above),
            tuplet_actual: Some(TupletPortion {
                tuplet_number: Some(TupletNumber {
                    value: 3,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_type: Some(TupletType {
                    value: NoteTypeValue::Eighth,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_dots: vec![],
            }),
            tuplet_normal: Some(TupletPortion {
                tuplet_number: Some(TupletNumber {
                    value: 2,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_type: None,
                tuplet_dots: vec![],
            }),
        };
        let sexpr = tuplet.to_sexpr();
        let parsed = Tuplet::from_sexpr(&sexpr).unwrap();
        assert_eq!(tuplet.r#type, parsed.r#type);
        assert!(parsed.tuplet_actual.is_some());
        assert!(parsed.tuplet_normal.is_some());
    }

    #[test]
    fn test_tuplet_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Tuplet::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Arpeggiate Tests (comprehensive)
    // ========================================================================

    #[test]
    fn test_arpeggiate_with_position() {
        let arp = Arpeggiate {
            number: Some(2),
            direction: Some(UpDown::Down),
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: Some(3.0),
                relative_y: Some(4.0),
            },
            color: Some("#123456".to_string()),
        };
        let sexpr = arp.to_sexpr();
        let parsed = Arpeggiate::from_sexpr(&sexpr).unwrap();
        assert_eq!(arp.number, parsed.number);
        assert_eq!(arp.direction, parsed.direction);
        assert_eq!(arp.color, parsed.color);
    }

    #[test]
    fn test_arpeggiate_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Arpeggiate::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // NonArpeggiate Tests
    // ========================================================================

    #[test]
    fn test_non_arpeggiate_round_trip() {
        let na = NonArpeggiate {
            r#type: TopBottom::Bottom,
            number: Some(1),
            position: Position::default(),
            color: None,
        };
        let sexpr = na.to_sexpr();
        let parsed = NonArpeggiate::from_sexpr(&sexpr).unwrap();
        assert_eq!(na.r#type, parsed.r#type);
        assert_eq!(na.number, parsed.number);
    }

    #[test]
    fn test_non_arpeggiate_with_position() {
        let na = NonArpeggiate {
            r#type: TopBottom::Top,
            number: None,
            position: Position {
                default_x: Some(5.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(10.0),
            },
            color: Some("#AABBCC".to_string()),
        };
        let sexpr = na.to_sexpr();
        let parsed = NonArpeggiate::from_sexpr(&sexpr).unwrap();
        assert_eq!(na.position.default_x, parsed.position.default_x);
        assert_eq!(na.color, parsed.color);
    }

    #[test]
    fn test_non_arpeggiate_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(NonArpeggiate::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // AccidentalMark Tests (comprehensive)
    // ========================================================================

    #[test]
    fn test_accidental_mark_with_position() {
        let am = AccidentalMark {
            value: AccidentalValue::Flat,
            placement: Some(AboveBelow::Below),
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(1.0),
                    default_y: Some(2.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#FFFF00".to_string()),
            },
        };
        let sexpr = am.to_sexpr();
        let parsed = AccidentalMark::from_sexpr(&sexpr).unwrap();
        assert_eq!(am.value, parsed.value);
        assert_eq!(am.placement, parsed.placement);
        assert_eq!(am.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_accidental_mark_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(AccidentalMark::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // OtherNotation Tests
    // ========================================================================

    #[test]
    fn test_other_notation_round_trip() {
        use crate::ir::common::StartStopSingle;
        let on = OtherNotation {
            value: "custom".to_string(),
            r#type: StartStopSingle::Single,
            number: Some(1),
            print_object: Some(YesNo::Yes),
            print_style: PrintStyle::default(),
            placement: Some(AboveBelow::Above),
        };
        let sexpr = on.to_sexpr();
        let parsed = OtherNotation::from_sexpr(&sexpr).unwrap();
        assert_eq!(on.value, parsed.value);
        assert_eq!(on.r#type, parsed.r#type);
    }

    #[test]
    fn test_other_notation_with_position_color() {
        use crate::ir::common::StartStopSingle;
        let on = OtherNotation {
            value: "test".to_string(),
            r#type: StartStopSingle::Start,
            number: None,
            print_object: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#112233".to_string()),
            },
            placement: None,
        };
        let sexpr = on.to_sexpr();
        let parsed = OtherNotation::from_sexpr(&sexpr).unwrap();
        assert_eq!(on.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_other_notation_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(OtherNotation::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // StrongAccent Tests
    // ========================================================================

    #[test]
    fn test_strong_accent_round_trip() {
        let sa = StrongAccent {
            r#type: Some(UpDown::Up),
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };
        let sexpr = sa.to_sexpr();
        let parsed = StrongAccent::from_sexpr(&sexpr).unwrap();
        assert_eq!(sa.r#type, parsed.r#type);
        assert_eq!(sa.placement, parsed.placement);
    }

    #[test]
    fn test_strong_accent_with_position() {
        let sa = StrongAccent {
            r#type: Some(UpDown::Down),
            placement: None,
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: Some(1.0),
                relative_y: Some(2.0),
            },
        };
        let sexpr = sa.to_sexpr();
        let parsed = StrongAccent::from_sexpr(&sexpr).unwrap();
        assert_eq!(sa.position.default_x, parsed.position.default_x);
    }

    #[test]
    fn test_strong_accent_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(StrongAccent::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // EmptyLine Tests
    // ========================================================================

    #[test]
    fn test_empty_line_round_trip() {
        let el = EmptyLine {
            line_shape: Some(LineShape::Curved),
            line_type: Some(LineType::Dashed),
            line_length: Some(LineLength::Medium),
            placement: Some(AboveBelow::Below),
            position: Position::default(),
        };
        let sexpr = el.to_sexpr();
        let parsed = EmptyLine::from_sexpr(&sexpr).unwrap();
        assert_eq!(el.line_shape, parsed.line_shape);
        assert_eq!(el.line_type, parsed.line_type);
        assert_eq!(el.line_length, parsed.line_length);
    }

    #[test]
    fn test_empty_line_with_position() {
        let el = EmptyLine {
            line_shape: None,
            line_type: None,
            line_length: None,
            placement: None,
            position: Position {
                default_x: Some(1.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(2.0),
            },
        };
        let sexpr = el.to_sexpr();
        let parsed = EmptyLine::from_sexpr(&sexpr).unwrap();
        assert_eq!(el.position.default_x, parsed.position.default_x);
    }

    #[test]
    fn test_empty_line_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(EmptyLine::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // BreathMark Tests (comprehensive)
    // ========================================================================

    #[test]
    fn test_breath_mark_all_values() {
        for value in [
            BreathMarkValue::Empty,
            BreathMarkValue::Comma,
            BreathMarkValue::Tick,
            BreathMarkValue::Upbow,
            BreathMarkValue::Salzedo,
        ] {
            let bm = BreathMark {
                value,
                placement: Some(AboveBelow::Above),
                position: Position::default(),
            };
            let sexpr = bm.to_sexpr();
            let parsed = BreathMark::from_sexpr(&sexpr).unwrap();
            assert_eq!(bm.value, parsed.value);
        }
    }

    #[test]
    fn test_breath_mark_with_position() {
        let bm = BreathMark {
            value: BreathMarkValue::Tick,
            placement: None,
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: None,
                relative_y: None,
            },
        };
        let sexpr = bm.to_sexpr();
        let parsed = BreathMark::from_sexpr(&sexpr).unwrap();
        assert_eq!(bm.position.default_x, parsed.position.default_x);
    }

    #[test]
    fn test_breath_mark_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(BreathMark::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Caesura Tests
    // ========================================================================

    #[test]
    fn test_caesura_round_trip() {
        let c = Caesura {
            value: CaesuraValue::Thick,
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };
        let sexpr = c.to_sexpr();
        let parsed = Caesura::from_sexpr(&sexpr).unwrap();
        assert_eq!(c.value, parsed.value);
        assert_eq!(c.placement, parsed.placement);
    }

    #[test]
    fn test_caesura_with_position() {
        let c = Caesura {
            value: CaesuraValue::Curved,
            placement: None,
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: Some(3.0),
                relative_y: Some(4.0),
            },
        };
        let sexpr = c.to_sexpr();
        let parsed = Caesura::from_sexpr(&sexpr).unwrap();
        assert_eq!(c.position.default_x, parsed.position.default_x);
    }

    #[test]
    fn test_caesura_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Caesura::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // OtherArticulation Tests
    // ========================================================================

    #[test]
    fn test_other_articulation_round_trip() {
        let oa = OtherArticulation {
            value: "custom-articulation".to_string(),
            placement: Some(AboveBelow::Below),
            print_style: PrintStyle::default(),
        };
        let sexpr = oa.to_sexpr();
        let parsed = OtherArticulation::from_sexpr(&sexpr).unwrap();
        assert_eq!(oa.value, parsed.value);
        assert_eq!(oa.placement, parsed.placement);
    }

    #[test]
    fn test_other_articulation_with_position_color() {
        let oa = OtherArticulation {
            value: "test".to_string(),
            placement: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(5.0),
                    default_y: Some(10.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#ABCDEF".to_string()),
            },
        };
        let sexpr = oa.to_sexpr();
        let parsed = OtherArticulation::from_sexpr(&sexpr).unwrap();
        assert_eq!(oa.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_other_articulation_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(OtherArticulation::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // ArticulationElement Tests - All Variants
    // ========================================================================

    #[test]
    fn test_articulation_element_accent() {
        let ae = ArticulationElement::Accent(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Accent(_));
    }

    #[test]
    fn test_articulation_element_strong_accent() {
        let ae = ArticulationElement::StrongAccent(StrongAccent::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::StrongAccent(_));
    }

    #[test]
    fn test_articulation_element_staccato() {
        let ae = ArticulationElement::Staccato(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Staccato(_));
    }

    #[test]
    fn test_articulation_element_tenuto() {
        let ae = ArticulationElement::Tenuto(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Tenuto(_));
    }

    #[test]
    fn test_articulation_element_detached_legato() {
        let ae = ArticulationElement::DetachedLegato(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::DetachedLegato(_));
    }

    #[test]
    fn test_articulation_element_staccatissimo() {
        let ae = ArticulationElement::Staccatissimo(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Staccatissimo(_));
    }

    #[test]
    fn test_articulation_element_spiccato() {
        let ae = ArticulationElement::Spiccato(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Spiccato(_));
    }

    #[test]
    fn test_articulation_element_scoop() {
        let ae = ArticulationElement::Scoop(EmptyLine::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Scoop(_));
    }

    #[test]
    fn test_articulation_element_plop() {
        let ae = ArticulationElement::Plop(EmptyLine::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Plop(_));
    }

    #[test]
    fn test_articulation_element_doit() {
        let ae = ArticulationElement::Doit(EmptyLine::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Doit(_));
    }

    #[test]
    fn test_articulation_element_falloff() {
        let ae = ArticulationElement::Falloff(EmptyLine::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Falloff(_));
    }

    #[test]
    fn test_articulation_element_breath_mark() {
        let ae = ArticulationElement::BreathMark(BreathMark {
            value: BreathMarkValue::Comma,
            placement: None,
            position: Position::default(),
        });
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::BreathMark(_));
    }

    #[test]
    fn test_articulation_element_caesura() {
        let ae = ArticulationElement::Caesura(Caesura {
            value: CaesuraValue::Normal,
            placement: None,
            position: Position::default(),
        });
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Caesura(_));
    }

    #[test]
    fn test_articulation_element_stress() {
        let ae = ArticulationElement::Stress(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Stress(_));
    }

    #[test]
    fn test_articulation_element_unstress() {
        let ae = ArticulationElement::Unstress(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::Unstress(_));
    }

    #[test]
    fn test_articulation_element_soft_accent() {
        let ae = ArticulationElement::SoftAccent(EmptyPlacement::default());
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::SoftAccent(_));
    }

    #[test]
    fn test_articulation_element_other() {
        let ae = ArticulationElement::OtherArticulation(OtherArticulation {
            value: "custom".to_string(),
            placement: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = ae.to_sexpr();
        let parsed = ArticulationElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, ArticulationElement::OtherArticulation(_));
    }

    #[test]
    fn test_articulation_element_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(ArticulationElement::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_articulation_element_error_unknown_variant() {
        let sexpr = Sexpr::list(vec![Sexpr::symbol("unknown-articulation")]);
        assert!(ArticulationElement::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Articulations Container Tests
    // ========================================================================

    #[test]
    fn test_articulations_empty() {
        let art = Articulations { content: vec![] };
        let sexpr = art.to_sexpr();
        let parsed = Articulations::from_sexpr(&sexpr).unwrap();
        assert!(parsed.content.is_empty());
    }

    #[test]
    fn test_articulations_multiple() {
        let art = Articulations {
            content: vec![
                ArticulationElement::Accent(EmptyPlacement::default()),
                ArticulationElement::Staccato(EmptyPlacement::default()),
                ArticulationElement::Tenuto(EmptyPlacement::default()),
            ],
        };
        let sexpr = art.to_sexpr();
        let parsed = Articulations::from_sexpr(&sexpr).unwrap();
        assert_eq!(3, parsed.content.len());
    }

    #[test]
    fn test_articulations_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Articulations::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // EmptyTrillSound Tests
    // ========================================================================

    #[test]
    fn test_empty_trill_sound_minimal() {
        let ets = EmptyTrillSound::default();
        let sexpr = ets.to_sexpr();
        let parsed = EmptyTrillSound::from_sexpr(&sexpr).unwrap();
        assert_eq!(ets, parsed);
    }

    #[test]
    fn test_empty_trill_sound_full() {
        let ets = EmptyTrillSound {
            placement: Some(AboveBelow::Above),
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: None,
                relative_y: None,
            },
            start_note: Some(StartNote::Upper),
            trill_step: Some(TrillStep::Whole),
            two_note_turn: Some(TwoNoteTurn::Half),
            accelerate: Some(YesNo::Yes),
            beats: Some(4.0),
            second_beat: Some(25.0),
            last_beat: Some(75.0),
        };
        let sexpr = ets.to_sexpr();
        let parsed = EmptyTrillSound::from_sexpr(&sexpr).unwrap();
        assert_eq!(ets.start_note, parsed.start_note);
        assert_eq!(ets.trill_step, parsed.trill_step);
        assert_eq!(ets.beats, parsed.beats);
    }

    #[test]
    fn test_empty_trill_sound_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(EmptyTrillSound::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Turn Tests
    // ========================================================================

    #[test]
    fn test_turn_minimal() {
        let turn = Turn::default();
        let sexpr = turn.to_sexpr();
        let parsed = Turn::from_sexpr(&sexpr).unwrap();
        assert_eq!(turn, parsed);
    }

    #[test]
    fn test_turn_full() {
        let turn = Turn {
            slash: Some(YesNo::Yes),
            placement: Some(AboveBelow::Below),
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: Some(1.0),
                relative_y: Some(2.0),
            },
            start_note: Some(StartNote::Main),
            trill_step: Some(TrillStep::Half),
            two_note_turn: Some(TwoNoteTurn::Whole),
            accelerate: Some(YesNo::No),
            beats: Some(2.0),
            second_beat: Some(33.0),
            last_beat: Some(66.0),
        };
        let sexpr = turn.to_sexpr();
        let parsed = Turn::from_sexpr(&sexpr).unwrap();
        assert_eq!(turn.slash, parsed.slash);
        assert_eq!(turn.start_note, parsed.start_note);
    }

    #[test]
    fn test_turn_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Turn::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Mordent Tests
    // ========================================================================

    #[test]
    fn test_mordent_minimal() {
        let mordent = Mordent::default();
        let sexpr = mordent.to_sexpr();
        let parsed = Mordent::from_sexpr(&sexpr).unwrap();
        assert_eq!(mordent, parsed);
    }

    #[test]
    fn test_mordent_full() {
        let mordent = Mordent {
            long: Some(YesNo::Yes),
            approach: Some(AboveBelow::Above),
            departure: Some(AboveBelow::Below),
            placement: Some(AboveBelow::Above),
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: Some(3.0),
                relative_y: Some(4.0),
            },
            start_note: Some(StartNote::Below),
            trill_step: Some(TrillStep::Unison),
            two_note_turn: Some(TwoNoteTurn::None),
            accelerate: Some(YesNo::Yes),
            beats: Some(3.0),
            second_beat: Some(50.0),
            last_beat: Some(50.0),
        };
        let sexpr = mordent.to_sexpr();
        let parsed = Mordent::from_sexpr(&sexpr).unwrap();
        assert_eq!(mordent.long, parsed.long);
        assert_eq!(mordent.approach, parsed.approach);
        assert_eq!(mordent.departure, parsed.departure);
    }

    #[test]
    fn test_mordent_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Mordent::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Tremolo Tests (comprehensive)
    // ========================================================================

    #[test]
    fn test_tremolo_minimal() {
        let trem = Tremolo {
            value: 1,
            r#type: None,
            placement: None,
            position: Position::default(),
        };
        let sexpr = trem.to_sexpr();
        let parsed = Tremolo::from_sexpr(&sexpr).unwrap();
        assert_eq!(trem.value, parsed.value);
    }

    #[test]
    fn test_tremolo_with_position() {
        let trem = Tremolo {
            value: 2,
            r#type: Some(TremoloType::Start),
            placement: Some(AboveBelow::Below),
            position: Position {
                default_x: Some(5.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(10.0),
            },
        };
        let sexpr = trem.to_sexpr();
        let parsed = Tremolo::from_sexpr(&sexpr).unwrap();
        assert_eq!(trem.position.default_x, parsed.position.default_x);
    }

    #[test]
    fn test_tremolo_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Tremolo::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // OtherOrnament Tests
    // ========================================================================

    #[test]
    fn test_other_ornament_round_trip() {
        let oo = OtherOrnament {
            value: "custom-ornament".to_string(),
            placement: Some(AboveBelow::Above),
            print_style: PrintStyle::default(),
        };
        let sexpr = oo.to_sexpr();
        let parsed = OtherOrnament::from_sexpr(&sexpr).unwrap();
        assert_eq!(oo.value, parsed.value);
        assert_eq!(oo.placement, parsed.placement);
    }

    #[test]
    fn test_other_ornament_with_position_color() {
        let oo = OtherOrnament {
            value: "test".to_string(),
            placement: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(1.0),
                    default_y: Some(2.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#FEDCBA".to_string()),
            },
        };
        let sexpr = oo.to_sexpr();
        let parsed = OtherOrnament::from_sexpr(&sexpr).unwrap();
        assert_eq!(oo.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_other_ornament_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(OtherOrnament::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // OrnamentElement Tests - All Variants
    // ========================================================================

    #[test]
    fn test_ornament_element_trill_mark() {
        let oe = OrnamentElement::TrillMark(EmptyTrillSound::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::TrillMark(_));
    }

    #[test]
    fn test_ornament_element_turn() {
        let oe = OrnamentElement::Turn(Turn::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::Turn(_));
    }

    #[test]
    fn test_ornament_element_delayed_turn() {
        let oe = OrnamentElement::DelayedTurn(Turn::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::DelayedTurn(_));
    }

    #[test]
    fn test_ornament_element_inverted_turn() {
        let oe = OrnamentElement::InvertedTurn(Turn::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::InvertedTurn(_));
    }

    #[test]
    fn test_ornament_element_delayed_inverted_turn() {
        let oe = OrnamentElement::DelayedInvertedTurn(Turn::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::DelayedInvertedTurn(_));
    }

    #[test]
    fn test_ornament_element_vertical_turn() {
        let oe = OrnamentElement::VerticalTurn(EmptyTrillSound::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::VerticalTurn(_));
    }

    #[test]
    fn test_ornament_element_inverted_vertical_turn() {
        let oe = OrnamentElement::InvertedVerticalTurn(EmptyTrillSound::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::InvertedVerticalTurn(_));
    }

    #[test]
    fn test_ornament_element_shake() {
        let oe = OrnamentElement::Shake(EmptyTrillSound::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::Shake(_));
    }

    #[test]
    fn test_ornament_element_wavy_line() {
        use crate::ir::common::WavyLine;
        let oe = OrnamentElement::WavyLine(WavyLine {
            r#type: StartStopContinue::Start,
            number: Some(1),
            position: Position::default(),
        });
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::WavyLine(_));
    }

    #[test]
    fn test_ornament_element_mordent() {
        let oe = OrnamentElement::Mordent(Mordent::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::Mordent(_));
    }

    #[test]
    fn test_ornament_element_inverted_mordent() {
        let oe = OrnamentElement::InvertedMordent(Mordent::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::InvertedMordent(_));
    }

    #[test]
    fn test_ornament_element_schleifer() {
        let oe = OrnamentElement::Schleifer(EmptyPlacement::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::Schleifer(_));
    }

    #[test]
    fn test_ornament_element_tremolo() {
        let oe = OrnamentElement::Tremolo(Tremolo {
            value: 3,
            r#type: Some(TremoloType::Single),
            placement: None,
            position: Position::default(),
        });
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::Tremolo(_));
    }

    #[test]
    fn test_ornament_element_haydn() {
        let oe = OrnamentElement::Haydn(EmptyTrillSound::default());
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::Haydn(_));
    }

    #[test]
    fn test_ornament_element_other() {
        let oe = OrnamentElement::OtherOrnament(OtherOrnament {
            value: "custom".to_string(),
            placement: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = oe.to_sexpr();
        let parsed = OrnamentElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, OrnamentElement::OtherOrnament(_));
    }

    #[test]
    fn test_ornament_element_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(OrnamentElement::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_ornament_element_error_unknown_variant() {
        let sexpr = Sexpr::list(vec![Sexpr::symbol("unknown-ornament")]);
        assert!(OrnamentElement::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // OrnamentWithAccidentals Tests
    // ========================================================================

    #[test]
    fn test_ornament_with_accidentals_no_marks() {
        let owa = OrnamentWithAccidentals {
            ornament: OrnamentElement::TrillMark(EmptyTrillSound::default()),
            accidental_marks: vec![],
        };
        let sexpr = owa.to_sexpr();
        let parsed = OrnamentWithAccidentals::from_sexpr(&sexpr).unwrap();
        assert!(parsed.accidental_marks.is_empty());
    }

    #[test]
    fn test_ornament_with_accidentals_with_marks() {
        let owa = OrnamentWithAccidentals {
            ornament: OrnamentElement::Turn(Turn::default()),
            accidental_marks: vec![
                AccidentalMark {
                    value: AccidentalValue::Sharp,
                    placement: Some(AboveBelow::Above),
                    print_style: PrintStyle::default(),
                },
                AccidentalMark {
                    value: AccidentalValue::Flat,
                    placement: Some(AboveBelow::Below),
                    print_style: PrintStyle::default(),
                },
            ],
        };
        let sexpr = owa.to_sexpr();
        let parsed = OrnamentWithAccidentals::from_sexpr(&sexpr).unwrap();
        assert_eq!(2, parsed.accidental_marks.len());
    }

    #[test]
    fn test_ornament_with_accidentals_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(OrnamentWithAccidentals::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Ornaments Container Tests
    // ========================================================================

    #[test]
    fn test_ornaments_empty() {
        let orn = Ornaments { content: vec![] };
        let sexpr = orn.to_sexpr();
        let parsed = Ornaments::from_sexpr(&sexpr).unwrap();
        assert!(parsed.content.is_empty());
    }

    #[test]
    fn test_ornaments_multiple() {
        let orn = Ornaments {
            content: vec![
                OrnamentWithAccidentals {
                    ornament: OrnamentElement::TrillMark(EmptyTrillSound::default()),
                    accidental_marks: vec![],
                },
                OrnamentWithAccidentals {
                    ornament: OrnamentElement::Mordent(Mordent::default()),
                    accidental_marks: vec![],
                },
            ],
        };
        let sexpr = orn.to_sexpr();
        let parsed = Ornaments::from_sexpr(&sexpr).unwrap();
        assert_eq!(2, parsed.content.len());
    }

    #[test]
    fn test_ornaments_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Ornaments::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Technical Struct Tests
    // ========================================================================

    #[test]
    fn test_harmonic_minimal() {
        let h = Harmonic::default();
        let sexpr = h.to_sexpr();
        let parsed = Harmonic::from_sexpr(&sexpr).unwrap();
        assert_eq!(h, parsed);
    }

    #[test]
    fn test_harmonic_full() {
        let h = Harmonic {
            natural: true,
            artificial: false,
            base_pitch: true,
            touching_pitch: false,
            sounding_pitch: true,
            placement: Some(AboveBelow::Above),
            print_object: Some(YesNo::Yes),
        };
        let sexpr = h.to_sexpr();
        let parsed = Harmonic::from_sexpr(&sexpr).unwrap();
        assert!(parsed.natural);
        assert!(!parsed.artificial);
        assert!(parsed.base_pitch);
        assert!(parsed.sounding_pitch);
    }

    #[test]
    fn test_harmonic_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Harmonic::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_fingering_with_all_fields() {
        let f = Fingering {
            value: "p".to_string(),
            substitution: Some(YesNo::Yes),
            alternate: Some(YesNo::No),
            placement: Some(AboveBelow::Below),
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(1.0),
                    default_y: Some(2.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#123456".to_string()),
            },
        };
        let sexpr = f.to_sexpr();
        let parsed = Fingering::from_sexpr(&sexpr).unwrap();
        assert_eq!(f.value, parsed.value);
        assert_eq!(f.substitution, parsed.substitution);
        assert_eq!(f.alternate, parsed.alternate);
        assert_eq!(f.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_fingering_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Fingering::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_pluck_round_trip() {
        let p = Pluck {
            value: "i".to_string(),
            placement: Some(AboveBelow::Above),
        };
        let sexpr = p.to_sexpr();
        let parsed = Pluck::from_sexpr(&sexpr).unwrap();
        assert_eq!(p.value, parsed.value);
        assert_eq!(p.placement, parsed.placement);
    }

    #[test]
    fn test_pluck_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Pluck::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_fret_minimal() {
        use crate::ir::common::Font;
        let f = Fret {
            value: 5,
            font: Font::default(),
            color: None,
        };
        let sexpr = f.to_sexpr();
        let parsed = Fret::from_sexpr(&sexpr).unwrap();
        assert_eq!(f.value, parsed.value);
    }

    #[test]
    fn test_fret_with_font_color() {
        use crate::ir::common::{Font, FontWeight};
        let f = Fret {
            value: 7,
            font: Font {
                font_family: Some("Arial".to_string()),
                font_style: None,
                font_size: None,
                font_weight: Some(FontWeight::Bold),
            },
            color: Some("#AABBCC".to_string()),
        };
        let sexpr = f.to_sexpr();
        let parsed = Fret::from_sexpr(&sexpr).unwrap();
        assert_eq!(f.value, parsed.value);
        assert_eq!(f.color, parsed.color);
    }

    #[test]
    fn test_fret_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Fret::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_string_number_round_trip() {
        let sn = StringNumber {
            value: 3,
            placement: Some(AboveBelow::Below),
            print_style: PrintStyle::default(),
        };
        let sexpr = sn.to_sexpr();
        let parsed = StringNumber::from_sexpr(&sexpr).unwrap();
        assert_eq!(sn.value, parsed.value);
        assert_eq!(sn.placement, parsed.placement);
    }

    #[test]
    fn test_string_number_with_position_color() {
        let sn = StringNumber {
            value: 1,
            placement: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(5.0),
                    default_y: Some(10.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: Some("#FEDCBA".to_string()),
            },
        };
        let sexpr = sn.to_sexpr();
        let parsed = StringNumber::from_sexpr(&sexpr).unwrap();
        assert_eq!(sn.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_string_number_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(StringNumber::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_hammer_pull_round_trip() {
        let hp = HammerPull {
            value: "H".to_string(),
            r#type: StartStop::Start,
            number: Some(1),
            placement: Some(AboveBelow::Above),
        };
        let sexpr = hp.to_sexpr();
        let parsed = HammerPull::from_sexpr(&sexpr).unwrap();
        assert_eq!(hp.value, parsed.value);
        assert_eq!(hp.r#type, parsed.r#type);
    }

    #[test]
    fn test_hammer_pull_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(HammerPull::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_bend_minimal() {
        let b = Bend {
            bend_alter: 1.0,
            pre_bend: false,
            release: None,
            with_bar: None,
        };
        let sexpr = b.to_sexpr();
        let parsed = Bend::from_sexpr(&sexpr).unwrap();
        assert!((b.bend_alter - parsed.bend_alter).abs() < 0.001);
        assert!(!parsed.pre_bend);
    }

    #[test]
    fn test_bend_full() {
        let b = Bend {
            bend_alter: 2.0,
            pre_bend: true,
            release: Some(BendRelease::Late),
            with_bar: Some("w/bar".to_string()),
        };
        let sexpr = b.to_sexpr();
        let parsed = Bend::from_sexpr(&sexpr).unwrap();
        assert!(parsed.pre_bend);
        assert_eq!(b.release, parsed.release);
        assert_eq!(b.with_bar, parsed.with_bar);
    }

    #[test]
    fn test_bend_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Bend::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_tap_round_trip() {
        let t = Tap {
            value: "T".to_string(),
            hand: Some(TapHand::Right),
        };
        let sexpr = t.to_sexpr();
        let parsed = Tap::from_sexpr(&sexpr).unwrap();
        assert_eq!(t.value, parsed.value);
        assert_eq!(t.hand, parsed.hand);
    }

    #[test]
    fn test_tap_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Tap::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_heel_toe_round_trip() {
        let ht = HeelToe {
            substitution: Some(YesNo::Yes),
            placement: Some(AboveBelow::Below),
        };
        let sexpr = ht.to_sexpr();
        let parsed = HeelToe::from_sexpr(&sexpr).unwrap();
        assert_eq!(ht.substitution, parsed.substitution);
        assert_eq!(ht.placement, parsed.placement);
    }

    #[test]
    fn test_heel_toe_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(HeelToe::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_hole_closed_round_trip() {
        let hc = HoleClosed {
            value: HoleClosedValue::Half,
            location: Some(HoleClosedLocation::Top),
        };
        let sexpr = hc.to_sexpr();
        let parsed = HoleClosed::from_sexpr(&sexpr).unwrap();
        assert_eq!(hc.value, parsed.value);
        assert_eq!(hc.location, parsed.location);
    }

    #[test]
    fn test_hole_closed_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(HoleClosed::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_hole_round_trip() {
        let h = Hole {
            hole_type: Some("thumb".to_string()),
            hole_closed: HoleClosed {
                value: HoleClosedValue::Yes,
                location: None,
            },
            hole_shape: Some("circle".to_string()),
        };
        let sexpr = h.to_sexpr();
        let parsed = Hole::from_sexpr(&sexpr).unwrap();
        assert_eq!(h.hole_type, parsed.hole_type);
        assert_eq!(h.hole_shape, parsed.hole_shape);
    }

    #[test]
    fn test_hole_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Hole::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_arrow_round_trip() {
        let a = Arrow {
            direction: Some(ArrowDirection::Up),
            style: Some(ArrowStyle::Filled),
            smufl: None,
        };
        let sexpr = a.to_sexpr();
        let parsed = Arrow::from_sexpr(&sexpr).unwrap();
        assert_eq!(a.direction, parsed.direction);
        assert_eq!(a.style, parsed.style);
    }

    #[test]
    fn test_arrow_with_smufl() {
        let a = Arrow {
            direction: None,
            style: None,
            smufl: Some("arrowhead".to_string()),
        };
        let sexpr = a.to_sexpr();
        let parsed = Arrow::from_sexpr(&sexpr).unwrap();
        assert_eq!(a.smufl, parsed.smufl);
    }

    #[test]
    fn test_arrow_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Arrow::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_handbell_round_trip() {
        let h = Handbell {
            value: HandbellValue::Martellato,
        };
        let sexpr = h.to_sexpr();
        let parsed = Handbell::from_sexpr(&sexpr).unwrap();
        assert_eq!(h.value, parsed.value);
    }

    #[test]
    fn test_handbell_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Handbell::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_harmon_mute_default() {
        let hm = HarmonMute::default();
        let sexpr = hm.to_sexpr();
        let parsed = HarmonMute::from_sexpr(&sexpr).unwrap();
        assert!(!parsed.open);
        assert!(!parsed.half);
    }

    #[test]
    fn test_harmon_mute_flags() {
        let hm = HarmonMute {
            open: true,
            half: true,
        };
        let sexpr = hm.to_sexpr();
        let parsed = HarmonMute::from_sexpr(&sexpr).unwrap();
        assert!(parsed.open);
        assert!(parsed.half);
    }

    #[test]
    fn test_harmon_mute_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(HarmonMute::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_other_technical_round_trip() {
        let ot = OtherTechnical {
            value: "custom-technical".to_string(),
            placement: Some(AboveBelow::Above),
            print_style: PrintStyle::default(),
        };
        let sexpr = ot.to_sexpr();
        let parsed = OtherTechnical::from_sexpr(&sexpr).unwrap();
        assert_eq!(ot.value, parsed.value);
        assert_eq!(ot.placement, parsed.placement);
    }

    #[test]
    fn test_other_technical_with_position_color() {
        let ot = OtherTechnical {
            value: "test".to_string(),
            placement: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(1.0),
                    default_y: Some(2.0),
                    relative_x: Some(3.0),
                    relative_y: Some(4.0),
                },
                font: Default::default(),
                color: Some("#999999".to_string()),
            },
        };
        let sexpr = ot.to_sexpr();
        let parsed = OtherTechnical::from_sexpr(&sexpr).unwrap();
        assert_eq!(ot.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_other_technical_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(OtherTechnical::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // TechnicalElement Tests - All Variants
    // ========================================================================

    #[test]
    fn test_technical_element_up_bow() {
        let te = TechnicalElement::UpBow(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::UpBow(_));
    }

    #[test]
    fn test_technical_element_down_bow() {
        let te = TechnicalElement::DownBow(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::DownBow(_));
    }

    #[test]
    fn test_technical_element_harmonic() {
        let te = TechnicalElement::Harmonic(Harmonic::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Harmonic(_));
    }

    #[test]
    fn test_technical_element_open_string() {
        let te = TechnicalElement::OpenString(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::OpenString(_));
    }

    #[test]
    fn test_technical_element_thumb_position() {
        let te = TechnicalElement::ThumbPosition(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::ThumbPosition(_));
    }

    #[test]
    fn test_technical_element_fingering() {
        let te = TechnicalElement::Fingering(Fingering {
            value: "1".to_string(),
            substitution: None,
            alternate: None,
            placement: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Fingering(_));
    }

    #[test]
    fn test_technical_element_pluck() {
        let te = TechnicalElement::Pluck(Pluck {
            value: "m".to_string(),
            placement: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Pluck(_));
    }

    #[test]
    fn test_technical_element_double_tongue() {
        let te = TechnicalElement::DoubleTongue(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::DoubleTongue(_));
    }

    #[test]
    fn test_technical_element_triple_tongue() {
        let te = TechnicalElement::TripleTongue(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::TripleTongue(_));
    }

    #[test]
    fn test_technical_element_stopped() {
        let te = TechnicalElement::Stopped(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Stopped(_));
    }

    #[test]
    fn test_technical_element_snap_pizzicato() {
        let te = TechnicalElement::SnapPizzicato(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::SnapPizzicato(_));
    }

    #[test]
    fn test_technical_element_fret() {
        use crate::ir::common::Font;
        let te = TechnicalElement::Fret(Fret {
            value: 5,
            font: Font::default(),
            color: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Fret(_));
    }

    #[test]
    fn test_technical_element_string() {
        let te = TechnicalElement::String(StringNumber {
            value: 2,
            placement: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::String(_));
    }

    #[test]
    fn test_technical_element_hammer_on() {
        let te = TechnicalElement::HammerOn(HammerPull {
            value: "H".to_string(),
            r#type: StartStop::Start,
            number: None,
            placement: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::HammerOn(_));
    }

    #[test]
    fn test_technical_element_pull_off() {
        let te = TechnicalElement::PullOff(HammerPull {
            value: "P".to_string(),
            r#type: StartStop::Stop,
            number: None,
            placement: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::PullOff(_));
    }

    #[test]
    fn test_technical_element_bend() {
        let te = TechnicalElement::Bend(Bend {
            bend_alter: 1.0,
            pre_bend: false,
            release: None,
            with_bar: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Bend(_));
    }

    #[test]
    fn test_technical_element_tap() {
        let te = TechnicalElement::Tap(Tap {
            value: "T".to_string(),
            hand: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Tap(_));
    }

    #[test]
    fn test_technical_element_heel() {
        let te = TechnicalElement::Heel(HeelToe::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Heel(_));
    }

    #[test]
    fn test_technical_element_toe() {
        let te = TechnicalElement::Toe(HeelToe::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Toe(_));
    }

    #[test]
    fn test_technical_element_fingernails() {
        let te = TechnicalElement::Fingernails(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Fingernails(_));
    }

    #[test]
    fn test_technical_element_hole() {
        let te = TechnicalElement::Hole(Hole {
            hole_type: None,
            hole_closed: HoleClosed {
                value: HoleClosedValue::Yes,
                location: None,
            },
            hole_shape: None,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Hole(_));
    }

    #[test]
    fn test_technical_element_arrow() {
        let te = TechnicalElement::Arrow(Arrow::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Arrow(_));
    }

    #[test]
    fn test_technical_element_handbell() {
        let te = TechnicalElement::Handbell(Handbell {
            value: HandbellValue::Damp,
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Handbell(_));
    }

    #[test]
    fn test_technical_element_brass_bend() {
        let te = TechnicalElement::BrassBend(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::BrassBend(_));
    }

    #[test]
    fn test_technical_element_flip() {
        let te = TechnicalElement::Flip(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Flip(_));
    }

    #[test]
    fn test_technical_element_smear() {
        let te = TechnicalElement::Smear(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Smear(_));
    }

    #[test]
    fn test_technical_element_open() {
        let te = TechnicalElement::Open(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Open(_));
    }

    #[test]
    fn test_technical_element_half_muted() {
        let te = TechnicalElement::HalfMuted(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::HalfMuted(_));
    }

    #[test]
    fn test_technical_element_harmon_mute() {
        let te = TechnicalElement::HarmonMute(HarmonMute::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::HarmonMute(_));
    }

    #[test]
    fn test_technical_element_golpe() {
        let te = TechnicalElement::Golpe(EmptyPlacement::default());
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::Golpe(_));
    }

    #[test]
    fn test_technical_element_other() {
        let te = TechnicalElement::OtherTechnical(OtherTechnical {
            value: "custom".to_string(),
            placement: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = te.to_sexpr();
        let parsed = TechnicalElement::from_sexpr(&sexpr).unwrap();
        matches!(parsed, TechnicalElement::OtherTechnical(_));
    }

    #[test]
    fn test_technical_element_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(TechnicalElement::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_technical_element_error_unknown_variant() {
        let sexpr = Sexpr::list(vec![Sexpr::symbol("unknown-technical")]);
        assert!(TechnicalElement::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Technical Container Tests
    // ========================================================================

    #[test]
    fn test_technical_empty() {
        let tech = Technical { content: vec![] };
        let sexpr = tech.to_sexpr();
        let parsed = Technical::from_sexpr(&sexpr).unwrap();
        assert!(parsed.content.is_empty());
    }

    #[test]
    fn test_technical_multiple() {
        let tech = Technical {
            content: vec![
                TechnicalElement::UpBow(EmptyPlacement::default()),
                TechnicalElement::DownBow(EmptyPlacement::default()),
                TechnicalElement::OpenString(EmptyPlacement::default()),
            ],
        };
        let sexpr = tech.to_sexpr();
        let parsed = Technical::from_sexpr(&sexpr).unwrap();
        assert_eq!(3, parsed.content.len());
    }

    #[test]
    fn test_technical_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Technical::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // NotationContent Tests - All Variants
    // ========================================================================

    #[test]
    fn test_notation_content_tied() {
        let nc = NotationContent::Tied(Tied {
            r#type: StartStopContinue::Start,
            number: None,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        });
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Tied(_));
    }

    #[test]
    fn test_notation_content_slur() {
        let nc = NotationContent::Slur(Slur {
            r#type: StartStopContinue::Start,
            number: 1,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        });
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Slur(_));
    }

    #[test]
    fn test_notation_content_tuplet() {
        let nc = NotationContent::Tuplet(Box::new(Tuplet {
            r#type: StartStop::Start,
            number: None,
            bracket: None,
            show_number: None,
            show_type: None,
            line_shape: None,
            position: Position::default(),
            placement: None,
            tuplet_actual: None,
            tuplet_normal: None,
        }));
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Tuplet(_));
    }

    #[test]
    fn test_notation_content_glissando() {
        let nc = NotationContent::Glissando(Glissando {
            r#type: StartStop::Start,
            number: None,
            text: None,
            line_type: None,
            position: Position::default(),
        });
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Glissando(_));
    }

    #[test]
    fn test_notation_content_slide() {
        let nc = NotationContent::Slide(Slide {
            r#type: StartStop::Start,
            number: None,
            text: None,
            line_type: None,
            position: Position::default(),
        });
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Slide(_));
    }

    #[test]
    fn test_notation_content_ornaments() {
        let nc = NotationContent::Ornaments(Box::new(Ornaments { content: vec![] }));
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Ornaments(_));
    }

    #[test]
    fn test_notation_content_technical() {
        let nc = NotationContent::Technical(Box::new(Technical { content: vec![] }));
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Technical(_));
    }

    #[test]
    fn test_notation_content_articulations() {
        let nc = NotationContent::Articulations(Box::new(Articulations { content: vec![] }));
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Articulations(_));
    }

    #[test]
    fn test_notation_content_dynamics() {
        use crate::ir::direction::Dynamics;
        let nc = NotationContent::Dynamics(Box::new(Dynamics::default()));
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Dynamics(_));
    }

    #[test]
    fn test_notation_content_fermata() {
        use crate::ir::notation::Fermata;
        let nc = NotationContent::Fermata(Fermata::default());
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Fermata(_));
    }

    #[test]
    fn test_notation_content_arpeggiate() {
        let nc = NotationContent::Arpeggiate(Arpeggiate::default());
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::Arpeggiate(_));
    }

    #[test]
    fn test_notation_content_non_arpeggiate() {
        let nc = NotationContent::NonArpeggiate(NonArpeggiate::default());
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::NonArpeggiate(_));
    }

    #[test]
    fn test_notation_content_accidental_mark() {
        let nc = NotationContent::AccidentalMark(AccidentalMark {
            value: AccidentalValue::Natural,
            placement: None,
            print_style: PrintStyle::default(),
        });
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::AccidentalMark(_));
    }

    #[test]
    fn test_notation_content_other_notation() {
        use crate::ir::common::StartStopSingle;
        let nc = NotationContent::OtherNotation(OtherNotation {
            value: "custom".to_string(),
            r#type: StartStopSingle::Single,
            number: None,
            print_object: None,
            print_style: PrintStyle::default(),
            placement: None,
        });
        let sexpr = nc.to_sexpr();
        let parsed = NotationContent::from_sexpr(&sexpr).unwrap();
        matches!(parsed, NotationContent::OtherNotation(_));
    }

    #[test]
    fn test_notation_content_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(NotationContent::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_notation_content_error_unknown_variant() {
        let sexpr = Sexpr::list(vec![Sexpr::symbol("unknown-notation")]);
        assert!(NotationContent::from_sexpr(&sexpr).is_err());
    }

    // ========================================================================
    // Notations Container Tests
    // ========================================================================

    #[test]
    fn test_notations_empty() {
        use crate::ir::common::Editorial;
        let notations = Notations {
            print_object: None,
            content: vec![],
            editorial: Editorial::default(),
        };
        let sexpr = notations.to_sexpr();
        let parsed = Notations::from_sexpr(&sexpr).unwrap();
        assert!(parsed.content.is_empty());
    }

    #[test]
    fn test_notations_multiple_content() {
        use crate::ir::common::Editorial;
        use crate::ir::notation::Fermata;
        let notations = Notations {
            print_object: Some(YesNo::No),
            content: vec![
                NotationContent::Fermata(Fermata::default()),
                NotationContent::Arpeggiate(Arpeggiate::default()),
                NotationContent::Articulations(Box::new(Articulations { content: vec![] })),
            ],
            editorial: Editorial::default(),
        };
        let sexpr = notations.to_sexpr();
        let parsed = Notations::from_sexpr(&sexpr).unwrap();
        assert_eq!(3, parsed.content.len());
        assert_eq!(Some(YesNo::No), parsed.print_object);
    }

    #[test]
    fn test_notations_error_not_list() {
        let sexpr = Sexpr::symbol("not-a-list");
        assert!(Notations::from_sexpr(&sexpr).is_err());
    }
}
