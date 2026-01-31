//! Voice navigation emission functions for MusicXML.
//!
//! This module handles the emission of backup and forward elements which are used
//! for multi-voice navigation in MusicXML.

use crate::ir::voice::{Backup, Forward};
use crate::musicxml::EmitError;
use crate::musicxml::writer::XmlWriter;

/// Emit a backup element.
pub(crate) fn emit_backup(w: &mut XmlWriter, backup: &Backup) -> Result<(), EmitError> {
    w.start_element("backup")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // duration (required)
    w.text_element("duration", &backup.duration.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // editorial elements (footnote, level) - skipped for now

    w.end_element("backup")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

/// Emit a forward element.
pub(crate) fn emit_forward(w: &mut XmlWriter, forward: &Forward) -> Result<(), EmitError> {
    w.start_element("forward")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // duration (required)
    w.text_element("duration", &forward.duration.to_string())
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;

    // editorial elements (footnote, level) - skipped for now

    // voice
    if let Some(ref voice) = forward.voice {
        w.text_element("voice", voice)
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    // staff
    if let Some(staff) = forward.staff {
        w.text_element("staff", &staff.to_string())
            .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    }

    w.end_element("forward")
        .map_err(|e| EmitError::XmlWrite(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::common::Editorial;

    #[test]
    fn test_emit_backup() {
        let mut w = XmlWriter::new();
        let backup = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };

        emit_backup(&mut w, &backup).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<backup>"));
        assert!(xml.contains("<duration>4</duration>"));
        assert!(xml.contains("</backup>"));
    }

    #[test]
    fn test_emit_forward_basic() {
        let mut w = XmlWriter::new();
        let forward = Forward {
            duration: 2,
            voice: None,
            staff: None,
            editorial: Editorial::default(),
        };

        emit_forward(&mut w, &forward).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<forward>"));
        assert!(xml.contains("<duration>2</duration>"));
        assert!(xml.contains("</forward>"));
    }

    #[test]
    fn test_emit_forward_with_voice_and_staff() {
        let mut w = XmlWriter::new();
        let forward = Forward {
            duration: 2,
            voice: Some("2".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };

        emit_forward(&mut w, &forward).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<duration>2</duration>"));
        assert!(xml.contains("<voice>2</voice>"));
        assert!(xml.contains("<staff>1</staff>"));
    }

    // =======================================================================
    // Additional tests for edge cases
    // =======================================================================

    #[test]
    fn test_emit_backup_large_duration() {
        let mut w = XmlWriter::new();
        let backup = Backup {
            duration: 65536,
            editorial: Editorial::default(),
        };

        emit_backup(&mut w, &backup).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<duration>65536</duration>"));
    }

    #[test]
    fn test_emit_backup_minimal_duration() {
        let mut w = XmlWriter::new();
        let backup = Backup {
            duration: 1,
            editorial: Editorial::default(),
        };

        emit_backup(&mut w, &backup).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<backup>"));
        assert!(xml.contains("<duration>1</duration>"));
        assert!(xml.contains("</backup>"));
    }

    #[test]
    fn test_emit_forward_with_voice_only() {
        let mut w = XmlWriter::new();
        let forward = Forward {
            duration: 8,
            voice: Some("3".to_string()),
            staff: None,
            editorial: Editorial::default(),
        };

        emit_forward(&mut w, &forward).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<duration>8</duration>"));
        assert!(xml.contains("<voice>3</voice>"));
        assert!(!xml.contains("<staff>"));
    }

    #[test]
    fn test_emit_forward_with_staff_only() {
        let mut w = XmlWriter::new();
        let forward = Forward {
            duration: 4,
            voice: None,
            staff: Some(2),
            editorial: Editorial::default(),
        };

        emit_forward(&mut w, &forward).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<duration>4</duration>"));
        assert!(!xml.contains("<voice>"));
        assert!(xml.contains("<staff>2</staff>"));
    }

    #[test]
    fn test_emit_forward_large_duration() {
        let mut w = XmlWriter::new();
        let forward = Forward {
            duration: 32768,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };

        emit_forward(&mut w, &forward).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<duration>32768</duration>"));
    }

    #[test]
    fn test_emit_forward_multiple_digit_voice() {
        let mut w = XmlWriter::new();
        let forward = Forward {
            duration: 4,
            voice: Some("12".to_string()),
            staff: Some(10),
            editorial: Editorial::default(),
        };

        emit_forward(&mut w, &forward).unwrap();
        let xml = w.into_string().unwrap();

        assert!(xml.contains("<voice>12</voice>"));
        assert!(xml.contains("<staff>10</staff>"));
    }
}
