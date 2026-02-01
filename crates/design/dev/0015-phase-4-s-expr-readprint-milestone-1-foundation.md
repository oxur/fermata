# Phase 4: S-expr Read/Print — Milestone 1: Foundation

> **For:** Claude Code (Opus) with Rust-SKILL.md agents
> **Scope:** S-expr infrastructure, traits, and common type serialization
> **Estimated Implementation Time:** 2-3 hours

---

## Overview

Phase 4 adds S-expression serialization to Fermata's IR types. This enables:

- **Pretty-printing:** IR → human-readable S-expr text
- **Reading:** S-expr text → validated IR

This milestone establishes the foundation: parser infrastructure, core traits, and common types.

---

## Architecture

### Module Structure

```
src/
├── ir/           # ✅ Existing
├── musicxml/     # ✅ Existing (emit.rs, parse.rs)
└── sexpr/        # 🆕 NEW
    ├── mod.rs       # Module exports
    ├── ast.rs       # Untyped S-expr AST
    ├── parser.rs    # Text → AST (nom)
    ├── printer.rs   # AST → Text (pretty-printing)
    ├── traits.rs    # ToSexpr, FromSexpr traits
    ├── error.rs     # Parse/convert errors
    └── convert/     # IR ↔ AST conversions
        ├── mod.rs
        ├── common.rs
        ├── pitch.rs
        ├── duration.rs
        ├── note.rs
        ├── beam.rs
        ├── attributes.rs
        ├── direction.rs
        ├── notation.rs
        ├── voice.rs
        ├── lyric.rs
        ├── measure.rs
        ├── part.rs
        └── score.rs
```

### Data Flow

```
S-expr Text ──parser──→ AST ──FromSexpr──→ IR (typed)
                                            │
                                            ▼
                         AST ←──ToSexpr──── IR
                          │
                          ▼
                    S-expr Text ←──printer
```

---

## Task 1: S-expr AST (`src/sexpr/ast.rs`)

Define the untyped S-expression AST.

### Types

```rust
/// An S-expression value
#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    /// Symbol: identifiers, keywords (`:foo`), numbers
    Symbol(String),

    /// String literal: "hello world"
    String(String),

    /// List: (foo bar baz)
    List(Vec<Sexpr>),
}
```

### Helper Methods

```rust
impl Sexpr {
    /// Create a symbol
    pub fn symbol(s: impl Into<String>) -> Self {
        Sexpr::Symbol(s.into())
    }

    /// Create a keyword (prefixed with `:`)
    pub fn keyword(s: impl AsRef<str>) -> Self {
        Sexpr::Symbol(format!(":{}", s.as_ref()))
    }

    /// Create a string literal
    pub fn string(s: impl Into<String>) -> Self {
        Sexpr::String(s.into())
    }

    /// Create a list
    pub fn list(items: impl IntoIterator<Item = Sexpr>) -> Self {
        Sexpr::List(items.into_iter().collect())
    }

    /// Check if this is a keyword (starts with `:`)
    pub fn is_keyword(&self) -> bool {
        matches!(self, Sexpr::Symbol(s) if s.starts_with(':'))
    }

    /// Get keyword name without the `:` prefix
    pub fn as_keyword(&self) -> Option<&str> {
        match self {
            Sexpr::Symbol(s) if s.starts_with(':') => Some(&s[1..]),
            _ => None,
        }
    }

    /// Check if this is a specific symbol
    pub fn is_symbol(&self, name: &str) -> bool {
        matches!(self, Sexpr::Symbol(s) if s == name)
    }

    /// Get as list
    pub fn as_list(&self) -> Option<&[Sexpr]> {
        match self {
            Sexpr::List(items) => Some(items),
            _ => None,
        }
    }

    /// Get as symbol string
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Sexpr::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Get as string literal
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Sexpr::String(s) => Some(s),
            _ => None,
        }
    }
}
```

### List Builder Pattern

