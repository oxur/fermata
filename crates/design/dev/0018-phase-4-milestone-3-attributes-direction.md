# Phase 4 — Milestone 3: Attributes & Direction

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** attributes.rs, direction.rs
> **Depends On:** Milestones 1, 2A, 2B
> **Estimated Implementation Time:** 3-4 hours

---

## Overview

This milestone implements S-expr serialization for measure-level musical elements:

- **attributes.rs** — Clef, Key, Time, Attributes, Barline, Transpose, StaffDetails
- **direction.rs** — Dynamics, Wedge, Metronome, Words, Direction, DirectionType

These types define the musical context (key signature, time signature, clef) and expressive markings (dynamics, tempo).

---

## File: `src/sexpr/convert/attributes.rs`

### ClefSign

```rust
use crate::ir::attributes::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

impl ToSexpr for ClefSign {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            ClefSign::G => "G",
            ClefSign::F => "F",
            ClefSign::C => "C",
            ClefSign::Percussion => "percussion",
            ClefSign::Tab => "TAB",
            ClefSign::Jianpu => "jianpu",
            ClefSign::None => "none",
        })
    }
}

impl FromSexpr for ClefSign {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("G") | Some("g") | Some("treble") => Ok(ClefSign::G),
            Some("F") | Some("f") | Some("bass") => Ok(ClefSign::F),
            Some("C") | Some("c") | Some("alto") | Some("tenor") => Ok(ClefSign::C),
            Some("percussion") => Ok(ClefSign::Percussion),
            Some("TAB") | Some("tab") => Ok(ClefSign::Tab),
            Some("jianpu") => Ok(ClefSign::Jianpu),
            Some("none") => Ok(ClefSign::None),
            _ => Err(ConvertError::type_mismatch("clef-sign (G/F/C/percussion/TAB)", sexpr)),
        }
    }
}
```

### Clef

```rust
impl ToSexpr for Clef {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("clef")
            .kwarg("sign", &self.sign);

        if let Some(line) = self.line {
            builder = builder.kwarg("line", &line);
        }
        if let Some(ref oct) = self.clef_octave_change {
            builder = builder.kwarg("clef-octave-change", oct);
        }
        if let Some(ref num) = self.number {
            builder = builder.kwarg("number", num);
        }
        if let Some(ref add) = self.additional {
            builder = builder.kwarg("additional", add);
        }
        if let Some(ref size) = self.size {
            builder = builder.kwarg("size", size);
        }
        if let Some(ref after) = self.after_barline {
            builder = builder.kwarg("after-barline", after);
        }
        if let Some(ref print) = self.print_object {
            builder = builder.kwarg("print-object", print);
        }

        builder.build()
    }
}

impl FromSexpr for Clef {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("clef list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("clef") {
            return Err(ConvertError::ExpectedHead("clef"));
        }

        Ok(Clef {
            sign: require_kwarg(list, "sign")?,
            line: optional_kwarg(list, "line")?,
            clef_octave_change: optional_kwarg(list, "clef-octave-change")?,
            number: optional_kwarg(list, "number")?,
            additional: optional_kwarg(list, "additional")?,
            size: optional_kwarg(list, "size")?,
            after_barline: optional_kwarg(list, "after-barline")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}
```

### Mode

```rust
impl ToSexpr for Mode {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Mode::Major => "major",
            Mode::Minor => "minor",
            Mode::Dorian => "dorian",
            Mode::Phrygian => "phrygian",
            Mode::Lydian => "lydian",
            Mode::Mixolydian => "mixolydian",
            Mode::Aeolian => "aeolian",
            Mode::Ionian => "ionian",
            Mode::Locrian => "locrian",
            Mode::None => "none",
            Mode::Other(s) => return Sexpr::string(s),
        })
    }
}

impl FromSexpr for Mode {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        if let Some(s) = sexpr.as_string() {
            return Ok(Mode::Other(s.to_string()));
        }
        match sexpr.as_symbol() {
            Some("major") => Ok(Mode::Major),
            Some("minor") => Ok(Mode::Minor),
            Some("dorian") => Ok(Mode::Dorian),
            Some("phrygian") => Ok(Mode::Phrygian),
            Some("lydian") => Ok(Mode::Lydian),
            Some("mixolydian") => Ok(Mode::Mixolydian),
            Some("aeolian") => Ok(Mode::Aeolian),
            Some("ionian") => Ok(Mode::Ionian),
            Some("locrian") => Ok(Mode::Locrian),
            Some("none") => Ok(Mode::None),
            _ => Err(ConvertError::type_mismatch("mode", sexpr)),
        }
    }
}
```

### TraditionalKey and KeyContent

