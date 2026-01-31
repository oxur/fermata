//! Fermata CLI - REPL and compiler for Fermata Lisp
//!
//! # Usage
//!
//! ```bash
//! # Start the REPL
//! fermata
//!
//! # Compile to MusicXML
//! fermata compile score.fm -o score.musicxml
//!
//! # Compile to LilyPond
//! fermata compile score.fm -o score.ly --target lilypond
//! ```

fn main() {
    println!("𝄐 Fermata v{}", fermata::VERSION);
    println!("An S-expression DSL for working with MusicXML");
    println!();
    println!("🚧 Under construction - REPL coming soon!");
    println!();
    println!("Planned commands:");
    println!(" fermata Start the REPL");
    println!(" fermata compile Compile .fm files to MusicXML/LilyPond");
    println!(" fermata render Render to SVG (via verovioxide)");
    println!(" fermata fmt Format Fermata source files");
    println!();
    println!("For now, use the library API:");
    println!();
    println!(r#" use fermata::{{parse, compile_to_musicxml}};"#);
    println!();
    println!("Repository: https://github.com/oxur/fermata");
}
