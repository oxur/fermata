//! S-expression conversions for `ir::attributes` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for measure-level
//! attribute types:
//! - Clef, ClefSign
//! - Key, KeyContent, TraditionalKey, KeyStep, Cancel, CancelLocation, Mode
//! - Time, TimeContent, TimeSignature, TimeSymbol
//! - Transpose, StaffDetails, StaffType, StaffTuning
//! - PartSymbol, GroupSymbolValue
//! - MeasureStyle, MeasureStyleContent
//! - Barline, BarStyle, Repeat, Ending, Winged
//! - Attributes (main container)

use crate::ir::attributes::{
    Attributes, BarStyle, Barline, Cancel, CancelLocation, Clef, ClefSign, Ending,
    GroupSymbolValue, Key, KeyContent, KeyStep, MeasureStyle, MeasureStyleContent, Mode,
    PartSymbol, Repeat, StaffDetails, StaffTuning, StaffType, Time, TimeContent, TimeSignature,
    TimeSymbol, TraditionalKey, Transpose, Winged,
};
use crate::ir::common::{Editorial, Position, PrintStyle};
use crate::ir::notation::{Fermata, FermataShape};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, get_head, optional_kwarg, require_kwarg};

// ============================================================================
// FermataShape
// ============================================================================

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
        })
    }
}

impl FromSexpr for FermataShape {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("normal") => Ok(FermataShape::Normal),
            Some("angled") => Ok(FermataShape::Angled),
            Some("square") => Ok(FermataShape::Square),
            Some("double-angled") => Ok(FermataShape::DoubleAngled),
            Some("double-square") => Ok(FermataShape::DoubleSquare),
            Some("double-dot") => Ok(FermataShape::DoubleDot),
            Some("half-curve") => Ok(FermataShape::HalfCurve),
            Some("curlew") => Ok(FermataShape::Curlew),
            _ => Err(ConvertError::type_mismatch("fermata-shape", sexpr)),
        }
    }
}

// ============================================================================
// Fermata
// ============================================================================

impl ToSexpr for Fermata {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("fermata")
            .kwarg_opt("shape", &self.shape)
            .kwarg_opt("type", &self.r#type);

        // Print style
        let pos = &self.print_style.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }
        if let Some(ref color) = self.print_style.color {
            builder = builder.kwarg("color", color);
        }

        builder.build()
    }
}

impl FromSexpr for Fermata {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("fermata list", sexpr))?;

        expect_head(list, "fermata")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };
        let color = optional_kwarg::<String>(list, "color")?;

        Ok(Fermata {
            shape: optional_kwarg(list, "shape")?,
            r#type: optional_kwarg(list, "type")?,
            print_style: PrintStyle {
                position,
                font: Default::default(),
                color,
            },
        })
    }
}

// ============================================================================
// ClefSign
// ============================================================================

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
            _ => Err(ConvertError::type_mismatch(
                "clef-sign (G/F/C/percussion/TAB)",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Clef
// ============================================================================

impl ToSexpr for Clef {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("clef")
            .kwarg("sign", &self.sign)
            .kwarg_opt("line", &self.line)
            .kwarg_opt("octave-change", &self.octave_change)
            .kwarg_opt("number", &self.number)
            .kwarg_opt("size", &self.size)
            .kwarg_opt("print-object", &self.print_object)
            .build()
    }
}

impl FromSexpr for Clef {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("clef list", sexpr))?;

        expect_head(list, "clef")?;

        Ok(Clef {
            sign: require_kwarg(list, "sign")?,
            line: optional_kwarg(list, "line")?,
            octave_change: optional_kwarg(list, "octave-change")?,
            number: optional_kwarg(list, "number")?,
            size: optional_kwarg(list, "size")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}

// ============================================================================
// Mode
// ============================================================================

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
        })
    }
}

impl FromSexpr for Mode {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
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

// ============================================================================
// CancelLocation
// ============================================================================

impl ToSexpr for CancelLocation {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            CancelLocation::Left => "left",
            CancelLocation::Right => "right",
            CancelLocation::BeforeBarline => "before-barline",
        })
    }
}

impl FromSexpr for CancelLocation {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("left") => Ok(CancelLocation::Left),
            Some("right") => Ok(CancelLocation::Right),
            Some("before-barline") => Ok(CancelLocation::BeforeBarline),
            _ => Err(ConvertError::type_mismatch("cancel-location", sexpr)),
        }
    }
}

// ============================================================================
// Cancel
// ============================================================================

impl ToSexpr for Cancel {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("cancel")
            .kwarg("fifths", &self.fifths)
            .kwarg_opt("location", &self.location)
            .build()
    }
}

impl FromSexpr for Cancel {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("cancel list", sexpr))?;

        expect_head(list, "cancel")?;

        Ok(Cancel {
            fifths: require_kwarg(list, "fifths")?,
            location: optional_kwarg(list, "location")?,
        })
    }
}

// ============================================================================
// TraditionalKey
// ============================================================================

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
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("traditional-key list", sexpr))?;

        expect_head(list, "traditional-key")?;

        Ok(TraditionalKey {
            cancel: optional_kwarg(list, "cancel")?,
            fifths: require_kwarg(list, "fifths")?,
            mode: optional_kwarg(list, "mode")?,
        })
    }
}

// ============================================================================
// KeyStep
// ============================================================================

impl ToSexpr for KeyStep {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("key-step")
            .kwarg("step", &self.step)
            .kwarg("alter", &self.alter)
            .kwarg_opt("accidental", &self.accidental)
            .build()
    }
}

impl FromSexpr for KeyStep {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("key-step list", sexpr))?;

        expect_head(list, "key-step")?;

        Ok(KeyStep {
            step: require_kwarg(list, "step")?,
            alter: require_kwarg(list, "alter")?,
            accidental: optional_kwarg(list, "accidental")?,
        })
    }
}

// ============================================================================
// KeyContent
// ============================================================================

impl ToSexpr for KeyContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            KeyContent::Traditional(tk) => tk.to_sexpr(),
            KeyContent::NonTraditional(steps) => ListBuilder::new("non-traditional-key")
                .kwarg_list("key-steps", steps)
                .build(),
        }
    }
}

impl FromSexpr for KeyContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("key-content", sexpr))?;

        match get_head(list)? {
            "traditional-key" => Ok(KeyContent::Traditional(TraditionalKey::from_sexpr(sexpr)?)),
            "non-traditional-key" => {
                let steps = optional_kwarg::<Vec<KeyStep>>(list, "key-steps")?.unwrap_or_default();
                Ok(KeyContent::NonTraditional(steps))
            }
            _ => Err(ConvertError::type_mismatch(
                "traditional-key or non-traditional-key",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Key
// ============================================================================

impl ToSexpr for Key {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("key")
            .kwarg_raw("content", self.content.to_sexpr())
            .kwarg_opt("number", &self.number)
            .kwarg_opt("print-object", &self.print_object)
            .build()
    }
}

impl FromSexpr for Key {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("key list", sexpr))?;

        expect_head(list, "key")?;

        Ok(Key {
            content: require_kwarg(list, "content")?,
            number: optional_kwarg(list, "number")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}

// ============================================================================
// TimeSymbol
// ============================================================================

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

// ============================================================================
// TimeSignature
// ============================================================================

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
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time-signature list", sexpr))?;

        expect_head(list, "time-signature")?;

        Ok(TimeSignature {
            beats: require_kwarg(list, "beats")?,
            beat_type: require_kwarg(list, "beat-type")?,
        })
    }
}

// ============================================================================
// TimeContent
// ============================================================================

impl ToSexpr for TimeContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            TimeContent::Measured { signatures } => ListBuilder::new("measured")
                .kwarg_list("signatures", signatures)
                .build(),
            TimeContent::SenzaMisura(value) => {
                if value.is_empty() {
                    ListBuilder::new("senza-misura").build()
                } else {
                    ListBuilder::new("senza-misura")
                        .kwarg("value", value)
                        .build()
                }
            }
        }
    }
}

impl FromSexpr for TimeContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time-content", sexpr))?;

        match get_head(list)? {
            "measured" => {
                let signatures =
                    optional_kwarg::<Vec<TimeSignature>>(list, "signatures")?.unwrap_or_default();
                Ok(TimeContent::Measured { signatures })
            }
            "senza-misura" => {
                let value = optional_kwarg::<String>(list, "value")?.unwrap_or_default();
                Ok(TimeContent::SenzaMisura(value))
            }
            _ => Err(ConvertError::type_mismatch(
                "measured or senza-misura",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// Time
// ============================================================================

impl ToSexpr for Time {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("time")
            .kwarg_raw("content", self.content.to_sexpr())
            .kwarg_opt("number", &self.number)
            .kwarg_opt("symbol", &self.symbol)
            .kwarg_opt("print-object", &self.print_object)
            .build()
    }
}

impl FromSexpr for Time {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("time list", sexpr))?;

        expect_head(list, "time")?;

        Ok(Time {
            content: require_kwarg(list, "content")?,
            number: optional_kwarg(list, "number")?,
            symbol: optional_kwarg(list, "symbol")?,
            print_object: optional_kwarg(list, "print-object")?,
        })
    }
}

// ============================================================================
// GroupSymbolValue
// ============================================================================

impl ToSexpr for GroupSymbolValue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            GroupSymbolValue::None => "none",
            GroupSymbolValue::Brace => "brace",
            GroupSymbolValue::Line => "line",
            GroupSymbolValue::Bracket => "bracket",
            GroupSymbolValue::Square => "square",
        })
    }
}

impl FromSexpr for GroupSymbolValue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("none") => Ok(GroupSymbolValue::None),
            Some("brace") => Ok(GroupSymbolValue::Brace),
            Some("line") => Ok(GroupSymbolValue::Line),
            Some("bracket") => Ok(GroupSymbolValue::Bracket),
            Some("square") => Ok(GroupSymbolValue::Square),
            _ => Err(ConvertError::type_mismatch("group-symbol-value", sexpr)),
        }
    }
}

