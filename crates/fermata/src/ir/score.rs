//! Score-level types.

use super::common::{
    Font, Identification, LeftCenterRight, Position, PrintStyle, StaffNumber, Tenths,
    TopMiddleBottom, YesNo,
};
use super::part::{Part, PartList};

/// The root score-partwise element.
#[derive(Debug, Clone, PartialEq)]
pub struct ScorePartwise {
    /// MusicXML version
    pub version: Option<String>,
    /// Work information
    pub work: Option<Work>,
    /// Movement number
    pub movement_number: Option<String>,
    /// Movement title
    pub movement_title: Option<String>,
    /// Identification
    pub identification: Option<Identification>,
    /// Score defaults
    pub defaults: Option<Defaults>,
    /// Credits
    pub credits: Vec<Credit>,
    /// Part list
    pub part_list: PartList,
    /// Parts
    pub parts: Vec<Part>,
}

/// Work information.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Work {
    /// Work number
    pub work_number: Option<String>,
    /// Work title
    pub work_title: Option<String>,
    /// Opus reference
    pub opus: Option<Opus>,
}

/// Opus reference.
#[derive(Debug, Clone, PartialEq)]
pub struct Opus {
    /// XLink href
    pub href: String,
}

// Identification and related types are defined in common.rs

/// Score defaults (layout, scaling, fonts).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Defaults {
    /// Scaling
    pub scaling: Option<Scaling>,
    /// Page layout
    pub page_layout: Option<PageLayout>,
    /// System layout
    pub system_layout: Option<SystemLayout>,
    /// Staff layouts
    pub staff_layout: Vec<StaffLayout>,
    /// Appearance
    pub appearance: Option<Appearance>,
    /// Music font
    pub music_font: Option<Font>,
    /// Word font
    pub word_font: Option<Font>,
    /// Lyric fonts
    pub lyric_fonts: Vec<LyricFont>,
    /// Lyric languages
    pub lyric_languages: Vec<LyricLanguage>,
}

/// Scaling.
#[derive(Debug, Clone, PartialEq)]
pub struct Scaling {
    /// Millimeters per tenths
    pub millimeters: f64,
    /// Tenths value
    pub tenths: f64,
}

/// Page layout.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageLayout {
    /// Page height
    pub page_height: Option<Tenths>,
    /// Page width
    pub page_width: Option<Tenths>,
    /// Page margins
    pub page_margins: Vec<PageMargins>,
}

/// Page margins.
#[derive(Debug, Clone, PartialEq)]
pub struct PageMargins {
    /// Margin type
    pub r#type: Option<MarginType>,
    /// Left margin
    pub left: Tenths,
    /// Right margin
    pub right: Tenths,
    /// Top margin
    pub top: Tenths,
    /// Bottom margin
    pub bottom: Tenths,
}

/// Margin types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarginType {
    /// Odd pages
    Odd,
    /// Even pages
    Even,
    /// Both odd and even
    Both,
}

/// System layout.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemLayout {
    /// System margins
    pub system_margins: Option<SystemMargins>,
    /// System distance
    pub system_distance: Option<Tenths>,
    /// Top system distance
    pub top_system_distance: Option<Tenths>,
    /// System dividers
    pub system_dividers: Option<SystemDividers>,
}

/// System margins.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemMargins {
    /// Left margin
    pub left: Tenths,
    /// Right margin
    pub right: Tenths,
}

/// System dividers.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemDividers {
    /// Left divider
    pub left_divider: Option<Divider>,
    /// Right divider
    pub right_divider: Option<Divider>,
}

/// Divider.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Divider {
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Staff layout.
#[derive(Debug, Clone, PartialEq)]
pub struct StaffLayout {
    /// Staff number
    pub number: Option<StaffNumber>,
    /// Staff distance
    pub staff_distance: Option<Tenths>,
}

/// Appearance settings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Appearance {
    /// Line widths
    pub line_widths: Vec<LineWidth>,
    /// Note sizes
    pub note_sizes: Vec<NoteSize>,
    /// Distances
    pub distances: Vec<Distance>,
    /// Other appearances
    pub other_appearances: Vec<OtherAppearance>,
}

/// Line width.
#[derive(Debug, Clone, PartialEq)]
pub struct LineWidth {
    /// Line type
    pub r#type: String,
    /// Width value
    pub value: Tenths,
}

/// Note size.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteSize {
    /// Note size type
    pub r#type: NoteSizeType,
    /// Size percentage
    pub value: f64,
}

