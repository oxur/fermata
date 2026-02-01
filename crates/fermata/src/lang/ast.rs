//! Fermata AST -- typed representation of user-facing syntax.
//!
//! This AST captures the ergonomic forms before compilation to IR.

use crate::ir::common::StartStop;

/// A complete Fermata score
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FermataScore {
    /// Optional title of the score
    pub title: Option<String>,
    /// Optional composer name
    pub composer: Option<String>,
    /// Parts in the score
    pub parts: Vec<FermataPart>,
}

/// A part in the score
#[derive(Debug, Clone, PartialEq)]
pub struct FermataPart {
    /// Part name (e.g., "Piano", "Violin I")
    pub name: String,
    /// Optional part ID (auto-generated if not provided)
    pub id: Option<String>,
    /// Optional part abbreviation (e.g., "Pno.", "Vln. I")
    pub abbreviation: Option<String>,
    /// Measures in this part
    pub measures: Vec<FermataMeasure>,
}

/// A measure containing music elements
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FermataMeasure {
    /// Optional measure number
    pub number: Option<u32>,
    /// Content elements in this measure
    pub content: Vec<MeasureElement>,
}

/// Elements that can appear in a measure
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureElement {
    /// A single note
    Note(FermataNote),
    /// A rest
    Rest(FermataRest),
    /// A chord (multiple simultaneous pitches)
    Chord(FermataChord),
    /// A tuplet wrapper
    Tuplet(FermataTuplet),
    /// A grace note
    GraceNote(FermataGraceNote),
    /// A dynamic marking
    Dynamic(DynamicMark),
    /// A tempo marking
    Tempo(TempoMark),
    /// A direction (words, rehearsal mark, etc.)
    Direction(FermataDirection),
    /// A key signature
    Key(KeySpec),
    /// A time signature
    Time(TimeSpec),
    /// A clef
    Clef(ClefSpec),
    /// A barline
    Barline(BarlineSpec),
    /// A slur mark
    Slur(SlurMark),
    /// A tie mark
    Tie(TieMark),
    /// A fermata mark
    Fermata(FermataMark),
    /// Move backward in time (for multiple voices)
    Backup(u32),
    /// Move forward in time
    Forward(u32),
}

/// A single note
#[derive(Debug, Clone, PartialEq)]
pub struct FermataNote {
    /// The pitch of the note
    pub pitch: FermataPitch,
    /// The duration of the note
    pub duration: FermataDuration,
    /// Optional voice number
    pub voice: Option<u32>,
    /// Optional staff number
    pub staff: Option<u32>,
    /// Optional stem direction
    pub stem: Option<StemDirection>,
    /// Articulations on this note
    pub articulations: Vec<Articulation>,
    /// Ornaments on this note
    pub ornaments: Vec<Ornament>,
    /// Tie start/stop
    pub tie: Option<StartStop>,
    /// Slur start/stop
    pub slur: Option<StartStop>,
    /// Optional lyric
    pub lyric: Option<LyricSpec>,
}

/// A rest
#[derive(Debug, Clone, PartialEq)]
pub struct FermataRest {
    /// The duration of the rest
    pub duration: FermataDuration,
    /// Optional voice number
    pub voice: Option<u32>,
    /// Optional staff number
    pub staff: Option<u32>,
    /// Whether this is a whole-measure rest
    pub measure_rest: bool,
}

/// A chord (multiple simultaneous pitches)
#[derive(Debug, Clone, PartialEq)]
pub struct FermataChord {
    /// The pitches in the chord
    pub pitches: Vec<FermataPitch>,
    /// The duration of the chord
    pub duration: FermataDuration,
    /// Optional voice number
    pub voice: Option<u32>,
    /// Optional staff number
    pub staff: Option<u32>,
    /// Optional stem direction
    pub stem: Option<StemDirection>,
    /// Articulations on this chord
    pub articulations: Vec<Articulation>,
    /// Ornaments on this chord
    pub ornaments: Vec<Ornament>,
    /// Optional arpeggiate direction
    pub arpeggiate: Option<ArpeggiateDirection>,
}

/// A grace note
#[derive(Debug, Clone, PartialEq)]
pub struct FermataGraceNote {
    /// The pitch of the grace note
    pub pitch: FermataPitch,
    /// Whether to show a slash through the stem
    pub slash: bool,
    /// Optional duration (for display purposes)
    pub duration: Option<FermataDuration>,
}

