//! Show command implementations for displaying reference information.

use std::process::ExitCode;

use owo_colors::OwoColorize;
use serde::Serialize;

use crate::{OutputFormat, ShowTopic};

/// Run a show command with the given topic and format.
pub fn run(topic: ShowTopic, format: OutputFormat, use_colors: bool) -> ExitCode {
    match topic {
        ShowTopic::Targets => show_targets(format, use_colors),
        ShowTopic::Syntax => show_syntax(format, use_colors),
        ShowTopic::Durations => show_durations(format, use_colors),
        ShowTopic::Pitches => show_pitches(format, use_colors),
        ShowTopic::Clefs => show_clefs(format, use_colors),
        ShowTopic::Keys => show_keys(format, use_colors),
        ShowTopic::Dynamics => show_dynamics(format, use_colors),
        ShowTopic::Articulations => show_articulations(format, use_colors),
        ShowTopic::Ornaments => show_ornaments(format, use_colors),
        ShowTopic::Instruments => show_instruments(format, use_colors),
        ShowTopic::Barlines => show_barlines(format, use_colors),
        ShowTopic::Accidentals => show_accidentals(format, use_colors),
        ShowTopic::Noteheads => show_noteheads(format, use_colors),
        ShowTopic::Fermatas => show_fermatas(format, use_colors),
    }
}

/// A reference item with keyword, description, and optional example.
#[derive(Debug, Serialize)]
struct RefItem {
    keyword: &'static str,
    description: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<&'static str>,
}

/// A category of reference items.
#[derive(Debug, Serialize)]
struct RefCategory {
    name: &'static str,
    items: Vec<RefItem>,
}

/// Print items in text format with optional colors.
fn print_text(title: &str, categories: &[RefCategory], use_colors: bool) {
    if use_colors {
        println!("{}", title.bold().underline());
    } else {
        println!("{}", title);
    }
    println!();

    for category in categories {
        if !category.name.is_empty() {
            if use_colors {
                println!("  {}", category.name.cyan().bold());
            } else {
                println!("  {}", category.name);
            }
        }
        for item in &category.items {
            if use_colors {
                print!("    {:16}", item.keyword.green());
            } else {
                print!("    {:16}", item.keyword);
            }
            print!("  {}", item.description);
            if let Some(example) = item.example {
                if use_colors {
                    print!("  {}", format!("({})", example).dimmed());
                } else {
                    print!("  ({})", example);
                }
            }
            println!();
        }
        println!();
    }
}

/// Print items in JSON format.
fn print_json(categories: &[RefCategory]) {
    let json = serde_json::to_string_pretty(categories).expect("JSON serialization failed");
    println!("{}", json);
}

/// Output helper that handles format selection.
fn output(
    title: &str,
    categories: Vec<RefCategory>,
    format: OutputFormat,
    use_colors: bool,
) -> ExitCode {
    match format {
        OutputFormat::Text => print_text(title, &categories, use_colors),
        OutputFormat::Json => print_json(&categories),
    }
    ExitCode::SUCCESS
}

// === Show command implementations ===

fn show_targets(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![RefCategory {
        name: "",
        items: vec![
            RefItem {
                keyword: "musicxml",
                description: "MusicXML format for notation software",
                example: Some("Finale, Sibelius, MuseScore, Dorico"),
            },
            RefItem {
                keyword: "xml",
                description: "Alias for musicxml",
                example: None,
            },
            RefItem {
                keyword: "lilypond",
                description: "LilyPond format (not yet implemented)",
                example: Some("Publication-quality PDF engraving"),
            },
            RefItem {
                keyword: "ly",
                description: "Alias for lilypond",
                example: None,
            },
        ],
    }];
    output("Output Targets", categories, format, use_colors)
}

