//! Notations: articulations, ornaments, technical, slurs, etc.

use super::common::{
    AboveBelow, AccidentalValue, Color, Editorial, EmptyPlacement, Font, LineType, NumberLevel,
    OverUnder, Position, PrintStyle, StartStop, StartStopContinue, StartStopSingle, UpDown,
    UprightInverted, WavyLine, YesNo,
};

/// Notations container (attached to notes).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Notations {
    /// Whether to print this notation
    pub print_object: Option<YesNo>,
    /// The notation content
    pub content: Vec<NotationContent>,
    /// Editorial information
    pub editorial: Editorial,
}

/// Notation content variants.
#[derive(Debug, Clone, PartialEq)]
pub enum NotationContent {
    /// Tied (visual tie marking)
    Tied(Tied),
    /// Slur
    Slur(Slur),
    /// Tuplet bracket (boxed to reduce enum size)
    Tuplet(Box<Tuplet>),
    /// Glissando
    Glissando(Glissando),
    /// Slide (portamento)
    Slide(Slide),
    /// Ornaments container
    Ornaments(Box<Ornaments>),
    /// Technical indications container
    Technical(Box<Technical>),
    /// Articulations container
    Articulations(Box<Articulations>),
    /// Dynamics
    Dynamics(Box<super::direction::Dynamics>),
    /// Fermata
    Fermata(Fermata),
    /// Arpeggiate
    Arpeggiate(Arpeggiate),
    /// Non-arpeggiate (bracket)
    NonArpeggiate(NonArpeggiate),
    /// Accidental mark in ornaments
    AccidentalMark(AccidentalMark),
    /// Other notation
    OtherNotation(OtherNotation),
}

/// Tied (visual tie marking).
#[derive(Debug, Clone, PartialEq)]
pub struct Tied {
    /// Start, stop, or continue
    pub r#type: StartStopContinue,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Line type
    pub line_type: Option<LineType>,
    /// Position attributes
    pub position: Position,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Orientation over or under
    pub orientation: Option<OverUnder>,
    /// Tie color
    pub color: Option<Color>,
}

/// Slur marking.
#[derive(Debug, Clone, PartialEq)]
pub struct Slur {
    /// Start, stop, or continue
    pub r#type: StartStopContinue,
    /// Number level (required for slurs)
    pub number: NumberLevel,
    /// Line type
    pub line_type: Option<LineType>,
    /// Position attributes
    pub position: Position,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Orientation over or under
    pub orientation: Option<OverUnder>,
    /// Slur color
    pub color: Option<Color>,
}

/// Tuplet notation (visual bracket/number).
#[derive(Debug, Clone, PartialEq)]
pub struct Tuplet {
    /// Start or stop
    pub r#type: StartStop,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Whether to show bracket
    pub bracket: Option<YesNo>,
    /// What to show for the number
    pub show_number: Option<ShowTuplet>,
    /// What to show for the type
    pub show_type: Option<ShowTuplet>,
    /// Line shape (straight or curved)
    pub line_shape: Option<LineShape>,
    /// Position attributes
    pub position: Position,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Actual portion information
    pub tuplet_actual: Option<TupletPortion>,
    /// Normal portion information
    pub tuplet_normal: Option<TupletPortion>,
}

/// What to show in a tuplet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowTuplet {
    /// Show actual number
    Actual,
    /// Show both actual and normal
    Both,
    /// Show nothing
    None,
}

/// Line shape for tuplet brackets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineShape {
    /// Straight line
    Straight,
    /// Curved line
    Curved,
}

/// A portion of a tuplet (actual or normal).
#[derive(Debug, Clone, PartialEq)]
pub struct TupletPortion {
    /// Tuplet number
    pub tuplet_number: Option<TupletNumber>,
    /// Tuplet type (note type)
    pub tuplet_type: Option<TupletType>,
    /// Tuplet dots
    pub tuplet_dots: Vec<TupletDot>,
}

/// Tuplet number display.
#[derive(Debug, Clone, PartialEq)]
pub struct TupletNumber {
    /// The number value
    pub value: u32,
    /// Font attributes
    pub font: Font,
    /// Color
    pub color: Option<Color>,
}

/// Tuplet type display.
#[derive(Debug, Clone, PartialEq)]
pub struct TupletType {
    /// The note type value
    pub value: super::duration::NoteTypeValue,
    /// Font attributes
    pub font: Font,
    /// Color
    pub color: Option<Color>,
}

/// Tuplet dot display.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TupletDot {
    /// Font attributes
    pub font: Font,
    /// Color
    pub color: Option<Color>,
}

/// Glissando.
#[derive(Debug, Clone, PartialEq)]
pub struct Glissando {
    /// Start or stop
    pub r#type: StartStop,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Text to display (e.g., "gliss.")
    pub text: Option<String>,
    /// Line type
    pub line_type: Option<LineType>,
    /// Position attributes
    pub position: Position,
}

/// Slide (portamento).
#[derive(Debug, Clone, PartialEq)]
pub struct Slide {
    /// Start or stop
    pub r#type: StartStop,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Text to display
    pub text: Option<String>,
    /// Line type
    pub line_type: Option<LineType>,
    /// Position attributes
    pub position: Position,
}

/// Fermata.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Fermata {
    /// Fermata shape
    pub shape: Option<FermataShape>,
    /// Upright or inverted
    pub r#type: Option<UprightInverted>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Fermata shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FermataShape {
    /// Normal fermata
    Normal,
    /// Angled fermata
    Angled,
    /// Square fermata
    Square,
    /// Double-angled fermata
    DoubleAngled,
    /// Double-square fermata
    DoubleSquare,
    /// Double-dot fermata
    DoubleDot,
    /// Half-curve fermata
    HalfCurve,
    /// Curlew fermata
    Curlew,
}

