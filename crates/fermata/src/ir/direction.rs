//! Direction types: dynamics, wedges, metronome, etc.

use super::common::{
    AboveBelow, Color, Divisions, Font, FormattedText, LeftCenterRight, LineType, NumberLevel,
    Octave, Position, PrintStyle, Semitones, StaffNumber, StartStop, StartStopContinue, Voice,
    YesNo,
};
use super::duration::NoteTypeValue;

/// A musical direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Direction {
    /// Placement above or below the staff
    pub placement: Option<AboveBelow>,
    /// Whether this is a directive (affects default formatting)
    pub directive: Option<YesNo>,
    /// The direction type elements
    pub direction_types: Vec<DirectionType>,
    /// Offset from the note
    pub offset: Option<Offset>,
    /// Voice this direction belongs to
    pub voice: Option<Voice>,
    /// Staff number
    pub staff: Option<StaffNumber>,
    /// Sound element for playback
    pub sound: Option<Sound>,
}

/// Wrapper for direction type content.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionType {
    /// The direction type content
    pub content: DirectionTypeContent,
}

/// Direction type content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectionTypeContent {
    /// Rehearsal marks
    Rehearsal(Vec<FormattedText>),
    /// Segno signs
    Segno(Vec<Segno>),
    /// Coda signs
    Coda(Vec<Coda>),
    /// Text words
    Words(Vec<Words>),
    /// Symbols
    Symbol(Vec<FormattedSymbol>),
    /// Crescendo/diminuendo wedge
    Wedge(Wedge),
    /// Dynamic markings
    Dynamics(Dynamics),
    /// Dashes for spanning text
    Dashes(Dashes),
    /// Bracket
    Bracket(Bracket),
    /// Pedal marking
    Pedal(Pedal),
    /// Metronome marking
    Metronome(Metronome),
    /// Octave shift (8va, 8vb, etc.)
    OctaveShift(OctaveShift),
    /// Harp pedal diagram
    HarpPedals(HarpPedals),
    /// Damp
    Damp(EmptyPrintStyle),
    /// Damp all
    DampAll(EmptyPrintStyle),
    /// Eyeglasses
    Eyeglasses(EmptyPrintStyle),
    /// String mute
    StringMute(StringMute),
    /// Scordatura
    Scordatura(Scordatura),
    /// Image
    Image(Image),
    /// Principal voice
    PrincipalVoice(PrincipalVoice),
    /// Percussion
    Percussion(Vec<Percussion>),
    /// Accordion registration
    AccordionRegistration(AccordionRegistration),
    /// Staff divide
    StaffDivide(StaffDivide),
    /// Other direction
    OtherDirection(OtherDirection),
}

/// Segno sign.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Segno {
    /// Print style attributes
    pub print_style: PrintStyle,
    /// SMuFL glyph name
    pub smufl: Option<String>,
}

/// Coda sign.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Coda {
    /// Print style attributes
    pub print_style: PrintStyle,
    /// SMuFL glyph name
    pub smufl: Option<String>,
}

/// Text direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Words {
    /// The text value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Text justification
    pub justify: Option<LeftCenterRight>,
    /// Language code
    pub lang: Option<String>,
}

/// Symbol direction.
#[derive(Debug, Clone, PartialEq)]
pub struct FormattedSymbol {
    /// The symbol value
    pub value: String,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Symbol justification
    pub justify: Option<LeftCenterRight>,
}

/// Dynamic markings.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dynamics {
    /// The dynamic elements
    pub content: Vec<DynamicElement>,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
}

/// Individual dynamic marking.
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicElement {
    /// p
    P,
    /// pp
    PP,
    /// ppp
    PPP,
    /// pppp
    PPPP,
    /// ppppp
    PPPPP,
    /// pppppp
    PPPPPP,
    /// f
    F,
    /// ff
    FF,
    /// fff
    FFF,
    /// ffff
    FFFF,
    /// fffff
    FFFFF,
    /// ffffff
    FFFFFF,
    /// mp
    MP,
    /// mf
    MF,
    /// sf
    SF,
    /// sfp
    SFP,
    /// sfpp
    SFPP,
    /// fp
    FP,
    /// rf
    RF,
    /// rfz
    RFZ,
    /// sfz
    SFZ,
    /// sffz
    SFFZ,
    /// fz
    FZ,
    /// n (niente)
    N,
    /// pf
    PF,
    /// sfzp
    SFZP,
    /// Other dynamics
    OtherDynamics(String),
}

