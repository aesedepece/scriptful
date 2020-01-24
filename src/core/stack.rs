//! An ordered sequence of values that can be operated in a [LIFO]-alike way.
//!
//! This module provides the [`Stack`][Stack] struct which in turn is the core of the [`Machine`][Machine] abstraction.
//!
//! For more details on [`Stack`][Stack], how it works and which methods does it provide, please go to the [`struct Stack` documentation][Stack].
//!
//! [LIFO]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
//! [Stack]: core/stack/struct.Stack.html
//! [Script]: core/type.Script.html
//! [Machine]: core/machine

use crate::prelude::*;
use smallvec::SmallVec;

#[derive(Debug, Default)]
pub struct Stack {
    main: SmallVec<[Value; 64]>,
    alt: SmallVec<[Value; 64]>,
}

/// An ordered sequence of values that can be operated in a [LIFO]-alike way.
///
/// Every `Stack` actually comprises two sequences of values: the `main` sub-stack and the `alt` sub-stack.
///
/// As its name indicates, the `main` sub-stack is the one you operate by default.
/// That is, the `alt` sub-stack cannot be operated directly — you can only move values between both sub-stacks with the [`pop_into_alt`][pop_into_alt] and [`push_from_alt`][push_from_alt] methods.
/// The `alt` sub-stack is therefore limited for usage as a sort of _clipboard_ for values.
///
/// [LIFO]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
/// [pop_into_alt]: #method.pop_into_alt
/// [push_from_alt]: #method.push_from_alt
impl Stack {
    /// Returns the number of values in the `main` sub-stack, also referred to as its 'length'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::prelude::Value::*;
    ///
    /// let mut stack = Stack::default();
    /// assert_eq!(stack.length(), 0);
    ///
    /// stack.push(Integer(i128::default()));
    /// assert_eq!(stack.length(), 1);
    ///
    /// stack.pop();
    /// assert_eq!(stack.length(), 0);
    /// ```
    pub fn length(&self) -> usize {
        self.main.len()
    }

    /// Removes the topmost value in the `main` sub-stack and returns it.
    ///
    /// # Panics
    /// Panics if there are no values left in the `main` stack.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::prelude::Value::*;
    ///
    /// let value = Integer(i128::default());
    /// let mut stack = Stack::default();
    /// stack.push(value.clone());
    /// let popped = stack.pop();
    ///
    /// assert_eq!(value, popped);
    /// ```
    pub fn pop(&mut self) -> Value {
        self.main.pop().unwrap()
    }

    /// Similar to [`pop`][pop], but instead of returning the popped value, it pushes it to the `alt` sub-stack.
    ///
    /// # Panics
    /// Panics if there are no values left in the `main` stack.
    ///
    /// [pop]: #method.pop
    pub fn pop_into_alt(&mut self) {
        self.alt.push(self.main.pop().unwrap())
    }

    /// Puts a value on top of the stack.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::prelude::Value::*;
    ///
    /// let value = Integer(i128::default());
    /// let mut stack = Stack::default();
    /// stack.push(value.clone());
    /// let topmost = stack.topmost();
    ///
    /// assert_eq!(value, *topmost);
    /// ```
    pub fn push(&mut self, item: Value) {
        self.main.push(item)
    }

    /// Similar to [`push`][push], but instead of receiving the value to be pushed as an argument, it pops it from the `alt` sub-stack.
    ///
    /// # Panics
    /// Panics if there are no values left in the `alt` sub-stack.
    ///
    /// [push]: #method.push
    pub fn push_from_alt(&mut self) {
        self.main.push(self.alt.pop().unwrap())
    }

    /// Returns a reference to the last value in the `main` sub-stack.
    ///
    /// # Panics
    /// Panics if there are no values left in the `alt` sub-stack.
    /// # Examples
    ///
    /// ```rust
    /// use scriptful::prelude::*;
    /// use scriptful::prelude::Value::*;
    ///
    /// let value = Integer(i128::default());
    /// let mut stack = Stack::default();
    /// stack.push(value.clone());
    /// let topmost = stack.topmost();
    ///
    /// assert_eq!(value, *topmost);
    /// ```
    pub fn topmost(&self) -> &Value {
        self.main.last().unwrap()
    }
}
