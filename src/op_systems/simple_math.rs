use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Add,
    Equal,
    Not,
    Sub,
}

pub fn simple_math_op_sys(stack: &mut Stack, operator: &OpCode) {
    use crate::prelude::Value::*;

    match operator {
        OpCode::Add => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a + b);
        }
        OpCode::Equal => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(Boolean(a == b));
        }
        OpCode::Not => {
            let x = stack.pop();
            stack.push(!x);
        }
        OpCode::Sub => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a - b);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::op_systems::simple_math::{simple_math_op_sys, OpCode};
    use crate::prelude::*;

    #[test]
    fn test_one_plus_one_equals_two() {
        let machine = &mut Machine::new(&simple_math_op_sys);

        machine.run_script(&[
            Item::Value(Value::Integer(1)),
            Item::Value(Value::Integer(1)),
            Item::Operator(OpCode::Add),
            Item::Value(Value::Integer(2)),
        ]);
    }
}
