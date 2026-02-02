//! S-expression conversions for `ir::note` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for note-related types:
//! - [`Rest`] - A rest
//! - [`PitchRestUnpitched`] - Union type for note content (pitch, rest, or unpitched)
//! - [`FullNote`] - Pitch/rest with chord flag
//! - [`Tie`] - Tie (playback, not visual)
//! - [`Grace`] - Grace note attributes
//! - [`NoteContent`] - Regular/Grace/Cue variants
//! - [`Accidental`] - Displayed accidental
//! - [`Instrument`] - Instrument reference
//! - [`Note`] - The complete note structure

use crate::ir::common::Position;
use crate::ir::note::{
    Accidental, FullNote, Grace, Instrument, Note, NoteContent, PitchRestUnpitched, Rest, Tie,
};
use crate::ir::pitch::{Pitch, Unpitched};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, get_head, optional_kwarg, require_kwarg};

// ============================================================================
// Rest
// ============================================================================

impl ToSexpr for Rest {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("rest")
            .kwarg_opt("measure", &self.measure)
            .kwarg_opt("display-step", &self.display_step)
            .kwarg_opt("display-octave", &self.display_octave)
            .build()
    }
}

impl FromSexpr for Rest {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("rest list", sexpr))?;

        expect_head(list, "rest")?;

        Ok(Rest {
            measure: optional_kwarg(list, "measure")?,
            display_step: optional_kwarg(list, "display-step")?,
            display_octave: optional_kwarg(list, "display-octave")?,
        })
    }
}

// ============================================================================
// PitchRestUnpitched
// ============================================================================

impl ToSexpr for PitchRestUnpitched {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            PitchRestUnpitched::Pitch(p) => p.to_sexpr(),
            PitchRestUnpitched::Rest(r) => r.to_sexpr(),
            PitchRestUnpitched::Unpitched(u) => u.to_sexpr(),
        }
    }
}

impl FromSexpr for PitchRestUnpitched {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pitch/rest/unpitched", sexpr))?;

        match get_head(list)? {
            "pitch" => Ok(PitchRestUnpitched::Pitch(Pitch::from_sexpr(sexpr)?)),
            "rest" => Ok(PitchRestUnpitched::Rest(Rest::from_sexpr(sexpr)?)),
            "unpitched" => Ok(PitchRestUnpitched::Unpitched(Unpitched::from_sexpr(sexpr)?)),
            other => Err(ConvertError::InvalidVariant(other.to_string())),
        }
    }
}

// ============================================================================
// FullNote
// ============================================================================

impl ToSexpr for FullNote {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("full-note")
            .kwarg_bool("chord", self.chord)
            .kwarg_raw("content", self.content.to_sexpr())
            .build()
    }
}

impl FromSexpr for FullNote {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("full-note list", sexpr))?;

        expect_head(list, "full-note")?;

        let chord = optional_kwarg::<bool>(list, "chord")?.unwrap_or(false);
        let content = require_kwarg(list, "content")?;

        Ok(FullNote { chord, content })
    }
}

// ============================================================================
// Tie
// ============================================================================

impl ToSexpr for Tie {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("tie")
            .kwarg("type", &self.r#type)
            .kwarg_opt("time-only", &self.time_only)
            .build()
    }
}

impl FromSexpr for Tie {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tie list", sexpr))?;

        expect_head(list, "tie")?;

        Ok(Tie {
            r#type: require_kwarg(list, "type")?,
            time_only: optional_kwarg(list, "time-only")?,
        })
    }
}

// ============================================================================
// Grace
// ============================================================================

impl ToSexpr for Grace {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("grace")
            .kwarg_opt("steal-time-previous", &self.steal_time_previous)
            .kwarg_opt("steal-time-following", &self.steal_time_following)
            .kwarg_opt("make-time", &self.make_time)
            .kwarg_opt("slash", &self.slash)
            .build()
    }
}