// ============================================================================
// PartSymbol
// ============================================================================

impl ToSexpr for PartSymbol {
    fn to_sexpr(&self) -> Sexpr {
        // Check if position has any content
        let pos_has_content = self.position.default_x.is_some()
            || self.position.default_y.is_some()
            || self.position.relative_x.is_some()
            || self.position.relative_y.is_some();

        let mut builder = ListBuilder::new("part-symbol")
            .kwarg("value", &self.value)
            .kwarg_opt("top-staff", &self.top_staff)
            .kwarg_opt("bottom-staff", &self.bottom_staff);

        if pos_has_content {
            builder = builder.kwarg_raw("position", self.position.to_sexpr());
        }
        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for PartSymbol {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("part-symbol list", sexpr))?;

        expect_head(list, "part-symbol")?;

        use crate::ir::common::Position;

        let position = match find_kwarg(list, "position") {
            Some(pos_sexpr) => Position::from_sexpr(pos_sexpr)?,
            None => Position::default(),
        };

        Ok(PartSymbol {
            value: require_kwarg(list, "value")?,
            top_staff: optional_kwarg(list, "top-staff")?,
            bottom_staff: optional_kwarg(list, "bottom-staff")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// StaffType
// ============================================================================

impl ToSexpr for StaffType {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StaffType::Ossia => "ossia",
            StaffType::Editorial => "editorial",
            StaffType::Cue => "cue",
            StaffType::Regular => "regular",
            StaffType::Alternate => "alternate",
        })
    }
}

impl FromSexpr for StaffType {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("ossia") => Ok(StaffType::Ossia),
            Some("editorial") => Ok(StaffType::Editorial),
            Some("cue") => Ok(StaffType::Cue),
            Some("regular") => Ok(StaffType::Regular),
            Some("alternate") => Ok(StaffType::Alternate),
            _ => Err(ConvertError::type_mismatch("staff-type", sexpr)),
        }
    }
}

// ============================================================================
// StaffTuning
// ============================================================================

impl ToSexpr for StaffTuning {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("staff-tuning")
            .kwarg("line", &self.line)
            .kwarg("tuning-step", &self.tuning_step)
            .kwarg_opt("tuning-alter", &self.tuning_alter)
            .kwarg("tuning-octave", &self.tuning_octave)
            .build()
    }
}

impl FromSexpr for StaffTuning {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("staff-tuning list", sexpr))?;

        expect_head(list, "staff-tuning")?;

        Ok(StaffTuning {
            line: require_kwarg(list, "line")?,
            tuning_step: require_kwarg(list, "tuning-step")?,
            tuning_alter: optional_kwarg(list, "tuning-alter")?,
            tuning_octave: require_kwarg(list, "tuning-octave")?,
        })
    }
}

// ============================================================================
// StaffDetails
// ============================================================================

impl ToSexpr for StaffDetails {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("staff-details")
            .kwarg_opt("number", &self.number)
            .kwarg_opt("staff-type", &self.staff_type)
            .kwarg_opt("staff-lines", &self.staff_lines);

        if !self.staff_tuning.is_empty() {
            builder = builder.kwarg_list("staff-tuning", &self.staff_tuning);
        }
        builder = builder
            .kwarg_opt("capo", &self.capo)
            .kwarg_opt("staff-size", &self.staff_size);

        builder.build()
    }
}

impl FromSexpr for StaffDetails {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("staff-details list", sexpr))?;

        expect_head(list, "staff-details")?;

        Ok(StaffDetails {
            number: optional_kwarg(list, "number")?,
            staff_type: optional_kwarg(list, "staff-type")?,
            staff_lines: optional_kwarg(list, "staff-lines")?,
            staff_tuning: optional_kwarg::<Vec<StaffTuning>>(list, "staff-tuning")?
                .unwrap_or_default(),
            capo: optional_kwarg(list, "capo")?,
            staff_size: optional_kwarg(list, "staff-size")?,
        })
    }
}

// ============================================================================
// Transpose
// ============================================================================

impl ToSexpr for Transpose {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("transpose")
            .kwarg_opt("number", &self.number)
            .kwarg_opt("diatonic", &self.diatonic)
            .kwarg("chromatic", &self.chromatic)
            .kwarg_opt("octave-change", &self.octave_change)
            .kwarg_opt("double", &self.double)
            .build()
    }
}

impl FromSexpr for Transpose {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("transpose list", sexpr))?;

        expect_head(list, "transpose")?;

        Ok(Transpose {
            number: optional_kwarg(list, "number")?,
            diatonic: optional_kwarg(list, "diatonic")?,
            chromatic: require_kwarg(list, "chromatic")?,
            octave_change: optional_kwarg(list, "octave-change")?,
            double: optional_kwarg(list, "double")?,
        })
    }
}

// ============================================================================
// MeasureStyleContent
// ============================================================================

impl ToSexpr for MeasureStyleContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            MeasureStyleContent::MultipleRest { count, use_symbols } => {
                ListBuilder::new("multiple-rest")
                    .kwarg("count", count)
                    .kwarg_opt("use-symbols", use_symbols)
                    .build()
            }
            MeasureStyleContent::MeasureRepeat { r#type, slashes } => {
                ListBuilder::new("measure-repeat")
                    .kwarg("type", r#type)
                    .kwarg_opt("slashes", slashes)
                    .build()
            }
            MeasureStyleContent::BeatRepeat { r#type, slashes } => ListBuilder::new("beat-repeat")
                .kwarg("type", r#type)
                .kwarg_opt("slashes", slashes)
                .build(),
            MeasureStyleContent::Slash { r#type, use_stems } => ListBuilder::new("slash")
                .kwarg("type", r#type)
                .kwarg_opt("use-stems", use_stems)
                .build(),
        }
    }
}

impl FromSexpr for MeasureStyleContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("measure-style-content", sexpr))?;

        match get_head(list)? {
            "multiple-rest" => Ok(MeasureStyleContent::MultipleRest {
                count: require_kwarg(list, "count")?,
                use_symbols: optional_kwarg(list, "use-symbols")?,
            }),
            "measure-repeat" => Ok(MeasureStyleContent::MeasureRepeat {
                r#type: require_kwarg(list, "type")?,
                slashes: optional_kwarg(list, "slashes")?,
            }),
            "beat-repeat" => Ok(MeasureStyleContent::BeatRepeat {
                r#type: require_kwarg(list, "type")?,
                slashes: optional_kwarg(list, "slashes")?,
            }),
            "slash" => Ok(MeasureStyleContent::Slash {
                r#type: require_kwarg(list, "type")?,
                use_stems: optional_kwarg(list, "use-stems")?,
            }),
            _ => Err(ConvertError::type_mismatch(
                "measure-style-content variant",
                sexpr,
            )),
        }
    }
}

// ============================================================================
// MeasureStyle
// ============================================================================

impl ToSexpr for MeasureStyle {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("measure-style")
            .kwarg_opt("number", &self.number)
            .kwarg_raw("content", self.content.to_sexpr())
            .build()
    }
}

impl FromSexpr for MeasureStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("measure-style list", sexpr))?;

        expect_head(list, "measure-style")?;

        Ok(MeasureStyle {
            number: optional_kwarg(list, "number")?,
            content: require_kwarg(list, "content")?,
        })
    }
}

// ============================================================================
// BarStyle
// ============================================================================

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

// ============================================================================
// Winged
// ============================================================================

impl ToSexpr for Winged {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Winged::None => "none",
            Winged::Straight => "straight",
            Winged::Curved => "curved",
            Winged::DoubleStraight => "double-straight",
            Winged::DoubleCurved => "double-curved",
        })
    }
}

impl FromSexpr for Winged {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("none") => Ok(Winged::None),
            Some("straight") => Ok(Winged::Straight),
            Some("curved") => Ok(Winged::Curved),
            Some("double-straight") => Ok(Winged::DoubleStraight),
            Some("double-curved") => Ok(Winged::DoubleCurved),
            _ => Err(ConvertError::type_mismatch("winged", sexpr)),
        }
    }
}

// ============================================================================
// Repeat
// ============================================================================

impl ToSexpr for Repeat {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("repeat")
            .kwarg("direction", &self.direction)
            .kwarg_opt("times", &self.times)
            .kwarg_opt("winged", &self.winged)
            .build()
    }
}

impl FromSexpr for Repeat {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("repeat list", sexpr))?;

        expect_head(list, "repeat")?;

        Ok(Repeat {
            direction: require_kwarg(list, "direction")?,
            times: optional_kwarg(list, "times")?,
            winged: optional_kwarg(list, "winged")?,
        })
    }
}

// ============================================================================
// Ending
// ============================================================================

impl ToSexpr for Ending {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("ending")
            .kwarg("type", &self.r#type)
            .kwarg("number", &self.number)
            .kwarg_opt("text", &self.text)
            .kwarg_opt("print-object", &self.print_object)
            .kwarg_opt("end-length", &self.end_length)
            .kwarg_opt("text-x", &self.text_x)
            .kwarg_opt("text-y", &self.text_y)
            .build()
    }
}

impl FromSexpr for Ending {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("ending list", sexpr))?;

        expect_head(list, "ending")?;

        Ok(Ending {
            r#type: require_kwarg(list, "type")?,
            number: require_kwarg(list, "number")?,
            text: optional_kwarg(list, "text")?,
            print_object: optional_kwarg(list, "print-object")?,
            end_length: optional_kwarg(list, "end-length")?,
            text_x: optional_kwarg(list, "text-x")?,
            text_y: optional_kwarg(list, "text-y")?,
        })
    }
}

// ============================================================================
// Barline
// ============================================================================

impl ToSexpr for Barline {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("barline")
            .kwarg_opt("location", &self.location)
            .kwarg_opt("bar-style", &self.bar_style);

