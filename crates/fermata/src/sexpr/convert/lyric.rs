//! S-expression conversions for `ir::lyric` types.
//!
//! This module implements [`ToSexpr`] and [`FromSexpr`] for lyric-related
//! types that represent text sung with notes:
//!
//! - [`Lyric`] - Main lyric container
//! - [`LyricContent`] - Content variants (syllable, extend, laughing, humming)
//! - [`Syllabic`] - Syllable position in word
//! - [`TextElementData`] - Formatted text element
//! - [`LyricExtension`] - Additional syllable after elision
//! - [`Elision`] - Elision character between syllables
//! - [`Extend`] - Lyric extension line

use crate::ir::common::Position;
use crate::ir::lyric::{Elision, Extend, Lyric, LyricContent, LyricExtension, Syllabic, TextElementData};
use crate::sexpr::{ConvertError, ConvertResult, FromSexpr, ListBuilder, Sexpr, ToSexpr};

use super::{expect_head, find_kwarg, get_head, optional_kwarg, require_kwarg};

// ============================================================================
// Syllabic
// ============================================================================

impl ToSexpr for Syllabic {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Syllabic::Single => "single",
            Syllabic::Begin => "begin",
            Syllabic::End => "end",
            Syllabic::Middle => "middle",
        })
    }
}

impl FromSexpr for Syllabic {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("single") => Ok(Syllabic::Single),
            Some("begin") => Ok(Syllabic::Begin),
            Some("end") => Ok(Syllabic::End),
            Some("middle") => Ok(Syllabic::Middle),
            _ => Err(ConvertError::type_mismatch("syllabic", sexpr)),
        }
    }
}

// ============================================================================
// TextElementData
// ============================================================================

impl ToSexpr for TextElementData {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("text").kwarg("value", &self.value);

        // Font (only emit if has content)
        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();
        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder = builder
            .kwarg_opt("color", &self.color)
            .kwarg_opt("lang", &self.lang);

        builder.build()
    }
}

impl FromSexpr for TextElementData {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("text list", sexpr))?;

        expect_head(list, "text")?;

        use crate::ir::common::Font;

        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(TextElementData {
            value: require_kwarg(list, "value")?,
            font,
            color: optional_kwarg(list, "color")?,
            lang: optional_kwarg(list, "lang")?,
        })
    }
}

// ============================================================================
// Elision
// ============================================================================

impl ToSexpr for Elision {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("elision").kwarg("value", &self.value);

        // Font (only emit if has content)
        let font = &self.font;
        let has_font = font.font_family.is_some()
            || font.font_style.is_some()
            || font.font_size.is_some()
            || font.font_weight.is_some();
        if has_font {
            builder = builder.kwarg_raw("font", font.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for Elision {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("elision list", sexpr))?;

        expect_head(list, "elision")?;

        use crate::ir::common::Font;

        let font = match find_kwarg(list, "font") {
            Some(fs) => Font::from_sexpr(fs)?,
            None => Font::default(),
        };

        Ok(Elision {
            value: require_kwarg(list, "value")?,
            font,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// Extend
// ============================================================================

impl ToSexpr for Extend {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("extend").kwarg_opt("type", &self.r#type);

        // Position (only emit if has content)
        let pos = &self.position;
        if pos.default_x.is_some()
            || pos.default_y.is_some()
            || pos.relative_x.is_some()
            || pos.relative_y.is_some()
        {
            builder = builder.kwarg_raw("position", pos.to_sexpr());
        }

        builder = builder.kwarg_opt("color", &self.color);

        builder.build()
    }
}

impl FromSexpr for Extend {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("extend list", sexpr))?;

        expect_head(list, "extend")?;

        let position = match find_kwarg(list, "position") {
            Some(ps) => Position::from_sexpr(ps)?,
            None => Position::default(),
        };

        Ok(Extend {
            r#type: optional_kwarg(list, "type")?,
            position,
            color: optional_kwarg(list, "color")?,
        })
    }
}

// ============================================================================
// LyricExtension
// ============================================================================

impl ToSexpr for LyricExtension {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("lyric-extension")
            .kwarg_raw("elision", self.elision.to_sexpr())
            .kwarg_opt("syllabic", &self.syllabic)
            .kwarg_raw("text", self.text.to_sexpr())
            .build()
    }
}

impl FromSexpr for LyricExtension {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("lyric-extension list", sexpr))?;

        expect_head(list, "lyric-extension")?;

        Ok(LyricExtension {
            elision: require_kwarg(list, "elision")?,
            syllabic: optional_kwarg(list, "syllabic")?,
            text: require_kwarg(list, "text")?,
        })
    }
}

// ============================================================================
// LyricContent
// ============================================================================

