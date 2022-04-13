use crate::core::value::Value;

/// An `Item` is each one of the entities that conform a [`Script`][Script].
///
/// In other words, a [`Script`][Script] is no more than an array of `Item`s in the likes of
/// `[Item<Op, Val>]`.
///
/// `Ìtem` does not implement any methods other than implementations of some traits from the
/// [`core`][core] crate.
///
/// [Script]: ../type.Script.html
/// [core]: https://doc.rust-lang.org/nightly/core/
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "use_serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Item<Op, Val = Value>
where
    Op: core::fmt::Debug,
    Val: core::fmt::Debug,
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
    /// [Value]: ../value/enum.Value.html
    /// [Boolean]: ../value/enum.Value.html#variant.Boolean
    /// [Float]: ../value/enum.Value.html#variant.Float
    /// [Integer]: ../value/enum.Value.html#variant.Integer
    /// [String]: ../value/enum.Value.html#variant.String
    Value(Val),
}

#[cfg(test)]
mod tests {
    use crate::core::item::Item::{self, *};
    use crate::core::value::Value::*;

    #[test]
    fn test_eq() {
        let op_item1: Item<u8> = Operator(u8::default());
        let op_item2: Item<u8> = Operator(u8::default());

        assert_eq!(op_item1, op_item2);

        let val_item1: Item<u8> = Value(Integer(i128::default()));
        let val_item2: Item<u8> = Value(Integer(i128::default()));

        assert_eq!(val_item1, val_item2);
    }

    #[test]
    fn test_ne() {
        let op_item1: Item<u8> = Operator(u8::default());
        let op_item2: Item<u8> = Operator(u8::default() + 1);

        assert_ne!(op_item1, op_item2);

        let val_item1: Item<u8> = Value(Integer(i128::default()));
        let val_item2: Item<u8> = Value(Integer(i128::default() + 1));

        assert_ne!(val_item1, val_item2);

        assert_ne!(op_item1, val_item1);
        assert_ne!(op_item1, val_item2);
        assert_ne!(op_item2, val_item1);
        assert_ne!(op_item2, val_item2);
    }
}
