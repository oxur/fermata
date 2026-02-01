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
            KeyContent::NonTraditional(steps) => {
                ListBuilder::new("non-traditional-key")
                    .kwarg_list("key-steps", steps)
                    .build()
            }
        }
    }
}

impl FromSexpr for KeyContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("key-content", sexpr))?;

        match get_head(list)? {
            "traditional-key" => {
                Ok(KeyContent::Traditional(TraditionalKey::from_sexpr(sexpr)?))
            }
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
                let signatures = optional_kwarg::<Vec<TimeSignature>>(list, "signatures")?
                    .unwrap_or_default();
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
            MeasureStyleContent::BeatRepeat { r#type, slashes } => {
                ListBuilder::new("beat-repeat")
                    .kwarg("type", r#type)
                    .kwarg_opt("slashes", slashes)
                    .build()
            }
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
    use crate::ir::common::{BackwardForward, Position, StartStopDiscontinue, YesNo};
    use crate::ir::pitch::Step;
    use crate::sexpr::print_sexpr;

    // === ClefSign Tests ===

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
    fn test_clefsign_aliases() {
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("treble")).unwrap(),
            ClefSign::G
        );
        assert_eq!(
            ClefSign::from_sexpr(&Sexpr::symbol("bass")).unwrap(),
            ClefSign::F
        );
    }

    // === Clef Tests ===

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

    // === Mode Tests ===

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

    // === Key Tests ===

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

    // === Time Tests ===

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

    // === Transpose Tests ===

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

    // === Barline Tests ===

    #[test]
    fn test_barline_simple() {
        let barline = Barline {
            location: Some(crate::ir::common::RightLeftMiddle::Right),
            bar_style: Some(BarStyle::LightHeavy),
            ..Default::default()
        };

        let sexpr = barline.to_sexpr();
        let parsed = Barline::from_sexpr(&sexpr).unwrap();
        assert_eq!(barline.location, parsed.location);
        assert_eq!(barline.bar_style, parsed.bar_style);
    }

    #[test]
    fn test_barline_with_repeat() {
        let barline = Barline {
            location: Some(crate::ir::common::RightLeftMiddle::Right),
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
    fn test_ending_round_trip() {
        let ending = Ending {
            r#type: StartStopDiscontinue::Start,
            number: "1".to_string(),
            text: Some("1.".to_string()),
            print_object: None,
            end_length: Some(30.0),
            text_x: None,
            text_y: None,
        };

        let sexpr = ending.to_sexpr();
        let parsed = Ending::from_sexpr(&sexpr).unwrap();
        assert_eq!(ending, parsed);
    }

    // === Attributes Tests ===

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

    // === MeasureStyle Tests ===

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

    // === PartSymbol Tests ===

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

    // === StaffDetails Tests ===

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
}