```rust
/// Builder for constructing S-expr lists with keyword arguments
pub struct ListBuilder {
    items: Vec<Sexpr>,
}

impl ListBuilder {
    pub fn new(head: impl Into<String>) -> Self {
        Self {
            items: vec![Sexpr::symbol(head)],
        }
    }

    /// Add a positional argument
    pub fn arg(mut self, value: Sexpr) -> Self {
        self.items.push(value);
        self
    }

    /// Add a keyword argument pair (only if value is Some)
    pub fn kwarg_opt<T: ToSexpr>(mut self, key: &str, value: &Option<T>) -> Self {
        if let Some(v) = value {
            self.items.push(Sexpr::keyword(key));
            self.items.push(v.to_sexpr());
        }
        self
    }

    /// Add a keyword argument pair (always)
    pub fn kwarg<T: ToSexpr>(mut self, key: &str, value: &T) -> Self {
        self.items.push(Sexpr::keyword(key));
        self.items.push(value.to_sexpr());
        self
    }

    /// Add keyword with raw Sexpr value
    pub fn kwarg_raw(mut self, key: &str, value: Sexpr) -> Self {
        self.items.push(Sexpr::keyword(key));
        self.items.push(value);
        self
    }

    /// Add a list of items under a keyword (only if non-empty)
    pub fn kwarg_list<T: ToSexpr>(mut self, key: &str, items: &[T]) -> Self {
        if !items.is_empty() {
            self.items.push(Sexpr::keyword(key));
            self.items.push(Sexpr::list(items.iter().map(|i| i.to_sexpr())));
        }
        self
    }

    /// Add a boolean keyword (only if true)
    pub fn kwarg_bool(mut self, key: &str, value: bool) -> Self {
        if value {
            self.items.push(Sexpr::keyword(key));
            self.items.push(Sexpr::symbol("t"));
        }
        self
    }

    pub fn build(self) -> Sexpr {
        Sexpr::List(self.items)
    }
}
```

---

## Task 2: Error Types (`src/sexpr/error.rs`)

```rust
use thiserror::Error;

/// Errors that can occur during S-expr parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Unexpected character: {0}")]
    UnexpectedChar(char),

    #[error("Unclosed string literal")]
    UnclosedString,

    #[error("Unclosed list")]
    UnclosedList,

    #[error("Invalid escape sequence: \\{0}")]
    InvalidEscape(char),

    #[error("Parse error: {0}")]
    Nom(String),
}

/// Errors that can occur during IR conversion
#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("Expected {expected}, found {found}")]
    TypeMismatch {
        expected: &'static str,
        found: String,
    },

    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("Unknown keyword: {0}")]
    UnknownKeyword(String),

    #[error("Invalid value for {field}: {value}")]
    InvalidValue {
        field: &'static str,
        value: String,
    },

    #[error("Expected list with head '{0}'")]
    ExpectedHead(&'static str),

    #[error("Invalid enum variant: {0}")]
    InvalidVariant(String),

    #[error("{context}: {source}")]
    Context {
        context: String,
        #[source]
        source: Box<ConvertError>,
    },
}

impl ConvertError {
    pub fn type_mismatch(expected: &'static str, found: &Sexpr) -> Self {
        Self::TypeMismatch {
            expected,
            found: format!("{:?}", found),
        }
    }

    pub fn with_context(self, context: impl Into<String>) -> Self {
        Self::Context {
            context: context.into(),
            source: Box::new(self),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
pub type ConvertResult<T> = Result<T, ConvertError>;
```

---

## Task 3: Core Traits (`src/sexpr/traits.rs`)

