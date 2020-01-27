use crate::prelude::*;

/// An `Item` is each of the atomic entities that conform a [`Script`][Script].
///
/// In other words, a [`Script`][Script] is no more than an array of `Item`s in the likes of
/// `[Item<Op>]`.
///
/// `Ìtem` does not implement any methods other than implementations of some traits from the
/// [`core`][core] crate.
///
/// [Script]: ../type.Script.html
/// [core]: https://doc.rust-lang.org/nightly/core/
#[derive(Debug)]
pub enum Item<Op, Val = Value>
where
    Op: core::fmt::Debug + core::cmp::Eq,
    Val: core::fmt::Debug + core::cmp::PartialEq,
{
    /// An operator code, either a variant of an user-defined [`enum`][enum] containing different
    /// operator identifiers, or any of the ones found in the [`op_systems`][op_systems] module.
    ///
    /// [enum]: https://doc.rust-lang.org/std/keyword.enum.html
    /// [op_systems]: ../../op_systems/
    Operator(Op),
    /// A value, either a variant of an user-defined [`enum`][enum] representing a type system, or
    /// an instance of any of the variants of [`Value`][Value], i.e. [`Boolean`][Boolean],
    /// [`Float`][Float], [`Integer`][Integer] or [`String`][String].
    ///
    /// [enum]: https://doc.rust-lang.org/std/keyword.enum.html
    ///  [Value]: ../value/enum.Value.html
    /// [Boolean]: ../value/enum.Value.html#variant.Boolean
    /// [Float]: ../value/enum.Value.html#variant.Float
    /// [Integer]: ../value/enum.Value.html#variant.Integer
    /// [String]: ../value/enum.Value.html#variant.String
    Value(Val),
}

impl<Op, Val> PartialEq for Item<Op, Val>
where
    Op: core::fmt::Debug + core::cmp::Eq,
    Val: core::fmt::Debug + core::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Operator(op), Self::Operator(other_op)) => op == other_op,
            (Self::Value(value), Self::Value(other_value)) => value == other_value,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_eq() {
        let op_item1: Item<u8> = Item::Operator(u8::default());
        let op_item2: Item<u8> = Item::Operator(u8::default());

        assert_eq!(op_item1, op_item2);

        let val_item1: Item<u8> = Item::Value(Value::Integer(i128::default()));
        let val_item2: Item<u8> = Item::Value(Value::Integer(i128::default()));

        assert_eq!(val_item1, val_item2);
    }

    #[test]
    fn test_ne() {
        let op_item1: Item<u8> = Item::Operator(u8::default());
        let op_item2: Item<u8> = Item::Operator(u8::default() + 1);

        assert_ne!(op_item1, op_item2);

        let val_item1: Item<u8> = Item::Value(Value::Integer(i128::default()));
        let val_item2: Item<u8> = Item::Value(Value::Integer(i128::default() + 1));

        assert_ne!(val_item1, val_item2);

        assert_ne!(op_item1, val_item1);
        assert_ne!(op_item1, val_item2);
        assert_ne!(op_item2, val_item1);
        assert_ne!(op_item2, val_item2);
    }
}