/// A tuplet wrapper
#[derive(Debug, Clone, PartialEq)]
pub struct FermataTuplet {
    /// Actual number of notes
    pub actual: u32,
    /// Normal number of notes (in the time of)
    pub normal: u32,
    /// Notes inside the tuplet
    pub notes: Vec<MeasureElement>,
}

/// A pitch (parsed from "c4", "f#5", etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct FermataPitch {
    /// The pitch step (letter name)
    pub step: PitchStep,
    /// Optional alteration
    pub alter: Option<PitchAlter>,
    /// Octave number (4 = middle C octave)
    pub octave: u8,
}

/// Pitch letter name
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchStep {
    /// C
    C,
    /// D
    D,
    /// E
    E,
    /// F
    F,
    /// G
    G,
    /// A
    A,
    /// B
    B,
}

/// Pitch alteration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitchAlter {
    /// Sharp (#), +1 semitone
    Sharp,
    /// Flat (b), -1 semitone
    Flat,
    /// Double sharp (x or ##), +2 semitones
    DoubleSharp,
    /// Double flat (bb), -2 semitones
    DoubleFlat,
    /// Natural (n), explicit natural sign
    Natural,
    /// Quarter sharp (+), +0.5 semitones (microtone)
    QuarterSharp,
    /// Quarter flat (d), -0.5 semitones (microtone)
    QuarterFlat,
    /// Three-quarter sharp, +1.5 semitones
    ThreeQuarterSharp,
    /// Three-quarter flat, -1.5 semitones
    ThreeQuarterFlat,
}

impl PitchAlter {
    /// Convert the alteration to semitones
    pub fn to_semitones(&self) -> f64 {
        match self {
            PitchAlter::DoubleFlat => -2.0,
            PitchAlter::ThreeQuarterFlat => -1.5,
            PitchAlter::Flat => -1.0,
            PitchAlter::QuarterFlat => -0.5,
            PitchAlter::Natural => 0.0,
            PitchAlter::QuarterSharp => 0.5,
            PitchAlter::Sharp => 1.0,
            PitchAlter::ThreeQuarterSharp => 1.5,
            PitchAlter::DoubleSharp => 2.0,
        }
    }
}

/// Duration specification
#[derive(Debug, Clone, PartialEq)]
pub struct FermataDuration {
    /// The base duration value
    pub base: DurationBase,
    /// Number of augmentation dots
    pub dots: u8,
}

impl Default for FermataDuration {
    fn default() -> Self {
        Self {
            base: DurationBase::Quarter,
            dots: 0,
        }
    }
}

/// Base duration value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DurationBase {
    /// Maxima (8 whole notes)
    Maxima,
    /// Long (4 whole notes)
    Long,
    /// Breve (double whole note)
    Breve,
    /// Whole note (semibreve)
    Whole,
    /// Half note (minim)
    Half,
    /// Quarter note (crotchet)
    #[default]
    Quarter,
    /// Eighth note (quaver)
    Eighth,
    /// Sixteenth note (semiquaver)
    Sixteenth,
    /// 32nd note (demisemiquaver)
    ThirtySecond,
    /// 64th note (hemidemisemiquaver)
    SixtyFourth,
    /// 128th note
    OneTwentyEighth,
    /// 256th note
    TwoFiftySixth,
    /// 512th note
    FiveTwelfth,
    /// 1024th note
    OneThousandTwentyFourth,
}

impl DurationBase {
    /// Duration relative to whole note (1.0 = whole)
    pub fn to_fraction(&self) -> f64 {
        match self {
            DurationBase::Maxima => 8.0,
            DurationBase::Long => 4.0,
            DurationBase::Breve => 2.0,
            DurationBase::Whole => 1.0,
            DurationBase::Half => 0.5,
            DurationBase::Quarter => 0.25,
            DurationBase::Eighth => 0.125,
            DurationBase::Sixteenth => 0.0625,
            DurationBase::ThirtySecond => 0.03125,
            DurationBase::SixtyFourth => 0.015625,
            DurationBase::OneTwentyEighth => 0.0078125,
            DurationBase::TwoFiftySixth => 0.00390625,
            DurationBase::FiveTwelfth => 0.001953125,
            DurationBase::OneThousandTwentyFourth => 0.0009765625,
        }
    }
}

