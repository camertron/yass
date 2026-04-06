use style::values::computed::OverflowWrap;
use magnus::{Ruby, typed_data, value::Id};

#[magnus::wrap(class = "Yass::Declarations::OverflowWrap")]
pub struct YOverflowWrap {
    overflow_wrap: OverflowWrap
}

impl YOverflowWrap {
    pub fn new(overflow_wrap: OverflowWrap) -> Self {
        Self { overflow_wrap }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.overflow_wrap {
            OverflowWrap::Normal => ruby.intern("normal"),
            OverflowWrap::BreakWord => ruby.intern("break_word"),
            OverflowWrap::Anywhere => ruby.intern("anywhere"),
        }
    }
}