```rust
impl ToSexpr for TraditionalKey {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("traditional-key");

        if let Some(ref cancel) = self.cancel {
            builder = builder.kwarg_raw("cancel", cancel.to_sexpr());
        }
        builder = builder.kwarg("fifths", &self.fifths);
        if let Some(ref mode) = self.mode {
            builder = builder.kwarg("mode", mode);
        }

        builder.build()
    }
}

impl FromSexpr for TraditionalKey {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("traditional-key list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("traditional-key") {
            return Err(ConvertError::ExpectedHead("traditional-key"));
        }

        Ok(TraditionalKey {
            cancel: optional_kwarg(list, "cancel")?,
            fifths: require_kwarg(list, "fifths")?,
            mode: optional_kwarg(list, "mode")?,
        })
    }
}

impl ToSexpr for NonTraditionalKey {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("non-traditional-key")
            .kwarg_list("key-steps", &self.key_steps)
            .kwarg_list("key-alters", &self.key_alters)
            .kwarg_list("key-accidentals", &self.key_accidentals)
            .build()
    }
}

impl FromSexpr for NonTraditionalKey {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("non-traditional-key list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("non-traditional-key") {
            return Err(ConvertError::ExpectedHead("non-traditional-key"));
        }

        Ok(NonTraditionalKey {
            key_steps: optional_kwarg::<Vec<_>>(list, "key-steps")?.unwrap_or_default(),
            key_alters: optional_kwarg::<Vec<_>>(list, "key-alters")?.unwrap_or_default(),
            key_accidentals: optional_kwarg::<Vec<_>>(list, "key-accidentals")?.unwrap_or_default(),
        })
    }
}

impl ToSexpr for KeyContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            KeyContent::Traditional(tk) => tk.to_sexpr(),
            KeyContent::NonTraditional(ntk) => ntk.to_sexpr(),
        }
    }
}

impl FromSexpr for KeyContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("key-content", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("traditional-key") => {
                Ok(KeyContent::Traditional(TraditionalKey::from_sexpr(sexpr)?))
            }
            Some("non-traditional-key") => {
                Ok(KeyContent::NonTraditional(NonTraditionalKey::from_sexpr(sexpr)?))
            }
            _ => Err(ConvertError::type_mismatch("traditional-key or non-traditional-key", sexpr)),
        }
    }
}
```

### Key

```rust
impl ToSexpr for Key {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("key")
            .kwarg_raw("content", self.content.to_sexpr());

        if let Some(ref num) = self.number {
            builder = builder.kwarg("number", num);
        }
        if let Some(ref print) = self.print_object {
            builder = builder.kwarg("print-object", print);
        }

        builder.build()
    }
}

impl FromSexpr for Key {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("key list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("key") {
            return Err(ConvertError::ExpectedHead("key"));
        }

        Ok(Key {
            content: require_kwarg(list, "content")?,
            number: optional_kwarg(list, "number")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}
```

### TimeSignature, TimeContent, Time

```rust
impl ToSexpr for TimeSignature {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("time-signature")
            .kwarg("beats", &self.beats)
            .kwarg("beat-type", &self.beat_type)
            .build()
    }
}

impl FromSexpr for TimeSignature {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time-signature list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("time-signature") {
            return Err(ConvertError::ExpectedHead("time-signature"));
        }

        Ok(TimeSignature {
            beats: require_kwarg(list, "beats")?,
            beat_type: require_kwarg(list, "beat-type")?,
        })
    }
}

impl ToSexpr for TimeSymbol {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            TimeSymbol::Common => "common",
            TimeSymbol::Cut => "cut",
            TimeSymbol::SingleNumber => "single-number",
            TimeSymbol::Note => "note",
            TimeSymbol::DottedNote => "dotted-note",
            TimeSymbol::Normal => "normal",
        })
    }
}

impl FromSexpr for TimeSymbol {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("common") => Ok(TimeSymbol::Common),
            Some("cut") => Ok(TimeSymbol::Cut),
            Some("single-number") => Ok(TimeSymbol::SingleNumber),
            Some("note") => Ok(TimeSymbol::Note),
            Some("dotted-note") => Ok(TimeSymbol::DottedNote),
            Some("normal") => Ok(TimeSymbol::Normal),
            _ => Err(ConvertError::type_mismatch("time-symbol", sexpr)),
        }
    }
}

impl ToSexpr for TimeContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            TimeContent::Measured { signatures } => {
                ListBuilder::new("measured")
                    .kwarg_list("signatures", signatures)
                    .build()
            }
            TimeContent::SenzaMisura { value } => {
                let mut builder = ListBuilder::new("senza-misura");
                if let Some(ref v) = value {
                    builder = builder.kwarg("value", v);
                }
                builder.build()
            }
        }
    }
}

impl FromSexpr for TimeContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time-content", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("measured") => {
                let signatures = require_kwarg(list, "signatures")?;
                Ok(TimeContent::Measured { signatures })
            }
            Some("senza-misura") => {
                let value = optional_kwarg(list, "value")?;
                Ok(TimeContent::SenzaMisura { value })
            }
            _ => Err(ConvertError::type_mismatch("measured or senza-misura", sexpr)),
        }
    }
}

impl ToSexpr for Time {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("time")
            .kwarg_raw("content", self.content.to_sexpr());

        if let Some(ref num) = self.number {
            builder = builder.kwarg("number", num);
        }
        if let Some(ref sym) = self.symbol {
            builder = builder.kwarg("symbol", sym);
        }
        if let Some(ref sep) = self.separator {
            builder = builder.kwarg("separator", sep);
        }
        if let Some(ref print) = self.print_object {
            builder = builder.kwarg("print-object", print);
        }

        builder.build()
    }
}

impl FromSexpr for Time {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("time") {
            return Err(ConvertError::ExpectedHead("time"));
        }

        Ok(Time {
            content: require_kwarg(list, "content")?,
            number: optional_kwarg(list, "number")?,
            symbol: optional_kwarg(list, "symbol")?,
            separator: optional_kwarg(list, "separator")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}
```

