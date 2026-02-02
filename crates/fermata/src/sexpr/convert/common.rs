//! S-expression conversions for `ir::common` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for all types
//! defined in `crate::ir::common`.

use super::optional_kwarg;
use crate::ir::common::{
    AboveBelow, AccidentalValue, BackwardForward, CssFontSize, Font, FontSize, FontStyle,
    FontWeight, FormattedText, LeftCenterRight, LineType, OverUnder, Position, PrintStyle,
    RightLeftMiddle, StartStop, StartStopContinue, StartStopDiscontinue, StartStopSingle,
    SymbolSize, TopMiddleBottom, UpDown, UprightInverted, WavyLine, YesNo,
};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

// ============================================================================
// YesNo
// ============================================================================

impl ToSexpr for YesNo {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            YesNo::Yes => "yes",
            YesNo::No => "no",
        })
    }
}

impl FromSexpr for YesNo {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("yes") => Ok(YesNo::Yes),
            Some("no") => Ok(YesNo::No),
            _ => Err(ConvertError::type_mismatch("yes/no", sexpr)),
        }
    }
}

// ============================================================================
// StartStop
// ============================================================================

impl ToSexpr for StartStop {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartStop::Start => "start",
            StartStop::Stop => "stop",
        })
    }
}

impl FromSexpr for StartStop {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(StartStop::Start),
            Some("stop") => Ok(StartStop::Stop),
            _ => Err(ConvertError::type_mismatch("start/stop", sexpr)),
        }
    }
}

// ============================================================================
// StartStopContinue
// ============================================================================

impl ToSexpr for StartStopContinue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartStopContinue::Start => "start",
            StartStopContinue::Stop => "stop",
            StartStopContinue::Continue => "continue",
        })
    }
}

impl FromSexpr for StartStopContinue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(StartStopContinue::Start),
            Some("stop") => Ok(StartStopContinue::Stop),
            Some("continue") => Ok(StartStopContinue::Continue),
            _ => Err(ConvertError::type_mismatch("start/stop/continue", sexpr)),
        }
    }
}

// ============================================================================
// StartStopSingle
// ============================================================================

impl ToSexpr for StartStopSingle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartStopSingle::Start => "start",
            StartStopSingle::Stop => "stop",
            StartStopSingle::Single => "single",
        })
    }
}

impl FromSexpr for StartStopSingle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(StartStopSingle::Start),
            Some("stop") => Ok(StartStopSingle::Stop),
            Some("single") => Ok(StartStopSingle::Single),
            _ => Err(ConvertError::type_mismatch("start/stop/single", sexpr)),
        }
    }
}

// ============================================================================
// StartStopDiscontinue
// ============================================================================

impl ToSexpr for StartStopDiscontinue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartStopDiscontinue::Start => "start",
            StartStopDiscontinue::Stop => "stop",
            StartStopDiscontinue::Discontinue => "discontinue",
        })
    }
}

impl FromSexpr for StartStopDiscontinue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(StartStopDiscontinue::Start),
            Some("stop") => Ok(StartStopDiscontinue::Stop),
            Some("discontinue") => Ok(StartStopDiscontinue::Discontinue),
            _ => Err(ConvertError::type_mismatch("start/stop/discontinue", sexpr)),
        }
    }
}

// ============================================================================
// AboveBelow
// ============================================================================

impl ToSexpr for AboveBelow {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            AboveBelow::Above => "above",
            AboveBelow::Below => "below",
        })
    }
}

impl FromSexpr for AboveBelow {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("above") => Ok(AboveBelow::Above),
            Some("below") => Ok(AboveBelow::Below),
            _ => Err(ConvertError::type_mismatch("above/below", sexpr)),
        }
    }
}

// ============================================================================
// UpDown
// ============================================================================

impl ToSexpr for UpDown {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            UpDown::Up => "up",
            UpDown::Down => "down",
        })
    }
}

