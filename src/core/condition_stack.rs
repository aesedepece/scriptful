// ConditionStack implementation from bitcoin-core
// https://github.com/bitcoin/bitcoin/blob/505ba3966562b10d6dd4162f3216a120c73a4edb/src/script/interpreter.cpp#L272
// https://bitslog.com/2017/04/17/new-quadratic-delays-in-bitcoin-scripts/
/** A data type to abstract out the condition stack during script execution.
*
* Conceptually it acts like a vector of booleans, one for each level of nested
* IF/THEN/ELSE, indicating whether we're in the active or inactive branch of
* each.
*
* The elements on the stack cannot be observed individually; we only need to
* expose whether the stack is empty and whether or not any false values are
* present at all. To implement OP_ELSE, a toggle_top modifier is added, which
* flips the last value without returning it.
*
* This uses an optimized implementation that does not materialize the
* actual stack. Instead, it just stores the size of the would-be stack,
* and the position of the first false value in it.
 */
#[derive(Debug)]
pub struct ConditionStack {
    stack_size: u32,
    first_false_pos: u32,
}

impl Default for ConditionStack {
    fn default() -> Self {
        Self {
            stack_size: 0,
            first_false_pos: Self::NO_FALSE,
        }
    }
}

impl ConditionStack {
    const NO_FALSE: u32 = u32::MAX;

    pub fn is_empty(&self) -> bool {
        self.stack_size == 0
    }

    pub fn all_true(&self) -> bool {
        self.first_false_pos == Self::NO_FALSE
    }

    pub fn push_back(&mut self, b: bool) {
        if (self.first_false_pos == Self::NO_FALSE) && !b {
            // The stack consists of all true values, and a false is added.
            // The first false value will appear at the current size.
            self.first_false_pos = self.stack_size;
        }

        self.stack_size += 1;
    }

    pub fn pop_back(&mut self) -> Option<()> {
        if self.stack_size == 0 {
            return None;
        }

        self.stack_size -= 1;
        if self.first_false_pos == self.stack_size {
            // When popping off the first false value, everything becomes true.
            self.first_false_pos = Self::NO_FALSE;
        }

        Some(())
    }

    pub fn toggle_top(&mut self) -> Option<()> {
        if self.stack_size == 0 {
            return None;
        }

        if self.first_false_pos == Self::NO_FALSE {
            // The current stack is all true values; the first false will be the top.
            self.first_false_pos = self.stack_size - 1;
        } else if self.first_false_pos == self.stack_size - 1 {
            // The top is the first false value; toggling it will make everything true.
            self.first_false_pos = Self::NO_FALSE;
        } else {
            // There is a false value, but not on top. No action is needed as toggling
            // anything but the first false value is unobservable.
        }

        Some(())
    }
}
