//! S-expression AST types.
//!
//! This module defines the core S-expression representation used for
//! serializing and deserializing musical scores in a Lisp-like syntax.

/// An S-expression value.
///
/// S-expressions provide a simple, uniform syntax for representing
/// structured data. This enum covers all the primitive types needed
/// to represent MusicXML scores.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::Sexpr;
///
/// // Create a simple pitch representation
/// let pitch = Sexpr::list(vec![
///     Sexpr::symbol("pitch"),
///     Sexpr::keyword("step"),
///     Sexpr::symbol("C"),
///     Sexpr::keyword("octave"),
///     Sexpr::Integer(4),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    /// A symbol (unquoted identifier): `note`, `pitch`, `C`
    Symbol(String),

    /// A keyword (colon-prefixed): `:step`, `:octave`, `:type`
    ///
    /// The stored string does NOT include the leading colon.
    Keyword(String),

    /// A string literal: `"Piano"`, `"4/4"`
    String(String),

    /// An integer: `4`, `-1`, `0`
    Integer(i64),

    /// A floating-point number: `2.5`, `-0.5`
    Float(f64),

    /// A boolean: `#t`, `#f`
    Bool(bool),

    /// Nil/null: `nil`
    Nil,

    /// A list: `(note :pitch (...) :duration 4)`
    List(Vec<Sexpr>),
}

impl Sexpr {
    /// Create a symbol from any string-like type.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let sym = Sexpr::symbol("note");
    /// assert!(sym.is_symbol("note"));
    /// ```
    pub fn symbol(s: impl Into<String>) -> Self {
        Sexpr::Symbol(s.into())
    }

    /// Create a keyword from any string-like type.
    ///
    /// If the string starts with a colon, it is stripped.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let kw1 = Sexpr::keyword("step");
    /// let kw2 = Sexpr::keyword(":step");
    /// assert_eq!(kw1, kw2);
    /// assert!(kw1.is_keyword("step"));
    /// ```
    pub fn keyword(s: impl Into<String>) -> Self {
        let s = s.into();
        let k = if s.starts_with(':') {
            s[1..].to_string()
        } else {
            s
        };
        Sexpr::Keyword(k)
    }

    /// Create a string literal.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let s = Sexpr::string("Piano");
    /// assert_eq!(s, Sexpr::String("Piano".to_string()));
    /// ```
    pub fn string(s: impl Into<String>) -> Self {
        Sexpr::String(s.into())
    }

    /// Create a list from a vector of S-expressions.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let list = Sexpr::list(vec![
    ///     Sexpr::symbol("pitch"),
    ///     Sexpr::keyword("step"),
    ///     Sexpr::symbol("C"),
    /// ]);
    /// assert!(list.as_list().is_some());
    /// ```
    pub fn list(items: Vec<Sexpr>) -> Self {
        Sexpr::List(items)
    }

    /// Check if this is a specific symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let sym = Sexpr::symbol("note");
    /// assert!(sym.is_symbol("note"));
    /// assert!(!sym.is_symbol("rest"));
    /// ```
    pub fn is_symbol(&self, name: &str) -> bool {
        matches!(self, Sexpr::Symbol(s) if s == name)
    }

    /// Check if this is a specific keyword.
    ///
    /// The name should NOT include the leading colon.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let kw = Sexpr::keyword("step");
    /// assert!(kw.is_keyword("step"));
    /// assert!(!kw.is_keyword("octave"));
    /// ```
    pub fn is_keyword(&self, name: &str) -> bool {
        matches!(self, Sexpr::Keyword(k) if k == name)
    }