/// Stem direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StemDirection {
    /// Stem points up
    #[default]
    Up,
    /// Stem points down
    Down,
    /// No stem (for whole notes, etc.)
    None,
    /// Double stem (rare, for special notation)
    Double,
}

/// Articulation marks (excluding fermata, which is a notation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Articulation {
    /// Staccato (short)
    Staccato,
    /// Staccatissimo (very short)
    Staccatissimo,
    /// Accent
    Accent,
    /// Strong accent (marcato)
    StrongAccent,
    /// Tenuto (held)
    Tenuto,
    /// Detached legato
    DetachedLegato,
    /// Breath mark
    BreathMark,
    /// Caesura (pause)
    Caesura,
}

/// Fermata mark (separate from articulations per MusicXML/IR structure)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FermataMark {
    /// Shape of the fermata
    pub shape: FermataShape,
    /// Whether the fermata is inverted (below the note)
    pub inverted: bool,
}

/// Fermata shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FermataShape {
    /// Normal curved fermata
    #[default]
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

/// Ornament marks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ornament {
    /// Trill
    Trill,
    /// Mordent
    Mordent,
    /// Inverted mordent
    InvertedMordent,
    /// Turn
    Turn,
    /// Inverted turn
    InvertedTurn,
    /// Delayed turn
    DelayedTurn,
    /// Shake
    Shake,
    /// Tremolo with given number of beams
    Tremolo(u8),
}

/// Arpeggio direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArpeggiateDirection {
    /// Arpeggiate upward
    Up,
    /// Arpeggiate downward
    Down,
    /// No specific direction
    #[default]
    None,
}

/// Dynamic marking
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicMark {
    // Standard dynamics (softest to loudest)
    /// Pianississississimo (pppppp)
    PPPPPP,
    /// Pianissississimo (ppppp)
    PPPPP,
    /// Pianississimo (pppp)
    PPPP,
    /// Pianissimo (ppp)
    PPP,
    /// Very soft (pp)
    PP,
    /// Soft (p)
    P,
    /// Moderately soft (mp)
    MP,
    /// Moderately loud (mf)
    MF,
    /// Loud (f)
    F,
    /// Very loud (ff)
    FF,
    /// Fortissimo (fff)
    FFF,
    /// Fortississimo (ffff)
    FFFF,
    /// Fortissississimo (fffff)
    FFFFF,
    /// Fortississississimo (ffffff)
    FFFFFF,
    // Combined dynamics
    /// Forte-piano
    FP,
    /// Sforzando
    SF,
    /// Sforzando-piano
    SFP,
    /// Sforzando-pianissimo
    SFPP,
    /// Sforzato
    SFZ,
    /// Sforzatissimo
    SFFZ,
    /// Sforzato-piano
    SFZP,
    /// Forzando
    FZ,
    /// Piano-forte
    PF,
    /// Rinforzando
    RF,
    /// Rinforzando with z
    RFZ,
    /// Niente (nothing, silence)
    N,
    /// Crescendo (start/stop)
    Crescendo(StartStop),
    /// Diminuendo (start/stop)
    Diminuendo(StartStop),
}

/// Tempo marking
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TempoMark {
    /// Optional text (e.g., "Allegro", "Andante")
    pub text: Option<String>,
    /// Optional beat unit for metronome marking
    pub beat_unit: Option<DurationBase>,
    /// Number of dots on the beat unit
    pub beat_unit_dots: u8,
    /// Optional beats per minute
    pub per_minute: Option<u32>,
}

/// General direction
#[derive(Debug, Clone, PartialEq)]
pub enum FermataDirection {
    /// Text direction (performance instructions)
    Words(String),
    /// Rehearsal mark
    Rehearsal(String),
    /// Segno sign
    Segno,
    /// Coda sign
    Coda,
    /// Pedal start
    PedalStart,
    /// Pedal stop
    PedalStop,
}

/// Key signature specification
#[derive(Debug, Clone, PartialEq)]
pub struct KeySpec {
    /// Root pitch step
    pub root: PitchStep,
    /// Optional alteration on the root (for F# major, Bb minor, etc.)
    pub root_alter: Option<PitchAlter>,
    /// Mode (major, minor, etc.)
    pub mode: Mode,
}

