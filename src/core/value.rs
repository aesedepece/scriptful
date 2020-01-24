#[derive(Clone, Debug)]
pub enum Value {
    Boolean(bool),
    Float(f64),
    Integer(i128),
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

impl core::ops::Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Float(a), Float(b)) => Float(a + b),
            (Float(a), Integer(b)) => Float(a + b as f64),
            (Integer(a), Integer(b)) => Integer(a + b),
            (Integer(a), Float(b)) => Float(a as f64 + b),
            (a, b) => panic!("Types of {:?} and {:?} cannot be added together", a, b),
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
