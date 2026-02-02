//! S-expression parser using nom.
//!
//! This module provides a nom-based parser for converting S-expression
//! text into the untyped [`Sexpr`] AST.
//!
//! # Supported Syntax
//!
//! - **Symbols**: Unquoted identifiers like `foo`, `note`, `C4`
//! - **Keywords**: Colon-prefixed identifiers like `:step`, `:octave`
//! - **Strings**: Double-quoted text with escape sequences: `"hello world"`
//! - **Numbers**: Integers and floating-point: `42`, `-3.14`
//! - **Booleans**: `#t`, `#f`, `true`, `false`, `nil`
//! - **Lists**: Parenthesized sequences: `(note :pitch C4)`
//! - **Comments**: Semicolon to end of line: `; this is a comment`
//!
//! # Examples
//!
//! ```
//! use fermata::sexpr::parser::parse;
//!
//! let sexpr = parse("(note :pitch C4)").unwrap();
//! assert!(sexpr.is_list());
//! ```

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{escaped, tag, take_while, take_while1},
    character::complete::{char, multispace0, none_of, one_of},
    combinator::{map, opt, recognize, value},
    multi::many0,
    sequence::{delimited, pair, preceded},
};

use super::ast::Sexpr;
use super::error::{ParseError, ParseResult};

/// Parse a complete S-expression from a string.
///
/// This parses a single S-expression from the input. Any trailing content
/// (other than whitespace or comments) will cause an error.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::parser::parse;
///
/// let sexpr = parse("(note :pitch C4)").unwrap();
/// assert!(sexpr.is_list());
///
/// // Trailing content causes an error
/// assert!(parse("(a) (b)").is_err());
/// ```
///
/// # Errors
///
/// Returns [`ParseError`] if the input contains invalid syntax.
pub fn parse(input: &str) -> ParseResult<Sexpr> {
    let (remaining, sexpr) =
        preceded(skip_ws_and_comments, sexpr)
            .parse(input)
            .map_err(|e| match e {
                nom::Err::Incomplete(_) => ParseError::UnexpectedEof,
                nom::Err::Error(e) | nom::Err::Failure(e) => ParseError::Nom(format!("{:?}", e)),
            })?;

    // Check for trailing content (allow whitespace/comments)
    let (remaining, _) = skip_ws_and_comments(remaining).map_err(|_| ParseError::UnexpectedEof)?;

    if !remaining.is_empty() {
        return Err(ParseError::TrailingContent(
            remaining[..remaining.len().min(20)].to_string(),
        ));
    }

    Ok(sexpr)
}

/// Parse multiple S-expressions from a string.
///
/// This parses zero or more S-expressions from the input, returning them
/// as a vector.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::parser::parse_all;
///
/// let sexprs = parse_all("(a) (b) (c)").unwrap();
/// assert_eq!(sexprs.len(), 3);
///
/// let empty = parse_all("").unwrap();
/// assert!(empty.is_empty());
/// ```
///
/// # Errors
///
/// Returns [`ParseError`] if any expression contains invalid syntax.
pub fn parse_all(input: &str) -> ParseResult<Vec<Sexpr>> {
    let (remaining, sexprs) = many0(preceded(skip_ws_and_comments, sexpr))
        .parse(input)
        .map_err(|e| match e {
            nom::Err::Incomplete(_) => ParseError::UnexpectedEof,
            nom::Err::Error(e) | nom::Err::Failure(e) => ParseError::Nom(format!("{:?}", e)),
        })?;

    // Check for trailing content
    let (remaining, _) = skip_ws_and_comments(remaining).map_err(|_| ParseError::UnexpectedEof)?;

    if !remaining.is_empty() {
        return Err(ParseError::TrailingContent(
            remaining[..remaining.len().min(20)].to_string(),
        ));
    }

    Ok(sexprs)
}

// === Internal Parsers ===

/// Parse a single S-expression.
fn sexpr(input: &str) -> IResult<&str, Sexpr> {
    preceded(
        skip_ws_and_comments,
        alt((boolean, nil, string_literal, number, keyword, symbol, list)),
    )
    .parse(input)
}

/// Skip whitespace and comments.
fn skip_ws_and_comments(input: &str) -> IResult<&str, ()> {
    let mut remaining = input;
    loop {
        // Skip whitespace
        let (rest, _) = multispace0.parse(remaining)?;
        remaining = rest;

        // Check for comment
        if remaining.starts_with(';') {
            // Skip to end of line
            remaining = remaining.find('\n').map_or("", |i| &remaining[i + 1..]);
        } else {
            break;
        }
    }
    Ok((remaining, ()))
}

