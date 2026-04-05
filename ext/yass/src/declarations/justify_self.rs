use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::SelfAlignment;

use crate::declarations::align_flags;

#[magnus::wrap(class = "Yass::Declarations::JustifySelf")]
pub struct YJustifySelf {
    self_alignment: SelfAlignment
}

impl YJustifySelf {
    pub fn new(self_alignment: SelfAlignment) -> Self {
        Self { self_alignment }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Id> {
        align_flags::to_id(rb_self.self_alignment.0.value(), ruby)
    }
}
