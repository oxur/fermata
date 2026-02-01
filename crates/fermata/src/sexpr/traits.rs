//! Conversion traits for S-expression serialization.
//!
//! This module defines the core traits for converting between IR types
//! and the untyped [`Sexpr`] AST:
//!
//! - [`ToSexpr`] - Convert an IR type to an S-expression
//! - [`FromSexpr`] - Parse an S-expression into an IR type
//!
//! The module also provides implementations for Rust primitive types
//! and common standard library types like `Option<T>` and `Vec<T>`.

use super::ast::Sexpr;
use super::error::{ConvertError, ConvertResult};

/// Convert an IR type to an S-expression.
///
/// This trait is implemented for IR types to enable serialization
/// to S-expression format.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, ToSexpr};
///
/// // Primitive types implement ToSexpr
/// assert_eq!(42i64.to_sexpr(), Sexpr::Integer(42));
/// assert_eq!("hello".to_string().to_sexpr(), Sexpr::String("hello".to_string()));
/// assert_eq!(true.to_sexpr(), Sexpr::symbol("t"));
/// ```
pub trait ToSexpr {
    /// Convert this value to an S-expression.
    fn to_sexpr(&self) -> Sexpr;
}

/// Parse an S-expression into an IR type.
///
/// This trait is implemented for IR types to enable deserialization
/// from S-expression format.
///
/// # Examples
///
/// ```
/// use fermata::sexpr::{Sexpr, FromSexpr};
///
/// let sexpr = Sexpr::Integer(42);
/// let value: i64 = i64::from_sexpr(&sexpr).unwrap();
/// assert_eq!(value, 42);
/// ```
pub trait FromSexpr: Sized {
    /// Parse an S-expression into this type.
    ///
    /// # Errors
    ///
    /// Returns a [`ConvertError`] if the S-expression cannot be converted
    /// to this type.
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self>;
}

// === String Implementation ===

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

impl ToSexpr for &str {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::String((*self).to_string())
    }
}

// === Boolean Implementation ===

impl ToSexpr for bool {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::symbol(if *self { "t" } else { "nil" })
    }
}

impl FromSexpr for bool {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr {
            Sexpr::Bool(b) => Ok(*b),
            Sexpr::Symbol(s) if s == "t" || s == "true" || s == "#t" => Ok(true),
            Sexpr::Symbol(s) if s == "nil" || s == "false" || s == "#f" => Ok(false),
            _ => Err(ConvertError::type_mismatch("boolean (t/nil)", sexpr)),
        }
    }
}

// === Numeric Type Implementations ===