    /// Get the symbol string if this is a Symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let sym = Sexpr::symbol("note");
    /// assert_eq!(sym.as_symbol(), Some("note"));
    ///
    /// let kw = Sexpr::keyword("step");
    /// assert_eq!(kw.as_symbol(), None);
    /// ```
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Sexpr::Symbol(s) => Some(s),
            _ => None,
        }
    }

    /// Get the keyword string if this is a Keyword (without the colon).
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let kw = Sexpr::keyword("step");
    /// assert_eq!(kw.as_keyword(), Some("step"));
    ///
    /// let sym = Sexpr::symbol("note");
    /// assert_eq!(sym.as_keyword(), None);
    /// ```
    pub fn as_keyword(&self) -> Option<&str> {
        match self {
            Sexpr::Keyword(k) => Some(k),
            _ => None,
        }
    }

    /// Get the list contents if this is a List.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let list = Sexpr::list(vec![Sexpr::symbol("a"), Sexpr::symbol("b")]);
    /// assert_eq!(list.as_list().map(|l| l.len()), Some(2));
    ///
    /// let sym = Sexpr::symbol("note");
    /// assert_eq!(sym.as_list(), None);
    /// ```
    pub fn as_list(&self) -> Option<&[Sexpr]> {
        match self {
            Sexpr::List(items) => Some(items),
            _ => None,
        }
    }

    /// Get the string value if this is a String.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let s = Sexpr::string("Piano");
    /// assert_eq!(s.as_string(), Some("Piano"));
    ///
    /// let sym = Sexpr::symbol("note");
    /// assert_eq!(sym.as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Sexpr::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get the integer value if this is an Integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let i = Sexpr::Integer(42);
    /// assert_eq!(i.as_integer(), Some(42));
    ///
    /// let f = Sexpr::Float(3.14);
    /// assert_eq!(f.as_integer(), None);
    /// ```
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Sexpr::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get the float value if this is a Float.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let f = Sexpr::Float(3.14);
    /// assert_eq!(f.as_float(), Some(3.14));
    ///
    /// let i = Sexpr::Integer(42);
    /// assert_eq!(i.as_float(), None);
    /// ```
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Sexpr::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Get the boolean value if this is a Bool.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// let b = Sexpr::Bool(true);
    /// assert_eq!(b.as_bool(), Some(true));
    ///
    /// let s = Sexpr::symbol("true");
    /// assert_eq!(s.as_bool(), None);
    /// ```
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Sexpr::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if this is Nil.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// assert!(Sexpr::Nil.is_nil());
    /// assert!(!Sexpr::Bool(false).is_nil());
    /// ```
    pub fn is_nil(&self) -> bool {
        matches!(self, Sexpr::Nil)
    }

    /// Check if this is a list (regardless of contents).
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// assert!(Sexpr::list(vec![]).is_list());
    /// assert!(!Sexpr::symbol("note").is_list());
    /// ```
    pub fn is_list(&self) -> bool {
        matches!(self, Sexpr::List(_))
    }

    /// Get the number value if this is an Integer or Float.
    ///
    /// This is useful when you need a numeric value and don't care
    /// about the exact representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::Sexpr;
    ///
    /// assert_eq!(Sexpr::Integer(42).as_number(), Some(42.0));
    /// assert_eq!(Sexpr::Float(3.14).as_number(), Some(3.14));
    /// assert_eq!(Sexpr::symbol("foo").as_number(), None);
    /// ```
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Sexpr::Integer(i) => Some(*i as f64),
            Sexpr::Float(f) => Some(*f),
            _ => None,
        }
    }
}

/// Builder for constructing S-expression lists with keyword arguments.
///
/// `ListBuilder` provides a fluent API for building S-expression lists
/// that follow the keyword argument pattern common in Lisp-like languages.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, ListBuilder};
///
/// let sexpr = ListBuilder::new("pitch")
///     .kwarg("step", &"C".to_string())
///     .kwarg("octave", &4i32)
///     .build();
///
/// // Produces: (pitch :step "C" :octave 4)
/// ```
///
/// # Optional Fields
///
/// Use `kwarg_opt` to only include a field if it has a value:
///
/// ```
/// use fermata::sexpr::ListBuilder;
///
/// let alter: Option<f64> = None;
/// let sexpr = ListBuilder::new("pitch")
///     .kwarg("step", &"C".to_string())
///     .kwarg_opt("alter", &alter)  // Not included because None
///     .kwarg("octave", &4i32)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ListBuilder {
    items: Vec<Sexpr>,
}

