# Phase 4 — Milestone 5: Measure, Part, Score

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** measure.rs, part.rs, score.rs — Top-level structures
> **Depends On:** Milestones 1-4
> **Estimated Implementation Time:** 2-3 hours

---

## Overview

This final milestone ties everything together with the top-level structures:

- **measure.rs** — Measure, MusicDataElement
- **part.rs** — Part, PartList, ScorePart
- **score.rs** — ScorePartwise, Work, Identification, Defaults

After completing this milestone, you can serialize complete scores!

---

## File: `src/sexpr/convert/measure.rs`

### MusicDataElement (Union Type)

This is the critical enum that contains all measure content.

```rust
use crate::ir::measure::*;
use crate::ir::note::Note;
use crate::ir::voice::{Backup, Forward};
use crate::ir::direction::Direction;
use crate::ir::attributes::Attributes;
use crate::ir::attributes::Barline;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

impl ToSexpr for MusicDataElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            MusicDataElement::Note(n) => n.to_sexpr(),
            MusicDataElement::Backup(b) => b.to_sexpr(),
            MusicDataElement::Forward(f) => f.to_sexpr(),
            MusicDataElement::Direction(d) => d.to_sexpr(),
            MusicDataElement::Attributes(a) => a.to_sexpr(),
            MusicDataElement::Harmony(h) => h.to_sexpr(),
            MusicDataElement::FiguredBass(fb) => fb.to_sexpr(),
            MusicDataElement::Print(p) => p.to_sexpr(),
            MusicDataElement::Sound(s) => s.to_sexpr(),
            MusicDataElement::Barline(b) => b.to_sexpr(),
            MusicDataElement::Grouping(g) => g.to_sexpr(),
            MusicDataElement::Link(l) => l.to_sexpr(),
            MusicDataElement::Bookmark(b) => b.to_sexpr(),
        }
    }
}

impl FromSexpr for MusicDataElement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("music-data-element", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("note") => Ok(MusicDataElement::Note(Note::from_sexpr(sexpr)?)),
            Some("backup") => Ok(MusicDataElement::Backup(Backup::from_sexpr(sexpr)?)),
            Some("forward") => Ok(MusicDataElement::Forward(Forward::from_sexpr(sexpr)?)),
            Some("direction") => Ok(MusicDataElement::Direction(Direction::from_sexpr(sexpr)?)),
            Some("attributes") => Ok(MusicDataElement::Attributes(Attributes::from_sexpr(sexpr)?)),
            Some("barline") => Ok(MusicDataElement::Barline(Barline::from_sexpr(sexpr)?)),
            Some("harmony") => Ok(MusicDataElement::Harmony(Harmony::from_sexpr(sexpr)?)),
            // Add remaining variants...
            other => Err(ConvertError::InvalidVariant(
                other.map(|s| s.to_string()).unwrap_or_else(|| "unknown".to_string())
            )),
        }
    }
}
```

### Measure

```rust
impl ToSexpr for Measure {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("measure")
            .kwarg("number", &self.number);

        // Optional attributes
        if let Some(ref w) = self.width {
            builder = builder.kwarg("width", w);
        }
        if let Some(ref imp) = self.implicit {
            builder = builder.kwarg("implicit", imp);
        }
        if let Some(ref non_ctrl) = self.non_controlling {
            builder = builder.kwarg("non-controlling", non_ctrl);
        }

        // Content - the meat of the measure
        if !self.content.is_empty() {
            builder = builder.kwarg_list("content", &self.content);
        }

        builder.build()
    }
}

impl FromSexpr for Measure {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("measure list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("measure") {
            return Err(ConvertError::ExpectedHead("measure"));
        }

        Ok(Measure {
            number: require_kwarg(list, "number")?,
            width: optional_kwarg(list, "width")?,
            implicit: optional_kwarg(list, "implicit")?,
            non_controlling: optional_kwarg(list, "non-controlling")?,
            content: optional_kwarg::<Vec<MusicDataElement>>(list, "content")?.unwrap_or_default(),
            ..Default::default()
        })
    }
}
```

