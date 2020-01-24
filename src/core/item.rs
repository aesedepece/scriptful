use crate::prelude::*;

#[derive(Debug)]
/// An `Item` is each of the atomic entities that conform a `Script`.
///
/// In other words, a `Script` is no more than an ordered sequence of `Item`s.
pub enum Item<Op>
where
    Op: core::fmt::Debug + core::cmp::Eq,
{
    Operator(Op),
    Value(Value),
}

impl<Op> PartialEq for Item<Op>
where
    Op: core::fmt::Debug + core::cmp::Eq,
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
