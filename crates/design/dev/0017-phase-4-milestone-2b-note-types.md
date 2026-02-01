# Phase 4 — Milestone 2B: Note Types

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Note, NoteContent, FullNote, Rest, Grace, Accidental
> **Depends On:** Milestone 2A
> **Estimated Implementation Time:** 2-3 hours

---

## Overview

The Note type is the most complex in the IR. This document covers:

- `Rest` — Rest indicator
- `PitchRestUnpitched` — Union type for note content
- `FullNote` — Pitch/rest with chord flag
- `Tie` — Tie sound element
- `Grace` — Grace note marker
- `NoteContent` — Regular/Grace/Cue variants
- `Accidental` — Displayed accidental
- `Note` — The complete note structure

---

## Task 1: Rest and Content Types (`src/sexpr/convert/note.rs`)

```rust
use crate::ir::note::*;
use crate::ir::pitch::*;
use crate::ir::duration::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

// ============ Rest ============

impl ToSexpr for Rest {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("rest");
        if let Some(ref step) = self.display_step {
            builder = builder.kwarg("display-step", step);
        }
        if let Some(ref oct) = self.display_octave {
            builder = builder.kwarg("display-octave", oct);
        }
        if let Some(measure) = self.measure {
            builder = builder.kwarg_bool("measure", measure);
        }
        builder.build()
    }
}

impl FromSexpr for Rest {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("rest list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("rest") {
            return Err(ConvertError::ExpectedHead("rest"));
        }

        Ok(Rest {
            display_step: optional_kwarg(list, "display-step")?,
            display_octave: optional_kwarg(list, "display-octave")?,
            measure: optional_kwarg(list, "measure")?,
        })
    }
}

// ============ PitchRestUnpitched ============

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
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pitch/rest/unpitched", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("pitch") => Ok(PitchRestUnpitched::Pitch(Pitch::from_sexpr(sexpr)?)),
            Some("rest") => Ok(PitchRestUnpitched::Rest(Rest::from_sexpr(sexpr)?)),
            Some("unpitched") => Ok(PitchRestUnpitched::Unpitched(Unpitched::from_sexpr(sexpr)?)),
            _ => Err(ConvertError::type_mismatch("pitch/rest/unpitched", sexpr)),
        }
    }
}

// ============ FullNote ============

impl ToSexpr for FullNote {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("full-note");
        if self.chord {
            builder = builder.kwarg_bool("chord", true);
        }
        builder = builder.kwarg_raw("content", self.content.to_sexpr());
        builder.build()
    }
}

impl FromSexpr for FullNote {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("full-note list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("full-note") {
            return Err(ConvertError::ExpectedHead("full-note"));
        }

        let chord = optional_kwarg::<bool>(list, "chord")?.unwrap_or(false);
        let content = require_kwarg(list, "content")?;

        Ok(FullNote { chord, content })
    }
}

// ============ Tie ============

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
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tie list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("tie") {
            return Err(ConvertError::ExpectedHead("tie"));
        }

        Ok(Tie {
            r#type: require_kwarg(list, "type")?,
            time_only: optional_kwarg(list, "time-only")?,
        })
    }
}

// ============ Grace ============

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
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("grace list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("grace") {
            return Err(ConvertError::ExpectedHead("grace"));
        }

        Ok(Grace {
            steal_time_previous: optional_kwarg(list, "steal-time-previous")?,
            steal_time_following: optional_kwarg(list, "steal-time-following")?,
            make_time: optional_kwarg(list, "make-time")?,
            slash: optional_kwarg(list, "slash")?,
        })
    }
}
```

---

## Task 2: NoteContent Enum

