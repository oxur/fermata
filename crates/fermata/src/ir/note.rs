//! Note, rest, chord, and grace note types.

use super::beam::{Notehead, Stem};
use super::common::{
    AccidentalValue, Divisions, Octave, Percent, Position, PositiveDivisions, StaffNumber,
    StartStop, SymbolSize, Voice, YesNo,
};
use super::duration::{Dot, NoteType, TimeModification};
use super::lyric::Lyric;
use super::notation::Notations;
use super::pitch::{Pitch, Unpitched};

// Re-export Beam for convenience
pub use super::beam::Beam;

// AccidentalValue is defined in common.rs and imported at the top of this file

/// A note element - the fundamental music content type.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    // Position/playback attributes
    /// Position attributes
    pub position: Position,
    /// Attack dynamics (percentage)
    pub dynamics: Option<f64>,
    /// End dynamics (percentage)
    pub end_dynamics: Option<f64>,
    /// Attack offset in divisions
    pub attack: Option<Divisions>,
    /// Release offset in divisions
    pub release: Option<Divisions>,
    /// Pizzicato playback
    pub pizzicato: Option<bool>,
    /// Whether to print the note
    pub print_object: Option<YesNo>,

    // Content variant (regular, grace, or cue)
    /// The note content (regular, grace, or cue)
    pub content: NoteContent,

    // Common children
    /// Instrument references
    pub instrument: Vec<Instrument>,
    /// Voice assignment
    pub voice: Option<Voice>,
    /// Note type (notated duration)
    pub r#type: Option<NoteType>,
    /// Augmentation dots
    pub dots: Vec<Dot>,
    /// Accidental display
    pub accidental: Option<Accidental>,
    /// Time modification (tuplet)
    pub time_modification: Option<TimeModification>,
    /// Stem direction
    pub stem: Option<Stem>,
    /// Notehead shape
    pub notehead: Option<Notehead>,
    /// Staff number
    pub staff: Option<StaffNumber>,
    /// Beams
    pub beams: Vec<Beam>,
    /// Notations
    pub notations: Vec<Notations>,
    /// Lyrics
    pub lyrics: Vec<Lyric>,
}

/// The three content variants for a note.
#[derive(Debug, Clone, PartialEq)]
pub enum NoteContent {
    /// Regular note with duration
    Regular {
        /// Full note content
        full_note: FullNote,
        /// Duration in divisions
        duration: PositiveDivisions,
        /// Ties
        ties: Vec<Tie>,
    },
    /// Grace note (no duration, steals time)
    Grace {
        /// Grace note attributes
        grace: Grace,
        /// Full note content
        full_note: FullNote,
        /// Ties
        ties: Vec<Tie>,
    },
    /// Cue note (for cue-sized notes)
    Cue {
        /// Full note content
        full_note: FullNote,
        /// Duration in divisions
        duration: PositiveDivisions,
    },
}

/// Full note content: chord flag + pitch/rest/unpitched.
#[derive(Debug, Clone, PartialEq)]
pub struct FullNote {
    /// If true, this note is part of a chord with the previous note
    pub chord: bool,
    /// The pitch, rest, or unpitched content
    pub content: PitchRestUnpitched,
}

/// Pitch, rest, or unpitched (mutually exclusive).
#[derive(Debug, Clone, PartialEq)]
pub enum PitchRestUnpitched {
    /// A pitched note
    Pitch(Pitch),
    /// A rest
    Rest(Rest),
    /// An unpitched note (percussion)
    Unpitched(Unpitched),
}

/// A rest.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rest {
    /// If yes, this is a whole-measure rest
    pub measure: Option<YesNo>,
    /// Display position for the rest symbol
    pub display_step: Option<super::pitch::Step>,
    /// Display octave
    pub display_octave: Option<Octave>,
}

/// Grace note attributes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Grace {
    /// Steal time from previous note (percentage)
    pub steal_time_previous: Option<Percent>,
    /// Steal time from following note (percentage)
    pub steal_time_following: Option<Percent>,
    /// Make time (in divisions)
    pub make_time: Option<Divisions>,
    /// Whether to show a slash through the stem
    pub slash: Option<YesNo>,
}

/// Tie (playback, not visual).
#[derive(Debug, Clone, PartialEq)]
pub struct Tie {
    /// Start or stop
    pub r#type: StartStop,
    /// Time-only attribute
    pub time_only: Option<String>,
}

/// Accidental display.
#[derive(Debug, Clone, PartialEq)]
pub struct Accidental {
    /// The accidental value
    pub value: AccidentalValue,
    /// Cautionary accidental
    pub cautionary: Option<YesNo>,
    /// Editorial accidental
    pub editorial: Option<YesNo>,
    /// Parentheses around accidental
    pub parentheses: Option<YesNo>,
    /// Bracket around accidental
    pub bracket: Option<YesNo>,
    /// Symbol size
    pub size: Option<SymbolSize>,
}

