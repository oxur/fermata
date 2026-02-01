//! S-expression conversions for `ir::voice` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for voice-related
//! types that control timing within measures:
//!
//! - [`Backup`] - Move backward in time within a measure
//! - [`Forward`] - Move forward in time within a measure

use crate::ir::common::Editorial;
use crate::ir::voice::{Backup, Forward};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, optional_kwarg, require_kwarg};

// ============================================================================
// Backup
// ============================================================================

impl ToSexpr for Backup {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("backup")
            .kwarg("duration", &self.duration)
            .build()
    }
}

impl FromSexpr for Backup {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("backup list", sexpr))?;

        expect_head(list, "backup")?;

        Ok(Backup {
            duration: require_kwarg(list, "duration")?,
            editorial: Editorial::default(),
        })
    }
}

// ============================================================================
// Forward
// ============================================================================

impl ToSexpr for Forward {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("forward")
            .kwarg("duration", &self.duration)
            .kwarg_opt("voice", &self.voice)
            .kwarg_opt("staff", &self.staff)
            .build()
    }
}

impl FromSexpr for Forward {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("forward list", sexpr))?;

        expect_head(list, "forward")?;

        Ok(Forward {
            duration: require_kwarg(list, "duration")?,
            voice: optional_kwarg(list, "voice")?,
            staff: optional_kwarg(list, "staff")?,
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
    use crate::sexpr::print_sexpr;

    // === Backup Tests ===

    #[test]
    fn test_backup_basic_round_trip() {
        let backup = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };

        let sexpr = backup.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("backup"));
        assert!(text.contains(":duration 4"));

        let parsed = Backup::from_sexpr(&sexpr).unwrap();
        assert_eq!(backup.duration, parsed.duration);
    }

    #[test]
    fn test_backup_large_duration() {
        let backup = Backup {
            duration: 256,
            editorial: Editorial::default(),
        };

        let sexpr = backup.to_sexpr();
        let parsed = Backup::from_sexpr(&sexpr).unwrap();
        assert_eq!(backup.duration, parsed.duration);
    }

    #[test]
    fn test_backup_to_sexpr_format() {
        let backup = Backup {
            duration: 8,
            editorial: Editorial::default(),
        };

        let sexpr = backup.to_sexpr();
        let list = sexpr.as_list().expect("should be a list");
        assert!(list[0].is_symbol("backup"));
    }

    #[test]
    fn test_backup_from_sexpr_missing_duration() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("backup")]);
        let result = Backup::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_backup_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("forward"),
            Sexpr::keyword("duration"),
            Sexpr::Integer(4),
        ]);
        let result = Backup::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // === Forward Tests ===

    #[test]
    fn test_forward_basic_round_trip() {
        let forward = Forward {
            duration: 4,
            voice: None,
            staff: None,
            editorial: Editorial::default(),
        };

        let sexpr = forward.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("forward"));
        assert!(text.contains(":duration 4"));

        let parsed = Forward::from_sexpr(&sexpr).unwrap();
        assert_eq!(forward.duration, parsed.duration);
        assert!(parsed.voice.is_none());
        assert!(parsed.staff.is_none());
    }

    #[test]
    fn test_forward_with_voice() {
        let forward = Forward {
            duration: 2,
            voice: Some("1".to_string()),
            staff: None,
            editorial: Editorial::default(),
        };

        let sexpr = forward.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":voice"));

        let parsed = Forward::from_sexpr(&sexpr).unwrap();
        assert_eq!(forward.voice, parsed.voice);
    }

    #[test]
    fn test_forward_with_staff() {
        let forward = Forward {
            duration: 8,
            voice: None,
            staff: Some(2),
            editorial: Editorial::default(),
        };

        let sexpr = forward.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":staff 2"));

        let parsed = Forward::from_sexpr(&sexpr).unwrap();
        assert_eq!(forward.staff, parsed.staff);
    }

    #[test]
    fn test_forward_with_voice_and_staff() {
        let forward = Forward {
            duration: 16,
            voice: Some("2".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };

        let sexpr = forward.to_sexpr();
        let parsed = Forward::from_sexpr(&sexpr).unwrap();
        assert_eq!(forward.duration, parsed.duration);
        assert_eq!(forward.voice, parsed.voice);
        assert_eq!(forward.staff, parsed.staff);
    }

    #[test]
    fn test_forward_from_sexpr_missing_duration() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("forward"),
            Sexpr::keyword("voice"),
            Sexpr::String("1".to_string()),
        ]);
        let result = Forward::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_forward_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("backup"),
            Sexpr::keyword("duration"),
            Sexpr::Integer(4),
        ]);
        let result = Forward::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_forward_to_sexpr_format() {
        let forward = Forward {
            duration: 4,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };

        let sexpr = forward.to_sexpr();
        let list = sexpr.as_list().expect("should be a list");
        assert!(list[0].is_symbol("forward"));
    }
}