impl ToSexpr for LyricContent {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            LyricContent::Syllable {
                syllabic,
                text,
                extensions,
                extend,
            } => {
                let mut builder = ListBuilder::new("syllable")
                    .kwarg_opt("syllabic", syllabic)
                    .kwarg_raw("text", text.to_sexpr());

                if !extensions.is_empty() {
                    builder = builder.kwarg_list("extensions", extensions);
                }

                if let Some(ext) = extend {
                    builder = builder.kwarg_raw("extend", ext.to_sexpr());
                }

                builder.build()
            }
            LyricContent::ExtendOnly(extend) => ListBuilder::new("extend-only")
                .kwarg_raw("extend", extend.to_sexpr())
                .build(),
            LyricContent::Laughing => ListBuilder::new("laughing").build(),
            LyricContent::Humming => ListBuilder::new("humming").build(),
        }
    }
}

impl FromSexpr for LyricContent {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("lyric-content", sexpr))?;

        match get_head(list)? {
            "syllable" => Ok(LyricContent::Syllable {
                syllabic: optional_kwarg(list, "syllabic")?,
                text: require_kwarg(list, "text")?,
                extensions: optional_kwarg::<Vec<LyricExtension>>(list, "extensions")?
                    .unwrap_or_default(),
                extend: optional_kwarg(list, "extend")?,
            }),
            "extend-only" => {
                let extend: Extend = require_kwarg(list, "extend")?;
                Ok(LyricContent::ExtendOnly(extend))
            }
            "laughing" => Ok(LyricContent::Laughing),
            "humming" => Ok(LyricContent::Humming),
            _ => Err(ConvertError::type_mismatch("lyric-content variant", sexpr)),
        }
    }
}

// ============================================================================
// Lyric
// ============================================================================

impl ToSexpr for Lyric {
    fn to_sexpr(&self) -> Sexpr {
        let mut builder = ListBuilder::new("lyric")
            .kwarg_opt("number", &self.number)
            .kwarg_opt("name", &self.name)
            .kwarg_opt("justify", &self.justify)
            .kwarg_opt("placement", &self.placement)
            .kwarg_opt("print-object", &self.print_object)
            .kwarg_raw("content", self.content.to_sexpr());

        if self.end_line {
            builder = builder.kwarg("end-line", &true);
        }
        if self.end_paragraph {
            builder = builder.kwarg("end-paragraph", &true);
        }

        builder.build()
    }
}

impl FromSexpr for Lyric {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr
            .as_list()
            .ok_or_else(|| ConvertError::type_mismatch("lyric list", sexpr))?;

        expect_head(list, "lyric")?;

