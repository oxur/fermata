//! Beam, stem, and notehead types.

use super::common::{BeamLevel, Color, Font, Tenths, YesNo};

/// Beam element for note grouping.
#[derive(Debug, Clone, PartialEq)]
pub struct Beam {
    /// The beam value (connection type)
    pub value: BeamValue,
    /// Beam level (1-8)
    pub number: BeamLevel,
    /// Feathered beam type
    pub fan: Option<Fan>,
    /// Beam color
    pub color: Option<Color>,
}

/// Beam value (connection type).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamValue {
    /// Begin a beam group
    Begin,
    /// Continue a beam group
    Continue,
    /// End a beam group
    End,
    /// Forward hook (partial beam to the right)
    ForwardHook,
    /// Backward hook (partial beam to the left)
    BackwardHook,
}

/// Beam fan (for feathered beams).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fan {
    /// Accelerando (beams spread out)
    Accel,
    /// Ritardando (beams come together)
    Rit,
    /// No fanning
    None,
}

/// Stem direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Stem {
    /// The stem value (direction)
    pub value: StemValue,
    /// Default Y position
    pub default_y: Option<Tenths>,
    /// Stem color
    pub color: Option<Color>,
}

/// Stem direction values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StemValue {
    /// Stem points down
    Down,
    /// Stem points up
    Up,
    /// Double stem
    Double,
    /// No stem
    None,
}

/// Notehead shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Notehead {
    /// The notehead value (shape)
    pub value: NoteheadValue,
    /// Whether the notehead is filled
    pub filled: Option<YesNo>,
    /// Whether the notehead has parentheses
    pub parentheses: Option<YesNo>,
    /// Font attributes
    pub font: Font,
    /// Notehead color
    pub color: Option<Color>,
}