```rust
// ============ NoteContent ============

impl ToSexpr for NoteContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            NoteContent::Regular { full_note, duration, ties } => {
                let mut builder = ListBuilder::new("regular")
                    .kwarg_raw("full-note", full_note.to_sexpr())
                    .kwarg("duration", duration);

                if !ties.is_empty() {
                    builder = builder.kwarg_list("ties", ties);
                }
                builder.build()
            }
            NoteContent::Grace { grace, full_note, ties } => {
                let mut builder = ListBuilder::new("grace-note")
                    .kwarg_raw("grace", grace.to_sexpr())
                    .kwarg_raw("full-note", full_note.to_sexpr());

                if !ties.is_empty() {
                    builder = builder.kwarg_list("ties", ties);
                }
                builder.build()
            }
            NoteContent::Cue { full_note, duration } => {
                ListBuilder::new("cue")
                    .kwarg_raw("full-note", full_note.to_sexpr())
                    .kwarg("duration", duration)
                    .build()
            }
        }
    }
}

impl FromSexpr for NoteContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("note-content list", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("regular") => {
                let full_note = require_kwarg(list, "full-note")?;
                let duration = require_kwarg(list, "duration")?;
                let ties = optional_kwarg::<Vec<Tie>>(list, "ties")?.unwrap_or_default();
                Ok(NoteContent::Regular { full_note, duration, ties })
            }
            Some("grace-note") => {
                let grace = require_kwarg(list, "grace")?;
                let full_note = require_kwarg(list, "full-note")?;
                let ties = optional_kwarg::<Vec<Tie>>(list, "ties")?.unwrap_or_default();
                Ok(NoteContent::Grace { grace, full_note, ties })
            }
            Some("cue") => {
                let full_note = require_kwarg(list, "full-note")?;
                let duration = require_kwarg(list, "duration")?;
                Ok(NoteContent::Cue { full_note, duration })
            }
            other => Err(ConvertError::InvalidVariant(
                other.map(|s| s.to_string()).unwrap_or_else(|| "non-symbol".to_string())
            )),
        }
    }
}
```

---

## Task 3: Accidental Types

```rust
// ============ AccidentalValue ============

impl ToSexpr for AccidentalValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            AccidentalValue::Sharp => "sharp",
            AccidentalValue::Natural => "natural",
            AccidentalValue::Flat => "flat",
            AccidentalValue::DoubleSharp => "double-sharp",
            AccidentalValue::DoubleFlat => "double-flat",
            AccidentalValue::NaturalSharp => "natural-sharp",
            AccidentalValue::NaturalFlat => "natural-flat",
            AccidentalValue::QuarterFlat => "quarter-flat",
            AccidentalValue::QuarterSharp => "quarter-sharp",
            AccidentalValue::ThreeQuartersFlat => "three-quarters-flat",
            AccidentalValue::ThreeQuartersSharp => "three-quarters-sharp",
            AccidentalValue::SharpDown => "sharp-down",
            AccidentalValue::SharpUp => "sharp-up",
            AccidentalValue::NaturalDown => "natural-down",
            AccidentalValue::NaturalUp => "natural-up",
            AccidentalValue::FlatDown => "flat-down",
            AccidentalValue::FlatUp => "flat-up",
            AccidentalValue::TripleSharp => "triple-sharp",
            AccidentalValue::TripleFlat => "triple-flat",
            AccidentalValue::Sori => "sori",
            AccidentalValue::Koron => "koron",
            AccidentalValue::Other(s) => return Sexpr::string(s),
        })
    }
}

impl FromSexpr for AccidentalValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        // Check for string (Other variant)
        if let Some(s) = sexpr.as_string() {
            return Ok(AccidentalValue::Other(s.to_string()));
        }

        match sexpr.as_symbol() {
            Some("sharp") => Ok(AccidentalValue::Sharp),
            Some("natural") => Ok(AccidentalValue::Natural),
            Some("flat") => Ok(AccidentalValue::Flat),
            Some("double-sharp") | Some("x") => Ok(AccidentalValue::DoubleSharp),
            Some("double-flat") => Ok(AccidentalValue::DoubleFlat),
            Some("natural-sharp") => Ok(AccidentalValue::NaturalSharp),
            Some("natural-flat") => Ok(AccidentalValue::NaturalFlat),
            Some("quarter-flat") => Ok(AccidentalValue::QuarterFlat),
            Some("quarter-sharp") => Ok(AccidentalValue::QuarterSharp),
            Some("three-quarters-flat") => Ok(AccidentalValue::ThreeQuartersFlat),
            Some("three-quarters-sharp") => Ok(AccidentalValue::ThreeQuartersSharp),
            Some("sharp-down") => Ok(AccidentalValue::SharpDown),
            Some("sharp-up") => Ok(AccidentalValue::SharpUp),
            Some("natural-down") => Ok(AccidentalValue::NaturalDown),
            Some("natural-up") => Ok(AccidentalValue::NaturalUp),
            Some("flat-down") => Ok(AccidentalValue::FlatDown),
            Some("flat-up") => Ok(AccidentalValue::FlatUp),
            Some("triple-sharp") => Ok(AccidentalValue::TripleSharp),
            Some("triple-flat") => Ok(AccidentalValue::TripleFlat),
            Some("sori") => Ok(AccidentalValue::Sori),
            Some("koron") => Ok(AccidentalValue::Koron),
            _ => Err(ConvertError::type_mismatch("accidental-value", sexpr)),
        }
    }
}

// ============ Accidental ============

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
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("accidental list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("accidental") {
            return Err(ConvertError::ExpectedHead("accidental"));
        }

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
```