/// Note size types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteSizeType {
    /// Cue notes
    Cue,
    /// Grace notes
    Grace,
    /// Grace-cue notes
    GraceCue,
    /// Large notes
    Large,
}

/// Distance.
#[derive(Debug, Clone, PartialEq)]
pub struct Distance {
    /// Distance type
    pub r#type: String,
    /// Distance value
    pub value: Tenths,
}

/// Other appearance.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherAppearance {
    /// Appearance type
    pub r#type: String,
    /// Appearance value
    pub value: String,
}

/// Lyric font.
#[derive(Debug, Clone, PartialEq)]
pub struct LyricFont {
    /// Lyric number
    pub number: Option<String>,
    /// Lyric name
    pub name: Option<String>,
    /// Font attributes
    pub font: Font,
}

/// Lyric language.
#[derive(Debug, Clone, PartialEq)]
pub struct LyricLanguage {
    /// Lyric number
    pub number: Option<String>,
    /// Lyric name
    pub name: Option<String>,
    /// Language code
    pub lang: String,
}

/// Credit for title, composer, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Credit {
    /// Page number
    pub page: Option<u32>,
    /// Credit content
    pub content: Vec<CreditContent>,
}

/// Credit content.
#[derive(Debug, Clone, PartialEq)]
pub enum CreditContent {
    /// Credit type
    CreditType(String),
    /// Link
    Link(Link),
    /// Bookmark
    Bookmark(Bookmark),
    /// Credit image
    CreditImage(CreditImage),
    /// Credit words
    CreditWords(CreditWords),
    /// Credit symbol
    CreditSymbol(CreditSymbol),
}

/// Link element.
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    /// XLink href
    pub href: String,
    /// XLink type
    pub r#type: Option<String>,
    /// XLink role
    pub role: Option<String>,
    /// XLink title
    pub title: Option<String>,
    /// XLink show
    pub show: Option<String>,
    /// XLink actuate
    pub actuate: Option<String>,
    /// Element name
    pub name: Option<String>,
    /// Element reference
    pub element: Option<String>,
    /// Position
    pub position: Option<u32>,
}

/// Bookmark element.
#[derive(Debug, Clone, PartialEq)]
pub struct Bookmark {
    /// Bookmark ID
    pub id: String,
    /// Bookmark name
    pub name: Option<String>,
    /// Element reference
    pub element: Option<String>,
    /// Position
    pub position: Option<u32>,
}

/// Credit image.
#[derive(Debug, Clone, PartialEq)]
pub struct CreditImage {
    /// Image source
    pub source: String,
    /// Image MIME type
    pub r#type: String,
    /// Position attributes
    pub position: Position,
}

/// Credit words.
#[derive(Debug, Clone, PartialEq)]
pub struct CreditWords {
    /// The text value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Text justification
    pub justify: Option<LeftCenterRight>,
    /// Horizontal alignment
    pub halign: Option<LeftCenterRight>,
    /// Vertical alignment
    pub valign: Option<TopMiddleBottom>,
    /// Language code
    pub lang: Option<String>,
}

