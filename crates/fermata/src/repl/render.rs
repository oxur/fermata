//! Rendering support for the REPL (requires `render` feature).
//!
//! This module provides integration with verovioxide for rendering
//! music notation to various formats (PNG, MEI, MIDI).
//!
//! The Verovio toolkit is cached to avoid expensive re-initialization
//! on every render call.

use std::sync::Mutex;

use crate::ir::score::ScorePartwise;
use crate::musicxml;

use super::error::{ReplError, ReplResult};
use super::session::RenderOptions;

use verovioxide::{Toolkit, Options as VerovioOptions, Mei, Midi, Png};

/// Global cached toolkit wrapped in a Mutex for thread-safe access.
/// Using Mutex instead of OnceLock because we need mutable access to the Toolkit.
static CACHED_TOOLKIT: Mutex<Option<Toolkit>> = Mutex::new(None);

/// Get or initialize the cached toolkit.
fn get_or_init_toolkit() -> ReplResult<std::sync::MutexGuard<'static, Option<Toolkit>>> {
    let mut guard = CACHED_TOOLKIT
        .lock()
        .map_err(|e| ReplError::render(format!("Failed to lock toolkit: {}", e)))?;

    if guard.is_none() {
        let toolkit = Toolkit::new()
            .map_err(|e| ReplError::render(format!("Failed to initialize verovio: {}", e)))?;
        *guard = Some(toolkit);
    }

    Ok(guard)
}

/// Terminal image protocol support level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalSupport {
    /// Kitty graphics protocol (best quality)
    Kitty,
    /// iTerm2 inline images
    Iterm2,
    /// Sixel graphics
    Sixel,
    /// Block characters only (fallback)
    Block,
}

impl TerminalSupport {
    /// Detect the current terminal's image support.
    pub fn detect() -> Self {
        // Check environment variables for terminal type
        if std::env::var("KITTY_WINDOW_ID").is_ok() {
            return Self::Kitty;
        }

        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            if term_program == "iTerm.app" {
                return Self::Iterm2;
            }
        }

        // Check for sixel support via TERM
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("sixel") || term.contains("mlterm") {
                return Self::Sixel;
            }
        }

        // Fallback to block characters
        Self::Block
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Kitty => "Kitty graphics protocol",
            Self::Iterm2 => "iTerm2 inline images",
            Self::Sixel => "Sixel graphics",
            Self::Block => "Unicode block characters (reduced quality)",
        }
    }
}

/// A renderer that converts ScorePartwise to various output formats.
pub struct Renderer {
    toolkit: Toolkit,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new() -> ReplResult<Self> {
        let toolkit = Toolkit::new()
            .map_err(|e| ReplError::render(format!("Failed to initialize verovio: {}", e)))?;
        Ok(Self { toolkit })
    }

    /// Load a score into the renderer.
    fn load_score(&mut self, score: &ScorePartwise) -> ReplResult<()> {
        // First convert to MusicXML
        let xml = musicxml::emit(score)
            .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

        // Load into verovio
        self.toolkit
            .load_data(&xml)
            .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

        Ok(())
    }

    /// Configure rendering options.
    fn configure(&mut self, options: &RenderOptions) -> ReplResult<()> {
        // Use larger page width for better resolution
        // Scale 100 = 100% of the page width
        let verovio_options = VerovioOptions::builder()
            .page_width(options.width.max(1500))  // Minimum 1500 for decent resolution
            .adjust_page_height(true)
            .scale(100)
            .build();

        self.toolkit
            .set_options(&verovio_options)
            .map_err(|e| ReplError::render(format!("Failed to set options: {}", e)))?;

        Ok(())
    }

    /// Get the number of pages in the loaded score.
    pub fn page_count(&self) -> u32 {
        self.toolkit.page_count()
    }

    /// Render to PNG bytes.
    pub fn render_png(
        &mut self,
        score: &ScorePartwise,
        options: &RenderOptions,
    ) -> ReplResult<Vec<u8>> {
        // Configure BEFORE loading (verovio requirement)
        self.configure(options)?;
        self.load_score(score)?;

        // Use 4x scale for higher resolution terminal display
        // Add white background for better visibility in terminals
        let png_bytes = self
            .toolkit
            .render(
                Png::page(options.page)
                    .scale(4.0)
                    .white_background()
            )
            .map_err(|e| ReplError::render(format!("Failed to render PNG: {}", e)))?;

        Ok(png_bytes)
    }

    /// Render to MEI string.
    pub fn render_mei(&mut self, score: &ScorePartwise) -> ReplResult<String> {
        self.load_score(score)?;

        let mei_string = self
            .toolkit
            .render(Mei)
            .map_err(|e| ReplError::render(format!("Failed to render MEI: {}", e)))?;

        Ok(mei_string)
    }

    /// Render to MIDI (base64-encoded).
    pub fn render_midi(&mut self, score: &ScorePartwise) -> ReplResult<String> {
        self.load_score(score)?;

        let midi_b64 = self
            .toolkit
            .render(Midi)
            .map_err(|e| ReplError::render(format!("Failed to render MIDI: {}", e)))?;

        Ok(midi_b64)
    }
}