---

## Task 4: The Complete Note Struct

```rust
// ============ Note ============

impl ToSexpr for Note {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("note")
            .kwarg_raw("content", self.content.to_sexpr());

        // Voice
        if let Some(ref v) = self.voice {
            builder = builder.kwarg("voice", v);
        }

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

        // Staff
        if let Some(ref staff) = self.staff {
            builder = builder.kwarg("staff", staff);
        }

        // Beams
        if !self.beams.is_empty() {
            builder = builder.kwarg_list("beams", &self.beams);
        }

        // Notations (covered in Milestone 3)
        if !self.notations.is_empty() {
            builder = builder.kwarg_list("notations", &self.notations);
        }

        // Lyrics (covered in Milestone 3)
        if !self.lyrics.is_empty() {
            builder = builder.kwarg_list("lyrics", &self.lyrics);
        }

        // Notehead
        if let Some(ref nh) = self.notehead {
            builder = builder.kwarg_raw("notehead", nh.to_sexpr());
        }

        // Instrument
        if let Some(ref inst) = self.instrument {
            builder = builder.kwarg("instrument", inst);
        }

        // Additional fields as needed...

        builder.build()
    }
}

impl FromSexpr for Note {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("note list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("note") {
            return Err(ConvertError::ExpectedHead("note"));
        }

        Ok(Note {
            content: require_kwarg(list, "content")?,
            voice: optional_kwarg(list, "voice")?,
            r#type: optional_kwarg(list, "type")?,
            dots: optional_kwarg::<Vec<Dot>>(list, "dots")?.unwrap_or_default(),
            accidental: optional_kwarg(list, "accidental")?,
            time_modification: optional_kwarg(list, "time-modification")?,
            stem: optional_kwarg(list, "stem")?,
            staff: optional_kwarg(list, "staff")?,
            beams: optional_kwarg::<Vec<Beam>>(list, "beams")?.unwrap_or_default(),
            notations: optional_kwarg::<Vec<Notations>>(list, "notations")?.unwrap_or_default(),
            lyrics: optional_kwarg::<Vec<Lyric>>(list, "lyrics")?.unwrap_or_default(),
            notehead: optional_kwarg(list, "notehead")?,
            instrument: optional_kwarg(list, "instrument")?,
            // Add remaining fields from your IR definition
            ..Default::default()  // If Note implements Default
        })
    }
}
```

---