/// Mode for key signature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Major mode
    #[default]
    Major,
    /// Minor mode
    Minor,
    /// Dorian mode
    Dorian,
    /// Phrygian mode
    Phrygian,
    /// Lydian mode
    Lydian,
    /// Mixolydian mode
    Mixolydian,
    /// Aeolian mode (natural minor)
    Aeolian,
    /// Ionian mode (same as major)
    Ionian,
    /// Locrian mode
    Locrian,
}

/// Time signature specification
#[derive(Debug, Clone, PartialEq)]
pub enum TimeSpec {
    /// Simple time signature (e.g., 4/4, 3/4, 6/8)
    Simple {
        /// Number of beats
        beats: u8,
        /// Beat type (4 = quarter, 8 = eighth, etc.)
        beat_type: u8,
    },
    /// Compound time signature (e.g., 2/4 + 3/8)
    Compound {
        /// Multiple time signatures
        signatures: Vec<(u8, u8)>,
    },
    /// Common time (4/4 with C symbol)
    Common,
    /// Cut time (2/2 with cut C symbol)
    Cut,
    /// Senza misura (no time signature)
    SenzaMisura,
}

impl Default for TimeSpec {
    fn default() -> Self {
        TimeSpec::Simple {
            beats: 4,
            beat_type: 4,
        }
    }
}

/// Clef specification
#[derive(Debug, Clone, PartialEq)]
pub enum ClefSpec {
    /// Treble clef (G clef on line 2)
    Treble,
    /// Bass clef (F clef on line 4)
    Bass,
    /// Alto clef (C clef on line 3)
    Alto,
    /// Tenor clef (C clef on line 4)
    Tenor,
    /// Treble clef, 8va bassa
    Treble8vb,
    /// Treble clef, 8va alta
    Treble8va,
    /// Bass clef, 8va bassa
    Bass8vb,
    /// Bass clef, 8va alta
    Bass8va,
    /// Percussion clef
    Percussion,
    /// Tab clef
    Tab,
    /// Custom clef with explicit sign, line, and octave change
    Custom {
        /// Clef sign (G, F, C, etc.)
        sign: char,
        /// Staff line (1-5)
        line: u8,
        /// Optional octave change (-1 = 8vb, +1 = 8va)
        octave_change: Option<i8>,
    },
}

impl Default for ClefSpec {
    fn default() -> Self {
        ClefSpec::Treble
    }
}

/// Action for endings (start, stop, or discontinue for jumps)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndingAction {
    /// Start an ending bracket
    Start,
    /// Stop an ending bracket
    Stop,
    /// Discontinue (when jumping back, e.g., D.S. skips ending 1)
    Discontinue,
}

/// Barline specification
#[derive(Debug, Clone, PartialEq)]
pub enum BarlineSpec {
    /// Regular single barline
    Regular,
    /// Double barline
    Double,
    /// Final barline (thick-thin)
    Final,
    /// Repeat forward (start repeat)
    RepeatForward,
    /// Repeat backward (end repeat)
    RepeatBackward,
    /// Repeat both directions
    RepeatBoth,
    /// Ending bracket
    Ending {
        /// Ending number (1, 2, etc.)
        number: u8,
        /// Ending action
        action: EndingAction,
    },
}

impl Default for BarlineSpec {
    fn default() -> Self {
        BarlineSpec::Regular
    }
}

/// Slur mark
#[derive(Debug, Clone, PartialEq)]
pub struct SlurMark {
    /// Start or stop action
    pub action: StartStop,
    /// Slur number (defaults to 1)
    pub number: u8,
}

impl Default for SlurMark {
    fn default() -> Self {
        Self {
            action: StartStop::Start,
            number: 1,
        }
    }
}

/// Tie mark
#[derive(Debug, Clone, PartialEq)]
pub struct TieMark {
    /// Start or stop action
    pub action: StartStop,
}

/// Lyric specification
#[derive(Debug, Clone, PartialEq)]
pub struct LyricSpec {
    /// The lyric text
    pub text: String,
    /// Syllabic type
    pub syllabic: Syllabic,
    /// Optional verse number
    pub verse: Option<u8>,
}