fn show_syntax(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Structure",
            items: vec![
                RefItem {
                    keyword: "(score ...)",
                    description: "Top-level score container",
                    example: None,
                },
                RefItem {
                    keyword: "(part ...)",
                    description: "Musical part/instrument",
                    example: Some("(part :piano ...)"),
                },
                RefItem {
                    keyword: "(measure ...)",
                    description: "A measure of music",
                    example: None,
                },
                RefItem {
                    keyword: "(voice ...)",
                    description: "Voice within a measure",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Notes",
            items: vec![
                RefItem {
                    keyword: "(note ...)",
                    description: "A single note",
                    example: Some("(note c4 :q)"),
                },
                RefItem {
                    keyword: "(rest ...)",
                    description: "A rest",
                    example: Some("(rest :q)"),
                },
                RefItem {
                    keyword: "(chord ...)",
                    description: "Multiple notes together",
                    example: Some("(chord :q (c4 e4 g4))"),
                },
            ],
        },
        RefCategory {
            name: "Attributes",
            items: vec![
                RefItem {
                    keyword: "(key ...)",
                    description: "Key signature",
                    example: Some("(key c :major)"),
                },
                RefItem {
                    keyword: "(time ...)",
                    description: "Time signature",
                    example: Some("(time 4 4)"),
                },
                RefItem {
                    keyword: "(clef ...)",
                    description: "Clef",
                    example: Some("(clef :treble)"),
                },
                RefItem {
                    keyword: "(tempo ...)",
                    description: "Tempo marking",
                    example: Some("(tempo 120 :quarter)"),
                },
            ],
        },
    ];
    output("Syntax Quick Reference", categories, format, use_colors)
}

