//! Common types, enums, and type aliases shared across the IR.

// === Type Aliases ===

/// Tenths of staff space (MusicXML's primary unit for positioning)
pub type Tenths = f64;

/// Duration in divisions (relative to `<divisions>` in attributes)
pub type Divisions = i64;

/// Positive duration value
pub type PositiveDivisions = u64;

/// Semitones for pitch alteration (-2 to +2 typical, microtones possible)
pub type Semitones = f64;

/// Octave number (0-9, where 4 is the octave starting at middle C)
pub type Octave = u8;

/// Staff number (1-based)
pub type StaffNumber = u16;

/// Beam level (1-8, for 8th through 1024th notes)
pub type BeamLevel = u8;

/// Number level for spanning elements (1-16)
pub type NumberLevel = u8;

/// Voice identifier (string, not integer - allows "1a", custom IDs)
pub type Voice = String;

/// CSS-style color string
pub type Color = String;

/// Percentage (0.0 to 100.0)
pub type Percent = f64;

// === Common Enums ===

/// Yes or no attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YesNo {
    /// Yes
    Yes,
    /// No
    No,
}

/// Start or stop attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStop {
    /// Start
    Start,
    /// Stop
    Stop,
}

/// Start, stop, or continue attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopContinue {
    /// Start
    Start,
    /// Stop
    Stop,
    /// Continue
    Continue,
}

/// Start, stop, or single attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopSingle {
    /// Start
    Start,
    /// Stop
    Stop,
    /// Single
    Single,
}

/// Start, stop, or discontinue attribute value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartStopDiscontinue {
    /// Start
    Start,
    /// Stop
    Stop,
    /// Discontinue
    Discontinue,
}

/// Above or below placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AboveBelow {
    /// Above the staff
    Above,
    /// Below the staff
    Below,
}

/// Up or down direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpDown {
    /// Up
    Up,
    /// Down
    Down,
}

/// Over or under orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverUnder {
    /// Over
    Over,
    /// Under
    Under,
}

/// Left, center, or right alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftCenterRight {
    /// Left
    Left,
    /// Center
    Center,
    /// Right
    Right,
}

/// Top, middle, or bottom vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopMiddleBottom {
    /// Top
    Top,
    /// Middle
    Middle,
    /// Bottom
    Bottom,
}

/// Backward or forward direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackwardForward {
    /// Backward
    Backward,
    /// Forward
    Forward,
}

/// Right, left, or middle barline location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RightLeftMiddle {
    /// Right
    Right,
    /// Left
    Left,
    /// Middle
    Middle,
}

/// Upright or inverted orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UprightInverted {
    /// Upright
    Upright,
    /// Inverted
    Inverted,
}

/// Symbol size variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSize {
    /// Full size
    Full,
    /// Cue size
    Cue,
    /// Grace-cue size
    GraceCue,
    /// Large size
    Large,
}

/// Line type for slurs, ties, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    /// Solid line
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// Wavy line
    Wavy,
}

/// Font style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    /// Normal style
    Normal,
    /// Italic style
    Italic,
}

/// Font weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    /// Normal weight
    Normal,
    /// Bold weight
    Bold,
}

// === Attribute Group Structs ===

/// Position attributes for placement relative to staff.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    /// Default horizontal position in tenths
    pub default_x: Option<Tenths>,
    /// Default vertical position in tenths
    pub default_y: Option<Tenths>,
    /// Relative horizontal position in tenths
    pub relative_x: Option<Tenths>,
    /// Relative vertical position in tenths
    pub relative_y: Option<Tenths>,
}

/// Font size can be CSS size or numeric points.
#[derive(Debug, Clone, PartialEq)]
pub enum FontSize {
    /// CSS font size keyword
    Css(CssFontSize),
    /// Numeric point size
    Points(f64),
}

/// CSS font size keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFontSize {
    /// xx-small
    XxSmall,
    /// x-small
    XSmall,
    /// small
    Small,
    /// medium
    Medium,
    /// large
    Large,
    /// x-large
    XLarge,
    /// xx-large
    XxLarge,
}

/// Font attributes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Font {
    /// Font family name
    pub font_family: Option<String>,
    /// Font style (normal, italic)
    pub font_style: Option<FontStyle>,
    /// Font size
    pub font_size: Option<FontSize>,
    /// Font weight (normal, bold)
    pub font_weight: Option<FontWeight>,
}