## Task 5: Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::{parse, print};

    #[test]
    fn test_simple_note_round_trip() {
        let note = Note {
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::C,
                        alter: None,
                        octave: Octave(4),
                    }),
                },
                duration: PositiveDivisions(1),
                ties: vec![],
            },
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
                relative_y: None,
                color: None,
            }),
            staff: None,
            beams: vec![],
            notations: vec![],
            lyrics: vec![],
            notehead: None,
            instrument: None,
        };

        let sexpr = note.to_sexpr();
        let text = print(&sexpr);
        println!("Note S-expr:\n{}", text);

        let ast = parse(&text).unwrap();
        let parsed = Note::from_sexpr(&ast).unwrap();

        assert!(matches!(parsed.content, NoteContent::Regular { .. }));
        assert_eq!(parsed.voice, Some("1".to_string()));
    }

    #[test]
    fn test_rest_note() {
        let rest = Note {
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Rest(Rest {
                        display_step: None,
                        display_octave: None,
                        measure: None,
                    }),
                },
                duration: PositiveDivisions(2),
                ties: vec![],
            },
            r#type: Some(NoteType {
                value: NoteTypeValue::Half,
                size: None,
            }),
            ..Default::default()
        };

        let text = print(&rest.to_sexpr());
        println!("Rest:\n{}", text);
        assert!(text.contains("rest"));
    }

    #[test]
    fn test_chord_tone() {
        let chord_tone = Note {
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: true,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::E,
                        alter: None,
                        octave: Octave(4),
                    }),
                },
                duration: PositiveDivisions(1),
                ties: vec![],
            },
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
            ..Default::default()
        };

        let text = print(&chord_tone.to_sexpr());
        println!("Chord tone:\n{}", text);
        assert!(text.contains(":chord t"));
    }

    #[test]
    fn test_grace_note() {
        let grace = Note {
            content: NoteContent::Grace {
                grace: Grace {
                    steal_time_previous: None,
                    steal_time_following: None,
                    make_time: None,
                    slash: Some(YesNo::Yes),
                },
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::D,
                        alter: None,
                        octave: Octave(5),
                    }),
                },
                ties: vec![],
            },
            r#type: Some(NoteType {
                value: NoteTypeValue::Eighth,
                size: None,
            }),
            ..Default::default()
        };

        let text = print(&grace.to_sexpr());
        println!("Grace note:\n{}", text);
        assert!(text.contains("grace-note"));
    }

    #[test]
    fn test_note_with_accidental() {
        let note = Note {
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::F,
                        alter: Some(Semitones(1.0)),
                        octave: Octave(4),
                    }),
                },
                duration: PositiveDivisions(1),
                ties: vec![],
            },
            accidental: Some(Accidental {
                value: AccidentalValue::Sharp,
                cautionary: Some(YesNo::Yes),
                editorial: None,
                parentheses: None,
                bracket: None,
                size: None,
            }),
            r#type: Some(NoteType {
                value: NoteTypeValue::Quarter,
                size: None,
            }),
            ..Default::default()
        };

        let text = print(&note.to_sexpr());
        println!("F# with accidental:\n{}", text);
        assert!(text.contains("accidental"));
        assert!(text.contains("sharp"));
    }

    #[test]
    fn test_tied_note() {
        let note = Note {
            content: NoteContent::Regular {
                full_note: FullNote {
                    chord: false,
                    content: PitchRestUnpitched::Pitch(Pitch {
                        step: Step::G,
                        alter: None,
                        octave: Octave(4),
                    }),
                },
                duration: PositiveDivisions(4),
                ties: vec![
                    Tie {
                        r#type: StartStop::Start,
                        time_only: None,
                    }
                ],
            },
            r#type: Some(NoteType {
                value: NoteTypeValue::Whole,
                size: None,
            }),
            ..Default::default()
        };

        let text = print(&note.to_sexpr());
        println!("Tied note:\n{}", text);
        assert!(text.contains("ties"));
    }
}
```

---

## Example Output

A complete quarter note C4:

```lisp
(note
  :content (regular
    :full-note (full-note :content (pitch :step C :octave 4))
    :duration 1)
  :voice "1"
  :type (note-type :value quarter)
  :stem (stem :value up))
```

A chord (C major):

```lisp
;; First note (root)
(note
  :content (regular
    :full-note (full-note :content (pitch :step C :octave 4))
    :duration 1)
  :type (note-type :value quarter))

;; Second note (third) - has :chord t
(note
  :content (regular
    :full-note (full-note :chord t :content (pitch :step E :octave 4))
    :duration 1)
  :type (note-type :value quarter))

;; Third note (fifth) - has :chord t
(note
  :content (regular
    :full-note (full-note :chord t :content (pitch :step G :octave 4))
    :duration 1)
  :type (note-type :value quarter))
```

---

## Acceptance Criteria

1. ✅ Rest, FullNote, Tie, Grace implement both traits
2. ✅ NoteContent (Regular/Grace/Cue) implements both traits
3. ✅ Accidental and AccidentalValue implement both traits
4. ✅ Complete Note struct implements both traits
5. ✅ Round-trip tests pass for all note variants
6. ✅ Output is readable and matches expected format

---

## Notes for Implementation

1. **Match actual IR structure** — Field names may differ; check `src/ir/note.rs`
2. **Default trait** — Add `#[derive(Default)]` to Note if not present
3. **Notations/Lyrics** — These are Vec types; leave empty until Milestone 3
4. **Grace note head** — Use `"grace-note"` to distinguish from the `grace` kwarg

---

*Next: Milestone 3 — Attributes, Direction, Notation*
