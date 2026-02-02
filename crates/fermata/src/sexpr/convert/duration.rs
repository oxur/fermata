//! S-expression conversions for `ir::duration` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for duration-related types:
//! - [`NoteTypeValue`] - Enumeration of note duration symbols (quarter, eighth, etc.)
//! - [`NoteType`] - Note type with optional size modifier
//! - [`Dot`] - Augmentation dot with placement and position
//! - [`TimeModification`] - Time modification for tuplets

use crate::ir::common::Position;
use crate::ir::duration::{Dot, NoteType, NoteTypeValue, TimeModification};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, optional_kwarg, require_kwarg};

// ============================================================================
// NoteTypeValue
// ============================================================================

impl ToSexpr for NoteTypeValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            NoteTypeValue::N1024th => "1024th",
            NoteTypeValue::N512th => "512th",
            NoteTypeValue::N256th => "256th",
            NoteTypeValue::N128th => "128th",
            NoteTypeValue::N64th => "64th",
            NoteTypeValue::N32nd => "32nd",
            NoteTypeValue::N16th => "16th",
            NoteTypeValue::Eighth => "eighth",
            NoteTypeValue::Quarter => "quarter",
            NoteTypeValue::Half => "half",
            NoteTypeValue::Whole => "whole",
            NoteTypeValue::Breve => "breve",
            NoteTypeValue::Long => "long",
            NoteTypeValue::Maxima => "maxima",
        })
    }
}

impl FromSexpr for NoteTypeValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("1024th") => Ok(NoteTypeValue::N1024th),
            Some("512th") => Ok(NoteTypeValue::N512th),
            Some("256th") => Ok(NoteTypeValue::N256th),
            Some("128th") => Ok(NoteTypeValue::N128th),
            Some("64th") => Ok(NoteTypeValue::N64th),
            Some("32nd") => Ok(NoteTypeValue::N32nd),
            Some("16th") => Ok(NoteTypeValue::N16th),
            Some("eighth") | Some("8th") => Ok(NoteTypeValue::Eighth),
            Some("quarter") => Ok(NoteTypeValue::Quarter),
            Some("half") => Ok(NoteTypeValue::Half),
            Some("whole") => Ok(NoteTypeValue::Whole),
            Some("breve") => Ok(NoteTypeValue::Breve),
            Some("long") => Ok(NoteTypeValue::Long),
            Some("maxima") => Ok(NoteTypeValue::Maxima),
            _ => Err(ConvertError::type_mismatch("note-type-value", sexpr)),
        }
    }
}

// ============================================================================
// NoteType
// ============================================================================

impl ToSexpr for NoteType {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("note-type")
            .kwarg("value", &self.value)
            .kwarg_opt("size", &self.size)
            .build()
    }
}

impl FromSexpr for NoteType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("note-type list", sexpr))?;

        expect_head(list, "note-type")?;

        Ok(NoteType {
            value: require_kwarg(list, "value")?,
            size: optional_kwarg(list, "size")?,
        })
    }
}

// ============================================================================
// Dot
// ============================================================================

impl ToSexpr for Dot {
    fn to_sexpr(&self) -> Sexpr {
        // Check if position has any content
        let pos_has_content = self.position.default_x.is_some()
            || self.position.default_y.is_some()
            || self.position.relative_x.is_some()
            || self.position.relative_y.is_some();

        let mut builder = ListBuilder::new("dot").kwarg_opt("placement", &self.placement);

        // Only add position if it has content
        if pos_has_content {
            builder = builder.kwarg_raw("position", self.position.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for Dot {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("dot list", sexpr))?;

        expect_head(list, "dot")?;

        // Parse position (defaults to empty if not present)
        let position = match find_kwarg(list, "position") {
            Some(pos_sexpr) => Position::from_sexpr(pos_sexpr)?,
            None => Position::default(),
        };

        Ok(Dot {
            placement: optional_kwarg(list, "placement")?,
            position,
        })
    }
}

// ============================================================================
// TimeModification
// ============================================================================

impl ToSexpr for TimeModification {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("time-modification")
            .kwarg("actual-notes", &self.actual_notes)
            .kwarg("normal-notes", &self.normal_notes)
            .kwarg_opt("normal-type", &self.normal_type);

        // Only include normal-dots if non-zero
        if self.normal_dots > 0 {
            builder = builder.kwarg("normal-dots", &self.normal_dots);
        }

        builder.build()
    }
}

impl FromSexpr for TimeModification {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time-modification list", sexpr))?;