/// Parse a symbol (non-keyword identifier).
fn symbol(input: &str) -> IResult<&str, Sexpr> {
    map(take_while1(is_symbol_char), |s: &str| {
        Sexpr::Symbol(s.to_string())
    })
    .parse(input)
}

/// Parse a keyword (colon-prefixed identifier).
fn keyword(input: &str) -> IResult<&str, Sexpr> {
    map(
        preceded(char(':'), take_while1(is_keyword_char)),
        |s: &str| Sexpr::Keyword(s.to_string()),
    )
    .parse(input)
}

/// Check if a character is valid in a symbol.
fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            '-' | '_'
                | '+'
                | '*'
                | '/'
                | '='
                | '<'
                | '>'
                | '!'
                | '?'
                | '.'
                | '#'
                | '@'
                | '$'
                | '%'
                | '^'
                | '&'
                | '~'
        )
}

/// Check if a character is valid in a keyword (after the colon).
fn is_keyword_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '-' | '_')
}

/// Parse a boolean value.
fn boolean(input: &str) -> IResult<&str, Sexpr> {
    alt((
        value(Sexpr::Bool(true), tag("#t")),
        value(Sexpr::Bool(false), tag("#f")),
    ))
    .parse(input)
}

/// Parse nil.
fn nil(input: &str) -> IResult<&str, Sexpr> {
    // Only match "nil" if not followed by symbol chars (to avoid matching "nilly")
    let (rest, _) = tag("nil").parse(input)?;
    if rest.chars().next().is_none_or(|c| !is_symbol_char(c)) {
        Ok((rest, Sexpr::Nil))
    } else {
        // It's actually a symbol starting with "nil"
        symbol(input)
    }
}

/// Parse a number (integer or float).
fn number(input: &str) -> IResult<&str, Sexpr> {
    let (rest, num_str) = recognize(pair(
        opt(char('-')),
        pair(
            take_while1(|c: char| c.is_ascii_digit()),
            opt(pair(char('.'), take_while(|c: char| c.is_ascii_digit()))),
        ),
    ))
    .parse(input)?;

    // Don't consume if followed by symbol chars (like "123abc")
    if rest
        .chars()
        .next()
        .is_some_and(|c| is_symbol_char(c) && !c.is_ascii_digit() && c != '.')
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )));
    }

    if num_str.contains('.') {
        let f: f64 = num_str.parse().map_err(|_| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Float))
        })?;
        Ok((rest, Sexpr::Float(f)))
    } else {
        let i: i64 = num_str.parse().map_err(|_| {
            nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
        })?;
        Ok((rest, Sexpr::Integer(i)))
    }
}

/// Parse a string literal.
fn string_literal(input: &str) -> IResult<&str, Sexpr> {
    // Handle empty string specially, then fall back to escaped content
    alt((
        // Empty string
        map(tag("\"\""), |_| Sexpr::String(String::new())),
        // Non-empty string with possible escapes
        map(
            delimited(
                char('"'),
                escaped(none_of("\"\\"), '\\', one_of("\"\\nrt")),
                char('"'),
            ),
            |s: &str| Sexpr::String(unescape_string(s)),
        ),
    ))
    .parse(input)
}

