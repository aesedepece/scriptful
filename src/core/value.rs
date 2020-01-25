/// The only data type that can be actually pushed or pulled from a [`Stack`][Stack].
///
/// `Value` has four variants that should cover most use cases, namely: [`Boolean`][Boolean],
/// [`Float`][Float], [`Integer`][Integer] or [`String`][String].
///
/// The point of `Value` is being used inside [`Script`s][Script] (wrapped in an [`Item`][Item]) or
/// to be pushed into a [`Stack`][Stack].
///
/// As explained in [known limitations], _"the possible value types that can be pushed into the
/// Stack is not generic nor customizable. Such feature will only be added if someone actually
/// equests it"_.
///
/// `Value` does not implement any methods other than implementations of some traits from the
/// [`core`][core] crate.
///
/// [Stack]: ../stack/struct.Stack.html
/// [Boolean]: #variant.Boolean
/// [Float]: #variant.Float
/// [Integer]: #variant.Integer
/// [String]: #variant.String
/// [Script]: ../type.Script.html
/// [Item]: ../item/enum.Item.html
/// [known limitations]: ../../#known-limitations
/// [core]: https://doc.rust-lang.org/nightly/core/
#[derive(Clone, Debug)]
pub enum Value {
    /// A binary value: either `true` or `false`.
    Boolean(bool),
    /// A signed floating point value.
    Float(f64),
    /// A signed integer value.
    Integer(i128),
    /// A string of characters.
    String(&'static str),
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

impl core::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Boolean(a), Boolean(b)) => a == b,
            (Float(_), _) | (_, Float(_)) => panic!("Float values cannot be compared"),
            (Integer(a), Integer(b)) => a == b,
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
        assert_eq!(!Integer(0), Integer(0));
        assert_eq!(!Integer(1), Integer(-1));
    }

    #[test]
    fn test_addition() {
        assert_eq!(Boolean(false) + Boolean(false), Boolean(false));
        assert_eq!(Boolean(false) + Boolean(true), Boolean(true));
        assert_eq!(Boolean(true) + Boolean(false), Boolean(true));
        assert_eq!(Boolean(true) + Boolean(true), Boolean(true));
        assert_eq!(Integer(1) + Integer(2), Integer(3));
        assert_eq!(Integer(1) + Integer(-2), Integer(-1));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(Boolean(false) - Boolean(false), Boolean(true));
        assert_eq!(Boolean(false) - Boolean(true), Boolean(false));
        assert_eq!(Boolean(true) - Boolean(false), Boolean(true));
        assert_eq!(Boolean(true) - Boolean(true), Boolean(true));
        assert_eq!(Integer(1) - Integer(2), Integer(-1));
        assert_eq!(Integer(1) - Integer(-2), Integer(3));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Boolean(false) * Boolean(false), Boolean(false));
        assert_eq!(Boolean(false) * Boolean(true), Boolean(false));
        assert_eq!(Boolean(true) * Boolean(false), Boolean(false));
        assert_eq!(Boolean(true) * Boolean(true), Boolean(true));
        assert_eq!(Integer(1) * Integer(2), Integer(2));
        assert_eq!(Integer(1) * Integer(-2), Integer(-2));
    }

    #[test]
    fn test_comparison() {
        assert_eq!(Boolean(false) == Boolean(false), true);
        assert_eq!(Boolean(false) == Boolean(true), false);
        assert_eq!(Boolean(true) == Boolean(false), false);
        assert_eq!(Boolean(true) == Boolean(true), true);
        assert_eq!(Integer(1) == Integer(1), true);
        assert_eq!(Integer(1) == Integer(2), false);
        assert_eq!(Integer(-1) == Integer(-1), true);
        assert_eq!(Integer(-1) == Integer(-2), false);
        assert_eq!(Integer(1) == Integer(-1), false);
    }
}