        expect_head(list, "time-modification")?;

        Ok(TimeModification {
            actual_notes: require_kwarg(list, "actual-notes")?,
            normal_notes: require_kwarg(list, "normal-notes")?,
            normal_type: optional_kwarg(list, "normal-type")?,
            normal_dots: optional_kwarg(list, "normal-dots")?.unwrap_or(0),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{AboveBelow, SymbolSize};
    use crate::sexpr::{parse, print_sexpr};

    // === NoteTypeValue Tests ===

    #[test]
    fn test_notetypevalue_to_sexpr_short_notes() {
        assert_eq!(NoteTypeValue::N1024th.to_sexpr(), Sexpr::symbol("1024th"));
        assert_eq!(NoteTypeValue::N512th.to_sexpr(), Sexpr::symbol("512th"));
        assert_eq!(NoteTypeValue::N256th.to_sexpr(), Sexpr::symbol("256th"));
        assert_eq!(NoteTypeValue::N128th.to_sexpr(), Sexpr::symbol("128th"));
        assert_eq!(NoteTypeValue::N64th.to_sexpr(), Sexpr::symbol("64th"));
        assert_eq!(NoteTypeValue::N32nd.to_sexpr(), Sexpr::symbol("32nd"));
        assert_eq!(NoteTypeValue::N16th.to_sexpr(), Sexpr::symbol("16th"));
    }

    #[test]
    fn test_notetypevalue_to_sexpr_common_notes() {
        assert_eq!(NoteTypeValue::Eighth.to_sexpr(), Sexpr::symbol("eighth"));
        assert_eq!(NoteTypeValue::Quarter.to_sexpr(), Sexpr::symbol("quarter"));
        assert_eq!(NoteTypeValue::Half.to_sexpr(), Sexpr::symbol("half"));
        assert_eq!(NoteTypeValue::Whole.to_sexpr(), Sexpr::symbol("whole"));
    }

    #[test]
    fn test_notetypevalue_to_sexpr_long_notes() {
        assert_eq!(NoteTypeValue::Breve.to_sexpr(), Sexpr::symbol("breve"));
        assert_eq!(NoteTypeValue::Long.to_sexpr(), Sexpr::symbol("long"));
        assert_eq!(NoteTypeValue::Maxima.to_sexpr(), Sexpr::symbol("maxima"));
    }

    #[test]
    fn test_notetypevalue_from_sexpr() {
        assert_eq!(
            NoteTypeValue::from_sexpr(&Sexpr::symbol("1024th")).unwrap(),
            NoteTypeValue::N1024th
        );
        assert_eq!(
            NoteTypeValue::from_sexpr(&Sexpr::symbol("quarter")).unwrap(),
            NoteTypeValue::Quarter
        );
        assert_eq!(
            NoteTypeValue::from_sexpr(&Sexpr::symbol("eighth")).unwrap(),
            NoteTypeValue::Eighth
        );
        // Also accept "8th" for eighth
        assert_eq!(
            NoteTypeValue::from_sexpr(&Sexpr::symbol("8th")).unwrap(),
            NoteTypeValue::Eighth
        );
    }

    #[test]
    fn test_notetypevalue_from_sexpr_invalid() {
        assert!(NoteTypeValue::from_sexpr(&Sexpr::symbol("unknown")).is_err());
        assert!(NoteTypeValue::from_sexpr(&Sexpr::Integer(4)).is_err());
    }

    #[test]
    fn test_notetypevalue_round_trip() {
        for value in [
            NoteTypeValue::N1024th,
            NoteTypeValue::N512th,
            NoteTypeValue::N256th,
            NoteTypeValue::N128th,
            NoteTypeValue::N64th,
            NoteTypeValue::N32nd,
            NoteTypeValue::N16th,
            NoteTypeValue::Eighth,
            NoteTypeValue::Quarter,
            NoteTypeValue::Half,
            NoteTypeValue::Whole,
            NoteTypeValue::Breve,
            NoteTypeValue::Long,
            NoteTypeValue::Maxima,
        ] {
            let sexpr = value.to_sexpr();
            let parsed = NoteTypeValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(value, parsed);
        }
    }

    // === NoteType Tests ===

    #[test]
    fn test_notetype_to_sexpr_simple() {
        let note_type = NoteType {
            value: NoteTypeValue::Quarter,
            size: None,
        };
        let sexpr = note_type.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(note-type :value quarter)");
    }

    #[test]
    fn test_notetype_to_sexpr_with_size() {
        let note_type = NoteType {
            value: NoteTypeValue::Eighth,
            size: Some(SymbolSize::Cue),
        };
        let sexpr = note_type.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(note-type :value eighth :size cue)");
    }

    #[test]
    fn test_notetype_from_sexpr_simple() {
        let sexpr = parse("(note-type :value quarter)").unwrap();
        let note_type = NoteType::from_sexpr(&sexpr).unwrap();
        assert_eq!(note_type.value, NoteTypeValue::Quarter);
        assert!(note_type.size.is_none());
    }

    #[test]
    fn test_notetype_from_sexpr_with_size() {
        let sexpr = parse("(note-type :value eighth :size cue)").unwrap();
        let note_type = NoteType::from_sexpr(&sexpr).unwrap();
        assert_eq!(note_type.value, NoteTypeValue::Eighth);
        assert_eq!(note_type.size, Some(SymbolSize::Cue));
    }

    #[test]
    fn test_notetype_from_sexpr_missing_value() {
        let sexpr = parse("(note-type :size cue)").unwrap();
        assert!(NoteType::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_notetype_round_trip() {
        let original = NoteType {
            value: NoteTypeValue::Half,
            size: Some(SymbolSize::GraceCue),
        };
        let sexpr = original.to_sexpr();
        let parsed = NoteType::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === Dot Tests ===

    #[test]
    fn test_dot_to_sexpr_default() {
        let dot = Dot::default();
        let sexpr = dot.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(dot)");
    }

    #[test]
    fn test_dot_to_sexpr_with_placement() {
        let dot = Dot {
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };
        let sexpr = dot.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(dot :placement above)");
    }

    #[test]
    fn test_dot_to_sexpr_with_position() {
        let dot = Dot {
            placement: None,
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: None,
                relative_y: None,
            },
        };
        let sexpr = dot.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":position"));
        assert!(text.contains(":default-x 5.0"));
        assert!(text.contains(":default-y 10.0"));
    }

    #[test]
    fn test_dot_from_sexpr_default() {
        let sexpr = parse("(dot)").unwrap();
        let dot = Dot::from_sexpr(&sexpr).unwrap();
        assert!(dot.placement.is_none());
        assert_eq!(dot.position, Position::default());
    }

    #[test]
    fn test_dot_from_sexpr_with_placement() {
        let sexpr = parse("(dot :placement below)").unwrap();
        let dot = Dot::from_sexpr(&sexpr).unwrap();
        assert_eq!(dot.placement, Some(AboveBelow::Below));
    }

    #[test]
    fn test_dot_round_trip_default() {
        let original = Dot::default();
        let sexpr = original.to_sexpr();
        let parsed = Dot::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_dot_round_trip_with_placement() {
        let original = Dot {
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };
        let sexpr = original.to_sexpr();
        let parsed = Dot::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_dot_round_trip_with_position() {
        let original = Dot {
            placement: Some(AboveBelow::Below),
            position: Position {
                default_x: Some(3.0),
                default_y: None,
                relative_x: None,
                relative_y: Some(2.0),
            },
        };
        let sexpr = original.to_sexpr();
        let parsed = Dot::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === TimeModification Tests ===

    #[test]
    fn test_timemodification_to_sexpr_triplet() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let sexpr = tm.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(time-modification :actual-notes 3 :normal-notes 2)");
    }

    #[test]
    fn test_timemodification_to_sexpr_with_type() {
        let tm = TimeModification {
            actual_notes: 5,
            normal_notes: 4,
            normal_type: Some(NoteTypeValue::Eighth),
            normal_dots: 0,
        };
        let sexpr = tm.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(
            text,
            "(time-modification :actual-notes 5 :normal-notes 4 :normal-type eighth)"
        );
    }

    #[test]
    fn test_timemodification_to_sexpr_with_dots() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: Some(NoteTypeValue::Quarter),
            normal_dots: 1,
        };
        let sexpr = tm.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":normal-dots 1"));
    }