/// Arpeggiate.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Arpeggiate {
    /// Number level
    pub number: Option<NumberLevel>,
    /// Direction (up or down)
    pub direction: Option<UpDown>,
    /// Position attributes
    pub position: Position,
    /// Color
    pub color: Option<Color>,
}

/// Non-arpeggiate (bracket).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NonArpeggiate {
    /// Top or bottom
    pub r#type: TopBottom,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Position attributes
    pub position: Position,
    /// Color
    pub color: Option<Color>,
}

/// Top or bottom for non-arpeggiate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TopBottom {
    /// Top
    #[default]
    Top,
    /// Bottom
    Bottom,
}

/// Accidental mark in ornaments.
#[derive(Debug, Clone, PartialEq)]
pub struct AccidentalMark {
    /// The accidental value
    pub value: AccidentalValue,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Other notation.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherNotation {
    /// The notation value
    pub value: String,
    /// Start, stop, or single
    pub r#type: StartStopSingle,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Whether to print
    pub print_object: Option<YesNo>,
    /// Print style attributes
    pub print_style: PrintStyle,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
}

// === Articulations ===

/// Articulations container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Articulations {
    /// The articulation elements
    pub content: Vec<ArticulationElement>,
}

/// Individual articulation types.
#[derive(Debug, Clone, PartialEq)]
pub enum ArticulationElement {
    /// Accent
    Accent(EmptyPlacement),
    /// Strong accent (marcato)
    StrongAccent(StrongAccent),
    /// Staccato
    Staccato(EmptyPlacement),
    /// Tenuto
    Tenuto(EmptyPlacement),
    /// Detached legato
    DetachedLegato(EmptyPlacement),
    /// Staccatissimo
    Staccatissimo(EmptyPlacement),
    /// Spiccato
    Spiccato(EmptyPlacement),
    /// Scoop
    Scoop(EmptyLine),
    /// Plop
    Plop(EmptyLine),
    /// Doit
    Doit(EmptyLine),
    /// Falloff
    Falloff(EmptyLine),
    /// Breath mark
    BreathMark(BreathMark),
    /// Caesura
    Caesura(Caesura),
    /// Stress
    Stress(EmptyPlacement),
    /// Unstress
    Unstress(EmptyPlacement),
    /// Soft accent
    SoftAccent(EmptyPlacement),
    /// Other articulation
    OtherArticulation(OtherArticulation),
}

/// Strong accent (marcato).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StrongAccent {
    /// Direction (up or down)
    pub r#type: Option<UpDown>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Empty line element for jazz articulations.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyLine {
    /// Line shape
    pub line_shape: Option<LineShape>,
    /// Line type
    pub line_type: Option<LineType>,
    /// Line length
    pub line_length: Option<LineLength>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Line length for jazz articulations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineLength {
    /// Short line
    Short,
    /// Medium line
    Medium,
    /// Long line
    Long,
}

/// Breath mark.
#[derive(Debug, Clone, PartialEq)]
pub struct BreathMark {
    /// Breath mark value
    pub value: BreathMarkValue,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Breath mark values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreathMarkValue {
    /// Empty (default comma)
    Empty,
    /// Comma
    Comma,
    /// Tick
    Tick,
    /// Upbow
    Upbow,
    /// Salzedo
    Salzedo,
}

/// Caesura.
#[derive(Debug, Clone, PartialEq)]
pub struct Caesura {
    /// Caesura value
    pub value: CaesuraValue,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Caesura values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaesuraValue {
    /// Normal caesura
    Normal,
    /// Thick caesura
    Thick,
    /// Short caesura
    Short,
    /// Curved caesura
    Curved,
    /// Single caesura
    Single,
}

/// Other articulation.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherArticulation {
    /// The articulation value
    pub value: String,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

// === Ornaments ===

/// Ornaments container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ornaments {
    /// The ornament elements with their accidental marks
    pub content: Vec<OrnamentWithAccidentals>,
}

/// Ornament with optional accidental marks.
#[derive(Debug, Clone, PartialEq)]
pub struct OrnamentWithAccidentals {
    /// The ornament element
    pub ornament: OrnamentElement,
    /// Accidental marks for the ornament
    pub accidental_marks: Vec<AccidentalMark>,
}

/// Individual ornament types.
#[derive(Debug, Clone, PartialEq)]
pub enum OrnamentElement {
    /// Trill mark
    TrillMark(EmptyTrillSound),
    /// Turn
    Turn(Turn),
    /// Delayed turn
    DelayedTurn(Turn),
    /// Inverted turn
    InvertedTurn(Turn),
    /// Delayed inverted turn
    DelayedInvertedTurn(Turn),
    /// Vertical turn
    VerticalTurn(EmptyTrillSound),
    /// Inverted vertical turn
    InvertedVerticalTurn(EmptyTrillSound),
    /// Shake
    Shake(EmptyTrillSound),
    /// Wavy line
    WavyLine(WavyLine),
    /// Mordent
    Mordent(Mordent),
    /// Inverted mordent
    InvertedMordent(Mordent),
    /// Schleifer
    Schleifer(EmptyPlacement),
    /// Tremolo
    Tremolo(Tremolo),
    /// Haydn
    Haydn(EmptyTrillSound),
    /// Other ornament
    OtherOrnament(OtherOrnament),
}

/// Empty trill sound attributes.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmptyTrillSound {
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
    /// Start note
    pub start_note: Option<StartNote>,
    /// Trill step
    pub trill_step: Option<TrillStep>,
    /// Two-note turn
    pub two_note_turn: Option<TwoNoteTurn>,
    /// Accelerate
    pub accelerate: Option<YesNo>,
    /// Number of beats
    pub beats: Option<f64>,
    /// Second beat percentage
    pub second_beat: Option<f64>,
    /// Last beat percentage
    pub last_beat: Option<f64>,
}

