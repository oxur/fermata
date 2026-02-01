//! S-expression conversions for `ir::beam` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for beam-related types:
//! - [`BeamValue`] - Beam connection type (begin, continue, end, hooks)
//! - [`Beam`] - Beam element for note grouping
//! - [`Fan`] - Beam fan for feathered beams
//! - [`StemValue`] - Stem direction values
//! - [`Stem`] - Stem direction with positioning
//! - [`NoteheadValue`] - Notehead shape values
//! - [`Notehead`] - Notehead with shape, fill, and styling

use crate::ir::beam::{Beam, BeamValue, Fan, Notehead, NoteheadValue, Stem, StemValue};
use crate::ir::common::Font;
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, optional_kwarg, require_kwarg};

// ============================================================================
// BeamValue
// ============================================================================

impl ToSexpr for BeamValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BeamValue::Begin => "begin",
            BeamValue::Continue => "continue",
            BeamValue::End => "end",
            BeamValue::ForwardHook => "forward-hook",
            BeamValue::BackwardHook => "backward-hook",
        })
    }
}

impl FromSexpr for BeamValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("begin") => Ok(BeamValue::Begin),
            Some("continue") => Ok(BeamValue::Continue),
            Some("end") => Ok(BeamValue::End),
            Some("forward-hook") => Ok(BeamValue::ForwardHook),
            Some("backward-hook") => Ok(BeamValue::BackwardHook),
            _ => Err(ConvertError::type_mismatch("beam-value", sexpr)),
        }
    }
}

// ============================================================================
// Fan
// ============================================================================

impl ToSexpr for Fan {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Fan::Accel => "accel",
            Fan::Rit => "rit",
            Fan::None => "none",
        })
    }
}

impl FromSexpr for Fan {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("accel") => Ok(Fan::Accel),
            Some("rit") => Ok(Fan::Rit),
            Some("none") => Ok(Fan::None),
            _ => Err(ConvertError::type_mismatch("fan", sexpr)),
        }
    }
}

// ============================================================================
// Beam
// ============================================================================

impl ToSexpr for Beam {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("beam")
            .kwarg("value", &self.value)
            .kwarg("number", &self.number)
            .kwarg_opt("fan", &self.fan)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

impl FromSexpr for Beam {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("beam list", sexpr))?;

        expect_head(list, "beam")?;

        Ok(Beam {
            value: require_kwarg(list, "value")?,
            number: require_kwarg(list, "number")?,
            fan: optional_kwarg(list, "fan")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// StemValue
// ============================================================================

impl ToSexpr for StemValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StemValue::Down => "down",
            StemValue::Up => "up",
            StemValue::Double => "double",
            StemValue::None => "none",
        })
    }
}

impl FromSexpr for StemValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("down") => Ok(StemValue::Down),
            Some("up") => Ok(StemValue::Up),
            Some("double") => Ok(StemValue::Double),
            Some("none") => Ok(StemValue::None),
            _ => Err(ConvertError::type_mismatch("stem-value", sexpr)),
        }
    }
}

// ============================================================================
// Stem
// ============================================================================

impl ToSexpr for Stem {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("stem")
            .kwarg("value", &self.value)
            .kwarg_opt("default-y", &self.default_y)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

impl FromSexpr for Stem {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("stem list", sexpr))?;

        expect_head(list, "stem")?;