    #[test]
    fn test_timemodification_from_sexpr_triplet() {
        let sexpr = parse("(time-modification :actual-notes 3 :normal-notes 2)").unwrap();
        let tm = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(tm.actual_notes, 3);
        assert_eq!(tm.normal_notes, 2);
        assert!(tm.normal_type.is_none());
        assert_eq!(tm.normal_dots, 0);
    }

    #[test]
    fn test_timemodification_from_sexpr_with_type() {
        let sexpr =
            parse("(time-modification :actual-notes 5 :normal-notes 4 :normal-type eighth)")
                .unwrap();
        let tm = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(tm.actual_notes, 5);
        assert_eq!(tm.normal_notes, 4);
        assert_eq!(tm.normal_type, Some(NoteTypeValue::Eighth));
    }

    #[test]
    fn test_timemodification_from_sexpr_missing_actual() {
        let sexpr = parse("(time-modification :normal-notes 2)").unwrap();
        assert!(TimeModification::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_timemodification_from_sexpr_missing_normal() {
        let sexpr = parse("(time-modification :actual-notes 3)").unwrap();
        assert!(TimeModification::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_timemodification_round_trip_triplet() {
        let original = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let sexpr = original.to_sexpr();
        let parsed = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_timemodification_round_trip_with_type() {
        let original = TimeModification {
            actual_notes: 7,
            normal_notes: 4,
            normal_type: Some(NoteTypeValue::N16th),
            normal_dots: 0,
        };
        let sexpr = original.to_sexpr();
        let parsed = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_timemodification_round_trip_with_dots() {
        let original = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: Some(NoteTypeValue::Eighth),
            normal_dots: 2,
        };
        let sexpr = original.to_sexpr();
        let parsed = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_timemodification_duplet() {
        let tm = TimeModification {
            actual_notes: 2,
            normal_notes: 3,
            normal_type: None,
            normal_dots: 0,
        };
        let sexpr = tm.to_sexpr();
        let parsed = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.actual_notes, 2);
        assert_eq!(parsed.normal_notes, 3);
    }

    #[test]
    fn test_timemodification_quintuplet() {
        let tm = TimeModification {
            actual_notes: 5,
            normal_notes: 4,
            normal_type: Some(NoteTypeValue::Eighth),
            normal_dots: 0,
        };
        let sexpr = tm.to_sexpr();
        let parsed = TimeModification::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.actual_notes, 5);
        assert_eq!(parsed.normal_notes, 4);
        assert_eq!(parsed.normal_type, Some(NoteTypeValue::Eighth));
    }
}
