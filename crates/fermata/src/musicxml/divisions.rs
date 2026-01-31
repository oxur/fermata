//! Duration and divisions calculations for MusicXML emission.
//!
//! The `divisions` element in MusicXML defines how many units equal one quarter note.
//! This module provides utilities for calculating durations based on note types,
//! dots, and time modifications (tuplets).

use crate::ir::{NoteTypeValue, TimeModification};

/// Standard divisions value that handles all common subdivisions.
///
/// 960 = LCM(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16)
/// This covers up to 16th note triplets and quintuplets.
pub const STANDARD_DIVISIONS: u32 = 960;

/// Convert a note type to its duration in divisions.
///
/// The duration is calculated relative to the given divisions per quarter note.
///
/// # Arguments
///
/// * `note_type` - The note type value (quarter, eighth, etc.)
/// * `divisions` - Divisions per quarter note
///
/// # Returns
///
/// The duration in divisions
///
/// # Examples
///
/// ```
/// use fermata::ir::NoteTypeValue;
/// use fermata::musicxml::note_type_to_divisions;
///
/// // With 960 divisions per quarter:
/// assert_eq!(note_type_to_divisions(&NoteTypeValue::Quarter, 960), 960);
/// assert_eq!(note_type_to_divisions(&NoteTypeValue::Half, 960), 1920);
/// assert_eq!(note_type_to_divisions(&NoteTypeValue::Eighth, 960), 480);
/// ```
pub fn note_type_to_divisions(note_type: &NoteTypeValue, divisions: u32) -> u32 {
    match note_type {
        NoteTypeValue::Maxima => divisions * 32,
        NoteTypeValue::Long => divisions * 16,
        NoteTypeValue::Breve => divisions * 8,
        NoteTypeValue::Whole => divisions * 4,
        NoteTypeValue::Half => divisions * 2,
        NoteTypeValue::Quarter => divisions,
        NoteTypeValue::Eighth => divisions / 2,
        NoteTypeValue::N16th => divisions / 4,
        NoteTypeValue::N32nd => divisions / 8,
        NoteTypeValue::N64th => divisions / 16,
        NoteTypeValue::N128th => divisions / 32,
        NoteTypeValue::N256th => divisions / 64,
        NoteTypeValue::N512th => divisions / 128,
        NoteTypeValue::N1024th => divisions / 256,
    }
}

/// Apply dots to a base duration.
///
/// Each dot adds half of the previous duration:
/// - 1 dot: base + base/2 = 1.5 * base
/// - 2 dots: base + base/2 + base/4 = 1.75 * base
/// - 3 dots: base + base/2 + base/4 + base/8 = 1.875 * base
///
/// # Arguments
///
/// * `base_duration` - The base duration before dots
/// * `num_dots` - Number of dots to apply
///
/// # Returns
///
/// The total duration after applying dots
///
/// # Examples
///
/// ```
/// use fermata::musicxml::apply_dots;
///
/// // Dotted quarter (960 base) = 960 + 480 = 1440
/// assert_eq!(apply_dots(960, 1), 1440);
///
/// // Double-dotted quarter = 960 + 480 + 240 = 1680
/// assert_eq!(apply_dots(960, 2), 1680);
/// ```
pub fn apply_dots(base_duration: u32, num_dots: usize) -> u32 {
    let mut total = base_duration;
    let mut addition = base_duration;
    for _ in 0..num_dots {
        addition /= 2;
        total += addition;
    }
    total
}

/// Apply time modification (tuplet ratio) to a duration.
///
/// For a tuplet, `actual_notes` notes are played in the time of `normal_notes`.
/// For example, in a triplet (3:2), 3 notes are played in the time of 2 notes.
/// So each note's duration = base_duration * normal / actual.
///
/// # Arguments
///
/// * `duration` - The base duration before time modification
/// * `time_mod` - The time modification (tuplet) specification
///
/// # Returns
///
/// The duration after applying time modification
///
/// # Examples
///
/// ```
/// use fermata::ir::TimeModification;
/// use fermata::musicxml::apply_time_modification;
///
/// // Triplet quarter (960 * 2/3 = 640)
/// let triplet = TimeModification {
///     actual_notes: 3,
///     normal_notes: 2,
///     normal_type: None,
///     normal_dots: 0,
/// };
/// assert_eq!(apply_time_modification(960, &triplet), 640);
/// ```
pub fn apply_time_modification(duration: u32, time_mod: &TimeModification) -> u32 {
    // actual_notes in the time of normal_notes
    // e.g., 3:2 triplet means 3 notes in time of 2
    // So each note's duration = base_duration * normal / actual
    (duration * time_mod.normal_notes) / time_mod.actual_notes
}

