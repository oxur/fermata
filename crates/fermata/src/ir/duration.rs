//! Duration and rhythm types.

use super::common::{AboveBelow, Position, SymbolSize};

/// Note type (notated duration symbol).
#[derive(Debug, Clone, PartialEq)]
pub struct NoteType {
    /// The note type value
    pub value: NoteTypeValue,
    /// Optional size modifier
    pub size: Option<SymbolSize>,
}

/// Enumeration of note type values (duration symbols).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteTypeValue {
    /// 1024th note
    N1024th,
    /// 512th note
    N512th,
    /// 256th note
    N256th,
    /// 128th note
    N128th,
    /// 64th note
    N64th,
    /// 32nd note
    N32nd,
    /// 16th note
    N16th,
    /// Eighth note
    Eighth,
    /// Quarter note
    Quarter,
    /// Half note
    Half,
    /// Whole note
    Whole,
    /// Breve (double whole note)
    Breve,
    /// Long (quadruple whole note)
    Long,
    /// Maxima (octuple whole note)
    Maxima,
}

/// Augmentation dot.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dot {
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Time modification for tuplets.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeModification {
    /// Actual number of notes in the tuplet
    pub actual_notes: u32,
    /// Normal number of notes the tuplet replaces
    pub normal_notes: u32,
    /// Note type of the normal notes (if different from actual)
    pub normal_type: Option<NoteTypeValue>,
    /// Number of dots on normal notes
    pub normal_dots: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === NoteTypeValue Tests ===

    #[test]
    fn test_notetypevalue_short_notes() {
        assert_eq!(NoteTypeValue::N1024th, NoteTypeValue::N1024th);
        assert_eq!(NoteTypeValue::N512th, NoteTypeValue::N512th);
        assert_eq!(NoteTypeValue::N256th, NoteTypeValue::N256th);
        assert_eq!(NoteTypeValue::N128th, NoteTypeValue::N128th);
        assert_eq!(NoteTypeValue::N64th, NoteTypeValue::N64th);
        assert_eq!(NoteTypeValue::N32nd, NoteTypeValue::N32nd);
        assert_eq!(NoteTypeValue::N16th, NoteTypeValue::N16th);
    }

    #[test]
    fn test_notetypevalue_common_notes() {
        assert_eq!(NoteTypeValue::Eighth, NoteTypeValue::Eighth);
        assert_eq!(NoteTypeValue::Quarter, NoteTypeValue::Quarter);
        assert_eq!(NoteTypeValue::Half, NoteTypeValue::Half);
        assert_eq!(NoteTypeValue::Whole, NoteTypeValue::Whole);
    }

    #[test]
    fn test_notetypevalue_long_notes() {
        assert_eq!(NoteTypeValue::Breve, NoteTypeValue::Breve);
        assert_eq!(NoteTypeValue::Long, NoteTypeValue::Long);
        assert_eq!(NoteTypeValue::Maxima, NoteTypeValue::Maxima);
    }

    #[test]
    fn test_notetypevalue_inequality() {
        assert_ne!(NoteTypeValue::Quarter, NoteTypeValue::Eighth);
        assert_ne!(NoteTypeValue::Half, NoteTypeValue::Whole);
    }

    #[test]
    fn test_notetypevalue_clone() {
        let note_type = NoteTypeValue::Quarter;
        let cloned = note_type.clone();
        assert_eq!(note_type, cloned);
    }

    #[test]
    fn test_notetypevalue_copy() {
        let note_type = NoteTypeValue::Eighth;
        let copied = note_type;
        assert_eq!(note_type, copied);
    }

    #[test]
    fn test_notetypevalue_debug() {
        assert_eq!(format!("{:?}", NoteTypeValue::Quarter), "Quarter");
        assert_eq!(format!("{:?}", NoteTypeValue::Eighth), "Eighth");
        assert_eq!(format!("{:?}", NoteTypeValue::Half), "Half");
        assert_eq!(format!("{:?}", NoteTypeValue::Whole), "Whole");
        assert_eq!(format!("{:?}", NoteTypeValue::N16th), "N16th");
    }

    // === NoteType Tests ===

    #[test]
    fn test_notetype_quarter() {
        let note_type = NoteType {
            value: NoteTypeValue::Quarter,
            size: None,
        };
        assert_eq!(note_type.value, NoteTypeValue::Quarter);
        assert!(note_type.size.is_none());
    }

    #[test]
    fn test_notetype_with_cue_size() {
        let note_type = NoteType {
            value: NoteTypeValue::Eighth,
            size: Some(SymbolSize::Cue),
        };
        assert_eq!(note_type.value, NoteTypeValue::Eighth);
        assert_eq!(note_type.size, Some(SymbolSize::Cue));
    }

    #[test]
    fn test_notetype_with_grace_cue_size() {
        let note_type = NoteType {
            value: NoteTypeValue::N16th,
            size: Some(SymbolSize::GraceCue),
        };
        assert_eq!(note_type.size, Some(SymbolSize::GraceCue));
    }

    #[test]
    fn test_notetype_with_full_size() {
        let note_type = NoteType {
            value: NoteTypeValue::Half,
            size: Some(SymbolSize::Full),
        };
        assert_eq!(note_type.size, Some(SymbolSize::Full));
    }

    #[test]
    fn test_notetype_with_large_size() {
        let note_type = NoteType {
            value: NoteTypeValue::Whole,
            size: Some(SymbolSize::Large),
        };
        assert_eq!(note_type.size, Some(SymbolSize::Large));
    }

    #[test]
    fn test_notetype_clone() {
        let note_type = NoteType {
            value: NoteTypeValue::Breve,
            size: Some(SymbolSize::Full),
        };
        let cloned = note_type.clone();
        assert_eq!(note_type, cloned);
    }

    #[test]
    fn test_notetype_equality() {
        let note_type1 = NoteType {
            value: NoteTypeValue::Quarter,
            size: None,
        };
        let note_type2 = NoteType {
            value: NoteTypeValue::Quarter,
            size: None,
        };
        assert_eq!(note_type1, note_type2);
    }

    #[test]
    fn test_notetype_inequality_value() {
        let note_type1 = NoteType {
            value: NoteTypeValue::Quarter,
            size: None,
        };
        let note_type2 = NoteType {
            value: NoteTypeValue::Eighth,
            size: None,
        };
        assert_ne!(note_type1, note_type2);
    }

    #[test]
    fn test_notetype_inequality_size() {
        let note_type1 = NoteType {
            value: NoteTypeValue::Quarter,
            size: None,
        };
        let note_type2 = NoteType {
            value: NoteTypeValue::Quarter,
            size: Some(SymbolSize::Cue),
        };
        assert_ne!(note_type1, note_type2);
    }

    // === Dot Tests ===

    #[test]
    fn test_dot_default() {
        let dot = Dot::default();
        assert!(dot.placement.is_none());
        assert_eq!(dot.position, Position::default());
    }

    #[test]
    fn test_dot_with_placement_above() {
        let dot = Dot {
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };
        assert_eq!(dot.placement, Some(AboveBelow::Above));
    }

    #[test]
    fn test_dot_with_placement_below() {
        let dot = Dot {
            placement: Some(AboveBelow::Below),
            position: Position::default(),
        };
        assert_eq!(dot.placement, Some(AboveBelow::Below));
    }

    #[test]
    fn test_dot_with_position() {
        let dot = Dot {
            placement: None,
            position: Position {
                default_x: Some(5.0),
                default_y: Some(10.0),
                relative_x: None,
                relative_y: None,
            },
        };
        assert_eq!(dot.position.default_x, Some(5.0));
        assert_eq!(dot.position.default_y, Some(10.0));
    }

    #[test]
    fn test_dot_clone() {
        let dot = Dot {
            placement: Some(AboveBelow::Above),
            position: Position {
                default_x: Some(3.0),
                ..Default::default()
            },
        };
        let cloned = dot.clone();
        assert_eq!(dot, cloned);
    }

    #[test]
    fn test_dot_equality() {
        let dot1 = Dot {
            placement: Some(AboveBelow::Below),
            position: Position::default(),
        };
        let dot2 = Dot {
            placement: Some(AboveBelow::Below),
            position: Position::default(),
        };
        assert_eq!(dot1, dot2);
    }

    #[test]
    fn test_dot_debug() {
        let dot = Dot::default();
        let debug_str = format!("{:?}", dot);
        assert!(debug_str.contains("Dot"));
    }

    // === TimeModification Tests ===

    #[test]
    fn test_timemodification_triplet() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        assert_eq!(tm.actual_notes, 3);
        assert_eq!(tm.normal_notes, 2);
        assert!(tm.normal_type.is_none());
        assert_eq!(tm.normal_dots, 0);
    }

    #[test]
    fn test_timemodification_duplet() {
        let tm = TimeModification {
            actual_notes: 2,
            normal_notes: 3,
            normal_type: None,
            normal_dots: 0,
        };
        assert_eq!(tm.actual_notes, 2);
        assert_eq!(tm.normal_notes, 3);
    }

    #[test]
    fn test_timemodification_quintuplet() {
        let tm = TimeModification {
            actual_notes: 5,
            normal_notes: 4,
            normal_type: Some(NoteTypeValue::Eighth),
            normal_dots: 0,
        };
        assert_eq!(tm.actual_notes, 5);
        assert_eq!(tm.normal_notes, 4);
        assert_eq!(tm.normal_type, Some(NoteTypeValue::Eighth));
    }

    #[test]
    fn test_timemodification_septuplet() {
        let tm = TimeModification {
            actual_notes: 7,
            normal_notes: 4,
            normal_type: Some(NoteTypeValue::N16th),
            normal_dots: 0,
        };
        assert_eq!(tm.actual_notes, 7);
        assert_eq!(tm.normal_notes, 4);
    }

    #[test]
    fn test_timemodification_with_dotted_normal() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: Some(NoteTypeValue::Quarter),
            normal_dots: 1,
        };
        assert_eq!(tm.normal_dots, 1);
    }

    #[test]
    fn test_timemodification_with_double_dotted() {
        let tm = TimeModification {
            actual_notes: 4,
            normal_notes: 3,
            normal_type: Some(NoteTypeValue::Eighth),
            normal_dots: 2,
        };
        assert_eq!(tm.normal_dots, 2);
    }

    #[test]
    fn test_timemodification_clone() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: Some(NoteTypeValue::Eighth),
            normal_dots: 1,
        };
        let cloned = tm.clone();
        assert_eq!(tm, cloned);
    }

    #[test]
    fn test_timemodification_equality() {
        let tm1 = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let tm2 = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        assert_eq!(tm1, tm2);
    }

    #[test]
    fn test_timemodification_inequality_actual() {
        let tm1 = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let tm2 = TimeModification {
            actual_notes: 5,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        assert_ne!(tm1, tm2);
    }

    #[test]
    fn test_timemodification_inequality_normal() {
        let tm1 = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let tm2 = TimeModification {
            actual_notes: 3,
            normal_notes: 4,
            normal_type: None,
            normal_dots: 0,
        };
        assert_ne!(tm1, tm2);
    }

    #[test]
    fn test_timemodification_debug() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        let debug_str = format!("{:?}", tm);
        assert!(debug_str.contains("TimeModification"));
        assert!(debug_str.contains("actual_notes"));
        assert!(debug_str.contains("normal_notes"));
    }
}
