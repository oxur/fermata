//! Pitch representation types.

use super::common::{Octave, Semitones};

/// A musical pitch with step, optional alteration, and octave.
#[derive(Debug, Clone, PartialEq)]
pub struct Pitch {
    /// The pitch step (A-G)
    pub step: Step,
    /// Pitch alteration in semitones (negative for flats, positive for sharps)
    pub alter: Option<Semitones>,
    /// Octave number (4 is the octave starting at middle C)
    pub octave: Octave,
}

/// The seven natural pitch steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// A
    A,
    /// B
    B,
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// F
    F,
    /// G
    G,
}

/// Unpitched note (percussion) with optional display position.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Unpitched {
    /// Display step for the unpitched note
    pub display_step: Option<Step>,
    /// Display octave for the unpitched note
    pub display_octave: Option<Octave>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Step Tests ===

    #[test]
    fn test_step_all_variants() {
        assert_eq!(Step::A, Step::A);
        assert_eq!(Step::B, Step::B);
        assert_eq!(Step::C, Step::C);
        assert_eq!(Step::D, Step::D);
        assert_eq!(Step::E, Step::E);
        assert_eq!(Step::F, Step::F);
        assert_eq!(Step::G, Step::G);
    }

    #[test]
    fn test_step_inequality() {
        assert_ne!(Step::A, Step::B);
        assert_ne!(Step::C, Step::D);
        assert_ne!(Step::F, Step::G);
    }

    #[test]
    fn test_step_clone() {
        let step = Step::C;
        let cloned = step.clone();
        assert_eq!(step, cloned);
    }

    #[test]
    fn test_step_copy() {
        let step = Step::E;
        let copied = step;
        assert_eq!(step, copied);
    }

    #[test]
    fn test_step_debug() {
        assert_eq!(format!("{:?}", Step::A), "A");
        assert_eq!(format!("{:?}", Step::B), "B");
        assert_eq!(format!("{:?}", Step::C), "C");
        assert_eq!(format!("{:?}", Step::D), "D");
        assert_eq!(format!("{:?}", Step::E), "E");
        assert_eq!(format!("{:?}", Step::F), "F");
        assert_eq!(format!("{:?}", Step::G), "G");
    }

    // === Pitch Tests ===

    #[test]
    fn test_pitch_middle_c() {
        let pitch = Pitch {
            step: Step::C,
            alter: None,
            octave: 4,
        };
        assert_eq!(pitch.step, Step::C);
        assert!(pitch.alter.is_none());
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_pitch_with_sharp() {
        let pitch = Pitch {
            step: Step::F,
            alter: Some(1.0),
            octave: 4,
        };
        assert_eq!(pitch.step, Step::F);
        assert_eq!(pitch.alter, Some(1.0));
        assert_eq!(pitch.octave, 4);
    }

    #[test]
    fn test_pitch_with_flat() {
        let pitch = Pitch {
            step: Step::B,
            alter: Some(-1.0),
            octave: 3,
        };
        assert_eq!(pitch.step, Step::B);
        assert_eq!(pitch.alter, Some(-1.0));
        assert_eq!(pitch.octave, 3);
    }

    #[test]
    fn test_pitch_with_double_sharp() {
        let pitch = Pitch {
            step: Step::G,
            alter: Some(2.0),
            octave: 5,
        };
        assert_eq!(pitch.alter, Some(2.0));
    }

    #[test]
    fn test_pitch_with_double_flat() {
        let pitch = Pitch {
            step: Step::A,
            alter: Some(-2.0),
            octave: 2,
        };
        assert_eq!(pitch.alter, Some(-2.0));
    }

    #[test]
    fn test_pitch_with_quarter_tone() {
        let pitch = Pitch {
            step: Step::D,
            alter: Some(0.5),
            octave: 4,
        };
        assert_eq!(pitch.alter, Some(0.5));
    }

    #[test]
    fn test_pitch_clone() {
        let pitch = Pitch {
            step: Step::E,
            alter: Some(-0.5),
            octave: 5,
        };
        let cloned = pitch.clone();
        assert_eq!(pitch, cloned);
    }

    #[test]
    fn test_pitch_equality() {
        let pitch1 = Pitch {
            step: Step::A,
            alter: None,
            octave: 4,
        };
        let pitch2 = Pitch {
            step: Step::A,
            alter: None,
            octave: 4,
        };
        assert_eq!(pitch1, pitch2);
    }

    #[test]
    fn test_pitch_inequality_step() {
        let pitch1 = Pitch {
            step: Step::A,
            alter: None,
            octave: 4,
        };
        let pitch2 = Pitch {
            step: Step::B,
            alter: None,
            octave: 4,
        };
        assert_ne!(pitch1, pitch2);
    }

    #[test]
    fn test_pitch_inequality_octave() {
        let pitch1 = Pitch {
            step: Step::C,
            alter: None,
            octave: 4,
        };
        let pitch2 = Pitch {
            step: Step::C,
            alter: None,
            octave: 5,
        };
        assert_ne!(pitch1, pitch2);
    }

    #[test]
    fn test_pitch_inequality_alter() {
        let pitch1 = Pitch {
            step: Step::F,
            alter: Some(1.0),
            octave: 4,
        };
        let pitch2 = Pitch {
            step: Step::F,
            alter: None,
            octave: 4,
        };
        assert_ne!(pitch1, pitch2);
    }

    #[test]
    fn test_pitch_debug() {
        let pitch = Pitch {
            step: Step::C,
            alter: Some(1.0),
            octave: 4,
        };
        let debug_str = format!("{:?}", pitch);
        assert!(debug_str.contains("Pitch"));
        assert!(debug_str.contains("step"));
        assert!(debug_str.contains("alter"));
        assert!(debug_str.contains("octave"));
    }

    #[test]
    fn test_pitch_extreme_octaves() {
        let low_pitch = Pitch {
            step: Step::A,
            alter: None,
            octave: 0,
        };
        assert_eq!(low_pitch.octave, 0);

        let high_pitch = Pitch {
            step: Step::C,
            alter: None,
            octave: 9,
        };
        assert_eq!(high_pitch.octave, 9);
    }

    // === Unpitched Tests ===

    #[test]
    fn test_unpitched_default() {
        let unpitched = Unpitched::default();
        assert!(unpitched.display_step.is_none());
        assert!(unpitched.display_octave.is_none());
    }

    #[test]
    fn test_unpitched_with_display_position() {
        let unpitched = Unpitched {
            display_step: Some(Step::E),
            display_octave: Some(4),
        };
        assert_eq!(unpitched.display_step, Some(Step::E));
        assert_eq!(unpitched.display_octave, Some(4));
    }

    #[test]
    fn test_unpitched_with_step_only() {
        let unpitched = Unpitched {
            display_step: Some(Step::G),
            display_octave: None,
        };
        assert_eq!(unpitched.display_step, Some(Step::G));
        assert!(unpitched.display_octave.is_none());
    }

    #[test]
    fn test_unpitched_with_octave_only() {
        let unpitched = Unpitched {
            display_step: None,
            display_octave: Some(5),
        };
        assert!(unpitched.display_step.is_none());
        assert_eq!(unpitched.display_octave, Some(5));
    }

    #[test]
    fn test_unpitched_clone() {
        let unpitched = Unpitched {
            display_step: Some(Step::F),
            display_octave: Some(3),
        };
        let cloned = unpitched.clone();
        assert_eq!(unpitched, cloned);
    }

    #[test]
    fn test_unpitched_equality() {
        let unpitched1 = Unpitched {
            display_step: Some(Step::A),
            display_octave: Some(4),
        };
        let unpitched2 = Unpitched {
            display_step: Some(Step::A),
            display_octave: Some(4),
        };
        assert_eq!(unpitched1, unpitched2);
    }

    #[test]
    fn test_unpitched_inequality() {
        let unpitched1 = Unpitched {
            display_step: Some(Step::A),
            display_octave: Some(4),
        };
        let unpitched2 = Unpitched {
            display_step: Some(Step::B),
            display_octave: Some(4),
        };
        assert_ne!(unpitched1, unpitched2);
    }

    #[test]
    fn test_unpitched_debug() {
        let unpitched = Unpitched::default();
        let debug_str = format!("{:?}", unpitched);
        assert!(debug_str.contains("Unpitched"));
    }
}
