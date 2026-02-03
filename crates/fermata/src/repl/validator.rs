//! Input validator for multi-line detection.

use reedline::{ValidationResult, Validator};

use super::input::needs_continuation;

/// Validator that detects incomplete S-expressions.
///
/// When the input has unbalanced parentheses, it returns `Incomplete`
/// to allow the user to continue typing on the next line.
#[derive(Debug, Default, Clone)]
pub struct FermataValidator;

impl FermataValidator {
    /// Create a new validator.
    pub fn new() -> Self {
        Self
    }
}

impl Validator for FermataValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        if needs_continuation(line) {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to check if result is Complete
    fn is_complete(result: ValidationResult) -> bool {
        matches!(result, ValidationResult::Complete)
    }

    /// Helper to check if result is Incomplete
    fn is_incomplete(result: ValidationResult) -> bool {
        matches!(result, ValidationResult::Incomplete)
    }

    #[test]
    fn test_validator_new() {
        let _validator = FermataValidator::new();
    }

    #[test]
    fn test_validator_default() {
        let _validator = FermataValidator::default();
    }

    #[test]
    fn test_validator_complete_empty() {
        let validator = FermataValidator::new();
        assert!(is_complete(validator.validate("")));
    }

    #[test]
    fn test_validator_complete_balanced() {
        let validator = FermataValidator::new();
        assert!(is_complete(validator.validate("(score)")));
    }

    #[test]
    fn test_validator_incomplete_open_paren() {
        let validator = FermataValidator::new();
        assert!(is_incomplete(validator.validate("(")));
    }

    #[test]
    fn test_validator_incomplete_expression() {
        let validator = FermataValidator::new();
        assert!(is_incomplete(validator.validate("(score :title")));
    }

    #[test]
    fn test_validator_complete_with_string() {
        let validator = FermataValidator::new();
        assert!(is_complete(validator.validate("(score :title \"test\")")));
    }

    #[test]
    fn test_validator_string_with_parens() {
        let validator = FermataValidator::new();
        // Parens inside string should not affect balance
        assert!(is_complete(validator.validate("(score :title \"(test)\")")));
    }

    #[test]
    fn test_validator_clone() {
        let validator = FermataValidator::new();
        let cloned = validator.clone();
        // Both should behave the same
        assert!(is_incomplete(validator.validate("(")));
        assert!(is_incomplete(cloned.validate("(")));
    }

    #[test]
    fn test_validator_debug() {
        let validator = FermataValidator::new();
        let debug_str = format!("{:?}", validator);
        assert!(debug_str.contains("FermataValidator"));
    }
}