```rust
use super::ast::Sexpr;
use super::error::ConvertResult;

/// Convert an IR type to an S-expression
pub trait ToSexpr {
    fn to_sexpr(&self) -> Sexpr;
}

/// Parse an S-expression into an IR type
pub trait FromSexpr: Sized {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self>;
}

// Implement for primitive types

impl ToSexpr for String {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::String(self.clone())
    }
}

impl FromSexpr for String {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr {
            Sexpr::String(s) => Ok(s.clone()),
            Sexpr::Symbol(s) => Ok(s.clone()),
            _ => Err(ConvertError::type_mismatch("string", sexpr)),
        }
    }
}

impl ToSexpr for bool {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(if *self { "t" } else { "nil" })
    }
}

impl FromSexpr for bool {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr {
            Sexpr::Symbol(s) if s == "t" || s == "true" => Ok(true),
            Sexpr::Symbol(s) if s == "nil" || s == "false" => Ok(false),
            _ => Err(ConvertError::type_mismatch("boolean (t/nil)", sexpr)),
        }
    }
}

// Numeric types
macro_rules! impl_numeric {
    ($($ty:ty),+) => {
        $(
            impl ToSexpr for $ty {
                fn to_sexpr(&self) -> Sexpr {
                    Sexpr::symbol(self.to_string())
                }
            }

            impl FromSexpr for $ty {
                fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
                    match sexpr {
                        Sexpr::Symbol(s) => s.parse()
                            .map_err(|_| ConvertError::InvalidValue {
                                field: stringify!($ty),
                                value: s.clone(),
                            }),
                        _ => Err(ConvertError::type_mismatch(stringify!($ty), sexpr)),
                    }
                }
            }
        )+
    };
}

impl_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

// Option<T>
impl<T: ToSexpr> ToSexpr for Option<T> {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            Some(v) => v.to_sexpr(),
            None => Sexpr::symbol("nil"),
        }
    }
}

// Vec<T>
impl<T: ToSexpr> ToSexpr for Vec<T> {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::list(self.iter().map(|item| item.to_sexpr()))
    }
}

impl<T: FromSexpr> FromSexpr for Vec<T> {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr {
            Sexpr::List(items) => items.iter().map(T::from_sexpr).collect(),
            _ => Err(ConvertError::type_mismatch("list", sexpr)),
        }
    }
}
```

---

## Task 4: S-expr Parser (`src/sexpr/parser.rs`)

Use `nom` for parsing S-expressions.

### Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
nom = "7"
```

### Parser Implementation

```rust
use nom::{
    IResult,
    branch::alt,
    bytes::complete::{escaped, tag, take_while1},
    character::complete::{char, multispace0, none_of, one_of},
    combinator::{map, recognize, value, cut},
    multi::many0,
    sequence::{delimited, preceded},
};

use super::ast::Sexpr;
use super::error::{ParseError, ParseResult};

/// Parse a complete S-expression from a string
pub fn parse(input: &str) -> ParseResult<Sexpr> {
    let (remaining, sexpr) = preceded(multispace0, sexpr)(input)
        .map_err(|e| ParseError::Nom(format!("{:?}", e)))?;

    // Check for trailing content (allow whitespace/comments)
    let remaining = remaining.trim();
    if !remaining.is_empty() && !remaining.starts_with(';') {
        return Err(ParseError::Nom(format!(
            "Unexpected trailing content: {}",
            &remaining[..remaining.len().min(20)]
        )));
    }

    Ok(sexpr)
}

/// Parse multiple S-expressions from a string
pub fn parse_all(input: &str) -> ParseResult<Vec<Sexpr>> {
    let (remaining, sexprs) = many0(preceded(skip_ws_and_comments, sexpr))(input)
        .map_err(|e| ParseError::Nom(format!("{:?}", e)))?;

    let remaining = remaining.trim();
    if !remaining.is_empty() {
        return Err(ParseError::Nom(format!(
            "Unexpected trailing content: {}",
            &remaining[..remaining.len().min(20)]
        )));
    }

    Ok(sexprs)
}

// Internal parsers

fn sexpr(input: &str) -> IResult<&str, Sexpr> {
    preceded(
        skip_ws_and_comments,
        alt((string_literal, list, symbol)),
    )(input)
}

fn skip_ws_and_comments(input: &str) -> IResult<&str, ()> {
    let mut remaining = input;
    loop {
        // Skip whitespace
        let (rest, _) = multispace0(remaining)?;
        remaining = rest;

        // Check for comment
        if remaining.starts_with(';') {
            // Skip to end of line
            remaining = remaining
                .find('\n')
                .map(|i| &remaining[i + 1..])
                .unwrap_or("");
        } else {
            break;
        }
    }
    Ok((remaining, ()))
}

fn symbol(input: &str) -> IResult<&str, Sexpr> {
    map(
        recognize(take_while1(is_symbol_char)),
        |s: &str| Sexpr::Symbol(s.to_string()),
    )(input)
}

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(c, ':' | '-' | '_' | '+' | '*' | '/' | '=' | '<' | '>' | '!' | '?' | '.' | '#')
}

fn string_literal(input: &str) -> IResult<&str, Sexpr> {
    map(
        delimited(
            char('"'),
            escaped(none_of("\"\\"), '\\', one_of("\"\\nrt")),
            char('"'),
        ),
        |s: &str| Sexpr::String(unescape_string(s)),
    )(input)
}

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

