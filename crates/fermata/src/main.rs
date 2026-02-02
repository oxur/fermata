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
//! # Show version
//! fermata --version
//! fermata version
//!
//! # Show help
//! fermata --help
//! fermata help
//! ```

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::ExitCode;

use fermata::lang::{check, compile};
use fermata::musicxml::emit;

const USAGE: &str = r#"
USAGE:
    fermata <COMMAND> [OPTIONS]

COMMANDS:
    compile <FILE>      Compile a .fm file to MusicXML
    check <FILE>        Check if a .fm file is valid
    version             Show version information
    help                Show this help message

OPTIONS:
    -o, --output <FILE>     Output file (default: stdout)
    -h, --help              Show help for a command
    -V, --version           Show version

EXAMPLES:
    fermata compile score.fm -o score.musicxml
    fermata check score.fm
    cat score.fm | fermata compile -
"#;

/// Entry point
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return ExitCode::SUCCESS;
    }

    let command = &args[1];

    match command.as_str() {
        "compile" => cmd_compile(&args[2..]),
        "check" => cmd_check(&args[2..]),
        "version" | "-V" | "--version" => {
            print_version();
            ExitCode::SUCCESS
        }
        "help" | "-h" | "--help" => {
            print_help();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Run 'fermata help' for usage information.");
            ExitCode::FAILURE
        }
    }
}

/// Compile command
fn cmd_compile(args: &[String]) -> ExitCode {
    let mut input_file: Option<&str> = None;
    let mut output_file: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --output requires a file path");
                    return ExitCode::FAILURE;
                }
                output_file = Some(&args[i + 1]);
                i += 2;
            }
            "-h" | "--help" => {
                println!("Compile a Fermata file to MusicXML.");
                println!();
                println!("USAGE:");
                println!("    fermata compile <FILE> [OPTIONS]");
                println!();
                println!("ARGS:");
                println!("    <FILE>    Input file (use '-' for stdin)");
                println!();
                println!("OPTIONS:");
                println!("    -o, --output <FILE>    Output file (default: stdout)");
                println!("    -h, --help             Show this help");
                return ExitCode::SUCCESS;
            }
            _ => {
                if input_file.is_none() {
                    input_file = Some(&args[i]);
                } else {
                    eprintln!("Error: unexpected argument '{}'", args[i]);
                    return ExitCode::FAILURE;
                }
                i += 1;
            }
        }
    }

    let input_file = match input_file {
        Some(f) => f,
        None => {
            eprintln!("Error: no input file specified");
            eprintln!("Run 'fermata compile --help' for usage information.");
            return ExitCode::FAILURE;
        }
    };

    // Read input
    let source = match read_input(input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Compile
    let score = match compile(&source) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Emit MusicXML
    let xml = match emit(&score) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("MusicXML generation error: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Write output
    match write_output(output_file, &xml) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error writing output: {}", e);
            ExitCode::FAILURE
        }
    }
}

/// Check command
fn cmd_check(args: &[String]) -> ExitCode {
    let mut input_file: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                println!("Check if a Fermata file is valid.");
                println!();
                println!("USAGE:");
                println!("    fermata check <FILE>");
                println!();
                println!("ARGS:");
                println!("    <FILE>    Input file (use '-' for stdin)");
                println!();
                println!("OPTIONS:");
                println!("    -h, --help    Show this help");
                return ExitCode::SUCCESS;
            }
            _ => {
                if input_file.is_none() {
                    input_file = Some(&args[i]);
                } else {
                    eprintln!("Error: unexpected argument '{}'", args[i]);
                    return ExitCode::FAILURE;
                }
                i += 1;
            }
        }
    }

    let input_file = match input_file {
        Some(f) => f,
        None => {
            eprintln!("Error: no input file specified");
            eprintln!("Run 'fermata check --help' for usage information.");
            return ExitCode::FAILURE;
        }
    };

    // Read input
    let source = match read_input(input_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return ExitCode::FAILURE;
        }
    };

    // Check
    match check(&source) {
        Ok(()) => {
            println!("OK: {} is valid", input_file);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error in {}: {}", input_file, e);
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
        Some(p) => {
            // Create parent directories if needed
            if let Some(parent) = Path::new(p).parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(p, content)
        }
        None => {
            let mut stdout = io::stdout().lock();
            stdout.write_all(content.as_bytes())?;
            stdout.flush()
        }
    }
}

/// Print version information
fn print_version() {
    println!("fermata {}", fermata::VERSION);
}

/// Print help message
fn print_help() {
    println!("fermata {} - An S-expression DSL for music notation", fermata::VERSION);
    println!("{}", USAGE);
}