/// Unescape a string (process escape sequences).
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Parse a list.
fn list(input: &str) -> IResult<&str, Sexpr> {
    map(
        delimited(
            char('('),
            many0(sexpr),
            preceded(skip_ws_and_comments, char(')')),
        ),
        Sexpr::List,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Symbol Tests ===

    #[test]
    fn test_parse_symbol_simple() {
        let result = parse("foo").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo".to_string()));
    }

    #[test]
    fn test_parse_symbol_with_hyphen() {
        let result = parse("foo-bar").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo-bar".to_string()));
    }

    #[test]
    fn test_parse_symbol_with_underscore() {
        let result = parse("foo_bar").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo_bar".to_string()));
    }

    #[test]
    fn test_parse_symbol_alphanumeric() {
        let result = parse("C4").unwrap();
        assert_eq!(result, Sexpr::Symbol("C4".to_string()));
    }

    #[test]
    fn test_parse_symbol_special_chars() {
        let result = parse("foo+bar").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo+bar".to_string()));
    }

    // === Keyword Tests ===

    #[test]
    fn test_parse_keyword_simple() {
        let result = parse(":step").unwrap();
        assert_eq!(result, Sexpr::Keyword("step".to_string()));
    }

    #[test]
    fn test_parse_keyword_with_hyphen() {
        let result = parse(":default-x").unwrap();
        assert_eq!(result, Sexpr::Keyword("default-x".to_string()));
    }

    #[test]
    fn test_parse_keyword_with_underscore() {
        let result = parse(":some_key").unwrap();
        assert_eq!(result, Sexpr::Keyword("some_key".to_string()));
    }

    // === Number Tests ===

    #[test]
    fn test_parse_integer_positive() {
        let result = parse("42").unwrap();
        assert_eq!(result, Sexpr::Integer(42));
    }

    #[test]
    fn test_parse_integer_negative() {
        let result = parse("-123").unwrap();
        assert_eq!(result, Sexpr::Integer(-123));
    }

    #[test]
    fn test_parse_integer_zero() {
        let result = parse("0").unwrap();
        assert_eq!(result, Sexpr::Integer(0));
    }

    #[test]
    fn test_parse_float_positive() {
        let result = parse("3.14").unwrap();
        assert_eq!(result, Sexpr::Float(3.14));
    }

    #[test]
    fn test_parse_float_negative() {
        let result = parse("-1.5").unwrap();
        assert_eq!(result, Sexpr::Float(-1.5));
    }

    #[test]
    fn test_parse_float_zero() {
        let result = parse("0.0").unwrap();
        assert_eq!(result, Sexpr::Float(0.0));
    }

    #[test]
    fn test_parse_float_trailing_zeros() {
        let result = parse("1.50").unwrap();
        assert_eq!(result, Sexpr::Float(1.5));
    }

    // === Boolean Tests ===

    #[test]
    fn test_parse_boolean_true() {
        let result = parse("#t").unwrap();
        assert_eq!(result, Sexpr::Bool(true));
    }

    #[test]
    fn test_parse_boolean_false() {
        let result = parse("#f").unwrap();
        assert_eq!(result, Sexpr::Bool(false));
    }

    // === Nil Tests ===

    #[test]
    fn test_parse_nil() {
        let result = parse("nil").unwrap();
        assert_eq!(result, Sexpr::Nil);
    }

    #[test]
    fn test_parse_nil_like_symbol() {
        // "nilly" should parse as a symbol, not nil + "ly"
        let result = parse("nilly").unwrap();
        assert_eq!(result, Sexpr::Symbol("nilly".to_string()));
    }

    // === String Tests ===

    #[test]
    fn test_parse_string_simple() {
        let result = parse(r#""hello""#).unwrap();
        assert_eq!(result, Sexpr::String("hello".to_string()));
    }

    #[test]
    fn test_parse_string_with_spaces() {
        let result = parse(r#""hello world""#).unwrap();
        assert_eq!(result, Sexpr::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_string_empty() {
        let result = parse(r#""""#).unwrap();
        assert_eq!(result, Sexpr::String(String::new()));
    }

    #[test]
    fn test_parse_string_escape_newline() {
        let result = parse(r#""line\nbreak""#).unwrap();
        assert_eq!(result, Sexpr::String("line\nbreak".to_string()));
    }

    #[test]
    fn test_parse_string_escape_tab() {
        let result = parse(r#""tab\there""#).unwrap();
        assert_eq!(result, Sexpr::String("tab\there".to_string()));
    }

    #[test]
    fn test_parse_string_escape_quote() {
        let result = parse(r#""say \"hi\"""#).unwrap();
        assert_eq!(result, Sexpr::String("say \"hi\"".to_string()));
    }

    #[test]
    fn test_parse_string_escape_backslash() {
        let result = parse(r#""path\\to\\file""#).unwrap();
        assert_eq!(result, Sexpr::String("path\\to\\file".to_string()));
    }

    // === List Tests ===

    #[test]
    fn test_parse_list_empty() {
        let result = parse("()").unwrap();
        assert_eq!(result, Sexpr::List(vec![]));
    }

    #[test]
    fn test_parse_list_single_symbol() {
        let result = parse("(foo)").unwrap();
        assert_eq!(result, Sexpr::List(vec![Sexpr::Symbol("foo".to_string())]));
    }

    #[test]
    fn test_parse_list_multiple_symbols() {
        let result = parse("(foo bar baz)").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("foo".to_string()),
                Sexpr::Symbol("bar".to_string()),
                Sexpr::Symbol("baz".to_string()),
            ])
        );
    }

    #[test]
    fn test_parse_list_mixed() {
        let result = parse("(note :pitch C4)").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("note".to_string()),
                Sexpr::Keyword("pitch".to_string()),
                Sexpr::Symbol("C4".to_string()),
            ])
        );
    }

    #[test]
    fn test_parse_list_nested() {
        let result = parse("(outer (inner))").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("outer".to_string()),
                Sexpr::List(vec![Sexpr::Symbol("inner".to_string())]),
            ])
        );
    }

    #[test]
    fn test_parse_list_deeply_nested() {
        let result = parse("(a (b (c d)))").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("a".to_string()),
                Sexpr::List(vec![
                    Sexpr::Symbol("b".to_string()),
                    Sexpr::List(vec![
                        Sexpr::Symbol("c".to_string()),
                        Sexpr::Symbol("d".to_string()),
                    ]),
                ]),
            ])
        );
    }

    #[test]
    fn test_parse_list_with_numbers() {
        let result = parse("(pos 10 -20.5)").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("pos".to_string()),
                Sexpr::Integer(10),
                Sexpr::Float(-20.5),
            ])
        );
    }

    // === Whitespace Tests ===

    #[test]
    fn test_parse_leading_whitespace() {
        let result = parse("   foo").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo".to_string()));
    }

    #[test]
    fn test_parse_trailing_whitespace() {
        let result = parse("foo   ").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo".to_string()));
    }

    #[test]
    fn test_parse_list_with_extra_whitespace() {
        let result = parse("(  foo   bar  )").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("foo".to_string()),
                Sexpr::Symbol("bar".to_string()),
            ])
        );
    }

    #[test]
    fn test_parse_list_with_newlines() {
        let result = parse("(\n  foo\n  bar\n)").unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("foo".to_string()),
                Sexpr::Symbol("bar".to_string()),
            ])
        );
    }

    // === Comment Tests ===

    #[test]
    fn test_parse_with_comment_before() {
        let result = parse("; comment\nfoo").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo".to_string()));
    }

    #[test]
    fn test_parse_with_comment_after() {
        let result = parse("foo ; comment").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo".to_string()));
    }

    #[test]
    fn test_parse_list_with_comments() {
        let result = parse(
            "(foo ; first element
              bar ; second element
             )",
        )
        .unwrap();
        assert_eq!(
            result,
            Sexpr::List(vec![
                Sexpr::Symbol("foo".to_string()),
                Sexpr::Symbol("bar".to_string()),
            ])
        );
    }

    #[test]
    fn test_parse_multiple_comments() {
        let result = parse("; line 1\n; line 2\nfoo").unwrap();
        assert_eq!(result, Sexpr::Symbol("foo".to_string()));
    }

    // === parse_all Tests ===

    #[test]
    fn test_parse_all_empty() {
        let result = parse_all("").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_all_single() {
        let result = parse_all("(foo)").unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_parse_all_multiple() {
        let result = parse_all("(a) (b) (c)").unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_all_with_comments() {
        let result = parse_all("; header\n(a)\n; middle\n(b)").unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_parse_all_whitespace_only() {
        let result = parse_all("   \n   ").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_all_comments_only() {
        let result = parse_all("; just comments\n; more comments").unwrap();
        assert!(result.is_empty());
    }

    // === Error Tests ===

    #[test]
    fn test_parse_trailing_content() {
        let err = parse("(a) (b)").unwrap_err();
        assert!(matches!(err, ParseError::TrailingContent(_)));
    }

    #[test]
    fn test_parse_unclosed_list() {
        let err = parse("(foo bar").unwrap_err();
        assert!(matches!(err, ParseError::Nom(_)));
    }

    #[test]
    fn test_parse_unclosed_string() {
        let err = parse(r#""unterminated"#).unwrap_err();
        assert!(matches!(err, ParseError::Nom(_)));
    }

    #[test]
    fn test_parse_unexpected_close_paren() {
        // Just a close paren should fail
        let err = parse(")").unwrap_err();
        assert!(matches!(err, ParseError::Nom(_)));
    }

    // === Complex Tests ===

    #[test]
    fn test_parse_score_example() {
        let input = r#"
            (score-partwise :version "4.0"
              (part-list
                (score-part :id "P1"
                  (part-name "Piano")))
              (part :id "P1"
                (measure :number "1")))
        "#;
        let result = parse(input).unwrap();
        assert!(result.is_list());
        let list = result.as_list().unwrap();
        assert!(list[0].is_symbol("score-partwise"));
    }

    #[test]
    fn test_parse_note_example() {
        let input = r#"
            (note
              :pitch (pitch :step C :octave 4)
              :duration 1
              :type quarter)
        "#;
        let result = parse(input).unwrap();
        assert!(result.is_list());
    }

    // === Round Trip Tests ===

    #[test]
    fn test_round_trip_symbol() {
        let input = "foo-bar";
        let parsed = parse(input).unwrap();
        let printed = format!("{:?}", parsed);
        assert!(printed.contains("foo-bar"));
    }

    #[test]
    fn test_round_trip_keyword() {
        let input = ":default-x";
        let parsed = parse(input).unwrap();
        assert_eq!(parsed, Sexpr::Keyword("default-x".to_string()));
    }

    #[test]
    fn test_round_trip_list() {
        let input = "(note :pitch (pitch :step C))";
        let parsed = parse(input).unwrap();
        assert!(parsed.is_list());
        let list = parsed.as_list().unwrap();
        assert_eq!(list.len(), 3);
    }
}