fn list(input: &str) -> IResult<&str, Sexpr> {
    map(
        delimited(
            char('('),
            many0(sexpr),
            preceded(skip_ws_and_comments, char(')')),
        ),
        Sexpr::List,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_symbol() {
        assert_eq!(parse("foo").unwrap(), Sexpr::symbol("foo"));
        assert_eq!(parse(":keyword").unwrap(), Sexpr::symbol(":keyword"));
        assert_eq!(parse("123").unwrap(), Sexpr::symbol("123"));
        assert_eq!(parse("-1.5").unwrap(), Sexpr::symbol("-1.5"));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(parse(r#""hello""#).unwrap(), Sexpr::string("hello"));
        assert_eq!(parse(r#""hello world""#).unwrap(), Sexpr::string("hello world"));
        assert_eq!(parse(r#""line\nbreak""#).unwrap(), Sexpr::string("line\nbreak"));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(parse("()").unwrap(), Sexpr::list(vec![]));
        assert_eq!(
            parse("(foo bar)").unwrap(),
            Sexpr::list(vec![Sexpr::symbol("foo"), Sexpr::symbol("bar")])
        );
    }

    #[test]
    fn test_parse_nested() {
        assert_eq!(
            parse("(note :pitch (pitch :step C))").unwrap(),
            Sexpr::list(vec![
                Sexpr::symbol("note"),
                Sexpr::symbol(":pitch"),
                Sexpr::list(vec![
                    Sexpr::symbol("pitch"),
                    Sexpr::symbol(":step"),
                    Sexpr::symbol("C"),
                ]),
            ])
        );
    }

    #[test]
    fn test_comments() {
        assert_eq!(
            parse("; comment\n(foo)").unwrap(),
            Sexpr::list(vec![Sexpr::symbol("foo")])
        );
    }
}
```

---

## Task 5: Pretty Printer (`src/sexpr/printer.rs`)

```rust
use super::ast::Sexpr;

/// Configuration for pretty-printing
#[derive(Debug, Clone)]
pub struct PrintConfig {
    /// Number of spaces per indentation level
    pub indent: usize,
    /// Maximum line width before breaking
    pub max_width: usize,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            indent: 2,
            max_width: 80,
        }
    }
}

/// Pretty-print an S-expression to a string
pub fn print(sexpr: &Sexpr) -> String {
    print_with_config(sexpr, &PrintConfig::default())
}

/// Pretty-print with custom configuration
pub fn print_with_config(sexpr: &Sexpr, config: &PrintConfig) -> String {
    let mut output = String::new();
    print_sexpr(&mut output, sexpr, 0, config);
    output
}

fn print_sexpr(out: &mut String, sexpr: &Sexpr, indent: usize, config: &PrintConfig) {
    match sexpr {
        Sexpr::Symbol(s) => out.push_str(s),
        Sexpr::String(s) => {
            out.push('"');
            out.push_str(&escape_string(s));
            out.push('"');
        }
        Sexpr::List(items) => {
            if items.is_empty() {
                out.push_str("()");
                return;
            }

            // Decide: single line or multi-line?
            let single_line = format_single_line(items);
            if single_line.len() + indent <= config.max_width {
                out.push('(');
                out.push_str(&single_line);
                out.push(')');
            } else {
                print_multiline(out, items, indent, config);
            }
        }
    }
}

fn format_single_line(items: &[Sexpr]) -> String {
    items
        .iter()
        .map(|item| match item {
            Sexpr::Symbol(s) => s.clone(),
            Sexpr::String(s) => format!("\"{}\"", escape_string(s)),
            Sexpr::List(inner) => {
                if inner.is_empty() {
                    "()".to_string()
                } else {
                    format!("({})", format_single_line(inner))
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn print_multiline(out: &mut String, items: &[Sexpr], indent: usize, config: &PrintConfig) {
    out.push('(');

    // Print head on same line
    if let Some(head) = items.first() {
        print_sexpr(out, head, indent + 1, config);
    }

    // Determine if this is a keyword-argument list
    let has_kwargs = items.iter().skip(1).any(|item| item.is_keyword());

    let child_indent = indent + config.indent;
    let indent_str = " ".repeat(child_indent);

    for (i, item) in items.iter().enumerate().skip(1) {
        if has_kwargs && item.is_keyword() {
            // Keyword on new line
            out.push('\n');
            out.push_str(&indent_str);
            print_sexpr(out, item, child_indent, config);
        } else if has_kwargs && i > 1 && items.get(i - 1).map(|p| p.is_keyword()).unwrap_or(false) {
            // Value after keyword on same line
            out.push(' ');
            print_sexpr(out, item, child_indent, config);
        } else {
            // Regular item on new line
            out.push('\n');
            out.push_str(&indent_str);
            print_sexpr(out, item, child_indent, config);
        }
    }

    out.push(')');
}

fn escape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_symbol() {
        assert_eq!(print(&Sexpr::symbol("foo")), "foo");
        assert_eq!(print(&Sexpr::symbol(":key")), ":key");
    }

    #[test]
    fn test_print_string() {
        assert_eq!(print(&Sexpr::string("hello")), "\"hello\"");
        assert_eq!(print(&Sexpr::string("line\nbreak")), "\"line\\nbreak\"");
    }

    #[test]
    fn test_print_simple_list() {
        let sexpr = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::symbol("c4"),
            Sexpr::symbol(":q"),
        ]);
        assert_eq!(print(&sexpr), "(note c4 :q)");
    }

    #[test]
    fn test_round_trip() {
        let original = "(score-partwise :version \"4.0\" (part-list))";
        let parsed = super::super::parser::parse(original).unwrap();
        let printed = print(&parsed);
        let reparsed = super::super::parser::parse(&printed).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
```

---

## Task 6: Convert Common Types (`src/sexpr/convert/common.rs`)

Convert the types from `src/ir/common.rs`.

```rust
use crate::ir::common::*;
use crate::sexpr::ast::{Sexpr, ListBuilder};
use crate::sexpr::traits::{ToSexpr, FromSexpr};
use crate::sexpr::error::{ConvertError, ConvertResult};

// ============ YesNo ============

impl ToSexpr for YesNo {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            YesNo::Yes => "yes",
            YesNo::No => "no",
        })
    }
}

impl FromSexpr for YesNo {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("yes") => Ok(YesNo::Yes),
            Some("no") => Ok(YesNo::No),
            _ => Err(ConvertError::type_mismatch("yes/no", sexpr)),
        }
    }
}

// ============ StartStop ============

impl ToSexpr for StartStop {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartStop::Start => "start",
            StartStop::Stop => "stop",
        })
    }
}

impl FromSexpr for StartStop {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(StartStop::Start),
            Some("stop") => Ok(StartStop::Stop),
            _ => Err(ConvertError::type_mismatch("start/stop", sexpr)),
        }
    }
}