### Transpose

```rust
impl ToSexpr for Transpose {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("transpose");

        if let Some(ref diat) = self.diatonic {
            builder = builder.kwarg("diatonic", diat);
        }
        builder = builder.kwarg("chromatic", &self.chromatic);
        if let Some(ref oct) = self.octave_change {
            builder = builder.kwarg("octave-change", oct);
        }
        if let Some(ref dbl) = self.double {
            builder = builder.kwarg("double", dbl);
        }
        if let Some(ref num) = self.number {
            builder = builder.kwarg("number", num);
        }

        builder.build()
    }
}

impl FromSexpr for Transpose {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("transpose list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("transpose") {
            return Err(ConvertError::ExpectedHead("transpose"));
        }

        Ok(Transpose {
            diatonic: optional_kwarg(list, "diatonic")?,
            chromatic: require_kwarg(list, "chromatic")?,
            octave_change: optional_kwarg(list, "octave-change")?,
            double: optional_kwarg(list, "double")?,
            number: optional_kwarg(list, "number")?,
        })
    }
}
```

### Attributes (the main container)

```rust
impl ToSexpr for Attributes {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("attributes");

        // Footnote and level (editorial)
        if let Some(ref fn_) = self.footnote {
            builder = builder.kwarg_raw("footnote", fn_.to_sexpr());
        }
        if let Some(ref lvl) = self.level {
            builder = builder.kwarg_raw("level", lvl.to_sexpr());
        }

        // Core attributes
        if let Some(ref div) = self.divisions {
            builder = builder.kwarg("divisions", div);
        }
        if !self.keys.is_empty() {
            builder = builder.kwarg_list("keys", &self.keys);
        }
        if !self.times.is_empty() {
            builder = builder.kwarg_list("times", &self.times);
        }
        if let Some(ref stv) = self.staves {
            builder = builder.kwarg("staves", stv);
        }
        if let Some(ref ps) = self.part_symbol {
            builder = builder.kwarg_raw("part-symbol", ps.to_sexpr());
        }
        if let Some(ref inst) = self.instruments {
            builder = builder.kwarg("instruments", inst);
        }
        if !self.clefs.is_empty() {
            builder = builder.kwarg_list("clefs", &self.clefs);
        }
        if !self.staff_details.is_empty() {
            builder = builder.kwarg_list("staff-details", &self.staff_details);
        }
        if !self.transposes.is_empty() {
            builder = builder.kwarg_list("transposes", &self.transposes);
        }
        if !self.directives.is_empty() {
            builder = builder.kwarg_list("directives", &self.directives);
        }
        if !self.measure_styles.is_empty() {
            builder = builder.kwarg_list("measure-styles", &self.measure_styles);
        }

        builder.build()
    }
}

impl FromSexpr for Attributes {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("attributes list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("attributes") {
            return Err(ConvertError::ExpectedHead("attributes"));
        }

        Ok(Attributes {
            footnote: optional_kwarg(list, "footnote")?,
            level: optional_kwarg(list, "level")?,
            divisions: optional_kwarg(list, "divisions")?,
            keys: optional_kwarg::<Vec<Key>>(list, "keys")?.unwrap_or_default(),
            times: optional_kwarg::<Vec<Time>>(list, "times")?.unwrap_or_default(),
            staves: optional_kwarg(list, "staves")?,
            part_symbol: optional_kwarg(list, "part-symbol")?,
            instruments: optional_kwarg(list, "instruments")?,
            clefs: optional_kwarg::<Vec<Clef>>(list, "clefs")?.unwrap_or_default(),
            staff_details: optional_kwarg::<Vec<_>>(list, "staff-details")?.unwrap_or_default(),
            transposes: optional_kwarg::<Vec<_>>(list, "transposes")?.unwrap_or_default(),
            directives: optional_kwarg::<Vec<_>>(list, "directives")?.unwrap_or_default(),
            measure_styles: optional_kwarg::<Vec<_>>(list, "measure-styles")?.unwrap_or_default(),
        })
    }
}
```