macro_rules! impl_integer_tosexpr {
    ($($ty:ty),+) => {
        $(
            impl ToSexpr for $ty {
                fn to_sexpr(&self) -> Sexpr {
                    Sexpr::Integer(*self as i64)
                }
            }

            impl FromSexpr for $ty {
                fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
                    match sexpr {
                        Sexpr::Integer(i) => {
                            (*i).try_into().map_err(|_| ConvertError::InvalidValue {
                                field: stringify!($ty),
                                value: i.to_string(),
                            })
                        }
                        Sexpr::Symbol(s) => s.parse().map_err(|_| ConvertError::InvalidValue {
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

impl_integer_tosexpr!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

macro_rules! impl_float_tosexpr {
    ($($ty:ty),+) => {
        $(
            impl ToSexpr for $ty {
                fn to_sexpr(&self) -> Sexpr {
                    Sexpr::Float(*self as f64)
                }
            }

            impl FromSexpr for $ty {
                fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
                    match sexpr {
                        Sexpr::Float(f) => Ok(*f as $ty),
                        Sexpr::Integer(i) => Ok(*i as $ty),
                        Sexpr::Symbol(s) => s.parse().map_err(|_| ConvertError::InvalidValue {
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

impl_float_tosexpr!(f32, f64);

// === Option<T> Implementation ===

impl<T: ToSexpr> ToSexpr for Option<T> {
    fn to_sexpr(&self) -> Sexpr {
        match self {
            Some(v) => v.to_sexpr(),
            None => Sexpr::Nil,
        }
    }
}

impl<T: FromSexpr> FromSexpr for Option<T> {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        match sexpr {
            Sexpr::Nil => Ok(None),
            Sexpr::Symbol(s) if s == "nil" => Ok(None),
            _ => T::from_sexpr(sexpr).map(Some),
        }
    }
}

// === Vec<T> Implementation ===

impl<T: ToSexpr> ToSexpr for Vec<T> {
    fn to_sexpr(&self) -> Sexpr {
        Sexpr::List(self.iter().map(|item| item.to_sexpr()).collect())
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

// === Sexpr Implementation (identity) ===

impl ToSexpr for Sexpr {
    fn to_sexpr(&self) -> Sexpr {
        self.clone()
    }
}

impl FromSexpr for Sexpr {
    fn from_sexpr(sexpr: &Sexpr) -> ConvertResult<Self> {
        Ok(sexpr.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === String Tests ===

    #[test]
    fn test_string_to_sexpr() {
        let s = "hello".to_string();
        assert_eq!(s.to_sexpr(), Sexpr::String("hello".to_string()));
    }

    #[test]
    fn test_str_to_sexpr() {
        let s: &str = "world";
        assert_eq!(s.to_sexpr(), Sexpr::String("world".to_string()));
    }

    #[test]
    fn test_string_from_sexpr_string() {
        let sexpr = Sexpr::String("hello".to_string());
        assert_eq!(String::from_sexpr(&sexpr).unwrap(), "hello");
    }

    #[test]
    fn test_string_from_sexpr_symbol() {
        let sexpr = Sexpr::Symbol("world".to_string());
        assert_eq!(String::from_sexpr(&sexpr).unwrap(), "world");
    }

    #[test]
    fn test_string_from_sexpr_type_mismatch() {
        let sexpr = Sexpr::Integer(42);
        assert!(String::from_sexpr(&sexpr).is_err());
    }

    // === Boolean Tests ===

    #[test]
    fn test_bool_true_to_sexpr() {
        assert_eq!(true.to_sexpr(), Sexpr::Symbol("t".to_string()));
    }

    #[test]
    fn test_bool_false_to_sexpr() {
        assert_eq!(false.to_sexpr(), Sexpr::Symbol("nil".to_string()));
    }

    #[test]
    fn test_bool_from_sexpr_t() {
        let sexpr = Sexpr::Symbol("t".to_string());
        assert_eq!(bool::from_sexpr(&sexpr).unwrap(), true);
    }

    #[test]
    fn test_bool_from_sexpr_true() {
        let sexpr = Sexpr::Symbol("true".to_string());
        assert_eq!(bool::from_sexpr(&sexpr).unwrap(), true);
    }

    #[test]
    fn test_bool_from_sexpr_hash_t() {
        let sexpr = Sexpr::Symbol("#t".to_string());
        assert_eq!(bool::from_sexpr(&sexpr).unwrap(), true);
    }

    #[test]
    fn test_bool_from_sexpr_nil() {
        let sexpr = Sexpr::Symbol("nil".to_string());
        assert_eq!(bool::from_sexpr(&sexpr).unwrap(), false);
    }

    #[test]
    fn test_bool_from_sexpr_false() {
        let sexpr = Sexpr::Symbol("false".to_string());
        assert_eq!(bool::from_sexpr(&sexpr).unwrap(), false);
    }

    #[test]
    fn test_bool_from_sexpr_hash_f() {
        let sexpr = Sexpr::Symbol("#f".to_string());
        assert_eq!(bool::from_sexpr(&sexpr).unwrap(), false);
    }

    #[test]
    fn test_bool_from_sexpr_bool_variant() {
        assert_eq!(bool::from_sexpr(&Sexpr::Bool(true)).unwrap(), true);
        assert_eq!(bool::from_sexpr(&Sexpr::Bool(false)).unwrap(), false);
    }

    #[test]
    fn test_bool_from_sexpr_type_mismatch() {
        let sexpr = Sexpr::Integer(1);
        assert!(bool::from_sexpr(&sexpr).is_err());
    }

    // === Integer Tests ===

    #[test]
    fn test_i32_to_sexpr() {
        assert_eq!(42i32.to_sexpr(), Sexpr::Integer(42));
    }

    #[test]
    fn test_i64_to_sexpr() {
        assert_eq!((-100i64).to_sexpr(), Sexpr::Integer(-100));
    }

    #[test]
    fn test_u8_to_sexpr() {
        assert_eq!(255u8.to_sexpr(), Sexpr::Integer(255));
    }

    #[test]
    fn test_i32_from_sexpr_integer() {
        let sexpr = Sexpr::Integer(42);
        assert_eq!(i32::from_sexpr(&sexpr).unwrap(), 42);
    }

    #[test]
    fn test_i32_from_sexpr_symbol() {
        let sexpr = Sexpr::Symbol("-123".to_string());
        assert_eq!(i32::from_sexpr(&sexpr).unwrap(), -123);
    }

    #[test]
    fn test_u8_from_sexpr_overflow() {
        let sexpr = Sexpr::Integer(256);
        assert!(u8::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_i32_from_sexpr_type_mismatch() {
        let sexpr = Sexpr::String("not a number".to_string());
        assert!(i32::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_i32_from_sexpr_invalid_symbol() {
        let sexpr = Sexpr::Symbol("abc".to_string());
        assert!(i32::from_sexpr(&sexpr).is_err());
    }

    // === Float Tests ===

    #[test]
    fn test_f64_to_sexpr() {
        assert_eq!(3.14f64.to_sexpr(), Sexpr::Float(3.14));
    }

    #[test]
    fn test_f32_to_sexpr() {
        let sexpr = 2.5f32.to_sexpr();
        if let Sexpr::Float(f) = sexpr {
            assert!((f - 2.5).abs() < 0.0001);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_f64_from_sexpr_float() {
        let sexpr = Sexpr::Float(3.14);
        assert!((f64::from_sexpr(&sexpr).unwrap() - 3.14).abs() < 0.0001);
    }

    #[test]
    fn test_f64_from_sexpr_integer() {
        let sexpr = Sexpr::Integer(42);
        assert_eq!(f64::from_sexpr(&sexpr).unwrap(), 42.0);
    }

    #[test]
    fn test_f64_from_sexpr_symbol() {
        let sexpr = Sexpr::Symbol("-1.5".to_string());
        assert!((f64::from_sexpr(&sexpr).unwrap() - (-1.5)).abs() < 0.0001);
    }

    #[test]
    fn test_f64_from_sexpr_type_mismatch() {
        let sexpr = Sexpr::String("not a number".to_string());
        assert!(f64::from_sexpr(&sexpr).is_err());
    }

    // === Option Tests ===

    #[test]
    fn test_option_some_to_sexpr() {
        let opt: Option<i32> = Some(42);
        assert_eq!(opt.to_sexpr(), Sexpr::Integer(42));
    }

    #[test]
    fn test_option_none_to_sexpr() {
        let opt: Option<i32> = None;
        assert_eq!(opt.to_sexpr(), Sexpr::Nil);
    }

    #[test]
    fn test_option_from_sexpr_some() {
        let sexpr = Sexpr::Integer(42);
        assert_eq!(Option::<i32>::from_sexpr(&sexpr).unwrap(), Some(42));
    }

    #[test]
    fn test_option_from_sexpr_nil() {
        let sexpr = Sexpr::Nil;
        assert_eq!(Option::<i32>::from_sexpr(&sexpr).unwrap(), None);
    }

    #[test]
    fn test_option_from_sexpr_nil_symbol() {
        let sexpr = Sexpr::Symbol("nil".to_string());
        assert_eq!(Option::<i32>::from_sexpr(&sexpr).unwrap(), None);
    }

    // === Vec Tests ===

    #[test]
    fn test_vec_to_sexpr() {
        let v = vec![1i32, 2, 3];
        assert_eq!(
            v.to_sexpr(),
            Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(2), Sexpr::Integer(3)])
        );
    }

    #[test]
    fn test_vec_empty_to_sexpr() {
        let v: Vec<i32> = vec![];
        assert_eq!(v.to_sexpr(), Sexpr::List(vec![]));
    }

    #[test]
    fn test_vec_from_sexpr() {
        let sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(2), Sexpr::Integer(3)]);
        assert_eq!(Vec::<i32>::from_sexpr(&sexpr).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_from_sexpr_empty() {
        let sexpr = Sexpr::List(vec![]);
        assert_eq!(Vec::<i32>::from_sexpr(&sexpr).unwrap(), Vec::<i32>::new());
    }

    #[test]
    fn test_vec_from_sexpr_type_mismatch() {
        let sexpr = Sexpr::Integer(42);
        assert!(Vec::<i32>::from_sexpr(&sexpr).is_err());
    }

    #[test]
    fn test_vec_from_sexpr_item_error() {
        let sexpr = Sexpr::List(vec![
            Sexpr::Integer(1),
            Sexpr::String("not a number".to_string()),
        ]);
        assert!(Vec::<i32>::from_sexpr(&sexpr).is_err());
    }

    // === Sexpr Identity Tests ===

    #[test]
    fn test_sexpr_to_sexpr() {
        let original = Sexpr::Symbol("test".to_string());
        assert_eq!(original.to_sexpr(), original);
    }

    #[test]
    fn test_sexpr_from_sexpr() {
        let original = Sexpr::List(vec![Sexpr::Integer(1)]);
        assert_eq!(Sexpr::from_sexpr(&original).unwrap(), original);
    }

    // === Round Trip Tests ===

    #[test]
    fn test_string_round_trip() {
        let original = "hello world".to_string();
        let sexpr = original.to_sexpr();
        let parsed = String::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_i64_round_trip() {
        let original = -12345i64;
        let sexpr = original.to_sexpr();
        let parsed = i64::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_f64_round_trip() {
        let original = -3.14159f64;
        let sexpr = original.to_sexpr();
        let parsed = f64::from_sexpr(&sexpr).unwrap();
        assert!((original - parsed).abs() < 0.00001);
    }

    #[test]
    fn test_bool_true_round_trip() {
        let original = true;
        let sexpr = original.to_sexpr();
        // Note: round trip requires parsing "t" symbol
        let parsed = bool::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_vec_i32_round_trip() {
        let original = vec![1i32, 2, 3, 4, 5];
        let sexpr = original.to_sexpr();
        let parsed = Vec::<i32>::from_sexpr(&sexpr).unwrap();
        assert_eq!(original, parsed);
    }
}