        Ok(Lyric {
            number: optional_kwarg(list, "number")?,
            name: optional_kwarg(list, "name")?,
            justify: optional_kwarg(list, "justify")?,
            placement: optional_kwarg(list, "placement")?,
            print_object: optional_kwarg(list, "print-object")?,
            content: require_kwarg(list, "content")?,
            end_line: optional_kwarg::<bool>(list, "end-line")?.unwrap_or(false),
            end_paragraph: optional_kwarg::<bool>(list, "end-paragraph")?.unwrap_or(false),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::{AboveBelow, Font, LeftCenterRight, StartStopContinue, YesNo};
    use crate::sexpr::print_sexpr;

    // === Syllabic Tests ===

    #[test]
    fn test_syllabic_round_trip() {
        for syllabic in [
            Syllabic::Single,
            Syllabic::Begin,
            Syllabic::End,
            Syllabic::Middle,
        ] {
            let sexpr = syllabic.to_sexpr();
            let parsed = Syllabic::from_sexpr(&sexpr).unwrap();
            assert_eq!(syllabic, parsed);
        }
    }

    #[test]
    fn test_syllabic_to_sexpr() {
        assert_eq!(Syllabic::Single.to_sexpr(), Sexpr::symbol("single"));
        assert_eq!(Syllabic::Begin.to_sexpr(), Sexpr::symbol("begin"));
        assert_eq!(Syllabic::End.to_sexpr(), Sexpr::symbol("end"));
        assert_eq!(Syllabic::Middle.to_sexpr(), Sexpr::symbol("middle"));
    }

    #[test]
    fn test_syllabic_from_sexpr_invalid() {
        let sexpr = Sexpr::symbol("invalid");
        assert!(Syllabic::from_sexpr(&sexpr).is_err());
    }

    // === TextElementData Tests ===

    #[test]
    fn test_text_element_data_basic_round_trip() {
        let text = TextElementData {
            value: "la".to_string(),
            font: Font::default(),
            color: None,
            lang: None,
        };

        let sexpr = text.to_sexpr();
        let text_str = print_sexpr(&sexpr);
        assert!(text_str.contains("text"));
        assert!(text_str.contains(":value"));

        let parsed = TextElementData::from_sexpr(&sexpr).unwrap();
        assert_eq!(text.value, parsed.value);
    }

    #[test]
    fn test_text_element_data_with_color() {
        let text = TextElementData {
            value: "word".to_string(),
            font: Font::default(),
            color: Some("#000000".to_string()),
            lang: None,
        };

        let sexpr = text.to_sexpr();
        let text_str = print_sexpr(&sexpr);
        assert!(text_str.contains("#000000"));

        let parsed = TextElementData::from_sexpr(&sexpr).unwrap();
        assert_eq!(text.color, parsed.color);
    }

    #[test]
    fn test_text_element_data_with_lang() {
        let text = TextElementData {
            value: "Hallo".to_string(),
            font: Font::default(),
            color: None,
            lang: Some("de".to_string()),
        };

        let sexpr = text.to_sexpr();
        let parsed = TextElementData::from_sexpr(&sexpr).unwrap();
        assert_eq!(text.lang, parsed.lang);
    }

    // === Elision Tests ===

    #[test]
    fn test_elision_space_round_trip() {
        let elision = Elision {
            value: " ".to_string(),
            font: Font::default(),
            color: None,
        };

        let sexpr = elision.to_sexpr();
        let parsed = Elision::from_sexpr(&sexpr).unwrap();
        assert_eq!(elision.value, parsed.value);
    }

    #[test]
    fn test_elision_with_color() {
        let elision = Elision {
            value: "_".to_string(),
            font: Font::default(),
            color: Some("#808080".to_string()),
        };

        let sexpr = elision.to_sexpr();
        let parsed = Elision::from_sexpr(&sexpr).unwrap();
        assert_eq!(elision.color, parsed.color);
    }

    // === Extend Tests ===

    #[test]
    fn test_extend_default_round_trip() {
        let extend = Extend::default();

        let sexpr = extend.to_sexpr();
        let parsed = Extend::from_sexpr(&sexpr).unwrap();
        assert_eq!(extend.r#type, parsed.r#type);
    }

    #[test]
    fn test_extend_start_round_trip() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Start),
            position: Position::default(),
            color: None,
        };

        let sexpr = extend.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("extend"));
        assert!(text.contains("start"));

        let parsed = Extend::from_sexpr(&sexpr).unwrap();
        assert_eq!(extend.r#type, parsed.r#type);
    }

