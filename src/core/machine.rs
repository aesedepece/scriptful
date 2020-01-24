use crate::prelude::*;

pub struct Machine<'a, Op> {
    op_sys: &'a dyn Fn(&mut Stack, &Op),
    stack: Stack,
}

impl<'a, Op> core::fmt::Debug for Machine<'a, Op> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.stack)
    }
}

impl<'a, Op> Machine<'a, Op>
where
    Op: core::fmt::Debug + core::cmp::Eq,
{
    pub fn new(op_sys: &'a dyn Fn(&mut Stack, &Op)) -> Self {
        Self {
            op_sys,
            stack: Stack::default(),
        }
    }

    pub fn operate(&mut self, item: &Item<Op>) {
        match item {
            Item::Operator(operator) => (self.op_sys)(&mut self.stack, operator),
            Item::Value(value) => self.stack.push((*value).clone()),
        }
    }

    pub fn run_script(&mut self, script: &[Item<Op>]) -> &Value {
        for item in script {
            self.operate(item);
        }

        self.stack.topmost()
    }
}
