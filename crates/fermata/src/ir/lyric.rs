//! Lyric types.

use super::common::{AboveBelow, Color, Font, LeftCenterRight, Position, StartStopContinue, YesNo};

/// Lyric element attached to a note.
#[derive(Debug, Clone, PartialEq)]
pub struct Lyric {
    /// Lyric number (for multiple verses)
    pub number: Option<String>,
    /// Lyric name
    pub name: Option<String>,
    /// Text justification
    pub justify: Option<LeftCenterRight>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// Lyric content
    pub content: LyricContent,
    /// End of line
    pub end_line: bool,
    /// End of paragraph
    pub end_paragraph: bool,
}

/// Lyric content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum LyricContent {
    /// A syllable with text
    Syllable {
        /// Syllabic position
        syllabic: Option<Syllabic>,
        /// Text element
        text: TextElementData,
        /// Additional syllables after elisions
        extensions: Vec<LyricExtension>,
        /// Extending line
        extend: Option<Extend>,
    },
    /// Just an extending line
    ExtendOnly(Extend),
    /// Laughing indication
    Laughing,
    /// Humming indication
    Humming,
}

/// Syllabic position in word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syllabic {
    /// Single syllable word
    Single,
    /// Beginning of multi-syllable word
    Begin,
    /// End of multi-syllable word
    End,
    /// Middle of multi-syllable word
    Middle,
}

/// Additional syllable after elision.
#[derive(Debug, Clone, PartialEq)]
pub struct LyricExtension {
    /// Elision element
    pub elision: Elision,
    /// Syllabic position
    pub syllabic: Option<Syllabic>,
    /// Text element
    pub text: TextElementData,
}

/// Text element with formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct TextElementData {
    /// The text value
    pub value: String,
    /// Font attributes
    pub font: Font,
    /// Text color
    pub color: Option<Color>,
    /// Language code
    pub lang: Option<String>,
}

/// Elision between syllables.
#[derive(Debug, Clone, PartialEq)]
pub struct Elision {
    /// The elision value (character to display)
    pub value: String,
    /// Font attributes
    pub font: Font,
    /// Color
    pub color: Option<Color>,
}