/// Notehead shape values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteheadValue {
    /// Slash notehead
    Slash,
    /// Triangle notehead
    Triangle,
    /// Diamond notehead
    Diamond,
    /// Square notehead
    Square,
    /// Cross notehead
    Cross,
    /// X notehead
    X,
    /// Circle-X notehead
    CircleX,
    /// Inverted triangle notehead
    InvertedTriangle,
    /// Arrow down notehead
    ArrowDown,
    /// Arrow up notehead
    ArrowUp,
    /// Circled notehead
    Circled,
    /// Slashed notehead
    Slashed,
    /// Back-slashed notehead
    BackSlashed,
    /// Normal notehead
    Normal,
    /// Cluster notehead
    Cluster,
    /// Circle-dot notehead
    CircleDot,
    /// Left triangle notehead
    LeftTriangle,
    /// Rectangle notehead
    Rectangle,
    /// No notehead
    None,
    /// Do (movable do solfege)
    Do,
    /// Re (movable do solfege)
    Re,
    /// Mi (movable do solfege)
    Mi,
    /// Fa (movable do solfege)
    Fa,
    /// Fa-up (movable do solfege)
    FaUp,
    /// So (movable do solfege)
    So,
    /// La (movable do solfege)
    La,
    /// Ti (movable do solfege)
    Ti,
    /// Other notehead type
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === BeamValue Tests ===

    #[test]
    fn test_beamvalue_all_variants() {
        assert_eq!(BeamValue::Begin, BeamValue::Begin);
        assert_eq!(BeamValue::Continue, BeamValue::Continue);
        assert_eq!(BeamValue::End, BeamValue::End);
        assert_eq!(BeamValue::ForwardHook, BeamValue::ForwardHook);
        assert_eq!(BeamValue::BackwardHook, BeamValue::BackwardHook);
    }

    #[test]
    fn test_beamvalue_inequality() {
        assert_ne!(BeamValue::Begin, BeamValue::End);
        assert_ne!(BeamValue::ForwardHook, BeamValue::BackwardHook);
    }

    #[test]
    fn test_beamvalue_clone() {
        let value = BeamValue::Begin;
        let cloned = value.clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn test_beamvalue_copy() {
        let value = BeamValue::Continue;
        let copied = value;
        assert_eq!(value, copied);
    }

    #[test]
    fn test_beamvalue_debug() {
        assert_eq!(format!("{:?}", BeamValue::Begin), "Begin");
        assert_eq!(format!("{:?}", BeamValue::Continue), "Continue");
        assert_eq!(format!("{:?}", BeamValue::End), "End");
        assert_eq!(format!("{:?}", BeamValue::ForwardHook), "ForwardHook");
        assert_eq!(format!("{:?}", BeamValue::BackwardHook), "BackwardHook");
    }

    // === Fan Tests ===

    #[test]
    fn test_fan_all_variants() {
        assert_eq!(Fan::Accel, Fan::Accel);
        assert_eq!(Fan::Rit, Fan::Rit);
        assert_eq!(Fan::None, Fan::None);
    }

    #[test]
    fn test_fan_inequality() {
        assert_ne!(Fan::Accel, Fan::Rit);
        assert_ne!(Fan::Accel, Fan::None);
    }

    #[test]
    fn test_fan_clone() {
        let fan = Fan::Accel;
        let cloned = fan.clone();
        assert_eq!(fan, cloned);
    }

    // === Beam Tests ===

    #[test]
    fn test_beam_basic() {
        let beam = Beam {
            value: BeamValue::Begin,
            number: 1,
            fan: None,
            color: None,
        };
        assert_eq!(beam.value, BeamValue::Begin);
        assert_eq!(beam.number, 1);
        assert!(beam.fan.is_none());
        assert!(beam.color.is_none());
    }

    #[test]
    fn test_beam_with_fan() {
        let beam = Beam {
            value: BeamValue::Continue,
            number: 1,
            fan: Some(Fan::Accel),
            color: None,
        };
        assert_eq!(beam.fan, Some(Fan::Accel));
    }

    #[test]
    fn test_beam_with_color() {
        let beam = Beam {
            value: BeamValue::End,
            number: 1,
            fan: None,
            color: Some("#0000FF".to_string()),
        };
        assert_eq!(beam.color, Some("#0000FF".to_string()));
    }

    #[test]
    fn test_beam_multiple_levels() {
        let beam_level_1 = Beam {
            value: BeamValue::Begin,
            number: 1,
            fan: None,
            color: None,
        };
        let beam_level_2 = Beam {
            value: BeamValue::Begin,
            number: 2,
            fan: None,
            color: None,
        };
        assert_eq!(beam_level_1.number, 1);
        assert_eq!(beam_level_2.number, 2);
    }

    #[test]
    fn test_beam_forward_hook() {
        let beam = Beam {
            value: BeamValue::ForwardHook,
            number: 2,
            fan: None,
            color: None,
        };
        assert_eq!(beam.value, BeamValue::ForwardHook);
    }

    #[test]
    fn test_beam_backward_hook() {
        let beam = Beam {
            value: BeamValue::BackwardHook,
            number: 3,
            fan: None,
            color: None,
        };
        assert_eq!(beam.value, BeamValue::BackwardHook);
    }

    #[test]
    fn test_beam_clone() {
        let beam = Beam {
            value: BeamValue::Continue,
            number: 1,
            fan: Some(Fan::Rit),
            color: Some("#FF0000".to_string()),
        };
        let cloned = beam.clone();
        assert_eq!(beam, cloned);
    }

    #[test]
    fn test_beam_equality() {
        let beam1 = Beam {
            value: BeamValue::Begin,
            number: 1,
            fan: None,
            color: None,
        };
        let beam2 = Beam {
            value: BeamValue::Begin,
            number: 1,
            fan: None,
            color: None,
        };
        assert_eq!(beam1, beam2);
    }

    #[test]
    fn test_beam_inequality() {
        let beam1 = Beam {
            value: BeamValue::Begin,
            number: 1,
            fan: None,
            color: None,
        };
        let beam2 = Beam {
            value: BeamValue::End,
            number: 1,
            fan: None,
            color: None,
        };
        assert_ne!(beam1, beam2);
    }

    // === StemValue Tests ===

    #[test]
    fn test_stemvalue_all_variants() {
        assert_eq!(StemValue::Down, StemValue::Down);
        assert_eq!(StemValue::Up, StemValue::Up);
        assert_eq!(StemValue::Double, StemValue::Double);
        assert_eq!(StemValue::None, StemValue::None);
    }

    #[test]
    fn test_stemvalue_inequality() {
        assert_ne!(StemValue::Down, StemValue::Up);
        assert_ne!(StemValue::Double, StemValue::None);
    }

    #[test]
    fn test_stemvalue_clone() {
        let value = StemValue::Up;
        let cloned = value.clone();
        assert_eq!(value, cloned);
    }

    // === Stem Tests ===

    #[test]
    fn test_stem_up() {
        let stem = Stem {
            value: StemValue::Up,
            default_y: None,
            color: None,
        };
        assert_eq!(stem.value, StemValue::Up);
    }

    #[test]
    fn test_stem_down() {
        let stem = Stem {
            value: StemValue::Down,
            default_y: Some(-35.0),
            color: None,
        };
        assert_eq!(stem.value, StemValue::Down);
        assert_eq!(stem.default_y, Some(-35.0));
    }

    #[test]
    fn test_stem_with_color() {
        let stem = Stem {
            value: StemValue::Up,
            default_y: Some(35.0),
            color: Some("#000000".to_string()),
        };
        assert_eq!(stem.color, Some("#000000".to_string()));
    }

    #[test]
    fn test_stem_double() {
        let stem = Stem {
            value: StemValue::Double,
            default_y: None,
            color: None,
        };
        assert_eq!(stem.value, StemValue::Double);
    }

    #[test]
    fn test_stem_none() {
        let stem = Stem {
            value: StemValue::None,
            default_y: None,
            color: None,
        };
        assert_eq!(stem.value, StemValue::None);
    }

    #[test]
    fn test_stem_clone() {
        let stem = Stem {
            value: StemValue::Up,
            default_y: Some(40.0),
            color: Some("#333333".to_string()),
        };
        let cloned = stem.clone();
        assert_eq!(stem, cloned);
    }

    // === NoteheadValue Tests ===

    #[test]
    fn test_noteheadvalue_standard_shapes() {
        assert_eq!(NoteheadValue::Normal, NoteheadValue::Normal);
        assert_eq!(NoteheadValue::Diamond, NoteheadValue::Diamond);
        assert_eq!(NoteheadValue::Triangle, NoteheadValue::Triangle);
        assert_eq!(NoteheadValue::Square, NoteheadValue::Square);
        assert_eq!(NoteheadValue::Slash, NoteheadValue::Slash);
    }

    #[test]
    fn test_noteheadvalue_cross_shapes() {
        assert_eq!(NoteheadValue::Cross, NoteheadValue::Cross);
        assert_eq!(NoteheadValue::X, NoteheadValue::X);
        assert_eq!(NoteheadValue::CircleX, NoteheadValue::CircleX);
    }

    #[test]
    fn test_noteheadvalue_arrow_shapes() {
        assert_eq!(NoteheadValue::ArrowDown, NoteheadValue::ArrowDown);
        assert_eq!(NoteheadValue::ArrowUp, NoteheadValue::ArrowUp);
    }

    #[test]
    fn test_noteheadvalue_special_shapes() {
        assert_eq!(
            NoteheadValue::InvertedTriangle,
            NoteheadValue::InvertedTriangle
        );
        assert_eq!(NoteheadValue::Circled, NoteheadValue::Circled);
        assert_eq!(NoteheadValue::Slashed, NoteheadValue::Slashed);
        assert_eq!(NoteheadValue::BackSlashed, NoteheadValue::BackSlashed);
        assert_eq!(NoteheadValue::Cluster, NoteheadValue::Cluster);
        assert_eq!(NoteheadValue::CircleDot, NoteheadValue::CircleDot);
        assert_eq!(NoteheadValue::LeftTriangle, NoteheadValue::LeftTriangle);
        assert_eq!(NoteheadValue::Rectangle, NoteheadValue::Rectangle);
        assert_eq!(NoteheadValue::None, NoteheadValue::None);
    }

    #[test]
    fn test_noteheadvalue_solfege_shapes() {
        assert_eq!(NoteheadValue::Do, NoteheadValue::Do);
        assert_eq!(NoteheadValue::Re, NoteheadValue::Re);
        assert_eq!(NoteheadValue::Mi, NoteheadValue::Mi);
        assert_eq!(NoteheadValue::Fa, NoteheadValue::Fa);
        assert_eq!(NoteheadValue::FaUp, NoteheadValue::FaUp);
        assert_eq!(NoteheadValue::So, NoteheadValue::So);
        assert_eq!(NoteheadValue::La, NoteheadValue::La);
        assert_eq!(NoteheadValue::Ti, NoteheadValue::Ti);
    }

    #[test]
    fn test_noteheadvalue_other() {
        assert_eq!(NoteheadValue::Other, NoteheadValue::Other);
    }

    #[test]
    fn test_noteheadvalue_clone() {
        let value = NoteheadValue::Diamond;
        let cloned = value.clone();
        assert_eq!(value, cloned);
    }

    // === Notehead Tests ===

    #[test]
    fn test_notehead_normal() {
        let notehead = Notehead {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: None,
            font: Font::default(),
            color: None,
        };
        assert_eq!(notehead.value, NoteheadValue::Normal);
    }

    #[test]
    fn test_notehead_diamond_filled() {
        let notehead = Notehead {
            value: NoteheadValue::Diamond,
            filled: Some(YesNo::Yes),
            parentheses: None,
            font: Font::default(),
            color: None,
        };
        assert_eq!(notehead.filled, Some(YesNo::Yes));
    }

    #[test]
    fn test_notehead_with_parentheses() {
        let notehead = Notehead {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: Some(YesNo::Yes),
            font: Font::default(),
            color: None,
        };
        assert_eq!(notehead.parentheses, Some(YesNo::Yes));
    }

    #[test]
    fn test_notehead_with_color() {
        let notehead = Notehead {
            value: NoteheadValue::X,
            filled: None,
            parentheses: None,
            font: Font::default(),
            color: Some("#FF0000".to_string()),
        };
        assert_eq!(notehead.color, Some("#FF0000".to_string()));
    }

    #[test]
    fn test_notehead_clone() {
        let notehead = Notehead {
            value: NoteheadValue::Triangle,
            filled: Some(YesNo::No),
            parentheses: Some(YesNo::Yes),
            font: Font::default(),
            color: Some("#0000FF".to_string()),
        };
        let cloned = notehead.clone();
        assert_eq!(notehead, cloned);
    }

    #[test]
    fn test_notehead_equality() {
        let notehead1 = Notehead {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: None,
            font: Font::default(),
            color: None,
        };
        let notehead2 = Notehead {
            value: NoteheadValue::Normal,
            filled: None,
            parentheses: None,
            font: Font::default(),
            color: None,
        };
        assert_eq!(notehead1, notehead2);
    }
}
