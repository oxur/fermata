//! Rendering support for the REPL (requires `render` feature).
//!
//! This module provides integration with verovioxide for rendering
//! music notation to various formats (PNG, MEI, MIDI).
//!
//! The Verovio toolkit is cached to avoid expensive re-initialization
//! on every render call.
//!
//! For terminal image display, we use native protocols (iTerm2, Kitty)
//! directly for maximum performance.

use std::sync::Mutex;

use log::debug;

use crate::ir::score::ScorePartwise;
use crate::musicxml;

use super::error::{ReplError, ReplResult};
use super::session::RenderOptions;

use verovioxide::{Mei, Midi, Options as VerovioOptions, Png, Toolkit};

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
    /// iTerm2 inline images (also supported by WezTerm)
    Iterm2,
    /// Sixel graphics
    Sixel,
    /// Unicode block characters (fallback, works everywhere)
    BlockChars,
    /// No image support at all
    None,
}

impl TerminalSupport {
    /// Detect the current terminal's image support.
    pub fn detect() -> Self {
        // Check environment variables for terminal type
        if std::env::var("KITTY_WINDOW_ID").is_ok() {
            return Self::Kitty;
        }

        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            match term_program.as_str() {
                "iTerm.app" => return Self::Iterm2,
                "WezTerm" => return Self::Iterm2, // WezTerm supports iTerm2 protocol
                _ => {}
            }
        }

        // Check for sixel support via TERM
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("sixel") || term.contains("mlterm") {
                return Self::Sixel;
            }
        }

        // Fall back to block characters (works on any terminal with Unicode + 24-bit color)
        Self::BlockChars
    }

    /// Get a human-readable description.
    pub fn description(&self) -> &'static str {
        match self {
            Self::Kitty => "Kitty graphics protocol",
            Self::Iterm2 => "iTerm2 inline images",
            Self::Sixel => "Sixel graphics",
            Self::BlockChars => "Unicode block characters",
            Self::None => "No image support",
        }
    }
}

/// Display a PNG image in the terminal.
pub fn display_png_in_terminal(
    png_bytes: &[u8],
    options: &RenderOptions,
    dark_mode: bool,
) -> ReplResult<()> {
    use std::time::Instant;

    let t0 = Instant::now();

    // Detect terminal support
    let support = TerminalSupport::detect();

    // Use native protocol for supported terminals, fall back to block characters
    let result = match support {
        TerminalSupport::Iterm2 => display_iterm2_inline(png_bytes, dark_mode),
        TerminalSupport::Kitty => display_kitty_inline(png_bytes, dark_mode),
        TerminalSupport::Sixel | TerminalSupport::BlockChars => {
            display_block_characters(png_bytes, options.width, dark_mode)
        }
        TerminalSupport::None => Err(ReplError::render(
            "Terminal does not support image display. Try ':set display mei' for text output.",
        )),
    };

    let elapsed = t0.elapsed();
    debug!(
        terminal_support = support.description(),
        dark_mode = dark_mode,
        elapsed_us = elapsed.as_micros();
        "PNG display completed"
    );

    result
}

/// Display image using iTerm2 inline image protocol.
/// Protocol: ESC ] 1337 ; File = [args] : base64_data BEL
fn display_iterm2_inline(png_bytes: &[u8], dark_mode: bool) -> ReplResult<()> {
    use base64::Engine;
    use std::io::Write;

    // Optionally invert for dark mode
    let final_bytes = if dark_mode {
        invert_png_colors(png_bytes)?
    } else {
        png_bytes.to_vec()
    };

    let encoded = base64::engine::general_purpose::STANDARD.encode(&final_bytes);

    // iTerm2 inline image escape sequence
    let mut stdout = std::io::stdout().lock();
    writeln!(
        stdout,
        "\x1b]1337;File=inline=1;preserveAspectRatio=1:{}\x07",
        encoded
    )
    .map_err(|e| ReplError::render(format!("Failed to write iTerm2 sequence: {}", e)))?;
    stdout
        .flush()
        .map_err(|e| ReplError::render(format!("Failed to flush: {}", e)))?;

    Ok(())
}

