//! S-expression conversions for `ir::pitch` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for pitch-related types:
//! - [`Step`] - The seven natural pitch steps (A-G)
//! - [`Pitch`] - A musical pitch with step, alteration, and octave
//! - [`Unpitched`] - An unpitched note (percussion) with optional display position

use crate::ir::pitch::{Pitch, Step, Unpitched};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, optional_kwarg, require_kwarg};

// ============================================================================
// Step
// ============================================================================

impl ToSexpr for Step {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Step::A => "A",
            Step::B => "B",
            Step::C => "C",
            Step::D => "D",
            Step::E => "E",
            Step::F => "F",
            Step::G => "G",
        })
    }
}

impl FromSexpr for Step {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("A") | Some("a") => Ok(Step::A),
            Some("B") | Some("b") => Ok(Step::B),
            Some("C") | Some("c") => Ok(Step::C),
            Some("D") | Some("d") => Ok(Step::D),
            Some("E") | Some("e") => Ok(Step::E),
            Some("F") | Some("f") => Ok(Step::F),
            Some("G") | Some("g") => Ok(Step::G),
            _ => Err(ConvertError::type_mismatch("step (A-G)", sexpr)),
        }
    }
}

// ============================================================================
// Pitch
// ============================================================================

impl ToSexpr for Pitch {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("pitch")
            .kwarg("step", &self.step)
            .kwarg_opt("alter", &self.alter)
            .kwarg("octave", &self.octave)
            .build()
    }
}

impl FromSexpr for Pitch {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pitch list", sexpr))?;

        expect_head(list, "pitch")?;

        Ok(Pitch {
            step: require_kwarg(list, "step")?,
            alter: optional_kwarg(list, "alter")?,
            octave: require_kwarg(list, "octave")?,
        })
    }
}

// ============================================================================
// Unpitched
// ============================================================================

impl ToSexpr for Unpitched {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("unpitched")
            .kwarg_opt("display-step", &self.display_step)
            .kwarg_opt("display-octave", &self.display_octave)
            .build()
    }
}