impl FromSexpr for Grace {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("grace list", sexpr))?;

        expect_head(list, "grace")?;

        Ok(Grace {
            steal_time_previous: optional_kwarg(list, "steal-time-previous")?,
            steal_time_following: optional_kwarg(list, "steal-time-following")?,
            make_time: optional_kwarg(list, "make-time")?,
            slash: optional_kwarg(list, "slash")?,
        })
    }
}

// ============================================================================
// NoteContent
// ============================================================================

impl ToSexpr for NoteContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            NoteContent::Regular {
                full_note,
                duration,
                ties,
            } => {
                let mut builder = ListBuilder::new("regular")
                    .kwarg_raw("full-note", full_note.to_sexpr())
                    .kwarg("duration", duration);

                if !ties.is_empty() {
                    builder = builder.kwarg_list("ties", ties);
                }
                builder.build()
            }
            NoteContent::Grace {
                grace,
                full_note,
                ties,
            } => {
                let mut builder = ListBuilder::new("grace-note")
                    .kwarg_raw("grace", grace.to_sexpr())
                    .kwarg_raw("full-note", full_note.to_sexpr());

                if !ties.is_empty() {
                    builder = builder.kwarg_list("ties", ties);
                }
                builder.build()
            }
            NoteContent::Cue {
                full_note,
                duration,
            } => ListBuilder::new("cue")
                .kwarg_raw("full-note", full_note.to_sexpr())
                .kwarg("duration", duration)
                .build(),
        }
    }
}

impl FromSexpr for NoteContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("note-content list", sexpr))?;

        match get_head(list)? {
            "regular" => {
                let full_note = require_kwarg(list, "full-note")?;
                let duration = require_kwarg(list, "duration")?;
                let ties = optional_kwarg::<Vec<Tie>>(list, "ties")?.unwrap_or_default();
                Ok(NoteContent::Regular {
                    full_note,
                    duration,
                    ties,
                })
            }
            "grace-note" => {
                let grace = require_kwarg(list, "grace")?;
                let full_note = require_kwarg(list, "full-note")?;
                let ties = optional_kwarg::<Vec<Tie>>(list, "ties")?.unwrap_or_default();
                Ok(NoteContent::Grace {
                    grace,
                    full_note,
                    ties,
                })
            }
            "cue" => {
                let full_note = require_kwarg(list, "full-note")?;
                let duration = require_kwarg(list, "duration")?;
                Ok(NoteContent::Cue {
                    full_note,
                    duration,
                })
            }
            other => Err(ConvertError::InvalidVariant(other.to_string())),
        }
    }
}

// ============================================================================
// Accidental
// ============================================================================

impl ToSexpr for Accidental {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("accidental")
            .kwarg("value", &self.value)
            .kwarg_opt("cautionary", &self.cautionary)
            .kwarg_opt("editorial", &self.editorial)
            .kwarg_opt("parentheses", &self.parentheses)
            .kwarg_opt("bracket", &self.bracket)
            .kwarg_opt("size", &self.size)
            .build()
    }
}

impl FromSexpr for Accidental {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("accidental list", sexpr))?;

        expect_head(list, "accidental")?;

        Ok(Accidental {
            value: require_kwarg(list, "value")?,
            cautionary: optional_kwarg(list, "cautionary")?,
            editorial: optional_kwarg(list, "editorial")?,
            parentheses: optional_kwarg(list, "parentheses")?,
            bracket: optional_kwarg(list, "bracket")?,
            size: optional_kwarg(list, "size")?,
        })
    }
}

// ============================================================================
// Instrument
// ============================================================================

impl ToSexpr for Instrument {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("instrument").kwarg("id", &self.id).build()
    }
}

impl FromSexpr for Instrument {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("instrument list", sexpr))?;

        expect_head(list, "instrument")?;

        Ok(Instrument {
            id: require_kwarg(list, "id")?,
        })
    }
}

// ============================================================================
// Note
// ============================================================================

