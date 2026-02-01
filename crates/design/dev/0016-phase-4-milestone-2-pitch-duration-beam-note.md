# Phase 4 — Milestone 2: Pitch, Duration, Beam, Note

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** Core musical types: pitch, duration, beam, note
> **Depends On:** Milestone 1 (foundation)
> **Estimated Implementation Time:** 3-4 hours

---

## Overview

This milestone implements S-expr serialization for the core musical types:

- `pitch.rs` — Pitch, Step, Unpitched
- `duration.rs` — NoteType, NoteTypeValue, Dot, TimeModification
- `beam.rs` — Beam, Stem, Notehead
- `note.rs` — Note, NoteContent, FullNote, Rest, Grace, Accidental

---

## Target S-expr Format

```lisp
;; Pitch
(pitch :step C :alter 1 :octave 4)

;; Note type
(note-type :value quarter)

;; Time modification (triplet)
(time-modification :actual-notes 3 :normal-notes 2)

;; Simple quarter note C4
(note
  :content (regular
    :full-note (full-note :content (pitch :step C :octave 4))
    :duration 1)
  :type (note-type :value quarter))
```

---

## Task 1: Pitch Types (`src/sexpr/convert/pitch.rs`)

```rust
use crate::ir::pitch::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

// ============ Step ============

impl ToSexpr for Step {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Step::A => "A", Step::B => "B", Step::C => "C", Step::D => "D",
            Step::E => "E", Step::F => "F", Step::G => "G",
        })
    }
}

impl FromSexpr for Step {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("A") | Some("a") => Ok(Step::A),
            Some("B") | Some("b") => Ok(Step::B),
            Some("C") | Some("c") => Ok(Step::C),
            Some("D") | Some("d") => Ok(Step::D),
            Some("E") | Some("e") => Ok(Step::E),
            Some("F") | Some("f") => Ok(Step::F),
            Some("G") | Some("g") => Ok(Step::G),
            _ => Err(ConvertError::type_mismatch("step (A-G)", sexpr)),
        }
    }
}

// ============ Octave ============
// Assuming: pub struct Octave(pub i8);

impl ToSexpr for Octave {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(self.0.to_string())
    }
}

impl FromSexpr for Octave {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let n = i8::from_sexpr(sexpr)?;
        if !(0..=9).contains(&n) {
            return Err(ConvertError::InvalidValue {
                field: "octave",
                value: n.to_string(),
            });
        }
        Ok(Octave(n))
    }
}

// ============ Semitones ============
// Assuming: pub struct Semitones(pub f32);

impl ToSexpr for Semitones {
    fn to_sexpr(&self) -> Sexpr {
        if self.0.fract() == 0.0 {
            Sexpr::symbol(format!("{}", self.0 as i32))
        } else {
            Sexpr::symbol(format!("{}", self.0))
        }
    }
}

impl FromSexpr for Semitones {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let n = f32::from_sexpr(sexpr)?;
        Ok(Semitones(n))
    }
}

// ============ Pitch ============

impl ToSexpr for Pitch {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("pitch")
            .kwarg("step", &self.step)
            .kwarg_opt("alter", &self.alter)
            .kwarg("octave", &self.octave)
            .build()
    }
}

impl FromSexpr for Pitch {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("pitch list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("pitch") {
            return Err(ConvertError::ExpectedHead("pitch"));
        }

        Ok(Pitch {
            step: require_kwarg(list, "step")?,
            alter: optional_kwarg(list, "alter")?,
            octave: require_kwarg(list, "octave")?,
        })
    }
}

// ============ Unpitched ============

impl ToSexpr for Unpitched {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("unpitched")
            .kwarg_opt("display-step", &self.display_step)
            .kwarg_opt("display-octave", &self.display_octave)
            .build()
    }
}

impl FromSexpr for Unpitched {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("unpitched list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("unpitched") {
            return Err(ConvertError::ExpectedHead("unpitched"));
        }

        Ok(Unpitched {
            display_step: optional_kwarg(list, "display-step")?,
            display_octave: optional_kwarg(list, "display-octave")?,
        })
    }
}
```

---

## Task 2: Duration Types (`src/sexpr/convert/duration.rs`)