impl ListBuilder {
    /// Create a new builder with the given head symbol.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ListBuilder;
    ///
    /// let builder = ListBuilder::new("note");
    /// let sexpr = builder.build();
    /// assert!(sexpr.as_list().unwrap()[0].is_symbol("note"));
    /// ```
    pub fn new(head: impl Into<String>) -> Self {
        Self {
            items: vec![Sexpr::symbol(head)],
        }
    }

    /// Add a positional argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::{Sexpr, ListBuilder};
    ///
    /// let sexpr = ListBuilder::new("list")
    ///     .arg(Sexpr::Integer(1))
    ///     .arg(Sexpr::Integer(2))
    ///     .build();
    /// // Produces: (list 1 2)
    /// ```
    pub fn arg(mut self, value: Sexpr) -> Self {
        self.items.push(value);
        self
    }

    /// Add a keyword argument pair (always included).
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ListBuilder;
    ///
    /// let sexpr = ListBuilder::new("note")
    ///     .kwarg("duration", &4i32)
    ///     .build();
    /// // Produces: (note :duration 4)
    /// ```
    pub fn kwarg<T: super::traits::ToSexpr>(mut self, key: &str, value: &T) -> Self {
        self.items.push(Sexpr::keyword(key));
        self.items.push(value.to_sexpr());
        self
    }

    /// Add a keyword argument pair only if the value is `Some`.
    ///
    /// This is useful for optional fields that should be omitted when `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ListBuilder;
    ///
    /// let alter: Option<f64> = Some(1.0);
    /// let sexpr = ListBuilder::new("pitch")
    ///     .kwarg_opt("alter", &alter)
    ///     .build();
    /// // Produces: (pitch :alter 1.0)
    ///
    /// let alter: Option<f64> = None;
    /// let sexpr = ListBuilder::new("pitch")
    ///     .kwarg_opt("alter", &alter)
    ///     .build();
    /// // Produces: (pitch)
    /// ```
    pub fn kwarg_opt<T: super::traits::ToSexpr>(mut self, key: &str, value: &Option<T>) -> Self {
        if let Some(v) = value {
            self.items.push(Sexpr::keyword(key));
            self.items.push(v.to_sexpr());
        }
        self
    }

    /// Add a keyword with a raw Sexpr value (always included).
    ///
    /// Use this when you already have a Sexpr and don't need conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::{Sexpr, ListBuilder};
    ///
    /// let pitch = Sexpr::list(vec![Sexpr::symbol("pitch")]);
    /// let sexpr = ListBuilder::new("note")
    ///     .kwarg_raw("pitch", pitch)
    ///     .build();
    /// ```
    pub fn kwarg_raw(mut self, key: &str, value: Sexpr) -> Self {
        self.items.push(Sexpr::keyword(key));
        self.items.push(value);
        self
    }

    /// Add a keyword with a raw Sexpr value only if it's `Some`.
    pub fn kwarg_raw_opt(mut self, key: &str, value: Option<Sexpr>) -> Self {
        if let Some(v) = value {
            self.items.push(Sexpr::keyword(key));
            self.items.push(v);
        }
        self
    }

    /// Add a list of items under a keyword (only if non-empty).
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ListBuilder;
    ///
    /// let items = vec![1i32, 2, 3];
    /// let sexpr = ListBuilder::new("container")
    ///     .kwarg_list("items", &items)
    ///     .build();
    /// // Produces: (container :items (1 2 3))
    ///
    /// let empty: Vec<i32> = vec![];
    /// let sexpr = ListBuilder::new("container")
    ///     .kwarg_list("items", &empty)
    ///     .build();
    /// // Produces: (container) - no :items because empty
    /// ```
    pub fn kwarg_list<T: super::traits::ToSexpr>(mut self, key: &str, items: &[T]) -> Self {
        if !items.is_empty() {
            self.items.push(Sexpr::keyword(key));
            self.items
                .push(Sexpr::list(items.iter().map(|i| i.to_sexpr()).collect()));
        }
        self
    }