// AccidentalValue is defined in common.rs and re-exported above

/// Instrument reference (for playback).
#[derive(Debug, Clone, PartialEq)]
pub struct Instrument {
    /// Instrument ID
    pub id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{AccidentalValue, Position, StartStop, SymbolSize, YesNo};
    use crate::ir::pitch::Step;

    // === FullNote Tests ===

    #[test]
    fn test_fullnote_pitched() {
        let full_note = FullNote {
            chord: false,
            content: PitchRestUnpitched::Pitch(Pitch {
                step: Step::C,
                alter: None,
                octave: 4,
            }),
        };
        assert!(!full_note.chord);
        if let PitchRestUnpitched::Pitch(p) = &full_note.content {
            assert_eq!(p.step, Step::C);
            assert_eq!(p.octave, 4);
        } else {
            panic!("Expected Pitch variant");
        }
    }

    #[test]
    fn test_fullnote_rest() {
        let full_note = FullNote {
            chord: false,
            content: PitchRestUnpitched::Rest(Rest::default()),
        };
        if let PitchRestUnpitched::Rest(r) = &full_note.content {
            assert!(r.measure.is_none());
        } else {
            panic!("Expected Rest variant");
        }
    }

    #[test]
    fn test_fullnote_unpitched() {
        let full_note = FullNote {
            chord: false,
            content: PitchRestUnpitched::Unpitched(Unpitched::default()),
        };
        if let PitchRestUnpitched::Unpitched(u) = &full_note.content {
            assert!(u.display_step.is_none());
        } else {
            panic!("Expected Unpitched variant");
        }
    }

    #[test]
    fn test_fullnote_chord() {
        let full_note = FullNote {
            chord: true,
            content: PitchRestUnpitched::Pitch(Pitch {
                step: Step::E,
                alter: None,
                octave: 4,
            }),
        };
        assert!(full_note.chord);
    }

    #[test]
    fn test_fullnote_clone() {
        let full_note = FullNote {
            chord: false,
            content: PitchRestUnpitched::Pitch(Pitch {
                step: Step::G,
                alter: Some(1.0),
                octave: 5,
            }),
        };
        let cloned = full_note.clone();
        assert_eq!(full_note, cloned);
    }

    // === PitchRestUnpitched Tests ===

    #[test]
    fn test_pitchrestunpitched_pitch_variant() {
        let content = PitchRestUnpitched::Pitch(Pitch {
            step: Step::A,
            alter: Some(-1.0),
            octave: 3,
        });
        if let PitchRestUnpitched::Pitch(p) = content {
            assert_eq!(p.step, Step::A);
            assert_eq!(p.alter, Some(-1.0));
        } else {
            panic!("Expected Pitch variant");
        }
    }

    #[test]
    fn test_pitchrestunpitched_rest_variant() {
        let content = PitchRestUnpitched::Rest(Rest {
            measure: Some(YesNo::Yes),
            display_step: None,
            display_octave: None,
        });
        if let PitchRestUnpitched::Rest(r) = content {
            assert_eq!(r.measure, Some(YesNo::Yes));
        } else {
            panic!("Expected Rest variant");
        }
    }

    #[test]
    fn test_pitchrestunpitched_unpitched_variant() {
        let content = PitchRestUnpitched::Unpitched(Unpitched {
            display_step: Some(Step::E),
            display_octave: Some(4),
        });
        if let PitchRestUnpitched::Unpitched(u) = content {
            assert_eq!(u.display_step, Some(Step::E));
        } else {
            panic!("Expected Unpitched variant");
        }
    }

    // === Rest Tests ===

    #[test]
    fn test_rest_default() {
        let rest = Rest::default();
        assert!(rest.measure.is_none());
        assert!(rest.display_step.is_none());
        assert!(rest.display_octave.is_none());
    }

    #[test]
    fn test_rest_whole_measure() {
        let rest = Rest {
            measure: Some(YesNo::Yes),
            display_step: None,
            display_octave: None,
        };
        assert_eq!(rest.measure, Some(YesNo::Yes));
    }

    #[test]
    fn test_rest_with_display_position() {
        let rest = Rest {
            measure: None,
            display_step: Some(Step::B),
            display_octave: Some(4),
        };
        assert_eq!(rest.display_step, Some(Step::B));
        assert_eq!(rest.display_octave, Some(4));
    }

    #[test]
    fn test_rest_clone() {
        let rest = Rest {
            measure: Some(YesNo::No),
            display_step: Some(Step::D),
            display_octave: Some(5),
        };
        let cloned = rest.clone();
        assert_eq!(rest, cloned);
    }

