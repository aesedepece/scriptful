use alloc::vec::Vec;

use crate::core::value::Value;

pub mod condition_stack;
pub mod item;
pub mod machine;
pub mod stack;
pub mod value;

/// A simple alias for referring an ordered sequence of [`Item`][Item]s of definite length.
///
/// [Item]: item
pub type Script<Op, Val = Value> = Vec<item::Item<Op, Val>>;

/// Convenient type alias for `&Script<Op, Val>`.
pub type ScriptRef<'a, Op, Val = Value> = &'a [item::Item<Op, Val>];

/// Generic error trait for all errors defined in this crate.
pub trait Error: core::fmt::Debug + PartialEq {
    fn from_str(input: &str) -> Self;
}
