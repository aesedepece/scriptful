use crate::prelude::*;

/// A convenient wrapper around [`Stack`][Stack] providing multiple operation methods, i.e.
/// xecuting scripts by evaluating operators and pushing values into the stack.
///
/// This is the preferred way to interact with [`Stack`s][Stack], as they do not support operators,
/// [`Item`s][Item], and other abstractions.
///
/// [Stack]: ../stack/struct.Stack.html
/// [Item]: ../item/enum.Item.html
pub struct Machine<'a, Op, Val>
where
    Val: core::fmt::Debug + core::cmp::PartialEq,
{
    op_sys: &'a dyn Fn(&mut Stack<Val>, &Op),
    stack: Stack<Val>,
}

impl<'a, Op, Val> Machine<'a, Op, Val>
where
    Op: core::fmt::Debug + core::cmp::Eq,
    Val: core::fmt::Debug + core::cmp::PartialEq + core::clone::Clone,
{
    /// A simple factory that helps constructing a `Machine` around a existing operator system, be
    /// it user defined or any of the ones in the [`op_systems`][op_systems] module.
    ///
    /// This method initializes the internal stack to be empty.
    ///
    /// [op_systems]: ../../op_systems/
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::op_systems::simple_math::simple_math_op_sys;
    ///
    /// // Instantiate the machine with a reference to your operator system, or any of the ones in
    /// // the `op_systems` module.
    /// let machine = Machine::new(&simple_math_op_sys);
    ///
    /// // Make sure the stack is initialized to be empty.
    /// assert_eq!(machine.stack_length(), 0);
    /// ```
    pub fn new(op_sys: &'a dyn Fn(&mut Stack<Val>, &Op)) -> Self {
        Self {
            op_sys,
            stack: Stack::<Val>::default(),
        }
    }

    /// The simplest way to make a `Machine` evaluate a single [`Item`][Item], be it a `Value` or
    /// `Operator`.
    ///
    /// Note that the preferred way to evaluate multiple [`Item`s][Item] at once is through the
    /// [`run_script`][run_script] method, which instead of single [`Item`s][Item] takes a
    /// [`Script`][Script], i.e. an array of [`Item`s][Item].
    ///
    /// # Panics
    ///
    /// Operating on a `Machine` that has an empty [`Stack`][Stack] can cause a panic if the
    /// [`Item`][Item] is an operator that tries to pop from it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::core::value::Value::*;
    /// use scriptful::op_systems::simple_math::*;
    ///
    /// // Instantiate the machine with a reference to your operator system, or any of the ones in
    /// // the `op_systems` module.    
    /// let mut machine = Machine::new(&simple_math_op_sys);
    ///
    /// // Operating a `Value::Integer(1)` should simply push it into the stack.
    /// let result = machine.operate(&Item::Value(Integer(1)));
    /// // Make sure the value gets pushed.
    /// assert_eq!(result, Some(&Integer(1)));
    /// // The length of the stack should be 1.
    /// assert_eq!(machine.stack_length(), 1);
    ///
    /// // Operating a `Value::Integer(2)` should simply push it into the stack.
    /// let result = machine.operate(&Item::Value(Integer(2)));
    /// // Make sure the value gets pushed.
    /// assert_eq!(result, Some(&Integer(2)));
    /// // The length of the stack should be 2.
    /// assert_eq!(machine.stack_length(), 2);
    ///
    /// // Operating an `MathOperator::Add` should pop the two topmost values in the stack, add them
    /// // together, and push the result back into the stack.
    /// let result = machine.operate(&Item::Operator(MathOperator::Add));
    /// // Make sure the result is 3.
    /// assert_eq!(result, Some(&Integer(3)));
    /// // The final length of the stack should be 1 again.
    /// assert_eq!(machine.stack_length(), 1);
    /// ```
    ///
    /// [Item]: ../item/enum.Item.html
    /// [run_script]: #method.run_script
    /// [Script]: ../type.Script.html
    /// [Stack]: ../stack/struct.Stack.html
    pub fn operate(&mut self, item: &Item<Op, Val>) -> Option<&Val> {
        match item {
            Item::Operator(operator) => (self.op_sys)(&mut self.stack, operator),
            Item::Value(value) => self.stack.push((*value).clone()),
        }

        self.stack.topmost()
    }

    /// Evaluates a [`Script`][Script] in the context of a `Machine`.
    ///
    /// # Panics
    ///
    /// Operating on a `Machine` that has an empty [`Stack`][Stack] can cause a panic if any of the
    /// [`Item`s][Item] in the [`Script`][Script] is an operator that tries to pop from it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::core::value::Value::*;
    /// use scriptful::op_systems::simple_math::*;
    ///
    /// // Instantiate the machine with a reference to your operator system, or any of the ones in
    /// // the `op_systems` module.
    /// let mut machine = Machine::new(&simple_math_op_sys);
    ///
    /// // Run a script that simply adds 1 and 2.
    /// let result = machine.run_script(&[
    ///    Item::Value(Integer(1)),
    ///    Item::Value(Integer(2)),
    ///    Item::Operator(MathOperator::Add),
    /// ]);
    ///
    /// // The result should unsurprisingly be 3.
    /// assert_eq!(result, Some(&Integer(3)));
    /// // The final length of the stack should be 1.
    /// assert_eq!(machine.stack_length(), 1);
    /// ```
    ///
    /// [Script]: ../type.Script.html
    /// [Stack]: ../stack/struct.Stack.html
    /// [Item]: ../item/enum.Item.html
    pub fn run_script(&mut self, script: &Script<Op, Val>) -> Option<&Val> {
        for item in script {
            self.operate(item);
        }

        self.stack.topmost()
    }

    /// Returns the number of [`Value`s][Value] currently in the [`Stack`][Stack].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::core::value::Value::*;
    /// use scriptful::op_systems::simple_math::*;
    ///
    /// // Instantiate the machine with a reference to your operator system, or any of the ones in
    /// // the `op_systems` module.
    /// let mut machine = Machine::new(&simple_math_op_sys);
    ///
    /// // Run a script that simply pushes 4 values into the stack.
    /// machine.run_script(&[
    ///     Item::Value(Boolean(true)),
    ///     Item::Value(Float(3.141592)),
    ///     Item::Value(Integer(1337)),
    ///     Item::Value(String("foo"))
    /// ]);
    ///
    /// // The final length of the stack should be 4.
    /// assert_eq!(machine.stack_length(), 4);
    /// ```
    ///
    /// [Value]: ../value/enum.Value.html
    /// [Stack]: ../stack/struct.Stack.html
    pub fn stack_length(&self) -> usize {
        self.stack.length()
    }
}

/// Debugging of `Machine` only shows the internal [`Stack`][Stack], but not the operator system.
///
/// The explanation for this is straightforward: how do you print a dynamic reference to a function?
///
/// [Stack]: ../stack/struct.Stack.html
impl<'a, Op, Val> core::fmt::Debug for Machine<'a, Op, Val>
where
    Val: core::fmt::Debug + core::cmp::PartialEq,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self.stack)
    }
}