impl FromSexpr for UpDown {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("up") => Ok(UpDown::Up),
            Some("down") => Ok(UpDown::Down),
            _ => Err(ConvertError::type_mismatch("up/down", sexpr)),
        }
    }
}

// ============================================================================
// OverUnder
// ============================================================================

impl ToSexpr for OverUnder {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            OverUnder::Over => "over",
            OverUnder::Under => "under",
        })
    }
}

impl FromSexpr for OverUnder {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("over") => Ok(OverUnder::Over),
            Some("under") => Ok(OverUnder::Under),
            _ => Err(ConvertError::type_mismatch("over/under", sexpr)),
        }
    }
}

// ============================================================================
// LeftCenterRight
// ============================================================================

impl ToSexpr for LeftCenterRight {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            LeftCenterRight::Left => "left",
            LeftCenterRight::Center => "center",
            LeftCenterRight::Right => "right",
        })
    }
}

impl FromSexpr for LeftCenterRight {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("left") => Ok(LeftCenterRight::Left),
            Some("center") => Ok(LeftCenterRight::Center),
            Some("right") => Ok(LeftCenterRight::Right),
            _ => Err(ConvertError::type_mismatch("left/center/right", sexpr)),
        }
    }
}

// ============================================================================
// TopMiddleBottom
// ============================================================================

impl ToSexpr for TopMiddleBottom {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TopMiddleBottom::Top => "top",
            TopMiddleBottom::Middle => "middle",
            TopMiddleBottom::Bottom => "bottom",
        })
    }
}

impl FromSexpr for TopMiddleBottom {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("top") => Ok(TopMiddleBottom::Top),
            Some("middle") => Ok(TopMiddleBottom::Middle),
            Some("bottom") => Ok(TopMiddleBottom::Bottom),
            _ => Err(ConvertError::type_mismatch("top/middle/bottom", sexpr)),
        }
    }
}

// ============================================================================
// BackwardForward
// ============================================================================

impl ToSexpr for BackwardForward {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BackwardForward::Backward => "backward",
            BackwardForward::Forward => "forward",
        })
    }
}

impl FromSexpr for BackwardForward {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("backward") => Ok(BackwardForward::Backward),
            Some("forward") => Ok(BackwardForward::Forward),
            _ => Err(ConvertError::type_mismatch("backward/forward", sexpr)),
        }
    }
}

// ============================================================================
// RightLeftMiddle
// ============================================================================

impl ToSexpr for RightLeftMiddle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            RightLeftMiddle::Right => "right",
            RightLeftMiddle::Left => "left",
            RightLeftMiddle::Middle => "middle",
        })
    }
}

impl FromSexpr for RightLeftMiddle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("right") => Ok(RightLeftMiddle::Right),
            Some("left") => Ok(RightLeftMiddle::Left),
            Some("middle") => Ok(RightLeftMiddle::Middle),
            _ => Err(ConvertError::type_mismatch("right/left/middle", sexpr)),
        }
    }
}

// ============================================================================
// UprightInverted
// ============================================================================

impl ToSexpr for UprightInverted {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            UprightInverted::Upright => "upright",
            UprightInverted::Inverted => "inverted",
        })
    }
}

impl FromSexpr for UprightInverted {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("upright") => Ok(UprightInverted::Upright),
            Some("inverted") => Ok(UprightInverted::Inverted),
            _ => Err(ConvertError::type_mismatch("upright/inverted", sexpr)),
        }
    }
}

// ============================================================================
// SymbolSize
// ============================================================================

impl ToSexpr for SymbolSize {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            SymbolSize::Full => "full",
            SymbolSize::Cue => "cue",
            SymbolSize::GraceCue => "grace-cue",
            SymbolSize::Large => "large",
        })
    }
}