impl FromSexpr for Unpitched {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("unpitched list", sexpr))?;

        expect_head(list, "unpitched")?;

        Ok(Unpitched {
            display_step: optional_kwarg(list, "display-step")?,
            display_octave: optional_kwarg(list, "display-octave")?,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::{parse, print_sexpr};

    // === Step Tests ===

    #[test]
    fn test_step_to_sexpr() {
        assert_eq!(Step::A.to_sexpr(), Sexpr::symbol("A"));
        assert_eq!(Step::B.to_sexpr(), Sexpr::symbol("B"));
        assert_eq!(Step::C.to_sexpr(), Sexpr::symbol("C"));
        assert_eq!(Step::D.to_sexpr(), Sexpr::symbol("D"));
        assert_eq!(Step::E.to_sexpr(), Sexpr::symbol("E"));
        assert_eq!(Step::F.to_sexpr(), Sexpr::symbol("F"));
        assert_eq!(Step::G.to_sexpr(), Sexpr::symbol("G"));
    }

    #[test]
    fn test_step_from_sexpr_uppercase() {
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("A")).unwrap(), Step::A);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("B")).unwrap(), Step::B);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("C")).unwrap(), Step::C);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("D")).unwrap(), Step::D);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("E")).unwrap(), Step::E);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("F")).unwrap(), Step::F);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("G")).unwrap(), Step::G);
    }

    #[test]
    fn test_step_from_sexpr_lowercase() {
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("a")).unwrap(), Step::A);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("b")).unwrap(), Step::B);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("c")).unwrap(), Step::C);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("d")).unwrap(), Step::D);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("e")).unwrap(), Step::E);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("f")).unwrap(), Step::F);
        assert_eq!(Step::from_sexpr(&Sexpr::symbol("g")).unwrap(), Step::G);
    }

    #[test]
    fn test_step_from_sexpr_invalid() {
        assert!(Step::from_sexpr(&Sexpr::symbol("H")).is_err());
        assert!(Step::from_sexpr(&Sexpr::symbol("X")).is_err());
        assert!(Step::from_sexpr(&Sexpr::Integer(0)).is_err());
    }

    #[test]
    fn test_step_round_trip() {
        for step in [
            Step::A,
            Step::B,
            Step::C,
            Step::D,
            Step::E,
            Step::F,
            Step::G,
        ] {
            let sexpr = step.to_sexpr();
            let parsed = Step::from_sexpr(&sexpr).unwrap();
            assert_eq!(step, parsed);
        }
    }

    // === Pitch Tests ===

    #[test]
    fn test_pitch_to_sexpr_simple() {
        let pitch = Pitch {
            step: Step::C,
            alter: None,
            octave: 4,
        };
        let sexpr = pitch.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(pitch :step C :octave 4)");
    }

    #[test]
    fn test_pitch_to_sexpr_with_alter() {
        let pitch = Pitch {
            step: Step::F,
            alter: Some(1.0),
            octave: 4,
        };
        let sexpr = pitch.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(pitch :step F :alter 1.0 :octave 4)");
    }

    #[test]
    fn test_pitch_to_sexpr_with_flat() {
        let pitch = Pitch {
            step: Step::B,
            alter: Some(-1.0),
            octave: 3,
        };
        let sexpr = pitch.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(pitch :step B :alter -1.0 :octave 3)");
    }

    #[test]
    fn test_pitch_to_sexpr_with_quarter_tone() {
        let pitch = Pitch {
            step: Step::D,
            alter: Some(0.5),
            octave: 5,
        };
        let sexpr = pitch.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(pitch :step D :alter 0.5 :octave 5)");
    }

    #[test]
    fn test_pitch_from_sexpr_simple() {
        let sexpr = parse("(pitch :step C :octave 4)").unwrap();
        let pitch = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, Step::C);
        assert_eq!(pitch.alter, None);
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_pitch_from_sexpr_with_alter() {
        let sexpr = parse("(pitch :step F :alter 1.0 :octave 4)").unwrap();
        let pitch = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(pitch.step, Step::F);
        assert_eq!(pitch.alter, Some(1.0));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_pitch_from_sexpr_missing_step() {
        let sexpr = parse("(pitch :octave 4)").unwrap();
        assert!(Pitch::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_pitch_from_sexpr_missing_octave() {
        let sexpr = parse("(pitch :step C)").unwrap();
        assert!(Pitch::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_pitch_from_sexpr_wrong_head() {
        let sexpr = parse("(note :step C :octave 4)").unwrap();
        assert!(Pitch::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_pitch_round_trip_simple() {
        let original = Pitch {
            step: Step::C,
            alter: None,
            octave: 4,
        };
        let sexpr = original.to_sexpr();
        let parsed = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_pitch_round_trip_with_alter() {
        let original = Pitch {
            step: Step::F,
            alter: Some(1.0),
            octave: 4,
        };
        let sexpr = original.to_sexpr();
        let parsed = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_pitch_round_trip_with_negative_alter() {
        let original = Pitch {
            step: Step::B,
            alter: Some(-1.0),
            octave: 3,
        };
        let sexpr = original.to_sexpr();
        let parsed = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_pitch_round_trip_extreme_octaves() {
        // Low octave
        let low = Pitch {
            step: Step::A,
            alter: None,
            octave: 0,
        };
        let sexpr = low.to_sexpr();
        let parsed = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(low, parsed);

        // High octave
        let high = Pitch {
            step: Step::C,
            alter: None,
            octave: 9,
        };
        let sexpr = high.to_sexpr();
        let parsed = Pitch::from_sexpr(&sexpr).unwrap();
        assert_eq!(high, parsed);
    }

    // === Unpitched Tests ===

    #[test]
    fn test_unpitched_to_sexpr_empty() {
        let unpitched = Unpitched::default();
        let sexpr = unpitched.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(unpitched)");
    }

    #[test]
    fn test_unpitched_to_sexpr_with_display() {
        let unpitched = Unpitched {
            display_step: Some(Step::E),
            display_octave: Some(4),
        };
        let sexpr = unpitched.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(unpitched :display-step E :display-octave 4)");
    }

    #[test]
    fn test_unpitched_to_sexpr_step_only() {
        let unpitched = Unpitched {
            display_step: Some(Step::G),
            display_octave: None,
        };
        let sexpr = unpitched.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(unpitched :display-step G)");
    }

    #[test]
    fn test_unpitched_to_sexpr_octave_only() {
        let unpitched = Unpitched {
            display_step: None,
            display_octave: Some(5),
        };
        let sexpr = unpitched.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(unpitched :display-octave 5)");
    }

    #[test]
    fn test_unpitched_from_sexpr_empty() {
        let sexpr = parse("(unpitched)").unwrap();
        let unpitched = Unpitched::from_sexpr(&sexpr).unwrap();
        assert!(unpitched.display_step.is_none());
        assert!(unpitched.display_octave.is_none());
    }

    #[test]
    fn test_unpitched_from_sexpr_with_display() {
        let sexpr = parse("(unpitched :display-step E :display-octave 4)").unwrap();
        let unpitched = Unpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(unpitched.display_step, Some(Step::E));
        assert_eq!(unpitched.display_octave, Some(4));
    }

    #[test]
    fn test_unpitched_from_sexpr_wrong_head() {
        let sexpr = parse("(pitch :step C)").unwrap();
        assert!(Unpitched::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_unpitched_round_trip_empty() {
        let original = Unpitched::default();
        let sexpr = original.to_sexpr();
        let parsed = Unpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_unpitched_round_trip_with_display() {
        let original = Unpitched {
            display_step: Some(Step::F),
            display_octave: Some(3),
        };
        let sexpr = original.to_sexpr();
        let parsed = Unpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_unpitched_round_trip_partial() {
        // Only step
        let step_only = Unpitched {
            display_step: Some(Step::A),
            display_octave: None,
        };
        let sexpr = step_only.to_sexpr();
        let parsed = Unpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(step_only, parsed);

        // Only octave
        let octave_only = Unpitched {
            display_step: None,
            display_octave: Some(5),
        };
        let sexpr = octave_only.to_sexpr();
        let parsed = Unpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(octave_only, parsed);
    }
}
