pub mod item;
pub mod machine;
pub mod stack;
pub mod value;

/// A simple alias for referring an ordered sequence of [`Item`][Item]s of definite length.
///
/// [Item]: item
pub type Script<Op, Val> = [item::Item<Op, Val>];