    /// Add a boolean keyword (only included if true).
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ListBuilder;
    ///
    /// let sexpr = ListBuilder::new("note")
    ///     .kwarg_bool("chord", true)
    ///     .build();
    /// // Produces: (note :chord t)
    ///
    /// let sexpr = ListBuilder::new("note")
    ///     .kwarg_bool("chord", false)
    ///     .build();
    /// // Produces: (note) - no :chord because false
    /// ```
    pub fn kwarg_bool(mut self, key: &str, value: bool) -> Self {
        if value {
            self.items.push(Sexpr::keyword(key));
            self.items.push(Sexpr::symbol("t"));
        }
        self
    }

    /// Add children under a keyword (only if non-empty), each on its own.
    ///
    /// Unlike `kwarg_list`, this adds children directly without wrapping
    /// them in a list. Useful for repeating elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::{Sexpr, ListBuilder};
    ///
    /// let children = vec![
    ///     Sexpr::list(vec![Sexpr::symbol("note")]),
    ///     Sexpr::list(vec![Sexpr::symbol("rest")]),
    /// ];
    /// let sexpr = ListBuilder::new("measure")
    ///     .children(&children)
    ///     .build();
    /// // Produces: (measure (note) (rest))
    /// ```
    pub fn children(mut self, items: &[Sexpr]) -> Self {
        for item in items {
            self.items.push(item.clone());
        }
        self
    }

    /// Add children converted via ToSexpr.
    pub fn children_from<T: super::traits::ToSexpr>(mut self, items: &[T]) -> Self {
        for item in items {
            self.items.push(item.to_sexpr());
        }
        self
    }

    /// Build the final S-expression list.
    ///
    /// # Examples
    ///
    /// ```
    /// use fermata::sexpr::ListBuilder;
    ///
    /// let sexpr = ListBuilder::new("note").build();
    /// assert!(sexpr.is_list());
    /// ```
    pub fn build(self) -> Sexpr {
        Sexpr::List(self.items)
    }
}