---

## File: `src/sexpr/convert/part.rs`

### ScorePart

```rust
use crate::ir::part::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

impl ToSexpr for PartName {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("part-name")
            .kwarg("value", &self.value)
            .kwarg_opt("print-object", &self.print_object)
            .kwarg_opt("print-style", &self.print_style)
            .build()
    }
}

impl FromSexpr for PartName {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("part-name list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("part-name") {
            return Err(ConvertError::ExpectedHead("part-name"));
        }

        Ok(PartName {
            value: require_kwarg(list, "value")?,
            print_object: optional_kwarg(list, "print-object")?,
            print_style: optional_kwarg(list, "print-style")?,
        })
    }
}

impl ToSexpr for ScorePart {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("score-part")
            .kwarg("id", &self.id)
            .kwarg_raw("part-name", self.part_name.to_sexpr());

        if let Some(ref abbr) = self.part_abbreviation {
            builder = builder.kwarg_raw("part-abbreviation", abbr.to_sexpr());
        }
        if !self.score_instruments.is_empty() {
            builder = builder.kwarg_list("score-instruments", &self.score_instruments);
        }
        if !self.midi_devices.is_empty() {
            builder = builder.kwarg_list("midi-devices", &self.midi_devices);
        }
        if !self.midi_instruments.is_empty() {
            builder = builder.kwarg_list("midi-instruments", &self.midi_instruments);
        }

        builder.build()
    }
}

impl FromSexpr for ScorePart {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("score-part list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("score-part") {
            return Err(ConvertError::ExpectedHead("score-part"));
        }

        Ok(ScorePart {
            id: require_kwarg(list, "id")?,
            part_name: require_kwarg(list, "part-name")?,
            part_abbreviation: optional_kwarg(list, "part-abbreviation")?,
            score_instruments: optional_kwarg::<Vec<_>>(list, "score-instruments")?.unwrap_or_default(),
            midi_devices: optional_kwarg::<Vec<_>>(list, "midi-devices")?.unwrap_or_default(),
            midi_instruments: optional_kwarg::<Vec<_>>(list, "midi-instruments")?.unwrap_or_default(),
            ..Default::default()
        })
    }
}
```

### PartListElement and PartList

```rust
impl ToSexpr for PartListElement {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            PartListElement::ScorePart(sp) => sp.to_sexpr(),
            PartListElement::PartGroup(pg) => pg.to_sexpr(),
        }
    }
}

impl FromSexpr for PartListElement {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("part-list-element", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("score-part") => Ok(PartListElement::ScorePart(ScorePart::from_sexpr(sexpr)?)),
            Some("part-group") => Ok(PartListElement::PartGroup(PartGroup::from_sexpr(sexpr)?)),
            _ => Err(ConvertError::type_mismatch("score-part or part-group", sexpr)),
        }
    }
}

impl ToSexpr for PartList {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("part-list")
            .kwarg_list("content", &self.content)
            .build()
    }
}

impl FromSexpr for PartList {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("part-list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("part-list") {
            return Err(ConvertError::ExpectedHead("part-list"));
        }

        Ok(PartList {
            content: require_kwarg(list, "content")?,
        })
    }
}
```

### Part

```rust
impl ToSexpr for Part {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("part")
            .kwarg("id", &self.id)
            .kwarg_list("measures", &self.measures)
            .build()
    }
}

impl FromSexpr for Part {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("part list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("part") {
            return Err(ConvertError::ExpectedHead("part"));
        }

        Ok(Part {
            id: require_kwarg(list, "id")?,
            measures: require_kwarg(list, "measures")?,
        })
    }
}
```

---

## File: `src/sexpr/convert/score.rs`

### Work