// ============ StartStopContinue ============

impl ToSexpr for StartStopContinue {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            StartStopContinue::Start => "start",
            StartStopContinue::Stop => "stop",
            StartStopContinue::Continue => "continue",
        })
    }
}

impl FromSexpr for StartStopContinue {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("start") => Ok(StartStopContinue::Start),
            Some("stop") => Ok(StartStopContinue::Stop),
            Some("continue") => Ok(StartStopContinue::Continue),
            _ => Err(ConvertError::type_mismatch("start/stop/continue", sexpr)),
        }
    }
}

// ============ AboveBelow ============

impl ToSexpr for AboveBelow {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            AboveBelow::Above => "above",
            AboveBelow::Below => "below",
        })
    }
}

impl FromSexpr for AboveBelow {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("above") => Ok(AboveBelow::Above),
            Some("below") => Ok(AboveBelow::Below),
            _ => Err(ConvertError::type_mismatch("above/below", sexpr)),
        }
    }
}

// ============ UpDown ============

impl ToSexpr for UpDown {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            UpDown::Up => "up",
            UpDown::Down => "down",
        })
    }
}

impl FromSexpr for UpDown {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("up") => Ok(UpDown::Up),
            Some("down") => Ok(UpDown::Down),
            _ => Err(ConvertError::type_mismatch("up/down", sexpr)),
        }
    }
}

// ============ LeftRight ============

impl ToSexpr for LeftRight {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            LeftRight::Left => "left",
            LeftRight::Right => "right",
        })
    }
}

impl FromSexpr for LeftRight {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("left") => Ok(LeftRight::Left),
            Some("right") => Ok(LeftRight::Right),
            _ => Err(ConvertError::type_mismatch("left/right", sexpr)),
        }
    }
}

