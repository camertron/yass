use style::values::computed::Overflow;
use magnus::{Ruby, typed_data, value::Id};

use crate::declarations::overflow::overflow_to_id;

#[magnus::wrap(class = "Yass::Declarations::OverflowX")]
pub struct YOverflowX {
    overflow: Overflow
}

impl YOverflowX {
    pub fn new(overflow: Overflow) -> Self {
        Self { overflow }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        overflow_to_id(&rb_self.overflow, ruby)
    }
}