    // === Grace Tests ===

    #[test]
    fn test_grace_default() {
        let grace = Grace::default();
        assert!(grace.steal_time_previous.is_none());
        assert!(grace.steal_time_following.is_none());
        assert!(grace.make_time.is_none());
        assert!(grace.slash.is_none());
    }

    #[test]
    fn test_grace_with_slash() {
        let grace = Grace {
            steal_time_previous: None,
            steal_time_following: None,
            make_time: None,
            slash: Some(YesNo::Yes),
        };
        assert_eq!(grace.slash, Some(YesNo::Yes));
    }

    #[test]
    fn test_grace_steal_time_previous() {
        let grace = Grace {
            steal_time_previous: Some(50.0),
            steal_time_following: None,
            make_time: None,
            slash: None,
        };
        assert_eq!(grace.steal_time_previous, Some(50.0));
    }

    #[test]
    fn test_grace_steal_time_following() {
        let grace = Grace {
            steal_time_previous: None,
            steal_time_following: Some(25.0),
            make_time: None,
            slash: None,
        };
        assert_eq!(grace.steal_time_following, Some(25.0));
    }

    #[test]
    fn test_grace_make_time() {
        let grace = Grace {
            steal_time_previous: None,
            steal_time_following: None,
            make_time: Some(10),
            slash: None,
        };
        assert_eq!(grace.make_time, Some(10));
    }

    #[test]
    fn test_grace_clone() {
        let grace = Grace {
            steal_time_previous: Some(30.0),
            steal_time_following: Some(20.0),
            make_time: Some(5),
            slash: Some(YesNo::Yes),
        };
        let cloned = grace.clone();
        assert_eq!(grace, cloned);
    }

    // === Tie Tests ===

    #[test]
    fn test_tie_start() {
        let tie = Tie {
            r#type: StartStop::Start,
            time_only: None,
        };
        assert_eq!(tie.r#type, StartStop::Start);
    }

    #[test]
    fn test_tie_stop() {
        let tie = Tie {
            r#type: StartStop::Stop,
            time_only: None,
        };
        assert_eq!(tie.r#type, StartStop::Stop);
    }

    #[test]
    fn test_tie_with_time_only() {
        let tie = Tie {
            r#type: StartStop::Start,
            time_only: Some("1".to_string()),
        };
        assert_eq!(tie.time_only, Some("1".to_string()));
    }

    #[test]
    fn test_tie_clone() {
        let tie = Tie {
            r#type: StartStop::Start,
            time_only: Some("2".to_string()),
        };
        let cloned = tie.clone();
        assert_eq!(tie, cloned);
    }

    // === Accidental Tests ===

    #[test]
    fn test_accidental_sharp() {
        let acc = Accidental {
            value: AccidentalValue::Sharp,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: None,
            size: None,
        };
        assert_eq!(acc.value, AccidentalValue::Sharp);
    }

    #[test]
    fn test_accidental_flat() {
        let acc = Accidental {
            value: AccidentalValue::Flat,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: None,
            size: None,
        };
        assert_eq!(acc.value, AccidentalValue::Flat);
    }

    #[test]
    fn test_accidental_natural() {
        let acc = Accidental {
            value: AccidentalValue::Natural,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: None,
            size: None,
        };
        assert_eq!(acc.value, AccidentalValue::Natural);
    }

    #[test]
    fn test_accidental_cautionary() {
        let acc = Accidental {
            value: AccidentalValue::Natural,
            cautionary: Some(YesNo::Yes),
            editorial: None,
            parentheses: None,
            bracket: None,
            size: None,
        };
        assert_eq!(acc.cautionary, Some(YesNo::Yes));
    }

    #[test]
    fn test_accidental_editorial() {
        let acc = Accidental {
            value: AccidentalValue::Sharp,
            cautionary: None,
            editorial: Some(YesNo::Yes),
            parentheses: None,
            bracket: None,
            size: None,
        };
        assert_eq!(acc.editorial, Some(YesNo::Yes));
    }

    #[test]
    fn test_accidental_parentheses() {
        let acc = Accidental {
            value: AccidentalValue::Flat,
            cautionary: None,
            editorial: None,
            parentheses: Some(YesNo::Yes),
            bracket: None,
            size: None,
        };
        assert_eq!(acc.parentheses, Some(YesNo::Yes));
    }

    #[test]
    fn test_accidental_bracket() {
        let acc = Accidental {
            value: AccidentalValue::DoubleSharp,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: Some(YesNo::Yes),
            size: None,
        };
        assert_eq!(acc.bracket, Some(YesNo::Yes));
    }

    #[test]
    fn test_accidental_with_size() {
        let acc = Accidental {
            value: AccidentalValue::Natural,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: None,
            size: Some(SymbolSize::Cue),
        };
        assert_eq!(acc.size, Some(SymbolSize::Cue));
    }

    #[test]
    fn test_accidental_clone() {
        let acc = Accidental {
            value: AccidentalValue::DoubleFlat,
            cautionary: Some(YesNo::Yes),
            editorial: Some(YesNo::No),
            parentheses: Some(YesNo::Yes),
            bracket: None,
            size: Some(SymbolSize::Full),
        };
        let cloned = acc.clone();
        assert_eq!(acc, cloned);
    }

    // === Instrument Tests ===

    #[test]
    fn test_instrument_construction() {
        let inst = Instrument {
            id: "P1-I1".to_string(),
        };
        assert_eq!(inst.id, "P1-I1");
    }

    #[test]
    fn test_instrument_clone() {
        let inst = Instrument {
            id: "Piano".to_string(),
        };
        let cloned = inst.clone();
        assert_eq!(inst, cloned);
    }

    // === NoteContent Tests ===

    #[test]
    fn test_notecontent_regular() {
        let content = NoteContent::Regular {
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
        };
        if let NoteContent::Regular {
            full_note,
            duration,
            ties,
        } = content
        {
            assert!(!full_note.chord);
            assert_eq!(duration, 4);
            assert!(ties.is_empty());
        } else {
            panic!("Expected Regular variant");
        }
    }

    #[test]
    fn test_notecontent_regular_with_ties() {
        let content = NoteContent::Regular {
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Pitch(Pitch {
                    step: Step::D,
                    alter: None,
                    octave: 4,
                }),
            },
            duration: 8,
            ties: vec![
                Tie {
                    r#type: StartStop::Start,
                    time_only: None,
                },
                Tie {
                    r#type: StartStop::Stop,
                    time_only: None,
                },
            ],
        };
        if let NoteContent::Regular { ties, .. } = content {
            assert_eq!(ties.len(), 2);
        } else {
            panic!("Expected Regular variant");
        }
    }

    #[test]
    fn test_notecontent_grace() {
        let content = NoteContent::Grace {
            grace: Grace {
                slash: Some(YesNo::Yes),
                ..Default::default()
            },
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Pitch(Pitch {
                    step: Step::E,
                    alter: None,
                    octave: 5,
                }),
            },
            ties: vec![],
        };
        if let NoteContent::Grace { grace, .. } = content {
            assert_eq!(grace.slash, Some(YesNo::Yes));
        } else {
            panic!("Expected Grace variant");
        }
    }