/// Crescendo/diminuendo wedge.
#[derive(Debug, Clone, PartialEq)]
pub struct Wedge {
    /// Wedge type
    pub r#type: WedgeType,
    /// Number level for multiple wedges
    pub number: Option<NumberLevel>,
    /// Spread at the point
    pub spread: Option<super::common::Tenths>,
    /// Whether the wedge ends at niente (nothing)
    pub niente: Option<YesNo>,
    /// Line type
    pub line_type: Option<LineType>,
    /// Position attributes
    pub position: Position,
    /// Wedge color
    pub color: Option<Color>,
}

/// Wedge type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WedgeType {
    /// Crescendo (opening wedge)
    Crescendo,
    /// Diminuendo (closing wedge)
    Diminuendo,
    /// Stop the wedge
    Stop,
    /// Continue the wedge
    Continue,
}

/// Dashes for spanning text.
#[derive(Debug, Clone, PartialEq)]
pub struct Dashes {
    /// Start, stop, or continue
    pub r#type: StartStopContinue,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Position attributes
    pub position: Position,
    /// Dashes color
    pub color: Option<Color>,
}

/// Bracket for grouping.
#[derive(Debug, Clone, PartialEq)]
pub struct Bracket {
    /// Start, stop, or continue
    pub r#type: StartStopContinue,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Line end type
    pub line_end: LineEnd,
    /// End length in tenths
    pub end_length: Option<super::common::Tenths>,
    /// Line type
    pub line_type: Option<LineType>,
    /// Position attributes
    pub position: Position,
    /// Bracket color
    pub color: Option<Color>,
}

/// Line end types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnd {
    /// Up hook
    Up,
    /// Down hook
    Down,
    /// Both hooks
    Both,
    /// Arrow
    Arrow,
    /// No hook
    None,
}

/// Pedal marking.
#[derive(Debug, Clone, PartialEq)]
pub struct Pedal {
    /// Pedal type
    pub r#type: PedalType,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Whether to show as a line
    pub line: Option<YesNo>,
    /// Whether to show the sign
    pub sign: Option<YesNo>,
    /// Whether to use abbreviated form
    pub abbreviated: Option<YesNo>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Pedal types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PedalType {
    /// Start the pedal
    Start,
    /// Stop the pedal
    Stop,
    /// Sostenuto pedal
    Sostenuto,
    /// Change the pedal
    Change,
    /// Continue the pedal
    Continue,
    /// Discontinue the pedal
    Discontinue,
    /// Resume the pedal
    Resume,
}

/// Metronome marking.
#[derive(Debug, Clone, PartialEq)]
pub struct Metronome {
    /// Whether to show parentheses
    pub parentheses: Option<YesNo>,
    /// Metronome content
    pub content: MetronomeContent,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Metronome content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum MetronomeContent {
    /// beat-unit = per-minute
    PerMinute {
        /// Beat unit note type
        beat_unit: NoteTypeValue,
        /// Number of dots on beat unit
        beat_unit_dots: u32,
        /// Beats per minute
        per_minute: PerMinute,
    },
    /// beat-unit = beat-unit (tempo change)
    BeatEquation {
        /// Left beat unit
        left_unit: NoteTypeValue,
        /// Left beat unit dots
        left_dots: u32,
        /// Right beat unit
        right_unit: NoteTypeValue,
        /// Right beat unit dots
        right_dots: u32,
    },
    /// Metric modulation with metric note groups
    MetricModulation {
        /// Metric relations
        metric_relation: Vec<MetricRelation>,
    },
}

/// Per-minute value for metronome.
#[derive(Debug, Clone, PartialEq)]
pub struct PerMinute {
    /// The per-minute value (may include ranges like "120-132")
    pub value: String,
    /// Font attributes
    pub font: Font,
}

/// Metric relation for complex metronome markings.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricRelation {
    /// Left side of the relation
    pub left: MetronomeNote,
    /// Right side of the relation
    pub right: MetronomeNote,
}

/// A note in a metronome marking.
#[derive(Debug, Clone, PartialEq)]
pub struct MetronomeNote {
    /// Note type
    pub note_type: NoteTypeValue,
    /// Number of dots
    pub dots: u32,
    /// Tuplet information
    pub tuplet: Option<MetronomeTuplet>,
}

/// Tuplet information in a metronome marking.
#[derive(Debug, Clone, PartialEq)]
pub struct MetronomeTuplet {
    /// Actual notes
    pub actual_notes: u32,
    /// Normal notes
    pub normal_notes: u32,
    /// Start or stop
    pub r#type: StartStop,
}

/// Octave shift (8va, 8vb, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct OctaveShift {
    /// Up, down, stop, or continue
    pub r#type: UpDownStopContinue,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Size of the shift (8 or 15)
    pub size: Option<u8>,
    /// Position attributes
    pub position: Position,
}