/// Lyric extension line.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Extend {
    /// Start, stop, or continue
    pub r#type: Option<StartStopContinue>,
    /// Position attributes
    pub position: Position,
    /// Line color
    pub color: Option<Color>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Syllabic Tests ===

    #[test]
    fn test_syllabic_all_variants() {
        assert_eq!(Syllabic::Single, Syllabic::Single);
        assert_eq!(Syllabic::Begin, Syllabic::Begin);
        assert_eq!(Syllabic::End, Syllabic::End);
        assert_eq!(Syllabic::Middle, Syllabic::Middle);
    }

    #[test]
    fn test_syllabic_inequality() {
        assert_ne!(Syllabic::Single, Syllabic::Begin);
        assert_ne!(Syllabic::Begin, Syllabic::End);
        assert_ne!(Syllabic::End, Syllabic::Middle);
    }

    #[test]
    fn test_syllabic_clone() {
        let syllabic = Syllabic::Begin;
        let cloned = syllabic.clone();
        assert_eq!(syllabic, cloned);
    }

    #[test]
    fn test_syllabic_copy() {
        let syllabic = Syllabic::End;
        let copied = syllabic;
        assert_eq!(syllabic, copied);
    }

    #[test]
    fn test_syllabic_debug() {
        assert_eq!(format!("{:?}", Syllabic::Single), "Single");
        assert_eq!(format!("{:?}", Syllabic::Begin), "Begin");
        assert_eq!(format!("{:?}", Syllabic::End), "End");
        assert_eq!(format!("{:?}", Syllabic::Middle), "Middle");
    }

    // === TextElementData Tests ===

    #[test]
    fn test_textelementdata_basic() {
        let text = TextElementData {
            value: "la".to_string(),
            font: Font::default(),
            color: None,
            lang: None,
        };
        assert_eq!(text.value, "la");
        assert!(text.color.is_none());
        assert!(text.lang.is_none());
    }

    #[test]
    fn test_textelementdata_with_color() {
        let text = TextElementData {
            value: "word".to_string(),
            font: Font::default(),
            color: Some("#000000".to_string()),
            lang: None,
        };
        assert_eq!(text.color, Some("#000000".to_string()));
    }

    #[test]
    fn test_textelementdata_with_lang() {
        let text = TextElementData {
            value: "Hallo".to_string(),
            font: Font::default(),
            color: None,
            lang: Some("de".to_string()),
        };
        assert_eq!(text.lang, Some("de".to_string()));
    }

    #[test]
    fn test_textelementdata_clone() {
        let text = TextElementData {
            value: "test".to_string(),
            font: Font::default(),
            color: Some("#FF0000".to_string()),
            lang: Some("en".to_string()),
        };
        let cloned = text.clone();
        assert_eq!(text, cloned);
    }

    // === Elision Tests ===

    #[test]
    fn test_elision_space() {
        let elision = Elision {
            value: " ".to_string(),
            font: Font::default(),
            color: None,
        };
        assert_eq!(elision.value, " ");
    }

    #[test]
    fn test_elision_underscore() {
        let elision = Elision {
            value: "_".to_string(),
            font: Font::default(),
            color: None,
        };
        assert_eq!(elision.value, "_");
    }

    #[test]
    fn test_elision_with_color() {
        let elision = Elision {
            value: " ".to_string(),
            font: Font::default(),
            color: Some("#808080".to_string()),
        };
        assert_eq!(elision.color, Some("#808080".to_string()));
    }

    #[test]
    fn test_elision_clone() {
        let elision = Elision {
            value: "-".to_string(),
            font: Font::default(),
            color: None,
        };
        let cloned = elision.clone();
        assert_eq!(elision, cloned);
    }

    // === Extend Tests ===

    #[test]
    fn test_extend_default() {
        let extend = Extend::default();
        assert!(extend.r#type.is_none());
        assert_eq!(extend.position, Position::default());
        assert!(extend.color.is_none());
    }

    #[test]
    fn test_extend_start() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Start),
            position: Position::default(),
            color: None,
        };
        assert_eq!(extend.r#type, Some(StartStopContinue::Start));
    }

    #[test]
    fn test_extend_stop() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Stop),
            position: Position::default(),
            color: None,
        };
        assert_eq!(extend.r#type, Some(StartStopContinue::Stop));
    }

    #[test]
    fn test_extend_continue() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Continue),
            position: Position::default(),
            color: None,
        };
        assert_eq!(extend.r#type, Some(StartStopContinue::Continue));
    }

    #[test]
    fn test_extend_with_color() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Start),
            position: Position::default(),
            color: Some("#0000FF".to_string()),
        };
        assert_eq!(extend.color, Some("#0000FF".to_string()));
    }

    #[test]
    fn test_extend_clone() {
        let extend = Extend {
            r#type: Some(StartStopContinue::Continue),
            position: Position {
                default_x: Some(10.0),
                ..Default::default()
            },
            color: Some("#00FF00".to_string()),
        };
        let cloned = extend.clone();
        assert_eq!(extend, cloned);
    }

    // === LyricExtension Tests ===

    #[test]
    fn test_lyricextension_basic() {
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
        assert_eq!(extension.elision.value, " ");
        assert_eq!(extension.text.value, "la");
    }

    #[test]
    fn test_lyricextension_with_syllabic() {
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
        assert_eq!(extension.syllabic, Some(Syllabic::End));
    }

    #[test]
    fn test_lyricextension_clone() {
        let extension = LyricExtension {
            elision: Elision {
                value: " ".to_string(),
                font: Font::default(),
                color: None,
            },
            syllabic: Some(Syllabic::Middle),
            text: TextElementData {
                value: "mi".to_string(),
                font: Font::default(),
                color: None,
                lang: None,
            },
        };
        let cloned = extension.clone();
        assert_eq!(extension, cloned);
    }

    // === LyricContent Tests ===

    #[test]
    fn test_lyriccontent_syllable_single() {
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
        if let LyricContent::Syllable { syllabic, text, .. } = content {
            assert_eq!(syllabic, Some(Syllabic::Single));
            assert_eq!(text.value, "love");
        } else {
            panic!("Expected Syllable variant");
        }
    }

    #[test]
    fn test_lyriccontent_syllable_begin() {
        let content = LyricContent::Syllable {
            syllabic: Some(Syllabic::Begin),
            text: TextElementData {
                value: "hap".to_string(),
                font: Font::default(),
                color: None,
                lang: None,
            },
            extensions: vec![],
            extend: None,
        };
        if let LyricContent::Syllable { syllabic, .. } = content {
            assert_eq!(syllabic, Some(Syllabic::Begin));
        } else {
            panic!("Expected Syllable variant");
        }
    }

    #[test]
    fn test_lyriccontent_syllable_with_extend() {
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
        if let LyricContent::Syllable { extend, .. } = content {
            assert!(extend.is_some());
        } else {
            panic!("Expected Syllable variant");
        }
    }

    #[test]
    fn test_lyriccontent_extend_only() {
        let content = LyricContent::ExtendOnly(Extend {
            r#type: Some(StartStopContinue::Continue),
            position: Position::default(),
            color: None,
        });
        if let LyricContent::ExtendOnly(extend) = content {
            assert_eq!(extend.r#type, Some(StartStopContinue::Continue));
        } else {
            panic!("Expected ExtendOnly variant");
        }
    }

    #[test]
    fn test_lyriccontent_laughing() {
        let content = LyricContent::Laughing;
        assert_eq!(content, LyricContent::Laughing);
    }

    #[test]
    fn test_lyriccontent_humming() {
        let content = LyricContent::Humming;
        assert_eq!(content, LyricContent::Humming);
    }

    // === Lyric Tests ===

    #[test]
    fn test_lyric_basic() {
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
        assert_eq!(lyric.number, Some("1".to_string()));
        assert!(!lyric.end_line);
        assert!(!lyric.end_paragraph);
    }

    #[test]
    fn test_lyric_verse_2() {
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
        assert_eq!(lyric.number, Some("2".to_string()));
        assert_eq!(lyric.name, Some("Verse 2".to_string()));
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
        assert_eq!(lyric.placement, Some(AboveBelow::Below));
        assert_eq!(lyric.justify, Some(LeftCenterRight::Left));
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
        assert!(lyric.end_line);
        assert!(!lyric.end_paragraph);
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
        assert!(lyric.end_line);
        assert!(lyric.end_paragraph);
    }

    #[test]
    fn test_lyric_clone() {
        let lyric = Lyric {
            number: Some("1".to_string()),
            name: Some("Chorus".to_string()),
            justify: Some(LeftCenterRight::Center),
            placement: Some(AboveBelow::Below),
            print_object: Some(YesNo::Yes),
            content: LyricContent::Syllable {
                syllabic: Some(Syllabic::Single),
                text: TextElementData {
                    value: "sing".to_string(),
                    font: Font::default(),
                    color: Some("#000000".to_string()),
                    lang: Some("en".to_string()),
                },
                extensions: vec![],
                extend: None,
            },
            end_line: false,
            end_paragraph: false,
        };
        let cloned = lyric.clone();
        assert_eq!(lyric, cloned);
    }
}