```rust
use crate::ir::duration::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

// ============ NoteTypeValue ============

impl ToSexpr for NoteTypeValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            NoteTypeValue::TwoHundredFiftySixth => "256th",
            NoteTypeValue::OneHundredTwentyEighth => "128th",
            NoteTypeValue::SixtyFourth => "64th",
            NoteTypeValue::ThirtySecond => "32nd",
            NoteTypeValue::Sixteenth => "16th",
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
            Some("256th") => Ok(NoteTypeValue::TwoHundredFiftySixth),
            Some("128th") => Ok(NoteTypeValue::OneHundredTwentyEighth),
            Some("64th") => Ok(NoteTypeValue::SixtyFourth),
            Some("32nd") => Ok(NoteTypeValue::ThirtySecond),
            Some("16th") => Ok(NoteTypeValue::Sixteenth),
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

// ============ NoteType ============

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
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("note-type list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("note-type") {
            return Err(ConvertError::ExpectedHead("note-type"));
        }

        Ok(NoteType {
            value: require_kwarg(list, "value")?,
            size: optional_kwarg(list, "size")?,
        })
    }
}

// ============ SymbolSize ============

impl ToSexpr for SymbolSize {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            SymbolSize::Full => "full",
            SymbolSize::Cue => "cue",
            SymbolSize::GraceCue => "grace-cue",
            SymbolSize::Large => "large",
        })
    }
}

impl FromSexpr for SymbolSize {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("full") => Ok(SymbolSize::Full),
            Some("cue") => Ok(SymbolSize::Cue),
            Some("grace-cue") => Ok(SymbolSize::GraceCue),
            Some("large") => Ok(SymbolSize::Large),
            _ => Err(ConvertError::type_mismatch("symbol-size", sexpr)),
        }
    }
}

// ============ Dot ============

impl ToSexpr for Dot {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("dot")
            .kwarg_opt("print-style", &self.print_style)
            .kwarg_opt("placement", &self.placement)
            .build()
    }
}

impl FromSexpr for Dot {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("dot list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("dot") {
            return Err(ConvertError::ExpectedHead("dot"));
        }

        Ok(Dot {
            print_style: optional_kwarg(list, "print-style")?,
            placement: optional_kwarg(list, "placement")?,
        })
    }
}

// ============ TimeModification ============

impl ToSexpr for TimeModification {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("time-modification")
            .kwarg("actual-notes", &self.actual_notes)
            .kwarg("normal-notes", &self.normal_notes)
            .kwarg_opt("normal-type", &self.normal_type)
            .kwarg_opt("normal-dots", &self.normal_dots)
            .build()
    }
}

impl FromSexpr for TimeModification {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time-modification list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("time-modification") {
            return Err(ConvertError::ExpectedHead("time-modification"));
        }

        Ok(TimeModification {
            actual_notes: require_kwarg(list, "actual-notes")?,
            normal_notes: require_kwarg(list, "normal-notes")?,
            normal_type: optional_kwarg(list, "normal-type")?,
            normal_dots: optional_kwarg(list, "normal-dots")?,
        })
    }
}

// ============ PositiveDivisions ============

impl ToSexpr for PositiveDivisions {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(self.0.to_string())
    }
}

impl FromSexpr for PositiveDivisions {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let n = u32::from_sexpr(sexpr)?;
        if n == 0 {
            return Err(ConvertError::InvalidValue {
                field: "positive-divisions",
                value: "0".to_string(),
            });
        }
        Ok(PositiveDivisions(n))
    }
}
```

---

## Task 3: Beam Types (`src/sexpr/convert/beam.rs`)

