//! ___Scriptful_ is a minimalistic `no_std` stack machine for executing domain specific interpreted languages.__
//!
//! This library is heavily inspired by [Forth] and [Script][BitcoinScript], the scripting language in Bitcoin.
//!
//! # General design
//!
//! The whole library is built around four main concepts:
//!
//! - __[Stack]__: an ordered sequence of values that can be operated in a [LIFO]-alike way.
//! - __[Item]__: either a `Value` (a piece of data to be pushed into the stack) or an `Operator` (the descriptor for an action that operates on the topmost items in the stack).
//! - __[Operator system]__: a function that decides how each operator will mutate a given stack.
//! - __[Script]__: an ordered sequence of items (values and operators) that can be passed to an operator system for operating on a given stack.
//! - __[Machine]__: a convenient wrapper around a stack that enables multiple modes of operation.
//!
//! Using this library is as easy as:
//!
//! 1. Defining your own set of operators, or using any of the ones that come bundled in the [`op_systems`][Operator system] module.
//! 2. Defining your own [operator system][Operator system] function, or using any of the ones that come bundled in the [`op_systems`][Operator system] module.
//! 3. Instantiating a [machine][Machine] with a reference to your operator system.
//! 4. Composing a [script][Script] and running it in the machine.
//!
//! # Quick example
//!
//! ```rust
//! use scriptful::prelude::*;
//! use scriptful::prelude::Value::*;
//!
//! // You can define your own operators.
//! #[derive(Debug, PartialEq, Eq)]
//! enum MyOperator {
//!     Add,
//!     Equal,
//!     Sub,
//! }
//!
//! // An operator system decides what to do with the stack when each operator is applied on it.
//! fn my_operator_system(stack: &mut Stack, operator: &MyOperator) {
//!     match operator {
//!         MyOperator::Add => {
//!             let a = stack.pop();
//!             let b = stack.pop();
//!             stack.push(a + b);
//!         }
//!         MyOperator::Equal => {
//!             let a = stack.pop();
//!             let b = stack.pop();
//!             stack.push(Value::Boolean(a == b));
//!         }
//!         MyOperator::Sub => {
//!             let a = stack.pop();
//!             let b = stack.pop();
//!             stack.push(a - b);
//!         }
//!     }
//! }
//!
//! // Instantiate the machine with a reference to your operator system.
//! let mut machine = Machine::new(&my_operator_system);
//!
//! // Run a script that simply adds 1 and 2.
//! let result = machine.run_script(&[
//!     Item::Value(Integer(1)),
//!     Item::Value(Integer(2)),
//!     Item::Operator(MyOperator::Add),
//! ]);
//!
//! // The result should unsurprisingly be 3.
//! assert_eq!(result, Some(&Integer(3)));
//! ```
//!
//! # Known limitations
//!
//! - [Stacks][Stack] are currently implemented using a fixed-length, actually stack-allocated vectors using [smallvec].
//! Thus the `main` sub-stack is limited to 64 values, and the `alt` sub-stack can only hold up to 8.
//! - _Beware of unwraps!_ This is a proof-of-concept and it is modelled to panic upon errors.
//! Making the library safe for production usage is in the near horizon though.
//!
//! # License
//!
//! Scriptful is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
//!
//! See [LICENSE-APACHE] and [LICENSE-MIT], and [COPYRIGHT] for details.
//!
//! [Forth]: https://en.wikipedia.org/wiki/Forth_(programming_language)
//! [BitcoinScript]: https://en.bitcoin.it/wiki/Script
//! [LIFO]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
//! [Stack]: core/stack/struct.Stack.html
//! [Item]: core/item/enum.Item.html
//! [Operator system]: op_systems/
//! [Script]: core/type.Script.html
//! [Machine]: core/machine/struct.Machine.html
//! [LICENSE-APACHE]: https://github.com/aesedepece/scriptful/blob/master/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/aesedepece/scriptful/blob/master/LICENSE-MIT
//! [COPYRIGHT]: https://github.com/aesedepece/scriptful/blob/master/COPYRIGHT
//! [smallvec]: https://crates.io/crates/smallvec

#![no_std]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

/// The core of this library.
///
/// Provides all the [`Item`][Item], [`Stack`][Stack], [`Machine`][Machine] and [`Value`][Value] goodness.
///
/// [Item]: item/
/// [Stack]: stack/
/// [Machine]: machine/
/// [Value]: value/
pub mod core;
/// Some ready-to-use operator systems that may be useful for _someone_, _somewhere_, _somewhen_.
pub mod op_systems;

/// Re-exports the most frequently used parts of this library so that they can be used more conveniently.
pub mod prelude {
    pub use crate::core::{item::Item, machine::Machine, stack::Stack, Script};
}
