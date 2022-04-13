use alloc::string::String;
#[cfg(feature = "use_serde")]
use serde;

/// An algebraic data type that can be used to represent many types of values that can be present in
/// a [`Stack`][Stack].
///
/// Scriptful supports customization of value types through generics, yet this [`enum`][enum] is
/// provided in expectation that some users will prefer not to define their own type systems but
/// rather use a stock solution.
///
/// `Value` has four variants that should cover most use cases, namely: [`Boolean`][Boolean],
/// [`Float`][Float], [`Integer`][Integer] or [`String`][String].
///
/// The point of `Value` is being used inside [`Script`s][Script] (wrapped in an [`Item`][Item]) or
/// to be pushed into a [`Stack`][Stack].
///
/// `Value` does not implement any methods other than implementations of some traits from the
/// [`core`][core] crate.
///
/// [Stack]: ../stack/struct.Stack.html
/// [enum]: https://doc.rust-lang.org/std/keyword.enum.html
/// [Boolean]: #variant.Boolean
/// [Float]: #variant.Float
/// [Integer]: #variant.Integer
/// [String]: #variant.String
/// [Script]: ../type.Script.html
/// [Item]: ../item/enum.Item.html
/// [known limitations]: ../../#known-limitations
/// [core]: https://doc.rust-lang.org/nightly/core/
#[derive(Clone, Debug, PartialOrd)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Value {
    /// A binary value: either `true` or `false`.
    Boolean(bool),
    /// A signed floating point value.
    Float(f64),
    /// A signed integer value.
    Integer(i128),
    /// A string of characters.
    String(String),
}

impl core::ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        use Value::*;
        match self {
            Boolean(x) => Boolean(!x),
            Float(x) => Float(-x),
            Integer(x) => Integer(-x),
            _ => panic!("Type of {:?} cannot be negated", self),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl core::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Boolean(a), Boolean(b)) => Boolean(a || b),
            (Float(a), Float(b)) => Float(a + b),
            (Float(a), Integer(b)) => Float(a + b as f64),
            (Integer(a), Integer(b)) => Integer(a + b),
            (Integer(a), Float(b)) => Float(a as f64 + b),
            (a, b) => panic!("Types of {:?} and {:?} cannot be added together", a, b),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl core::ops::Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Boolean(a), Boolean(b)) => Boolean(a && b),
            (Float(a), Float(b)) => Float(a * b),
            (Float(a), Integer(b)) => Float(a * b as f64),
            (Integer(a), Integer(b)) => Integer(a * b),
            (Integer(a), Float(b)) => Float(a as f64 * b),
            (a, b) => panic!("Types of {:?} and {:?} cannot be multiplied together", a, b),
        }
    }
}

impl core::ops::Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + !rhs
    }
}

/// Approximate comparison, so as to support comparison of floating point values.
///
/// A floating point values is considered equal to another float or an integer if the difference is
/// less than `10^9`.
impl core::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Boolean(a), Boolean(b)) => a == b,
            (Float(a), Float(b)) => (a - b) * (a - b) < 0.000_000_000_000_000_000_1,
            (Float(a), Integer(b)) => {
                (a - *b as f64) * (a - *b as f64) < 0.000_000_000_000_000_000_1
            }
            (Integer(a), Integer(b)) => a == b,
            (Integer(a), Float(b)) => {
                (*a as f64 - b) * (*a as f64 - *b) < 0.000_000_000_000_000_000_1
            }
            (String(a), String(b)) => a == b,
            _ => false,
        }
    }
}

// The `Float` variant is untested because floating point numbers cannot be trivially nor accurately
// compared.
#[cfg(test)]
mod tests {
    use crate::core::value::Value::*;

    #[test]
    fn test_negation() {
        assert_eq!(!Boolean(true), Boolean(false));
        assert_eq!(!Boolean(false), Boolean(true));
        assert_eq!(!Float(0.), Float(0.));
        assert_eq!(!Float(1.1), Float(-1.1));
        assert_eq!(!Integer(0), Integer(0));
        assert_eq!(!Integer(1), Integer(-1));
    }