/// Combined print-style attributes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrintStyle {
    /// Position attributes
    pub position: Position,
    /// Font attributes
    pub font: Font,
    /// Color value
    pub color: Option<Color>,
}

/// Editorial information (footnote and level).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Editorial {
    /// Footnote text
    pub footnote: Option<FormattedText>,
    /// Editorial level
    pub level: Option<Level>,
}

/// Formatted text with optional formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedText {
    /// The text value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Language code
    pub lang: Option<String>,
}

/// Level for editorial annotations.
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    /// The level value
    pub value: String,
    /// Whether this is a reference level
    pub reference: Option<YesNo>,
}

/// Empty placement - used for simple articulations.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyPlacement {
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Wavy line (for trills, barlines, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct WavyLine {
    /// Start, stop, or continue
    pub r#type: StartStopContinue,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Position attributes
    pub position: Position,
}

// === Identification types (used in both score and part) ===

/// Identification (creators, rights, encoding).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Identification {
    /// Creators (composer, lyricist, etc.)
    pub creators: Vec<TypedText>,
    /// Rights (copyright)
    pub rights: Vec<TypedText>,
    /// Encoding information
    pub encoding: Option<Encoding>,
    /// Source
    pub source: Option<String>,
    /// Relations
    pub relations: Vec<TypedText>,
    /// Miscellaneous
    pub miscellaneous: Option<Miscellaneous>,
}

/// Text with a type attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedText {
    /// The text value
    pub value: String,
    /// The type (e.g., "composer", "lyricist")
    pub r#type: Option<String>,
}

/// Encoding information.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Encoding {
    /// Encoding content
    pub content: Vec<EncodingContent>,
}

/// Encoding content elements.
#[derive(Debug, Clone, PartialEq)]
pub enum EncodingContent {
    /// Encoding date
    EncodingDate(String),
    /// Encoder
    Encoder(TypedText),
    /// Software used
    Software(String),
    /// Encoding description
    EncodingDescription(String),
    /// Supports declaration
    Supports(Supports),
}

/// Supports element.
#[derive(Debug, Clone, PartialEq)]
pub struct Supports {
    /// Yes or no
    pub r#type: YesNo,
    /// Element name
    pub element: String,
    /// Attribute name
    pub attribute: Option<String>,
    /// Attribute value
    pub value: Option<String>,
}

/// Miscellaneous information.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Miscellaneous {
    /// Miscellaneous fields
    pub fields: Vec<MiscellaneousField>,
}

/// Miscellaneous field.
#[derive(Debug, Clone, PartialEq)]
pub struct MiscellaneousField {
    /// Field name
    pub name: String,
    /// Field value
    pub value: String,
}

// === Accidental types (used in note, notation, attributes, and part) ===