fn show_durations(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Standard Durations",
            items: vec![
                RefItem {
                    keyword: ":w",
                    description: "Whole note (semibreve)",
                    example: Some("4 beats in 4/4"),
                },
                RefItem {
                    keyword: ":h",
                    description: "Half note (minim)",
                    example: Some("2 beats in 4/4"),
                },
                RefItem {
                    keyword: ":q",
                    description: "Quarter note (crotchet)",
                    example: Some("1 beat in 4/4"),
                },
                RefItem {
                    keyword: ":8",
                    description: "Eighth note (quaver)",
                    example: Some("1/2 beat in 4/4"),
                },
                RefItem {
                    keyword: ":16",
                    description: "Sixteenth note (semiquaver)",
                    example: Some("1/4 beat in 4/4"),
                },
                RefItem {
                    keyword: ":32",
                    description: "Thirty-second note (demisemiquaver)",
                    example: None,
                },
                RefItem {
                    keyword: ":64",
                    description: "Sixty-fourth note (hemidemisemiquaver)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Extended Durations",
            items: vec![
                RefItem {
                    keyword: ":breve",
                    description: "Double whole note",
                    example: Some("8 beats in 4/4"),
                },
                RefItem {
                    keyword: ":long",
                    description: "Quadruple whole note",
                    example: None,
                },
                RefItem {
                    keyword: ":maxima",
                    description: "Octuple whole note",
                    example: None,
                },
                RefItem {
                    keyword: ":128",
                    description: "128th note",
                    example: None,
                },
                RefItem {
                    keyword: ":256",
                    description: "256th note",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Modifiers",
            items: vec![
                RefItem {
                    keyword: ":dot",
                    description: "Add augmentation dot (1.5x duration)",
                    example: Some("(note c4 :q :dot)"),
                },
                RefItem {
                    keyword: ":dots 2",
                    description: "Add multiple dots",
                    example: Some("(note c4 :h :dots 2)"),
                },
            ],
        },
    ];
    output("Duration Symbols", categories, format, use_colors)
}

fn show_pitches(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Pitch Names",
            items: vec![RefItem {
                keyword: "c d e f g a b",
                description: "Basic pitch names (case-insensitive)",
                example: Some("c4, D5, e3"),
            }],
        },
        RefCategory {
            name: "Octaves",
            items: vec![
                RefItem {
                    keyword: "0-9",
                    description: "Octave number follows pitch name",
                    example: Some("c4 = middle C"),
                },
                RefItem {
                    keyword: "c4",
                    description: "Middle C (261.63 Hz)",
                    example: None,
                },
                RefItem {
                    keyword: "a4",
                    description: "Concert A (440 Hz)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Accidentals",
            items: vec![
                RefItem {
                    keyword: "#",
                    description: "Sharp (raises pitch by semitone)",
                    example: Some("f#4, c#5"),
                },
                RefItem {
                    keyword: "b",
                    description: "Flat (lowers pitch by semitone)",
                    example: Some("bb3, eb4"),
                },
                RefItem {
                    keyword: "x",
                    description: "Double sharp",
                    example: Some("fx4"),
                },
                RefItem {
                    keyword: "bb",
                    description: "Double flat",
                    example: Some("bbb3"),
                },
                RefItem {
                    keyword: "n",
                    description: "Natural (explicit)",
                    example: Some("fn4"),
                },
            ],
        },
    ];
    output("Pitch Notation", categories, format, use_colors)
}

fn show_clefs(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Common Clefs",
            items: vec![
                RefItem {
                    keyword: ":treble",
                    description: "G clef on line 2 (treble clef)",
                    example: Some("Violin, flute, right hand piano"),
                },
                RefItem {
                    keyword: ":bass",
                    description: "F clef on line 4 (bass clef)",
                    example: Some("Cello, bass, left hand piano"),
                },
                RefItem {
                    keyword: ":alto",
                    description: "C clef on line 3 (alto clef)",
                    example: Some("Viola"),
                },
                RefItem {
                    keyword: ":tenor",
                    description: "C clef on line 4 (tenor clef)",
                    example: Some("Cello, bassoon, trombone"),
                },
            ],
        },
        RefCategory {
            name: "Special Clefs",
            items: vec![
                RefItem {
                    keyword: ":percussion",
                    description: "Percussion clef (neutral clef)",
                    example: Some("Drums, unpitched percussion"),
                },
                RefItem {
                    keyword: ":tab",
                    description: "Tablature clef",
                    example: Some("Guitar tablature"),
                },
                RefItem {
                    keyword: ":none",
                    description: "No clef displayed",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Octave Clefs",
            items: vec![
                RefItem {
                    keyword: ":treble-8vb",
                    description: "Treble clef, octave lower",
                    example: Some("Guitar, tenor voice"),
                },
                RefItem {
                    keyword: ":treble-8va",
                    description: "Treble clef, octave higher",
                    example: Some("Piccolo, glockenspiel"),
                },
                RefItem {
                    keyword: ":bass-8vb",
                    description: "Bass clef, octave lower",
                    example: Some("Contrabass"),
                },
            ],
        },
    ];
    output("Clefs", categories, format, use_colors)
}

fn show_keys(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Major Keys",
            items: vec![
                RefItem {
                    keyword: "c :major",
                    description: "C major (no sharps or flats)",
                    example: Some("(key c :major)"),
                },
                RefItem {
                    keyword: "g :major",
                    description: "G major (1 sharp)",
                    example: Some("F#"),
                },
                RefItem {
                    keyword: "d :major",
                    description: "D major (2 sharps)",
                    example: Some("F#, C#"),
                },
                RefItem {
                    keyword: "f :major",
                    description: "F major (1 flat)",
                    example: Some("Bb"),
                },
                RefItem {
                    keyword: "bb :major",
                    description: "Bb major (2 flats)",
                    example: Some("Bb, Eb"),
                },
            ],
        },
        RefCategory {
            name: "Minor Keys",
            items: vec![
                RefItem {
                    keyword: "a :minor",
                    description: "A minor (relative to C major)",
                    example: Some("(key a :minor)"),
                },
                RefItem {
                    keyword: "e :minor",
                    description: "E minor (1 sharp)",
                    example: None,
                },
                RefItem {
                    keyword: "d :minor",
                    description: "D minor (1 flat)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Modes",
            items: vec![
                RefItem {
                    keyword: ":dorian",
                    description: "Dorian mode",
                    example: Some("D dorian = D E F G A B C D"),
                },
                RefItem {
                    keyword: ":phrygian",
                    description: "Phrygian mode",
                    example: None,
                },
                RefItem {
                    keyword: ":lydian",
                    description: "Lydian mode",
                    example: None,
                },
                RefItem {
                    keyword: ":mixolydian",
                    description: "Mixolydian mode",
                    example: None,
                },
                RefItem {
                    keyword: ":aeolian",
                    description: "Aeolian mode (natural minor)",
                    example: None,
                },
                RefItem {
                    keyword: ":locrian",
                    description: "Locrian mode",
                    example: None,
                },
                RefItem {
                    keyword: ":ionian",
                    description: "Ionian mode (major)",
                    example: None,
                },
            ],
        },
    ];
    output("Key Signatures", categories, format, use_colors)
}

fn show_dynamics(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Soft Dynamics",
            items: vec![
                RefItem {
                    keyword: ":pppppp",
                    description: "As soft as possible",
                    example: None,
                },
                RefItem {
                    keyword: ":ppppp",
                    description: "Extremely soft",
                    example: None,
                },
                RefItem {
                    keyword: ":pppp",
                    description: "Very very soft",
                    example: None,
                },
                RefItem {
                    keyword: ":ppp",
                    description: "Very soft (pianississimo)",
                    example: None,
                },
                RefItem {
                    keyword: ":pp",
                    description: "Soft (pianissimo)",
                    example: None,
                },
                RefItem {
                    keyword: ":p",
                    description: "Soft (piano)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Medium Dynamics",
            items: vec![
                RefItem {
                    keyword: ":mp",
                    description: "Moderately soft (mezzo-piano)",
                    example: None,
                },
                RefItem {
                    keyword: ":mf",
                    description: "Moderately loud (mezzo-forte)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Loud Dynamics",
            items: vec![
                RefItem {
                    keyword: ":f",
                    description: "Loud (forte)",
                    example: None,
                },
                RefItem {
                    keyword: ":ff",
                    description: "Very loud (fortissimo)",
                    example: None,
                },
                RefItem {
                    keyword: ":fff",
                    description: "Very very loud (fortississimo)",
                    example: None,
                },
                RefItem {
                    keyword: ":ffff",
                    description: "Extremely loud",
                    example: None,
                },
                RefItem {
                    keyword: ":fffff",
                    description: "As loud as possible",
                    example: None,
                },
                RefItem {
                    keyword: ":ffffff",
                    description: "As loud as possible",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Accent Dynamics",
            items: vec![
                RefItem {
                    keyword: ":sf",
                    description: "Sforzando (sudden accent)",
                    example: None,
                },
                RefItem {
                    keyword: ":sfz",
                    description: "Sforzando",
                    example: None,
                },
                RefItem {
                    keyword: ":sffz",
                    description: "Sforzando fortissimo",
                    example: None,
                },
                RefItem {
                    keyword: ":fp",
                    description: "Forte-piano (loud then soft)",
                    example: None,
                },
                RefItem {
                    keyword: ":sfp",
                    description: "Sforzando-piano",
                    example: None,
                },
                RefItem {
                    keyword: ":sfpp",
                    description: "Sforzando-pianissimo",
                    example: None,
                },
                RefItem {
                    keyword: ":fz",
                    description: "Forzando",
                    example: None,
                },
                RefItem {
                    keyword: ":rf",
                    description: "Rinforzando",
                    example: None,
                },
                RefItem {
                    keyword: ":rfz",
                    description: "Rinforzando",
                    example: None,
                },
                RefItem {
                    keyword: ":n",
                    description: "Niente (nothing, fade to silence)",
                    example: None,
                },
            ],
        },
    ];
    output("Dynamic Markings", categories, format, use_colors)
}

fn show_articulations(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Standard Articulations",
            items: vec![
                RefItem {
                    keyword: ":staccato",
                    description: "Short, detached",
                    example: Some("."),
                },
                RefItem {
                    keyword: ":staccatissimo",
                    description: "Very short, extremely detached",
                    example: Some("wedge"),
                },
                RefItem {
                    keyword: ":tenuto",
                    description: "Held for full value",
                    example: Some("-"),
                },
                RefItem {
                    keyword: ":accent",
                    description: "Emphasized attack",
                    example: Some(">"),
                },
                RefItem {
                    keyword: ":strong-accent",
                    description: "Strongly emphasized (marcato)",
                    example: Some("^"),
                },
                RefItem {
                    keyword: ":detached-legato",
                    description: "Slightly separated but smooth",
                    example: Some("portato"),
                },
            ],
        },
        RefCategory {
            name: "Jazz Articulations",
            items: vec![
                RefItem {
                    keyword: ":scoop",
                    description: "Slide up into note",
                    example: None,
                },
                RefItem {
                    keyword: ":plop",
                    description: "Slide down into note",
                    example: None,
                },
                RefItem {
                    keyword: ":doit",
                    description: "Slide up from note",
                    example: None,
                },
                RefItem {
                    keyword: ":falloff",
                    description: "Slide down from note",
                    example: None,
                },
                RefItem {
                    keyword: ":spiccato",
                    description: "Light bouncing bow",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Breath & Pause",
            items: vec![
                RefItem {
                    keyword: ":breath-mark",
                    description: "Breath mark (comma)",
                    example: None,
                },
                RefItem {
                    keyword: ":caesura",
                    description: "Full stop/railroad tracks",
                    example: None,
                },
                RefItem {
                    keyword: ":stress",
                    description: "Stress mark",
                    example: None,
                },
                RefItem {
                    keyword: ":unstress",
                    description: "Unstress mark",
                    example: None,
                },
            ],
        },
    ];
    output("Articulations", categories, format, use_colors)
}

fn show_ornaments(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Trills",
            items: vec![
                RefItem {
                    keyword: ":trill",
                    description: "Trill (rapid alternation)",
                    example: Some("tr"),
                },
                RefItem {
                    keyword: ":shake",
                    description: "Shake ornament",
                    example: None,
                },
                RefItem {
                    keyword: ":wavy-line",
                    description: "Wavy line (trill extension)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Turns",
            items: vec![
                RefItem {
                    keyword: ":turn",
                    description: "Turn (4-note ornament)",
                    example: None,
                },
                RefItem {
                    keyword: ":inverted-turn",
                    description: "Inverted turn",
                    example: None,
                },
                RefItem {
                    keyword: ":delayed-turn",
                    description: "Turn played after main note",
                    example: None,
                },
                RefItem {
                    keyword: ":vertical-turn",
                    description: "Vertical turn symbol",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Mordents",
            items: vec![
                RefItem {
                    keyword: ":mordent",
                    description: "Lower mordent",
                    example: None,
                },
                RefItem {
                    keyword: ":inverted-mordent",
                    description: "Upper mordent (pralltriller)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Other Ornaments",
            items: vec![
                RefItem {
                    keyword: ":tremolo",
                    description: "Tremolo (rapid repetition)",
                    example: Some("1-8 slashes"),
                },
                RefItem {
                    keyword: ":schleifer",
                    description: "Baroque slide",
                    example: None,
                },
            ],
        },
    ];
    output("Ornaments", categories, format, use_colors)
}

fn show_instruments(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Keyboard",
            items: vec![
                RefItem {
                    keyword: ":piano",
                    description: "Piano",
                    example: Some("Grand staff"),
                },
                RefItem {
                    keyword: ":organ",
                    description: "Organ",
                    example: None,
                },
                RefItem {
                    keyword: ":harpsichord",
                    description: "Harpsichord",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Strings",
            items: vec![
                RefItem {
                    keyword: ":violin",
                    description: "Violin",
                    example: Some("Treble clef"),
                },
                RefItem {
                    keyword: ":viola",
                    description: "Viola",
                    example: Some("Alto clef"),
                },
                RefItem {
                    keyword: ":cello",
                    description: "Violoncello",
                    example: Some("Bass/tenor clef"),
                },
                RefItem {
                    keyword: ":bass",
                    description: "Double bass/contrabass",
                    example: Some("Bass clef, sounds 8vb"),
                },
                RefItem {
                    keyword: ":guitar",
                    description: "Guitar",
                    example: Some("Treble clef, sounds 8vb"),
                },
                RefItem {
                    keyword: ":harp",
                    description: "Harp",
                    example: Some("Grand staff"),
                },
            ],
        },
        RefCategory {
            name: "Woodwinds",
            items: vec![
                RefItem {
                    keyword: ":flute",
                    description: "Flute",
                    example: Some("Treble clef"),
                },
                RefItem {
                    keyword: ":oboe",
                    description: "Oboe",
                    example: None,
                },
                RefItem {
                    keyword: ":clarinet",
                    description: "Clarinet (Bb)",
                    example: Some("Transposing"),
                },
                RefItem {
                    keyword: ":bassoon",
                    description: "Bassoon",
                    example: Some("Bass/tenor clef"),
                },
                RefItem {
                    keyword: ":saxophone",
                    description: "Saxophone",
                    example: Some("Transposing"),
                },
            ],
        },
        RefCategory {
            name: "Brass",
            items: vec![
                RefItem {
                    keyword: ":trumpet",
                    description: "Trumpet",
                    example: Some("Transposing"),
                },
                RefItem {
                    keyword: ":horn",
                    description: "French horn",
                    example: Some("Transposing"),
                },
                RefItem {
                    keyword: ":trombone",
                    description: "Trombone",
                    example: Some("Bass/tenor clef"),
                },
                RefItem {
                    keyword: ":tuba",
                    description: "Tuba",
                    example: Some("Bass clef"),
                },
            ],
        },
        RefCategory {
            name: "Percussion",
            items: vec![
                RefItem {
                    keyword: ":timpani",
                    description: "Timpani",
                    example: Some("Bass clef, pitched"),
                },
                RefItem {
                    keyword: ":drums",
                    description: "Drum set",
                    example: Some("Percussion clef"),
                },
                RefItem {
                    keyword: ":percussion",
                    description: "General percussion",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Voice",
            items: vec![
                RefItem {
                    keyword: ":soprano",
                    description: "Soprano voice",
                    example: Some("Treble clef"),
                },
                RefItem {
                    keyword: ":alto",
                    description: "Alto voice",
                    example: Some("Treble clef"),
                },
                RefItem {
                    keyword: ":tenor",
                    description: "Tenor voice",
                    example: Some("Treble 8vb or tenor clef"),
                },
                RefItem {
                    keyword: ":baritone",
                    description: "Baritone voice",
                    example: Some("Bass clef"),
                },
                RefItem {
                    keyword: ":bass-voice",
                    description: "Bass voice",
                    example: Some("Bass clef"),
                },
            ],
        },
    ];
    output("Instruments", categories, format, use_colors)
}

fn show_barlines(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Standard Barlines",
            items: vec![
                RefItem {
                    keyword: ":regular",
                    description: "Single barline",
                    example: Some("|"),
                },
                RefItem {
                    keyword: ":double",
                    description: "Double barline (light-light)",
                    example: Some("||"),
                },
                RefItem {
                    keyword: ":final",
                    description: "Final barline (light-heavy)",
                    example: Some("|]"),
                },
                RefItem {
                    keyword: ":heavy",
                    description: "Heavy single barline",
                    example: None,
                },
                RefItem {
                    keyword: ":heavy-heavy",
                    description: "Double heavy barline",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Repeat Barlines",
            items: vec![
                RefItem {
                    keyword: ":repeat-forward",
                    description: "Start repeat",
                    example: Some("|:"),
                },
                RefItem {
                    keyword: ":repeat-backward",
                    description: "End repeat",
                    example: Some(":|"),
                },
                RefItem {
                    keyword: ":repeat-both",
                    description: "End and start repeat",
                    example: Some(":|:"),
                },
            ],
        },
        RefCategory {
            name: "Special Barlines",
            items: vec![
                RefItem {
                    keyword: ":dashed",
                    description: "Dashed barline",
                    example: None,
                },
                RefItem {
                    keyword: ":dotted",
                    description: "Dotted barline",
                    example: None,
                },
                RefItem {
                    keyword: ":tick",
                    description: "Tick mark (short barline)",
                    example: None,
                },
                RefItem {
                    keyword: ":short",
                    description: "Short barline",
                    example: None,
                },
                RefItem {
                    keyword: ":none",
                    description: "No barline",
                    example: None,
                },
            ],
        },
    ];
    output("Barline Types", categories, format, use_colors)
}

fn show_accidentals(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Standard Accidentals",
            items: vec![
                RefItem {
                    keyword: ":sharp",
                    description: "Sharp",
                    example: Some("#"),
                },
                RefItem {
                    keyword: ":flat",
                    description: "Flat",
                    example: Some("b"),
                },
                RefItem {
                    keyword: ":natural",
                    description: "Natural",
                    example: None,
                },
                RefItem {
                    keyword: ":double-sharp",
                    description: "Double sharp",
                    example: Some("x"),
                },
                RefItem {
                    keyword: ":double-flat",
                    description: "Double flat",
                    example: Some("bb"),
                },
            ],
        },
        RefCategory {
            name: "Compound Accidentals",
            items: vec![
                RefItem {
                    keyword: ":natural-sharp",
                    description: "Natural then sharp",
                    example: None,
                },
                RefItem {
                    keyword: ":natural-flat",
                    description: "Natural then flat",
                    example: None,
                },
                RefItem {
                    keyword: ":sharp-sharp",
                    description: "Sharp-sharp (same as double sharp)",
                    example: None,
                },
                RefItem {
                    keyword: ":flat-flat",
                    description: "Flat-flat (same as double flat)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Microtonal Accidentals",
            items: vec![
                RefItem {
                    keyword: ":quarter-sharp",
                    description: "Quarter-tone sharp",
                    example: None,
                },
                RefItem {
                    keyword: ":quarter-flat",
                    description: "Quarter-tone flat",
                    example: None,
                },
                RefItem {
                    keyword: ":three-quarters-sharp",
                    description: "Three-quarter-tone sharp",
                    example: None,
                },
                RefItem {
                    keyword: ":three-quarters-flat",
                    description: "Three-quarter-tone flat",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Arrow Accidentals",
            items: vec![
                RefItem {
                    keyword: ":sharp-up",
                    description: "Sharp raised by comma",
                    example: None,
                },
                RefItem {
                    keyword: ":sharp-down",
                    description: "Sharp lowered by comma",
                    example: None,
                },
                RefItem {
                    keyword: ":flat-up",
                    description: "Flat raised by comma",
                    example: None,
                },
                RefItem {
                    keyword: ":flat-down",
                    description: "Flat lowered by comma",
                    example: None,
                },
                RefItem {
                    keyword: ":natural-up",
                    description: "Natural raised by comma",
                    example: None,
                },
                RefItem {
                    keyword: ":natural-down",
                    description: "Natural lowered by comma",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Special Accidentals",
            items: vec![
                RefItem {
                    keyword: ":sori",
                    description: "Persian sori",
                    example: None,
                },
                RefItem {
                    keyword: ":koron",
                    description: "Persian koron",
                    example: None,
                },
                RefItem {
                    keyword: ":triple-sharp",
                    description: "Triple sharp",
                    example: None,
                },
                RefItem {
                    keyword: ":triple-flat",
                    description: "Triple flat",
                    example: None,
                },
            ],
        },
    ];
    output("Accidentals", categories, format, use_colors)
}

fn show_noteheads(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Standard Noteheads",
            items: vec![
                RefItem {
                    keyword: ":normal",
                    description: "Normal oval notehead",
                    example: Some("Default"),
                },
                RefItem {
                    keyword: ":diamond",
                    description: "Diamond notehead",
                    example: Some("Harmonics"),
                },
                RefItem {
                    keyword: ":triangle",
                    description: "Triangle notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":square",
                    description: "Square notehead",
                    example: Some("Gregorian chant"),
                },
                RefItem {
                    keyword: ":rectangle",
                    description: "Rectangle notehead",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Percussion/Special",
            items: vec![
                RefItem {
                    keyword: ":x",
                    description: "X notehead",
                    example: Some("Hi-hat, spoken"),
                },
                RefItem {
                    keyword: ":cross",
                    description: "Cross notehead",
                    example: Some("Percussion"),
                },
                RefItem {
                    keyword: ":circle-x",
                    description: "Circled X notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":slash",
                    description: "Slash notehead",
                    example: Some("Rhythmic notation"),
                },
                RefItem {
                    keyword: ":cluster",
                    description: "Cluster notehead",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Directional",
            items: vec![
                RefItem {
                    keyword: ":arrow-up",
                    description: "Arrow up notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":arrow-down",
                    description: "Arrow down notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":inverted-triangle",
                    description: "Inverted triangle",
                    example: None,
                },
                RefItem {
                    keyword: ":left-triangle",
                    description: "Left-pointing triangle",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Decorative",
            items: vec![
                RefItem {
                    keyword: ":circled",
                    description: "Circled notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":slashed",
                    description: "Slashed notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":back-slashed",
                    description: "Back-slashed notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":circle-dot",
                    description: "Circle with dot notehead",
                    example: None,
                },
                RefItem {
                    keyword: ":none",
                    description: "No notehead (stem only)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Shape Note (Solfege)",
            items: vec![
                RefItem {
                    keyword: ":do",
                    description: "Do (triangle)",
                    example: None,
                },
                RefItem {
                    keyword: ":re",
                    description: "Re (moon/crescent)",
                    example: None,
                },
                RefItem {
                    keyword: ":mi",
                    description: "Mi (diamond)",
                    example: None,
                },
                RefItem {
                    keyword: ":fa",
                    description: "Fa (right-pointing triangle)",
                    example: None,
                },
                RefItem {
                    keyword: ":so",
                    description: "So (oval)",
                    example: None,
                },
                RefItem {
                    keyword: ":la",
                    description: "La (rectangle)",
                    example: None,
                },
                RefItem {
                    keyword: ":ti",
                    description: "Ti (ice cream cone)",
                    example: None,
                },
            ],
        },
    ];
    output("Notehead Shapes", categories, format, use_colors)
}

fn show_fermatas(format: OutputFormat, use_colors: bool) -> ExitCode {
    let categories = vec![
        RefCategory {
            name: "Standard Fermatas",
            items: vec![
                RefItem {
                    keyword: ":fermata",
                    description: "Normal fermata",
                    example: Some("Standard pause symbol"),
                },
                RefItem {
                    keyword: ":fermata-inverted",
                    description: "Inverted fermata (below staff)",
                    example: None,
                },
            ],
        },
        RefCategory {
            name: "Fermata Shapes",
            items: vec![
                RefItem {
                    keyword: ":normal",
                    description: "Normal curved fermata",
                    example: Some("Most common"),
                },
                RefItem {
                    keyword: ":angled",
                    description: "Angled/triangular fermata",
                    example: Some("Short fermata"),
                },
                RefItem {
                    keyword: ":square",
                    description: "Square fermata",
                    example: Some("Long fermata"),
                },
                RefItem {
                    keyword: ":double-angled",
                    description: "Double angled fermata",
                    example: Some("Very short"),
                },
                RefItem {
                    keyword: ":double-square",
                    description: "Double square fermata",
                    example: Some("Very long"),
                },
                RefItem {
                    keyword: ":double-dot",
                    description: "Double-dot fermata",
                    example: None,
                },
                RefItem {
                    keyword: ":half-curve",
                    description: "Half-curve fermata",
                    example: None,
                },
                RefItem {
                    keyword: ":curlew",
                    description: "Curlew fermata (Britten)",
                    example: None,
                },
            ],
        },
    ];
    output("Fermata Shapes", categories, format, use_colors)
}