impl FromSexpr for SymbolSize {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("full") => Ok(SymbolSize::Full),
            Some("cue") => Ok(SymbolSize::Cue),
            Some("grace-cue") => Ok(SymbolSize::GraceCue),
            Some("large") => Ok(SymbolSize::Large),
            _ => Err(ConvertError::type_mismatch(
                "full/cue/grace-cue/large",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// LineType
// ============================================================================

impl ToSexpr for LineType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            LineType::Solid => "solid",
            LineType::Dashed => "dashed",
            LineType::Dotted => "dotted",
            LineType::Wavy => "wavy",
        })
    }
}

impl FromSexpr for LineType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("solid") => Ok(LineType::Solid),
            Some("dashed") => Ok(LineType::Dashed),
            Some("dotted") => Ok(LineType::Dotted),
            Some("wavy") => Ok(LineType::Wavy),
            _ => Err(ConvertError::type_mismatch(
                "solid/dashed/dotted/wavy",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// FontStyle
// ============================================================================

impl ToSexpr for FontStyle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
        })
    }
}

impl FromSexpr for FontStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("normal") => Ok(FontStyle::Normal),
            Some("italic") => Ok(FontStyle::Italic),
            _ => Err(ConvertError::type_mismatch("normal/italic", sexpr)),
        }
    }
}

// ============================================================================
// FontWeight
// ============================================================================

impl ToSexpr for FontWeight {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            FontWeight::Normal => "normal",
            FontWeight::Bold => "bold",
        })
    }
}

impl FromSexpr for FontWeight {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("normal") => Ok(FontWeight::Normal),
            Some("bold") => Ok(FontWeight::Bold),
            _ => Err(ConvertError::type_mismatch("normal/bold", sexpr)),
        }
    }
}

// ============================================================================
// CssFontSize
// ============================================================================

impl ToSexpr for CssFontSize {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            CssFontSize::XxSmall => "xx-small",
            CssFontSize::XSmall => "x-small",
            CssFontSize::Small => "small",
            CssFontSize::Medium => "medium",
            CssFontSize::Large => "large",
            CssFontSize::XLarge => "x-large",
            CssFontSize::XxLarge => "xx-large",
        })
    }
}

impl FromSexpr for CssFontSize {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("xx-small") => Ok(CssFontSize::XxSmall),
            Some("x-small") => Ok(CssFontSize::XSmall),
            Some("small") => Ok(CssFontSize::Small),
            Some("medium") => Ok(CssFontSize::Medium),
            Some("large") => Ok(CssFontSize::Large),
            Some("x-large") => Ok(CssFontSize::XLarge),
            Some("xx-large") => Ok(CssFontSize::XxLarge),
            _ => Err(ConvertError::type_mismatch("CSS font size", sexpr)),
        }
    }
}

// ============================================================================
// FontSize
// ============================================================================

impl ToSexpr for FontSize {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            FontSize::Css(css) => css.to_sexpr(),
            FontSize::Points(pts) => Sexpr::Float(*pts),
        }
    }
}

impl FromSexpr for FontSize {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        // Try as CSS size first
        if let Some(sym) = sexpr.as_symbol() {
            if matches!(
                sym,
                "xx-small" | "x-small" | "small" | "medium" | "large" | "x-large" | "xx-large"
            ) {
                return CssFontSize::from_sexpr(sexpr).map(FontSize::Css);
            }
        }
        // Otherwise try as points
        f64::from_sexpr(sexpr).map(FontSize::Points)
    }
}

// ============================================================================
// Position
// ============================================================================

impl ToSexpr for Position {
    fn to_sexpr(&self) -> Sexpr {
        // Only emit if any field is set
        let has_content = self.default_x.is_some()
            || self.default_y.is_some()
            || self.relative_x.is_some()
            || self.relative_y.is_some();

        if !has_content {
            return Sexpr::Nil;
        }

        ListBuilder::new("position")
            .kwarg_opt("default-x", &self.default_x)
            .kwarg_opt("default-y", &self.default_y)
            .kwarg_opt("relative-x", &self.relative_x)
            .kwarg_opt("relative-y", &self.relative_y)
            .build()
    }
}