        if let Some(ref repeat) = self.repeat {
            builder = builder.kwarg_raw("repeat", repeat.to_sexpr());
        }
        if let Some(ref ending) = self.ending {
            builder = builder.kwarg_raw("ending", ending.to_sexpr());
        }
        if let Some(ref segno) = self.segno {
            builder = builder.kwarg_raw("segno", segno.to_sexpr());
        }
        if let Some(ref coda) = self.coda {
            builder = builder.kwarg_raw("coda", coda.to_sexpr());
        }
        if !self.fermatas.is_empty() {
            builder = builder.kwarg_list("fermatas", &self.fermatas);
        }

        builder.build()
    }
}

impl FromSexpr for Barline {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("barline list", sexpr))?;

        expect_head(list, "barline")?;

        Ok(Barline {
            location: optional_kwarg(list, "location")?,
            bar_style: optional_kwarg(list, "bar-style")?,
            editorial: Editorial::default(),
            wavy_line: optional_kwarg(list, "wavy-line")?,
            segno: optional_kwarg(list, "segno")?,
            coda: optional_kwarg(list, "coda")?,
            fermatas: optional_kwarg(list, "fermatas")?.unwrap_or_default(),
            ending: optional_kwarg(list, "ending")?,
            repeat: optional_kwarg(list, "repeat")?,
        })
    }
}

// ============================================================================
// Attributes (main container)
// ============================================================================

impl ToSexpr for Attributes {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("attributes");

        // Divisions
        if let Some(ref div) = self.divisions {
            builder = builder.kwarg("divisions", div);
        }
        // Keys
        if !self.keys.is_empty() {
            builder = builder.kwarg_list("keys", &self.keys);
        }
        // Times
        if !self.times.is_empty() {
            builder = builder.kwarg_list("times", &self.times);
        }
        // Staves
        if let Some(ref stv) = self.staves {
            builder = builder.kwarg("staves", stv);
        }
        // Part symbol
        if let Some(ref ps) = self.part_symbol {
            builder = builder.kwarg_raw("part-symbol", ps.to_sexpr());
        }
        // Instruments
        if let Some(ref inst) = self.instruments {
            builder = builder.kwarg("instruments", inst);
        }
        // Clefs
        if !self.clefs.is_empty() {
            builder = builder.kwarg_list("clefs", &self.clefs);
        }
        // Staff details
        if !self.staff_details.is_empty() {
            builder = builder.kwarg_list("staff-details", &self.staff_details);
        }
        // Transpose (singular field name in IR)
        if !self.transpose.is_empty() {
            builder = builder.kwarg_list("transpose", &self.transpose);
        }
        // Measure styles
        if !self.measure_styles.is_empty() {
            builder = builder.kwarg_list("measure-styles", &self.measure_styles);
        }

        builder.build()
    }
}