impl Default for ListBuilder {
    fn default() -> Self {
        Self { items: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Symbol Tests ===

    #[test]
    fn test_symbol_creation() {
        let sym = Sexpr::symbol("note");
        assert_eq!(sym, Sexpr::Symbol("note".to_string()));
    }

    #[test]
    fn test_symbol_from_string() {
        let sym = Sexpr::symbol(String::from("pitch"));
        assert_eq!(sym, Sexpr::Symbol("pitch".to_string()));
    }

    #[test]
    fn test_is_symbol_true() {
        let sym = Sexpr::symbol("note");
        assert!(sym.is_symbol("note"));
    }

    #[test]
    fn test_is_symbol_false_wrong_name() {
        let sym = Sexpr::symbol("note");
        assert!(!sym.is_symbol("rest"));
    }

    #[test]
    fn test_is_symbol_false_wrong_type() {
        let kw = Sexpr::keyword("note");
        assert!(!kw.is_symbol("note"));
    }

    #[test]
    fn test_as_symbol_some() {
        let sym = Sexpr::symbol("note");
        assert_eq!(sym.as_symbol(), Some("note"));
    }

    #[test]
    fn test_as_symbol_none() {
        let kw = Sexpr::keyword("step");
        assert_eq!(kw.as_symbol(), None);
    }

    // === Keyword Tests ===

    #[test]
    fn test_keyword_creation_without_colon() {
        let kw = Sexpr::keyword("step");
        assert_eq!(kw, Sexpr::Keyword("step".to_string()));
    }

    #[test]
    fn test_keyword_creation_with_colon() {
        let kw = Sexpr::keyword(":step");
        assert_eq!(kw, Sexpr::Keyword("step".to_string()));
    }

    #[test]
    fn test_keyword_creation_equality() {
        let kw1 = Sexpr::keyword("step");
        let kw2 = Sexpr::keyword(":step");
        assert_eq!(kw1, kw2);
    }

    #[test]
    fn test_is_keyword_true() {
        let kw = Sexpr::keyword("step");
        assert!(kw.is_keyword("step"));
    }

    #[test]
    fn test_is_keyword_false_wrong_name() {
        let kw = Sexpr::keyword("step");
        assert!(!kw.is_keyword("octave"));
    }

    #[test]
    fn test_is_keyword_false_wrong_type() {
        let sym = Sexpr::symbol("step");
        assert!(!sym.is_keyword("step"));
    }

    #[test]
    fn test_as_keyword_some() {
        let kw = Sexpr::keyword("step");
        assert_eq!(kw.as_keyword(), Some("step"));
    }

    #[test]
    fn test_as_keyword_none() {
        let sym = Sexpr::symbol("note");
        assert_eq!(sym.as_keyword(), None);
    }

    // === String Tests ===

    #[test]
    fn test_string_creation() {
        let s = Sexpr::string("Piano");
        assert_eq!(s, Sexpr::String("Piano".to_string()));
    }

    #[test]
    fn test_as_string_some() {
        let s = Sexpr::string("Piano");
        assert_eq!(s.as_string(), Some("Piano"));
    }

    #[test]
    fn test_as_string_none() {
        let sym = Sexpr::symbol("Piano");
        assert_eq!(sym.as_string(), None);
    }

    // === List Tests ===

    #[test]
    fn test_list_creation_empty() {
        let list = Sexpr::list(vec![]);
        assert_eq!(list, Sexpr::List(vec![]));
    }

    #[test]
    fn test_list_creation_with_items() {
        let list = Sexpr::list(vec![Sexpr::symbol("a"), Sexpr::symbol("b")]);
        assert_eq!(
            list,
            Sexpr::List(vec![Sexpr::symbol("a"), Sexpr::symbol("b")])
        );
    }

    #[test]
    fn test_as_list_some() {
        let list = Sexpr::list(vec![Sexpr::symbol("a")]);
        let items = list.as_list();
        assert!(items.is_some());
        assert_eq!(items.unwrap().len(), 1);
    }

    #[test]
    fn test_as_list_none() {
        let sym = Sexpr::symbol("note");
        assert_eq!(sym.as_list(), None);
    }

    #[test]
    fn test_is_list_true() {
        let list = Sexpr::list(vec![]);
        assert!(list.is_list());
    }

    #[test]
    fn test_is_list_false() {
        let sym = Sexpr::symbol("note");
        assert!(!sym.is_list());
    }

    // === Integer Tests ===

    #[test]
    fn test_integer_creation() {
        let i = Sexpr::Integer(42);
        assert_eq!(i, Sexpr::Integer(42));
    }

    #[test]
    fn test_integer_negative() {
        let i = Sexpr::Integer(-5);
        assert_eq!(i.as_integer(), Some(-5));
    }

    #[test]
    fn test_as_integer_some() {
        let i = Sexpr::Integer(42);
        assert_eq!(i.as_integer(), Some(42));
    }

    #[test]
    fn test_as_integer_none() {
        let f = Sexpr::Float(3.14);
        assert_eq!(f.as_integer(), None);
    }

    // === Float Tests ===

    #[test]
    fn test_float_creation() {
        let f = Sexpr::Float(3.14);
        assert_eq!(f, Sexpr::Float(3.14));
    }

    #[test]
    fn test_float_negative() {
        let f = Sexpr::Float(-0.5);
        assert_eq!(f.as_float(), Some(-0.5));
    }

    #[test]
    fn test_as_float_some() {
        let f = Sexpr::Float(2.5);
        assert_eq!(f.as_float(), Some(2.5));
    }

    #[test]
    fn test_as_float_none() {
        let i = Sexpr::Integer(42);
        assert_eq!(i.as_float(), None);
    }

    // === Bool Tests ===

    #[test]
    fn test_bool_true() {
        let b = Sexpr::Bool(true);
        assert_eq!(b.as_bool(), Some(true));
    }

    #[test]
    fn test_bool_false() {
        let b = Sexpr::Bool(false);
        assert_eq!(b.as_bool(), Some(false));
    }

    #[test]
    fn test_as_bool_none() {
        let s = Sexpr::symbol("true");
        assert_eq!(s.as_bool(), None);
    }

    // === Nil Tests ===

    #[test]
    fn test_nil() {
        let nil = Sexpr::Nil;
        assert!(nil.is_nil());
    }

    #[test]
    fn test_is_nil_false() {
        let b = Sexpr::Bool(false);
        assert!(!b.is_nil());
    }

    // === Clone and PartialEq Tests ===

    #[test]
    fn test_clone() {
        let original = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::keyword("pitch"),
            Sexpr::Integer(4),
        ]);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_partial_eq_equal() {
        let a = Sexpr::symbol("note");
        let b = Sexpr::symbol("note");
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq_not_equal() {
        let a = Sexpr::symbol("note");
        let b = Sexpr::symbol("rest");
        assert_ne!(a, b);
    }

    #[test]
    fn test_partial_eq_different_types() {
        let sym = Sexpr::symbol("step");
        let kw = Sexpr::keyword("step");
        assert_ne!(sym, kw);
    }

    // === Debug Tests ===

    #[test]
    fn test_debug_symbol() {
        let sym = Sexpr::symbol("note");
        let debug = format!("{:?}", sym);
        assert!(debug.contains("Symbol"));
        assert!(debug.contains("note"));
    }

    #[test]
    fn test_debug_list() {
        let list = Sexpr::list(vec![Sexpr::symbol("a")]);
        let debug = format!("{:?}", list);
        assert!(debug.contains("List"));
    }

    // === Nested Structure Tests ===

    #[test]
    fn test_nested_list() {
        let inner = Sexpr::list(vec![Sexpr::symbol("pitch"), Sexpr::Integer(4)]);
        let outer = Sexpr::list(vec![Sexpr::symbol("note"), inner]);

        assert!(outer.is_list());
        let items = outer.as_list().unwrap();
        assert_eq!(items.len(), 2);
        assert!(items[0].is_symbol("note"));
        assert!(items[1].is_list());
    }

    #[test]
    fn test_complex_structure() {
        // (note :pitch (pitch :step C :octave 4) :duration 1)
        let pitch = Sexpr::list(vec![
            Sexpr::symbol("pitch"),
            Sexpr::keyword("step"),
            Sexpr::symbol("C"),
            Sexpr::keyword("octave"),
            Sexpr::Integer(4),
        ]);

        let note = Sexpr::list(vec![
            Sexpr::symbol("note"),
            Sexpr::keyword("pitch"),
            pitch,
            Sexpr::keyword("duration"),
            Sexpr::Integer(1),
        ]);

        assert!(note.is_list());
        let items = note.as_list().unwrap();
        assert_eq!(items.len(), 5);
        assert!(items[0].is_symbol("note"));
        assert!(items[1].is_keyword("pitch"));
        assert!(items[2].is_list());
    }

    // === as_number Tests ===

    #[test]
    fn test_as_number_integer() {
        let i = Sexpr::Integer(42);
        assert_eq!(i.as_number(), Some(42.0));
    }

    #[test]
    fn test_as_number_float() {
        let f = Sexpr::Float(3.14);
        assert!((f.as_number().unwrap() - 3.14).abs() < 0.0001);
    }

    #[test]
    fn test_as_number_none() {
        let s = Sexpr::symbol("foo");
        assert_eq!(s.as_number(), None);
    }

    // === ListBuilder Tests ===

    #[test]
    fn test_list_builder_new() {
        let builder = ListBuilder::new("note");
        let sexpr = builder.build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 1);
        assert!(items[0].is_symbol("note"));
    }

    #[test]
    fn test_list_builder_arg() {
        let sexpr = ListBuilder::new("list")
            .arg(Sexpr::Integer(1))
            .arg(Sexpr::Integer(2))
            .build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[1].as_integer(), Some(1));
        assert_eq!(items[2].as_integer(), Some(2));
    }

