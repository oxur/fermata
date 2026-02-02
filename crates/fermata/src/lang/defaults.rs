//! Default value inference for Fermata compilation.
//!
//! When optional fields are not specified, these defaults are used.

use super::ast::{PitchAlter, PitchStep};
use crate::ir::duration::NoteTypeValue;

/// Default divisions per quarter note
pub const DEFAULT_DIVISIONS: u32 = 960;

/// Default voice number
pub const DEFAULT_VOICE: &str = "1";

/// Default staff number
pub const DEFAULT_STAFF: u32 = 1;

/// Default MusicXML version
pub const DEFAULT_MUSICXML_VERSION: &str = "4.0";

/// Calculate divisions value for a duration
pub fn duration_to_divisions(note_type: NoteTypeValue, dots: u8) -> u32 {
    let base = match note_type {
        NoteTypeValue::Maxima => DEFAULT_DIVISIONS * 32,
        NoteTypeValue::Long => DEFAULT_DIVISIONS * 16,
        NoteTypeValue::Breve => DEFAULT_DIVISIONS * 8,
        NoteTypeValue::Whole => DEFAULT_DIVISIONS * 4,
        NoteTypeValue::Half => DEFAULT_DIVISIONS * 2,
        NoteTypeValue::Quarter => DEFAULT_DIVISIONS,
        NoteTypeValue::Eighth => DEFAULT_DIVISIONS / 2,
        NoteTypeValue::N16th => DEFAULT_DIVISIONS / 4,
        NoteTypeValue::N32nd => DEFAULT_DIVISIONS / 8,
        NoteTypeValue::N64th => DEFAULT_DIVISIONS / 16,
        NoteTypeValue::N128th => DEFAULT_DIVISIONS / 32,
        NoteTypeValue::N256th => DEFAULT_DIVISIONS / 64,
        NoteTypeValue::N512th => DEFAULT_DIVISIONS / 128,
        NoteTypeValue::N1024th => DEFAULT_DIVISIONS / 256,
    };

    // Apply dots: each dot adds half of the previous value
    let mut total = base;
    let mut dot_value = base / 2;
    for _ in 0..dots {
        total += dot_value;
        dot_value /= 2;
    }
    total
}

/// Compute the fifths value for a key signature
///
/// Handles both natural keys (C major, G major) and keys with accidentals
/// (F# major, Bb minor, etc.)
pub fn key_to_fifths(root: PitchStep, root_alter: Option<PitchAlter>, mode: &str) -> i8 {
    // Base fifths for natural major keys
    let natural_major_fifths: i8 = match root {
        PitchStep::C => 0,
        PitchStep::G => 1,
        PitchStep::D => 2,
        PitchStep::A => 3,
        PitchStep::E => 4,
        PitchStep::B => 5,
        PitchStep::F => -1,
    };

    // Adjust for accidentals on the root
    let alter_offset: i8 = match root_alter {
        Some(PitchAlter::Sharp) => 7, // F# major = 6 = F major (-1) + 7
        Some(PitchAlter::Flat) => -7, // Gb major = -6 = G major (1) - 7
        Some(PitchAlter::DoubleSharp) => 14,
        Some(PitchAlter::DoubleFlat) => -14,
        _ => 0,
    };

    let major_fifths = natural_major_fifths + alter_offset;

    // Adjust for mode (relative to major)
    let mode_offset: i8 = match mode.to_lowercase().as_str() {
        "major" | "ionian" => 0,
        "minor" | "aeolian" => -3, // A minor = C major, so -3 from relative major
        "dorian" => -2,
        "phrygian" => -4,
        "lydian" => 1,
        "mixolydian" => -1,
        "locrian" => -5,
        _ => 0,
    };

    major_fifths + mode_offset
}