/// Display image using Kitty graphics protocol.
/// Protocol: ESC _ G [args] ; base64_chunk ESC \
fn display_kitty_inline(png_bytes: &[u8], dark_mode: bool) -> ReplResult<()> {
    use base64::Engine;
    use std::io::Write;

    // Optionally invert for dark mode
    let final_bytes = if dark_mode {
        invert_png_colors(png_bytes)?
    } else {
        png_bytes.to_vec()
    };

    let encoded = base64::engine::general_purpose::STANDARD.encode(&final_bytes);
    let mut stdout = std::io::stdout().lock();

    // Kitty protocol sends data in chunks
    let chunk_size = 4096;
    let chunks: Vec<&str> = encoded
        .as_bytes()
        .chunks(chunk_size)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let is_last = i == chunks.len() - 1;
        if i == 0 {
            // First chunk: a=T (transmit and display), f=100 (PNG), m=1 (more data) or m=0 (last)
            write!(
                stdout,
                "\x1b_Ga=T,f=100,m={};{}\x1b\\",
                if is_last { 0 } else { 1 },
                chunk
            )
            .map_err(|e| ReplError::render(format!("Failed to write Kitty sequence: {}", e)))?;
        } else {
            // Continuation chunks
            write!(
                stdout,
                "\x1b_Gm={};{}\x1b\\",
                if is_last { 0 } else { 1 },
                chunk
            )
            .map_err(|e| ReplError::render(format!("Failed to write Kitty sequence: {}", e)))?;
        }
    }

    writeln!(stdout).map_err(|e| ReplError::render(format!("Failed to write newline: {}", e)))?;
    stdout
        .flush()
        .map_err(|e| ReplError::render(format!("Failed to flush: {}", e)))?;

    Ok(())
}

/// Detect if the terminal is using a dark background.
fn is_dark_mode() -> bool {
    // Check COLORFGBG (format: "fg;bg" where higher bg values = lighter)
    if let Ok(colorfgbg) = std::env::var("COLORFGBG") {
        if let Some(bg) = colorfgbg.split(';').nth(1) {
            if let Ok(bg_val) = bg.parse::<u32>() {
                // Values 0-6 are typically dark, 7+ are light
                return bg_val < 7;
            }
        }
    }

    // Check for common dark mode indicators
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("dark") {
            return true;
        }
    }

    // Check macOS appearance
    #[cfg(target_os = "macos")]
    if let Ok(output) = std::process::Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output()
    {
        if output.status.success() {
            let style = String::from_utf8_lossy(&output.stdout);
            if style.trim().eq_ignore_ascii_case("dark") {
                return true;
            }
        }
    }

    // Default to light mode (most common for code editors)
    false
}

/// Get the terminal width in columns, with a sensible default.
fn get_terminal_width() -> u32 {
    // Use crossterm to get actual terminal size
    if let Ok((width, _)) = crossterm::terminal::size() {
        // Leave a small margin
        (width as u32).saturating_sub(2).max(40)
    } else {
        // Default fallback
        120
    }
}

/// Invert colors in a PNG image for dark mode display.
/// Inverts RGB values while preserving alpha.
fn invert_png_colors(png_bytes: &[u8]) -> ReplResult<Vec<u8>> {
    use image::ImageEncoder;
    use image::codecs::png::{CompressionType, FilterType, PngEncoder};

    let img = image::load_from_memory(png_bytes)
        .map_err(|e| ReplError::render(format!("Failed to decode PNG for inversion: {}", e)))?;

    let mut rgba = img.to_rgba8();

    // Invert RGB, keep alpha
    for pixel in rgba.pixels_mut() {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
        // pixel[3] (alpha) stays the same
    }

    // Re-encode as PNG with fast compression (speed over size)
    let mut output = Vec::with_capacity(png_bytes.len());
    let encoder =
        PngEncoder::new_with_quality(&mut output, CompressionType::Fast, FilterType::NoFilter);
    encoder
        .write_image(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| ReplError::render(format!("Failed to re-encode PNG: {}", e)))?;

    Ok(output)
}

/// Quarter block characters for 2x2 pixel patterns.
/// Index by: (top_left << 3) | (top_right << 2) | (bottom_left << 1) | bottom_right
/// Where 1 = foreground color, 0 = background color
const QUARTER_BLOCKS: [char; 16] = [
    ' ', // 0000 - all background
    '▗', // 0001 - bottom right
    '▖', // 0010 - bottom left
    '▄', // 0011 - bottom half
    '▝', // 0100 - top right
    '▐', // 0101 - right half
    '▞', // 0110 - diagonal (TL-BR empty)
    '▟', // 0111 - all except top left
    '▘', // 1000 - top left
    '▚', // 1001 - diagonal (TR-BL empty)
    '▌', // 1010 - left half
    '▙', // 1011 - all except top right
    '▀', // 1100 - top half
    '▜', // 1101 - all except bottom left
    '▛', // 1110 - all except bottom right
    '█', // 1111 - all foreground (full block)
];