```rust
use crate::ir::beam::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

// ============ BeamValue ============

impl ToSexpr for BeamValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BeamValue::Begin => "begin",
            BeamValue::Continue => "continue",
            BeamValue::End => "end",
            BeamValue::ForwardHook => "forward-hook",
            BeamValue::BackwardHook => "backward-hook",
        })
    }
}

impl FromSexpr for BeamValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("begin") => Ok(BeamValue::Begin),
            Some("continue") => Ok(BeamValue::Continue),
            Some("end") => Ok(BeamValue::End),
            Some("forward-hook") => Ok(BeamValue::ForwardHook),
            Some("backward-hook") => Ok(BeamValue::BackwardHook),
            _ => Err(ConvertError::type_mismatch("beam-value", sexpr)),
        }
    }
}

// ============ Beam ============

impl ToSexpr for Beam {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("beam")
            .kwarg("number", &self.number)
            .kwarg("value", &self.value)
            .kwarg_opt("repeater", &self.repeater)
            .kwarg_opt("fan", &self.fan)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

impl FromSexpr for Beam {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("beam list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("beam") {
            return Err(ConvertError::ExpectedHead("beam"));
        }

        Ok(Beam {
            number: require_kwarg(list, "number")?,
            value: require_kwarg(list, "value")?,
            repeater: optional_kwarg(list, "repeater")?,
            fan: optional_kwarg(list, "fan")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============ Fan ============

impl ToSexpr for Fan {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Fan::Accel => "accel",
            Fan::Rit => "rit",
            Fan::None => "none",
        })
    }
}

impl FromSexpr for Fan {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("accel") => Ok(Fan::Accel),
            Some("rit") => Ok(Fan::Rit),
            Some("none") => Ok(Fan::None),
            _ => Err(ConvertError::type_mismatch("fan", sexpr)),
        }
    }
}

// ============ Stem ============

impl ToSexpr for StemValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StemValue::Up => "up",
            StemValue::Down => "down",
            StemValue::None => "none",
            StemValue::Double => "double",
        })
    }
}

impl FromSexpr for StemValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("up") => Ok(StemValue::Up),
            Some("down") => Ok(StemValue::Down),
            Some("none") => Ok(StemValue::None),
            Some("double") => Ok(StemValue::Double),
            _ => Err(ConvertError::type_mismatch("stem-value", sexpr)),
        }
    }
}

impl ToSexpr for Stem {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("stem")
            .kwarg("value", &self.value)
            .kwarg_opt("default-y", &self.default_y)
            .kwarg_opt("relative-y", &self.relative_y)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

impl FromSexpr for Stem {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("stem list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("stem") {
            return Err(ConvertError::ExpectedHead("stem"));
        }

        Ok(Stem {
            value: require_kwarg(list, "value")?,
            default_y: optional_kwarg(list, "default-y")?,
            relative_y: optional_kwarg(list, "relative-y")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============ Notehead ============

impl ToSexpr for NoteheadValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            NoteheadValue::Slash => "slash",
            NoteheadValue::Triangle => "triangle",
            NoteheadValue::Diamond => "diamond",
            NoteheadValue::Square => "square",
            NoteheadValue::Cross => "cross",
            NoteheadValue::X => "x",
            NoteheadValue::Normal => "normal",
            NoteheadValue::None => "none",
            NoteheadValue::Other(s) => return Sexpr::string(s),
            // Add other variants as needed
        })
    }
}

impl FromSexpr for NoteheadValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        if let Some(s) = sexpr.as_string() {
            return Ok(NoteheadValue::Other(s.to_string()));
        }
        match sexpr.as_symbol() {
            Some("slash") => Ok(NoteheadValue::Slash),
            Some("triangle") => Ok(NoteheadValue::Triangle),
            Some("diamond") => Ok(NoteheadValue::Diamond),
            Some("square") => Ok(NoteheadValue::Square),
            Some("cross") => Ok(NoteheadValue::Cross),
            Some("x") => Ok(NoteheadValue::X),
            Some("normal") => Ok(NoteheadValue::Normal),
            Some("none") => Ok(NoteheadValue::None),
            _ => Err(ConvertError::type_mismatch("notehead-value", sexpr)),
        }
    }
}

impl ToSexpr for Notehead {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("notehead")
            .kwarg("value", &self.value)
            .kwarg_opt("filled", &self.filled)
            .kwarg_opt("parentheses", &self.parentheses)
            .kwarg_opt("font", &self.font)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

impl FromSexpr for Notehead {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("notehead list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("notehead") {
            return Err(ConvertError::ExpectedHead("notehead"));
        }

        Ok(Notehead {
            value: require_kwarg(list, "value")?,
            filled: optional_kwarg(list, "filled")?,
            parentheses: optional_kwarg(list, "parentheses")?,
            font: optional_kwarg(list, "font")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}
```

---

## Task 4: Note Types (`src/sexpr/convert/note.rs`)

See **Milestone 2B** document for the complete Note implementation.

---

## Acceptance Criteria

1. ✅ Pitch, Step, Octave, Semitones implement traits
2. ✅ NoteType, NoteTypeValue, Dot, TimeModification implement traits
3. ✅ Beam, Stem, Notehead implement traits
4. ✅ Round-trip tests pass

---

*Continue to Milestone 2B for Note types*