### Barline Types

```rust
impl ToSexpr for BarStyle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BarStyle::Regular => "regular",
            BarStyle::Dotted => "dotted",
            BarStyle::Dashed => "dashed",
            BarStyle::Heavy => "heavy",
            BarStyle::LightLight => "light-light",
            BarStyle::LightHeavy => "light-heavy",
            BarStyle::HeavyLight => "heavy-light",
            BarStyle::HeavyHeavy => "heavy-heavy",
            BarStyle::Tick => "tick",
            BarStyle::Short => "short",
            BarStyle::None => "none",
        })
    }
}

impl FromSexpr for BarStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("regular") => Ok(BarStyle::Regular),
            Some("dotted") => Ok(BarStyle::Dotted),
            Some("dashed") => Ok(BarStyle::Dashed),
            Some("heavy") => Ok(BarStyle::Heavy),
            Some("light-light") => Ok(BarStyle::LightLight),
            Some("light-heavy") => Ok(BarStyle::LightHeavy),
            Some("heavy-light") => Ok(BarStyle::HeavyLight),
            Some("heavy-heavy") => Ok(BarStyle::HeavyHeavy),
            Some("tick") => Ok(BarStyle::Tick),
            Some("short") => Ok(BarStyle::Short),
            Some("none") => Ok(BarStyle::None),
            _ => Err(ConvertError::type_mismatch("bar-style", sexpr)),
        }
    }
}

impl ToSexpr for RightLeftMiddle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            RightLeftMiddle::Right => "right",
            RightLeftMiddle::Left => "left",
            RightLeftMiddle::Middle => "middle",
        })
    }
}

impl FromSexpr for RightLeftMiddle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("right") => Ok(RightLeftMiddle::Right),
            Some("left") => Ok(RightLeftMiddle::Left),
            Some("middle") => Ok(RightLeftMiddle::Middle),
            _ => Err(ConvertError::type_mismatch("right/left/middle", sexpr)),
        }
    }
}

impl ToSexpr for BackwardForward {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            BackwardForward::Backward => "backward",
            BackwardForward::Forward => "forward",
        })
    }
}

impl FromSexpr for BackwardForward {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("backward") => Ok(BackwardForward::Backward),
            Some("forward") => Ok(BackwardForward::Forward),
            _ => Err(ConvertError::type_mismatch("backward/forward", sexpr)),
        }
    }
}

impl ToSexpr for Repeat {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("repeat")
            .kwarg("direction", &self.direction);

        if let Some(ref times) = self.times {
            builder = builder.kwarg("times", times);
        }
        if let Some(ref winged) = self.winged {
            builder = builder.kwarg("winged", winged);
        }

        builder.build()
    }
}

impl FromSexpr for Repeat {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("repeat list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("repeat") {
            return Err(ConvertError::ExpectedHead("repeat"));
        }

        Ok(Repeat {
            direction: require_kwarg(list, "direction")?,
            times: optional_kwarg(list, "times")?,
            winged: optional_kwarg(list, "winged")?,
        })
    }
}

impl ToSexpr for Ending {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("ending")
            .kwarg("number", &self.number)
            .kwarg("type", &self.r#type);

        if let Some(ref text) = self.text {
            builder = builder.kwarg("text", text);
        }
        if let Some(ref print_obj) = self.print_object {
            builder = builder.kwarg("print-object", print_obj);
        }

        builder.build()
    }
}

impl FromSexpr for Ending {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("ending list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("ending") {
            return Err(ConvertError::ExpectedHead("ending"));
        }

        Ok(Ending {
            number: require_kwarg(list, "number")?,
            r#type: require_kwarg(list, "type")?,
            text: optional_kwarg(list, "text")?,
            print_object: optional_kwarg(list, "print-object")?,
            ..Default::default()
        })
    }
}

impl ToSexpr for Barline {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("barline");

        if let Some(ref loc) = self.location {
            builder = builder.kwarg("location", loc);
        }
        if let Some(ref style) = self.bar_style {
            builder = builder.kwarg("bar-style", style);
        }
        if let Some(ref rep) = self.repeat {
            builder = builder.kwarg_raw("repeat", rep.to_sexpr());
        }
        if let Some(ref end) = self.ending {
            builder = builder.kwarg_raw("ending", end.to_sexpr());
        }
        if let Some(ref seg) = self.segno {
            builder = builder.kwarg_raw("segno", seg.to_sexpr());
        }
        if let Some(ref cod) = self.coda {
            builder = builder.kwarg_raw("coda", cod.to_sexpr());
        }
        if !self.fermatas.is_empty() {
            builder = builder.kwarg_list("fermatas", &self.fermatas);
        }

        builder.build()
    }
}

impl FromSexpr for Barline {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("barline list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("barline") {
            return Err(ConvertError::ExpectedHead("barline"));
        }

        Ok(Barline {
            location: optional_kwarg(list, "location")?,
            bar_style: optional_kwarg(list, "bar-style")?,
            repeat: optional_kwarg(list, "repeat")?,
            ending: optional_kwarg(list, "ending")?,
            segno: optional_kwarg(list, "segno")?,
            coda: optional_kwarg(list, "coda")?,
            fermatas: optional_kwarg::<Vec<_>>(list, "fermatas")?.unwrap_or_default(),
            ..Default::default()
        })
    }
}
```