/// Up, down, stop, or continue for octave shifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpDownStopContinue {
    /// Up (8va)
    Up,
    /// Down (8vb)
    Down,
    /// Stop the shift
    Stop,
    /// Continue the shift
    Continue,
}

/// Offset for direction placement.
#[derive(Debug, Clone, PartialEq)]
pub struct Offset {
    /// Offset value in divisions
    pub value: Divisions,
    /// Whether the offset affects sound
    pub sound: Option<YesNo>,
}

/// Sound element for playback.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sound {
    /// Tempo in beats per minute
    pub tempo: Option<f64>,
    /// Dynamics percentage
    pub dynamics: Option<f64>,
    /// Da capo
    pub dacapo: Option<YesNo>,
    /// Segno target
    pub segno: Option<String>,
    /// Dal segno
    pub dalsegno: Option<String>,
    /// Coda target
    pub coda: Option<String>,
    /// To coda
    pub tocoda: Option<String>,
    /// Divisions
    pub divisions: Option<Divisions>,
    /// Forward repeat
    pub forward_repeat: Option<YesNo>,
    /// Fine
    pub fine: Option<String>,
    /// Time only
    pub time_only: Option<String>,
    /// Pizzicato
    pub pizzicato: Option<YesNo>,
}

// Placeholder types for less common direction types

/// Empty print style.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyPrintStyle {
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Harp pedal diagram.
#[derive(Debug, Clone, PartialEq)]
pub struct HarpPedals {
    /// Pedal tunings
    pub pedal_tuning: Vec<PedalTuning>,
}

/// Individual harp pedal tuning.
#[derive(Debug, Clone, PartialEq)]
pub struct PedalTuning {
    /// Pedal step
    pub pedal_step: super::pitch::Step,
    /// Pedal alteration
    pub pedal_alter: Semitones,
}

/// String mute.
#[derive(Debug, Clone, PartialEq)]
pub struct StringMute {
    /// On or off
    pub r#type: OnOff,
}

/// On or off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnOff {
    /// On
    On,
    /// Off
    Off,
}

/// Scordatura (alternate tuning).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scordatura {
    /// Accord elements
    pub accord: Vec<Accord>,
}

/// Individual string tuning in scordatura.
#[derive(Debug, Clone, PartialEq)]
pub struct Accord {
    /// String number
    pub string: u8,
    /// Tuning step
    pub tuning_step: super::pitch::Step,
    /// Tuning alteration
    pub tuning_alter: Option<Semitones>,
    /// Tuning octave
    pub tuning_octave: Octave,
}

/// Image element.
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    /// Image source URL
    pub source: String,
    /// Image MIME type
    pub r#type: String,
    /// Position attributes
    pub position: Position,
}

/// Principal voice indicator.
#[derive(Debug, Clone, PartialEq)]
pub struct PrincipalVoice {
    /// Start or stop
    pub r#type: StartStop,
    /// Symbol type
    pub symbol: PrincipalVoiceSymbol,
}

/// Principal voice symbol types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalVoiceSymbol {
    /// Hauptstimme (H)
    Hauptstimme,
    /// Nebenstimme (N)
    Nebenstimme,
    /// Plain text
    Plain,
    /// No symbol
    None,
}

/// Percussion element.
#[derive(Debug, Clone, PartialEq)]
pub struct Percussion {
    /// Percussion content
    pub content: PercussionContent,
}