impl FromSexpr for Attributes {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("attributes list", sexpr))?;

        expect_head(list, "attributes")?;

        Ok(Attributes {
            editorial: Editorial::default(),
            divisions: optional_kwarg(list, "divisions")?,
            keys: optional_kwarg::<Vec<Key>>(list, "keys")?.unwrap_or_default(),
            times: optional_kwarg::<Vec<Time>>(list, "times")?.unwrap_or_default(),
            staves: optional_kwarg(list, "staves")?,
            part_symbol: optional_kwarg(list, "part-symbol")?,
            instruments: optional_kwarg(list, "instruments")?,
            clefs: optional_kwarg::<Vec<Clef>>(list, "clefs")?.unwrap_or_default(),
            staff_details: optional_kwarg::<Vec<StaffDetails>>(list, "staff-details")?
                .unwrap_or_default(),
            transpose: optional_kwarg::<Vec<Transpose>>(list, "transpose")?.unwrap_or_default(),
            measure_styles: optional_kwarg::<Vec<MeasureStyle>>(list, "measure-styles")?
                .unwrap_or_default(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{
        AccidentalValue, BackwardForward, Position, RightLeftMiddle, StartStop,
        StartStopDiscontinue, SymbolSize, UprightInverted, YesNo,
    };
    use crate::ir::pitch::Step;
    use crate::sexpr::print_sexpr;

    // ========================================================================
    // FermataShape Tests
    // ========================================================================

    #[test]
    fn test_fermatashape_round_trip_all_variants() {
        for shape in [
            FermataShape::Normal,
            FermataShape::Angled,
            FermataShape::Square,
            FermataShape::DoubleAngled,
            FermataShape::DoubleSquare,
            FermataShape::DoubleDot,
            FermataShape::HalfCurve,
            FermataShape::Curlew,
        ] {
            let sexpr = shape.to_sexpr();
            let parsed = FermataShape::from_sexpr(&sexpr).unwrap();
            assert_eq!(shape, parsed);
        }
    }

    #[test]
    fn test_fermatashape_from_sexpr_invalid_symbol() {
        let result = FermataShape::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_fermatashape_from_sexpr_not_symbol() {
        let result = FermataShape::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // Fermata Tests
    // ========================================================================

    #[test]
    fn test_fermata_minimal_round_trip() {
        let fermata = Fermata {
            shape: None,
            r#type: None,
            print_style: PrintStyle::default(),
        };

        let sexpr = fermata.to_sexpr();
        let parsed = Fermata::from_sexpr(&sexpr).unwrap();
        assert_eq!(fermata.shape, parsed.shape);
        assert_eq!(fermata.r#type, parsed.r#type);
    }

    #[test]
    fn test_fermata_with_shape_and_type() {
        let fermata = Fermata {
            shape: Some(FermataShape::Angled),
            r#type: Some(UprightInverted::Inverted),
            print_style: PrintStyle::default(),
        };

        let sexpr = fermata.to_sexpr();
        let parsed = Fermata::from_sexpr(&sexpr).unwrap();
        assert_eq!(fermata.shape, parsed.shape);
        assert_eq!(fermata.r#type, parsed.r#type);
    }

    #[test]
    fn test_fermata_with_position() {
        let fermata = Fermata {
            shape: Some(FermataShape::Normal),
            r#type: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: Some(10.0),
                    default_y: Some(20.0),
                    relative_x: None,
                    relative_y: None,
                },
                font: Default::default(),
                color: None,
            },
        };

        let sexpr = fermata.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));

        let parsed = Fermata::from_sexpr(&sexpr).unwrap();
        assert_eq!(
            fermata.print_style.position.default_x,
            parsed.print_style.position.default_x
        );
    }

    #[test]
    fn test_fermata_with_color() {
        let fermata = Fermata {
            shape: None,
            r#type: None,
            print_style: PrintStyle {
                position: Position::default(),
                font: Default::default(),
                color: Some("#FF0000".to_string()),
            },
        };

        let sexpr = fermata.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("color"));

        let parsed = Fermata::from_sexpr(&sexpr).unwrap();
        assert_eq!(fermata.print_style.color, parsed.print_style.color);
    }

    #[test]
    fn test_fermata_from_sexpr_not_list() {
        let result = Fermata::from_sexpr(&Sexpr::symbol("fermata"));
        assert!(result.is_err());
    }

    #[test]
    fn test_fermata_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-fermata")]);
        let result = Fermata::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // ClefSign Tests
    // ========================================================================

    #[test]
    fn test_clefsign_round_trip() {
        for sign in [
            ClefSign::G,
            ClefSign::F,
            ClefSign::C,
            ClefSign::Percussion,
            ClefSign::Tab,
            ClefSign::Jianpu,
            ClefSign::None,
        ] {
            let sexpr = sign.to_sexpr();
            let parsed = ClefSign::from_sexpr(&sexpr).unwrap();
            assert_eq!(sign, parsed);
        }
    }

    #[test]
    fn test_clefsign_aliases_g() {
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("G")).unwrap(),
            ClefSign::G
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("g")).unwrap(),
            ClefSign::G
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("treble")).unwrap(),
            ClefSign::G
        );
    }

    #[test]
    fn test_clefsign_aliases_f() {
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("F")).unwrap(),
            ClefSign::F
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("f")).unwrap(),
            ClefSign::F
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("bass")).unwrap(),
            ClefSign::F
        );
    }

    #[test]
    fn test_clefsign_aliases_c() {
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("C")).unwrap(),
            ClefSign::C
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("c")).unwrap(),
            ClefSign::C
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("alto")).unwrap(),
            ClefSign::C
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("tenor")).unwrap(),
            ClefSign::C
        );
    }

    #[test]
    fn test_clefsign_aliases_tab() {
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("TAB")).unwrap(),
            ClefSign::Tab
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("tab")).unwrap(),
            ClefSign::Tab
        );
    }

    #[test]
    fn test_clefsign_from_sexpr_invalid_symbol() {
        let result = ClefSign::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_clefsign_from_sexpr_not_symbol() {
        let result = ClefSign::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // Clef Tests
    // ========================================================================

    #[test]
    fn test_clef_treble_round_trip() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        let sexpr = clef.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("clef"));
        assert!(text.contains(":sign G"));
        assert!(text.contains(":line 2"));

        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef, parsed);
    }

    #[test]
    fn test_clef_with_octave_change() {
        let clef = Clef {
            sign: ClefSign::G,
            line: Some(2),
            octave_change: Some(-1),
            number: Some(1),
            size: None,
            print_object: None,
        };

        let sexpr = clef.to_sexpr();
        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef, parsed);
    }

    #[test]
    fn test_clef_with_all_optional_fields() {
        let clef = Clef {
            sign: ClefSign::F,
            line: Some(4),
            octave_change: Some(1),
            number: Some(2),
            size: Some(SymbolSize::Cue),
            print_object: Some(YesNo::Yes),
        };

        let sexpr = clef.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":size cue"));
        assert!(text.contains(":print-object yes"));

        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef, parsed);
    }

    #[test]
    fn test_clef_from_sexpr_not_list() {
        let result = Clef::from_sexpr(&Sexpr::symbol("clef"));
        assert!(result.is_err());
    }

    #[test]
    fn test_clef_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-clef")]);
        let result = Clef::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_clef_from_sexpr_missing_sign() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("clef"),
            Sexpr::keyword("line"),
            Sexpr::Integer(2),
        ]);
        let result = Clef::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Mode Tests
    // ========================================================================

    #[test]
    fn test_mode_round_trip() {
        for mode in [
            Mode::Major,
            Mode::Minor,
            Mode::Dorian,
            Mode::Phrygian,
            Mode::Lydian,
            Mode::Mixolydian,
            Mode::Aeolian,
            Mode::Ionian,
            Mode::Locrian,
            Mode::None,
        ] {
            let sexpr = mode.to_sexpr();
            let parsed = Mode::from_sexpr(&sexpr).unwrap();
            assert_eq!(mode, parsed);
        }
    }

    #[test]
    fn test_mode_from_sexpr_invalid_symbol() {
        let result = Mode::from_sexpr(&Sexpr::symbol("invalid-mode"));
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_from_sexpr_not_symbol() {
        let result = Mode::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // CancelLocation Tests
    // ========================================================================

    #[test]
    fn test_cancellocation_round_trip() {
        for loc in [
            CancelLocation::Left,
            CancelLocation::Right,
            CancelLocation::BeforeBarline,
        ] {
            let sexpr = loc.to_sexpr();
            let parsed = CancelLocation::from_sexpr(&sexpr).unwrap();
            assert_eq!(loc, parsed);
        }
    }

    #[test]
    fn test_cancellocation_from_sexpr_invalid_symbol() {
        let result = CancelLocation::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_cancellocation_from_sexpr_not_symbol() {
        let result = CancelLocation::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // Cancel Tests
    // ========================================================================

    #[test]
    fn test_cancel_minimal_round_trip() {
        let cancel = Cancel {
            fifths: -2,
            location: None,
        };

        let sexpr = cancel.to_sexpr();
        let parsed = Cancel::from_sexpr(&sexpr).unwrap();
        assert_eq!(cancel, parsed);
    }

    #[test]
    fn test_cancel_with_location() {
        for loc in [
            CancelLocation::Left,
            CancelLocation::Right,
            CancelLocation::BeforeBarline,
        ] {
            let cancel = Cancel {
                fifths: 3,
                location: Some(loc),
            };

            let sexpr = cancel.to_sexpr();
            let parsed = Cancel::from_sexpr(&sexpr).unwrap();
            assert_eq!(cancel, parsed);
        }
    }

    #[test]
    fn test_cancel_from_sexpr_not_list() {
        let result = Cancel::from_sexpr(&Sexpr::symbol("cancel"));
        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-cancel")]);
        let result = Cancel::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_from_sexpr_missing_fifths() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("cancel"),
            Sexpr::keyword("location"),
            Sexpr::symbol("left"),
        ]);
        let result = Cancel::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // TraditionalKey Tests
    // ========================================================================

    #[test]
    fn test_traditionalkey_minimal_round_trip() {
        let key = TraditionalKey {
            cancel: None,
            fifths: 0,
            mode: None,
        };

        let sexpr = key.to_sexpr();
        let parsed = TraditionalKey::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_traditionalkey_with_cancel_and_mode() {
        let key = TraditionalKey {
            cancel: Some(Cancel {
                fifths: -2,
                location: Some(CancelLocation::Left),
            }),
            fifths: 3,
            mode: Some(Mode::Major),
        };

        let sexpr = key.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("cancel"));
        assert!(text.contains("mode"));

        let parsed = TraditionalKey::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_traditionalkey_from_sexpr_not_list() {
        let result = TraditionalKey::from_sexpr(&Sexpr::symbol("traditional-key"));
        assert!(result.is_err());
    }

    #[test]
    fn test_traditionalkey_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-traditional-key")]);
        let result = TraditionalKey::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_traditionalkey_from_sexpr_missing_fifths() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("traditional-key"),
            Sexpr::keyword("mode"),
            Sexpr::symbol("major"),
        ]);
        let result = TraditionalKey::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // KeyStep Tests
    // ========================================================================

    #[test]
    fn test_keystep_minimal_round_trip() {
        let step = KeyStep {
            step: Step::F,
            alter: 1.0,
            accidental: None,
        };

        let sexpr = step.to_sexpr();
        let parsed = KeyStep::from_sexpr(&sexpr).unwrap();
        assert_eq!(step, parsed);
    }

    #[test]
    fn test_keystep_with_accidental() {
        let step = KeyStep {
            step: Step::C,
            alter: 1.0,
            accidental: Some(AccidentalValue::Sharp),
        };

        let sexpr = step.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("accidental"));

        let parsed = KeyStep::from_sexpr(&sexpr).unwrap();
        assert_eq!(step, parsed);
    }

    #[test]
    fn test_keystep_from_sexpr_not_list() {
        let result = KeyStep::from_sexpr(&Sexpr::symbol("key-step"));
        assert!(result.is_err());
    }

    #[test]
    fn test_keystep_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-key-step")]);
        let result = KeyStep::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_keystep_from_sexpr_missing_step() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("key-step"),
            Sexpr::keyword("alter"),
            Sexpr::Float(1.0),
        ]);
        let result = KeyStep::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_keystep_from_sexpr_missing_alter() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("key-step"),
            Sexpr::keyword("step"),
            Sexpr::symbol("F"),
        ]);
        let result = KeyStep::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // KeyContent Tests
    // ========================================================================

    #[test]
    fn test_keycontent_traditional_round_trip() {
        let content = KeyContent::Traditional(TraditionalKey {
            cancel: None,
            fifths: 2,
            mode: Some(Mode::Major),
        });

        let sexpr = content.to_sexpr();
        let parsed = KeyContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_keycontent_nontraditional_round_trip() {
        let content = KeyContent::NonTraditional(vec![
            KeyStep {
                step: Step::F,
                alter: 1.0,
                accidental: None,
            },
            KeyStep {
                step: Step::C,
                alter: 1.0,
                accidental: Some(AccidentalValue::Sharp),
            },
        ]);

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("non-traditional-key"));

        let parsed = KeyContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_keycontent_nontraditional_empty() {
        let content = KeyContent::NonTraditional(vec![]);

        let sexpr = content.to_sexpr();
        let parsed = KeyContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_keycontent_from_sexpr_not_list() {
        let result = KeyContent::from_sexpr(&Sexpr::symbol("key-content"));
        assert!(result.is_err());
    }

    #[test]
    fn test_keycontent_from_sexpr_unknown_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("unknown-key-type")]);
        let result = KeyContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Key Tests
    // ========================================================================

    #[test]
    fn test_key_c_major_round_trip() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };

        let sexpr = key.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("key"));
        assert!(text.contains("fifths"));
        assert!(text.contains("0"));

        let parsed = Key::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_key_with_cancel() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: Some(Cancel {
                    fifths: -2,
                    location: Some(CancelLocation::Left),
                }),
                fifths: 3,
                mode: Some(Mode::Major),
            }),
            number: None,
            print_object: None,
        };

        let sexpr = key.to_sexpr();
        let parsed = Key::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_key_non_traditional() {
        let key = Key {
            content: KeyContent::NonTraditional(vec![
                KeyStep {
                    step: Step::F,
                    alter: 1.0,
                    accidental: None,
                },
                KeyStep {
                    step: Step::C,
                    alter: 1.0,
                    accidental: None,
                },
            ]),
            number: None,
            print_object: None,
        };

        let sexpr = key.to_sexpr();
        let parsed = Key::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_key_with_number_and_print_object() {
        let key = Key {
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths: 1,
                mode: Some(Mode::Major),
            }),
            number: Some(1),
            print_object: Some(YesNo::No),
        };

        let sexpr = key.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":number 1"));
        assert!(text.contains(":print-object no"));

        let parsed = Key::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_key_from_sexpr_not_list() {
        let result = Key::from_sexpr(&Sexpr::symbol("key"));
        assert!(result.is_err());
    }

    #[test]
    fn test_key_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-key")]);
        let result = Key::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_key_from_sexpr_missing_content() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("key"),
            Sexpr::keyword("number"),
            Sexpr::Integer(1),
        ]);
        let result = Key::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // TimeSymbol Tests
    // ========================================================================

    #[test]
    fn test_timesymbol_round_trip() {
        for symbol in [
            TimeSymbol::Common,
            TimeSymbol::Cut,
            TimeSymbol::SingleNumber,
            TimeSymbol::Note,
            TimeSymbol::DottedNote,
            TimeSymbol::Normal,
        ] {
            let sexpr = symbol.to_sexpr();
            let parsed = TimeSymbol::from_sexpr(&sexpr).unwrap();
            assert_eq!(symbol, parsed);
        }
    }

    #[test]
    fn test_timesymbol_from_sexpr_invalid_symbol() {
        let result = TimeSymbol::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_timesymbol_from_sexpr_not_symbol() {
        let result = TimeSymbol::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // TimeSignature Tests
    // ========================================================================

    #[test]
    fn test_timesignature_round_trip() {
        let sig = TimeSignature {
            beats: "4".to_string(),
            beat_type: "4".to_string(),
        };

        let sexpr = sig.to_sexpr();
        let parsed = TimeSignature::from_sexpr(&sexpr).unwrap();
        assert_eq!(sig, parsed);
    }

    #[test]
    fn test_timesignature_compound() {
        let sig = TimeSignature {
            beats: "3+2".to_string(),
            beat_type: "8".to_string(),
        };

        let sexpr = sig.to_sexpr();
        let parsed = TimeSignature::from_sexpr(&sexpr).unwrap();
        assert_eq!(sig, parsed);
    }

    #[test]
    fn test_timesignature_from_sexpr_not_list() {
        let result = TimeSignature::from_sexpr(&Sexpr::symbol("time-signature"));
        assert!(result.is_err());
    }

    #[test]
    fn test_timesignature_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-time-signature")]);
        let result = TimeSignature::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_timesignature_from_sexpr_missing_beats() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("time-signature"),
            Sexpr::keyword("beat-type"),
            Sexpr::String("4".to_string()),
        ]);
        let result = TimeSignature::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_timesignature_from_sexpr_missing_beat_type() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("time-signature"),
            Sexpr::keyword("beats"),
            Sexpr::String("4".to_string()),
        ]);
        let result = TimeSignature::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // TimeContent Tests
    // ========================================================================

    #[test]
    fn test_timecontent_measured_round_trip() {
        let content = TimeContent::Measured {
            signatures: vec![TimeSignature {
                beats: "4".to_string(),
                beat_type: "4".to_string(),
            }],
        };

        let sexpr = content.to_sexpr();
        let parsed = TimeContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_timecontent_measured_empty_signatures() {
        let content = TimeContent::Measured { signatures: vec![] };

        let sexpr = content.to_sexpr();
        let parsed = TimeContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_timecontent_senza_misura_empty() {
        let content = TimeContent::SenzaMisura("".to_string());

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("senza-misura"));
        assert!(!text.contains("value")); // empty should not have value keyword

        let parsed = TimeContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_timecontent_senza_misura_with_value() {
        let content = TimeContent::SenzaMisura("free".to_string());

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("senza-misura"));
        assert!(text.contains("value"));

        let parsed = TimeContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_timecontent_from_sexpr_not_list() {
        let result = TimeContent::from_sexpr(&Sexpr::symbol("measured"));
        assert!(result.is_err());
    }

    #[test]
    fn test_timecontent_from_sexpr_unknown_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("unknown-time-content")]);
        let result = TimeContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Time Tests
    // ========================================================================

    #[test]
    fn test_time_4_4_round_trip() {
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "4".to_string(),
                    beat_type: "4".to_string(),
                }],
            },
            number: None,
            symbol: Some(TimeSymbol::Common),
            print_object: None,
        };

        let sexpr = time.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("common"));

        let parsed = Time::from_sexpr(&sexpr).unwrap();
        assert_eq!(time, parsed);
    }

    #[test]
    fn test_time_with_all_optional_fields() {
        let time = Time {
            content: TimeContent::Measured {
                signatures: vec![TimeSignature {
                    beats: "2".to_string(),
                    beat_type: "2".to_string(),
                }],
            },
            number: Some(1),
            symbol: Some(TimeSymbol::Cut),
            print_object: Some(YesNo::Yes),
        };

        let sexpr = time.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":number 1"));
        assert!(text.contains(":symbol cut"));
        assert!(text.contains(":print-object yes"));

        let parsed = Time::from_sexpr(&sexpr).unwrap();
        assert_eq!(time, parsed);
    }

    #[test]
    fn test_time_senza_misura() {
        let time = Time {
            content: TimeContent::SenzaMisura("".to_string()),
            number: None,
            symbol: None,
            print_object: None,
        };

        let sexpr = time.to_sexpr();
        let parsed = Time::from_sexpr(&sexpr).unwrap();
        assert_eq!(time, parsed);
    }

    #[test]
    fn test_time_from_sexpr_not_list() {
        let result = Time::from_sexpr(&Sexpr::symbol("time"));
        assert!(result.is_err());
    }

    #[test]
    fn test_time_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-time")]);
        let result = Time::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_time_from_sexpr_missing_content() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("time"),
            Sexpr::keyword("number"),
            Sexpr::Integer(1),
        ]);
        let result = Time::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // GroupSymbolValue Tests
    // ========================================================================

    #[test]
    fn test_groupsymbolvalue_round_trip() {
        for value in [
            GroupSymbolValue::None,
            GroupSymbolValue::Brace,
            GroupSymbolValue::Line,
            GroupSymbolValue::Bracket,
            GroupSymbolValue::Square,
        ] {
            let sexpr = value.to_sexpr();
            let parsed = GroupSymbolValue::from_sexpr(&sexpr).unwrap();
            assert_eq!(value, parsed);
        }
    }

    #[test]
    fn test_groupsymbolvalue_from_sexpr_invalid_symbol() {
        let result = GroupSymbolValue::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_groupsymbolvalue_from_sexpr_not_symbol() {
        let result = GroupSymbolValue::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // PartSymbol Tests
    // ========================================================================

    #[test]
    fn test_part_symbol_brace() {
        let symbol = PartSymbol {
            value: GroupSymbolValue::Brace,
            top_staff: Some(1),
            bottom_staff: Some(2),
            position: Position::default(),
            color: None,
        };

        let sexpr = symbol.to_sexpr();
        let parsed = PartSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(symbol, parsed);
    }

    #[test]
    fn test_part_symbol_minimal() {
        let symbol = PartSymbol {
            value: GroupSymbolValue::Line,
            top_staff: None,
            bottom_staff: None,
            position: Position::default(),
            color: None,
        };

        let sexpr = symbol.to_sexpr();
        let parsed = PartSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(symbol, parsed);
    }

    #[test]
    fn test_part_symbol_with_position() {
        let symbol = PartSymbol {
            value: GroupSymbolValue::Bracket,
            top_staff: None,
            bottom_staff: None,
            position: Position {
                default_x: Some(10.0),
                default_y: None,
                relative_x: Some(5.0),
                relative_y: None,
            },
            color: None,
        };

        let sexpr = symbol.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));

        let parsed = PartSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(symbol.position.default_x, parsed.position.default_x);
        assert_eq!(symbol.position.relative_x, parsed.position.relative_x);
    }

    #[test]
    fn test_part_symbol_with_color() {
        let symbol = PartSymbol {
            value: GroupSymbolValue::Square,
            top_staff: None,
            bottom_staff: None,
            position: Position::default(),
            color: Some("#00FF00".to_string()),
        };

        let sexpr = symbol.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("color"));

        let parsed = PartSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(symbol.color, parsed.color);
    }

    #[test]
    fn test_part_symbol_from_sexpr_not_list() {
        let result = PartSymbol::from_sexpr(&Sexpr::symbol("part-symbol"));
        assert!(result.is_err());
    }

    #[test]
    fn test_part_symbol_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-part-symbol")]);
        let result = PartSymbol::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_part_symbol_from_sexpr_missing_value() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("part-symbol"),
            Sexpr::keyword("top-staff"),
            Sexpr::Integer(1),
        ]);
        let result = PartSymbol::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // StaffType Tests
    // ========================================================================

    #[test]
    fn test_stafftype_round_trip() {
        for staff_type in [
            StaffType::Ossia,
            StaffType::Editorial,
            StaffType::Cue,
            StaffType::Regular,
            StaffType::Alternate,
        ] {
            let sexpr = staff_type.to_sexpr();
            let parsed = StaffType::from_sexpr(&sexpr).unwrap();
            assert_eq!(staff_type, parsed);
        }
    }

    #[test]
    fn test_stafftype_from_sexpr_invalid_symbol() {
        let result = StaffType::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_stafftype_from_sexpr_not_symbol() {
        let result = StaffType::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // StaffTuning Tests
    // ========================================================================

    #[test]
    fn test_stafftuning_round_trip() {
        let tuning = StaffTuning {
            line: 1,
            tuning_step: Step::E,
            tuning_alter: None,
            tuning_octave: 2,
        };

        let sexpr = tuning.to_sexpr();
        let parsed = StaffTuning::from_sexpr(&sexpr).unwrap();
        assert_eq!(tuning, parsed);
    }

    #[test]
    fn test_stafftuning_with_alter() {
        let tuning = StaffTuning {
            line: 2,
            tuning_step: Step::B,
            tuning_alter: Some(-0.5),
            tuning_octave: 3,
        };

        let sexpr = tuning.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("tuning-alter"));

        let parsed = StaffTuning::from_sexpr(&sexpr).unwrap();
        assert_eq!(tuning, parsed);
    }

    #[test]
    fn test_stafftuning_from_sexpr_not_list() {
        let result = StaffTuning::from_sexpr(&Sexpr::symbol("staff-tuning"));
        assert!(result.is_err());
    }

    #[test]
    fn test_stafftuning_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-staff-tuning")]);
        let result = StaffTuning::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_stafftuning_from_sexpr_missing_line() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("staff-tuning"),
            Sexpr::keyword("tuning-step"),
            Sexpr::symbol("E"),
            Sexpr::keyword("tuning-octave"),
            Sexpr::Integer(2),
        ]);
        let result = StaffTuning::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_stafftuning_from_sexpr_missing_tuning_step() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("staff-tuning"),
            Sexpr::keyword("line"),
            Sexpr::Integer(1),
            Sexpr::keyword("tuning-octave"),
            Sexpr::Integer(2),
        ]);
        let result = StaffTuning::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_stafftuning_from_sexpr_missing_tuning_octave() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("staff-tuning"),
            Sexpr::keyword("line"),
            Sexpr::Integer(1),
            Sexpr::keyword("tuning-step"),
            Sexpr::symbol("E"),
        ]);
        let result = StaffTuning::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // StaffDetails Tests
    // ========================================================================

    #[test]
    fn test_staff_details_minimal() {
        let details = StaffDetails::default();

        let sexpr = details.to_sexpr();
        let parsed = StaffDetails::from_sexpr(&sexpr).unwrap();
        assert_eq!(details, parsed);
    }

    #[test]
    fn test_staff_details_with_tuning() {
        let details = StaffDetails {
            staff_lines: Some(6),
            staff_tuning: vec![StaffTuning {
                line: 1,
                tuning_step: Step::E,
                tuning_alter: None,
                tuning_octave: 2,
            }],
            ..Default::default()
        };

        let sexpr = details.to_sexpr();
        let parsed = StaffDetails::from_sexpr(&sexpr).unwrap();
        assert_eq!(details, parsed);
    }

    #[test]
    fn test_staff_details_with_all_fields() {
        let details = StaffDetails {
            number: Some(1),
            staff_type: Some(StaffType::Regular),
            staff_lines: Some(5),
            staff_tuning: vec![
                StaffTuning {
                    line: 1,
                    tuning_step: Step::E,
                    tuning_alter: None,
                    tuning_octave: 2,
                },
                StaffTuning {
                    line: 2,
                    tuning_step: Step::A,
                    tuning_alter: None,
                    tuning_octave: 2,
                },
            ],
            capo: Some(2),
            staff_size: Some(80.0),
        };

        let sexpr = details.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":number 1"));
        assert!(text.contains(":staff-type regular"));
        assert!(text.contains(":capo 2"));
        assert!(text.contains(":staff-size 80"));

        let parsed = StaffDetails::from_sexpr(&sexpr).unwrap();
        assert_eq!(details, parsed);
    }

    #[test]
    fn test_staff_details_from_sexpr_not_list() {
        let result = StaffDetails::from_sexpr(&Sexpr::symbol("staff-details"));
        assert!(result.is_err());
    }

    #[test]
    fn test_staff_details_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-staff-details")]);
        let result = StaffDetails::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Transpose Tests
    // ========================================================================

    #[test]
    fn test_transpose_clarinet() {
        let transpose = Transpose {
            number: None,
            diatonic: Some(-1),
            chromatic: -2,
            octave_change: None,
            double: None,
        };

        let sexpr = transpose.to_sexpr();
        let parsed = Transpose::from_sexpr(&sexpr).unwrap();
        assert_eq!(transpose, parsed);
    }

    #[test]
    fn test_transpose_with_all_fields() {
        let transpose = Transpose {
            number: Some(1),
            diatonic: Some(-7),
            chromatic: -12,
            octave_change: Some(-1),
            double: Some(YesNo::Yes),
        };

        let sexpr = transpose.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":number 1"));
        assert!(text.contains(":diatonic -7"));
        assert!(text.contains(":chromatic -12"));
        assert!(text.contains(":octave-change -1"));
        assert!(text.contains(":double yes"));

        let parsed = Transpose::from_sexpr(&sexpr).unwrap();
        assert_eq!(transpose, parsed);
    }

    #[test]
    fn test_transpose_from_sexpr_not_list() {
        let result = Transpose::from_sexpr(&Sexpr::symbol("transpose"));
        assert!(result.is_err());
    }

    #[test]
    fn test_transpose_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-transpose")]);
        let result = Transpose::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_transpose_from_sexpr_missing_chromatic() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("transpose"),
            Sexpr::keyword("diatonic"),
            Sexpr::Integer(-1),
        ]);
        let result = Transpose::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // MeasureStyleContent Tests
    // ========================================================================

    #[test]
    fn test_measurestylecontent_multiple_rest() {
        let content = MeasureStyleContent::MultipleRest {
            count: 4,
            use_symbols: Some(YesNo::Yes),
        };

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("multiple-rest"));

        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_multiple_rest_no_use_symbols() {
        let content = MeasureStyleContent::MultipleRest {
            count: 2,
            use_symbols: None,
        };

        let sexpr = content.to_sexpr();
        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_measure_repeat() {
        let content = MeasureStyleContent::MeasureRepeat {
            r#type: StartStop::Start,
            slashes: Some(2),
        };

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("measure-repeat"));

        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_measure_repeat_no_slashes() {
        let content = MeasureStyleContent::MeasureRepeat {
            r#type: StartStop::Stop,
            slashes: None,
        };

        let sexpr = content.to_sexpr();
        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_beat_repeat() {
        let content = MeasureStyleContent::BeatRepeat {
            r#type: StartStop::Start,
            slashes: Some(1),
        };

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("beat-repeat"));

        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_beat_repeat_no_slashes() {
        let content = MeasureStyleContent::BeatRepeat {
            r#type: StartStop::Stop,
            slashes: None,
        };

        let sexpr = content.to_sexpr();
        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_slash() {
        let content = MeasureStyleContent::Slash {
            r#type: StartStop::Start,
            use_stems: Some(YesNo::Yes),
        };

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("slash"));

        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_slash_no_use_stems() {
        let content = MeasureStyleContent::Slash {
            r#type: StartStop::Stop,
            use_stems: None,
        };

        let sexpr = content.to_sexpr();
        let parsed = MeasureStyleContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_measurestylecontent_from_sexpr_not_list() {
        let result = MeasureStyleContent::from_sexpr(&Sexpr::symbol("multiple-rest"));
        assert!(result.is_err());
    }

    #[test]
    fn test_measurestylecontent_from_sexpr_unknown_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("unknown-style")]);
        let result = MeasureStyleContent::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // MeasureStyle Tests
    // ========================================================================

    #[test]
    fn test_measure_style_multiple_rest() {
        let style = MeasureStyle {
            number: None,
            content: MeasureStyleContent::MultipleRest {
                count: 4,
                use_symbols: Some(YesNo::Yes),
            },
        };

        let sexpr = style.to_sexpr();
        let parsed = MeasureStyle::from_sexpr(&sexpr).unwrap();
        assert_eq!(style.content, parsed.content);
    }

    #[test]
    fn test_measure_style_with_number() {
        let style = MeasureStyle {
            number: Some(1),
            content: MeasureStyleContent::Slash {
                r#type: StartStop::Start,
                use_stems: None,
            },
        };

        let sexpr = style.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":number 1"));

        let parsed = MeasureStyle::from_sexpr(&sexpr).unwrap();
        assert_eq!(style, parsed);
    }

    #[test]
    fn test_measure_style_from_sexpr_not_list() {
        let result = MeasureStyle::from_sexpr(&Sexpr::symbol("measure-style"));
        assert!(result.is_err());
    }

    #[test]
    fn test_measure_style_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-measure-style")]);
        let result = MeasureStyle::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_measure_style_from_sexpr_missing_content() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("measure-style"),
            Sexpr::keyword("number"),
            Sexpr::Integer(1),
        ]);
        let result = MeasureStyle::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // BarStyle Tests
    // ========================================================================

    #[test]
    fn test_barstyle_round_trip() {
        for style in [
            BarStyle::Regular,
            BarStyle::Dotted,
            BarStyle::Dashed,
            BarStyle::Heavy,
            BarStyle::LightLight,
            BarStyle::LightHeavy,
            BarStyle::HeavyLight,
            BarStyle::HeavyHeavy,
            BarStyle::Tick,
            BarStyle::Short,
            BarStyle::None,
        ] {
            let sexpr = style.to_sexpr();
            let parsed = BarStyle::from_sexpr(&sexpr).unwrap();
            assert_eq!(style, parsed);
        }
    }

    #[test]
    fn test_barstyle_from_sexpr_invalid_symbol() {
        let result = BarStyle::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_barstyle_from_sexpr_not_symbol() {
        let result = BarStyle::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // Winged Tests
    // ========================================================================

    #[test]
    fn test_winged_round_trip() {
        for winged in [
            Winged::None,
            Winged::Straight,
            Winged::Curved,
            Winged::DoubleStraight,
            Winged::DoubleCurved,
        ] {
            let sexpr = winged.to_sexpr();
            let parsed = Winged::from_sexpr(&sexpr).unwrap();
            assert_eq!(winged, parsed);
        }
    }

    #[test]
    fn test_winged_from_sexpr_invalid_symbol() {
        let result = Winged::from_sexpr(&Sexpr::symbol("invalid"));
        assert!(result.is_err());
    }

    #[test]
    fn test_winged_from_sexpr_not_symbol() {
        let result = Winged::from_sexpr(&Sexpr::Integer(42));
        assert!(result.is_err());
    }

    // ========================================================================
    // Repeat Tests
    // ========================================================================

    #[test]
    fn test_repeat_backward_minimal() {
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: None,
            winged: None,
        };

        let sexpr = repeat.to_sexpr();
        let parsed = Repeat::from_sexpr(&sexpr).unwrap();
        assert_eq!(repeat, parsed);
    }

    #[test]
    fn test_repeat_forward_minimal() {
        let repeat = Repeat {
            direction: BackwardForward::Forward,
            times: None,
            winged: None,
        };

        let sexpr = repeat.to_sexpr();
        let parsed = Repeat::from_sexpr(&sexpr).unwrap();
        assert_eq!(repeat, parsed);
    }

    #[test]
    fn test_repeat_with_times_and_winged() {
        let repeat = Repeat {
            direction: BackwardForward::Backward,
            times: Some(3),
            winged: Some(Winged::Curved),
        };

        let sexpr = repeat.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":times 3"));
        assert!(text.contains(":winged curved"));

        let parsed = Repeat::from_sexpr(&sexpr).unwrap();
        assert_eq!(repeat, parsed);
    }

    #[test]
    fn test_repeat_from_sexpr_not_list() {
        let result = Repeat::from_sexpr(&Sexpr::symbol("repeat"));
        assert!(result.is_err());
    }

    #[test]
    fn test_repeat_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-repeat")]);
        let result = Repeat::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_repeat_from_sexpr_missing_direction() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("repeat"),
            Sexpr::keyword("times"),
            Sexpr::Integer(2),
        ]);
        let result = Repeat::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Ending Tests
    // ========================================================================

    #[test]
    fn test_ending_minimal() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: None,
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };

        let sexpr = ending.to_sexpr();
        let parsed = Ending::from_sexpr(&sexpr).unwrap();
        assert_eq!(ending, parsed);
    }

    #[test]
    fn test_ending_with_all_fields() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: Some("1.".to_string()),
            print_object: Some(YesNo::Yes),
            end_length: Some(30.0),
            text_x: Some(5.0),
            text_y: Some(-10.0),
        };

        let sexpr = ending.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":text \"1.\""));
        assert!(text.contains(":print-object yes"));
        assert!(text.contains(":end-length 30"));
        assert!(text.contains(":text-x 5"));
        assert!(text.contains(":text-y -10"));

        let parsed = Ending::from_sexpr(&sexpr).unwrap();
        assert_eq!(ending, parsed);
    }

    #[test]
    fn test_ending_stop() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Stop,
            number: "1".to_string(),
            text: None,
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };

        let sexpr = ending.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":type stop"));

        let parsed = Ending::from_sexpr(&sexpr).unwrap();
        assert_eq!(ending, parsed);
    }

    #[test]
    fn test_ending_discontinue() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Discontinue,
            number: "1, 2".to_string(),
            text: None,
            print_object: None,
            end_length: None,
            text_x: None,
            text_y: None,
        };

        let sexpr = ending.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":type discontinue"));

        let parsed = Ending::from_sexpr(&sexpr).unwrap();
        assert_eq!(ending, parsed);
    }

    #[test]
    fn test_ending_from_sexpr_not_list() {
        let result = Ending::from_sexpr(&Sexpr::symbol("ending"));
        assert!(result.is_err());
    }

    #[test]
    fn test_ending_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-ending")]);
        let result = Ending::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_ending_from_sexpr_missing_type() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("ending"),
            Sexpr::keyword("number"),
            Sexpr::String("1".to_string()),
        ]);
        let result = Ending::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    #[test]
    fn test_ending_from_sexpr_missing_number() {
        let sexpr = Sexpr::List(vec![
            Sexpr::symbol("ending"),
            Sexpr::keyword("type"),
            Sexpr::symbol("start"),
        ]);
        let result = Ending::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Barline Tests
    // ========================================================================

    #[test]
    fn test_barline_minimal() {
        let barline = Barline::default();

        let sexpr = barline.to_sexpr();
        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert_eq!(barline.location, parsed.location);
        assert_eq!(barline.bar_style, parsed.bar_style);
    }

    #[test]
    fn test_barline_simple() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert_eq!(barline.location, parsed.location);
        assert_eq!(barline.bar_style, parsed.bar_style);
    }

    #[test]
    fn test_barline_left_location() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Left),
            bar_style: Some(BarStyle::HeavyLight),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":location left"));
        assert!(text.contains(":bar-style heavy-light"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert_eq!(barline.location, parsed.location);
    }

    #[test]
    fn test_barline_middle_location() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Middle),
            bar_style: Some(BarStyle::Dotted),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":location middle"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert_eq!(barline.location, parsed.location);
    }

    #[test]
    fn test_barline_with_repeat() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            repeat: Some(Repeat {
                direction: BackwardForward::Backward,
                times: Some(2),
                winged: None,
            }),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("repeat"));
        assert!(text.contains("backward"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert!(parsed.repeat.is_some());
    }

    #[test]
    fn test_barline_with_ending() {
        let barline = Barline {
            location: Some(RightLeftMiddle::Left),
            bar_style: Some(BarStyle::HeavyLight),
            ending: Some(Ending {
                r#type: StartStopDiscontinue::Start,
                number: "1".to_string(),
                text: Some("1.".to_string()),
                print_object: None,
                end_length: None,
                text_x: None,
                text_y: None,
            }),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("ending"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert!(parsed.ending.is_some());
    }

    #[test]
    fn test_barline_with_segno() {
        use crate::ir::direction::Segno;

        let barline = Barline {
            segno: Some(Segno {
                print_style: PrintStyle::default(),
                smufl: None,
            }),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("segno"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert!(parsed.segno.is_some());
    }

    #[test]
    fn test_barline_with_coda() {
        use crate::ir::direction::Coda;

        let barline = Barline {
            coda: Some(Coda {
                print_style: PrintStyle::default(),
                smufl: None,
            }),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("coda"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert!(parsed.coda.is_some());
    }

    #[test]
    fn test_barline_with_fermatas() {
        let barline = Barline {
            fermatas: vec![
                Fermata {
                    shape: Some(FermataShape::Normal),
                    r#type: Some(UprightInverted::Upright),
                    print_style: PrintStyle::default(),
                },
                Fermata {
                    shape: Some(FermataShape::Angled),
                    r#type: Some(UprightInverted::Inverted),
                    print_style: PrintStyle::default(),
                },
            ],
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("fermatas"));

        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.fermatas.len(), 2);
    }

    #[test]
    fn test_barline_from_sexpr_not_list() {
        let result = Barline::from_sexpr(&Sexpr::symbol("barline"));
        assert!(result.is_err());
    }

    #[test]
    fn test_barline_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-barline")]);
        let result = Barline::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Attributes Tests
    // ========================================================================

    #[test]
    fn test_attributes_minimal() {
        let attrs = Attributes::default();

        let sexpr = attrs.to_sexpr();
        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(attrs.divisions, parsed.divisions);
        assert!(parsed.keys.is_empty());
        assert!(parsed.times.is_empty());
        assert!(parsed.clefs.is_empty());
    }

    #[test]
    fn test_attributes_with_divisions() {
        let attrs = Attributes {
            divisions: Some(4),
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":divisions 4"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(attrs.divisions, parsed.divisions);
    }

    #[test]
    fn test_attributes_with_staves() {
        let attrs = Attributes {
            staves: Some(2),
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":staves 2"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(attrs.staves, parsed.staves);
    }

    #[test]
    fn test_attributes_with_instruments() {
        let attrs = Attributes {
            instruments: Some(3),
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":instruments 3"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(attrs.instruments, parsed.instruments);
    }

    #[test]
    fn test_attributes_with_part_symbol() {
        let attrs = Attributes {
            part_symbol: Some(PartSymbol {
                value: GroupSymbolValue::Brace,
                top_staff: Some(1),
                bottom_staff: Some(2),
                position: Position::default(),
                color: None,
            }),
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("part-symbol"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert!(parsed.part_symbol.is_some());
    }

    #[test]
    fn test_attributes_with_staff_details() {
        let attrs = Attributes {
            staff_details: vec![StaffDetails {
                number: Some(1),
                staff_type: Some(StaffType::Regular),
                staff_lines: Some(5),
                ..Default::default()
            }],
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("staff-details"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.staff_details.len(), 1);
    }

    #[test]
    fn test_attributes_with_transpose() {
        let attrs = Attributes {
            transpose: vec![Transpose {
                number: None,
                diatonic: Some(-1),
                chromatic: -2,
                octave_change: None,
                double: None,
            }],
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("transpose"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.transpose.len(), 1);
    }

    #[test]
    fn test_attributes_with_measure_styles() {
        let attrs = Attributes {
            measure_styles: vec![MeasureStyle {
                number: None,
                content: MeasureStyleContent::MultipleRest {
                    count: 4,
                    use_symbols: None,
                },
            }],
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("measure-styles"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.measure_styles.len(), 1);
    }

    #[test]
    fn test_attributes_full_round_trip() {
        let attrs = Attributes {
            divisions: Some(4),
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
                number: None,
                symbol: None,
                print_object: None,
            }],
            clefs: vec![Clef {
                sign: ClefSign::G,
                line: Some(2),
                octave_change: None,
                number: None,
                size: None,
                print_object: None,
            }],
            staves: Some(2),
            ..Default::default()
        };

        let sexpr = attrs.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("attributes"));
        assert!(text.contains(":divisions 4"));

        let parsed = Attributes::from_sexpr(&sexpr).unwrap();
        assert_eq!(attrs.divisions, parsed.divisions);
        assert_eq!(attrs.keys.len(), parsed.keys.len());
        assert_eq!(attrs.times.len(), parsed.times.len());
        assert_eq!(attrs.clefs.len(), parsed.clefs.len());
    }

    #[test]
    fn test_attributes_from_sexpr_not_list() {
        let result = Attributes::from_sexpr(&Sexpr::symbol("attributes"));
        assert!(result.is_err());
    }

    #[test]
    fn test_attributes_from_sexpr_wrong_head() {
        let sexpr = Sexpr::List(vec![Sexpr::symbol("not-attributes")]);
        let result = Attributes::from_sexpr(&sexpr);
        assert!(result.is_err());
    }

    // ========================================================================
    // Additional Coverage Tests
    // ========================================================================

    #[test]
    fn test_fermata_with_all_position_fields() {
        let fermata = Fermata {
            shape: None,
            r#type: None,
            print_style: PrintStyle {
                position: Position {
                    default_x: None,
                    default_y: Some(5.0),
                    relative_x: Some(2.0),
                    relative_y: Some(-3.0),
                },
                font: Default::default(),
                color: None,
            },
        };

        let sexpr = fermata.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));

        let parsed = Fermata::from_sexpr(&sexpr).unwrap();
        assert_eq!(
            fermata.print_style.position.relative_x,
            parsed.print_style.position.relative_x
        );
        assert_eq!(
            fermata.print_style.position.relative_y,
            parsed.print_style.position.relative_y
        );
    }

    #[test]
    fn test_part_symbol_with_all_position_fields() {
        let symbol = PartSymbol {
            value: GroupSymbolValue::Line,
            top_staff: None,
            bottom_staff: None,
            position: Position {
                default_x: Some(1.0),
                default_y: Some(2.0),
                relative_x: Some(3.0),
                relative_y: Some(4.0),
            },
            color: Some("#0000FF".to_string()),
        };

        let sexpr = symbol.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("position"));
        assert!(text.contains("color"));

        let parsed = PartSymbol::from_sexpr(&sexpr).unwrap();
        assert_eq!(symbol.position.default_x, parsed.position.default_x);
        assert_eq!(symbol.position.default_y, parsed.position.default_y);
        assert_eq!(symbol.position.relative_x, parsed.position.relative_x);
        assert_eq!(symbol.position.relative_y, parsed.position.relative_y);
    }

    #[test]
    fn test_timecontent_measured_multiple_signatures() {
        let content = TimeContent::Measured {
            signatures: vec![
                TimeSignature {
                    beats: "3".to_string(),
                    beat_type: "4".to_string(),
                },
                TimeSignature {
                    beats: "2".to_string(),
                    beat_type: "8".to_string(),
                },
            ],
        };

        let sexpr = content.to_sexpr();
        let parsed = TimeContent::from_sexpr(&sexpr).unwrap();

        if let TimeContent::Measured { signatures } = &parsed {
            assert_eq!(signatures.len(), 2);
        } else {
            panic!("Expected Measured time content");
        }
    }

    #[test]
    fn test_clef_percussion_minimal() {
        let clef = Clef {
            sign: ClefSign::Percussion,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        let sexpr = clef.to_sexpr();
        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef, parsed);
    }

    #[test]
    fn test_clef_tab_with_all_options() {
        let clef = Clef {
            sign: ClefSign::Tab,
            line: Some(5),
            octave_change: Some(0),
            number: Some(1),
            size: Some(SymbolSize::Full),
            print_object: Some(YesNo::No),
        };

        let sexpr = clef.to_sexpr();
        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef, parsed);
    }

    #[test]
    fn test_clef_jianpu() {
        let clef = Clef {
            sign: ClefSign::Jianpu,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        let sexpr = clef.to_sexpr();
        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef.sign, parsed.sign);
    }

    #[test]
    fn test_clef_none_sign() {
        let clef = Clef {
            sign: ClefSign::None,
            line: None,
            octave_change: None,
            number: None,
            size: None,
            print_object: None,
        };

        let sexpr = clef.to_sexpr();
        let parsed = Clef::from_sexpr(&sexpr).unwrap();
        assert_eq!(clef.sign, parsed.sign);
    }

    #[test]
    fn test_traditionalkey_without_mode() {
        let key = TraditionalKey {
            cancel: None,
            fifths: 5, // B major
            mode: None,
        };

        let sexpr = key.to_sexpr();
        let parsed = TraditionalKey::from_sexpr(&sexpr).unwrap();
        assert_eq!(key, parsed);
    }

    #[test]
    fn test_keystep_with_double_flat() {
        let step = KeyStep {
            step: Step::B,
            alter: -2.0,
            accidental: Some(AccidentalValue::DoubleFlat),
        };

        let sexpr = step.to_sexpr();
        let parsed = KeyStep::from_sexpr(&sexpr).unwrap();
        assert_eq!(step, parsed);
    }

    #[test]
    fn test_cancel_all_locations() {
        for loc in [
            CancelLocation::Left,
            CancelLocation::Right,
            CancelLocation::BeforeBarline,
        ] {
            let cancel = Cancel {
                fifths: 2,
                location: Some(loc),
            };

            let sexpr = cancel.to_sexpr();
            let parsed = Cancel::from_sexpr(&sexpr).unwrap();
            assert_eq!(cancel.location, parsed.location);
        }
    }

    #[test]
    fn test_staffdetails_ossia() {
        let details = StaffDetails {
            number: Some(2),
            staff_type: Some(StaffType::Ossia),
            staff_lines: Some(5),
            staff_tuning: vec![],
            capo: None,
            staff_size: Some(60.0),
        };

        let sexpr = details.to_sexpr();
        let parsed = StaffDetails::from_sexpr(&sexpr).unwrap();
        assert_eq!(details, parsed);
    }

    #[test]
    fn test_staffdetails_cue() {
        let details = StaffDetails {
            number: None,
            staff_type: Some(StaffType::Cue),
            staff_lines: None,
            staff_tuning: vec![],
            capo: None,
            staff_size: Some(75.0),
        };

        let sexpr = details.to_sexpr();
        let parsed = StaffDetails::from_sexpr(&sexpr).unwrap();
        assert_eq!(details.staff_type, parsed.staff_type);
    }

    #[test]
    fn test_stafftuning_all_steps() {
        for step in [
            Step::A,
            Step::B,
            Step::C,
            Step::D,
            Step::E,
            Step::F,
            Step::G,
        ] {
            let tuning = StaffTuning {
                line: 1,
                tuning_step: step,
                tuning_alter: Some(0.0),
                tuning_octave: 4,
            };

            let sexpr = tuning.to_sexpr();
            let parsed = StaffTuning::from_sexpr(&sexpr).unwrap();
            assert_eq!(tuning.tuning_step, parsed.tuning_step);
        }
    }

    #[test]
    fn test_transpose_octave_up() {
        let transpose = Transpose {
            number: None,
            diatonic: None,
            chromatic: 12,
            octave_change: Some(1),
            double: None,
        };

        let sexpr = transpose.to_sexpr();
        let parsed = Transpose::from_sexpr(&sexpr).unwrap();
        assert_eq!(transpose, parsed);
    }

    #[test]
    fn test_repeat_all_winged_types() {
        for winged in [
            Winged::None,
            Winged::Straight,
            Winged::Curved,
            Winged::DoubleStraight,
            Winged::DoubleCurved,
        ] {
            let repeat = Repeat {
                direction: BackwardForward::Forward,
                times: None,
                winged: Some(winged),
            };

            let sexpr = repeat.to_sexpr();
            let parsed = Repeat::from_sexpr(&sexpr).unwrap();
            assert_eq!(repeat.winged, parsed.winged);
        }
    }

    #[test]
    fn test_barline_all_bar_styles() {
        for style in [
            BarStyle::Regular,
            BarStyle::Dotted,
            BarStyle::Dashed,
            BarStyle::Heavy,
            BarStyle::LightLight,
            BarStyle::LightHeavy,
            BarStyle::HeavyLight,
            BarStyle::HeavyHeavy,
            BarStyle::Tick,
            BarStyle::Short,
            BarStyle::None,
        ] {
            let barline = Barline {
                bar_style: Some(style),
                ..Default::default()
            };

            let sexpr = barline.to_sexpr();
            let parsed = Barline::from_sexpr(&sexpr).unwrap();
            assert_eq!(barline.bar_style, parsed.bar_style);
        }
    }

    #[test]
    fn test_time_all_symbols() {
        for symbol in [
            TimeSymbol::Common,
            TimeSymbol::Cut,
            TimeSymbol::SingleNumber,
            TimeSymbol::Note,
            TimeSymbol::DottedNote,
            TimeSymbol::Normal,
        ] {
            let time = Time {
                content: TimeContent::Measured {
                    signatures: vec![TimeSignature {
                        beats: "4".to_string(),
                        beat_type: "4".to_string(),
                    }],
                },
                number: None,
                symbol: Some(symbol),
                print_object: None,
            };

            let sexpr = time.to_sexpr();
            let parsed = Time::from_sexpr(&sexpr).unwrap();
            assert_eq!(time.symbol, parsed.symbol);
        }
    }

    #[test]
    fn test_fermata_all_shapes() {
        for shape in [
            FermataShape::Normal,
            FermataShape::Angled,
            FermataShape::Square,
            FermataShape::DoubleAngled,
            FermataShape::DoubleSquare,
            FermataShape::DoubleDot,
            FermataShape::HalfCurve,
            FermataShape::Curlew,
        ] {
            let fermata = Fermata {
                shape: Some(shape),
                r#type: None,
                print_style: PrintStyle::default(),
            };

            let sexpr = fermata.to_sexpr();
            let parsed = Fermata::from_sexpr(&sexpr).unwrap();
            assert_eq!(fermata.shape, parsed.shape);
        }
    }

    #[test]
    fn test_fermata_both_types() {
        for t in [UprightInverted::Upright, UprightInverted::Inverted] {
            let fermata = Fermata {
                shape: None,
                r#type: Some(t),
                print_style: PrintStyle::default(),
            };

            let sexpr = fermata.to_sexpr();
            let parsed = Fermata::from_sexpr(&sexpr).unwrap();
            assert_eq!(fermata.r#type, parsed.r#type);
        }
    }

    #[test]
    fn test_groupsymbolvalue_all_values() {
        for value in [
            GroupSymbolValue::None,
            GroupSymbolValue::Brace,
            GroupSymbolValue::Line,
            GroupSymbolValue::Bracket,
            GroupSymbolValue::Square,
        ] {
            let symbol = PartSymbol {
                value,
                top_staff: None,
                bottom_staff: None,
                position: Position::default(),
                color: None,
            };

            let sexpr = symbol.to_sexpr();
            let parsed = PartSymbol::from_sexpr(&sexpr).unwrap();
            assert_eq!(symbol.value, parsed.value);
        }
    }

    #[test]
    fn test_measurestyle_all_content_types_with_number() {
        let contents = vec![
            MeasureStyleContent::MultipleRest {
                count: 2,
                use_symbols: None,
            },
            MeasureStyleContent::MeasureRepeat {
                r#type: StartStop::Start,
                slashes: None,
            },
            MeasureStyleContent::BeatRepeat {
                r#type: StartStop::Stop,
                slashes: Some(2),
            },
            MeasureStyleContent::Slash {
                r#type: StartStop::Start,
                use_stems: Some(YesNo::No),
            },
        ];

        for content in contents {
            let style = MeasureStyle {
                number: Some(1),
                content,
            };

            let sexpr = style.to_sexpr();
            let parsed = MeasureStyle::from_sexpr(&sexpr).unwrap();
            assert_eq!(style, parsed);
        }
    }

    #[test]
    fn test_mode_all_church_modes() {
        for mode in [
            Mode::Dorian,
            Mode::Phrygian,
            Mode::Lydian,
            Mode::Mixolydian,
            Mode::Aeolian,
            Mode::Ionian,
            Mode::Locrian,
        ] {
            let key = TraditionalKey {
                cancel: None,
                fifths: 0,
                mode: Some(mode),
            };

            let sexpr = key.to_sexpr();
            let parsed = TraditionalKey::from_sexpr(&sexpr).unwrap();
            assert_eq!(key.mode, parsed.mode);
        }
    }

    #[test]
    fn test_stafftype_all_variants_in_details() {
        for staff_type in [
            StaffType::Ossia,
            StaffType::Editorial,
            StaffType::Cue,
            StaffType::Regular,
            StaffType::Alternate,
        ] {
            let details = StaffDetails {
                number: None,
                staff_type: Some(staff_type),
                staff_lines: None,
                staff_tuning: vec![],
                capo: None,
                staff_size: None,
            };

            let sexpr = details.to_sexpr();
            let parsed = StaffDetails::from_sexpr(&sexpr).unwrap();
            assert_eq!(details.staff_type, parsed.staff_type);
        }
    }

    #[test]
    fn test_ending_all_types() {
        for t in [
            StartStopDiscontinue::Start,
            StartStopDiscontinue::Stop,
            StartStopDiscontinue::Discontinue,
        ] {
            let ending = Ending {
                r#type: t,
                number: "1".to_string(),
                text: None,
                print_object: None,
                end_length: None,
                text_x: None,
                text_y: None,
            };

            let sexpr = ending.to_sexpr();
            let parsed = Ending::from_sexpr(&sexpr).unwrap();
            assert_eq!(ending.r#type, parsed.r#type);
        }
    }
}
