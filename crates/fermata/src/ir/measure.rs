//! Measure and music data types.

use super::attributes::{Attributes, Barline};
use super::common::{Tenths, YesNo};
use super::direction::Direction;
use super::note::Note;
use super::voice::{Backup, Forward};

/// A measure within a part.
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
    /// Measure number (required)
    pub number: String,
    /// Whether this is an implicit measure (e.g., pickup)
    pub implicit: Option<YesNo>,
    /// Whether this measure is non-controlling (for multi-part scores)
    pub non_controlling: Option<YesNo>,
    /// Measure width in tenths
    pub width: Option<Tenths>,
    /// Music data content
    pub content: Vec<MusicDataElement>,
}

/// Elements that can appear within a measure.
#[derive(Debug, Clone, PartialEq)]
pub enum MusicDataElement {
    /// A note, rest, or chord member
    Note(Box<Note>),
    /// Move backward in time (for multiple voices)
    Backup(Backup),
    /// Move forward in time
    Forward(Forward),
    /// Musical direction (dynamics, tempo, etc.)
    Direction(Box<Direction>),
    /// Measure attributes (key, time, clef, etc.)
    Attributes(Box<Attributes>),
    /// Barline
    Barline(Box<Barline>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{Editorial, Position, YesNo};
    use crate::ir::note::{FullNote, NoteContent, PitchRestUnpitched, Rest};
    use crate::ir::pitch::{Pitch, Step};

    // === Measure Tests ===

    #[test]
    fn test_measure_basic() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        assert_eq!(measure.number, "1");
        assert!(measure.implicit.is_none());
        assert!(measure.non_controlling.is_none());
        assert!(measure.width.is_none());
        assert!(measure.content.is_empty());
    }