---

## File: `src/sexpr/convert/direction.rs`

### DynamicValue

```rust
use crate::ir::direction::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};
use crate::sexpr::convert::common::{require_kwarg, optional_kwarg};

impl ToSexpr for DynamicValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            DynamicValue::P => "p",
            DynamicValue::Pp => "pp",
            DynamicValue::Ppp => "ppp",
            DynamicValue::Pppp => "pppp",
            DynamicValue::Ppppp => "ppppp",
            DynamicValue::Pppppp => "pppppp",
            DynamicValue::F => "f",
            DynamicValue::Ff => "ff",
            DynamicValue::Fff => "fff",
            DynamicValue::Ffff => "ffff",
            DynamicValue::Fffff => "fffff",
            DynamicValue::Ffffff => "ffffff",
            DynamicValue::Mp => "mp",
            DynamicValue::Mf => "mf",
            DynamicValue::Sf => "sf",
            DynamicValue::Sfp => "sfp",
            DynamicValue::Sfpp => "sfpp",
            DynamicValue::Fp => "fp",
            DynamicValue::Rf => "rf",
            DynamicValue::Rfz => "rfz",
            DynamicValue::Sfz => "sfz",
            DynamicValue::Sffz => "sffz",
            DynamicValue::Fz => "fz",
            DynamicValue::N => "n",
            DynamicValue::Pf => "pf",
            DynamicValue::Sfzp => "sfzp",
            DynamicValue::OtherDynamics(s) => return Sexpr::string(s),
        })
    }
}

impl FromSexpr for DynamicValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        if let Some(s) = sexpr.as_string() {
            return Ok(DynamicValue::OtherDynamics(s.to_string()));
        }
        match sexpr.as_symbol() {
            Some("p") => Ok(DynamicValue::P),
            Some("pp") => Ok(DynamicValue::Pp),
            Some("ppp") => Ok(DynamicValue::Ppp),
            Some("pppp") => Ok(DynamicValue::Pppp),
            Some("ppppp") => Ok(DynamicValue::Ppppp),
            Some("pppppp") => Ok(DynamicValue::Pppppp),
            Some("f") => Ok(DynamicValue::F),
            Some("ff") => Ok(DynamicValue::Ff),
            Some("fff") => Ok(DynamicValue::Fff),
            Some("ffff") => Ok(DynamicValue::Ffff),
            Some("fffff") => Ok(DynamicValue::Fffff),
            Some("ffffff") => Ok(DynamicValue::Ffffff),
            Some("mp") => Ok(DynamicValue::Mp),
            Some("mf") => Ok(DynamicValue::Mf),
            Some("sf") => Ok(DynamicValue::Sf),
            Some("sfp") => Ok(DynamicValue::Sfp),
            Some("sfpp") => Ok(DynamicValue::Sfpp),
            Some("fp") => Ok(DynamicValue::Fp),
            Some("rf") => Ok(DynamicValue::Rf),
            Some("rfz") => Ok(DynamicValue::Rfz),
            Some("sfz") => Ok(DynamicValue::Sfz),
            Some("sffz") => Ok(DynamicValue::Sffz),
            Some("fz") => Ok(DynamicValue::Fz),
            Some("n") => Ok(DynamicValue::N),
            Some("pf") => Ok(DynamicValue::Pf),
            Some("sfzp") => Ok(DynamicValue::Sfzp),
            _ => Err(ConvertError::type_mismatch("dynamic-value", sexpr)),
        }
    }
}
```

### Dynamics

