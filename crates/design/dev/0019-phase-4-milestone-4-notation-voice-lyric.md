# Phase 4 — Milestone 4: Notation, Voice, Lyric

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** notation.rs, voice.rs, lyric.rs
> **Depends On:** Milestones 1-3
> **Estimated Implementation Time:** 2-3 hours

---

## Overview

Implement S-expr serialization for:

- **notation.rs** — Articulations, Ornaments, Slur, Tied, Tuplet, Notations
- **voice.rs** — Backup, Forward
- **lyric.rs** — Lyric, Syllabic, TextElementData

---

## File: `src/sexpr/convert/notation.rs`

### Articulations

```rust
// Individual articulations are typically empty-placement types
impl ToSexpr for Staccato {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("staccato")
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("print-style", &self.print_style)
            .build()
    }
}

// Repeat pattern for: Accent, Tenuto, StrongAccent, DetachedLegato, etc.

impl ToSexpr for Articulations {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("articulations");
        // Each articulation type gets its own list
        if !self.accents.is_empty() {
            builder = builder.kwarg_list("accents", &self.accents);
        }
        if !self.staccatos.is_empty() {
            builder = builder.kwarg_list("staccatos", &self.staccatos);
        }
        if !self.tenutos.is_empty() {
            builder = builder.kwarg_list("tenutos", &self.tenutos);
        }
        // ... other articulation types
        builder.build()
    }
}
```

### Ornaments

```rust
impl ToSexpr for TrillMark {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("trill-mark")
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("print-style", &self.print_style)
            .build()
    }
}

impl ToSexpr for Mordent {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("mordent")
            .kwarg_opt("long", &self.long)
            .kwarg_opt("approach", &self.approach)
            .kwarg_opt("departure", &self.departure)
            .kwarg_opt("placement", &self.placement)
            .build()
    }
}

impl ToSexpr for Ornaments {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("ornaments");
        if !self.trill_marks.is_empty() {
            builder = builder.kwarg_list("trill-marks", &self.trill_marks);
        }
        if !self.mordents.is_empty() {
            builder = builder.kwarg_list("mordents", &self.mordents);
        }
        if !self.turns.is_empty() {
            builder = builder.kwarg_list("turns", &self.turns);
        }
        // ... other ornament types
        builder.build()
    }
}
```

### Slur and Tied

```rust
impl ToSexpr for Slur {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("slur")
            .kwarg("type", &self.r#type)  // start/stop/continue
            .kwarg_opt("number", &self.number)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("orientation", &self.orientation)
            .kwarg_opt("line-type", &self.line_type)
            .kwarg_opt("bezier", &self.bezier)
            .build()
    }
}

impl FromSexpr for Slur {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("slur list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("slur") {
            return Err(ConvertError::ExpectedHead("slur"));
        }

        Ok(Slur {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            placement: optional_kwarg(list, "placement")?,
            orientation: optional_kwarg(list, "orientation")?,
            line_type: optional_kwarg(list, "line-type")?,
            bezier: optional_kwarg(list, "bezier")?,
            ..Default::default()
        })
    }
}

impl ToSexpr for Tied {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("tied")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("orientation", &self.orientation)
            .kwarg_opt("line-type", &self.line_type)
            .build()
    }
}

impl FromSexpr for Tied {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tied list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("tied") {
            return Err(ConvertError::ExpectedHead("tied"));
        }

        Ok(Tied {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            orientation: optional_kwarg(list, "orientation")?,
            line_type: optional_kwarg(list, "line-type")?,
            ..Default::default()
        })
    }
}
```

### Tuplet