    #[test]
    fn test_addition() {
        assert_eq!(Boolean(false) + Boolean(false), Boolean(false));
        assert_eq!(Boolean(false) + Boolean(true), Boolean(true));
        assert_eq!(Boolean(true) + Boolean(false), Boolean(true));
        assert_eq!(Boolean(true) + Boolean(true), Boolean(true));
        assert_eq!(Float(1.1) + Float(2.2), Float(3.3));
        assert_eq!(Float(1.1) + Float(-2.2), Float(-1.1));
        assert_eq!(Float(1.1) + Integer(2), Float(3.1));
        assert_eq!(Float(1.1) + Integer(-2), Float(-0.9));
        assert_eq!(Integer(1) + Integer(2), Integer(3));
        assert_eq!(Integer(1) + Integer(-2), Integer(-1));
        assert_eq!(Integer(1) + Float(2.2), Float(3.2));
        assert_eq!(Integer(1) + Float(-2.1), Float(-1.1));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(Boolean(false) - Boolean(false), Boolean(true));
        assert_eq!(Boolean(false) - Boolean(true), Boolean(false));
        assert_eq!(Boolean(true) - Boolean(false), Boolean(true));
        assert_eq!(Boolean(true) - Boolean(true), Boolean(true));
        assert_eq!(Float(1.1) - Float(2.2), Float(-1.1));
        assert_eq!(Float(1.1) - Float(-2.2), Float(3.3));
        assert_eq!(Float(1.1) - Integer(2), Float(-0.9));
        assert_eq!(Float(1.1) - Integer(-2), Float(3.1));
        assert_eq!(Integer(1) - Integer(2), Integer(-1));
        assert_eq!(Integer(1) - Integer(-2), Integer(3));
        assert_eq!(Integer(1) - Float(2.2), Float(-1.2));
        assert_eq!(Integer(1) - Float(-2.2), Float(3.2));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Boolean(false) * Boolean(false), Boolean(false));
        assert_eq!(Boolean(false) * Boolean(true), Boolean(false));
        assert_eq!(Boolean(true) * Boolean(false), Boolean(false));
        assert_eq!(Boolean(true) * Boolean(true), Boolean(true));
        assert_eq!(Float(1.1) * Float(2.2), Float(2.42));
        assert_eq!(Float(1.1) * Float(-2.2), Float(-2.42));
        assert_eq!(Float(1.1) * Integer(2), Float(2.2));
        assert_eq!(Float(1.1) * Integer(-2), Float(-2.2));
        assert_eq!(Integer(1) * Integer(2), Integer(2));
        assert_eq!(Integer(1) * Integer(-2), Integer(-2));
        assert_eq!(Integer(1) * Float(2.2), Float(2.2));
        assert_eq!(Integer(1) * Float(-2.2), Float(-2.2));
    }

    #[test]
    fn test_comparison() {
        assert_eq!(Boolean(false) == Boolean(false), true);
        assert_eq!(Boolean(false) == Boolean(true), false);
        assert_eq!(Boolean(true) == Boolean(false), false);
        assert_eq!(Boolean(true) == Boolean(true), true);
        assert_eq!(Float(1.1) == Float(1.1), true);
        assert_eq!(Float(1.1) == Float(2.2), false);
        assert_eq!(Float(-1.1) == Float(-1.1), true);
        assert_eq!(Float(-1.1) == Float(-2.2), false);
        assert_eq!(Float(1.1) == Float(-1.1), false);
        assert_eq!(Float(1.) == Integer(1), true);
        assert_eq!(Float(-1.) == Integer(-1), true);
        assert_eq!(Integer(1) == Integer(1), true);
        assert_eq!(Integer(1) == Integer(2), false);
        assert_eq!(Integer(-1) == Integer(-1), true);
        assert_eq!(Integer(-1) == Integer(-2), false);
        assert_eq!(Integer(1) == Integer(-1), false);
        assert_eq!(Integer(1) == Float(1.), true);
        assert_eq!(Integer(-1) == Float(-1.), true);
    }
}