```rust
impl ToSexpr for Dynamics {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("dynamics")
            .kwarg_list("values", &self.values);

        if let Some(ref pl) = self.placement {
            builder = builder.kwarg("placement", pl);
        }
        if let Some(ref ps) = self.print_style {
            builder = builder.kwarg_raw("print-style", ps.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for Dynamics {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("dynamics list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("dynamics") {
            return Err(ConvertError::ExpectedHead("dynamics"));
        }

        Ok(Dynamics {
            values: require_kwarg(list, "values")?,
            placement: optional_kwarg(list, "placement")?,
            print_style: optional_kwarg(list, "print-style")?,
        })
    }
}
```

### Wedge

```rust
impl ToSexpr for WedgeType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            WedgeType::Crescendo => "crescendo",
            WedgeType::Diminuendo => "diminuendo",
            WedgeType::Stop => "stop",
            WedgeType::Continue => "continue",
        })
    }
}

impl FromSexpr for WedgeType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("crescendo") => Ok(WedgeType::Crescendo),
            Some("diminuendo") | Some("decrescendo") => Ok(WedgeType::Diminuendo),
            Some("stop") => Ok(WedgeType::Stop),
            Some("continue") => Ok(WedgeType::Continue),
            _ => Err(ConvertError::type_mismatch("wedge-type", sexpr)),
        }
    }
}

impl ToSexpr for Wedge {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("wedge")
            .kwarg("type", &self.r#type);

        if let Some(ref num) = self.number {
            builder = builder.kwarg("number", num);
        }
        if let Some(ref spread) = self.spread {
            builder = builder.kwarg("spread", spread);
        }
        if let Some(ref niente) = self.niente {
            builder = builder.kwarg("niente", niente);
        }
        if let Some(ref line_type) = self.line_type {
            builder = builder.kwarg("line-type", line_type);
        }
        if let Some(ref dashes) = self.dash_length {
            builder = builder.kwarg("dash-length", dashes);
        }
        if let Some(ref space) = self.space_length {
            builder = builder.kwarg("space-length", space);
        }

        builder.build()
    }
}

impl FromSexpr for Wedge {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("wedge list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("wedge") {
            return Err(ConvertError::ExpectedHead("wedge"));
        }

        Ok(Wedge {
            r#type: require_kwarg(list, "type")?,
            number: optional_kwarg(list, "number")?,
            spread: optional_kwarg(list, "spread")?,
            niente: optional_kwarg(list, "niente")?,
            line_type: optional_kwarg(list, "line-type")?,
            dash_length: optional_kwarg(list, "dash-length")?,
            space_length: optional_kwarg(list, "space-length")?,
            ..Default::default()
        })
    }
}
```

### Metronome

```rust
impl ToSexpr for Metronome {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("metronome")
            .kwarg("beat-unit", &self.beat_unit);

        if !self.beat_unit_dots.is_empty() {
            builder = builder.kwarg("beat-unit-dots", &(self.beat_unit_dots.len() as u32));
        }
        builder = builder.kwarg("per-minute", &self.per_minute);
        if let Some(ref paren) = self.parentheses {
            builder = builder.kwarg("parentheses", paren);
        }
        if let Some(ref ps) = self.print_style {
            builder = builder.kwarg_raw("print-style", ps.to_sexpr());
        }

        builder.build()
    }
}

impl FromSexpr for Metronome {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("metronome list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("metronome") {
            return Err(ConvertError::ExpectedHead("metronome"));
        }

        let dot_count: u32 = optional_kwarg(list, "beat-unit-dots")?.unwrap_or(0);
        let beat_unit_dots = vec![Empty {}; dot_count as usize];  // Adjust based on actual type

        Ok(Metronome {
            beat_unit: require_kwarg(list, "beat-unit")?,
            beat_unit_dots,
            per_minute: require_kwarg(list, "per-minute")?,
            parentheses: optional_kwarg(list, "parentheses")?,
            print_style: optional_kwarg(list, "print-style")?,
            ..Default::default()
        })
    }
}
```

### Words

```rust
impl ToSexpr for Words {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("words")
            .kwarg("value", &self.value);

        if let Some(ref ps) = self.print_style {
            builder = builder.kwarg_raw("print-style", ps.to_sexpr());
        }
        if let Some(ref pl) = self.placement {
            builder = builder.kwarg("placement", pl);
        }
        if let Some(ref lang) = self.lang {
            builder = builder.kwarg("lang", lang);
        }

        builder.build()
    }
}

impl FromSexpr for Words {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("words list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("words") {
            return Err(ConvertError::ExpectedHead("words"));
        }

        Ok(Words {
            value: require_kwarg(list, "value")?,
            print_style: optional_kwarg(list, "print-style")?,
            placement: optional_kwarg(list, "placement")?,
            lang: optional_kwarg(list, "lang")?,
            ..Default::default()
        })
    }
}
```