        Ok(Stem {
            value: require_kwarg(list, "value")?,
            default_y: optional_kwarg(list, "default-y")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// NoteheadValue
// ============================================================================

impl ToSexpr for NoteheadValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            NoteheadValue::Slash => "slash",
            NoteheadValue::Triangle => "triangle",
            NoteheadValue::Diamond => "diamond",
            NoteheadValue::Square => "square",
            NoteheadValue::Cross => "cross",
            NoteheadValue::X => "x",
            NoteheadValue::CircleX => "circle-x",
            NoteheadValue::InvertedTriangle => "inverted-triangle",
            NoteheadValue::ArrowDown => "arrow-down",
            NoteheadValue::ArrowUp => "arrow-up",
            NoteheadValue::Circled => "circled",
            NoteheadValue::Slashed => "slashed",
            NoteheadValue::BackSlashed => "back-slashed",
            NoteheadValue::Normal => "normal",
            NoteheadValue::Cluster => "cluster",
            NoteheadValue::CircleDot => "circle-dot",
            NoteheadValue::LeftTriangle => "left-triangle",
            NoteheadValue::Rectangle => "rectangle",
            NoteheadValue::None => "none",
            NoteheadValue::Do => "do",
            NoteheadValue::Re => "re",
            NoteheadValue::Mi => "mi",
            NoteheadValue::Fa => "fa",
            NoteheadValue::FaUp => "fa-up",
            NoteheadValue::So => "so",
            NoteheadValue::La => "la",
            NoteheadValue::Ti => "ti",
            NoteheadValue::Other => "other",
        })
    }
}

impl FromSexpr for NoteheadValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("slash") => Ok(NoteheadValue::Slash),
            Some("triangle") => Ok(NoteheadValue::Triangle),
            Some("diamond") => Ok(NoteheadValue::Diamond),
            Some("square") => Ok(NoteheadValue::Square),
            Some("cross") => Ok(NoteheadValue::Cross),
            Some("x") => Ok(NoteheadValue::X),
            Some("circle-x") => Ok(NoteheadValue::CircleX),
            Some("inverted-triangle") => Ok(NoteheadValue::InvertedTriangle),
            Some("arrow-down") => Ok(NoteheadValue::ArrowDown),
            Some("arrow-up") => Ok(NoteheadValue::ArrowUp),
            Some("circled") => Ok(NoteheadValue::Circled),
            Some("slashed") => Ok(NoteheadValue::Slashed),
            Some("back-slashed") => Ok(NoteheadValue::BackSlashed),
            Some("normal") => Ok(NoteheadValue::Normal),
            Some("cluster") => Ok(NoteheadValue::Cluster),
            Some("circle-dot") => Ok(NoteheadValue::CircleDot),
            Some("left-triangle") => Ok(NoteheadValue::LeftTriangle),
            Some("rectangle") => Ok(NoteheadValue::Rectangle),
            Some("none") => Ok(NoteheadValue::None),
            Some("do") => Ok(NoteheadValue::Do),
            Some("re") => Ok(NoteheadValue::Re),
            Some("mi") => Ok(NoteheadValue::Mi),
            Some("fa") => Ok(NoteheadValue::Fa),
            Some("fa-up") => Ok(NoteheadValue::FaUp),
            Some("so") => Ok(NoteheadValue::So),
            Some("la") => Ok(NoteheadValue::La),
            Some("ti") => Ok(NoteheadValue::Ti),
            Some("other") => Ok(NoteheadValue::Other),
            _ => Err(ConvertError::type_mismatch("notehead-value", sexpr)),
        }
    }
}

// ============================================================================
// Notehead
// ============================================================================