    #[test]
    fn test_measure_with_number() {
        let measure = Measure {
            number: "42".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        assert_eq!(measure.number, "42");
    }

    #[test]
    fn test_measure_implicit_pickup() {
        let measure = Measure {
            number: "0".to_string(),
            implicit: Some(YesNo::Yes),
            non_controlling: None,
            width: None,
            content: vec![],
        };
        assert_eq!(measure.implicit, Some(YesNo::Yes));
    }

    #[test]
    fn test_measure_non_controlling() {
        let measure = Measure {
            number: "5".to_string(),
            implicit: None,
            non_controlling: Some(YesNo::Yes),
            width: None,
            content: vec![],
        };
        assert_eq!(measure.non_controlling, Some(YesNo::Yes));
    }

    #[test]
    fn test_measure_with_width() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: Some(200.0),
            content: vec![],
        };
        assert_eq!(measure.width, Some(200.0));
    }

    #[test]
    fn test_measure_clone() {
        let measure = Measure {
            number: "10".to_string(),
            implicit: Some(YesNo::No),
            non_controlling: Some(YesNo::No),
            width: Some(150.5),
            content: vec![],
        };
        let cloned = measure.clone();
        assert_eq!(measure, cloned);
    }

    #[test]
    fn test_measure_equality() {
        let measure1 = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        let measure2 = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        assert_eq!(measure1, measure2);
    }

    #[test]
    fn test_measure_inequality() {
        let measure1 = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        let measure2 = Measure {
            number: "2".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        assert_ne!(measure1, measure2);
    }

    #[test]
    fn test_measure_debug() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![],
        };
        let debug_str = format!("{:?}", measure);
        assert!(debug_str.contains("Measure"));
        assert!(debug_str.contains("number"));
    }

    // === MusicDataElement Tests ===

    #[test]
    fn test_musicdataelement_note() {
        let note = Note {
            position: Position::default(),
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            print_object: None,
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::C,
                        alter: None,
                        octave: 4,
                    }),
                },
                duration: 4,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: None,
            dots: vec![],
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };
        let element = MusicDataElement::Note(Box::new(note));
        if let MusicDataElement::Note(n) = element {
            assert_eq!(n.voice, Some("1".to_string()));
        } else {
            panic!("Expected Note variant");
        }
    }

    #[test]
    fn test_musicdataelement_rest() {
        let note = Note {
            position: Position::default(),
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            print_object: None,
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Rest(Rest::default()),
                },
                duration: 4,
                ties: vec![],
            },
            instrument: vec![],
            voice: Some("1".to_string()),
            r#type: None,
            dots: vec![],
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };
        let element = MusicDataElement::Note(Box::new(note));
        if let MusicDataElement::Note(n) = element {
            if let NoteContent::Regular { full_note, .. } = &n.content {
                assert!(matches!(full_note.content, PitchRestUnpitched::Rest(_)));
            }
        } else {
            panic!("Expected Note variant");
        }
    }

    #[test]
    fn test_musicdataelement_backup() {
        let backup = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };
        let element = MusicDataElement::Backup(backup);
        if let MusicDataElement::Backup(b) = element {
            assert_eq!(b.duration, 4);
        } else {
            panic!("Expected Backup variant");
        }
    }

    #[test]
    fn test_musicdataelement_forward() {
        let forward = Forward {
            duration: 2,
            voice: Some("2".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };
        let element = MusicDataElement::Forward(forward);
        if let MusicDataElement::Forward(f) = element {
            assert_eq!(f.duration, 2);
            assert_eq!(f.voice, Some("2".to_string()));
        } else {
            panic!("Expected Forward variant");
        }
    }

    #[test]
    fn test_musicdataelement_direction() {
        let direction = Direction {
            placement: None,
            directive: None,
            direction_types: vec![],
            offset: None,
            voice: None,
            staff: None,
            sound: None,
        };
        let element = MusicDataElement::Direction(Box::new(direction));
        if let MusicDataElement::Direction(d) = element {
            assert!(d.placement.is_none());
        } else {
            panic!("Expected Direction variant");
        }
    }

    #[test]
    fn test_musicdataelement_attributes() {
        let attributes = Attributes::default();
        let element = MusicDataElement::Attributes(Box::new(attributes));
        if let MusicDataElement::Attributes(a) = element {
            assert!(a.divisions.is_none());
        } else {
            panic!("Expected Attributes variant");
        }
    }

    #[test]
    fn test_musicdataelement_barline() {
        let barline = Barline::default();
        let element = MusicDataElement::Barline(Box::new(barline));
        if let MusicDataElement::Barline(b) = element {
            assert!(b.location.is_none());
        } else {
            panic!("Expected Barline variant");
        }
    }

    #[test]
    fn test_musicdataelement_clone() {
        let backup = Backup {
            duration: 8,
            editorial: Editorial::default(),
        };
        let element = MusicDataElement::Backup(backup);
        let cloned = element.clone();
        assert_eq!(element, cloned);
    }

    // === Measure with content Tests ===

    #[test]
    fn test_measure_with_attributes() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![MusicDataElement::Attributes(Box::new(Attributes {
                divisions: Some(4),
                ..Default::default()
            }))],
        };
        assert_eq!(measure.content.len(), 1);
        if let MusicDataElement::Attributes(attr) = &measure.content[0] {
            assert_eq!(attr.divisions, Some(4));
        } else {
            panic!("Expected Attributes element");
        }
    }

    #[test]
    fn test_measure_with_multiple_elements() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                MusicDataElement::Attributes(Box::new(Attributes::default())),
                MusicDataElement::Note(Box::new(Note {
                    position: Position::default(),
                    dynamics: None,
                    end_dynamics: None,
                    attack: None,
                    release: None,
                    pizzicato: None,
                    print_object: None,
                    content: NoteContent::Regular {
                        full_note: FullNote {
                            chord: false,
                            content: PitchRestUnpitched::Pitch(Pitch {
                                step: Step::C,
                                alter: None,
                                octave: 4,
                            }),
                        },
                        duration: 4,
                        ties: vec![],
                    },
                    instrument: vec![],
                    voice: None,
                    r#type: None,
                    dots: vec![],
                    accidental: None,
                    time_modification: None,
                    stem: None,
                    notehead: None,
                    staff: None,
                    beams: vec![],
                    notations: vec![],
                    lyrics: vec![],
                })),
                MusicDataElement::Barline(Box::new(Barline::default())),
            ],
        };
        assert_eq!(measure.content.len(), 3);
    }

    #[test]
    fn test_measure_with_backup_and_forward() {
        let measure = Measure {
            number: "1".to_string(),
            implicit: None,
            non_controlling: None,
            width: None,
            content: vec![
                MusicDataElement::Backup(Backup {
                    duration: 4,
                    editorial: Editorial::default(),
                }),
                MusicDataElement::Forward(Forward {
                    duration: 2,
                    voice: None,
                    staff: None,
                    editorial: Editorial::default(),
                }),
            ],
        };
        assert_eq!(measure.content.len(), 2);
    }
}