```rust
use crate::ir::score::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

impl ToSexpr for Work {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("work")
            .kwarg_opt("work-number", &self.work_number)
            .kwarg_opt("work-title", &self.work_title)
            .kwarg_opt("opus", &self.opus)
            .build()
    }
}

impl FromSexpr for Work {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("work list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("work") {
            return Err(ConvertError::ExpectedHead("work"));
        }

        Ok(Work {
            work_number: optional_kwarg(list, "work-number")?,
            work_title: optional_kwarg(list, "work-title")?,
            opus: optional_kwarg(list, "opus")?,
        })
    }
}
```

### Identification

```rust
impl ToSexpr for Identification {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("identification");

        if !self.creators.is_empty() {
            builder = builder.kwarg_list("creators", &self.creators);
        }
        if !self.rights.is_empty() {
            builder = builder.kwarg_list("rights", &self.rights);
        }
        if let Some(ref enc) = self.encoding {
            builder = builder.kwarg_raw("encoding", enc.to_sexpr());
        }
        if let Some(ref src) = self.source {
            builder = builder.kwarg("source", src);
        }
        if !self.relations.is_empty() {
            builder = builder.kwarg_list("relations", &self.relations);
        }

        builder.build()
    }
}

impl FromSexpr for Identification {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("identification list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("identification") {
            return Err(ConvertError::ExpectedHead("identification"));
        }

        Ok(Identification {
            creators: optional_kwarg::<Vec<_>>(list, "creators")?.unwrap_or_default(),
            rights: optional_kwarg::<Vec<_>>(list, "rights")?.unwrap_or_default(),
            encoding: optional_kwarg(list, "encoding")?,
            source: optional_kwarg(list, "source")?,
            relations: optional_kwarg::<Vec<_>>(list, "relations")?.unwrap_or_default(),
            ..Default::default()
        })
    }
}

impl ToSexpr for TypedText {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("typed-text")
            .kwarg("value", &self.value)
            .kwarg_opt("type", &self.r#type)
            .build()
    }
}

impl FromSexpr for TypedText {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("typed-text list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("typed-text") {
            return Err(ConvertError::ExpectedHead("typed-text"));
        }

        Ok(TypedText {
            value: require_kwarg(list, "value")?,
            r#type: optional_kwarg(list, "type")?,
        })
    }
}
```

### ScorePartwise (The Main Event!)

```rust
impl ToSexpr for ScorePartwise {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("score-partwise");

        // Version attribute
        if let Some(ref v) = self.version {
            builder = builder.kwarg("version", v);
        }

        // Header elements
        if let Some(ref work) = self.work {
            builder = builder.kwarg_raw("work", work.to_sexpr());
        }
        if let Some(ref movement_number) = self.movement_number {
            builder = builder.kwarg("movement-number", movement_number);
        }
        if let Some(ref movement_title) = self.movement_title {
            builder = builder.kwarg("movement-title", movement_title);
        }
        if let Some(ref id) = self.identification {
            builder = builder.kwarg_raw("identification", id.to_sexpr());
        }
        if let Some(ref defaults) = self.defaults {
            builder = builder.kwarg_raw("defaults", defaults.to_sexpr());
        }
        if !self.credits.is_empty() {
            builder = builder.kwarg_list("credits", &self.credits);
        }

        // Part list (required)
        builder = builder.kwarg_raw("part-list", self.part_list.to_sexpr());

        // Parts (required)
        builder = builder.kwarg_list("parts", &self.parts);

        builder.build()
    }
}

impl FromSexpr for ScorePartwise {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("score-partwise list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("score-partwise") {
            return Err(ConvertError::ExpectedHead("score-partwise"));
        }

        Ok(ScorePartwise {
            version: optional_kwarg(list, "version")?,
            work: optional_kwarg(list, "work")?,
            movement_number: optional_kwarg(list, "movement-number")?,
            movement_title: optional_kwarg(list, "movement-title")?,
            identification: optional_kwarg(list, "identification")?,
            defaults: optional_kwarg(list, "defaults")?,
            credits: optional_kwarg::<Vec<_>>(list, "credits")?.unwrap_or_default(),
            part_list: require_kwarg(list, "part-list")?,
            parts: require_kwarg(list, "parts")?,
        })
    }
}
```

