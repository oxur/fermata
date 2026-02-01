//! Conversions between IR types and S-expressions.
//!
//! This module contains [`ToSexpr`](super::traits::ToSexpr) and
//! [`FromSexpr`](super::traits::FromSexpr) implementations for IR types.
//!
//! # Module Organization
//!
//! - [`common`] - Common types from `ir::common` (enums, Position, Font, etc.)
//! - [`pitch`] - Pitch-related types (Step, Pitch, Unpitched)
//! - [`duration`] - Duration-related types (NoteTypeValue, NoteType, Dot, TimeModification)
//! - [`beam`] - Beam-related types (BeamValue, Beam, Fan, StemValue, Stem, NoteheadValue, Notehead)
//! - [`note`] - Note-related types (Rest, FullNote, Tie, Grace, NoteContent, Accidental, Instrument, Note)
//! - [`attributes`] - Measure attributes (Clef, Key, Time, Transpose, Barline, etc.)
//! - [`direction`] - Direction types (Dynamics, Wedge, Metronome, Words, Pedal, etc.)
//!
//! # Helpers
//!
//! This module also provides helper functions for parsing keyword arguments
//! from S-expression lists:
//!
//! - [`find_kwarg`] - Find a keyword argument value
//! - [`require_kwarg`] - Find and parse a required keyword argument
//! - [`optional_kwarg`] - Find and parse an optional keyword argument
//! - [`parse_kwargs`] - Iterate over keyword-value pairs

pub mod attributes;
pub mod beam;
pub mod common;
pub mod direction;
pub mod duration;
pub mod note;
pub mod pitch;

use super::{ConvertError, ConvertResult, FromSexpr, Sexpr};

/// Find a keyword argument value in a list.
///
/// Given a list like `(foo :bar 1 :baz 2)`, this finds the value
/// following a specific keyword.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, convert::find_kwarg};
///
/// let list = vec![
///     Sexpr::symbol("note"),
///     Sexpr::keyword("duration"),
///     Sexpr::Integer(4),
/// ];
///
/// assert_eq!(find_kwarg(&list, "duration"), Some(&Sexpr::Integer(4)));
/// assert_eq!(find_kwarg(&list, "pitch"), None);
/// ```
pub fn find_kwarg<'a>(list: &'a [Sexpr], key: &str) -> Option<&'a Sexpr> {
    let mut iter = list.iter();
    while let Some(item) = iter.next() {
        if item.is_keyword(key) {
            return iter.next();
        }
    }
    None
}

/// Find and parse a required keyword argument.
///
/// Returns an error if the keyword is not found or cannot be parsed.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, convert::require_kwarg};
///
/// let list = vec![
///     Sexpr::symbol("note"),
///     Sexpr::keyword("duration"),
///     Sexpr::Integer(4),
/// ];
///
/// let duration: i32 = require_kwarg(&list, "duration").unwrap();
/// assert_eq!(duration, 4);
/// ```
///
/// # Errors
///
/// Returns [`ConvertError::MissingField`] if the keyword is not found.
pub fn require_kwarg<T: FromSexpr>(list: &[Sexpr], key: &'static str) -> ConvertResult<T> {
    find_kwarg(list, key)
        .ok_or(ConvertError::MissingField(key))
        .and_then(|v| T::from_sexpr(v).map_err(|e| e.with_context(format!("field ':{}'", key))))
}

/// Find and parse an optional keyword argument.
///
/// Returns `Ok(None)` if the keyword is not found.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, convert::optional_kwarg};
///
/// let list = vec![
///     Sexpr::symbol("note"),
///     Sexpr::keyword("duration"),
///     Sexpr::Integer(4),
/// ];
///
/// let duration: Option<i32> = optional_kwarg(&list, "duration").unwrap();
/// assert_eq!(duration, Some(4));
///
/// let pitch: Option<i32> = optional_kwarg(&list, "pitch").unwrap();
/// assert_eq!(pitch, None);
/// ```
pub fn optional_kwarg<T: FromSexpr>(list: &[Sexpr], key: &str) -> ConvertResult<Option<T>> {
    match find_kwarg(list, key) {
        Some(value) => T::from_sexpr(value)
            .map(Some)
            .map_err(|e| e.with_context(format!("field ':{}'", key))),
        None => Ok(None),
    }
}

/// Iterate over keyword-value pairs in a list.
///
/// This yields `(keyword_name, value)` pairs for all keywords in the list.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, convert::parse_kwargs};
///
/// let list = vec![
///     Sexpr::symbol("note"),
///     Sexpr::keyword("duration"),
///     Sexpr::Integer(4),
///     Sexpr::keyword("octave"),
///     Sexpr::Integer(5),
/// ];
///
/// let kwargs: Vec<_> = parse_kwargs(&list).collect();
/// assert_eq!(kwargs.len(), 2);
/// assert_eq!(kwargs[0].0, "duration");
/// assert_eq!(kwargs[1].0, "octave");
/// ```
pub fn parse_kwargs(list: &[Sexpr]) -> impl Iterator<Item = (&str, &Sexpr)> {
    list.iter().enumerate().filter_map(|(i, item)| {
        item.as_keyword()
            .and_then(|key| list.get(i + 1).map(|value| (key, value)))
    })
}

