//! # 𝄐 Fermata
//!
//! **An S-expression DSL for working with MusicXML**
//!
//! Fermata is a Lisp-like domain-specific language for describing musical notation.
//! It compiles to MusicXML, LilyPond, and Rust code.
//!
//! ## Quick Example
//!
//! ```ignore
//! use fermata::{parse, compile_to_musicxml};
//!
//! let source = r#"
//! (score
//! (part :piano
//! (measure
//! (note c4 :q)
//! (note d4 :q)
//! (note e4 :q)
//! (note f4 :q))))
//! "#;
//!
//! let ast = parse(source)?;
//! let musicxml = compile_to_musicxml(&ast)?;
//! ```
//!
//! ## Status
//!
//! This crate is under active development. The API will change.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod ast {
    //! Abstract Syntax Tree for Fermata expressions.
    //!
    //! The AST represents parsed Fermata code as a tree structure that can be:
    //! - Evaluated (interpreted)
    //! - Compiled to MusicXML
    //! - Compiled to LilyPond
    //! - Compiled to Rust code

    /// A complete Fermata expression
    #[derive(Debug, Clone, PartialEq)]
    pub enum Expr {
        /// A symbol like `c4`, `:quarter`, `piano`
        Symbol(String),
        /// A keyword like `:q`, `:major`, `:treble`
        Keyword(String),
        /// An integer literal
        Integer(i64),
        /// A floating-point literal
        Float(f64),
        /// A string literal
        String(String),
        /// A list of expressions: `(note c4 :q)`
        List(Vec<Expr>),
        /// A quoted expression: `'(c4 e4 g4)`
        Quote(Box<Expr>),
    }

    /// A parsed Fermata score
    #[derive(Debug, Clone, PartialEq)]
    pub struct Score {
        /// Score metadata (title, composer, etc.)
        pub metadata: Metadata,
        /// Parts in the score
        pub parts: Vec<Part>,
    }

    /// Score metadata
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Metadata {
        /// Title of the work
        pub title: Option<String>,
        /// Composer name
        pub composer: Option<String>,
        /// Tempo marking
        pub tempo: Option<Tempo>,
    }

    /// Tempo specification
    #[derive(Debug, Clone, PartialEq)]
    pub struct Tempo {
        /// Beats per minute
        pub bpm: u32,
        /// Beat unit (quarter, half, etc.)
        pub beat_unit: Duration,
    }

    /// A musical part (instrument)
    #[derive(Debug, Clone, PartialEq)]
    pub struct Part {
        /// Instrument identifier
        pub instrument: String,
        /// Staves in this part
        pub staves: Vec<Staff>,
    }

    /// A staff within a part
    #[derive(Debug, Clone, PartialEq)]
    pub struct Staff {
        /// Clef for this staff
        pub clef: Clef,
        /// Measures in this staff
        pub measures: Vec<Measure>,
    }

    /// Clef types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Clef {
        /// Treble clef (G clef)
        Treble,
        /// Bass clef (F clef)
        Bass,
        /// Alto clef (C clef, middle line)
        Alto,
        /// Tenor clef (C clef)
        Tenor,
        /// Percussion clef
        Percussion,
    }

    /// A measure (bar)
    #[derive(Debug, Clone, PartialEq)]
    pub struct Measure {
        /// Optional measure number
        pub number: Option<u32>,
        /// Voices in this measure
        pub voices: Vec<Voice>,
        /// Key signature change (if any)
        pub key: Option<Key>,
        /// Time signature change (if any)
        pub time: Option<TimeSignature>,
    }

    /// A voice within a measure
    #[derive(Debug, Clone, PartialEq)]
    pub struct Voice {
        /// Elements in this voice
        pub elements: Vec<Element>,
    }

    /// A musical element (note, rest, chord, etc.)
    #[derive(Debug, Clone, PartialEq)]
    pub enum Element {
        /// A single note
        Note(Note),
        /// A rest
        Rest(Rest),
        /// A chord (multiple simultaneous notes)
        Chord(Chord),
        /// A tuplet group
        Tuplet(Tuplet),
    }

    /// A musical note
    #[derive(Debug, Clone, PartialEq)]
    pub struct Note {
        /// Pitch of the note
        pub pitch: Pitch,
        /// Duration of the note
        pub duration: Duration,
        /// Articulations and ornaments
        pub articulations: Vec<Articulation>,
        /// Dynamics
        pub dynamics: Option<Dynamic>,
        /// Is this note tied to the next?
        pub tie: Option<TieType>,
    }

    /// A rest
    #[derive(Debug, Clone, PartialEq)]
    pub struct Rest {
        /// Duration of the rest
        pub duration: Duration,
    }

    /// A chord (multiple simultaneous pitches)
    #[derive(Debug, Clone, PartialEq)]
    pub struct Chord {
        /// Pitches in the chord
        pub pitches: Vec<Pitch>,
        /// Duration of the chord
        pub duration: Duration,
        /// Articulations
        pub articulations: Vec<Articulation>,
    }

    /// A tuplet (triplet, etc.)
    #[derive(Debug, Clone, PartialEq)]
    pub struct Tuplet {
        /// Actual notes in the time of...
        pub actual: u32,
        /// ...normal notes
        pub normal: u32,
        /// Elements in the tuplet
        pub elements: Vec<Element>,
    }

    /// A musical pitch
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Pitch {
        /// Note name (C, D, E, F, G, A, B)
        pub step: Step,
        /// Alteration (sharp, flat, natural)
        pub alter: Alter,
        /// Octave (4 = middle C octave)
        pub octave: i8,
    }

    /// Note step (letter name)
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Step {
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Alter {
        /// Double flat
        DoubleFlat,
        /// Flat
        Flat,
        /// Natural (no alteration)
        Natural,
        /// Sharp
        Sharp,
        /// Double sharp
        DoubleSharp,
    }

    /// Note duration
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Duration {
        /// Whole note (semibreve)
        Whole,
        /// Half note (minim)
        Half,
        /// Quarter note (crotchet)
        Quarter,
        /// Eighth note (quaver)
        Eighth,
        /// Sixteenth note (semiquaver)
        Sixteenth,
        /// Thirty-second note (demisemiquaver)
        ThirtySecond,
        /// Sixty-fourth note
        SixtyFourth,
    }

    /// Articulation and ornament types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Articulation {
        /// Staccato (short)
        Staccato,
        /// Accent
        Accent,
        /// Tenuto (held)
        Tenuto,
        /// Fermata (pause)
        Fermata,
        /// Trill
        Trill,
        /// Mordent
        Mordent,
        /// Turn
        Turn,
        /// Dotted (extends duration by 50%)
        Dot,
        /// Double-dotted
        DoubleDot,
    }

    /// Dynamic markings
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Dynamic {
        /// Pianississimo
        PPP,
        /// Pianissimo
        PP,
        /// Piano
        P,
        /// Mezzo-piano
        MP,
        /// Mezzo-forte
        MF,
        /// Forte
        F,
        /// Fortissimo
        FF,
        /// Fortississimo
        FFF,
    }

    /// Tie types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TieType {
        /// Start of a tie
        Start,
        /// End of a tie
        Stop,
        /// Continue (middle of a multi-note tie)
        Continue,
    }

    /// Key signature
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Key {
        /// Root note
        pub root: Step,
        /// Alteration of root
        pub alter: Alter,
        /// Mode
        pub mode: Mode,
    }

    /// Musical mode
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mode {
        /// Major mode
        Major,
        /// Minor mode (natural)
        Minor,
        /// Dorian mode
        Dorian,
        /// Phrygian mode
        Phrygian,
        /// Lydian mode
        Lydian,
        /// Mixolydian mode
        Mixolydian,
        /// Locrian mode
        Locrian,
    }

    /// Time signature
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TimeSignature {
        /// Beats per measure
        pub beats: u8,
        /// Beat unit (4 = quarter, 8 = eighth, etc.)
        pub beat_type: u8,
    }
}