/// Credit symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct CreditSymbol {
    /// The symbol value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Symbol justification
    pub justify: Option<LeftCenterRight>,
    /// Horizontal alignment
    pub halign: Option<LeftCenterRight>,
    /// Vertical alignment
    pub valign: Option<TopMiddleBottom>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::part::{PartName, ScorePart};

    // === ScorePartwise Tests ===

    #[test]
    fn test_scorepartwise_basic() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        };
        assert_eq!(score.version, Some("4.0".to_string()));
        assert!(score.parts.is_empty());
    }

    #[test]
    fn test_scorepartwise_with_work() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: Some(Work {
                work_number: Some("Op. 1".to_string()),
                work_title: Some("Symphony No. 1".to_string()),
                opus: None,
            }),
            movement_number: Some("1".to_string()),
            movement_title: Some("Allegro".to_string()),
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        };
        assert!(score.work.is_some());
        assert_eq!(score.movement_number, Some("1".to_string()));
    }

    #[test]
    fn test_scorepartwise_with_parts() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList {
                content: vec![crate::ir::part::PartListElement::ScorePart(ScorePart {
                    id: "P1".to_string(),
                    identification: None,
                    part_name: PartName {
                        value: "Piano".to_string(),
                        print_style: PrintStyle::default(),
                        print_object: None,
                        justify: None,
                    },
                    part_name_display: None,
                    part_abbreviation: None,
                    part_abbreviation_display: None,
                    group: vec![],
                    score_instruments: vec![],
                    midi_devices: vec![],
                    midi_instruments: vec![],
                })],
            },
            parts: vec![Part {
                id: "P1".to_string(),
                measures: vec![],
            }],
        };
        assert_eq!(score.parts.len(), 1);
    }

    #[test]
    fn test_scorepartwise_clone() {
        let score = ScorePartwise {
            version: Some("4.0".to_string()),
            work: None,
            movement_number: None,
            movement_title: None,
            identification: None,
            defaults: None,
            credits: vec![],
            part_list: PartList { content: vec![] },
            parts: vec![],
        };
        let cloned = score.clone();
        assert_eq!(score, cloned);
    }

    // === Work Tests ===

    #[test]
    fn test_work_default() {
        let work = Work::default();
        assert!(work.work_number.is_none());
        assert!(work.work_title.is_none());
        assert!(work.opus.is_none());
    }

    #[test]
    fn test_work_with_title() {
        let work = Work {
            work_number: None,
            work_title: Some("Nocturne".to_string()),
            opus: None,
        };
        assert_eq!(work.work_title, Some("Nocturne".to_string()));
    }

    #[test]
    fn test_work_with_opus() {
        let work = Work {
            work_number: Some("Op. 9, No. 2".to_string()),
            work_title: Some("Nocturne in E-flat Major".to_string()),
            opus: Some(Opus {
                href: "opus9.xml".to_string(),
            }),
        };
        assert!(work.opus.is_some());
    }

    // === Opus Tests ===

    #[test]
    fn test_opus_basic() {
        let opus = Opus {
            href: "complete-works.xml".to_string(),
        };
        assert_eq!(opus.href, "complete-works.xml");
    }

    // === Defaults Tests ===

    #[test]
    fn test_defaults_default() {
        let defaults = Defaults::default();
        assert!(defaults.scaling.is_none());
        assert!(defaults.page_layout.is_none());
        assert!(defaults.system_layout.is_none());
        assert!(defaults.staff_layout.is_empty());
        assert!(defaults.appearance.is_none());
    }

    #[test]
    fn test_defaults_with_scaling() {
        let defaults = Defaults {
            scaling: Some(Scaling {
                millimeters: 7.056,
                tenths: 40.0,
            }),
            ..Default::default()
        };
        assert!(defaults.scaling.is_some());
    }

    #[test]
    fn test_defaults_with_fonts() {
        let defaults = Defaults {
            music_font: Some(Font {
                font_family: Some("Bravura".to_string()),
                ..Default::default()
            }),
            word_font: Some(Font {
                font_family: Some("Times New Roman".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(defaults.music_font.is_some());
        assert!(defaults.word_font.is_some());
    }

    // === Scaling Tests ===

    #[test]
    fn test_scaling_basic() {
        let scaling = Scaling {
            millimeters: 7.056,
            tenths: 40.0,
        };
        assert_eq!(scaling.millimeters, 7.056);
        assert_eq!(scaling.tenths, 40.0);
    }

    // === PageLayout Tests ===

    #[test]
    fn test_pagelayout_default() {
        let layout = PageLayout::default();
        assert!(layout.page_height.is_none());
        assert!(layout.page_width.is_none());
        assert!(layout.page_margins.is_empty());
    }

    #[test]
    fn test_pagelayout_with_dimensions() {
        let layout = PageLayout {
            page_height: Some(1683.36),
            page_width: Some(1190.88),
            page_margins: vec![],
        };
        assert_eq!(layout.page_height, Some(1683.36));
        assert_eq!(layout.page_width, Some(1190.88));
    }

    #[test]
    fn test_pagelayout_with_margins() {
        let layout = PageLayout {
            page_height: Some(1683.36),
            page_width: Some(1190.88),
            page_margins: vec![PageMargins {
                r#type: Some(MarginType::Both),
                left: 70.0,
                right: 70.0,
                top: 88.0,
                bottom: 88.0,
            }],
        };
        assert_eq!(layout.page_margins.len(), 1);
    }

    // === PageMargins Tests ===

    #[test]
    fn test_pagemargins_basic() {
        let margins = PageMargins {
            r#type: Some(MarginType::Both),
            left: 70.0,
            right: 70.0,
            top: 88.0,
            bottom: 88.0,
        };
        assert_eq!(margins.left, 70.0);
        assert_eq!(margins.right, 70.0);
    }

    #[test]
    fn test_pagemargins_odd_even() {
        let odd = PageMargins {
            r#type: Some(MarginType::Odd),
            left: 70.0,
            right: 50.0,
            top: 88.0,
            bottom: 88.0,
        };
        let even = PageMargins {
            r#type: Some(MarginType::Even),
            left: 50.0,
            right: 70.0,
            top: 88.0,
            bottom: 88.0,
        };
        assert_eq!(odd.r#type, Some(MarginType::Odd));
        assert_eq!(even.r#type, Some(MarginType::Even));
    }

    // === MarginType Tests ===

    #[test]
    fn test_margintype_all_variants() {
        assert_eq!(MarginType::Odd, MarginType::Odd);
        assert_eq!(MarginType::Even, MarginType::Even);
        assert_eq!(MarginType::Both, MarginType::Both);
    }

    // === SystemLayout Tests ===

    #[test]
    fn test_systemlayout_default() {
        let layout = SystemLayout::default();
        assert!(layout.system_margins.is_none());
        assert!(layout.system_distance.is_none());
        assert!(layout.top_system_distance.is_none());
    }

    #[test]
    fn test_systemlayout_with_margins() {
        let layout = SystemLayout {
            system_margins: Some(SystemMargins {
                left: 0.0,
                right: 0.0,
            }),
            system_distance: Some(117.0),
            top_system_distance: Some(117.0),
            system_dividers: None,
        };
        assert!(layout.system_margins.is_some());
        assert_eq!(layout.system_distance, Some(117.0));
    }

    // === SystemMargins Tests ===

    #[test]
    fn test_systemmargins_basic() {
        let margins = SystemMargins {
            left: 21.0,
            right: 21.0,
        };
        assert_eq!(margins.left, 21.0);
        assert_eq!(margins.right, 21.0);
    }

    // === SystemDividers Tests ===

    #[test]
    fn test_systemdividers_default() {
        let dividers = SystemDividers::default();
        assert!(dividers.left_divider.is_none());
        assert!(dividers.right_divider.is_none());
    }

    #[test]
    fn test_systemdividers_with_values() {
        let dividers = SystemDividers {
            left_divider: Some(Divider {
                print_object: Some(YesNo::Yes),
                print_style: PrintStyle::default(),
            }),
            right_divider: Some(Divider {
                print_object: Some(YesNo::Yes),
                print_style: PrintStyle::default(),
            }),
        };
        assert!(dividers.left_divider.is_some());
        assert!(dividers.right_divider.is_some());
    }

    // === Divider Tests ===

    #[test]
    fn test_divider_default() {
        let divider = Divider::default();
        assert!(divider.print_object.is_none());
    }

    // === StaffLayout Tests ===

    #[test]
    fn test_stafflayout_basic() {
        let layout = StaffLayout {
            number: Some(2),
            staff_distance: Some(65.0),
        };
        assert_eq!(layout.number, Some(2));
        assert_eq!(layout.staff_distance, Some(65.0));
    }

    // === Appearance Tests ===

    #[test]
    fn test_appearance_default() {
        let appearance = Appearance::default();
        assert!(appearance.line_widths.is_empty());
        assert!(appearance.note_sizes.is_empty());
        assert!(appearance.distances.is_empty());
    }

    #[test]
    fn test_appearance_with_line_widths() {
        let appearance = Appearance {
            line_widths: vec![
                LineWidth {
                    r#type: "stem".to_string(),
                    value: 0.8333,
                },
                LineWidth {
                    r#type: "beam".to_string(),
                    value: 5.0,
                },
            ],
            note_sizes: vec![],
            distances: vec![],
            other_appearances: vec![],
        };
        assert_eq!(appearance.line_widths.len(), 2);
    }

    // === LineWidth Tests ===

    #[test]
    fn test_linewidth_basic() {
        let lw = LineWidth {
            r#type: "staff".to_string(),
            value: 0.8333,
        };
        assert_eq!(lw.r#type, "staff");
        assert_eq!(lw.value, 0.8333);
    }

    // === NoteSize Tests ===

    #[test]
    fn test_notesize_cue() {
        let ns = NoteSize {
            r#type: NoteSizeType::Cue,
            value: 60.0,
        };
        assert_eq!(ns.r#type, NoteSizeType::Cue);
        assert_eq!(ns.value, 60.0);
    }

    #[test]
    fn test_notesize_grace() {
        let ns = NoteSize {
            r#type: NoteSizeType::Grace,
            value: 60.0,
        };
        assert_eq!(ns.r#type, NoteSizeType::Grace);
    }

    // === NoteSizeType Tests ===

    #[test]
    fn test_notesizetype_all_variants() {
        assert_eq!(NoteSizeType::Cue, NoteSizeType::Cue);
        assert_eq!(NoteSizeType::Grace, NoteSizeType::Grace);
        assert_eq!(NoteSizeType::GraceCue, NoteSizeType::GraceCue);
        assert_eq!(NoteSizeType::Large, NoteSizeType::Large);
    }

    // === Distance Tests ===

    #[test]
    fn test_distance_basic() {
        let dist = Distance {
            r#type: "hyphen".to_string(),
            value: 60.0,
        };
        assert_eq!(dist.r#type, "hyphen");
        assert_eq!(dist.value, 60.0);
    }

    // === OtherAppearance Tests ===

    #[test]
    fn test_otherappearance_basic() {
        let oa = OtherAppearance {
            r#type: "custom".to_string(),
            value: "value".to_string(),
        };
        assert_eq!(oa.r#type, "custom");
    }

    // === LyricFont Tests ===

    #[test]
    fn test_lyricfont_basic() {
        let lf = LyricFont {
            number: Some("1".to_string()),
            name: Some("verse".to_string()),
            font: Font {
                font_family: Some("Times New Roman".to_string()),
                ..Default::default()
            },
        };
        assert_eq!(lf.number, Some("1".to_string()));
    }

    // === LyricLanguage Tests ===

    #[test]
    fn test_lyriclanguage_basic() {
        let ll = LyricLanguage {
            number: Some("1".to_string()),
            name: None,
            lang: "en".to_string(),
        };
        assert_eq!(ll.lang, "en");
    }

    // === Credit Tests ===

    #[test]
    fn test_credit_basic() {
        let credit = Credit {
            page: Some(1),
            content: vec![],
        };
        assert_eq!(credit.page, Some(1));
    }

    #[test]
    fn test_credit_with_words() {
        let credit = Credit {
            page: Some(1),
            content: vec![
                CreditContent::CreditType("title".to_string()),
                CreditContent::CreditWords(CreditWords {
                    value: "Symphony No. 5".to_string(),
                    print_style: PrintStyle::default(),
                    justify: Some(LeftCenterRight::Center),
                    halign: Some(LeftCenterRight::Center),
                    valign: Some(TopMiddleBottom::Top),
                    lang: None,
                }),
            ],
        };
        assert_eq!(credit.content.len(), 2);
    }

    // === CreditContent Tests ===

    #[test]
    fn test_creditcontent_credit_type() {
        let content = CreditContent::CreditType("composer".to_string());
        if let CreditContent::CreditType(ct) = content {
            assert_eq!(ct, "composer");
        }
    }

    #[test]
    fn test_creditcontent_credit_words() {
        let content = CreditContent::CreditWords(CreditWords {
            value: "Ludwig van Beethoven".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            halign: None,
            valign: None,
            lang: None,
        });
        if let CreditContent::CreditWords(cw) = content {
            assert_eq!(cw.value, "Ludwig van Beethoven");
        }
    }

    #[test]
    fn test_creditcontent_credit_image() {
        let content = CreditContent::CreditImage(CreditImage {
            source: "logo.png".to_string(),
            r#type: "image/png".to_string(),
            position: Position::default(),
        });
        if let CreditContent::CreditImage(ci) = content {
            assert_eq!(ci.source, "logo.png");
        }
    }

    // === Link Tests ===

    #[test]
    fn test_link_basic() {
        let link = Link {
            href: "http://example.com".to_string(),
            r#type: Some("simple".to_string()),
            role: None,
            title: Some("Example Link".to_string()),
            show: None,
            actuate: None,
            name: None,
            element: None,
            position: None,
        };
        assert_eq!(link.href, "http://example.com");
    }

    // === Bookmark Tests ===

    #[test]
    fn test_bookmark_basic() {
        let bookmark = Bookmark {
            id: "bookmark1".to_string(),
            name: Some("Introduction".to_string()),
            element: None,
            position: None,
        };
        assert_eq!(bookmark.id, "bookmark1");
    }

    // === CreditWords Tests ===

    #[test]
    fn test_creditwords_full() {
        let cw = CreditWords {
            value: "Title".to_string(),
            print_style: PrintStyle::default(),
            justify: Some(LeftCenterRight::Center),
            halign: Some(LeftCenterRight::Center),
            valign: Some(TopMiddleBottom::Top),
            lang: Some("en".to_string()),
        };
        assert_eq!(cw.value, "Title");
        assert_eq!(cw.justify, Some(LeftCenterRight::Center));
    }

    // === CreditSymbol Tests ===

    #[test]
    fn test_creditsymbol_basic() {
        let cs = CreditSymbol {
            value: "coda".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            halign: None,
            valign: None,
        };
        assert_eq!(cs.value, "coda");
    }
}