/// Display image using Unicode quarter block characters with 24-bit ANSI colors.
///
/// Each character cell represents a 2x2 grid of pixels. We find the best
/// 2 colors (foreground/background) and pattern for each cell.
/// This gives 2x the horizontal resolution compared to half blocks.
fn display_block_characters(png_bytes: &[u8], _max_width: u32, dark_mode: bool) -> ReplResult<()> {
    use image::GenericImageView;
    use std::fmt::Write as FmtWrite;
    use std::io::Write;

    // Decode the PNG
    let img = image::load_from_memory(png_bytes)
        .map_err(|e| ReplError::render(format!("Failed to decode PNG: {}", e)))?;

    // Convert transparent pixels to solid colors and optionally invert for dark mode
    let img = {
        let mut rgba = img.to_rgba8();
        for pixel in rgba.pixels_mut() {
            // Replace transparent/semi-transparent pixels with white (paper color)
            if pixel[3] < 128 {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = 255;
            }
            // Invert colors for dark mode
            if dark_mode {
                pixel[0] = 255 - pixel[0];
                pixel[1] = 255 - pixel[1];
                pixel[2] = 255 - pixel[2];
            }
        }
        image::DynamicImage::ImageRgba8(rgba)
    };

    let (orig_w, orig_h) = img.dimensions();

    // Get actual terminal width in characters
    let term_width = get_terminal_width();

    // With quarter blocks, each char represents 2x2 pixels
    // So we need 2x the pixels horizontally compared to terminal width
    let pixel_width = (term_width * 2).min(orig_w);
    let scale = pixel_width as f32 / orig_w as f32;
    let pixel_height = (orig_h as f32 * scale).ceil() as u32;
    // Round to multiples of 2 for quarter-block rendering
    let pixel_width = if pixel_width % 2 == 0 {
        pixel_width
    } else {
        pixel_width + 1
    };
    let pixel_height = if pixel_height % 2 == 0 {
        pixel_height
    } else {
        pixel_height + 1
    };

    // Resize using Lanczos3 for sharper downscaling
    let img = img.resize_exact(
        pixel_width,
        pixel_height,
        image::imageops::FilterType::Lanczos3,
    );
    let (w, h) = img.dimensions();

    debug!(
        term_width = term_width,
        orig_w = orig_w,
        orig_h = orig_h,
        scaled_w = w,
        scaled_h = h,
        char_cols = w / 2,
        char_rows = h / 2;
        "Quarter block rendering"
    );

    // Pre-allocate output buffer
    let char_cols = w / 2;
    let char_rows = h / 2;
    let estimated_size = (char_cols as usize * char_rows as usize) * 40;
    let mut output = String::with_capacity(estimated_size);

    // Render using quarter block characters
    for cy in 0..char_rows {
        for cx in 0..char_cols {
            let px = cx * 2;
            let py = cy * 2;

            // Get the 4 pixels for this character cell
            let tl = img.get_pixel(px, py);
            let tr = img.get_pixel(px + 1, py);
            let bl = img.get_pixel(px, py + 1);
            let br = img.get_pixel(px + 1, py + 1);

            // Find the two most distinct colors using simple clustering
            let pixels = [tl, tr, bl, br];
            let (fg, bg, pattern) = find_best_two_colors(&pixels);

            // Output ANSI escape + character
            let _ = write!(
                output,
                "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}",
                fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, QUARTER_BLOCKS[pattern as usize]
            );
        }
        output.push_str("\x1b[0m\n"); // Reset colors and newline
    }

    // Write all at once
    let mut stdout = std::io::stdout().lock();
    stdout
        .write_all(output.as_bytes())
        .map_err(|e| ReplError::render(format!("Failed to write block characters: {}", e)))?;
    stdout
        .flush()
        .map_err(|e| ReplError::render(format!("Failed to flush: {}", e)))?;

    Ok(())
}

/// Find the best two colors to represent a 2x2 block of pixels.
/// Returns (foreground_rgb, background_rgb, pattern_index).
fn find_best_two_colors(pixels: &[image::Rgba<u8>; 4]) -> ((u8, u8, u8), (u8, u8, u8), u8) {
    // Convert to RGB tuples for easier handling
    let colors: [(u8, u8, u8); 4] = [
        (pixels[0][0], pixels[0][1], pixels[0][2]),
        (pixels[1][0], pixels[1][1], pixels[1][2]),
        (pixels[2][0], pixels[2][1], pixels[2][2]),
        (pixels[3][0], pixels[3][1], pixels[3][2]),
    ];

    // Simple approach: find the two most different colors
    // Then assign each pixel to the nearest one
    let mut max_dist = 0u32;
    let mut c1_idx = 0;
    let mut c2_idx = 1;

    for i in 0..4 {
        for j in (i + 1)..4 {
            let dist = color_distance(colors[i], colors[j]);
            if dist > max_dist {
                max_dist = dist;
                c1_idx = i;
                c2_idx = j;
            }
        }
    }

    let fg = colors[c1_idx];
    let bg = colors[c2_idx];

    // Build pattern: for each pixel, 1 if closer to fg, 0 if closer to bg
    let mut pattern: u8 = 0;
    for (i, &color) in colors.iter().enumerate() {
        let dist_to_fg = color_distance(color, fg);
        let dist_to_bg = color_distance(color, bg);
        if dist_to_fg <= dist_to_bg {
            pattern |= 1 << (3 - i); // top-left is bit 3, bottom-right is bit 0
        }
    }

    (fg, bg, pattern)
}