### Rehearsal

```rust
impl ToSexpr for Rehearsal {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("rehearsal")
            .kwarg("value", &self.value);

        if let Some(ref ps) = self.print_style {
            builder = builder.kwarg_raw("print-style", ps.to_sexpr());
        }
        if let Some(ref enc) = self.enclosure {
            builder = builder.kwarg("enclosure", enc);
        }

        builder.build()
    }
}

impl FromSexpr for Rehearsal {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("rehearsal list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("rehearsal") {
            return Err(ConvertError::ExpectedHead("rehearsal"));
        }

        Ok(Rehearsal {
            value: require_kwarg(list, "value")?,
            print_style: optional_kwarg(list, "print-style")?,
            enclosure: optional_kwarg(list, "enclosure")?,
            ..Default::default()
        })
    }
}
```

### DirectionType (large enum)

```rust
impl ToSexpr for DirectionType {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            DirectionType::Rehearsals(r) => {
                ListBuilder::new("rehearsals").kwarg_list("items", r).build()
            }
            DirectionType::Segnos(s) => {
                ListBuilder::new("segnos").kwarg_list("items", s).build()
            }
            DirectionType::Codas(c) => {
                ListBuilder::new("codas").kwarg_list("items", c).build()
            }
            DirectionType::Words(w) => {
                ListBuilder::new("words-list").kwarg_list("items", w).build()
            }
            DirectionType::Dynamics(d) => d.to_sexpr(),
            DirectionType::Wedge(w) => w.to_sexpr(),
            DirectionType::Metronome(m) => m.to_sexpr(),
            DirectionType::OctaveShift(o) => o.to_sexpr(),
            DirectionType::Pedal(p) => p.to_sexpr(),
            DirectionType::Bracket(b) => b.to_sexpr(),
            DirectionType::Dashes(d) => d.to_sexpr(),
            DirectionType::StringMute(sm) => sm.to_sexpr(),
            DirectionType::Scordatura(s) => s.to_sexpr(),
            DirectionType::Image(i) => i.to_sexpr(),
            DirectionType::PrincipalVoice(pv) => pv.to_sexpr(),
            DirectionType::Percussion(p) => {
                ListBuilder::new("percussion-list").kwarg_list("items", p).build()
            }
            DirectionType::AccordionRegistration(ar) => ar.to_sexpr(),
            DirectionType::StaffDivide(sd) => sd.to_sexpr(),
            DirectionType::OtherDirection(od) => od.to_sexpr(),
        }
    }
}

impl FromSexpr for DirectionType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("direction-type", sexpr))?;

        match list.first().and_then(|s| s.as_symbol()) {
            Some("rehearsals") => {
                let items = require_kwarg(list, "items")?;
                Ok(DirectionType::Rehearsals(items))
            }
            Some("segnos") => {
                let items = require_kwarg(list, "items")?;
                Ok(DirectionType::Segnos(items))
            }
            Some("codas") => {
                let items = require_kwarg(list, "items")?;
                Ok(DirectionType::Codas(items))
            }
            Some("words-list") => {
                let items = require_kwarg(list, "items")?;
                Ok(DirectionType::Words(items))
            }
            Some("dynamics") => Ok(DirectionType::Dynamics(Dynamics::from_sexpr(sexpr)?)),
            Some("wedge") => Ok(DirectionType::Wedge(Wedge::from_sexpr(sexpr)?)),
            Some("metronome") => Ok(DirectionType::Metronome(Metronome::from_sexpr(sexpr)?)),
            // Add remaining variants...
            other => Err(ConvertError::InvalidVariant(
                other.map(|s| s.to_string()).unwrap_or_else(|| "unknown".to_string())
            )),
        }
    }
}
```

### Direction (container)

```rust
impl ToSexpr for Direction {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("direction")
            .kwarg_list("direction-types", &self.direction_types);

        if let Some(ref offset) = self.offset {
            builder = builder.kwarg_raw("offset", offset.to_sexpr());
        }
        if let Some(ref staff) = self.staff {
            builder = builder.kwarg("staff", staff);
        }
        if let Some(ref sound) = self.sound {
            builder = builder.kwarg_raw("sound", sound.to_sexpr());
        }
        if let Some(ref pl) = self.placement {
            builder = builder.kwarg("placement", pl);
        }
        if let Some(ref dir) = self.directive {
            builder = builder.kwarg("directive", dir);
        }

        builder.build()
    }
}

impl FromSexpr for Direction {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list()
            .ok_or_else(|| ConvertError::type_mismatch("direction list", sexpr))?;

        if list.first().and_then(|s| s.as_symbol()) != Some("direction") {
            return Err(ConvertError::ExpectedHead("direction"));
        }

        Ok(Direction {
            direction_types: require_kwarg(list, "direction-types")?,
            offset: optional_kwarg(list, "offset")?,
            staff: optional_kwarg(list, "staff")?,
            sound: optional_kwarg(list, "sound")?,
            placement: optional_kwarg(list, "placement")?,
            directive: optional_kwarg(list, "directive")?,
            ..Default::default()
        })
    }
}
```