/// Start note for trills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartNote {
    /// Upper note
    Upper,
    /// Main note
    Main,
    /// Below note
    Below,
}

/// Trill step interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrillStep {
    /// Whole step
    Whole,
    /// Half step
    Half,
    /// Unison
    Unison,
}

/// Two-note turn interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoNoteTurn {
    /// Whole step
    Whole,
    /// Half step
    Half,
    /// None
    None,
}

/// Turn ornament.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Turn {
    /// Slash through the turn
    pub slash: Option<YesNo>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
    /// Start note
    pub start_note: Option<StartNote>,
    /// Trill step
    pub trill_step: Option<TrillStep>,
    /// Two-note turn
    pub two_note_turn: Option<TwoNoteTurn>,
    /// Accelerate
    pub accelerate: Option<YesNo>,
    /// Number of beats
    pub beats: Option<f64>,
    /// Second beat percentage
    pub second_beat: Option<f64>,
    /// Last beat percentage
    pub last_beat: Option<f64>,
}

/// Mordent ornament.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mordent {
    /// Long mordent
    pub long: Option<YesNo>,
    /// Approach direction
    pub approach: Option<AboveBelow>,
    /// Departure direction
    pub departure: Option<AboveBelow>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
    /// Start note
    pub start_note: Option<StartNote>,
    /// Trill step
    pub trill_step: Option<TrillStep>,
    /// Two-note turn
    pub two_note_turn: Option<TwoNoteTurn>,
    /// Accelerate
    pub accelerate: Option<YesNo>,
    /// Number of beats
    pub beats: Option<f64>,
    /// Second beat percentage
    pub second_beat: Option<f64>,
    /// Last beat percentage
    pub last_beat: Option<f64>,
}

/// Tremolo ornament.
#[derive(Debug, Clone, PartialEq)]
pub struct Tremolo {
    /// Number of tremolo marks (1-8)
    pub value: u8,
    /// Tremolo type
    pub r#type: Option<TremoloType>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Position attributes
    pub position: Position,
}

/// Tremolo types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TremoloType {
    /// Start of two-note tremolo
    Start,
    /// Stop of two-note tremolo
    Stop,
    /// Single-note tremolo
    Single,
    /// Unmeasured tremolo
    Unmeasured,
}

/// Other ornament.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherOrnament {
    /// The ornament value
    pub value: String,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

// === Technical ===

/// Technical indications.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Technical {
    /// The technical elements
    pub content: Vec<TechnicalElement>,
}

/// Individual technical elements.
#[derive(Debug, Clone, PartialEq)]
pub enum TechnicalElement {
    /// Up-bow
    UpBow(EmptyPlacement),
    /// Down-bow
    DownBow(EmptyPlacement),
    /// Harmonic
    Harmonic(Harmonic),
    /// Open string
    OpenString(EmptyPlacement),
    /// Thumb position
    ThumbPosition(EmptyPlacement),
    /// Fingering
    Fingering(Fingering),
    /// Pluck
    Pluck(Pluck),
    /// Double tongue
    DoubleTongue(EmptyPlacement),
    /// Triple tongue
    TripleTongue(EmptyPlacement),
    /// Stopped
    Stopped(EmptyPlacement),
    /// Snap pizzicato
    SnapPizzicato(EmptyPlacement),
    /// Fret number
    Fret(Fret),
    /// String number
    String(StringNumber),
    /// Hammer-on
    HammerOn(HammerPull),
    /// Pull-off
    PullOff(HammerPull),
    /// Bend
    Bend(Bend),
    /// Tap
    Tap(Tap),
    /// Heel
    Heel(HeelToe),
    /// Toe
    Toe(HeelToe),
    /// Fingernails
    Fingernails(EmptyPlacement),
    /// Hole
    Hole(Hole),
    /// Arrow
    Arrow(Arrow),
    /// Handbell
    Handbell(Handbell),
    /// Brass bend
    BrassBend(EmptyPlacement),
    /// Flip
    Flip(EmptyPlacement),
    /// Smear
    Smear(EmptyPlacement),
    /// Open
    Open(EmptyPlacement),
    /// Half-muted
    HalfMuted(EmptyPlacement),
    /// Harmon mute
    HarmonMute(HarmonMute),
    /// Golpe
    Golpe(EmptyPlacement),
    /// Other technical
    OtherTechnical(OtherTechnical),
}

// Simplified technical types

/// Harmonic indication.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Harmonic {
    /// Natural harmonic
    pub natural: bool,
    /// Artificial harmonic
    pub artificial: bool,
    /// Base pitch shown
    pub base_pitch: bool,
    /// Touching pitch shown
    pub touching_pitch: bool,
    /// Sounding pitch shown
    pub sounding_pitch: bool,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Whether to print
    pub print_object: Option<YesNo>,
}