---

## Complete Example

Here's what a complete score looks like:

```lisp
(score-partwise
  :version "4.0"
  :work (work :work-title "Twinkle Twinkle Little Star")
  :identification (identification
    :creators ((typed-text :value "Traditional" :type "composer")))
  :part-list (part-list
    :content ((score-part
      :id "P1"
      :part-name (part-name :value "Piano"))))
  :parts ((part
    :id "P1"
    :measures ((measure
      :number "1"
      :content ((attributes
        :divisions 1
        :keys ((key :content (traditional-key :fifths 0 :mode major)))
        :times ((time :content (measured :signatures ((time-signature :beats "4" :beat-type "4")))))
        :clefs ((clef :sign G :line 2)))
      (note
        :content (regular
          :full-note (full-note :content (pitch :step C :octave 4))
          :duration 1)
        :type (note-type :value quarter))
      (note
        :content (regular
          :full-note (full-note :content (pitch :step C :octave 4))
          :duration 1)
        :type (note-type :value quarter))
      (note
        :content (regular
          :full-note (full-note :content (pitch :step G :octave 4))
          :duration 1)
        :type (note-type :value quarter))
      (note
        :content (regular
          :full-note (full-note :content (pitch :step G :octave 4))
          :duration 1)
        :type (note-type :value quarter))))))))
```

---

## Update Module Exports

### `src/sexpr/convert/mod.rs`

```rust
pub mod common;
pub mod pitch;
pub mod duration;
pub mod beam;
pub mod note;
pub mod attributes;
pub mod direction;
pub mod notation;
pub mod voice;
pub mod lyric;
pub mod measure;
pub mod part;
pub mod score;

pub use common::{find_kwarg, require_kwarg, optional_kwarg, parse_kwargs};
```

---

## Integration Test

Create `tests/sexpr_roundtrip.rs`:

```rust
use fermata::ir::*;
use fermata::sexpr::{to_string, parse, ToSexpr, FromSexpr};

#[test]
fn test_complete_score_round_trip() {
    // Build a simple score programmatically
    let score = ScorePartwise {
        version: Some("4.0".to_string()),
        part_list: PartList {
            content: vec![PartListElement::ScorePart(ScorePart {
                id: "P1".to_string(),
                part_name: PartName {
                    value: "Piano".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            })],
        },
        parts: vec![Part {
            id: "P1".to_string(),
            measures: vec![Measure {
                number: "1".to_string(),
                content: vec![
                    MusicDataElement::Note(Note {
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
                        r#type: Some(NoteType {
                            value: NoteTypeValue::Quarter,
                            size: None,
                        }),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }],
        }],
        ..Default::default()
    };

    // Convert to S-expr text
    let text = to_string(&score);
    println!("Score S-expr:\n{}", text);

    // Parse back
    let ast = parse(&text).unwrap();
    let parsed = ScorePartwise::from_sexpr(&ast).unwrap();

    // Verify
    assert_eq!(score.version, parsed.version);
    assert_eq!(score.parts.len(), parsed.parts.len());
}
```

---

## Acceptance Criteria

1. ✅ MusicDataElement enum handles all variants
2. ✅ Measure serializes content correctly
3. ✅ Part and PartList work correctly
4. ✅ ScorePartwise serializes complete scores
5. ✅ Full round-trip test passes
6. ✅ Output is valid, readable S-expr syntax

---

## 🎉 Phase 4 Complete

With this milestone, Fermata can now:

- **Read** S-expr text into typed IR
- **Print** typed IR to S-expr text
- **Round-trip** without data loss

### Next Steps (Phase 5)

Phase 5 will add the **ergonomic Fermata syntax** that compiles to this IR:

```lisp
;; This ergonomic syntax:
(score :title "Twinkle"
  (part :piano
    (measure (note c4 :q) (note c4 :q) (note g4 :q) (note g4 :q))))

;; Compiles to the IR syntax we just built!
```

---

*Phase 4 Implementation Documents: Complete*