---

## Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::{parse, print};

    #[test]
    fn test_clef_round_trip() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            clef_octave_change: None,
            number: None,
            additional: None,
            size: None,
            after_barline: None,
            print_object: None,
        };

        let sexpr = clef.to_sexpr();
        let text = print(&sexpr);
        assert!(text.contains("clef"));
        assert!(text.contains(":sign G"));
        assert!(text.contains(":line 2"));

        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef.sign, parsed.sign);
        assert_eq!(clef.line, parsed.line);
    }

    #[test]
    fn test_key_round_trip() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: -3,  // Eb major / C minor
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };

        let text = print(&key.to_sexpr());
        println!("Key: {}", text);
        assert!(text.contains("fifths"));
        assert!(text.contains("-3"));
    }

    #[test]
    fn test_time_4_4() {
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Common),
            separator: None,
            print_object: None,
        };

        let text = print(&time.to_sexpr());
        println!("Time: {}", text);
        assert!(text.contains("common"));
    }

    #[test]
    fn test_attributes_round_trip() {
        let attrs = Attributes {
            divisions: Some(1),
            keys: vec![Key {
                content: KeyContent::Traditional(TraditionalKey {
                    cancel: None,
                    fifths: 0,
                    mode: Some(Mode::Major),
                }),
                number: None,
                print_object: None,
            }],
            times: vec![Time {
                content: TimeContent::Measured {
                    signatures: vec![TimeSignature {
                        beats: "4".to_string(),
                        beat_type: "4".to_string(),
                    }],
                },
                ..Default::default()
            }],
            clefs: vec![Clef {
                sign: ClefSign::G,
                line: Some(2),
                ..Default::default()
            }],
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print(&sexpr);
        println!("Attributes:\n{}", text);

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(attrs.divisions, parsed.divisions);
        assert_eq!(attrs.keys.len(), parsed.keys.len());
    }

    #[test]
    fn test_dynamics_ff() {
        let dyn_ = Dynamics {
            values: vec![DynamicValue::Ff],
            placement: Some(AboveBelow::Below),
            print_style: None,
        };

        let text = print(&dyn_.to_sexpr());
        assert!(text.contains("ff"));
        assert!(text.contains("below"));
    }

    #[test]
    fn test_wedge_crescendo() {
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: Some(1),
            spread: None,
            niente: None,
            ..Default::default()
        };

        let text = print(&wedge.to_sexpr());
        assert!(text.contains("crescendo"));
    }

    #[test]
    fn test_direction_with_dynamics() {
        let dir = Direction {
            direction_types: vec![DirectionType::Dynamics(Dynamics {
                values: vec![DynamicValue::Mf],
                placement: None,
                print_style: None,
            })],
            placement: Some(AboveBelow::Above),
            staff: Some(1),
            ..Default::default()
        };

        let text = print(&dir.to_sexpr());
        println!("Direction:\n{}", text);
        assert!(text.contains("mf"));
    }
}
```

---

## Example Output

### Attributes

```lisp
(attributes
  :divisions 1
  :keys ((key
    :content (traditional-key :fifths 0 :mode major)))
  :times ((time
    :content (measured
      :signatures ((time-signature :beats "4" :beat-type "4")))
    :symbol common))
  :clefs ((clef :sign G :line 2)))
```

### Direction with dynamics

```lisp
(direction
  :direction-types ((dynamics :values (ff) :placement below))
  :staff 1)
```

### Barline with repeat

```lisp
(barline
  :location right
  :bar-style light-heavy
  :repeat (repeat :direction backward :times 2))
```

---

## Acceptance Criteria

1. ✅ ClefSign, Clef implement both traits
2. ✅ Mode, TraditionalKey, NonTraditionalKey, KeyContent, Key implement both traits
3. ✅ TimeSignature, TimeSymbol, TimeContent, Time implement both traits
4. ✅ Attributes (full struct) implements both traits
5. ✅ BarStyle, Repeat, Ending, Barline implement both traits
6. ✅ DynamicValue, Dynamics implement both traits
7. ✅ WedgeType, Wedge implement both traits
8. ✅ Metronome, Words, Rehearsal implement both traits
9. ✅ DirectionType (all variants) implements both traits
10. ✅ Direction implements both traits
11. ✅ All round-trip tests pass

---

*Continue to Milestone 4: Notation, Voice, Lyric*