/// Percussion content types.
#[derive(Debug, Clone, PartialEq)]
pub enum PercussionContent {
    /// Glass percussion
    Glass(Glass),
    /// Metal percussion
    Metal(Metal),
    /// Wood percussion
    Wood(Wood),
    /// Pitched percussion
    Pitched(Pitched),
    /// Membrane percussion
    Membrane(Membrane),
    /// Effect
    Effect(Effect),
    /// Timpani
    Timpani,
    /// Beater
    Beater(Beater),
    /// Stick
    Stick(Stick),
    /// Stick location
    StickLocation(StickLocation),
    /// Other percussion
    OtherPercussion(String),
}

// Simplified percussion types

/// Glass percussion.
#[derive(Debug, Clone, PartialEq)]
pub struct Glass {
    /// Glass type value
    pub value: String,
}

/// Metal percussion.
#[derive(Debug, Clone, PartialEq)]
pub struct Metal {
    /// Metal type value
    pub value: String,
}

/// Wood percussion.
#[derive(Debug, Clone, PartialEq)]
pub struct Wood {
    /// Wood type value
    pub value: String,
}

/// Pitched percussion.
#[derive(Debug, Clone, PartialEq)]
pub struct Pitched {
    /// Pitched instrument value
    pub value: String,
}

/// Membrane percussion.
#[derive(Debug, Clone, PartialEq)]
pub struct Membrane {
    /// Membrane type value
    pub value: String,
}

/// Effect.
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    /// Effect value
    pub value: String,
}

/// Beater.
#[derive(Debug, Clone, PartialEq)]
pub struct Beater {
    /// Beater value
    pub value: String,
}

/// Stick.
#[derive(Debug, Clone, PartialEq)]
pub struct Stick {
    /// Stick value
    pub value: String,
}

/// Stick location.
#[derive(Debug, Clone, PartialEq)]
pub struct StickLocation {
    /// Stick location value
    pub value: String,
}

/// Accordion registration.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccordionRegistration {
    /// High register
    pub accordion_high: bool,
    /// Middle register (0-3 dots)
    pub accordion_middle: Option<u8>,
    /// Low register
    pub accordion_low: bool,
}

/// Staff divide.
#[derive(Debug, Clone, PartialEq)]
pub struct StaffDivide {
    /// Staff divide symbol
    pub r#type: StaffDivideSymbol,
}

/// Staff divide symbol types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaffDivideSymbol {
    /// Down
    Down,
    /// Up
    Up,
    /// Up-down
    UpDown,
}