    #[test]
    fn test_notecontent_cue() {
        let content = NoteContent::Cue {
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Pitch(Pitch {
                    step: Step::F,
                    alter: Some(1.0),
                    octave: 4,
                }),
            },
            duration: 2,
        };
        if let NoteContent::Cue {
            full_note,
            duration,
        } = content
        {
            assert_eq!(duration, 2);
            if let PitchRestUnpitched::Pitch(p) = &full_note.content {
                assert_eq!(p.step, Step::F);
            }
        } else {
            panic!("Expected Cue variant");
        }
    }

    // === Note Tests ===

    #[test]
    fn test_note_simple_quarter() {
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
                duration: 1,
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
            staff: Some(1),
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };
        assert_eq!(note.voice, Some("1".to_string()));
        assert_eq!(note.staff, Some(1));
    }

    #[test]
    fn test_note_with_dynamics() {
        let note = Note {
            position: Position::default(),
            dynamics: Some(80.0),
            end_dynamics: Some(70.0),
            attack: Some(-10),
            release: Some(5),
            pizzicato: Some(true),
            print_object: Some(YesNo::Yes),
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::A,
                        alter: None,
                        octave: 4,
                    }),
                },
                duration: 2,
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
        };
        assert_eq!(note.dynamics, Some(80.0));
        assert_eq!(note.end_dynamics, Some(70.0));
        assert_eq!(note.attack, Some(-10));
        assert_eq!(note.release, Some(5));
        assert_eq!(note.pizzicato, Some(true));
    }

    #[test]
    fn test_note_clone() {
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
            voice: Some("2".to_string()),
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
        let cloned = note.clone();
        assert_eq!(note, cloned);
    }

    #[test]
    fn test_note_with_instruments() {
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
                        step: Step::G,
                        alter: None,
                        octave: 4,
                    }),
                },
                duration: 1,
                ties: vec![],
            },
            instrument: vec![
                Instrument {
                    id: "I1".to_string(),
                },
                Instrument {
                    id: "I2".to_string(),
                },
            ],
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
        };
        assert_eq!(note.instrument.len(), 2);
    }
}
