use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::ContentDistribution;

use crate::declarations::align_flags;

#[magnus::wrap(class = "Yass::Declarations::AlignContent")]
pub struct YAlignContent {
    content_distribution: ContentDistribution
}

impl YAlignContent {
    pub fn new(content_distribution: ContentDistribution) -> Self {
        Self { content_distribution }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Id> {
        align_flags::to_id(rb_self.content_distribution.primary().value(), ruby)
    }
}