/// Calculate squared Euclidean distance between two RGB colors.
#[inline]
fn color_distance(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> u32 {
    let dr = (c1.0 as i32 - c2.0 as i32).unsigned_abs();
    let dg = (c1.1 as i32 - c2.1 as i32).unsigned_abs();
    let db = (c1.2 as i32 - c2.2 as i32).unsigned_abs();
    dr * dr + dg * dg + db * db
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
    use std::time::Instant;

    let t0 = Instant::now();

    let xml = musicxml::emit(score)
        .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

    let t1 = Instant::now();

    let mut guard = get_or_init_toolkit()?;
    let toolkit = guard.as_mut().unwrap();

    toolkit
        .load_data(&xml)
        .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

    let t2 = Instant::now();

    let result = toolkit
        .render(Mei)
        .map_err(|e| ReplError::render(format!("Failed to render MEI: {}", e)))?;

    let t3 = Instant::now();

    debug!(
        emit_us = (t1 - t0).as_micros(),
        load_us = (t2 - t1).as_micros(),
        render_us = (t3 - t2).as_micros(),
        total_us = (t3 - t0).as_micros();
        "MEI render completed"
    );

    Ok(result)
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
    use std::time::Instant;

    let t0 = Instant::now();

    let xml = musicxml::emit(score)
        .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

    let t1 = Instant::now();

    let mut guard = get_or_init_toolkit()?;
    let toolkit = guard.as_mut().unwrap();

    toolkit
        .load_data(&xml)
        .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

    let t2 = Instant::now();

    let result = toolkit
        .render(Midi)
        .map_err(|e| ReplError::render(format!("Failed to render MIDI: {}", e)))?;

    let t3 = Instant::now();

    debug!(
        emit_us = (t1 - t0).as_micros(),
        load_us = (t2 - t1).as_micros(),
        render_us = (t3 - t2).as_micros(),
        total_us = (t3 - t0).as_micros();
        "MIDI render completed"
    );

    Ok(result)
}

/// Render and display a score as PNG in the terminal.
pub fn display_as_png(
    score: &ScorePartwise,
    options: &RenderOptions,
    use_colors: bool,
) -> Option<String> {
    match render_png_cached(score, options) {
        Ok((png_bytes, page_count, dark_mode)) => {
            // Display the image
            if let Err(e) = display_png_in_terminal(&png_bytes, options, dark_mode) {
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
fn render_png_cached(
    score: &ScorePartwise,
    options: &RenderOptions,
) -> ReplResult<(Vec<u8>, u32, bool)> {
    use std::time::Instant;

    let t0 = Instant::now();

    let xml = musicxml::emit(score)
        .map_err(|e| ReplError::render(format!("Failed to emit MusicXML: {}", e)))?;

    let t1 = Instant::now();

    let mut guard = get_or_init_toolkit()?;
    let toolkit = guard.as_mut().unwrap();

    let t2 = Instant::now();

    // Configure options BEFORE loading (verovio requirement)
    let verovio_options = VerovioOptions::builder()
        .page_width(options.width.max(1200))
        .adjust_page_height(true)
        .scale(100)
        .build();

    toolkit
        .set_options(&verovio_options)
        .map_err(|e| ReplError::render(format!("Failed to set options: {}", e)))?;

    let t3 = Instant::now();

    toolkit
        .load_data(&xml)
        .map_err(|e| ReplError::render(format!("Failed to load score: {}", e)))?;

    let t4 = Instant::now();

    let page_count = toolkit.page_count();

    // Use dark mode from options, or auto-detect if not set
    let dark_mode = options.dark_mode.unwrap_or_else(is_dark_mode);

    // Use 2x scale for good quality while maintaining reasonable performance
    // Skip white_background() to get transparency
    let png_bytes = toolkit
        .render(Png::page(options.page).scale(2.0))
        .map_err(|e| ReplError::render(format!("Failed to render PNG: {}", e)))?;

    let t5 = Instant::now();

    debug!(
        emit_us = (t1 - t0).as_micros(),
        lock_us = (t2 - t1).as_micros(),
        options_us = (t3 - t2).as_micros(),
        load_us = (t4 - t3).as_micros(),
        render_us = (t5 - t4).as_micros(),
        total_us = (t5 - t0).as_micros(),
        png_size_bytes = png_bytes.len();
        "PNG render completed"
    );

    Ok((png_bytes, page_count, dark_mode))
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
        assert!(!TerminalSupport::BlockChars.description().is_empty());
        assert!(!TerminalSupport::None.description().is_empty());
    }
}