```rust
impl ToSexpr for TupletType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TupletType::Start => "start",
            TupletType::Stop => "stop",
        })
    }
}

impl ToSexpr for ShowTuplet {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            ShowTuplet::Actual => "actual",
            ShowTuplet::Both => "both",
            ShowTuplet::None => "none",
        })
    }
}

impl ToSexpr for Tuplet {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("tuplet")
            .kwarg("type", &self.r#type)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("bracket", &self.bracket)
            .kwarg_opt("show-number", &self.show_number)
            .kwarg_opt("show-type", &self.show_type)
            .kwarg_opt("line-shape", &self.line_shape)
            .kwarg_opt("placement", &self.placement)
            .build()
    }
}

impl FromSexpr for Tuplet {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("tuplet list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("tuplet") {
            return Err(ConvertError::ExpectedHead("tuplet"));
        }

        Ok(Tuplet {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            bracket: optional_kwarg(list, "bracket")?,
            show_number: optional_kwarg(list, "show-number")?,
            show_type: optional_kwarg(list, "show-type")?,
            line_shape: optional_kwarg(list, "line-shape")?,
            placement: optional_kwarg(list, "placement")?,
            ..Default::default()
        })
    }
}
```

### Fermata

```rust
impl ToSexpr for FermataShape {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            FermataShape::Normal => "normal",
            FermataShape::Angled => "angled",
            FermataShape::Square => "square",
            FermataShape::DoubleAngled => "double-angled",
            FermataShape::DoubleSquare => "double-square",
            FermataShape::DoubleDot => "double-dot",
            FermataShape::HalfCurve => "half-curve",
            FermataShape::Curlew => "curlew",
            FermataShape::Empty => "empty",
        })
    }
}

impl ToSexpr for Fermata {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("fermata")
            .kwarg_opt("shape", &self.shape)
            .kwarg_opt("type", &self.r#type)  // upright/inverted
            .kwarg_opt("print-style", &self.print_style)
            .build()
    }
}
```

### Notations Container

```rust
impl ToSexpr for Notations {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("notations");

        if !self.tieds.is_empty() {
            builder = builder.kwarg_list("tieds", &self.tieds);
        }
        if !self.slurs.is_empty() {
            builder = builder.kwarg_list("slurs", &self.slurs);
        }
        if !self.tuplets.is_empty() {
            builder = builder.kwarg_list("tuplets", &self.tuplets);
        }
        if !self.fermatas.is_empty() {
            builder = builder.kwarg_list("fermatas", &self.fermatas);
        }
        if let Some(ref art) = self.articulations {
            builder = builder.kwarg_raw("articulations", art.to_sexpr());
        }
        if let Some(ref orn) = self.ornaments {
            builder = builder.kwarg_raw("ornaments", orn.to_sexpr());
        }
        if let Some(ref tech) = self.technical {
            builder = builder.kwarg_raw("technical", tech.to_sexpr());
        }
        if let Some(ref dyn_) = self.dynamics {
            builder = builder.kwarg_raw("dynamics", dyn_.to_sexpr());
        }
        if !self.arpeggiate.is_empty() {
            builder = builder.kwarg_list("arpeggiate", &self.arpeggiate);
        }
        if !self.glissandos.is_empty() {
            builder = builder.kwarg_list("glissandos", &self.glissandos);
        }

        builder.build()
    }
}

impl FromSexpr for Notations {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("notations list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("notations") {
            return Err(ConvertError::ExpectedHead("notations"));
        }

        Ok(Notations {
            tieds: optional_kwarg::<Vec<Tied>>(list, "tieds")?.unwrap_or_default(),
            slurs: optional_kwarg::<Vec<Slur>>(list, "slurs")?.unwrap_or_default(),
            tuplets: optional_kwarg::<Vec<Tuplet>>(list, "tuplets")?.unwrap_or_default(),
            fermatas: optional_kwarg::<Vec<Fermata>>(list, "fermatas")?.unwrap_or_default(),
            articulations: optional_kwarg(list, "articulations")?,
            ornaments: optional_kwarg(list, "ornaments")?,
            technical: optional_kwarg(list, "technical")?,
            dynamics: optional_kwarg(list, "dynamics")?,
            arpeggiate: optional_kwarg::<Vec<_>>(list, "arpeggiate")?.unwrap_or_default(),
            glissandos: optional_kwarg::<Vec<_>>(list, "glissandos")?.unwrap_or_default(),
            ..Default::default()
        })
    }
}
```