/// Other direction element.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherDirection {
    /// The direction value
    pub value: String,
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::pitch::Step;

    // === Direction Tests ===

    #[test]
    fn test_direction_basic() {
        let direction = Direction {
            placement: None,
            directive: None,
            direction_types: vec![],
            offset: None,
            voice: None,
            staff: None,
            sound: None,
        };
        assert!(direction.placement.is_none());
        assert!(direction.direction_types.is_empty());
    }

    #[test]
    fn test_direction_with_placement() {
        let direction = Direction {
            placement: Some(AboveBelow::Above),
            directive: None,
            direction_types: vec![],
            offset: None,
            voice: None,
            staff: None,
            sound: None,
        };
        assert_eq!(direction.placement, Some(AboveBelow::Above));
    }

    #[test]
    fn test_direction_with_staff() {
        let direction = Direction {
            placement: None,
            directive: None,
            direction_types: vec![],
            offset: None,
            voice: Some("1".to_string()),
            staff: Some(1),
            sound: None,
        };
        assert_eq!(direction.staff, Some(1));
        assert_eq!(direction.voice, Some("1".to_string()));
    }

    #[test]
    fn test_direction_clone() {
        let direction = Direction {
            placement: Some(AboveBelow::Below),
            directive: Some(YesNo::Yes),
            direction_types: vec![],
            offset: None,
            voice: None,
            staff: None,
            sound: None,
        };
        let cloned = direction.clone();
        assert_eq!(direction, cloned);
    }

    // === DirectionType Tests ===

    #[test]
    fn test_directiontype_dynamics() {
        let dt = DirectionType {
            content: DirectionTypeContent::Dynamics(Dynamics {
                content: vec![DynamicElement::F],
                print_style: PrintStyle::default(),
                placement: None,
            }),
        };
        if let DirectionTypeContent::Dynamics(d) = dt.content {
            assert_eq!(d.content.len(), 1);
        }
    }

    #[test]
    fn test_directiontype_wedge() {
        let dt = DirectionType {
            content: DirectionTypeContent::Wedge(Wedge {
                r#type: WedgeType::Crescendo,
                number: None,
                spread: None,
                niente: None,
                line_type: None,
                position: Position::default(),
                color: None,
            }),
        };
        if let DirectionTypeContent::Wedge(w) = dt.content {
            assert_eq!(w.r#type, WedgeType::Crescendo);
        }
    }

    // === Segno Tests ===

    #[test]
    fn test_segno_default() {
        let segno = Segno::default();
        assert_eq!(segno.print_style, PrintStyle::default());
        assert!(segno.smufl.is_none());
    }

    #[test]
    fn test_segno_with_smufl() {
        let segno = Segno {
            print_style: PrintStyle::default(),
            smufl: Some("segno".to_string()),
        };
        assert_eq!(segno.smufl, Some("segno".to_string()));
    }

    // === Coda Tests ===

    #[test]
    fn test_coda_default() {
        let coda = Coda::default();
        assert!(coda.smufl.is_none());
    }

    // === Words Tests ===

    #[test]
    fn test_words_basic() {
        let words = Words {
            value: "cresc.".to_string(),
            print_style: PrintStyle::default(),
            justify: None,
            lang: None,
        };
        assert_eq!(words.value, "cresc.");
    }

    #[test]
    fn test_words_with_lang() {
        let words = Words {
            value: "dolce".to_string(),
            print_style: PrintStyle::default(),
            justify: Some(LeftCenterRight::Left),
            lang: Some("it".to_string()),
        };
        assert_eq!(words.lang, Some("it".to_string()));
    }

    // === Dynamics Tests ===

    #[test]
    fn test_dynamics_default() {
        let dynamics = Dynamics::default();
        assert!(dynamics.content.is_empty());
    }

    #[test]
    fn test_dynamics_forte() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::F],
            print_style: PrintStyle::default(),
            placement: None,
        };
        assert_eq!(dynamics.content[0], DynamicElement::F);
    }

    #[test]
    fn test_dynamics_piano() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::P],
            print_style: PrintStyle::default(),
            placement: Some(AboveBelow::Below),
        };
        assert_eq!(dynamics.content[0], DynamicElement::P);
        assert_eq!(dynamics.placement, Some(AboveBelow::Below));
    }

    #[test]
    fn test_dynamics_mezzo_forte() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::MF],
            print_style: PrintStyle::default(),
            placement: None,
        };
        assert_eq!(dynamics.content[0], DynamicElement::MF);
    }

    #[test]
    fn test_dynamics_fortissimo() {
        let dynamics = Dynamics {
            content: vec![DynamicElement::FF],
            print_style: PrintStyle::default(),
            placement: None,
        };
        assert_eq!(dynamics.content[0], DynamicElement::FF);
    }

    // === DynamicElement Tests ===

    #[test]
    fn test_dynamicelement_piano_levels() {
        assert_eq!(DynamicElement::P, DynamicElement::P);
        assert_eq!(DynamicElement::PP, DynamicElement::PP);
        assert_eq!(DynamicElement::PPP, DynamicElement::PPP);
        assert_eq!(DynamicElement::PPPP, DynamicElement::PPPP);
        assert_eq!(DynamicElement::PPPPP, DynamicElement::PPPPP);
        assert_eq!(DynamicElement::PPPPPP, DynamicElement::PPPPPP);
    }

    #[test]
    fn test_dynamicelement_forte_levels() {
        assert_eq!(DynamicElement::F, DynamicElement::F);
        assert_eq!(DynamicElement::FF, DynamicElement::FF);
        assert_eq!(DynamicElement::FFF, DynamicElement::FFF);
        assert_eq!(DynamicElement::FFFF, DynamicElement::FFFF);
        assert_eq!(DynamicElement::FFFFF, DynamicElement::FFFFF);
        assert_eq!(DynamicElement::FFFFFF, DynamicElement::FFFFFF);
    }

    #[test]
    fn test_dynamicelement_mezzo() {
        assert_eq!(DynamicElement::MP, DynamicElement::MP);
        assert_eq!(DynamicElement::MF, DynamicElement::MF);
    }

    #[test]
    fn test_dynamicelement_sforzando() {
        assert_eq!(DynamicElement::SF, DynamicElement::SF);
        assert_eq!(DynamicElement::SFP, DynamicElement::SFP);
        assert_eq!(DynamicElement::SFPP, DynamicElement::SFPP);
        assert_eq!(DynamicElement::SFZ, DynamicElement::SFZ);
        assert_eq!(DynamicElement::SFFZ, DynamicElement::SFFZ);
        assert_eq!(DynamicElement::SFZP, DynamicElement::SFZP);
    }

    #[test]
    fn test_dynamicelement_other() {
        assert_eq!(DynamicElement::FP, DynamicElement::FP);
        assert_eq!(DynamicElement::RF, DynamicElement::RF);
        assert_eq!(DynamicElement::RFZ, DynamicElement::RFZ);
        assert_eq!(DynamicElement::FZ, DynamicElement::FZ);
        assert_eq!(DynamicElement::N, DynamicElement::N);
        assert_eq!(DynamicElement::PF, DynamicElement::PF);
    }

    #[test]
    fn test_dynamicelement_other_dynamics() {
        let other = DynamicElement::OtherDynamics("fp".to_string());
        if let DynamicElement::OtherDynamics(s) = other {
            assert_eq!(s, "fp");
        }
    }

    // === Wedge Tests ===

    #[test]
    fn test_wedge_crescendo() {
        let wedge = Wedge {
            r#type: WedgeType::Crescendo,
            number: Some(1),
            spread: Some(15.0),
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };
        assert_eq!(wedge.r#type, WedgeType::Crescendo);
        assert_eq!(wedge.spread, Some(15.0));
    }

    #[test]
    fn test_wedge_diminuendo() {
        let wedge = Wedge {
            r#type: WedgeType::Diminuendo,
            number: None,
            spread: None,
            niente: Some(YesNo::Yes),
            line_type: Some(LineType::Dashed),
            position: Position::default(),
            color: None,
        };
        assert_eq!(wedge.r#type, WedgeType::Diminuendo);
        assert_eq!(wedge.niente, Some(YesNo::Yes));
    }

    #[test]
    fn test_wedge_stop() {
        let wedge = Wedge {
            r#type: WedgeType::Stop,
            number: Some(1),
            spread: None,
            niente: None,
            line_type: None,
            position: Position::default(),
            color: None,
        };
        assert_eq!(wedge.r#type, WedgeType::Stop);
    }

    // === WedgeType Tests ===

    #[test]
    fn test_wedgetype_all_variants() {
        assert_eq!(WedgeType::Crescendo, WedgeType::Crescendo);
        assert_eq!(WedgeType::Diminuendo, WedgeType::Diminuendo);
        assert_eq!(WedgeType::Stop, WedgeType::Stop);
        assert_eq!(WedgeType::Continue, WedgeType::Continue);
    }

    // === Pedal Tests ===

    #[test]
    fn test_pedal_start() {
        let pedal = Pedal {
            r#type: PedalType::Start,
            number: None,
            line: Some(YesNo::Yes),
            sign: Some(YesNo::Yes),
            abbreviated: None,
            print_style: PrintStyle::default(),
        };
        assert_eq!(pedal.r#type, PedalType::Start);
    }

    #[test]
    fn test_pedal_stop() {
        let pedal = Pedal {
            r#type: PedalType::Stop,
            number: None,
            line: None,
            sign: None,
            abbreviated: None,
            print_style: PrintStyle::default(),
        };
        assert_eq!(pedal.r#type, PedalType::Stop);
    }

    // === PedalType Tests ===

    #[test]
    fn test_pedaltype_all_variants() {
        assert_eq!(PedalType::Start, PedalType::Start);
        assert_eq!(PedalType::Stop, PedalType::Stop);
        assert_eq!(PedalType::Sostenuto, PedalType::Sostenuto);
        assert_eq!(PedalType::Change, PedalType::Change);
        assert_eq!(PedalType::Continue, PedalType::Continue);
        assert_eq!(PedalType::Discontinue, PedalType::Discontinue);
        assert_eq!(PedalType::Resume, PedalType::Resume);
    }

    // === Metronome Tests ===

    #[test]
    fn test_metronome_per_minute() {
        let metronome = Metronome {
            parentheses: None,
            content: MetronomeContent::PerMinute {
                beat_unit: NoteTypeValue::Quarter,
                beat_unit_dots: 0,
                per_minute: PerMinute {
                    value: "120".to_string(),
                    font: Font::default(),
                },
            },
            print_style: PrintStyle::default(),
        };
        if let MetronomeContent::PerMinute { per_minute, .. } = metronome.content {
            assert_eq!(per_minute.value, "120");
        }
    }

    #[test]
    fn test_metronome_beat_equation() {
        let metronome = Metronome {
            parentheses: Some(YesNo::Yes),
            content: MetronomeContent::BeatEquation {
                left_unit: NoteTypeValue::Half,
                left_dots: 0,
                right_unit: NoteTypeValue::Quarter,
                right_dots: 1,
            },
            print_style: PrintStyle::default(),
        };
        if let MetronomeContent::BeatEquation {
            left_unit,
            right_unit,
            ..
        } = metronome.content
        {
            assert_eq!(left_unit, NoteTypeValue::Half);
            assert_eq!(right_unit, NoteTypeValue::Quarter);
        }
    }

    // === OctaveShift Tests ===

    #[test]
    fn test_octaveshift_8va() {
        let shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(8),
            position: Position::default(),
        };
        assert_eq!(shift.r#type, UpDownStopContinue::Up);
        assert_eq!(shift.size, Some(8));
    }

    #[test]
    fn test_octaveshift_8vb() {
        let shift = OctaveShift {
            r#type: UpDownStopContinue::Down,
            number: None,
            size: Some(8),
            position: Position::default(),
        };
        assert_eq!(shift.r#type, UpDownStopContinue::Down);
    }

    #[test]
    fn test_octaveshift_15ma() {
        let shift = OctaveShift {
            r#type: UpDownStopContinue::Up,
            number: None,
            size: Some(15),
            position: Position::default(),
        };
        assert_eq!(shift.size, Some(15));
    }

    // === UpDownStopContinue Tests ===

    #[test]
    fn test_updownstopcontinue_all_variants() {
        assert_eq!(UpDownStopContinue::Up, UpDownStopContinue::Up);
        assert_eq!(UpDownStopContinue::Down, UpDownStopContinue::Down);
        assert_eq!(UpDownStopContinue::Stop, UpDownStopContinue::Stop);
        assert_eq!(UpDownStopContinue::Continue, UpDownStopContinue::Continue);
    }

    // === Sound Tests ===

    #[test]
    fn test_sound_default() {
        let sound = Sound::default();
        assert!(sound.tempo.is_none());
        assert!(sound.dynamics.is_none());
    }

    #[test]
    fn test_sound_with_tempo() {
        let sound = Sound {
            tempo: Some(120.0),
            ..Default::default()
        };
        assert_eq!(sound.tempo, Some(120.0));
    }

    #[test]
    fn test_sound_with_dynamics() {
        let sound = Sound {
            dynamics: Some(80.0),
            ..Default::default()
        };
        assert_eq!(sound.dynamics, Some(80.0));
    }

    #[test]
    fn test_sound_dacapo() {
        let sound = Sound {
            dacapo: Some(YesNo::Yes),
            ..Default::default()
        };
        assert_eq!(sound.dacapo, Some(YesNo::Yes));
    }

    // === HarpPedals Tests ===

    #[test]
    fn test_harppedals_basic() {
        let pedals = HarpPedals {
            pedal_tuning: vec![
                PedalTuning {
                    pedal_step: Step::D,
                    pedal_alter: 0.0,
                },
                PedalTuning {
                    pedal_step: Step::C,
                    pedal_alter: 1.0,
                },
            ],
        };
        assert_eq!(pedals.pedal_tuning.len(), 2);
    }

    // === StringMute Tests ===

    #[test]
    fn test_stringmute_on() {
        let mute = StringMute { r#type: OnOff::On };
        assert_eq!(mute.r#type, OnOff::On);
    }

    #[test]
    fn test_stringmute_off() {
        let mute = StringMute { r#type: OnOff::Off };
        assert_eq!(mute.r#type, OnOff::Off);
    }

    // === OnOff Tests ===

    #[test]
    fn test_onoff_all_variants() {
        assert_eq!(OnOff::On, OnOff::On);
        assert_eq!(OnOff::Off, OnOff::Off);
    }

    // === Scordatura Tests ===

    #[test]
    fn test_scordatura_default() {
        let scordatura = Scordatura::default();
        assert!(scordatura.accord.is_empty());
    }

    #[test]
    fn test_scordatura_with_accord() {
        let scordatura = Scordatura {
            accord: vec![Accord {
                string: 6,
                tuning_step: Step::D,
                tuning_alter: None,
                tuning_octave: 2,
            }],
        };
        assert_eq!(scordatura.accord.len(), 1);
        assert_eq!(scordatura.accord[0].tuning_step, Step::D);
    }

    // === AccordionRegistration Tests ===

    #[test]
    fn test_accordionregistration_default() {
        let reg = AccordionRegistration::default();
        assert!(!reg.accordion_high);
        assert!(reg.accordion_middle.is_none());
        assert!(!reg.accordion_low);
    }

    #[test]
    fn test_accordionregistration_full() {
        let reg = AccordionRegistration {
            accordion_high: true,
            accordion_middle: Some(2),
            accordion_low: true,
        };
        assert!(reg.accordion_high);
        assert_eq!(reg.accordion_middle, Some(2));
        assert!(reg.accordion_low);
    }

    // === StaffDivide Tests ===

    #[test]
    fn test_staffdivide_all_symbols() {
        assert_eq!(StaffDivideSymbol::Down, StaffDivideSymbol::Down);
        assert_eq!(StaffDivideSymbol::Up, StaffDivideSymbol::Up);
        assert_eq!(StaffDivideSymbol::UpDown, StaffDivideSymbol::UpDown);
    }

    // === PrincipalVoice Tests ===

    #[test]
    fn test_principalvoice_hauptstimme() {
        let pv = PrincipalVoice {
            r#type: StartStop::Start,
            symbol: PrincipalVoiceSymbol::Hauptstimme,
        };
        assert_eq!(pv.symbol, PrincipalVoiceSymbol::Hauptstimme);
    }

    #[test]
    fn test_principalvoice_nebenstimme() {
        let pv = PrincipalVoice {
            r#type: StartStop::Start,
            symbol: PrincipalVoiceSymbol::Nebenstimme,
        };
        assert_eq!(pv.symbol, PrincipalVoiceSymbol::Nebenstimme);
    }

    // === PrincipalVoiceSymbol Tests ===

    #[test]
    fn test_principalvoicesymbol_all_variants() {
        assert_eq!(
            PrincipalVoiceSymbol::Hauptstimme,
            PrincipalVoiceSymbol::Hauptstimme
        );
        assert_eq!(
            PrincipalVoiceSymbol::Nebenstimme,
            PrincipalVoiceSymbol::Nebenstimme
        );
        assert_eq!(PrincipalVoiceSymbol::Plain, PrincipalVoiceSymbol::Plain);
        assert_eq!(PrincipalVoiceSymbol::None, PrincipalVoiceSymbol::None);
    }

    // === LineEnd Tests ===

    #[test]
    fn test_lineend_all_variants() {
        assert_eq!(LineEnd::Up, LineEnd::Up);
        assert_eq!(LineEnd::Down, LineEnd::Down);
        assert_eq!(LineEnd::Both, LineEnd::Both);
        assert_eq!(LineEnd::Arrow, LineEnd::Arrow);
        assert_eq!(LineEnd::None, LineEnd::None);
    }

    // === Offset Tests ===

    #[test]
    fn test_offset_basic() {
        let offset = Offset {
            value: 2,
            sound: None,
        };
        assert_eq!(offset.value, 2);
    }

    #[test]
    fn test_offset_with_sound() {
        let offset = Offset {
            value: -1,
            sound: Some(YesNo::Yes),
        };
        assert_eq!(offset.value, -1);
        assert_eq!(offset.sound, Some(YesNo::Yes));
    }

    // === Image Tests ===

    #[test]
    fn test_image_basic() {
        let image = Image {
            source: "image.png".to_string(),
            r#type: "image/png".to_string(),
            position: Position::default(),
        };
        assert_eq!(image.source, "image.png");
        assert_eq!(image.r#type, "image/png");
    }

    // === OtherDirection Tests ===

    #[test]
    fn test_otherdirection_basic() {
        let other = OtherDirection {
            value: "custom".to_string(),
            print_object: Some(YesNo::Yes),
            print_style: PrintStyle::default(),
        };
        assert_eq!(other.value, "custom");
    }
}
