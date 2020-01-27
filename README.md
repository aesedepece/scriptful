# ![Scriptful](logo.png)

[![Build Status](https://travis-ci.com/aesedepece/scriptful.svg?branch=master)](https://travis-ci.com/aesedepece/scriptful)
[![Crate](https://img.shields.io/crates/v/scriptful.svg)](https://crates.io/crates/scriptful)
[![Docs](https://docs.rs/scriptful/badge.svg)](https://docs.rs/scriptful)
![License](https://img.shields.io/crates/l/scriptful.svg)

___Scriptful_ is a minimalist `no_std` stack machine for interpreting scripts written with domain specific interpreted languages.__

This library is heavily inspired by the [Forth] programming language and [Script][BitcoinScript] (the scripting language in Bitcoin).

## General design

The whole library is built around these concepts:

- __[Stack]__: an ordered sequence of values that can be operated in a [LIFO]-alike way.
- __[Item]__: either a `Value` (a piece of data to be pushed into the stack) or an `Operator` (the descriptor for an action that operates on the topmost items in the stack).
- __Type system__: an [`enum`][enum] whose variants are all the possible data types allowed in a [`Stack`][Stack].
- __[Operator system]__: a function that decides how each operator will mutate a given stack.
- __[Script]__: an ordered sequence of items (values and operators) that can be passed to an operator system for operating on a given stack.
- __[Machine]__: a convenient wrapper around a stack that enables multiple modes of operation.

Using this library is as easy as:

1. Defining your own set of operators, or using any of the ones that come bundled in the [`op_systems`][Operator system] module.
2. Defining your own type system, or using the [`Value`][Value] type system that comes bundled in the [`core::value`][Value] module.
3. Defining your own [operator system][Operator system] function, or using any of the ones that come bundled in the [`op_systems`][Operator system] module.
4. Instantiating a [machine][Machine] with a reference to your operator system.
5. Composing a [script][Script] and running it in the machine.

## Quick example

```rust
use scriptful::prelude::*;
use scriptful::core::value::Value::*;

// You can define your own operators.
#[derive(Debug, PartialEq, Eq)]
enum MyOperator {
    Add,
    Equal,
    Sub,
}

// An operator system decides what to do with the stack when each operator is applied on it.
fn my_operator_system(stack: &mut Stack, operator: &MyOperator) {
    match operator {
        MyOperator::Add => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a + b);
        }
        MyOperator::Equal => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(Boolean(a == b));
        }
        MyOperator::Sub => {
            let a = stack.pop();
            let b = stack.pop();
            stack.push(a - b);
        }
    }
}

// Instantiate the machine with a reference to your operator system.
let mut machine = Machine::new(&my_operator_system);

// Run a script that simply adds 1 and 2.
let result = machine.run_script(&[
    Item::Value(Integer(1)),
    Item::Value(Integer(2)),
    Item::Operator(MyOperator::Add),
]);

// The result should unsurprisingly be 3.
assert_eq!(result, Some(&Integer(3)));
```

## Known limitations

- [Stacks][Stack] are currently implemented using a fixed-length, actually stack-allocated vectors using [smallvec].
Thus the `main` sub-stack is limited to 64 values, and the `alt` sub-stack can only hold up to 8.
- _Beware of unwraps!_ This is a proof-of-concept and it is modelled to panic upon errors.
Making the library safe for production usage is in the near horizon though.
- The possible value types that can be pushed into the [Stack] is not generic nor customizable.
Such feature will only be added if someone actually requests it.

## License

Scriptful is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE] and [LICENSE-MIT], and [COPYRIGHT] for details.

[Forth]: https://en.wikipedia.org/wiki/Forth_(programming_language)
[BitcoinScript]: https://en.bitcoin.it/wiki/Script
[LIFO]: https://en.wikipedia.org/wiki/Stack_(abstract_data_type)
[Stack]: https://docs.rs/scriptful/latest/scriptful/core/stack/struct.Stack.html
[Item]: https://docs.rs/scriptful/latest/scriptful/core/item/enum.Item.html
[Operator system]: https://docs.rs/scriptful/latest/scriptful/op_systems/
[Script]: https://docs.rs/scriptful/latest/scriptful/core/type.Script.html
[Machine]: https://docs.rs/scriptful/latest/scriptful/core/machine/struct.Machine.html
[smallvec]: https://crates.io/crates/smallvec
[Value]: core/value/enum.Value.html
[enum]: https://doc.rust-lang.org/std/keyword.enum.html
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
[COPYRIGHT]: COPYRIGHT