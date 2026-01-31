//! Voice-related types: backup, forward.

use super::common::{Editorial, PositiveDivisions, StaffNumber, Voice};

/// Move backward in time within a measure (for multiple voices).
#[derive(Debug, Clone, PartialEq)]
pub struct Backup {
    /// Duration to move backward in divisions
    pub duration: PositiveDivisions,
    /// Editorial information
    pub editorial: Editorial,
}

/// Move forward in time within a measure.
#[derive(Debug, Clone, PartialEq)]
pub struct Forward {
    /// Duration to move forward in divisions
    pub duration: PositiveDivisions,
    /// Voice assignment
    pub voice: Option<Voice>,
    /// Staff number
    pub staff: Option<StaffNumber>,
    /// Editorial information
    pub editorial: Editorial,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Backup Tests ===

    #[test]
    fn test_backup_basic() {
        let backup = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };
        assert_eq!(backup.duration, 4);
    }

    #[test]
    fn test_backup_large_duration() {
        let backup = Backup {
            duration: 256,
            editorial: Editorial::default(),
        };
        assert_eq!(backup.duration, 256);
    }

    #[test]
    fn test_backup_clone() {
        let backup = Backup {
            duration: 8,
            editorial: Editorial::default(),
        };
        let cloned = backup.clone();
        assert_eq!(backup, cloned);
    }

    #[test]
    fn test_backup_equality() {
        let backup1 = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };
        let backup2 = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };
        assert_eq!(backup1, backup2);
    }

    #[test]
    fn test_backup_inequality() {
        let backup1 = Backup {
            duration: 4,
            editorial: Editorial::default(),
        };
        let backup2 = Backup {
            duration: 8,
            editorial: Editorial::default(),
        };
        assert_ne!(backup1, backup2);
    }

    #[test]
    fn test_backup_debug() {
        let backup = Backup {
            duration: 2,
            editorial: Editorial::default(),
        };
        let debug_str = format!("{:?}", backup);
        assert!(debug_str.contains("Backup"));
        assert!(debug_str.contains("duration"));
    }

    // === Forward Tests ===

    #[test]
    fn test_forward_basic() {
        let forward = Forward {
            duration: 4,
            voice: None,
            staff: None,
            editorial: Editorial::default(),
        };
        assert_eq!(forward.duration, 4);
        assert!(forward.voice.is_none());
        assert!(forward.staff.is_none());
    }

    #[test]
    fn test_forward_with_voice() {
        let forward = Forward {
            duration: 2,
            voice: Some("1".to_string()),
            staff: None,
            editorial: Editorial::default(),
        };
        assert_eq!(forward.voice, Some("1".to_string()));
    }

    #[test]
    fn test_forward_with_staff() {
        let forward = Forward {
            duration: 8,
            voice: None,
            staff: Some(2),
            editorial: Editorial::default(),
        };
        assert_eq!(forward.staff, Some(2));
    }

    #[test]
    fn test_forward_with_voice_and_staff() {
        let forward = Forward {
            duration: 16,
            voice: Some("2".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };
        assert_eq!(forward.voice, Some("2".to_string()));
        assert_eq!(forward.staff, Some(1));
    }

    #[test]
    fn test_forward_clone() {
        let forward = Forward {
            duration: 4,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };
        let cloned = forward.clone();
        assert_eq!(forward, cloned);
    }

    #[test]
    fn test_forward_equality() {
        let forward1 = Forward {
            duration: 4,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };
        let forward2 = Forward {
            duration: 4,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };
        assert_eq!(forward1, forward2);
    }

    #[test]
    fn test_forward_inequality_duration() {
        let forward1 = Forward {
            duration: 4,
            voice: None,
            staff: None,
            editorial: Editorial::default(),
        };
        let forward2 = Forward {
            duration: 8,
            voice: None,
            staff: None,
            editorial: Editorial::default(),
        };
        assert_ne!(forward1, forward2);
    }

    #[test]
    fn test_forward_inequality_voice() {
        let forward1 = Forward {
            duration: 4,
            voice: Some("1".to_string()),
            staff: None,
            editorial: Editorial::default(),
        };
        let forward2 = Forward {
            duration: 4,
            voice: Some("2".to_string()),
            staff: None,
            editorial: Editorial::default(),
        };
        assert_ne!(forward1, forward2);
    }

    #[test]
    fn test_forward_debug() {
        let forward = Forward {
            duration: 4,
            voice: Some("1".to_string()),
            staff: Some(1),
            editorial: Editorial::default(),
        };
        let debug_str = format!("{:?}", forward);
        assert!(debug_str.contains("Forward"));
        assert!(debug_str.contains("duration"));
        assert!(debug_str.contains("voice"));
        assert!(debug_str.contains("staff"));
    }
}
