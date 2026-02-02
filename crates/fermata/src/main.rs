//! Fermata CLI - Compiler for Fermata S-expression music notation.
//!
//! # Usage
//!
//! ```bash
//! # Check if a file is valid
//! fermata check score.fm
//!
//! # Compile to MusicXML
//! fermata compile score.fm -o score.musicxml
//!
//! # Show reference information
//! fermata show durations
//! fermata show targets --format json
//!
//! # Show version
//! fermata --version
//!
//! # Show help
//! fermata --help
//! ```

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use owo_colors::OwoColorize;

use fermata::lang::{check, compile};
use fermata::musicxml::emit;

mod show;

/// An S-expression DSL for music notation
#[derive(Parser)]
#[command(name = "fermata", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Fermata file to MusicXML
    Compile {
        /// Input file (use '-' for stdin)
        #[arg(value_name = "FILE")]
        file: Option<String>,

        /// Output file (omit for stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputTarget::MusicXml)]
        target: OutputTarget,
    },

    /// Check if a Fermata file is valid
    Check {
        /// Input file (use '-' for stdin)
        #[arg(value_name = "FILE")]
        file: Option<String>,
    },

    /// Display reference information
    Show {
        /// Topic to display
        #[command(subcommand)]
        topic: ShowTopic,

        /// Output format
        #[arg(long, value_enum, default_value_t = OutputFormat::Text, global = true)]
        format: OutputFormat,
    },
}

/// Output target format for compilation
#[derive(Clone, ValueEnum)]
enum OutputTarget {
    /// MusicXML format
    #[value(alias = "xml")]
    MusicXml,
    /// LilyPond format (not yet implemented)
    #[value(alias = "ly")]
    LilyPond,
}

/// Output format for show commands
#[derive(Clone, Copy, ValueEnum, Default)]
pub enum OutputFormat {
    /// Human-readable text with colors
    #[default]
    Text,
    /// Machine-readable JSON
    Json,
}

/// Topics for the show command
#[derive(Clone, Subcommand)]
pub enum ShowTopic {
    /// Supported output formats
    Targets,
    /// Quick syntax reference
    Syntax,
    /// Duration symbols (:w, :h, :q, :8, etc.)
    Durations,
    /// Pitch notation (C4, D#5, Bb3, etc.)
    Pitches,
    /// Available clefs
    Clefs,
    /// Key signatures and modes
    Keys,
    /// Dynamic markings (pp, p, mp, mf, f, ff, etc.)
    Dynamics,
    /// Articulation types
    Articulations,
    /// Ornament types
    Ornaments,
    /// Instrument shortcuts
    Instruments,
    /// Barline types
    Barlines,
    /// Accidental variations
    Accidentals,
    /// Notehead shapes
    Noteheads,
    /// Fermata shapes
    Fermatas,
}

/// Entry point
fn main() -> ExitCode {
    let cli = Cli::parse();

    // Determine if colors should be used
    let use_colors = !cli.no_color && std::env::var("NO_COLOR").is_err();

    match cli.command {
        Some(Commands::Compile {
            file,
            output,
            target,
        }) => cmd_compile(file.as_deref(), output.as_deref(), target, use_colors),
        Some(Commands::Check { file }) => cmd_check(file.as_deref(), use_colors),
        Some(Commands::Show { topic, format }) => show::run(topic, format, use_colors),
        None => {
            // No subcommand provided - show help
            use clap::CommandFactory;
            Cli::command().print_help().unwrap();
            println!();
            ExitCode::SUCCESS
        }
    }
}

/// Print an error message with optional coloring.
fn print_error(label: &str, message: &str, use_colors: bool) {
    if use_colors {
        eprintln!("{}: {}", label.red(), message);
    } else {
        eprintln!("{}: {}", label, message);
    }
}

/// Compile command
fn cmd_compile(
    file: Option<&str>,
    output: Option<&str>,
    target: OutputTarget,
    use_colors: bool,
) -> ExitCode {
    // Default to stdin if no file specified
    let input_path = file.unwrap_or("-");

    // Read input
    let source = match read_input(input_path) {
        Ok(s) => s,
        Err(e) => {
            print_error("Error reading input", &e.to_string(), use_colors);
            return ExitCode::FAILURE;
        }
    };

    // Compile
    let score = match compile(&source) {
        Ok(s) => s,
        Err(e) => {
            print_error("Compilation error", &e.to_string(), use_colors);
            return ExitCode::FAILURE;
        }
    };

    // Generate output based on target
    let output_content = match target {
        OutputTarget::MusicXml => match emit(&score) {
            Ok(x) => x,
            Err(e) => {
                print_error("MusicXML generation error", &e.to_string(), use_colors);
                return ExitCode::FAILURE;
            }
        },
        OutputTarget::LilyPond => {
            print_error(
                "Error",
                "LilyPond output is not yet implemented",
                use_colors,
            );
            return ExitCode::FAILURE;
        }
    };

    // Write output
    match write_output(output, &output_content) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            print_error("Error writing output", &e.to_string(), use_colors);
            ExitCode::FAILURE
        }
    }
}

/// Check command
fn cmd_check(file: Option<&str>, use_colors: bool) -> ExitCode {
    // Default to stdin if no file specified
    let input_path = file.unwrap_or("-");

    // Read input
    let source = match read_input(input_path) {
        Ok(s) => s,
        Err(e) => {
            print_error("Error reading input", &e.to_string(), use_colors);
            return ExitCode::FAILURE;
        }
    };

    // Check
    match check(&source) {
        Ok(()) => {
            if use_colors {
                println!("{}: {} is valid", "OK".green(), input_path);
            } else {
                println!("OK: {} is valid", input_path);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            if use_colors {
                eprintln!("{} in {}: {}", "Error".red(), input_path, e);
            } else {
                eprintln!("Error in {}: {}", input_path, e);
            }
            ExitCode::FAILURE
        }
    }
}

/// Read input from file or stdin
fn read_input(path: &str) -> io::Result<String> {
    if path == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read_to_string(path)
    }
}

/// Write output to file or stdout
fn write_output(path: Option<&str>, content: &str) -> io::Result<()> {
    match path {
        Some("-") | None => {
            let mut stdout = io::stdout().lock();
            stdout.write_all(content.as_bytes())?;
            stdout.flush()
        }
        Some(p) => {
            // Create parent directories if needed
            if let Some(parent) = Path::new(p).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(p, content)
        }
    }
}