// ============ Position ============

impl ToSexpr for Position {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("position")
            .kwarg_opt("default-x", &self.default_x)
            .kwarg_opt("default-y", &self.default_y)
            .kwarg_opt("relative-x", &self.relative_x)
            .kwarg_opt("relative-y", &self.relative_y)
            .build()
    }
}

impl FromSexpr for Position {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        let list = sexpr.as_list().ok_or_else(|| ConvertError::type_mismatch("list", sexpr))?;

        let mut pos = Position::default();
        let mut iter = list.iter().skip(1); // skip head

        while let Some(item) = iter.next() {
            if let Some(key) = item.as_keyword() {
                let value = iter.next().ok_or(ConvertError::MissingField(key))?;
                match key {
                    "default-x" => pos.default_x = Some(f32::from_sexpr(value)?),
                    "default-y" => pos.default_y = Some(f32::from_sexpr(value)?),
                    "relative-x" => pos.relative_x = Some(f32::from_sexpr(value)?),
                    "relative-y" => pos.relative_y = Some(f32::from_sexpr(value)?),
                    _ => return Err(ConvertError::UnknownKeyword(key.to_string())),
                }
            }
        }

        Ok(pos)
    }
}

// ============ Font ============

impl ToSexpr for Font {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("font")
            .kwarg_opt("family", &self.family)
            .kwarg_opt("style", &self.style)
            .kwarg_opt("size", &self.size)
            .kwarg_opt("weight", &self.weight)
            .build()
    }
}

// ============ FontStyle ============

impl ToSexpr for FontStyle {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
        })
    }
}

impl FromSexpr for FontStyle {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("normal") => Ok(FontStyle::Normal),
            Some("italic") => Ok(FontStyle::Italic),
            _ => Err(ConvertError::type_mismatch("normal/italic", sexpr)),
        }
    }
}

// ============ FontWeight ============

impl ToSexpr for FontWeight {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            FontWeight::Normal => "normal",
            FontWeight::Bold => "bold",
        })
    }
}

impl FromSexpr for FontWeight {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("normal") => Ok(FontWeight::Normal),
            Some("bold") => Ok(FontWeight::Bold),
            _ => Err(ConvertError::type_mismatch("normal/bold", sexpr)),
        }
    }
}

// ============ Color ============

impl ToSexpr for Color {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::string(&self.value)
    }
}

impl FromSexpr for Color {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        Ok(Color {
            value: String::from_sexpr(sexpr)?,
        })
    }
}

// ============ PrintStyle ============

impl ToSexpr for PrintStyle {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("print-style")
            .kwarg_opt("position", &self.position)
            .kwarg_opt("font", &self.font)
            .kwarg_opt("color", &self.color)
            .build()
    }
}

// ============ PrintStyleAlign ============

impl ToSexpr for PrintStyleAlign {
    fn to_sexpr(&self) -> Sexpr {
        ListBuilder::new("print-style-align")
            .kwarg_opt("print-style", &self.print_style)
            .kwarg_opt("halign", &self.halign)
            .kwarg_opt("valign", &self.valign)
            .build()
    }
}

// ============ Halign / Valign ============

impl ToSexpr for Halign {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Halign::Left => "left",
            Halign::Center => "center",
            Halign::Right => "right",
        })
    }
}

impl FromSexpr for Halign {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("left") => Ok(Halign::Left),
            Some("center") => Ok(Halign::Center),
            Some("right") => Ok(Halign::Right),
            _ => Err(ConvertError::type_mismatch("left/center/right", sexpr)),
        }
    }
}

impl ToSexpr for Valign {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(match self {
            Valign::Top => "top",
            Valign::Middle => "middle",
            Valign::Bottom => "bottom",
            Valign::Baseline => "baseline",
        })
    }
}

impl FromSexpr for Valign {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr.as_symbol() {
            Some("top") => Ok(Valign::Top),
            Some("middle") => Ok(Valign::Middle),
            Some("bottom") => Ok(Valign::Bottom),
            Some("baseline") => Ok(Valign::Baseline),
            _ => Err(ConvertError::type_mismatch("top/middle/bottom/baseline", sexpr)),
        }
    }
}

// ============ Tenths (type alias) ============
// If Tenths is `type Tenths = f32`, no impl needed (covered by f32)

