use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::box_::OverflowClipMarginBox, specified::OverflowClipMargin};

use crate::{cached_value::CachedValue, declarations::length::length_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::OverflowClipMargin", mark)]
pub struct YOverflowClipMargin {
    offset: CachedValue<OverflowClipMargin>,
    visual_box: OverflowClipMarginBox,
}

impl YOverflowClipMargin {
    pub fn new(overflow_clip_margin: OverflowClipMargin) -> Self {
        Self {
            visual_box: overflow_clip_margin.visual_box,
            offset: CachedValue::new(overflow_clip_margin, |overflow_clip_margin, ruby| {
                length_to_value(&overflow_clip_margin.offset.0, ruby)
            }),
        }
    }

    pub fn offset(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.offset.get(ruby)
    }

    pub fn visual_box(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.visual_box {
            OverflowClipMarginBox::ContentBox => ruby.intern("content_box"),
            OverflowClipMarginBox::PaddingBox => ruby.intern("padding_box"),
            OverflowClipMarginBox::BorderBox => ruby.intern("border_box"),
        }
    }
}

impl DataTypeFunctions for YOverflowClipMargin {
    fn mark(&self, marker: &gc::Marker) {
        self.offset.mark(marker);
    }
}