impl ToSexpr for Note {
    fn to_sexpr(&self) -> Sexpr {
        // Check if position has any content
        let pos_has_content = self.position.default_x.is_some()
            || self.position.default_y.is_some()
            || self.position.relative_x.is_some()
            || self.position.relative_y.is_some();

        let mut builder = ListBuilder::new("note");

        // Position/playback attributes
        if pos_has_content {
            builder = builder.kwarg_raw("position", self.position.to_sexpr());
        }
        builder = builder
            .kwarg_opt("dynamics", &self.dynamics)
            .kwarg_opt("end-dynamics", &self.end_dynamics)
            .kwarg_opt("attack", &self.attack)
            .kwarg_opt("release", &self.release)
            .kwarg_opt("pizzicato", &self.pizzicato)
            .kwarg_opt("print-object", &self.print_object);

        // Content
        builder = builder.kwarg_raw("content", self.content.to_sexpr());

        // Instruments (Vec)
        if !self.instrument.is_empty() {
            builder = builder.kwarg_list("instrument", &self.instrument);
        }

        // Voice
        builder = builder.kwarg_opt("voice", &self.voice);

        // Type
        if let Some(ref t) = self.r#type {
            builder = builder.kwarg_raw("type", t.to_sexpr());
        }

        // Dots
        if !self.dots.is_empty() {
            builder = builder.kwarg_list("dots", &self.dots);
        }

        // Accidental
        if let Some(ref acc) = self.accidental {
            builder = builder.kwarg_raw("accidental", acc.to_sexpr());
        }

        // Time modification
        if let Some(ref tm) = self.time_modification {
            builder = builder.kwarg_raw("time-modification", tm.to_sexpr());
        }

        // Stem
        if let Some(ref stem) = self.stem {
            builder = builder.kwarg_raw("stem", stem.to_sexpr());
        }

        // Notehead
        if let Some(ref nh) = self.notehead {
            builder = builder.kwarg_raw("notehead", nh.to_sexpr());
        }

        // Staff
        builder = builder.kwarg_opt("staff", &self.staff);

        // Beams
        if !self.beams.is_empty() {
            builder = builder.kwarg_list("beams", &self.beams);
        }

        // Notations and Lyrics are covered in Milestone 3
        // Their trait implementations will be added then

        builder.build()
    }
}