// ============ Helper for keyword parsing ============

/// Parse keyword arguments from a list, returning (keyword, value) pairs
pub fn parse_kwargs(list: &[Sexpr]) -> impl Iterator<Item = (&str, &Sexpr)> {
    list.iter()
        .enumerate()
        .filter_map(|(i, item)| {
            item.as_keyword().and_then(|key| {
                list.get(i + 1).map(|value| (key, value))
            })
        })
}

/// Find a keyword argument value in a list
pub fn find_kwarg<'a>(list: &'a [Sexpr], key: &str) -> Option<&'a Sexpr> {
    parse_kwargs(list).find(|(k, _)| *k == key).map(|(_, v)| v)
}

/// Find and parse a required keyword argument
pub fn require_kwarg<T: FromSexpr>(list: &[Sexpr], key: &str) -> ConvertResult<T> {
    find_kwarg(list, key)
        .ok_or(ConvertError::MissingField(Box::leak(key.to_string().into_boxed_str())))
        .and_then(T::from_sexpr)
}

/// Find and parse an optional keyword argument
pub fn optional_kwarg<T: FromSexpr>(list: &[Sexpr], key: &str) -> ConvertResult<Option<T>> {
    match find_kwarg(list, key) {
        Some(value) => T::from_sexpr(value).map(Some),
        None => Ok(None),
    }
}
```

---

## Task 7: Module Setup (`src/sexpr/mod.rs`)

```rust
//! S-expression serialization for Fermata IR types.
//!
//! This module provides:
//! - An untyped S-expression AST
//! - A parser (text → AST)
//! - A pretty-printer (AST → text)
//! - Traits for converting between IR types and AST

pub mod ast;
pub mod error;
pub mod parser;
pub mod printer;
pub mod traits;
pub mod convert;

pub use ast::{Sexpr, ListBuilder};
pub use error::{ParseError, ConvertError, ParseResult, ConvertResult};
pub use parser::{parse, parse_all};
pub use printer::{print, print_with_config, PrintConfig};
pub use traits::{ToSexpr, FromSexpr};

/// Convenience function: IR → S-expr text
pub fn to_string<T: ToSexpr>(value: &T) -> String {
    print(&value.to_sexpr())
}

/// Convenience function: S-expr text → IR
pub fn from_str<T: FromSexpr>(input: &str) -> Result<T, Box<dyn std::error::Error>> {
    let ast = parse(input)?;
    T::from_sexpr(&ast).map_err(Into::into)
}
```

---

## Testing Strategy

### Unit Tests (in each module)

Each conversion module should have:

1. Round-trip tests: `T → Sexpr → T`
2. Pretty-print format tests
3. Error case tests (invalid input)

### Integration Test

Create `tests/sexpr_common.rs`:

```rust
use fermata::ir::common::*;
use fermata::sexpr::{to_string, from_str, ToSexpr, FromSexpr, parse, print};

#[test]
fn test_yes_no_round_trip() {
    let original = YesNo::Yes;
    let sexpr = original.to_sexpr();
    let parsed = YesNo::from_sexpr(&sexpr).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_position_round_trip() {
    let original = Position {
        default_x: Some(10.0),
        default_y: None,
        relative_x: Some(-5.0),
        relative_y: Some(0.0),
    };
    let text = to_string(&original);
    println!("Position: {}", text);
    // ... additional assertions
}
```

---

## Acceptance Criteria

1. ✅ `Sexpr` AST type with Symbol, String, List variants
2. ✅ `nom`-based parser handles symbols, strings, lists, comments
3. ✅ Pretty-printer produces readable, properly-indented output
4. ✅ `ToSexpr` and `FromSexpr` traits defined
5. ✅ All `src/ir/common.rs` types implement both traits
6. ✅ Round-trip tests pass for all common types
7. ✅ Error types provide clear, actionable messages

---

## Notes for Implementation

1. **Match existing IR types exactly** — Check `src/ir/common.rs` for actual field names and types
2. **Keyword naming convention** — Use kebab-case (`:default-x`) to match MusicXML
3. **Optional fields** — Only emit when `Some`
4. **Enum variants** — Use lowercase symbols (`start`, not `Start`)

---

*Next: Milestone 2 — Pitch, Duration, and Note Types*