/// Check that a list has the expected head symbol.
///
/// # Errors
///
/// Returns [`ConvertError::ExpectedHead`] if the head doesn't match.
pub fn expect_head(list: &[Sexpr], expected: &'static str) -> ConvertResult<()> {
    match list.first() {
        Some(head) if head.is_symbol(expected) => Ok(()),
        Some(_) => Err(ConvertError::ExpectedHead(expected)),
        None => Err(ConvertError::ExpectedHead(expected)),
    }
}

/// Get the head symbol of a list, returning an error if not a symbol.
pub fn get_head(list: &[Sexpr]) -> ConvertResult<&str> {
    list.first()
        .and_then(|s| s.as_symbol())
        .ok_or(ConvertError::TypeMismatch {
            expected: "list with symbol head",
            found: "list without symbol head".to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_list() -> Vec<Sexpr> {
        vec![
            Sexpr::symbol("note"),
            Sexpr::keyword("duration"),
            Sexpr::Integer(4),
            Sexpr::keyword("octave"),
            Sexpr::Integer(5),
            Sexpr::keyword("name"),
            Sexpr::String("test".to_string()),
        ]
    }

    // === find_kwarg Tests ===

    #[test]
    fn test_find_kwarg_found() {
        let list = make_test_list();
        assert_eq!(find_kwarg(&list, "duration"), Some(&Sexpr::Integer(4)));
    }

    #[test]
    fn test_find_kwarg_found_string() {
        let list = make_test_list();
        assert_eq!(
            find_kwarg(&list, "name"),
            Some(&Sexpr::String("test".to_string()))
        );
    }

    #[test]
    fn test_find_kwarg_not_found() {
        let list = make_test_list();
        assert_eq!(find_kwarg(&list, "missing"), None);
    }

    #[test]
    fn test_find_kwarg_empty_list() {
        let list: Vec<Sexpr> = vec![];
        assert_eq!(find_kwarg(&list, "anything"), None);
    }

    #[test]
    fn test_find_kwarg_no_keywords() {
        let list = vec![Sexpr::symbol("foo"), Sexpr::symbol("bar")];
        assert_eq!(find_kwarg(&list, "foo"), None);
    }

    // === require_kwarg Tests ===

    #[test]
    fn test_require_kwarg_found() {
        let list = make_test_list();
        let result: i32 = require_kwarg(&list, "duration").unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_require_kwarg_not_found() {
        let list = make_test_list();
        let result: ConvertResult<i32> = require_kwarg(&list, "missing");
        assert!(matches!(result, Err(ConvertError::MissingField("missing"))));
    }

    #[test]
    fn test_require_kwarg_parse_error() {
        let list = make_test_list();
        let result: ConvertResult<i32> = require_kwarg(&list, "name"); // "test" can't be i32
        assert!(result.is_err());
    }

    // === optional_kwarg Tests ===

    #[test]
    fn test_optional_kwarg_found() {
        let list = make_test_list();
        let result: Option<i32> = optional_kwarg(&list, "duration").unwrap();
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_optional_kwarg_not_found() {
        let list = make_test_list();
        let result: Option<i32> = optional_kwarg(&list, "missing").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_optional_kwarg_parse_error() {
        let list = make_test_list();
        let result: ConvertResult<Option<i32>> = optional_kwarg(&list, "name");
        assert!(result.is_err());
    }

    // === parse_kwargs Tests ===

    #[test]
    fn test_parse_kwargs() {
        let list = make_test_list();
        let kwargs: Vec<_> = parse_kwargs(&list).collect();
        assert_eq!(kwargs.len(), 3);
        assert_eq!(kwargs[0].0, "duration");
        assert_eq!(kwargs[1].0, "octave");
        assert_eq!(kwargs[2].0, "name");
    }

    #[test]
    fn test_parse_kwargs_empty() {
        let list: Vec<Sexpr> = vec![];
        let kwargs: Vec<_> = parse_kwargs(&list).collect();
        assert!(kwargs.is_empty());
    }

    #[test]
    fn test_parse_kwargs_no_keywords() {
        let list = vec![Sexpr::symbol("foo"), Sexpr::symbol("bar")];
        let kwargs: Vec<_> = parse_kwargs(&list).collect();
        assert!(kwargs.is_empty());
    }

    // === expect_head Tests ===

    #[test]
    fn test_expect_head_match() {
        let list = make_test_list();
        assert!(expect_head(&list, "note").is_ok());
    }

    #[test]
    fn test_expect_head_mismatch() {
        let list = make_test_list();
        let result = expect_head(&list, "rest");
        assert!(matches!(result, Err(ConvertError::ExpectedHead("rest"))));
    }

    #[test]
    fn test_expect_head_empty() {
        let list: Vec<Sexpr> = vec![];
        let result = expect_head(&list, "note");
        assert!(matches!(result, Err(ConvertError::ExpectedHead("note"))));
    }

    // === get_head Tests ===

    #[test]
    fn test_get_head_symbol() {
        let list = make_test_list();
        assert_eq!(get_head(&list).unwrap(), "note");
    }

    #[test]
    fn test_get_head_not_symbol() {
        let list = vec![Sexpr::Integer(42)];
        assert!(get_head(&list).is_err());
    }

    #[test]
    fn test_get_head_empty() {
        let list: Vec<Sexpr> = vec![];
        assert!(get_head(&list).is_err());
    }
}