/// Display a PNG image in the terminal using viuer.
pub fn display_png_in_terminal(png_bytes: &[u8], _options: &RenderOptions) -> ReplResult<()> {
    let img = image::load_from_memory(png_bytes)
        .map_err(|e| ReplError::render(format!("Failed to decode PNG: {}", e)))?;

    // Let viuer auto-detect the best protocol and size
    // Don't constrain width - let it use available terminal width
    let config = viuer::Config {
        absolute_offset: false,
        // These None values let viuer choose optimal sizing
        width: None,
        height: None,
        // Try to use native terminal graphics if available
        use_kitty: true,
        use_iterm: true,
        ..Default::default()
    };

    viuer::print(&img, &config)
        .map_err(|e| ReplError::render(format!("Failed to display image: {}", e)))?;

    Ok(())
}

/// Format a score as MEI (actual rendering).
pub fn format_as_mei(score: &ScorePartwise, use_colors: bool) -> String {
    match render_mei_cached(score) {
        Ok(mei) => {
            if use_colors {
                use owo_colors::OwoColorize;
                format!("{}", mei.cyan())
            } else {
                mei
            }
        }
        Err(e) => format_render_error(&e, use_colors),
    }
}

/// Render MEI using the cached toolkit.
fn render_mei_cached(score: &ScorePartwise) -> ReplResult<String> {
    let xml = musicxml::emit(score)
        .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

    let mut guard = get_or_init_toolkit()?;
    let toolkit = guard.as_mut().unwrap();

    toolkit
        .load_data(&xml)
        .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

    toolkit
        .render(Mei)
        .map_err(|e| ReplError::render(format!("Failed to render MEI: {}", e)))
}

/// Format a score as MIDI (base64, actual rendering).
pub fn format_as_midi(score: &ScorePartwise, use_colors: bool) -> String {
    match render_midi_cached(score) {
        Ok(midi_b64) => {
            let output = format!("MIDI (base64):\n{}", midi_b64);
            if use_colors {
                use owo_colors::OwoColorize;
                format!("{}", output.cyan())
            } else {
                output
            }
        }
        Err(e) => format_render_error(&e, use_colors),
    }
}

/// Render MIDI using the cached toolkit.
fn render_midi_cached(score: &ScorePartwise) -> ReplResult<String> {
    let xml = musicxml::emit(score)
        .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

    let mut guard = get_or_init_toolkit()?;
    let toolkit = guard.as_mut().unwrap();

    toolkit
        .load_data(&xml)
        .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

    toolkit
        .render(Midi)
        .map_err(|e| ReplError::render(format!("Failed to render MIDI: {}", e)))
}

/// Render and display a score as PNG in the terminal.
pub fn display_as_png(
    score: &ScorePartwise,
    options: &RenderOptions,
    use_colors: bool,
) -> Option<String> {
    match render_png_cached(score, options) {
        Ok((png_bytes, page_count)) => {
            // Display the image
            if let Err(e) = display_png_in_terminal(&png_bytes, options) {
                return Some(format_render_error(&e, use_colors));
            }

            // Show page info if enabled and multi-page
            if options.show_page_info && page_count > 1 {
                let info = format!("Page {} of {}", options.page, page_count);
                if use_colors {
                    use owo_colors::OwoColorize;
                    Some(format!("{}", info.dimmed()))
                } else {
                    Some(info)
                }
            } else {
                None
            }
        }
        Err(e) => Some(format_render_error(&e, use_colors)),
    }
}

/// Render PNG using the cached toolkit.
fn render_png_cached(score: &ScorePartwise, options: &RenderOptions) -> ReplResult<(Vec<u8>, u32)> {
    let xml = musicxml::emit(score)
        .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

    let mut guard = get_or_init_toolkit()?;
    let toolkit = guard.as_mut().unwrap();

    // Configure options BEFORE loading (verovio requirement)
    let verovio_options = VerovioOptions::builder()
        .page_width(options.width.max(1200))  // Reduced from 1500 for faster rendering
        .adjust_page_height(true)
        .scale(100)
        .build();

    toolkit
        .set_options(&verovio_options)
        .map_err(|e| ReplError::render(format!("Failed to set options: {}", e)))?;

    toolkit
        .load_data(&xml)
        .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

    let page_count = toolkit.page_count();

    // Use 2x scale instead of 4x for faster rendering while maintaining reasonable quality
    let png_bytes = toolkit
        .render(
            Png::page(options.page)
                .scale(2.0)
                .white_background()
        )
        .map_err(|e| ReplError::render(format!("Failed to render PNG: {}", e)))?;

    Ok((png_bytes, page_count))
}

/// Format a render error.
fn format_render_error(error: &ReplError, use_colors: bool) -> String {
    if use_colors {
        use owo_colors::OwoColorize;
        format!("{}: {}", "Render error".red(), error)
    } else {
        format!("Render error: {}", error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_support_detect() {
        // Just verify it doesn't panic
        let _ = TerminalSupport::detect();
    }

    #[test]
    fn test_terminal_support_description() {
        assert!(!TerminalSupport::Kitty.description().is_empty());
        assert!(!TerminalSupport::Iterm2.description().is_empty());
        assert!(!TerminalSupport::Sixel.description().is_empty());
        assert!(!TerminalSupport::Block.description().is_empty());
    }
}