/// Syllabic type for lyrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Syllabic {
    /// Single syllable word
    #[default]
    Single,
    /// Beginning of a multi-syllable word
    Begin,
    /// Middle of a multi-syllable word
    Middle,
    /// End of a multi-syllable word
    End,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_base_quarter_fraction() {
        assert_eq!(DurationBase::Quarter.to_fraction(), 0.25);
    }

    #[test]
    fn test_duration_base_whole_fraction() {
        assert_eq!(DurationBase::Whole.to_fraction(), 1.0);
    }

    #[test]
    fn test_duration_base_half_fraction() {
        assert_eq!(DurationBase::Half.to_fraction(), 0.5);
    }

    #[test]
    fn test_duration_base_eighth_fraction() {
        assert_eq!(DurationBase::Eighth.to_fraction(), 0.125);
    }

    #[test]
    fn test_duration_base_sixteenth_fraction() {
        assert_eq!(DurationBase::Sixteenth.to_fraction(), 0.0625);
    }

    #[test]
    fn test_duration_base_maxima_fraction() {
        assert_eq!(DurationBase::Maxima.to_fraction(), 8.0);
    }

    #[test]
    fn test_duration_base_long_fraction() {
        assert_eq!(DurationBase::Long.to_fraction(), 4.0);
    }

    #[test]
    fn test_duration_base_breve_fraction() {
        assert_eq!(DurationBase::Breve.to_fraction(), 2.0);
    }

    #[test]
    fn test_duration_base_one_thousand_twenty_fourth_fraction() {
        assert_eq!(
            DurationBase::OneThousandTwentyFourth.to_fraction(),
            0.0009765625
        );
    }

    #[test]
    fn test_pitch_alter_sharp_semitones() {
        assert_eq!(PitchAlter::Sharp.to_semitones(), 1.0);
    }

    #[test]
    fn test_pitch_alter_flat_semitones() {
        assert_eq!(PitchAlter::Flat.to_semitones(), -1.0);
    }

    #[test]
    fn test_pitch_alter_double_sharp_semitones() {
        assert_eq!(PitchAlter::DoubleSharp.to_semitones(), 2.0);
    }

    #[test]
    fn test_pitch_alter_double_flat_semitones() {
        assert_eq!(PitchAlter::DoubleFlat.to_semitones(), -2.0);
    }

    #[test]
    fn test_pitch_alter_natural_semitones() {
        assert_eq!(PitchAlter::Natural.to_semitones(), 0.0);
    }

    #[test]
    fn test_pitch_alter_quarter_sharp_semitones() {
        assert_eq!(PitchAlter::QuarterSharp.to_semitones(), 0.5);
    }

    #[test]
    fn test_pitch_alter_quarter_flat_semitones() {
        assert_eq!(PitchAlter::QuarterFlat.to_semitones(), -0.5);
    }

    #[test]
    fn test_pitch_alter_three_quarter_sharp_semitones() {
        assert_eq!(PitchAlter::ThreeQuarterSharp.to_semitones(), 1.5);
    }

    #[test]
    fn test_pitch_alter_three_quarter_flat_semitones() {
        assert_eq!(PitchAlter::ThreeQuarterFlat.to_semitones(), -1.5);
    }

    #[test]
    fn test_duration_base_default() {
        assert_eq!(DurationBase::default(), DurationBase::Quarter);
    }

    #[test]
    fn test_stem_direction_default() {
        assert_eq!(StemDirection::default(), StemDirection::Up);
    }

    #[test]
    fn test_mode_default() {
        assert_eq!(Mode::default(), Mode::Major);
    }

    #[test]
    fn test_clef_spec_default() {
        assert_eq!(ClefSpec::default(), ClefSpec::Treble);
    }

    #[test]
    fn test_time_spec_default() {
        assert_eq!(
            TimeSpec::default(),
            TimeSpec::Simple {
                beats: 4,
                beat_type: 4
            }
        );
    }

    #[test]
    fn test_barline_spec_default() {
        assert_eq!(BarlineSpec::default(), BarlineSpec::Regular);
    }

    #[test]
    fn test_slur_mark_default() {
        let slur = SlurMark::default();
        assert_eq!(slur.number, 1);
        assert_eq!(slur.action, StartStop::Start);
    }

    #[test]
    fn test_fermata_mark_default() {
        let fermata = FermataMark::default();
        assert_eq!(fermata.shape, FermataShape::Normal);
        assert!(!fermata.inverted);
    }

    #[test]
    fn test_fermata_shape_default() {
        assert_eq!(FermataShape::default(), FermataShape::Normal);
    }

    #[test]
    fn test_syllabic_default() {
        assert_eq!(Syllabic::default(), Syllabic::Single);
    }

    #[test]
    fn test_arpeggiate_direction_default() {
        assert_eq!(ArpeggiateDirection::default(), ArpeggiateDirection::None);
    }

    #[test]
    fn test_fermata_duration_default() {
        let dur = FermataDuration::default();
        assert_eq!(dur.base, DurationBase::Quarter);
        assert_eq!(dur.dots, 0);
    }

    #[test]
    fn test_fermata_score_default() {
        let score = FermataScore::default();
        assert!(score.title.is_none());
        assert!(score.composer.is_none());
        assert!(score.parts.is_empty());
    }

    #[test]
    fn test_fermata_measure_default() {
        let measure = FermataMeasure::default();
        assert!(measure.number.is_none());
        assert!(measure.content.is_empty());
    }

    #[test]
    fn test_tempo_mark_default() {
        let tempo = TempoMark::default();
        assert!(tempo.text.is_none());
        assert!(tempo.beat_unit.is_none());
        assert_eq!(tempo.beat_unit_dots, 0);
        assert!(tempo.per_minute.is_none());
    }

    #[test]
    fn test_pitch_step_variants() {
        assert_ne!(PitchStep::C, PitchStep::D);
        assert_ne!(PitchStep::E, PitchStep::F);
        assert_ne!(PitchStep::G, PitchStep::A);
        assert_ne!(PitchStep::A, PitchStep::B);
    }

    #[test]
    fn test_fermata_pitch_clone() {
        let pitch = FermataPitch {
            step: PitchStep::C,
            alter: Some(PitchAlter::Sharp),
            octave: 4,
        };
        let cloned = pitch.clone();
        assert_eq!(pitch, cloned);
    }

    #[test]
    fn test_fermata_part_clone() {
        let part = FermataPart {
            name: "Piano".to_string(),
            id: Some("P1".to_string()),
            abbreviation: Some("Pno.".to_string()),
            measures: vec![],
        };
        let cloned = part.clone();
        assert_eq!(part, cloned);
    }

    #[test]
    fn test_key_spec_clone() {
        let key = KeySpec {
            root: PitchStep::C,
            root_alter: None,
            mode: Mode::Major,
        };
        let cloned = key.clone();
        assert_eq!(key, cloned);
    }

    #[test]
    fn test_dynamic_mark_variants() {
        let p = DynamicMark::P;
        let f = DynamicMark::F;
        assert_ne!(p, f);
    }

    #[test]
    fn test_dynamic_mark_crescendo() {
        let cresc = DynamicMark::Crescendo(StartStop::Start);
        if let DynamicMark::Crescendo(action) = cresc {
            assert_eq!(action, StartStop::Start);
        } else {
            panic!("Expected Crescendo variant");
        }
    }

    #[test]
    fn test_measure_element_note_variant() {
        let note = FermataNote {
            pitch: FermataPitch {
                step: PitchStep::C,
                alter: None,
                octave: 4,
            },
            duration: FermataDuration::default(),
            voice: None,
            staff: None,
            stem: None,
            articulations: vec![],
            ornaments: vec![],
            tie: None,
            slur: None,
            lyric: None,
        };
        let elem = MeasureElement::Note(note);
        if let MeasureElement::Note(n) = elem {
            assert_eq!(n.pitch.step, PitchStep::C);
        } else {
            panic!("Expected Note variant");
        }
    }

    #[test]
    fn test_clef_spec_custom() {
        let clef = ClefSpec::Custom {
            sign: 'G',
            line: 2,
            octave_change: Some(-1),
        };
        if let ClefSpec::Custom {
            sign,
            line,
            octave_change,
        } = clef
        {
            assert_eq!(sign, 'G');
            assert_eq!(line, 2);
            assert_eq!(octave_change, Some(-1));
        } else {
            panic!("Expected Custom variant");
        }
    }

    #[test]
    fn test_time_spec_compound() {
        let time = TimeSpec::Compound {
            signatures: vec![(2, 4), (3, 8)],
        };
        if let TimeSpec::Compound { signatures } = time {
            assert_eq!(signatures.len(), 2);
        } else {
            panic!("Expected Compound variant");
        }
    }

    #[test]
    fn test_barline_spec_ending() {
        let barline = BarlineSpec::Ending {
            number: 1,
            action: EndingAction::Start,
        };
        if let BarlineSpec::Ending { number, action } = barline {
            assert_eq!(number, 1);
            assert_eq!(action, EndingAction::Start);
        } else {
            panic!("Expected Ending variant");
        }
    }
}