pub mod error {
    //! Error handling for Fermata parsing and compilation.

    /// Errors that can occur during Fermata operations
    #[derive(Debug)]
    pub enum Error {
        /// Parse error with location information
        Parse {
            /// Error message
            message: String,
            /// Line number (1-indexed)
            line: usize,
            /// Column number (1-indexed)
            column: usize,
        },
        /// Semantic error (valid syntax but invalid meaning)
        Semantic(String),
        /// IO error
        Io(std::io::Error),
        /// MusicXML generation error
        MusicXml(String),
        /// LilyPond generation error
        LilyPond(String),
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Parse {
                    message,
                    line,
                    column,
                } => {
                    write!(f, "Parse error at {}:{}: {}", line, column, message)
                }
                Error::Semantic(msg) => write!(f, "Semantic error: {}", msg),
                Error::Io(e) => write!(f, "IO error: {}", e),
                Error::MusicXml(msg) => write!(f, "MusicXML error: {}", msg),
                Error::LilyPond(msg) => write!(f, "LilyPond error: {}", msg),
            }
        }
    }

    impl std::error::Error for Error {}

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Error::Io(e)
        }
    }
}

/// Result type for Fermata operations
pub type Result<T> = std::result::Result<T, error::Error>;

// Re-exports for convenience
pub use ast::*;
pub use error::Error;