/// Accidental values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccidentalValue {
    /// Sharp
    Sharp,
    /// Natural
    Natural,
    /// Flat
    Flat,
    /// Double sharp (x)
    DoubleSharp,
    /// Sharp-sharp
    SharpSharp,
    /// Flat-flat
    FlatFlat,
    /// Double flat
    DoubleFlat,
    /// Natural-sharp
    NaturalSharp,
    /// Natural-flat
    NaturalFlat,
    /// Quarter flat
    QuarterFlat,
    /// Quarter sharp
    QuarterSharp,
    /// Three-quarters flat
    ThreeQuartersFlat,
    /// Three-quarters sharp
    ThreeQuartersSharp,
    /// Sharp-down
    SharpDown,
    /// Sharp-up
    SharpUp,
    /// Natural-down
    NaturalDown,
    /// Natural-up
    NaturalUp,
    /// Flat-down
    FlatDown,
    /// Flat-up
    FlatUp,
    /// Triple sharp
    TripleSharp,
    /// Triple flat
    TripleFlat,
    /// Slash quarter sharp
    SlashQuarterSharp,
    /// Slash sharp
    SlashSharp,
    /// Slash flat
    SlashFlat,
    /// Double slash flat
    DoubleSlashFlat,
    /// Sharp-1 (Stein-Zimmermann)
    Sharp1,
    /// Sharp-2 (Stein-Zimmermann)
    Sharp2,
    /// Sharp-3 (Stein-Zimmermann)
    Sharp3,
    /// Sharp-5 (Stein-Zimmermann)
    Sharp5,
    /// Flat-1 (Stein-Zimmermann)
    Flat1,
    /// Flat-2 (Stein-Zimmermann)
    Flat2,
    /// Flat-3 (Stein-Zimmermann)
    Flat3,
    /// Flat-4 (Stein-Zimmermann)
    Flat4,
    /// Sori (Persian)
    Sori,
    /// Koron (Persian)
    Koron,
    /// Other accidental
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === YesNo Tests ===

    #[test]
    fn test_yesno_yes_variant() {
        let value = YesNo::Yes;
        assert_eq!(value, YesNo::Yes);
    }

    #[test]
    fn test_yesno_no_variant() {
        let value = YesNo::No;
        assert_eq!(value, YesNo::No);
    }

    #[test]
    fn test_yesno_clone() {
        let value = YesNo::Yes;
        let cloned = value.clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn test_yesno_copy() {
        let value = YesNo::No;
        let copied = value;
        assert_eq!(value, copied);
    }

    #[test]
    fn test_yesno_debug() {
        assert_eq!(format!("{:?}", YesNo::Yes), "Yes");
        assert_eq!(format!("{:?}", YesNo::No), "No");
    }

    // === StartStop Tests ===

    #[test]
    fn test_startstop_start_variant() {
        let value = StartStop::Start;
        assert_eq!(value, StartStop::Start);
    }

    #[test]
    fn test_startstop_stop_variant() {
        let value = StartStop::Stop;
        assert_eq!(value, StartStop::Stop);
    }

    #[test]
    fn test_startstop_clone_and_copy() {
        let value = StartStop::Start;
        let cloned = value.clone();
        let copied = value;
        assert_eq!(value, cloned);
        assert_eq!(value, copied);
    }

    // === StartStopContinue Tests ===

    #[test]
    fn test_startstopcontinue_all_variants() {
        assert_eq!(StartStopContinue::Start, StartStopContinue::Start);
        assert_eq!(StartStopContinue::Stop, StartStopContinue::Stop);
        assert_eq!(StartStopContinue::Continue, StartStopContinue::Continue);
    }

    #[test]
    fn test_startstopcontinue_inequality() {
        assert_ne!(StartStopContinue::Start, StartStopContinue::Stop);
        assert_ne!(StartStopContinue::Stop, StartStopContinue::Continue);
    }

    // === StartStopSingle Tests ===

    #[test]
    fn test_startstopsingle_all_variants() {
        assert_eq!(StartStopSingle::Start, StartStopSingle::Start);
        assert_eq!(StartStopSingle::Stop, StartStopSingle::Stop);
        assert_eq!(StartStopSingle::Single, StartStopSingle::Single);
    }

    // === StartStopDiscontinue Tests ===

    #[test]
    fn test_startstopdiscontinue_all_variants() {
        assert_eq!(
            StartStopDiscontinue::Start,
            StartStopDiscontinue::Start.clone()
        );
        assert_eq!(StartStopDiscontinue::Stop, StartStopDiscontinue::Stop);
        assert_eq!(
            StartStopDiscontinue::Discontinue,
            StartStopDiscontinue::Discontinue
        );
    }

    // === AboveBelow Tests ===

    #[test]
    fn test_abovebelow_all_variants() {
        let above = AboveBelow::Above;
        let below = AboveBelow::Below;
        assert_ne!(above, below);
        assert_eq!(above.clone(), AboveBelow::Above);
    }

    // === UpDown Tests ===

    #[test]
    fn test_updown_all_variants() {
        assert_eq!(UpDown::Up, UpDown::Up);
        assert_eq!(UpDown::Down, UpDown::Down);
        assert_ne!(UpDown::Up, UpDown::Down);
    }

    // === OverUnder Tests ===

    #[test]
    fn test_overunder_all_variants() {
        assert_eq!(OverUnder::Over, OverUnder::Over);
        assert_eq!(OverUnder::Under, OverUnder::Under);
    }

    // === LeftCenterRight Tests ===

    #[test]
    fn test_leftcenterright_all_variants() {
        assert_eq!(LeftCenterRight::Left, LeftCenterRight::Left);
        assert_eq!(LeftCenterRight::Center, LeftCenterRight::Center);
        assert_eq!(LeftCenterRight::Right, LeftCenterRight::Right);
    }

    // === TopMiddleBottom Tests ===

    #[test]
    fn test_topmiddlebottom_all_variants() {
        assert_eq!(TopMiddleBottom::Top, TopMiddleBottom::Top);
        assert_eq!(TopMiddleBottom::Middle, TopMiddleBottom::Middle);
        assert_eq!(TopMiddleBottom::Bottom, TopMiddleBottom::Bottom);
    }

    // === BackwardForward Tests ===

    #[test]
    fn test_backwardforward_all_variants() {
        assert_eq!(BackwardForward::Backward, BackwardForward::Backward);
        assert_eq!(BackwardForward::Forward, BackwardForward::Forward);
    }

    // === RightLeftMiddle Tests ===

    #[test]
    fn test_rightleftmiddle_all_variants() {
        assert_eq!(RightLeftMiddle::Right, RightLeftMiddle::Right);
        assert_eq!(RightLeftMiddle::Left, RightLeftMiddle::Left);
        assert_eq!(RightLeftMiddle::Middle, RightLeftMiddle::Middle);
    }

    // === UprightInverted Tests ===

    #[test]
    fn test_uprightinverted_all_variants() {
        assert_eq!(UprightInverted::Upright, UprightInverted::Upright);
        assert_eq!(UprightInverted::Inverted, UprightInverted::Inverted);
    }

    // === SymbolSize Tests ===

    #[test]
    fn test_symbolsize_all_variants() {
        assert_eq!(SymbolSize::Full, SymbolSize::Full);
        assert_eq!(SymbolSize::Cue, SymbolSize::Cue);
        assert_eq!(SymbolSize::GraceCue, SymbolSize::GraceCue);
        assert_eq!(SymbolSize::Large, SymbolSize::Large);
    }

    // === LineType Tests ===

    #[test]
    fn test_linetype_all_variants() {
        assert_eq!(LineType::Solid, LineType::Solid);
        assert_eq!(LineType::Dashed, LineType::Dashed);
        assert_eq!(LineType::Dotted, LineType::Dotted);
        assert_eq!(LineType::Wavy, LineType::Wavy);
    }

    // === FontStyle Tests ===

    #[test]
    fn test_fontstyle_all_variants() {
        assert_eq!(FontStyle::Normal, FontStyle::Normal);
        assert_eq!(FontStyle::Italic, FontStyle::Italic);
    }

    // === FontWeight Tests ===

    #[test]
    fn test_fontweight_all_variants() {
        assert_eq!(FontWeight::Normal, FontWeight::Normal);
        assert_eq!(FontWeight::Bold, FontWeight::Bold);
    }

    // === Position Tests ===

    #[test]
    fn test_position_default() {
        let pos = Position::default();
        assert!(pos.default_x.is_none());
        assert!(pos.default_y.is_none());
        assert!(pos.relative_x.is_none());
        assert!(pos.relative_y.is_none());
    }

    #[test]
    fn test_position_with_values() {
        let pos = Position {
            default_x: Some(10.0),
            default_y: Some(20.0),
            relative_x: Some(-5.0),
            relative_y: Some(15.0),
        };
        assert_eq!(pos.default_x, Some(10.0));
        assert_eq!(pos.default_y, Some(20.0));
        assert_eq!(pos.relative_x, Some(-5.0));
        assert_eq!(pos.relative_y, Some(15.0));
    }

    #[test]
    fn test_position_clone() {
        let pos = Position {
            default_x: Some(5.5),
            ..Default::default()
        };
        let cloned = pos.clone();
        assert_eq!(pos, cloned);
    }

    // === CssFontSize Tests ===

    #[test]
    fn test_cssfontsize_all_variants() {
        assert_eq!(CssFontSize::XxSmall, CssFontSize::XxSmall);
        assert_eq!(CssFontSize::XSmall, CssFontSize::XSmall);
        assert_eq!(CssFontSize::Small, CssFontSize::Small);
        assert_eq!(CssFontSize::Medium, CssFontSize::Medium);
        assert_eq!(CssFontSize::Large, CssFontSize::Large);
        assert_eq!(CssFontSize::XLarge, CssFontSize::XLarge);
        assert_eq!(CssFontSize::XxLarge, CssFontSize::XxLarge);
    }

    // === FontSize Tests ===

    #[test]
    fn test_fontsize_css_variant() {
        let size = FontSize::Css(CssFontSize::Medium);
        assert_eq!(size, FontSize::Css(CssFontSize::Medium));
    }

    #[test]
    fn test_fontsize_points_variant() {
        let size = FontSize::Points(12.0);
        assert_eq!(size, FontSize::Points(12.0));
    }

    #[test]
    fn test_fontsize_clone() {
        let size = FontSize::Points(14.5);
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    // === Font Tests ===

    #[test]
    fn test_font_default() {
        let font = Font::default();
        assert!(font.font_family.is_none());
        assert!(font.font_style.is_none());
        assert!(font.font_size.is_none());
        assert!(font.font_weight.is_none());
    }

    #[test]
    fn test_font_with_values() {
        let font = Font {
            font_family: Some("Arial".to_string()),
            font_style: Some(FontStyle::Italic),
            font_size: Some(FontSize::Points(12.0)),
            font_weight: Some(FontWeight::Bold),
        };
        assert_eq!(font.font_family, Some("Arial".to_string()));
        assert_eq!(font.font_style, Some(FontStyle::Italic));
        assert_eq!(font.font_size, Some(FontSize::Points(12.0)));
        assert_eq!(font.font_weight, Some(FontWeight::Bold));
    }

    // === PrintStyle Tests ===

    #[test]
    fn test_printstyle_default() {
        let ps = PrintStyle::default();
        assert_eq!(ps.position, Position::default());
        assert_eq!(ps.font, Font::default());
        assert!(ps.color.is_none());
    }

    #[test]
    fn test_printstyle_with_color() {
        let ps = PrintStyle {
            position: Position::default(),
            font: Font::default(),
            color: Some("#FF0000".to_string()),
        };
        assert_eq!(ps.color, Some("#FF0000".to_string()));
    }

    // === Editorial Tests ===

    #[test]
    fn test_editorial_default() {
        let ed = Editorial::default();
        assert!(ed.footnote.is_none());
        assert!(ed.level.is_none());
    }

    // === FormattedText Tests ===

    #[test]
    fn test_formattedtext_construction() {
        let ft = FormattedText {
            value: "Test text".to_string(),
            print_style: PrintStyle::default(),
            lang: Some("en".to_string()),
        };
        assert_eq!(ft.value, "Test text");
        assert_eq!(ft.lang, Some("en".to_string()));
    }

    #[test]
    fn test_formattedtext_clone() {
        let ft = FormattedText {
            value: "Clone test".to_string(),
            print_style: PrintStyle::default(),
            lang: None,
        };
        let cloned = ft.clone();
        assert_eq!(ft, cloned);
    }

    // === Level Tests ===

    #[test]
    fn test_level_construction() {
        let level = Level {
            value: "1".to_string(),
            reference: Some(YesNo::Yes),
        };
        assert_eq!(level.value, "1");
        assert_eq!(level.reference, Some(YesNo::Yes));
    }

    // === EmptyPlacement Tests ===

    #[test]
    fn test_emptyplacement_default() {
        let ep = EmptyPlacement::default();
        assert!(ep.placement.is_none());
        assert_eq!(ep.position, Position::default());
    }

    #[test]
    fn test_emptyplacement_with_placement() {
        let ep = EmptyPlacement {
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        };
        assert_eq!(ep.placement, Some(AboveBelow::Above));
    }

    // === WavyLine Tests ===

    #[test]
    fn test_wavyline_construction() {
        let wl = WavyLine {
            r#type: StartStopContinue::Start,
            number: Some(1),
            position: Position::default(),
        };
        assert_eq!(wl.r#type, StartStopContinue::Start);
        assert_eq!(wl.number, Some(1));
    }

    // === Identification Tests ===

    #[test]
    fn test_identification_default() {
        let id = Identification::default();
        assert!(id.creators.is_empty());
        assert!(id.rights.is_empty());
        assert!(id.encoding.is_none());
        assert!(id.source.is_none());
        assert!(id.relations.is_empty());
        assert!(id.miscellaneous.is_none());
    }

    #[test]
    fn test_identification_with_creators() {
        let id = Identification {
            creators: vec![TypedText {
                value: "John Doe".to_string(),
                r#type: Some("composer".to_string()),
            }],
            ..Default::default()
        };
        assert_eq!(id.creators.len(), 1);
        assert_eq!(id.creators[0].value, "John Doe");
    }

    // === TypedText Tests ===

    #[test]
    fn test_typedtext_construction() {
        let tt = TypedText {
            value: "Test".to_string(),
            r#type: Some("test-type".to_string()),
        };
        assert_eq!(tt.value, "Test");
        assert_eq!(tt.r#type, Some("test-type".to_string()));
    }

    #[test]
    fn test_typedtext_without_type() {
        let tt = TypedText {
            value: "No type".to_string(),
            r#type: None,
        };
        assert!(tt.r#type.is_none());
    }

    // === Encoding Tests ===

    #[test]
    fn test_encoding_default() {
        let enc = Encoding::default();
        assert!(enc.content.is_empty());
    }

    #[test]
    fn test_encoding_with_content() {
        let enc = Encoding {
            content: vec![
                EncodingContent::EncodingDate("2024-01-01".to_string()),
                EncodingContent::Software("Fermata".to_string()),
            ],
        };
        assert_eq!(enc.content.len(), 2);
    }

    // === EncodingContent Tests ===

    #[test]
    fn test_encodingcontent_encoding_date() {
        let content = EncodingContent::EncodingDate("2024-01-01".to_string());
        if let EncodingContent::EncodingDate(date) = content {
            assert_eq!(date, "2024-01-01");
        } else {
            panic!("Expected EncodingDate variant");
        }
    }

    #[test]
    fn test_encodingcontent_encoder() {
        let content = EncodingContent::Encoder(TypedText {
            value: "Jane Smith".to_string(),
            r#type: None,
        });
        if let EncodingContent::Encoder(tt) = content {
            assert_eq!(tt.value, "Jane Smith");
        } else {
            panic!("Expected Encoder variant");
        }
    }

    #[test]
    fn test_encodingcontent_software() {
        let content = EncodingContent::Software("Test Software".to_string());
        if let EncodingContent::Software(s) = content {
            assert_eq!(s, "Test Software");
        } else {
            panic!("Expected Software variant");
        }
    }

    #[test]
    fn test_encodingcontent_encoding_description() {
        let content = EncodingContent::EncodingDescription("Description".to_string());
        if let EncodingContent::EncodingDescription(desc) = content {
            assert_eq!(desc, "Description");
        } else {
            panic!("Expected EncodingDescription variant");
        }
    }

    #[test]
    fn test_encodingcontent_supports() {
        let supports = Supports {
            r#type: YesNo::Yes,
            element: "print".to_string(),
            attribute: Some("new-page".to_string()),
            value: Some("yes".to_string()),
        };
        let content = EncodingContent::Supports(supports.clone());
        if let EncodingContent::Supports(s) = content {
            assert_eq!(s.element, "print");
        } else {
            panic!("Expected Supports variant");
        }
    }

    // === Supports Tests ===

    #[test]
    fn test_supports_construction() {
        let supports = Supports {
            r#type: YesNo::Yes,
            element: "beam".to_string(),
            attribute: None,
            value: None,
        };
        assert_eq!(supports.r#type, YesNo::Yes);
        assert_eq!(supports.element, "beam");
        assert!(supports.attribute.is_none());
    }

    // === Miscellaneous Tests ===

    #[test]
    fn test_miscellaneous_default() {
        let misc = Miscellaneous::default();
        assert!(misc.fields.is_empty());
    }

    #[test]
    fn test_miscellaneous_with_fields() {
        let misc = Miscellaneous {
            fields: vec![MiscellaneousField {
                name: "custom".to_string(),
                value: "data".to_string(),
            }],
        };
        assert_eq!(misc.fields.len(), 1);
    }

    // === MiscellaneousField Tests ===

    #[test]
    fn test_miscellaneousfield_construction() {
        let field = MiscellaneousField {
            name: "field-name".to_string(),
            value: "field-value".to_string(),
        };
        assert_eq!(field.name, "field-name");
        assert_eq!(field.value, "field-value");
    }

    // === AccidentalValue Tests ===

    #[test]
    fn test_accidentalvalue_basic_variants() {
        assert_eq!(AccidentalValue::Sharp, AccidentalValue::Sharp);
        assert_eq!(AccidentalValue::Natural, AccidentalValue::Natural);
        assert_eq!(AccidentalValue::Flat, AccidentalValue::Flat);
    }

    #[test]
    fn test_accidentalvalue_double_variants() {
        assert_eq!(AccidentalValue::DoubleSharp, AccidentalValue::DoubleSharp);
        assert_eq!(AccidentalValue::DoubleFlat, AccidentalValue::DoubleFlat);
        assert_eq!(AccidentalValue::SharpSharp, AccidentalValue::SharpSharp);
        assert_eq!(AccidentalValue::FlatFlat, AccidentalValue::FlatFlat);
    }

    #[test]
    fn test_accidentalvalue_natural_combo_variants() {
        assert_eq!(AccidentalValue::NaturalSharp, AccidentalValue::NaturalSharp);
        assert_eq!(AccidentalValue::NaturalFlat, AccidentalValue::NaturalFlat);
    }

    #[test]
    fn test_accidentalvalue_quarter_tone_variants() {
        assert_eq!(AccidentalValue::QuarterFlat, AccidentalValue::QuarterFlat);
        assert_eq!(AccidentalValue::QuarterSharp, AccidentalValue::QuarterSharp);
        assert_eq!(
            AccidentalValue::ThreeQuartersFlat,
            AccidentalValue::ThreeQuartersFlat
        );
        assert_eq!(
            AccidentalValue::ThreeQuartersSharp,
            AccidentalValue::ThreeQuartersSharp
        );
    }

    #[test]
    fn test_accidentalvalue_arrow_variants() {
        assert_eq!(AccidentalValue::SharpDown, AccidentalValue::SharpDown);
        assert_eq!(AccidentalValue::SharpUp, AccidentalValue::SharpUp);
        assert_eq!(AccidentalValue::NaturalDown, AccidentalValue::NaturalDown);
        assert_eq!(AccidentalValue::NaturalUp, AccidentalValue::NaturalUp);
        assert_eq!(AccidentalValue::FlatDown, AccidentalValue::FlatDown);
        assert_eq!(AccidentalValue::FlatUp, AccidentalValue::FlatUp);
    }

    #[test]
    fn test_accidentalvalue_triple_variants() {
        assert_eq!(AccidentalValue::TripleSharp, AccidentalValue::TripleSharp);
        assert_eq!(AccidentalValue::TripleFlat, AccidentalValue::TripleFlat);
    }

    #[test]
    fn test_accidentalvalue_slash_variants() {
        assert_eq!(
            AccidentalValue::SlashQuarterSharp,
            AccidentalValue::SlashQuarterSharp
        );
        assert_eq!(AccidentalValue::SlashSharp, AccidentalValue::SlashSharp);
        assert_eq!(AccidentalValue::SlashFlat, AccidentalValue::SlashFlat);
        assert_eq!(
            AccidentalValue::DoubleSlashFlat,
            AccidentalValue::DoubleSlashFlat
        );
    }

    #[test]
    fn test_accidentalvalue_stein_zimmermann_variants() {
        assert_eq!(AccidentalValue::Sharp1, AccidentalValue::Sharp1);
        assert_eq!(AccidentalValue::Sharp2, AccidentalValue::Sharp2);
        assert_eq!(AccidentalValue::Sharp3, AccidentalValue::Sharp3);
        assert_eq!(AccidentalValue::Sharp5, AccidentalValue::Sharp5);
        assert_eq!(AccidentalValue::Flat1, AccidentalValue::Flat1);
        assert_eq!(AccidentalValue::Flat2, AccidentalValue::Flat2);
        assert_eq!(AccidentalValue::Flat3, AccidentalValue::Flat3);
        assert_eq!(AccidentalValue::Flat4, AccidentalValue::Flat4);
    }

    #[test]
    fn test_accidentalvalue_persian_variants() {
        assert_eq!(AccidentalValue::Sori, AccidentalValue::Sori);
        assert_eq!(AccidentalValue::Koron, AccidentalValue::Koron);
    }

    #[test]
    fn test_accidentalvalue_other_variant() {
        assert_eq!(AccidentalValue::Other, AccidentalValue::Other);
    }

    #[test]
    fn test_accidentalvalue_clone() {
        let acc = AccidentalValue::Sharp;
        let cloned = acc.clone();
        assert_eq!(acc, cloned);
    }

    #[test]
    fn test_accidentalvalue_copy() {
        let acc = AccidentalValue::Flat;
        let copied = acc;
        assert_eq!(acc, copied);
    }
}