/// Generate a part ID from index
pub fn generate_part_id(index: usize) -> String {
    format!("P{}", index + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Constants Tests ===

    #[test]
    fn test_default_divisions_value() {
        assert_eq!(DEFAULT_DIVISIONS, 960);
    }

    #[test]
    fn test_default_voice_value() {
        assert_eq!(DEFAULT_VOICE, "1");
    }

    #[test]
    fn test_default_staff_value() {
        assert_eq!(DEFAULT_STAFF, 1);
    }

    #[test]
    fn test_default_musicxml_version() {
        assert_eq!(DEFAULT_MUSICXML_VERSION, "4.0");
    }

    // === duration_to_divisions Tests ===

    #[test]
    fn test_duration_to_divisions_quarter() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Quarter, 0),
            DEFAULT_DIVISIONS
        );
    }

    #[test]
    fn test_duration_to_divisions_half() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Half, 0),
            DEFAULT_DIVISIONS * 2
        );
    }

    #[test]
    fn test_duration_to_divisions_whole() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Whole, 0),
            DEFAULT_DIVISIONS * 4
        );
    }

    #[test]
    fn test_duration_to_divisions_eighth() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Eighth, 0),
            DEFAULT_DIVISIONS / 2
        );
    }

    #[test]
    fn test_duration_to_divisions_sixteenth() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N16th, 0),
            DEFAULT_DIVISIONS / 4
        );
    }

    #[test]
    fn test_duration_to_divisions_breve() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Breve, 0),
            DEFAULT_DIVISIONS * 8
        );
    }

    #[test]
    fn test_duration_to_divisions_long() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Long, 0),
            DEFAULT_DIVISIONS * 16
        );
    }

    #[test]
    fn test_duration_to_divisions_maxima() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::Maxima, 0),
            DEFAULT_DIVISIONS * 32
        );
    }

    #[test]
    fn test_duration_to_divisions_32nd() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N32nd, 0),
            DEFAULT_DIVISIONS / 8
        );
    }

    #[test]
    fn test_duration_to_divisions_64th() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N64th, 0),
            DEFAULT_DIVISIONS / 16
        );
    }

    #[test]
    fn test_duration_to_divisions_128th() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N128th, 0),
            DEFAULT_DIVISIONS / 32
        );
    }

    #[test]
    fn test_duration_to_divisions_256th() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N256th, 0),
            DEFAULT_DIVISIONS / 64
        );
    }

    #[test]
    fn test_duration_to_divisions_512th() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N512th, 0),
            DEFAULT_DIVISIONS / 128
        );
    }

    #[test]
    fn test_duration_to_divisions_1024th() {
        assert_eq!(
            duration_to_divisions(NoteTypeValue::N1024th, 0),
            DEFAULT_DIVISIONS / 256
        );
    }

    #[test]
    fn test_duration_to_divisions_dotted_quarter() {
        // Dotted quarter = 1.5 * quarter
        let dotted_quarter = duration_to_divisions(NoteTypeValue::Quarter, 1);
        assert_eq!(dotted_quarter, DEFAULT_DIVISIONS + DEFAULT_DIVISIONS / 2);
    }

    #[test]
    fn test_duration_to_divisions_double_dotted_quarter() {
        // Double-dotted quarter = 1.75 * quarter
        let double_dotted = duration_to_divisions(NoteTypeValue::Quarter, 2);
        let expected = DEFAULT_DIVISIONS + DEFAULT_DIVISIONS / 2 + DEFAULT_DIVISIONS / 4;
        assert_eq!(double_dotted, expected);
    }

    #[test]
    fn test_duration_to_divisions_triple_dotted_quarter() {
        let triple_dotted = duration_to_divisions(NoteTypeValue::Quarter, 3);
        let expected = DEFAULT_DIVISIONS
            + DEFAULT_DIVISIONS / 2
            + DEFAULT_DIVISIONS / 4
            + DEFAULT_DIVISIONS / 8;
        assert_eq!(triple_dotted, expected);
    }

    #[test]
    fn test_duration_to_divisions_dotted_half() {
        let dotted_half = duration_to_divisions(NoteTypeValue::Half, 1);
        assert_eq!(dotted_half, DEFAULT_DIVISIONS * 2 + DEFAULT_DIVISIONS);
    }

    #[test]
    fn test_duration_to_divisions_dotted_eighth() {
        let dotted_eighth = duration_to_divisions(NoteTypeValue::Eighth, 1);
        assert_eq!(dotted_eighth, DEFAULT_DIVISIONS / 2 + DEFAULT_DIVISIONS / 4);
    }

    // === key_to_fifths Tests ===

    #[test]
    fn test_key_to_fifths_c_major() {
        assert_eq!(key_to_fifths(PitchStep::C, None, "major"), 0);
    }

    #[test]
    fn test_key_to_fifths_g_major() {
        assert_eq!(key_to_fifths(PitchStep::G, None, "major"), 1);
    }

    #[test]
    fn test_key_to_fifths_d_major() {
        assert_eq!(key_to_fifths(PitchStep::D, None, "major"), 2);
    }

    #[test]
    fn test_key_to_fifths_a_major() {
        assert_eq!(key_to_fifths(PitchStep::A, None, "major"), 3);
    }

    #[test]
    fn test_key_to_fifths_e_major() {
        assert_eq!(key_to_fifths(PitchStep::E, None, "major"), 4);
    }

    #[test]
    fn test_key_to_fifths_b_major() {
        assert_eq!(key_to_fifths(PitchStep::B, None, "major"), 5);
    }

    #[test]
    fn test_key_to_fifths_f_major() {
        assert_eq!(key_to_fifths(PitchStep::F, None, "major"), -1);
    }

    #[test]
    fn test_key_to_fifths_f_sharp_major() {
        // F# major = 6 fifths
        assert_eq!(
            key_to_fifths(PitchStep::F, Some(PitchAlter::Sharp), "major"),
            6
        );
    }

    #[test]
    fn test_key_to_fifths_b_flat_major() {
        // Bb major = -2 fifths
        assert_eq!(
            key_to_fifths(PitchStep::B, Some(PitchAlter::Flat), "major"),
            -2
        );
    }

    #[test]
    fn test_key_to_fifths_g_flat_major() {
        // Gb major = -6 fifths
        assert_eq!(
            key_to_fifths(PitchStep::G, Some(PitchAlter::Flat), "major"),
            -6
        );
    }

    #[test]
    fn test_key_to_fifths_c_sharp_major() {
        // C# major = 7 fifths
        assert_eq!(
            key_to_fifths(PitchStep::C, Some(PitchAlter::Sharp), "major"),
            7
        );
    }

    #[test]
    fn test_key_to_fifths_a_minor() {
        // A minor = C major - 3 = -3 + 3 = 0
        // Actually: A is 3 fifths as major, minus 3 for minor = 0
        assert_eq!(key_to_fifths(PitchStep::A, None, "minor"), 0);
    }

    #[test]
    fn test_key_to_fifths_e_minor() {
        // E minor = G major = 1 fifth
        // E major = 4, minor = 4 - 3 = 1
        assert_eq!(key_to_fifths(PitchStep::E, None, "minor"), 1);
    }

    #[test]
    fn test_key_to_fifths_d_minor() {
        // D minor = F major = -1 fifth
        // D major = 2, minor = 2 - 3 = -1
        assert_eq!(key_to_fifths(PitchStep::D, None, "minor"), -1);
    }

    #[test]
    fn test_key_to_fifths_d_dorian() {
        // D dorian = same key sig as C major = 0 fifths
        // D major = 2, dorian = 2 - 2 = 0
        assert_eq!(key_to_fifths(PitchStep::D, None, "dorian"), 0);
    }

    #[test]
    fn test_key_to_fifths_e_phrygian() {
        // E phrygian = same key sig as C major = 0 fifths
        // E major = 4, phrygian = 4 - 4 = 0
        assert_eq!(key_to_fifths(PitchStep::E, None, "phrygian"), 0);
    }

    #[test]
    fn test_key_to_fifths_f_lydian() {
        // F lydian = same key sig as C major = 0 fifths
        // F major = -1, lydian = -1 + 1 = 0
        assert_eq!(key_to_fifths(PitchStep::F, None, "lydian"), 0);
    }

    #[test]
    fn test_key_to_fifths_g_mixolydian() {
        // G mixolydian = same key sig as C major = 0 fifths
        // G major = 1, mixolydian = 1 - 1 = 0
        assert_eq!(key_to_fifths(PitchStep::G, None, "mixolydian"), 0);
    }

    #[test]
    fn test_key_to_fifths_b_locrian() {
        // B locrian = same key sig as C major = 0 fifths
        // B major = 5, locrian = 5 - 5 = 0
        assert_eq!(key_to_fifths(PitchStep::B, None, "locrian"), 0);
    }

    #[test]
    fn test_key_to_fifths_c_ionian() {
        // C ionian = C major = 0 fifths
        assert_eq!(key_to_fifths(PitchStep::C, None, "ionian"), 0);
    }

    #[test]
    fn test_key_to_fifths_a_aeolian() {
        // A aeolian = A minor = 0 fifths
        assert_eq!(key_to_fifths(PitchStep::A, None, "aeolian"), 0);
    }

    #[test]
    fn test_key_to_fifths_case_insensitive() {
        assert_eq!(key_to_fifths(PitchStep::C, None, "MAJOR"), 0);
        assert_eq!(key_to_fifths(PitchStep::C, None, "Major"), 0);
        assert_eq!(key_to_fifths(PitchStep::A, None, "MINOR"), 0);
    }

    #[test]
    fn test_key_to_fifths_unknown_mode() {
        // Unknown mode defaults to 0 offset (treated as major)
        assert_eq!(key_to_fifths(PitchStep::C, None, "unknown"), 0);
    }

    #[test]
    fn test_key_to_fifths_double_sharp() {
        // C## major would be +14 from C major
        assert_eq!(
            key_to_fifths(PitchStep::C, Some(PitchAlter::DoubleSharp), "major"),
            14
        );
    }

    #[test]
    fn test_key_to_fifths_double_flat() {
        // Cbb major would be -14 from C major
        assert_eq!(
            key_to_fifths(PitchStep::C, Some(PitchAlter::DoubleFlat), "major"),
            -14
        );
    }

    #[test]
    fn test_key_to_fifths_natural_alter() {
        // Natural has no effect
        assert_eq!(
            key_to_fifths(PitchStep::C, Some(PitchAlter::Natural), "major"),
            0
        );
    }

    // === generate_part_id Tests ===

    #[test]
    fn test_generate_part_id_first() {
        assert_eq!(generate_part_id(0), "P1");
    }

    #[test]
    fn test_generate_part_id_second() {
        assert_eq!(generate_part_id(1), "P2");
    }

    #[test]
    fn test_generate_part_id_tenth() {
        assert_eq!(generate_part_id(9), "P10");
    }

    #[test]
    fn test_generate_part_id_large() {
        assert_eq!(generate_part_id(99), "P100");
    }
}