    #[test]
    fn test_extend_with_color() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Stop),
            position: Position::default(),
            color: Some("#0000FF".to_string()),
        };

        let sexpr = extend.to_sexpr();
        let parsed = Extend::from_sexpr(&sexpr).unwrap();
        assert_eq!(extend.color, parsed.color);
    }

    #[test]
    fn test_extend_with_position() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Continue),
            position: Position {
                default_x: Some(10.0),
                default_y: None,
                relative_x: None,
                relative_y: None,
            },
            color: None,
        };

        let sexpr = extend.to_sexpr();
        let parsed = Extend::from_sexpr(&sexpr).unwrap();
        assert_eq!(extend.position.default_x, parsed.position.default_x);
    }

    // === LyricExtension Tests ===

    #[test]
    fn test_lyric_extension_round_trip() {
        let extension = LyricExtension {
            elision: Elision {
                value: " ".to_string(),
                font: Font::default(),
                color: None,
            },
            syllabic: None,
            text: TextElementData {
                value: "la".to_string(),
                font: Font::default(),
                color: None,
                lang: None,
            },
        };

        let sexpr = extension.to_sexpr();
        let parsed = LyricExtension::from_sexpr(&sexpr).unwrap();
        assert_eq!(extension.elision.value, parsed.elision.value);
        assert_eq!(extension.text.value, parsed.text.value);
    }

    #[test]
    fn test_lyric_extension_with_syllabic() {
        let extension = LyricExtension {
            elision: Elision {
                value: "_".to_string(),
                font: Font::default(),
                color: None,
            },
            syllabic: Some(Syllabic::End),
            text: TextElementData {
                value: "ing".to_string(),
                font: Font::default(),
                color: None,
                lang: None,
            },
        };

        let sexpr = extension.to_sexpr();
        let parsed = LyricExtension::from_sexpr(&sexpr).unwrap();
        assert_eq!(extension.syllabic, parsed.syllabic);
    }

    // === LyricContent Tests ===

    #[test]
    fn test_lyric_content_syllable_round_trip() {
        let content = LyricContent::Syllable {
            syllabic: Some(Syllabic::Single),
            text: TextElementData {
                value: "love".to_string(),
                font: Font::default(),
                color: None,
                lang: None,
            },
            extensions: vec![],
            extend: None,
        };

        let sexpr = content.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("syllable"));
        assert!(text.contains("love"));

        let parsed = LyricContent::from_sexpr(&sexpr).unwrap();
        if let LyricContent::Syllable { syllabic, text, .. } = parsed {
            assert_eq!(syllabic, Some(Syllabic::Single));
            assert_eq!(text.value, "love");
        } else {
            panic!("Expected Syllable variant");
        }
    }

    #[test]
    fn test_lyric_content_syllable_with_extend() {
        let content = LyricContent::Syllable {
            syllabic: Some(Syllabic::End),
            text: TextElementData {
                value: "py".to_string(),
                font: Font::default(),
                color: None,
                lang: None,
            },
            extensions: vec![],
            extend: Some(Extend {
                r#type: Some(StartStopContinue::Start),
                position: Position::default(),
                color: None,
            }),
        };

        let sexpr = content.to_sexpr();
        let parsed = LyricContent::from_sexpr(&sexpr).unwrap();
        if let LyricContent::Syllable { extend, .. } = parsed {
            assert!(extend.is_some());
        } else {
            panic!("Expected Syllable variant");
        }
    }

    #[test]
    fn test_lyric_content_extend_only() {
        let content = LyricContent::ExtendOnly(Extend {
            r#type: Some(StartStopContinue::Continue),
            position: Position::default(),
            color: None,
        });

        let sexpr = content.to_sexpr();
        let parsed = LyricContent::from_sexpr(&sexpr).unwrap();
        if let LyricContent::ExtendOnly(extend) = parsed {
            assert_eq!(extend.r#type, Some(StartStopContinue::Continue));
        } else {
            panic!("Expected ExtendOnly variant");
        }
    }

    #[test]
    fn test_lyric_content_laughing() {
        let content = LyricContent::Laughing;

        let sexpr = content.to_sexpr();
        let parsed = LyricContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed, LyricContent::Laughing);
    }

    #[test]
    fn test_lyric_content_humming() {
        let content = LyricContent::Humming;

        let sexpr = content.to_sexpr();
        let parsed = LyricContent::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed, LyricContent::Humming);
    }

    // === Lyric Tests ===

    #[test]
    fn test_lyric_basic_round_trip() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Single),
                text: TextElementData {
                    value: "word".to_string(),
                    font: Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        let sexpr = lyric.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains("lyric"));
        assert!(text.contains(":number"));

        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert_eq!(lyric.number, parsed.number);
        assert!(!parsed.end_line);
        assert!(!parsed.end_paragraph);
    }

    #[test]
    fn test_lyric_with_name() {
        let lyric = Lyric {
            number: Some("2".to_string()),
            name: Some("Verse 2".to_string()),
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Single),
                text: TextElementData {
                    value: "second".to_string(),
                    font: Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        let sexpr = lyric.to_sexpr();
        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert_eq!(lyric.name, parsed.name);
    }

    #[test]
    fn test_lyric_with_placement() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: Some(LeftCenterRight::Left),
            placement: Some(AboveBelow::Below),
            print_object: Some(YesNo::Yes),
            content: LyricContent::Syllable {
                syllabic: None,
                text: TextElementData {
                    value: "text".to_string(),
                    font: Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };

        let sexpr = lyric.to_sexpr();
        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert_eq!(lyric.placement, parsed.placement);
        assert_eq!(lyric.justify, parsed.justify);
    }

    #[test]
    fn test_lyric_end_line() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::End),
                text: TextElementData {
                    value: "line".to_string(),
                    font: Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: true,
            end_paragraph: false,
        };

        let sexpr = lyric.to_sexpr();
        let text = print_sexpr(&sexpr);
        assert!(text.contains(":end-line"));

        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert!(parsed.end_line);
        assert!(!parsed.end_paragraph);
    }

    #[test]
    fn test_lyric_end_paragraph() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::End),
                text: TextElementData {
                    value: "end".to_string(),
                    font: Font::default(),
                    color: None,
                    lang: None,
                },
                extensions: vec![],
                extend: None,
            },
            end_line: true,
            end_paragraph: true,
        };

        let sexpr = lyric.to_sexpr();
        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert!(parsed.end_line);
        assert!(parsed.end_paragraph);
    }

    #[test]
    fn test_lyric_laughing_content() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Laughing,
            end_line: false,
            end_paragraph: false,
        };

        let sexpr = lyric.to_sexpr();
        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.content, LyricContent::Laughing);
    }

    #[test]
    fn test_lyric_humming_content() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: None,
            justify: None,
            placement: None,
            print_object: None,
            content: LyricContent::Humming,
            end_line: false,
            end_paragraph: false,
        };

        let sexpr = lyric.to_sexpr();
        let parsed = Lyric::from_sexpr(&sexpr).unwrap();
        assert_eq!(parsed.content, LyricContent::Humming);
    }
}