impl FromSexpr for Note {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("note list", sexpr))?;

        expect_head(list, "note")?;

        // Parse position (defaults to empty if not present)
        let position = match find_kwarg(list, "position") {
            Some(pos_sexpr) => Position::from_sexpr(pos_sexpr)?,
            None => Position::default(),
        };

        Ok(Note {
            position,
            dynamics: optional_kwarg(list, "dynamics")?,
            end_dynamics: optional_kwarg(list, "end-dynamics")?,
            attack: optional_kwarg(list, "attack")?,
            release: optional_kwarg(list, "release")?,
            pizzicato: optional_kwarg(list, "pizzicato")?,
            print_object: optional_kwarg(list, "print-object")?,
            content: require_kwarg(list, "content")?,
            instrument: optional_kwarg::<Vec<Instrument>>(list, "instrument")?.unwrap_or_default(),
            voice: optional_kwarg(list, "voice")?,
            r#type: optional_kwarg(list, "type")?,
            dots: optional_kwarg(list, "dots")?.unwrap_or_default(),
            accidental: optional_kwarg(list, "accidental")?,
            time_modification: optional_kwarg(list, "time-modification")?,
            stem: optional_kwarg(list, "stem")?,
            notehead: optional_kwarg(list, "notehead")?,
            staff: optional_kwarg(list, "staff")?,
            beams: optional_kwarg(list, "beams")?.unwrap_or_default(),
            // Notations and Lyrics are covered in Milestone 3
            notations: vec![],
            lyrics: vec![],
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::beam::{Beam, BeamValue, Stem, StemValue};
    use crate::ir::common::{AccidentalValue, StartStop, SymbolSize, YesNo};
    use crate::ir::duration::{NoteType, NoteTypeValue};
    use crate::ir::pitch::Step;
    use crate::sexpr::{parse, print_sexpr};

    // === Rest Tests ===

    #[test]
    fn test_rest_to_sexpr_default() {
        let rest = Rest::default();
        let sexpr = rest.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(rest)");
    }

    #[test]
    fn test_rest_to_sexpr_whole_measure() {
        let rest = Rest {
            measure: Some(YesNo::Yes),
            display_step: None,
            display_octave: None,
        };
        let sexpr = rest.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(rest :measure yes)");
    }

    #[test]
    fn test_rest_to_sexpr_with_display() {
        let rest = Rest {
            measure: None,
            display_step: Some(Step::B),
            display_octave: Some(4),
        };
        let sexpr = rest.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":display-step B"));
        assert!(text.contains(":display-octave 4"));
    }

    #[test]
    fn test_rest_from_sexpr_default() {
        let sexpr = parse("(rest)").unwrap();
        let rest = Rest::from_sexpr(&sexpr).unwrap();
        assert!(rest.measure.is_none());
        assert!(rest.display_step.is_none());
        assert!(rest.display_octave.is_none());
    }

    #[test]
    fn test_rest_from_sexpr_whole_measure() {
        let sexpr = parse("(rest :measure yes)").unwrap();
        let rest = Rest::from_sexpr(&sexpr).unwrap();
        assert_eq!(rest.measure, Some(YesNo::Yes));
    }

    #[test]
    fn test_rest_round_trip() {
        let original = Rest {
            measure: Some(YesNo::No),
            display_step: Some(Step::D),
            display_octave: Some(5),
        };
        let sexpr = original.to_sexpr();
        let parsed = Rest::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === PitchRestUnpitched Tests ===

    #[test]
    fn test_pitchrestunpitched_pitch() {
        let content = PitchRestUnpitched::Pitch(Pitch {
            step: Step::C,
            alter: None,
            octave: 4,
        });
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.starts_with("(pitch"));
    }

    #[test]
    fn test_pitchrestunpitched_rest() {
        let content = PitchRestUnpitched::Rest(Rest::default());
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.starts_with("(rest"));
    }

    #[test]
    fn test_pitchrestunpitched_unpitched() {
        let content = PitchRestUnpitched::Unpitched(Unpitched::default());
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.starts_with("(unpitched"));
    }

    #[test]
    fn test_pitchrestunpitched_round_trip_pitch() {
        let original = PitchRestUnpitched::Pitch(Pitch {
            step: Step::F,
            alter: Some(1.0),
            octave: 4,
        });
        let sexpr = original.to_sexpr();
        let parsed = PitchRestUnpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_pitchrestunpitched_round_trip_rest() {
        let original = PitchRestUnpitched::Rest(Rest {
            measure: Some(YesNo::Yes),
            display_step: None,
            display_octave: None,
        });
        let sexpr = original.to_sexpr();
        let parsed = PitchRestUnpitched::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === FullNote Tests ===

    #[test]
    fn test_fullnote_to_sexpr_simple() {
        let full_note = FullNote {
            chord: false,
            content: PitchRestUnpitched::Pitch(Pitch {
                step: Step::C,
                alter: None,
                octave: 4,
            }),
        };
        let sexpr = full_note.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("(full-note"));
        assert!(text.contains(":content (pitch"));
        assert!(!text.contains(":chord"));
    }

    #[test]
    fn test_fullnote_to_sexpr_chord() {
        let full_note = FullNote {
            chord: true,
            content: PitchRestUnpitched::Pitch(Pitch {
                step: Step::E,
                alter: None,
                octave: 4,
            }),
        };
        let sexpr = full_note.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":chord t"));
    }

    #[test]
    fn test_fullnote_round_trip() {
        let original = FullNote {
            chord: true,
            content: PitchRestUnpitched::Rest(Rest::default()),
        };
        let sexpr = original.to_sexpr();
        let parsed = FullNote::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === Tie Tests ===

    #[test]
    fn test_tie_to_sexpr_start() {
        let tie = Tie {
            r#type: StartStop::Start,
            time_only: None,
        };
        let sexpr = tie.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(tie :type start)");
    }

    #[test]
    fn test_tie_to_sexpr_with_time_only() {
        let tie = Tie {
            r#type: StartStop::Start,
            time_only: Some("1".to_string()),
        };
        let sexpr = tie.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":time-only \"1\""));
    }

    #[test]
    fn test_tie_round_trip() {
        let original = Tie {
            r#type: StartStop::Stop,
            time_only: Some("2".to_string()),
        };
        let sexpr = original.to_sexpr();
        let parsed = Tie::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === Grace Tests ===

    #[test]
    fn test_grace_to_sexpr_default() {
        let grace = Grace::default();
        let sexpr = grace.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(grace)");
    }

    #[test]
    fn test_grace_to_sexpr_with_slash() {
        let grace = Grace {
            steal_time_previous: None,
            steal_time_following: None,
            make_time: None,
            slash: Some(YesNo::Yes),
        };
        let sexpr = grace.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(grace :slash yes)");
    }

    #[test]
    fn test_grace_to_sexpr_with_steal_time() {
        let grace = Grace {
            steal_time_previous: Some(50.0),
            steal_time_following: Some(25.0),
            make_time: Some(10),
            slash: None,
        };
        let sexpr = grace.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":steal-time-previous 50.0"));
        assert!(text.contains(":steal-time-following 25.0"));
        assert!(text.contains(":make-time 10"));
    }

    #[test]
    fn test_grace_round_trip() {
        let original = Grace {
            steal_time_previous: Some(30.0),
            steal_time_following: Some(20.0),
            make_time: Some(5),
            slash: Some(YesNo::Yes),
        };
        let sexpr = original.to_sexpr();
        let parsed = Grace::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
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
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.starts_with("(regular"));
        assert!(text.contains(":full-note"));
        assert!(text.contains(":duration 4"));
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
            ties: vec![Tie {
                r#type: StartStop::Start,
                time_only: None,
            }],
        };
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":ties"));
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
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.starts_with("(grace-note"));
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
        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.starts_with("(cue"));
    }

    #[test]
    fn test_notecontent_round_trip_regular() {
        let original = NoteContent::Regular {
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Pitch(Pitch {
                    step: Step::G,
                    alter: None,
                    octave: 4,
                }),
            },
            duration: 4,
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
        let sexpr = original.to_sexpr();
        let parsed = NoteContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_notecontent_round_trip_grace() {
        let original = NoteContent::Grace {
            grace: Grace {
                slash: Some(YesNo::Yes),
                steal_time_previous: Some(50.0),
                ..Default::default()
            },
            full_note: FullNote {
                chord: false,
                content: PitchRestUnpitched::Pitch(Pitch {
                    step: Step::A,
                    alter: None,
                    octave: 5,
                }),
            },
            ties: vec![],
        };
        let sexpr = original.to_sexpr();
        let parsed = NoteContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === Accidental Tests ===

    #[test]
    fn test_accidental_to_sexpr_simple() {
        let acc = Accidental {
            value: AccidentalValue::Sharp,
            cautionary: None,
            editorial: None,
            parentheses: None,
            bracket: None,
            size: None,
        };
        let sexpr = acc.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(accidental :value sharp)");
    }

    #[test]
    fn test_accidental_to_sexpr_with_options() {
        let acc = Accidental {
            value: AccidentalValue::Natural,
            cautionary: Some(YesNo::Yes),
            editorial: Some(YesNo::No),
            parentheses: Some(YesNo::Yes),
            bracket: None,
            size: Some(SymbolSize::Cue),
        };
        let sexpr = acc.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":cautionary yes"));
        assert!(text.contains(":editorial no"));
        assert!(text.contains(":parentheses yes"));
        assert!(text.contains(":size cue"));
    }

    #[test]
    fn test_accidental_round_trip() {
        let original = Accidental {
            value: AccidentalValue::DoubleSharp,
            cautionary: Some(YesNo::Yes),
            editorial: Some(YesNo::No),
            parentheses: Some(YesNo::Yes),
            bracket: Some(YesNo::No),
            size: Some(SymbolSize::Full),
        };
        let sexpr = original.to_sexpr();
        let parsed = Accidental::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    // === Instrument Tests ===

    #[test]
    fn test_instrument_to_sexpr() {
        let inst = Instrument {
            id: "P1-I1".to_string(),
        };
        let sexpr = inst.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert_eq!(text, "(instrument :id \"P1-I1\")");
    }

    #[test]
    fn test_instrument_round_trip() {
        let original = Instrument {
            id: "Piano".to_string(),
        };
        let sexpr = original.to_sexpr();
        let parsed = Instrument::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
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
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
            dots: vec![],
            accidental: None,
            time_modification: None,
            stem: Some(Stem {
                value: StemValue::Up,
                default_y: None,
                color: None,
            }),
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        let sexpr = note.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("(note"));
        assert!(text.contains(":content (regular"));
        assert!(text.contains(":voice \"1\""));
        assert!(text.contains(":type (note-type :value quarter)"));
        assert!(text.contains(":stem (stem :value up)"));
    }

    #[test]
    fn test_note_rest() {
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
                duration: 2,
                ties: vec![],
            },
            instrument: vec![],
            voice: None,
            r#type: Some(NoteType {
                value: NoteTypeValue::Half,
                size: None,
            }),
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

        let sexpr = note.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("(rest)"));
    }

    #[test]
    fn test_note_chord_tone() {
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
                    chord: true,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::E,
                        alter: None,
                        octave: 4,
                    }),
                },
                duration: 1,
                ties: vec![],
            },
            instrument: vec![],
            voice: None,
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
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

        let sexpr = note.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":chord t"));
    }

    #[test]
    fn test_note_with_beams() {
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
            instrument: vec![],
            voice: None,
            r#type: Some(NoteType {
                value: NoteTypeValue::Eighth,
                size: None,
            }),
            dots: vec![],
            accidental: None,
            time_modification: None,
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![Beam {
                value: BeamValue::Begin,
                number: 1,
                fan: None,
                color: None,
            }],
            notations: vec![],
            lyrics: vec![],
        };

        let sexpr = note.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":beams"));
        assert!(text.contains("(beam :value begin :number 1)"));
    }

    #[test]
    fn test_note_round_trip_simple() {
        let original = Note {
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
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
            dots: vec![],
            accidental: None,
            time_modification: None,
            stem: Some(Stem {
                value: StemValue::Up,
                default_y: None,
                color: None,
            }),
            notehead: None,
            staff: Some(1),
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        let sexpr = original.to_sexpr();
        let parsed = Note::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_note_round_trip_with_accidental() {
        let original = Note {
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
                        step: Step::F,
                        alter: Some(1.0),
                        octave: 4,
                    }),
                },
                duration: 1,
                ties: vec![],
            },
            instrument: vec![],
            voice: None,
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
            dots: vec![],
            accidental: Some(Accidental {
                value: AccidentalValue::Sharp,
                cautionary: Some(YesNo::Yes),
                editorial: None,
                parentheses: None,
                bracket: None,
                size: None,
            }),
            time_modification: None,
            stem: None,
            notehead: None,
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
        };

        let sexpr = original.to_sexpr();
        let parsed = Note::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_note_round_trip_grace() {
        let original = Note {
            position: Position::default(),
            dynamics: None,
            end_dynamics: None,
            attack: None,
            release: None,
            pizzicato: None,
            print_object: None,
            content: NoteContent::Grace {
                grace: Grace {
                    slash: Some(YesNo::Yes),
                    ..Default::default()
                },
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::D,
                        alter: None,
                        octave: 5,
                    }),
                },
                ties: vec![],
            },
            instrument: vec![],
            voice: None,
            r#type: Some(NoteType {
                value: NoteTypeValue::Eighth,
                size: None,
            }),
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

        let sexpr = original.to_sexpr();
        let parsed = Note::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_note_round_trip_with_dynamics() {
        let original = Note {
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

        let sexpr = original.to_sexpr();
        let parsed = Note::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }
}