/// Parse Fermata source code into an AST.
///
/// # Example
///
/// ```ignore
/// let ast = fermata::parse("(note c4 :q)")?;
/// ```
///
/// # Errors
///
/// Returns `Error::Parse` if the source contains syntax errors.
pub fn parse(_source: &str) -> Result<Expr> {
    // TODO: Implement parser
    todo!("Parser not yet implemented")
}

/// Compile a Fermata AST to MusicXML.
///
/// # Example
///
/// ```ignore
/// let ast = fermata::parse("(score (part :piano (measure (note c4 :q))))")?;
/// let xml = fermata::compile_to_musicxml(&ast)?;
/// ```
///
/// # Errors
///
/// Returns `Error::MusicXml` if the AST cannot be converted to valid MusicXML.
pub fn compile_to_musicxml(_ast: &Expr) -> Result<String> {
    // TODO: Implement MusicXML compiler
    todo!("MusicXML compiler not yet implemented")
}

/// Compile a Fermata AST to LilyPond format.
///
/// # Example
///
/// ```ignore
/// let ast = fermata::parse("(score (part :piano (measure (note c4 :q))))")?;
/// let ly = fermata::compile_to_lilypond(&ast)?;
/// ```
///
/// # Errors
///
/// Returns `Error::LilyPond` if the AST cannot be converted to valid LilyPond.
pub fn compile_to_lilypond(_ast: &Expr) -> Result<String> {
    // TODO: Implement LilyPond compiler
    todo!("LilyPond compiler not yet implemented")
}

/// Parse MusicXML into a Fermata AST.
///
/// This enables round-tripping: MusicXML → Fermata → MusicXML
///
/// # Example
///
/// ```ignore
/// let xml = std::fs::read_to_string("score.musicxml")?;
/// let ast = fermata::parse_musicxml(&xml)?;
/// println!("{}", fermata::format(&ast));
/// ```
pub fn parse_musicxml(_xml: &str) -> Result<Expr> {
    // TODO: Implement MusicXML parser
    todo!("MusicXML parser not yet implemented")
}

/// Format a Fermata AST as pretty-printed source code.
///
/// # Example
///
/// ```ignore
/// let ast = fermata::parse("(note c4 :q)")?;
/// let formatted = fermata::format(&ast);
/// assert_eq!(formatted, "(note c4 :q)");
/// ```
pub fn format(_ast: &Expr) -> String {
    // TODO: Implement formatter
    todo!("Formatter not yet implemented")
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