/// Fingering indication.
#[derive(Debug, Clone, PartialEq)]
pub struct Fingering {
    /// Fingering value (e.g., "1", "2", "p", "i", "m", "a")
    pub value: String,
    /// Substitution fingering
    pub substitution: Option<YesNo>,
    /// Alternate fingering
    pub alternate: Option<YesNo>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Pluck indication.
#[derive(Debug, Clone, PartialEq)]
pub struct Pluck {
    /// Pluck value
    pub value: String,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
}

/// Fret number.
#[derive(Debug, Clone, PartialEq)]
pub struct Fret {
    /// Fret number value
    pub value: u8,
    /// Font attributes
    pub font: Font,
    /// Color
    pub color: Option<Color>,
}

/// String number.
#[derive(Debug, Clone, PartialEq)]
pub struct StringNumber {
    /// String number value
    pub value: u8,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

/// Hammer-on or pull-off.
#[derive(Debug, Clone, PartialEq)]
pub struct HammerPull {
    /// Text value (e.g., "H", "P")
    pub value: String,
    /// Start or stop
    pub r#type: StartStop,
    /// Number level
    pub number: Option<NumberLevel>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
}

/// Bend.
#[derive(Debug, Clone, PartialEq)]
pub struct Bend {
    /// Bend alteration in semitones
    pub bend_alter: super::common::Semitones,
    /// Pre-bend
    pub pre_bend: bool,
    /// Release type
    pub release: Option<BendRelease>,
    /// With bar text
    pub with_bar: Option<String>,
}

/// Bend release timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BendRelease {
    /// Early release
    Early,
    /// Late release
    Late,
}

/// Tap indication.
#[derive(Debug, Clone, PartialEq)]
pub struct Tap {
    /// Tap value
    pub value: String,
    /// Which hand
    pub hand: Option<TapHand>,
}

/// Tap hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TapHand {
    /// Left hand
    Left,
    /// Right hand
    Right,
}

/// Heel or toe indication.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeelToe {
    /// Substitution
    pub substitution: Option<YesNo>,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
}

/// Hole indication for wind instruments.
#[derive(Debug, Clone, PartialEq)]
pub struct Hole {
    /// Hole type
    pub hole_type: Option<String>,
    /// Hole closed state
    pub hole_closed: HoleClosed,
    /// Hole shape
    pub hole_shape: Option<String>,
}

/// Hole closed state.
#[derive(Debug, Clone, PartialEq)]
pub struct HoleClosed {
    /// Hole closed value
    pub value: HoleClosedValue,
    /// Hole closed location
    pub location: Option<HoleClosedLocation>,
}

/// Hole closed values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoleClosedValue {
    /// Yes (fully closed)
    Yes,
    /// No (open)
    No,
    /// Half (half-covered)
    Half,
}

/// Hole closed location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoleClosedLocation {
    /// Right side
    Right,
    /// Bottom
    Bottom,
    /// Left side
    Left,
    /// Top
    Top,
}

/// Arrow indication.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Arrow {
    /// Arrow direction
    pub direction: Option<ArrowDirection>,
    /// Arrow style
    pub style: Option<ArrowStyle>,
    /// SMuFL glyph
    pub smufl: Option<String>,
}

/// Arrow direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowDirection {
    /// Left
    Left,
    /// Up
    Up,
    /// Right
    Right,
    /// Down
    Down,
    /// Northwest
    Northwest,
    /// Northeast
    Northeast,
    /// Southeast
    Southeast,
    /// Southwest
    Southwest,
    /// Left-right
    LeftRight,
    /// Up-down
    UpDown,
    /// Northwest-southeast
    NorthwestSoutheast,
    /// Northeast-southwest
    NortheastSouthwest,
    /// Other
    Other,
}

/// Arrow style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowStyle {
    /// Single arrow
    Single,
    /// Double arrow
    Double,
    /// Filled arrow
    Filled,
    /// Hollow arrow
    Hollow,
    /// Paired arrows
    Paired,
    /// Combined arrows
    Combined,
    /// Other style
    Other,
}

/// Handbell indication.
#[derive(Debug, Clone, PartialEq)]
pub struct Handbell {
    /// Handbell technique value
    pub value: HandbellValue,
}

/// Handbell values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandbellValue {
    /// Belltree
    Belltree,
    /// Damp
    Damp,
    /// Echo
    Echo,
    /// Gyro
    Gyro,
    /// Hand martellato
    HandMartellato,
    /// Mallet lift
    MalletLift,
    /// Mallet table
    MalletTable,
    /// Martellato
    Martellato,
    /// Martellato lift
    MartellatoLift,
    /// Muted martellato
    MutedMartellato,
    /// Pluck lift
    PluckLift,
    /// Swing
    Swing,
}

/// Harmon mute.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarmonMute {
    /// Open
    pub open: bool,
    /// Half
    pub half: bool,
}

/// Other technical indication.
#[derive(Debug, Clone, PartialEq)]
pub struct OtherTechnical {
    /// The technical value
    pub value: String,
    /// Placement above or below
    pub placement: Option<AboveBelow>,
    /// Print style attributes
    pub print_style: PrintStyle,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::duration::NoteTypeValue;

    // === Notations Tests ===

    #[test]
    fn test_notations_default() {
        let notations = Notations::default();
        assert!(notations.print_object.is_none());
        assert!(notations.content.is_empty());
    }

    #[test]
    fn test_notations_with_content() {
        let notations = Notations {
            print_object: Some(YesNo::Yes),
            content: vec![NotationContent::Fermata(Fermata::default())],
            editorial: Editorial::default(),
        };
        assert_eq!(notations.content.len(), 1);
    }

    // === NotationContent Tests ===