impl ToSexpr for Notehead {
    fn to_sexpr(&self) -> Sexpr {
        // Check if font has any content
        let font_has_content = self.font.font_family.is_some()
            || self.font.font_style.is_some()
            || self.font.font_size.is_some()
            || self.font.font_weight.is_some();

        let mut builder = ListBuilder::new("notehead")
            .kwarg("value", &self.value)
            .kwarg_opt("filled", &self.filled)
            .kwarg_opt("parentheses", &self.parentheses);

        // Only add font if it has content
        if font_has_content {
            builder = builder.kwarg_raw("font", self.font.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for Notehead {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("notehead list", sexpr))?;

        expect_head(list, "notehead")?;

        // Parse font (defaults to empty if not present)
        let font = match find_kwarg(list, "font") {
            Some(font_sexpr) => Font::from_sexpr(font_sexpr)?,
            None => Font::default(),
        };

        Ok(Notehead {
            value: require_kwarg(list, "value")?,
            filled: optional_kwarg(list, "filled")?,
            parentheses: optional_kwarg(list, "parentheses")?,
            font,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::YesNo;
    use crate::sexpr::{parse, print_sexpr};

    // === BeamValue Tests ===

    #[test]
    fn test_beamvalue_to_sexpr() {
        assert_eq!(BeamValue::Begin.to_sexpr(), Sexpr::symbol("begin"));
        assert_eq!(BeamValue::Continue.to_sexpr(), Sexpr::symbol("continue"));
        assert_eq!(BeamValue::End.to_sexpr(), Sexpr::symbol("end"));
        assert_eq!(
            BeamValue::ForwardHook.to_sexpr(),
            Sexpr::symbol("forward-hook")
        );
        assert_eq!(
            BeamValue::BackwardHook.to_sexpr(),
            Sexpr::symbol("backward-hook")
        );
    }

    #[test]
    fn test_beamvalue_from_sexpr() {
        assert_eq!(
            BeamValue::from_sexpr(&Sexpr::symbol("begin")).unwrap(),
            BeamValue::Begin
        );
        assert_eq!(
            BeamValue::from_sexpr(&Sexpr::symbol("continue")).unwrap(),
            BeamValue::Continue
        );
        assert_eq!(
            BeamValue::from_sexpr(&Sexpr::symbol("end")).unwrap(),
            BeamValue::End
        );
        assert_eq!(
            BeamValue::from_sexpr(&Sexpr::symbol("forward-hook")).unwrap(),
            BeamValue::ForwardHook
        );
        assert_eq!(
            BeamValue::from_sexpr(&Sexpr::symbol("backward-hook")).unwrap(),
            BeamValue::BackwardHook
        );
    }

    #[test]
    fn test_beamvalue_from_sexpr_invalid() {
        assert!(BeamValue::from_sexpr(&Sexpr::symbol("unknown")).is_err());
        assert!(BeamValue::from_sexpr(&Sexpr::Integer(1)).is_err());
    }

    #[test]
    fn test_beamvalue_round_trip() {
        for value in [
            BeamValue::Begin,
            BeamValue::Continue,
            BeamValue::End,
            BeamValue::ForwardHook,
            BeamValue::BackwardHook,
        ] {
            let sexpr = value.to_sexpr();
            let parsed = BeamValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(value, parsed);
        }
    }

    // === Fan Tests ===

    #[test]
    fn test_fan_to_sexpr() {
        assert_eq!(Fan::Accel.to_sexpr(), Sexpr::symbol("accel"));
        assert_eq!(Fan::Rit.to_sexpr(), Sexpr::symbol("rit"));
        assert_eq!(Fan::None.to_sexpr(), Sexpr::symbol("none"));
    }

    #[test]
    fn test_fan_from_sexpr() {
        assert_eq!(Fan::from_sexpr(&Sexpr::symbol("accel")).unwrap(), Fan::Accel);
        assert_eq!(Fan::from_sexpr(&Sexpr::symbol("rit")).unwrap(), Fan::Rit);
        assert_eq!(Fan::from_sexpr(&Sexpr::symbol("none")).unwrap(), Fan::None);
    }

    #[test]
    fn test_fan_round_trip() {
        for value in [Fan::Accel, Fan::Rit, Fan::None] {
            let sexpr = value.to_sexpr();
            let parsed = Fan::from_sexpr(&sexpr).unwrap();
            assert_eq!(value, parsed);
        }
    }

    // === Beam Tests ===

    #[test]
    fn test_beam_to_sexpr_simple() {
        let beam = Beam {
            value: BeamValue::Begin,
            number: 1,
            fan: None,
            color: None,
        };
        let sexpr = beam.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(beam :value begin :number 1)");
    }

    #[test]
    fn test_beam_to_sexpr_with_fan() {
        let beam = Beam {
            value: BeamValue::Continue,
            number: 1,
            fan: Some(Fan::Accel),
            color: None,
        };
        let sexpr = beam.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(beam :value continue :number 1 :fan accel)");
    }

    #[test]
    fn test_beam_to_sexpr_with_color() {
        let beam = Beam {
            value: BeamValue::End,
            number: 2,
            fan: None,
            color: Some("#0000FF".to_string()),
        };
        let sexpr = beam.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":color \"#0000FF\""));
    }

    #[test]
    fn test_beam_from_sexpr_simple() {
        let sexpr = parse("(beam :value begin :number 1)").unwrap();
        let beam = Beam::from_sexpr(&sexpr).unwrap();
        assert_eq!(beam.value, BeamValue::Begin);
        assert_eq!(beam.number, 1);
        assert!(beam.fan.is_none());
        assert!(beam.color.is_none());
    }

    #[test]
    fn test_beam_from_sexpr_with_fan() {
        let sexpr = parse("(beam :value continue :number 1 :fan rit)").unwrap();
        let beam = Beam::from_sexpr(&sexpr).unwrap();
        assert_eq!(beam.value, BeamValue::Continue);
        assert_eq!(beam.fan, Some(Fan::Rit));
    }

    #[test]
    fn test_beam_from_sexpr_missing_value() {
        let sexpr = parse("(beam :number 1)").unwrap();
        assert!(Beam::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_beam_from_sexpr_missing_number() {
        let sexpr = parse("(beam :value begin)").unwrap();
        assert!(Beam::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_beam_round_trip() {
        let original = Beam {
            value: BeamValue::ForwardHook,
            number: 2,
            fan: Some(Fan::Accel),
            color: Some("#FF0000".to_string()),
        };
        let sexpr = original.to_sexpr();
        let parsed = Beam::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === StemValue Tests ===

    #[test]
    fn test_stemvalue_to_sexpr() {
        assert_eq!(StemValue::Down.to_sexpr(), Sexpr::symbol("down"));
        assert_eq!(StemValue::Up.to_sexpr(), Sexpr::symbol("up"));
        assert_eq!(StemValue::Double.to_sexpr(), Sexpr::symbol("double"));
        assert_eq!(StemValue::None.to_sexpr(), Sexpr::symbol("none"));
    }

    #[test]
    fn test_stemvalue_from_sexpr() {
        assert_eq!(
            StemValue::from_sexpr(&Sexpr::symbol("down")).unwrap(),
            StemValue::Down
        );
        assert_eq!(
            StemValue::from_sexpr(&Sexpr::symbol("up")).unwrap(),
            StemValue::Up
        );
        assert_eq!(
            StemValue::from_sexpr(&Sexpr::symbol("double")).unwrap(),
            StemValue::Double
        );
        assert_eq!(
            StemValue::from_sexpr(&Sexpr::symbol("none")).unwrap(),
            StemValue::None
        );
    }

    #[test]
    fn test_stemvalue_round_trip() {
        for value in [
            StemValue::Down,
            StemValue::Up,
            StemValue::Double,
            StemValue::None,
        ] {
            let sexpr = value.to_sexpr();
            let parsed = StemValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(value, parsed);
        }
    }

    // === Stem Tests ===

    #[test]
    fn test_stem_to_sexpr_simple() {
        let stem = Stem {
            value: StemValue::Up,
            default_y: None,
            color: None,
        };
        let sexpr = stem.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(stem :value up)");
    }

    #[test]
    fn test_stem_to_sexpr_with_default_y() {
        let stem = Stem {
            value: StemValue::Down,
            default_y: Some(-35.0),
            color: None,
        };
        let sexpr = stem.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(stem :value down :default-y -35.0)");
    }

    #[test]
    fn test_stem_to_sexpr_with_color() {
        let stem = Stem {
            value: StemValue::Up,
            default_y: Some(35.0),
            color: Some("#000000".to_string()),
        };
        let sexpr = stem.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":color \"#000000\""));
    }

    #[test]
    fn test_stem_from_sexpr_simple() {
        let sexpr = parse("(stem :value up)").unwrap();
        let stem = Stem::from_sexpr(&sexpr).unwrap();
        assert_eq!(stem.value, StemValue::Up);
        assert!(stem.default_y.is_none());
        assert!(stem.color.is_none());
    }

    #[test]
    fn test_stem_from_sexpr_with_default_y() {
        let sexpr = parse("(stem :value down :default-y -35.0)").unwrap();
        let stem = Stem::from_sexpr(&sexpr).unwrap();
        assert_eq!(stem.value, StemValue::Down);
        assert_eq!(stem.default_y, Some(-35.0));
    }

    #[test]
    fn test_stem_from_sexpr_missing_value() {
        let sexpr = parse("(stem :default-y 10.0)").unwrap();
        assert!(Stem::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_stem_round_trip() {
        let original = Stem {
            value: StemValue::Up,
            default_y: Some(40.0),
            color: Some("#333333".to_string()),
        };
        let sexpr = original.to_sexpr();
        let parsed = Stem::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === NoteheadValue Tests ===

    #[test]
    fn test_noteheadvalue_to_sexpr_standard() {
        assert_eq!(NoteheadValue::Normal.to_sexpr(), Sexpr::symbol("normal"));
        assert_eq!(NoteheadValue::Diamond.to_sexpr(), Sexpr::symbol("diamond"));
        assert_eq!(
            NoteheadValue::Triangle.to_sexpr(),
            Sexpr::symbol("triangle")
        );
        assert_eq!(NoteheadValue::Square.to_sexpr(), Sexpr::symbol("square"));
        assert_eq!(NoteheadValue::Slash.to_sexpr(), Sexpr::symbol("slash"));
    }

    #[test]
    fn test_noteheadvalue_to_sexpr_cross() {
        assert_eq!(NoteheadValue::Cross.to_sexpr(), Sexpr::symbol("cross"));
        assert_eq!(NoteheadValue::X.to_sexpr(), Sexpr::symbol("x"));
        assert_eq!(NoteheadValue::CircleX.to_sexpr(), Sexpr::symbol("circle-x"));
    }

    #[test]
    fn test_noteheadvalue_to_sexpr_solfege() {
        assert_eq!(NoteheadValue::Do.to_sexpr(), Sexpr::symbol("do"));
        assert_eq!(NoteheadValue::Re.to_sexpr(), Sexpr::symbol("re"));
        assert_eq!(NoteheadValue::Mi.to_sexpr(), Sexpr::symbol("mi"));
        assert_eq!(NoteheadValue::Fa.to_sexpr(), Sexpr::symbol("fa"));
        assert_eq!(NoteheadValue::FaUp.to_sexpr(), Sexpr::symbol("fa-up"));
        assert_eq!(NoteheadValue::So.to_sexpr(), Sexpr::symbol("so"));
        assert_eq!(NoteheadValue::La.to_sexpr(), Sexpr::symbol("la"));
        assert_eq!(NoteheadValue::Ti.to_sexpr(), Sexpr::symbol("ti"));
    }

    #[test]
    fn test_noteheadvalue_from_sexpr() {
        assert_eq!(
            NoteheadValue::from_sexpr(&Sexpr::symbol("normal")).unwrap(),
            NoteheadValue::Normal
        );
        assert_eq!(
            NoteheadValue::from_sexpr(&Sexpr::symbol("diamond")).unwrap(),
            NoteheadValue::Diamond
        );
        assert_eq!(
            NoteheadValue::from_sexpr(&Sexpr::symbol("inverted-triangle")).unwrap(),
            NoteheadValue::InvertedTriangle
        );
    }

    #[test]
    fn test_noteheadvalue_from_sexpr_invalid() {
        assert!(NoteheadValue::from_sexpr(&Sexpr::symbol("unknown")).is_err());
    }

    #[test]
    fn test_noteheadvalue_round_trip() {
        for value in [
            NoteheadValue::Slash,
            NoteheadValue::Triangle,
            NoteheadValue::Diamond,
            NoteheadValue::Square,
            NoteheadValue::Cross,
            NoteheadValue::X,
            NoteheadValue::CircleX,
            NoteheadValue::InvertedTriangle,
            NoteheadValue::ArrowDown,
            NoteheadValue::ArrowUp,
            NoteheadValue::Circled,
            NoteheadValue::Slashed,
            NoteheadValue::BackSlashed,
            NoteheadValue::Normal,
            NoteheadValue::Cluster,
            NoteheadValue::CircleDot,
            NoteheadValue::LeftTriangle,
            NoteheadValue::Rectangle,
            NoteheadValue::None,
            NoteheadValue::Do,
            NoteheadValue::Re,
            NoteheadValue::Mi,
            NoteheadValue::Fa,
            NoteheadValue::FaUp,
            NoteheadValue::So,
            NoteheadValue::La,
            NoteheadValue::Ti,
            NoteheadValue::Other,
        ] {
            let sexpr = value.to_sexpr();
            let parsed = NoteheadValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(value, parsed);
        }
    }

    // === Notehead Tests ===

    #[test]
    fn test_notehead_to_sexpr_simple() {
        let notehead = Notehead {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: None,
            font: Font::default(),
            color: None,
        };
        let sexpr = notehead.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(notehead :value normal)");
    }

    #[test]
    fn test_notehead_to_sexpr_with_filled() {
        let notehead = Notehead {
            value: NoteheadValue::Diamond,
            filled: Some(YesNo::Yes),
            parentheses: None,
            font: Font::default(),
            color: None,
        };
        let sexpr = notehead.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(notehead :value diamond :filled yes)");
    }

    #[test]
    fn test_notehead_to_sexpr_with_parentheses() {
        let notehead = Notehead {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: Some(YesNo::Yes),
            font: Font::default(),
            color: None,
        };
        let sexpr = notehead.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":parentheses yes"));
    }

    #[test]
    fn test_notehead_to_sexpr_with_color() {
        let notehead = Notehead {
            value: NoteheadValue::X,
            filled: None,
            parentheses: None,
            font: Font::default(),
            color: Some("#FF0000".to_string()),
        };
        let sexpr = notehead.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":color \"#FF0000\""));
    }

    #[test]
    fn test_notehead_from_sexpr_simple() {
        let sexpr = parse("(notehead :value normal)").unwrap();
        let notehead = Notehead::from_sexpr(&sexpr).unwrap();
        assert_eq!(notehead.value, NoteheadValue::Normal);
        assert!(notehead.filled.is_none());
        assert!(notehead.parentheses.is_none());
        assert_eq!(notehead.font, Font::default());
        assert!(notehead.color.is_none());
    }

    #[test]
    fn test_notehead_from_sexpr_with_filled() {
        let sexpr = parse("(notehead :value diamond :filled yes)").unwrap();
        let notehead = Notehead::from_sexpr(&sexpr).unwrap();
        assert_eq!(notehead.value, NoteheadValue::Diamond);
        assert_eq!(notehead.filled, Some(YesNo::Yes));
    }

    #[test]
    fn test_notehead_from_sexpr_missing_value() {
        let sexpr = parse("(notehead :filled yes)").unwrap();
        assert!(Notehead::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_notehead_round_trip() {
        let original = Notehead {
            value: NoteheadValue::Triangle,
            filled: Some(YesNo::No),
            parentheses: Some(YesNo::Yes),
            font: Font::default(),
            color: Some("#0000FF".to_string()),
        };
        let sexpr = original.to_sexpr();
        let parsed = Notehead::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_notehead_round_trip_with_all_values() {
        // Test various notehead values
        for value in [
            NoteheadValue::Normal,
            NoteheadValue::Diamond,
            NoteheadValue::X,
            NoteheadValue::Slash,
            NoteheadValue::Do,
        ] {
            let original = Notehead {
                value,
                filled: None,
                parentheses: None,
                font: Font::default(),
                color: None,
            };
            let sexpr = original.to_sexpr();
            let parsed = Notehead::from_sexpr(&sexpr).unwrap();
            assert_eq!(original, parsed);
        }
    }
}