    #[test]
    fn test_list_builder_kwarg() {
        let sexpr = ListBuilder::new("note").kwarg("duration", &4i32).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
        assert!(items[1].is_keyword("duration"));
        assert_eq!(items[2].as_integer(), Some(4));
    }

    #[test]
    fn test_list_builder_kwarg_opt_some() {
        let value: Option<i32> = Some(42);
        let sexpr = ListBuilder::new("test").kwarg_opt("value", &value).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
        assert!(items[1].is_keyword("value"));
    }

    #[test]
    fn test_list_builder_kwarg_opt_none() {
        let value: Option<i32> = None;
        let sexpr = ListBuilder::new("test").kwarg_opt("value", &value).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 1); // Only the head
    }

    #[test]
    fn test_list_builder_kwarg_raw() {
        let inner = Sexpr::list(vec![Sexpr::symbol("pitch")]);
        let sexpr = ListBuilder::new("note").kwarg_raw("pitch", inner).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
        assert!(items[2].is_list());
    }

    #[test]
    fn test_list_builder_kwarg_raw_opt_some() {
        let inner = Some(Sexpr::Integer(5));
        let sexpr = ListBuilder::new("test")
            .kwarg_raw_opt("value", inner)
            .build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_list_builder_kwarg_raw_opt_none() {
        let inner: Option<Sexpr> = None;
        let sexpr = ListBuilder::new("test")
            .kwarg_raw_opt("value", inner)
            .build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 1);
    }

    #[test]
    fn test_list_builder_kwarg_list_non_empty() {
        let items = vec![1i32, 2, 3];
        let sexpr = ListBuilder::new("container")
            .kwarg_list("items", &items)
            .build();
        let list = sexpr.as_list().unwrap();
        assert_eq!(list.len(), 3);
        assert!(list[1].is_keyword("items"));
        assert!(list[2].is_list());
        assert_eq!(list[2].as_list().unwrap().len(), 3);
    }

    #[test]
    fn test_list_builder_kwarg_list_empty() {
        let items: Vec<i32> = vec![];
        let sexpr = ListBuilder::new("container")
            .kwarg_list("items", &items)
            .build();
        let list = sexpr.as_list().unwrap();
        assert_eq!(list.len(), 1); // Empty list is omitted
    }

    #[test]
    fn test_list_builder_kwarg_bool_true() {
        let sexpr = ListBuilder::new("note").kwarg_bool("chord", true).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
        assert!(items[1].is_keyword("chord"));
        assert!(items[2].is_symbol("t"));
    }

    #[test]
    fn test_list_builder_kwarg_bool_false() {
        let sexpr = ListBuilder::new("note").kwarg_bool("chord", false).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 1); // False is omitted
    }

    #[test]
    fn test_list_builder_children() {
        let children = vec![
            Sexpr::list(vec![Sexpr::symbol("note")]),
            Sexpr::list(vec![Sexpr::symbol("rest")]),
        ];
        let sexpr = ListBuilder::new("measure").children(&children).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 3);
        assert!(items[1].is_list());
        assert!(items[2].is_list());
    }

    #[test]
    fn test_list_builder_children_from() {
        let numbers = vec![1i32, 2, 3];
        let sexpr = ListBuilder::new("numbers").children_from(&numbers).build();
        let items = sexpr.as_list().unwrap();
        assert_eq!(items.len(), 4); // head + 3 numbers
    }

    #[test]
    fn test_list_builder_chained() {
        let sexpr = ListBuilder::new("note")
            .kwarg("duration", &4i32)
            .kwarg_opt("alter", &Some(1.0f64))
            .kwarg_bool("chord", true)
            .build();
        let items = sexpr.as_list().unwrap();
        // head + :duration 4 + :alter 1.0 + :chord t = 7 items
        assert_eq!(items.len(), 7);
    }

    #[test]
    fn test_list_builder_default() {
        let builder = ListBuilder::default();
        let sexpr = builder.build();
        assert_eq!(sexpr, Sexpr::List(vec![]));
    }

    #[test]
    fn test_list_builder_clone() {
        let builder = ListBuilder::new("test").kwarg("value", &42i32);
        let cloned = builder.clone();
        assert_eq!(builder.build(), cloned.build());
    }
}