    #[test]
    fn test_notationcontent_tied() {
        let content = NotationContent::Tied(Tied {
            r#type: StartStopContinue::Start,
            number: None,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        });
        if let NotationContent::Tied(t) = content {
            assert_eq!(t.r#type, StartStopContinue::Start);
        }
    }

    #[test]
    fn test_notationcontent_slur() {
        let content = NotationContent::Slur(Slur {
            r#type: StartStopContinue::Start,
            number: 1,
            line_type: None,
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            orientation: None,
            color: None,
        });
        if let NotationContent::Slur(s) = content {
            assert_eq!(s.number, 1);
        }
    }

    #[test]
    fn test_notationcontent_fermata() {
        let content = NotationContent::Fermata(Fermata {
            shape: Some(FermataShape::Normal),
            r#type: Some(UprightInverted::Upright),
            print_style: PrintStyle::default(),
        });
        if let NotationContent::Fermata(f) = content {
            assert_eq!(f.shape, Some(FermataShape::Normal));
        }
    }

    // === Tied Tests ===

    #[test]
    fn test_tied_start() {
        let tied = Tied {
            r#type: StartStopContinue::Start,
            number: Some(1),
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        };
        assert_eq!(tied.r#type, StartStopContinue::Start);
    }

    #[test]
    fn test_tied_stop() {
        let tied = Tied {
            r#type: StartStopContinue::Stop,
            number: Some(1),
            line_type: Some(LineType::Solid),
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            orientation: Some(OverUnder::Over),
            color: Some("#000000".to_string()),
        };
        assert_eq!(tied.r#type, StartStopContinue::Stop);
    }

    // === Slur Tests ===

    #[test]
    fn test_slur_basic() {
        let slur = Slur {
            r#type: StartStopContinue::Start,
            number: 1,
            line_type: None,
            position: Position::default(),
            placement: None,
            orientation: None,
            color: None,
        };
        assert_eq!(slur.number, 1);
    }

    #[test]
    fn test_slur_with_all_attrs() {
        let slur = Slur {
            r#type: StartStopContinue::Stop,
            number: 2,
            line_type: Some(LineType::Dashed),
            position: Position::default(),
            placement: Some(AboveBelow::Below),
            orientation: Some(OverUnder::Under),
            color: Some("#0000FF".to_string()),
        };
        assert_eq!(slur.line_type, Some(LineType::Dashed));
    }

    // === Tuplet Tests ===

    #[test]
    fn test_tuplet_start() {
        let tuplet = Tuplet {
            r#type: StartStop::Start,
            number: Some(1),
            bracket: Some(YesNo::Yes),
            show_number: Some(ShowTuplet::Actual),
            show_type: None,
            line_shape: Some(LineShape::Curved),
            position: Position::default(),
            placement: Some(AboveBelow::Above),
            tuplet_actual: None,
            tuplet_normal: None,
        };
        assert_eq!(tuplet.r#type, StartStop::Start);
        assert_eq!(tuplet.bracket, Some(YesNo::Yes));
    }

    #[test]
    fn test_tuplet_with_portions() {
        let tuplet = Tuplet {
            r#type: StartStop::Start,
            number: None,
            bracket: None,
            show_number: Some(ShowTuplet::Both),
            show_type: Some(ShowTuplet::None),
            line_shape: None,
            position: Position::default(),
            placement: None,
            tuplet_actual: Some(TupletPortion {
                tuplet_number: Some(TupletNumber {
                    value: 3,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_type: None,
                tuplet_dots: vec![],
            }),
            tuplet_normal: Some(TupletPortion {
                tuplet_number: Some(TupletNumber {
                    value: 2,
                    font: Font::default(),
                    color: None,
                }),
                tuplet_type: None,
                tuplet_dots: vec![],
            }),
        };
        assert!(tuplet.tuplet_actual.is_some());
        assert!(tuplet.tuplet_normal.is_some());
    }

    // === ShowTuplet Tests ===

    #[test]
    fn test_showtuplet_all_variants() {
        assert_eq!(ShowTuplet::Actual, ShowTuplet::Actual);
        assert_eq!(ShowTuplet::Both, ShowTuplet::Both);
        assert_eq!(ShowTuplet::None, ShowTuplet::None);
    }

    // === LineShape Tests ===

    #[test]
    fn test_lineshape_all_variants() {
        assert_eq!(LineShape::Straight, LineShape::Straight);
        assert_eq!(LineShape::Curved, LineShape::Curved);
    }

    // === Glissando Tests ===

    #[test]
    fn test_glissando_start() {
        let gliss = Glissando {
            r#type: StartStop::Start,
            number: Some(1),
            text: Some("gliss.".to_string()),
            line_type: Some(LineType::Wavy),
            position: Position::default(),
        };
        assert_eq!(gliss.text, Some("gliss.".to_string()));
    }

    // === Slide Tests ===

    #[test]
    fn test_slide_basic() {
        let slide = Slide {
            r#type: StartStop::Start,
            number: None,
            text: None,
            line_type: Some(LineType::Solid),
            position: Position::default(),
        };
        assert_eq!(slide.line_type, Some(LineType::Solid));
    }

    // === Fermata Tests ===

    #[test]
    fn test_fermata_default() {
        let fermata = Fermata::default();
        assert!(fermata.shape.is_none());
        assert!(fermata.r#type.is_none());
    }

    #[test]
    fn test_fermata_normal() {
        let fermata = Fermata {
            shape: Some(FermataShape::Normal),
            r#type: Some(UprightInverted::Upright),
            print_style: PrintStyle::default(),
        };
        assert_eq!(fermata.shape, Some(FermataShape::Normal));
    }

    #[test]
    fn test_fermata_inverted() {
        let fermata = Fermata {
            shape: Some(FermataShape::Normal),
            r#type: Some(UprightInverted::Inverted),
            print_style: PrintStyle::default(),
        };
        assert_eq!(fermata.r#type, Some(UprightInverted::Inverted));
    }

    // === FermataShape Tests ===

    #[test]
    fn test_fermatashape_all_variants() {
        assert_eq!(FermataShape::Normal, FermataShape::Normal);
        assert_eq!(FermataShape::Angled, FermataShape::Angled);
        assert_eq!(FermataShape::Square, FermataShape::Square);
        assert_eq!(FermataShape::DoubleAngled, FermataShape::DoubleAngled);
        assert_eq!(FermataShape::DoubleSquare, FermataShape::DoubleSquare);
        assert_eq!(FermataShape::DoubleDot, FermataShape::DoubleDot);
        assert_eq!(FermataShape::HalfCurve, FermataShape::HalfCurve);
        assert_eq!(FermataShape::Curlew, FermataShape::Curlew);
    }

    // === Arpeggiate Tests ===

    #[test]
    fn test_arpeggiate_default() {
        let arp = Arpeggiate::default();
        assert!(arp.number.is_none());
        assert!(arp.direction.is_none());
    }

    #[test]
    fn test_arpeggiate_up() {
        let arp = Arpeggiate {
            number: Some(1),
            direction: Some(UpDown::Up),
            position: Position::default(),
            color: None,
        };
        assert_eq!(arp.direction, Some(UpDown::Up));
    }

    // === NonArpeggiate Tests ===

    #[test]
    fn test_nonarpeggiate_default() {
        let na = NonArpeggiate::default();
        assert_eq!(na.r#type, TopBottom::Top);
    }

    #[test]
    fn test_nonarpeggiate_bottom() {
        let na = NonArpeggiate {
            r#type: TopBottom::Bottom,
            number: Some(1),
            position: Position::default(),
            color: None,
        };
        assert_eq!(na.r#type, TopBottom::Bottom);
    }

    // === TopBottom Tests ===

    #[test]
    fn test_topbottom_all_variants() {
        assert_eq!(TopBottom::Top, TopBottom::Top);
        assert_eq!(TopBottom::Bottom, TopBottom::Bottom);
    }

    // === AccidentalMark Tests ===

    #[test]
    fn test_accidentalmark_sharp() {
        let mark = AccidentalMark {
            value: AccidentalValue::Sharp,
            placement: Some(AboveBelow::Above),
            print_style: PrintStyle::default(),
        };
        assert_eq!(mark.value, AccidentalValue::Sharp);
    }

    // === Articulations Tests ===

    #[test]
    fn test_articulations_default() {
        let art = Articulations::default();
        assert!(art.content.is_empty());
    }

    #[test]
    fn test_articulations_accent() {
        let art = Articulations {
            content: vec![ArticulationElement::Accent(EmptyPlacement::default())],
        };
        assert_eq!(art.content.len(), 1);
    }

    // === ArticulationElement Tests ===

    #[test]
    fn test_articulationelement_staccato() {
        let elem = ArticulationElement::Staccato(EmptyPlacement::default());
        if let ArticulationElement::Staccato(_) = elem {
            // Pass
        } else {
            panic!("Expected Staccato");
        }
    }

    #[test]
    fn test_articulationelement_tenuto() {
        let elem = ArticulationElement::Tenuto(EmptyPlacement::default());
        if let ArticulationElement::Tenuto(_) = elem {
            // Pass
        } else {
            panic!("Expected Tenuto");
        }
    }

    #[test]
    fn test_articulationelement_strong_accent() {
        let elem = ArticulationElement::StrongAccent(StrongAccent {
            r#type: Some(UpDown::Up),
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        });
        if let ArticulationElement::StrongAccent(sa) = elem {
            assert_eq!(sa.r#type, Some(UpDown::Up));
        }
    }

    #[test]
    fn test_articulationelement_breath_mark() {
        let elem = ArticulationElement::BreathMark(BreathMark {
            value: BreathMarkValue::Comma,
            placement: Some(AboveBelow::Above),
            position: Position::default(),
        });
        if let ArticulationElement::BreathMark(bm) = elem {
            assert_eq!(bm.value, BreathMarkValue::Comma);
        }
    }

    #[test]
    fn test_articulationelement_caesura() {
        let elem = ArticulationElement::Caesura(Caesura {
            value: CaesuraValue::Normal,
            placement: None,
            position: Position::default(),
        });
        if let ArticulationElement::Caesura(c) = elem {
            assert_eq!(c.value, CaesuraValue::Normal);
        }
    }

    // === BreathMarkValue Tests ===

    #[test]
    fn test_breathmarkvalue_all_variants() {
        assert_eq!(BreathMarkValue::Empty, BreathMarkValue::Empty);
        assert_eq!(BreathMarkValue::Comma, BreathMarkValue::Comma);
        assert_eq!(BreathMarkValue::Tick, BreathMarkValue::Tick);
        assert_eq!(BreathMarkValue::Upbow, BreathMarkValue::Upbow);
        assert_eq!(BreathMarkValue::Salzedo, BreathMarkValue::Salzedo);
    }

    // === CaesuraValue Tests ===

    #[test]
    fn test_caesuravalue_all_variants() {
        assert_eq!(CaesuraValue::Normal, CaesuraValue::Normal);
        assert_eq!(CaesuraValue::Thick, CaesuraValue::Thick);
        assert_eq!(CaesuraValue::Short, CaesuraValue::Short);
        assert_eq!(CaesuraValue::Curved, CaesuraValue::Curved);
        assert_eq!(CaesuraValue::Single, CaesuraValue::Single);
    }

    // === Ornaments Tests ===

    #[test]
    fn test_ornaments_default() {
        let orn = Ornaments::default();
        assert!(orn.content.is_empty());
    }

    #[test]
    fn test_ornaments_trill() {
        let orn = Ornaments {
            content: vec![OrnamentWithAccidentals {
                ornament: OrnamentElement::TrillMark(EmptyTrillSound::default()),
                accidental_marks: vec![],
            }],
        };
        assert_eq!(orn.content.len(), 1);
    }

    // === OrnamentElement Tests ===

    #[test]
    fn test_ornamentelement_turn() {
        let elem = OrnamentElement::Turn(Turn::default());
        if let OrnamentElement::Turn(_) = elem {
            // Pass
        } else {
            panic!("Expected Turn");
        }
    }

    #[test]
    fn test_ornamentelement_mordent() {
        let elem = OrnamentElement::Mordent(Mordent {
            long: Some(YesNo::Yes),
            ..Default::default()
        });
        if let OrnamentElement::Mordent(m) = elem {
            assert_eq!(m.long, Some(YesNo::Yes));
        }
    }

    #[test]
    fn test_ornamentelement_tremolo() {
        let elem = OrnamentElement::Tremolo(Tremolo {
            value: 3,
            r#type: Some(TremoloType::Single),
            placement: None,
            position: Position::default(),
        });
        if let OrnamentElement::Tremolo(t) = elem {
            assert_eq!(t.value, 3);
        }
    }

    // === TremoloType Tests ===

    #[test]
    fn test_tremolotype_all_variants() {
        assert_eq!(TremoloType::Start, TremoloType::Start);
        assert_eq!(TremoloType::Stop, TremoloType::Stop);
        assert_eq!(TremoloType::Single, TremoloType::Single);
        assert_eq!(TremoloType::Unmeasured, TremoloType::Unmeasured);
    }

    // === StartNote Tests ===

    #[test]
    fn test_startnote_all_variants() {
        assert_eq!(StartNote::Upper, StartNote::Upper);
        assert_eq!(StartNote::Main, StartNote::Main);
        assert_eq!(StartNote::Below, StartNote::Below);
    }

    // === TrillStep Tests ===

    #[test]
    fn test_trillstep_all_variants() {
        assert_eq!(TrillStep::Whole, TrillStep::Whole);
        assert_eq!(TrillStep::Half, TrillStep::Half);
        assert_eq!(TrillStep::Unison, TrillStep::Unison);
    }

    // === TwoNoteTurn Tests ===

    #[test]
    fn test_twonoteturn_all_variants() {
        assert_eq!(TwoNoteTurn::Whole, TwoNoteTurn::Whole);
        assert_eq!(TwoNoteTurn::Half, TwoNoteTurn::Half);
        assert_eq!(TwoNoteTurn::None, TwoNoteTurn::None);
    }

    // === Technical Tests ===

    #[test]
    fn test_technical_default() {
        let tech = Technical::default();
        assert!(tech.content.is_empty());
    }

    #[test]
    fn test_technical_upbow() {
        let tech = Technical {
            content: vec![TechnicalElement::UpBow(EmptyPlacement::default())],
        };
        assert_eq!(tech.content.len(), 1);
    }

    // === TechnicalElement Tests ===

    #[test]
    fn test_technicalelement_fingering() {
        let elem = TechnicalElement::Fingering(Fingering {
            value: "1".to_string(),
            substitution: None,
            alternate: None,
            placement: Some(AboveBelow::Above),
            print_style: PrintStyle::default(),
        });
        if let TechnicalElement::Fingering(f) = elem {
            assert_eq!(f.value, "1");
        }
    }

    #[test]
    fn test_technicalelement_fret() {
        let elem = TechnicalElement::Fret(Fret {
            value: 5,
            font: Font::default(),
            color: None,
        });
        if let TechnicalElement::Fret(f) = elem {
            assert_eq!(f.value, 5);
        }
    }

    #[test]
    fn test_technicalelement_string() {
        let elem = TechnicalElement::String(StringNumber {
            value: 1,
            placement: None,
            print_style: PrintStyle::default(),
        });
        if let TechnicalElement::String(s) = elem {
            assert_eq!(s.value, 1);
        }
    }

    #[test]
    fn test_technicalelement_harmonic() {
        let elem = TechnicalElement::Harmonic(Harmonic {
            natural: true,
            artificial: false,
            base_pitch: false,
            touching_pitch: false,
            sounding_pitch: false,
            placement: None,
            print_object: None,
        });
        if let TechnicalElement::Harmonic(h) = elem {
            assert!(h.natural);
        }
    }

    #[test]
    fn test_technicalelement_bend() {
        let elem = TechnicalElement::Bend(Bend {
            bend_alter: 1.0,
            pre_bend: false,
            release: Some(BendRelease::Early),
            with_bar: None,
        });
        if let TechnicalElement::Bend(b) = elem {
            assert_eq!(b.bend_alter, 1.0);
        }
    }

    // === BendRelease Tests ===

    #[test]
    fn test_bendrelease_all_variants() {
        assert_eq!(BendRelease::Early, BendRelease::Early);
        assert_eq!(BendRelease::Late, BendRelease::Late);
    }

    // === TapHand Tests ===

    #[test]
    fn test_taphand_all_variants() {
        assert_eq!(TapHand::Left, TapHand::Left);
        assert_eq!(TapHand::Right, TapHand::Right);
    }

    // === HoleClosedValue Tests ===

    #[test]
    fn test_holeclosedvalue_all_variants() {
        assert_eq!(HoleClosedValue::Yes, HoleClosedValue::Yes);
        assert_eq!(HoleClosedValue::No, HoleClosedValue::No);
        assert_eq!(HoleClosedValue::Half, HoleClosedValue::Half);
    }

    // === HoleClosedLocation Tests ===

    #[test]
    fn test_holeclosedlocation_all_variants() {
        assert_eq!(HoleClosedLocation::Right, HoleClosedLocation::Right);
        assert_eq!(HoleClosedLocation::Bottom, HoleClosedLocation::Bottom);
        assert_eq!(HoleClosedLocation::Left, HoleClosedLocation::Left);
        assert_eq!(HoleClosedLocation::Top, HoleClosedLocation::Top);
    }

    // === ArrowDirection Tests ===

    #[test]
    fn test_arrowdirection_cardinal() {
        assert_eq!(ArrowDirection::Left, ArrowDirection::Left);
        assert_eq!(ArrowDirection::Up, ArrowDirection::Up);
        assert_eq!(ArrowDirection::Right, ArrowDirection::Right);
        assert_eq!(ArrowDirection::Down, ArrowDirection::Down);
    }

    #[test]
    fn test_arrowdirection_diagonal() {
        assert_eq!(ArrowDirection::Northwest, ArrowDirection::Northwest);
        assert_eq!(ArrowDirection::Northeast, ArrowDirection::Northeast);
        assert_eq!(ArrowDirection::Southeast, ArrowDirection::Southeast);
        assert_eq!(ArrowDirection::Southwest, ArrowDirection::Southwest);
    }

    #[test]
    fn test_arrowdirection_bidirectional() {
        assert_eq!(ArrowDirection::LeftRight, ArrowDirection::LeftRight);
        assert_eq!(ArrowDirection::UpDown, ArrowDirection::UpDown);
        assert_eq!(
            ArrowDirection::NorthwestSoutheast,
            ArrowDirection::NorthwestSoutheast
        );
        assert_eq!(
            ArrowDirection::NortheastSouthwest,
            ArrowDirection::NortheastSouthwest
        );
    }

    // === ArrowStyle Tests ===

    #[test]
    fn test_arrowstyle_all_variants() {
        assert_eq!(ArrowStyle::Single, ArrowStyle::Single);
        assert_eq!(ArrowStyle::Double, ArrowStyle::Double);
        assert_eq!(ArrowStyle::Filled, ArrowStyle::Filled);
        assert_eq!(ArrowStyle::Hollow, ArrowStyle::Hollow);
        assert_eq!(ArrowStyle::Paired, ArrowStyle::Paired);
        assert_eq!(ArrowStyle::Combined, ArrowStyle::Combined);
        assert_eq!(ArrowStyle::Other, ArrowStyle::Other);
    }

    // === HandbellValue Tests ===

    #[test]
    fn test_handbellvalue_all_variants() {
        assert_eq!(HandbellValue::Belltree, HandbellValue::Belltree);
        assert_eq!(HandbellValue::Damp, HandbellValue::Damp);
        assert_eq!(HandbellValue::Echo, HandbellValue::Echo);
        assert_eq!(HandbellValue::Gyro, HandbellValue::Gyro);
        assert_eq!(HandbellValue::HandMartellato, HandbellValue::HandMartellato);
        assert_eq!(HandbellValue::MalletLift, HandbellValue::MalletLift);
        assert_eq!(HandbellValue::MalletTable, HandbellValue::MalletTable);
        assert_eq!(HandbellValue::Martellato, HandbellValue::Martellato);
        assert_eq!(HandbellValue::MartellatoLift, HandbellValue::MartellatoLift);
        assert_eq!(
            HandbellValue::MutedMartellato,
            HandbellValue::MutedMartellato
        );
        assert_eq!(HandbellValue::PluckLift, HandbellValue::PluckLift);
        assert_eq!(HandbellValue::Swing, HandbellValue::Swing);
    }

    // === HarmonMute Tests ===

    #[test]
    fn test_harmonmute_default() {
        let hm = HarmonMute::default();
        assert!(!hm.open);
        assert!(!hm.half);
    }

    #[test]
    fn test_harmonmute_open() {
        let hm = HarmonMute {
            open: true,
            half: false,
        };
        assert!(hm.open);
    }

    // === TupletPortion Tests ===

    #[test]
    fn test_tupletportion_with_type() {
        let portion = TupletPortion {
            tuplet_number: Some(TupletNumber {
                value: 3,
                font: Font::default(),
                color: None,
            }),
            tuplet_type: Some(TupletType {
                value: NoteTypeValue::Eighth,
                font: Font::default(),
                color: None,
            }),
            tuplet_dots: vec![TupletDot::default()],
        };
        assert!(portion.tuplet_type.is_some());
        assert_eq!(portion.tuplet_dots.len(), 1);
    }

    // === LinLength Tests ===

    #[test]
    fn test_linelength_all_variants() {
        assert_eq!(LineLength::Short, LineLength::Short);
        assert_eq!(LineLength::Medium, LineLength::Medium);
        assert_eq!(LineLength::Long, LineLength::Long);
    }

    // === EmptyTrillSound Tests ===

    #[test]
    fn test_emptytrillsound_default() {
        let ets = EmptyTrillSound::default();
        assert!(ets.placement.is_none());
        assert!(ets.start_note.is_none());
        assert!(ets.beats.is_none());
    }

    #[test]
    fn test_emptytrillsound_with_values() {
        let ets = EmptyTrillSound {
            placement: Some(AboveBelow::Above),
            position: Position::default(),
            start_note: Some(StartNote::Upper),
            trill_step: Some(TrillStep::Half),
            two_note_turn: Some(TwoNoteTurn::Whole),
            accelerate: Some(YesNo::Yes),
            beats: Some(4.0),
            second_beat: Some(25.0),
            last_beat: Some(75.0),
        };
        assert_eq!(ets.beats, Some(4.0));
    }
}