---

## File: `src/sexpr/convert/voice.rs`

```rust
use crate::ir::voice::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

// ============ Backup ============

impl ToSexpr for Backup {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("backup")
            .kwarg("duration", &self.duration)
            .build()
    }
}

impl FromSexpr for Backup {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("backup list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("backup") {
            return Err(ConvertError::ExpectedHead("backup"));
        }

        Ok(Backup {
            duration: require_kwarg(list, "duration")?,
        })
    }
}

// ============ Forward ============

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
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("forward list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("forward") {
            return Err(ConvertError::ExpectedHead("forward"));
        }

        Ok(Forward {
            duration: require_kwarg(list, "duration")?,
            voice: optional_kwarg(list, "voice")?,
            staff: optional_kwarg(list, "staff")?,
        })
    }
}
```

---

## File: `src/sexpr/convert/lyric.rs`

```rust
use crate::ir::lyric::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

// ============ Syllabic ============

impl ToSexpr for Syllabic {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Syllabic::Single => "single",
            Syllabic::Begin => "begin",
            Syllabic::Middle => "middle",
            Syllabic::End => "end",
        })
    }
}

impl FromSexpr for Syllabic {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("single") => Ok(Syllabic::Single),
            Some("begin") => Ok(Syllabic::Begin),
            Some("middle") => Ok(Syllabic::Middle),
            Some("end") => Ok(Syllabic::End),
            _ => Err(ConvertError::type_mismatch("syllabic", sexpr)),
        }
    }
}

// ============ TextElementData ============

impl ToSexpr for TextElementData {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("text")
            .kwarg("value", &self.value)
            .kwarg_opt("font", &self.font)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

impl FromSexpr for TextElementData {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("text list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("text") {
            return Err(ConvertError::ExpectedHead("text"));
        }

        Ok(TextElementData {
            value: require_kwarg(list, "value")?,
            font: optional_kwarg(list, "font")?,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============ Lyric ============

impl ToSexpr for Lyric {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("lyric");

        if let Some(ref n) = self.number {
            builder = builder.kwarg("number", n);
        }
        if let Some(ref s) = self.syllabic {
            builder = builder.kwarg("syllabic", s);
        }
        if let Some(ref t) = self.text {
            builder = builder.kwarg_raw("text", t.to_sexpr());
        }
        if let Some(ref ext) = self.extend {
            builder = builder.kwarg_raw("extend", ext.to_sexpr());
        }
        if let Some(ref el) = self.elision {
            builder = builder.kwarg_raw("elision", el.to_sexpr());
        }
        if let Some(p) = self.placement {
            builder = builder.kwarg("placement", &p);
        }

        builder.build()
    }
}

impl FromSexpr for Lyric {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("lyric list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("lyric") {
            return Err(ConvertError::ExpectedHead("lyric"));
        }

        Ok(Lyric {
            number: optional_kwarg(list, "number")?,
            syllabic: optional_kwarg(list, "syllabic")?,
            text: optional_kwarg(list, "text")?,
            extend: optional_kwarg(list, "extend")?,
            elision: optional_kwarg(list, "elision")?,
            placement: optional_kwarg(list, "placement")?,
            ..Default::default()
        })
    }
}
```

---

## Example Output

### Notations with slur and articulations

```lisp
(notations
  :slurs ((slur :type start :number 1 :placement above))
  :articulations (articulations :staccatos ((staccato :placement above))))
```

### Lyric

```lisp
(lyric
  :number "1"
  :syllabic begin
  :text (text :value "Hel"))
```

### Backup/Forward

```lisp
(backup :duration 4)
(forward :duration 2 :voice "2")
```

---

## Acceptance Criteria

1. ✅ All notation types (Slur, Tied, Tuplet, Fermata, Articulations, Ornaments)
2. ✅ Notations container properly serializes all sub-elements
3. ✅ Backup and Forward implement both traits
4. ✅ Lyric types implement both traits
5. ✅ Round-trip tests pass

---

*Continue to Milestone 5: Measure, Part, Score (Final)*