impl FromSexpr for Position {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        // Handle nil as empty position
        if sexpr.is_nil() || sexpr.is_symbol("nil") {
            return Ok(Position::default());
        }

        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("list or nil", sexpr))?;

        // Empty list is valid
        if list.is_empty() {
            return Ok(Position::default());
        }

        Ok(Position {
            default_x: optional_kwarg(list, "default-x")?,
            default_y: optional_kwarg(list, "default-y")?,
            relative_x: optional_kwarg(list, "relative-x")?,
            relative_y: optional_kwarg(list, "relative-y")?,
        })
    }
}

// ============================================================================
// Font
// ============================================================================

impl ToSexpr for Font {
    fn to_sexpr(&self) -> Sexpr {
        // Only emit if any field is set
        let has_content = self.font_family.is_some()
            || self.font_style.is_some()
            || self.font_size.is_some()
            || self.font_weight.is_some();

        if !has_content {
            return Sexpr::Nil;
        }

        ListBuilder::new("font")
            .kwarg_opt("font-family", &self.font_family)
            .kwarg_opt("font-style", &self.font_style)
            .kwarg_opt("font-size", &self.font_size)
            .kwarg_opt("font-weight", &self.font_weight)
            .build()
    }
}

impl FromSexpr for Font {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        // Handle nil as empty font
        if sexpr.is_nil() || sexpr.is_symbol("nil") {
            return Ok(Font::default());
        }

        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("list or nil", sexpr))?;

        // Empty list is valid
        if list.is_empty() {
            return Ok(Font::default());
        }

        Ok(Font {
            font_family: optional_kwarg(list, "font-family")?,
            font_style: optional_kwarg(list, "font-style")?,
            font_size: optional_kwarg(list, "font-size")?,
            font_weight: optional_kwarg(list, "font-weight")?,
        })
    }
}

// ============================================================================
// PrintStyle
// ============================================================================

impl ToSexpr for PrintStyle {
    fn to_sexpr(&self) -> Sexpr {
        // Check if position has any content
        let pos_has_content = self.position.default_x.is_some()
            || self.position.default_y.is_some()
            || self.position.relative_x.is_some()
            || self.position.relative_y.is_some();

        // Check if font has any content
        let font_has_content = self.font.font_family.is_some()
            || self.font.font_style.is_some()
            || self.font.font_size.is_some()
            || self.font.font_weight.is_some();

        let has_content = pos_has_content || font_has_content || self.color.is_some();

        if !has_content {
            return Sexpr::Nil;
        }

        let mut builder = ListBuilder::new("print-style");

        // Only add position if it has content
        if pos_has_content {
            builder = builder.kwarg_raw("position", self.position.to_sexpr());
        }

        // Only add font if it has content
        if font_has_content {
            builder = builder.kwarg_raw("font", self.font.to_sexpr());
        }

        // Add color if present
        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for PrintStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        // Handle nil as empty print style
        if sexpr.is_nil() || sexpr.is_symbol("nil") {
            return Ok(PrintStyle::default());
        }

        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("list or nil", sexpr))?;

        // Empty list is valid
        if list.is_empty() {
            return Ok(PrintStyle::default());
        }

        // Parse position (defaults to empty if not present)
        let position = match super::find_kwarg(list, "position") {
            Some(pos_sexpr) => Position::from_sexpr(pos_sexpr)?,
            None => Position::default(),
        };

        // Parse font (defaults to empty if not present)
        let font = match super::find_kwarg(list, "font") {
            Some(font_sexpr) => Font::from_sexpr(font_sexpr)?,
            None => Font::default(),
        };

        let color = optional_kwarg(list, "color")?;

        Ok(PrintStyle {
            position,
            font,
            color,
        })
    }
}

// ============================================================================
// FormattedText
// ============================================================================

impl ToSexpr for FormattedText {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("formatted-text").kwarg("value", &self.value);

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

        builder = builder.kwarg_opt("lang", &self.lang);

        builder.build()
    }
}

