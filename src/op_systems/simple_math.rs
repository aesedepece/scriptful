use crate::prelude::*;

/// Frequently used mathematical operators.
#[derive(Debug, PartialEq, Eq)]
pub enum MathOperator {
    /// Addition of two numbers (`a + b`).
    Add,
    /// Equivalence of two numbers (`a == b`).
    Equal,
    /// Multiplication of two numbers (`a * b`)
    Mul,
    /// Negation of one number (`-a`).
    Not,
    /// Subtraction of two numbers (`a - b`).
    Sub,
}

/// A simple operator system that decides how each of the variants of [`MathOperator`][MathOperator]
/// trigger push and pulls on the [`Stack`][Stack] inside a [`Machine`][Machine].
///
/// [MathOperator]: enum.MathOperator.html
/// [Stack]: ../../core/stack/struct.Stack.html
/// [Machine]: ../../core/machine/struct.Machine.html
pub fn simple_math_op_sys(stack: &mut Stack, operator: &MathOperator) {
    use crate::core::value::Value::*;

    match operator {
        MathOperator::Add => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a + b);
        }
        MathOperator::Equal => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(Boolean(a == b));
        }
        MathOperator::Mul => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a * b);
        }
        MathOperator::Not => {
            let x = stack.pop();
            stack.push(!x);
        }
        MathOperator::Sub => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a - b);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::value::Value::*;
    use crate::op_systems::simple_math::{simple_math_op_sys, MathOperator};
    use crate::prelude::Item::*;
    use crate::prelude::*;

    #[test]
    fn test_one_plus_one_equals_two() {
        let machine = &mut Machine::new(&simple_math_op_sys);

        let result = machine
            .run_script(&[
                Value(Integer(1)),
                Value(Integer(1)),
                Operator(MathOperator::Add),
                Value(Integer(2)),
                Operator(MathOperator::Equal),
            ])
            .unwrap();

        assert_eq!(result, &Boolean(true));
    }
}