/// Calculate the complete duration for a note.
///
/// This function combines note type, dots, and time modification to calculate
/// the final duration in divisions.
///
/// # Arguments
///
/// * `note_type` - The note type value
/// * `dots` - Number of dots
/// * `time_modification` - Optional time modification for tuplets
/// * `divisions` - Divisions per quarter note
///
/// # Returns
///
/// The complete duration in divisions
///
/// # Examples
///
/// ```
/// use fermata::ir::{NoteTypeValue, TimeModification};
/// use fermata::musicxml::calculate_duration;
///
/// // Plain quarter note
/// assert_eq!(
///     calculate_duration(&NoteTypeValue::Quarter, 0, None, 960),
///     960
/// );
///
/// // Dotted quarter note
/// assert_eq!(
///     calculate_duration(&NoteTypeValue::Quarter, 1, None, 960),
///     1440
/// );
///
/// // Triplet quarter note
/// let triplet = TimeModification {
///     actual_notes: 3,
///     normal_notes: 2,
///     normal_type: None,
///     normal_dots: 0,
/// };
/// assert_eq!(
///     calculate_duration(&NoteTypeValue::Quarter, 0, Some(&triplet), 960),
///     640
/// );
/// ```
pub fn calculate_duration(
    note_type: &NoteTypeValue,
    dots: usize,
    time_modification: Option<&TimeModification>,
    divisions: u32,
) -> u32 {
    let base = note_type_to_divisions(note_type, divisions);
    let dotted = apply_dots(base, dots);
    match time_modification {
        Some(tm) => apply_time_modification(dotted, tm),
        None => dotted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === STANDARD_DIVISIONS Tests ===

    #[test]
    fn test_standard_divisions_value() {
        assert_eq!(STANDARD_DIVISIONS, 960);
    }

    #[test]
    fn test_standard_divisions_divisibility() {
        // Should be divisible by common subdivisions
        assert_eq!(STANDARD_DIVISIONS % 2, 0); // For half notes
        assert_eq!(STANDARD_DIVISIONS % 3, 0); // For triplets
        assert_eq!(STANDARD_DIVISIONS % 4, 0); // For 16th notes
        assert_eq!(STANDARD_DIVISIONS % 5, 0); // For quintuplets
        assert_eq!(STANDARD_DIVISIONS % 6, 0); // For sextuplets
        assert_eq!(STANDARD_DIVISIONS % 8, 0); // For 32nd notes
        assert_eq!(STANDARD_DIVISIONS % 12, 0); // For compound meters
        assert_eq!(STANDARD_DIVISIONS % 16, 0); // For 64th notes
    }

    // === note_type_to_divisions Tests ===

    #[test]
    fn test_note_type_quarter() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Quarter, STANDARD_DIVISIONS),
            960
        );
    }

    #[test]
    fn test_note_type_half() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Half, STANDARD_DIVISIONS),
            1920
        );
    }

    #[test]
    fn test_note_type_whole() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Whole, STANDARD_DIVISIONS),
            3840
        );
    }

    #[test]
    fn test_note_type_eighth() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Eighth, STANDARD_DIVISIONS),
            480
        );
    }

    #[test]
    fn test_note_type_16th() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N16th, STANDARD_DIVISIONS),
            240
        );
    }

    #[test]
    fn test_note_type_32nd() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N32nd, STANDARD_DIVISIONS),
            120
        );
    }

    #[test]
    fn test_note_type_64th() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N64th, STANDARD_DIVISIONS),
            60
        );
    }

    #[test]
    fn test_note_type_128th() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N128th, STANDARD_DIVISIONS),
            30
        );
    }

    #[test]
    fn test_note_type_256th() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N256th, STANDARD_DIVISIONS),
            15
        );
    }

    #[test]
    fn test_note_type_breve() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Breve, STANDARD_DIVISIONS),
            7680
        );
    }

    #[test]
    fn test_note_type_long() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Long, STANDARD_DIVISIONS),
            15360
        );
    }

    #[test]
    fn test_note_type_maxima() {
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::Maxima, STANDARD_DIVISIONS),
            30720
        );
    }

    #[test]
    fn test_note_type_512th() {
        // 960 / 128 = 7 (integer division, some precision loss)
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N512th, STANDARD_DIVISIONS),
            7
        );
    }

    #[test]
    fn test_note_type_1024th() {
        // 960 / 256 = 3 (integer division, some precision loss)
        assert_eq!(
            note_type_to_divisions(&NoteTypeValue::N1024th, STANDARD_DIVISIONS),
            3
        );
    }

    #[test]
    fn test_note_type_with_different_divisions() {
        // With 4 divisions per quarter (common in simple MusicXML)
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Quarter, 4), 4);
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Half, 4), 8);
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Whole, 4), 16);
        assert_eq!(note_type_to_divisions(&NoteTypeValue::Eighth, 4), 2);
        assert_eq!(note_type_to_divisions(&NoteTypeValue::N16th, 4), 1);
    }

    // === apply_dots Tests ===

    #[test]
    fn test_apply_dots_zero() {
        assert_eq!(apply_dots(960, 0), 960);
    }

    #[test]
    fn test_apply_dots_single() {
        // Dotted quarter: 960 + 480 = 1440
        assert_eq!(apply_dots(960, 1), 1440);
    }

    #[test]
    fn test_apply_dots_double() {
        // Double-dotted quarter: 960 + 480 + 240 = 1680
        assert_eq!(apply_dots(960, 2), 1680);
    }

    #[test]
    fn test_apply_dots_triple() {
        // Triple-dotted quarter: 960 + 480 + 240 + 120 = 1800
        assert_eq!(apply_dots(960, 3), 1800);
    }

    #[test]
    fn test_apply_dots_half_note() {
        // Dotted half: 1920 + 960 = 2880
        assert_eq!(apply_dots(1920, 1), 2880);
    }

    #[test]
    fn test_apply_dots_eighth_note() {
        // Dotted eighth: 480 + 240 = 720
        assert_eq!(apply_dots(480, 1), 720);
    }

    // === apply_time_modification Tests ===

    #[test]
    fn test_apply_time_modification_triplet() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        // Triplet quarter: 960 * 2 / 3 = 640
        assert_eq!(apply_time_modification(960, &tm), 640);
    }

    #[test]
    fn test_apply_time_modification_duplet() {
        let tm = TimeModification {
            actual_notes: 2,
            normal_notes: 3,
            normal_type: None,
            normal_dots: 0,
        };
        // Duplet: 960 * 3 / 2 = 1440
        assert_eq!(apply_time_modification(960, &tm), 1440);
    }

    #[test]
    fn test_apply_time_modification_quintuplet() {
        let tm = TimeModification {
            actual_notes: 5,
            normal_notes: 4,
            normal_type: None,
            normal_dots: 0,
        };
        // Quintuplet: 960 * 4 / 5 = 768
        assert_eq!(apply_time_modification(960, &tm), 768);
    }

    #[test]
    fn test_apply_time_modification_septuplet() {
        let tm = TimeModification {
            actual_notes: 7,
            normal_notes: 4,
            normal_type: None,
            normal_dots: 0,
        };
        // Septuplet: 960 * 4 / 7 = 548 (integer division)
        assert_eq!(apply_time_modification(960, &tm), 548);
    }

    #[test]
    fn test_apply_time_modification_sextuplet() {
        let tm = TimeModification {
            actual_notes: 6,
            normal_notes: 4,
            normal_type: None,
            normal_dots: 0,
        };
        // Sextuplet: 960 * 4 / 6 = 640
        assert_eq!(apply_time_modification(960, &tm), 640);
    }

    // === calculate_duration Tests ===

    #[test]
    fn test_calculate_duration_plain_quarter() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::Quarter, 0, None, STANDARD_DIVISIONS),
            960
        );
    }

    #[test]
    fn test_calculate_duration_dotted_quarter() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::Quarter, 1, None, STANDARD_DIVISIONS),
            1440
        );
    }

    #[test]
    fn test_calculate_duration_double_dotted_quarter() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::Quarter, 2, None, STANDARD_DIVISIONS),
            1680
        );
    }

    #[test]
    fn test_calculate_duration_triplet_quarter() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        assert_eq!(
            calculate_duration(&NoteTypeValue::Quarter, 0, Some(&tm), STANDARD_DIVISIONS),
            640
        );
    }

    #[test]
    fn test_calculate_duration_dotted_triplet_quarter() {
        // Dotted triplet quarter: first apply dots (1440), then tuplet ratio
        // 1440 * 2 / 3 = 960
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        assert_eq!(
            calculate_duration(&NoteTypeValue::Quarter, 1, Some(&tm), STANDARD_DIVISIONS),
            960
        );
    }

    #[test]
    fn test_calculate_duration_half_note() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::Half, 0, None, STANDARD_DIVISIONS),
            1920
        );
    }

    #[test]
    fn test_calculate_duration_dotted_half() {
        // Dotted half: 1920 + 960 = 2880
        assert_eq!(
            calculate_duration(&NoteTypeValue::Half, 1, None, STANDARD_DIVISIONS),
            2880
        );
    }

    #[test]
    fn test_calculate_duration_eighth_note() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::Eighth, 0, None, STANDARD_DIVISIONS),
            480
        );
    }

    #[test]
    fn test_calculate_duration_triplet_eighth() {
        let tm = TimeModification {
            actual_notes: 3,
            normal_notes: 2,
            normal_type: None,
            normal_dots: 0,
        };
        // Triplet eighth: 480 * 2 / 3 = 320
        assert_eq!(
            calculate_duration(&NoteTypeValue::Eighth, 0, Some(&tm), STANDARD_DIVISIONS),
            320
        );
    }

    #[test]
    fn test_calculate_duration_whole_note() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::Whole, 0, None, STANDARD_DIVISIONS),
            3840
        );
    }

    #[test]
    fn test_calculate_duration_16th_note() {
        assert_eq!(
            calculate_duration(&NoteTypeValue::N16th, 0, None, STANDARD_DIVISIONS),
            240
        );
    }
}