impl FromSexpr for FormattedText {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("formatted-text list", sexpr))?;

        // Check head
        if !list.first().is_some_and(|h| h.is_symbol("formatted-text")) {
            return Err(ConvertError::type_mismatch("formatted-text", sexpr));
        }

        let position = match super::find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(FormattedText {
            value: super::require_kwarg(list, "value")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
            lang: optional_kwarg(list, "lang")?,
        })
    }
}

// ============================================================================
// WavyLine
// ============================================================================

impl ToSexpr for WavyLine {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("wavy-line")
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

        builder.build()
    }
}

impl FromSexpr for WavyLine {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("wavy-line list", sexpr))?;

        // Check head
        if !list.first().is_some_and(|h| h.is_symbol("wavy-line")) {
            return Err(ConvertError::type_mismatch("wavy-line", sexpr));
        }

        let position = match super::find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(WavyLine {
            r#type: super::require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            position,
        })
    }
}

// ============================================================================
// AccidentalValue
// ============================================================================

impl ToSexpr for AccidentalValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
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
        })
    }
}

impl FromSexpr for AccidentalValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("sharp") => Ok(AccidentalValue::Sharp),
            Some("natural") => Ok(AccidentalValue::Natural),
            Some("flat") => Ok(AccidentalValue::Flat),
            Some("double-sharp") => Ok(AccidentalValue::DoubleSharp),
            Some("sharp-sharp") => Ok(AccidentalValue::SharpSharp),
            Some("flat-flat") => Ok(AccidentalValue::FlatFlat),
            Some("double-flat") => Ok(AccidentalValue::DoubleFlat),
            Some("natural-sharp") => Ok(AccidentalValue::NaturalSharp),
            Some("natural-flat") => Ok(AccidentalValue::NaturalFlat),
            Some("quarter-flat") => Ok(AccidentalValue::QuarterFlat),
            Some("quarter-sharp") => Ok(AccidentalValue::QuarterSharp),
            Some("three-quarters-flat") => Ok(AccidentalValue::ThreeQuartersFlat),
            Some("three-quarters-sharp") => Ok(AccidentalValue::ThreeQuartersSharp),
            Some("sharp-down") => Ok(AccidentalValue::SharpDown),
            Some("sharp-up") => Ok(AccidentalValue::SharpUp),
            Some("natural-down") => Ok(AccidentalValue::NaturalDown),
            Some("natural-up") => Ok(AccidentalValue::NaturalUp),
            Some("flat-down") => Ok(AccidentalValue::FlatDown),
            Some("flat-up") => Ok(AccidentalValue::FlatUp),
            Some("triple-sharp") => Ok(AccidentalValue::TripleSharp),
            Some("triple-flat") => Ok(AccidentalValue::TripleFlat),
            Some("slash-quarter-sharp") => Ok(AccidentalValue::SlashQuarterSharp),
            Some("slash-sharp") => Ok(AccidentalValue::SlashSharp),
            Some("slash-flat") => Ok(AccidentalValue::SlashFlat),
            Some("double-slash-flat") => Ok(AccidentalValue::DoubleSlashFlat),
            Some("sharp-1") => Ok(AccidentalValue::Sharp1),
            Some("sharp-2") => Ok(AccidentalValue::Sharp2),
            Some("sharp-3") => Ok(AccidentalValue::Sharp3),
            Some("sharp-5") => Ok(AccidentalValue::Sharp5),
            Some("flat-1") => Ok(AccidentalValue::Flat1),
            Some("flat-2") => Ok(AccidentalValue::Flat2),
            Some("flat-3") => Ok(AccidentalValue::Flat3),
            Some("flat-4") => Ok(AccidentalValue::Flat4),
            Some("sori") => Ok(AccidentalValue::Sori),
            Some("koron") => Ok(AccidentalValue::Koron),
            Some("other") => Ok(AccidentalValue::Other),
            _ => Err(ConvertError::type_mismatch("accidental value", sexpr)),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // === YesNo Tests ===

    #[test]
    fn test_yesno_yes_to_sexpr() {
        assert_eq!(YesNo::Yes.to_sexpr(), Sexpr::symbol("yes"));
    }

    #[test]
    fn test_yesno_no_to_sexpr() {
        assert_eq!(YesNo::No.to_sexpr(), Sexpr::symbol("no"));
    }

    #[test]
    fn test_yesno_round_trip_yes() {
        let original = YesNo::Yes;
        let sexpr = original.to_sexpr();
        let parsed = YesNo::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_yesno_round_trip_no() {
        let original = YesNo::No;
        let sexpr = original.to_sexpr();
        let parsed = YesNo::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_yesno_from_sexpr_invalid() {
        let sexpr = Sexpr::symbol("maybe");
        assert!(YesNo::from_sexpr(&sexpr).is_err());
    }

    // === StartStop Tests ===

    #[test]
    fn test_startstop_to_sexpr() {
        assert_eq!(StartStop::Start.to_sexpr(), Sexpr::symbol("start"));
        assert_eq!(StartStop::Stop.to_sexpr(), Sexpr::symbol("stop"));
    }

    #[test]
    fn test_startstop_round_trip() {
        for original in [StartStop::Start, StartStop::Stop] {
            let sexpr = original.to_sexpr();
            let parsed = StartStop::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === StartStopContinue Tests ===

    #[test]
    fn test_startstopcontinue_to_sexpr() {
        assert_eq!(StartStopContinue::Start.to_sexpr(), Sexpr::symbol("start"));
        assert_eq!(StartStopContinue::Stop.to_sexpr(), Sexpr::symbol("stop"));
        assert_eq!(
            StartStopContinue::Continue.to_sexpr(),
            Sexpr::symbol("continue")
        );
    }

    #[test]
    fn test_startstopcontinue_round_trip() {
        for original in [
            StartStopContinue::Start,
            StartStopContinue::Stop,
            StartStopContinue::Continue,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = StartStopContinue::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === StartStopSingle Tests ===

    #[test]
    fn test_startstopsingle_round_trip() {
        for original in [
            StartStopSingle::Start,
            StartStopSingle::Stop,
            StartStopSingle::Single,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = StartStopSingle::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === StartStopDiscontinue Tests ===

    #[test]
    fn test_startstopdiscontinue_round_trip() {
        for original in [
            StartStopDiscontinue::Start,
            StartStopDiscontinue::Stop,
            StartStopDiscontinue::Discontinue,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = StartStopDiscontinue::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === AboveBelow Tests ===

    #[test]
    fn test_abovebelow_round_trip() {
        for original in [AboveBelow::Above, AboveBelow::Below] {
            let sexpr = original.to_sexpr();
            let parsed = AboveBelow::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === UpDown Tests ===

    #[test]
    fn test_updown_round_trip() {
        for original in [UpDown::Up, UpDown::Down] {
            let sexpr = original.to_sexpr();
            let parsed = UpDown::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === OverUnder Tests ===

    #[test]
    fn test_overunder_round_trip() {
        for original in [OverUnder::Over, OverUnder::Under] {
            let sexpr = original.to_sexpr();
            let parsed = OverUnder::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === LeftCenterRight Tests ===

    #[test]
    fn test_leftcenterright_round_trip() {
        for original in [
            LeftCenterRight::Left,
            LeftCenterRight::Center,
            LeftCenterRight::Right,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = LeftCenterRight::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === TopMiddleBottom Tests ===

    #[test]
    fn test_topmiddlebottom_round_trip() {
        for original in [
            TopMiddleBottom::Top,
            TopMiddleBottom::Middle,
            TopMiddleBottom::Bottom,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = TopMiddleBottom::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === BackwardForward Tests ===

    #[test]
    fn test_backwardforward_round_trip() {
        for original in [BackwardForward::Backward, BackwardForward::Forward] {
            let sexpr = original.to_sexpr();
            let parsed = BackwardForward::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === RightLeftMiddle Tests ===

    #[test]
    fn test_rightleftmiddle_round_trip() {
        for original in [
            RightLeftMiddle::Right,
            RightLeftMiddle::Left,
            RightLeftMiddle::Middle,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = RightLeftMiddle::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === UprightInverted Tests ===

    #[test]
    fn test_uprightinverted_round_trip() {
        for original in [UprightInverted::Upright, UprightInverted::Inverted] {
            let sexpr = original.to_sexpr();
            let parsed = UprightInverted::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === SymbolSize Tests ===

    #[test]
    fn test_symbolsize_round_trip() {
        for original in [
            SymbolSize::Full,
            SymbolSize::Cue,
            SymbolSize::GraceCue,
            SymbolSize::Large,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = SymbolSize::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_symbolsize_grace_cue_kebab() {
        // GraceCue should serialize as "grace-cue"
        assert_eq!(SymbolSize::GraceCue.to_sexpr(), Sexpr::symbol("grace-cue"));
    }

    // === LineType Tests ===

    #[test]
    fn test_linetype_round_trip() {
        for original in [
            LineType::Solid,
            LineType::Dashed,
            LineType::Dotted,
            LineType::Wavy,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = LineType::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === FontStyle Tests ===

    #[test]
    fn test_fontstyle_round_trip() {
        for original in [FontStyle::Normal, FontStyle::Italic] {
            let sexpr = original.to_sexpr();
            let parsed = FontStyle::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === FontWeight Tests ===

    #[test]
    fn test_fontweight_round_trip() {
        for original in [FontWeight::Normal, FontWeight::Bold] {
            let sexpr = original.to_sexpr();
            let parsed = FontWeight::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    // === CssFontSize Tests ===

    #[test]
    fn test_cssfontsize_round_trip() {
        for original in [
            CssFontSize::XxSmall,
            CssFontSize::XSmall,
            CssFontSize::Small,
            CssFontSize::Medium,
            CssFontSize::Large,
            CssFontSize::XLarge,
            CssFontSize::XxLarge,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = CssFontSize::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_cssfontsize_kebab_case() {
        assert_eq!(CssFontSize::XxSmall.to_sexpr(), Sexpr::symbol("xx-small"));
        assert_eq!(CssFontSize::XLarge.to_sexpr(), Sexpr::symbol("x-large"));
    }

    // === FontSize Tests ===

    #[test]
    fn test_fontsize_css_round_trip() {
        let original = FontSize::Css(CssFontSize::Medium);
        let sexpr = original.to_sexpr();
        let parsed = FontSize::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_fontsize_points_round_trip() {
        let original = FontSize::Points(12.5);
        let sexpr = original.to_sexpr();
        let parsed = FontSize::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_fontsize_points_to_sexpr() {
        let fs = FontSize::Points(14.0);
        assert_eq!(fs.to_sexpr(), Sexpr::Float(14.0));
    }

    // === Position Tests ===

    #[test]
    fn test_position_empty_to_sexpr() {
        let pos = Position::default();
        assert_eq!(pos.to_sexpr(), Sexpr::Nil);
    }

    #[test]
    fn test_position_with_values_to_sexpr() {
        let pos = Position {
            default_x: Some(10.0),
            default_y: Some(20.0),
            relative_x: None,
            relative_y: None,
        };
        let sexpr = pos.to_sexpr();
        assert!(sexpr.is_list());
        let list = sexpr.as_list().unwrap();
        assert!(list[0].is_symbol("position"));
    }

    #[test]
    fn test_position_round_trip_empty() {
        let original = Position::default();
        let sexpr = original.to_sexpr();
        let parsed = Position::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_position_round_trip_with_values() {
        let original = Position {
            default_x: Some(10.5),
            default_y: Some(-20.0),
            relative_x: Some(5.0),
            relative_y: None,
        };
        let sexpr = original.to_sexpr();
        let parsed = Position::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_position_from_nil() {
        let sexpr = Sexpr::Nil;
        let pos = Position::from_sexpr(&sexpr).unwrap();
        assert_eq!(pos, Position::default());
    }

    // === Font Tests ===

    #[test]
    fn test_font_empty_to_sexpr() {
        let font = Font::default();
        assert_eq!(font.to_sexpr(), Sexpr::Nil);
    }

    #[test]
    fn test_font_with_values_to_sexpr() {
        let font = Font {
            font_family: Some("Arial".to_string()),
            font_style: Some(FontStyle::Italic),
            font_size: Some(FontSize::Points(12.0)),
            font_weight: Some(FontWeight::Bold),
        };
        let sexpr = font.to_sexpr();
        assert!(sexpr.is_list());
    }

    #[test]
    fn test_font_round_trip_empty() {
        let original = Font::default();
        let sexpr = original.to_sexpr();
        let parsed = Font::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_font_round_trip_with_values() {
        let original = Font {
            font_family: Some("Times New Roman".to_string()),
            font_style: Some(FontStyle::Normal),
            font_size: Some(FontSize::Css(CssFontSize::Large)),
            font_weight: None,
        };
        let sexpr = original.to_sexpr();
        let parsed = Font::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === PrintStyle Tests ===

    #[test]
    fn test_printstyle_empty_to_sexpr() {
        let ps = PrintStyle::default();
        assert_eq!(ps.to_sexpr(), Sexpr::Nil);
    }

    #[test]
    fn test_printstyle_with_color_only() {
        let ps = PrintStyle {
            position: Position::default(),
            font: Font::default(),
            color: Some("#FF0000".to_string()),
        };
        let sexpr = ps.to_sexpr();
        assert!(sexpr.is_list());
    }

    #[test]
    fn test_printstyle_round_trip_empty() {
        let original = PrintStyle::default();
        let sexpr = original.to_sexpr();
        let parsed = PrintStyle::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_printstyle_round_trip_with_all() {
        let original = PrintStyle {
            position: Position {
                default_x: Some(10.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(5.0),
            },
            font: Font {
                font_family: Some("Helvetica".to_string()),
                font_style: None,
                font_size: Some(FontSize::Points(11.0)),
                font_weight: Some(FontWeight::Bold),
            },
            color: Some("#00FF00".to_string()),
        };
        let sexpr = original.to_sexpr();
        let parsed = PrintStyle::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === AccidentalValue Tests ===

    #[test]
    fn test_accidentalvalue_basic_round_trip() {
        for original in [
            AccidentalValue::Sharp,
            AccidentalValue::Natural,
            AccidentalValue::Flat,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = AccidentalValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_accidentalvalue_double_round_trip() {
        for original in [
            AccidentalValue::DoubleSharp,
            AccidentalValue::DoubleFlat,
            AccidentalValue::SharpSharp,
            AccidentalValue::FlatFlat,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = AccidentalValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_accidentalvalue_quarter_tone_round_trip() {
        for original in [
            AccidentalValue::QuarterFlat,
            AccidentalValue::QuarterSharp,
            AccidentalValue::ThreeQuartersFlat,
            AccidentalValue::ThreeQuartersSharp,
        ] {
            let sexpr = original.to_sexpr();
            let parsed = AccidentalValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }

    #[test]
    fn test_accidentalvalue_kebab_case() {
        assert_eq!(
            AccidentalValue::DoubleSharp.to_sexpr(),
            Sexpr::symbol("double-sharp")
        );
        assert_eq!(
            AccidentalValue::ThreeQuartersFlat.to_sexpr(),
            Sexpr::symbol("three-quarters-flat")
        );
    }

    #[test]
    fn test_accidentalvalue_invalid() {
        let sexpr = Sexpr::symbol("unknown-accidental");
        assert!(AccidentalValue::from_sexpr(&sexpr).is_err());
    }
}
