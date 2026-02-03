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
use fermata::musicxml::{emit, parse};
use fermata::repl::Repl;
use fermata::sexpr::{ToSexpr, print_sexpr};

mod show;

/// An S-expression DSL for music notation
#[derive(Parser)]
#[command(name = "fermata", version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,

    /// Set log level (error, warn, info, debug, trace). Use 'debug' for verbose output; 'trace' includes noisy dependency logs.
    #[arg(short = 'l', long, global = true, default_value = "warn")]
    log_level: String,

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

    /// Import MusicXML and convert to Fermata Lisp
    Import {
        /// Input MusicXML file (use '-' for stdin)
        #[arg(value_name = "FILE")]
        file: Option<String>,

        /// Output file (omit for stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<String>,
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

    /// Start the interactive REPL
    Repl,
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

    // Initialize logging (stderr to avoid interleaving with REPL output)
    let log_level: twyg::LogLevel = cli.log_level.parse().unwrap_or(twyg::LogLevel::Warn);
    if let Ok(log_opts) = twyg::OptsBuilder::new()
        .coloured(use_colors)
        .output(twyg::Output::Stderr)
        .level(log_level)
        .report_caller(false)
        .build()
    {
        // Ignore InitError (logger already set, e.g. in tests)
        match twyg::setup(log_opts) {
            Ok(_) | Err(twyg::TwygError::InitError) => {}
            Err(e) => eprintln!("Warning: Failed to initialize logger: {:?}", e),
        }
    }

    match cli.command {
        Some(Commands::Compile {
            file,
            output,
            target,
        }) => cmd_compile(file.as_deref(), output.as_deref(), target, use_colors),
        Some(Commands::Check { file }) => cmd_check(file.as_deref(), use_colors),
        Some(Commands::Import { file, output }) => {
            cmd_import(file.as_deref(), output.as_deref(), use_colors)
        }
        Some(Commands::Show { topic, format }) => show::run(topic, format, use_colors),
        Some(Commands::Repl) | None => {
            // Launch the interactive REPL (default when no command given)
            cmd_repl(use_colors)
        }
    }
}

/// REPL command - start interactive session
fn cmd_repl(use_colors: bool) -> ExitCode {
    match Repl::new(use_colors) {
        Ok(mut repl) => match repl.run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("REPL error: {}", e);
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("Failed to start REPL: {}", e);
            ExitCode::FAILURE
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

/// Import command - convert MusicXML to Fermata Lisp
fn cmd_import(file: Option<&str>, output: Option<&str>, use_colors: bool) -> ExitCode {
    // Default to stdin if no file specified
    let input_path = file.unwrap_or("-");

    // Read input
    let xml = match read_input(input_path) {
        Ok(s) => s,
        Err(e) => {
            print_error("Error reading input", &e.to_string(), use_colors);
            return ExitCode::FAILURE;
        }
    };

    // Parse MusicXML
    let score = match parse(&xml) {
        Ok(s) => s,
        Err(e) => {
            print_error("MusicXML parse error", &e.to_string(), use_colors);
            return ExitCode::FAILURE;
        }
    };

    // Convert to S-expression
    let sexpr = score.to_sexpr();

    // Print to string
    let output_content = print_sexpr(&sexpr);

    // Write output
    match write_output(output, &output_content) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            print_error("Error writing output", &e.to_string(), use_colors);
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
