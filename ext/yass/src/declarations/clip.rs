use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{ClipRect, ClipRectOrAuto, length::LengthPercentageOrAuto}, specified::Length};

use crate::{cached_value::CachedValue, declarations::length::length_to_value};

type ClipSide = LengthPercentageOrAuto<Length>;
type ClipRectValue = ClipRect<ClipSide>;
type ClipValue = ClipRectOrAuto<ClipRectValue>;

fn make_clip_value(value: &ClipValue, ruby: &Ruby) -> Value {
    match value {
        ClipRectOrAuto::Auto => YClipAuto::new().into_value_with(ruby),
        ClipRectOrAuto::Rect(rect) => YClipRect::new(rect.clone()).into_value_with(ruby),
    }
}

fn make_clip_side(value: &ClipSide, ruby: &Ruby) -> Value {
    match value {
        LengthPercentageOrAuto::Auto => YClipLengthAuto::new().into_value_with(ruby),
        LengthPercentageOrAuto::LengthPercentage(length) => {
            YClipLength::new(length.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Clip", mark)]
pub struct YClip {
    value: CachedValue<ClipValue>,
}

impl YClip {
    pub fn new(specified_value: Box<ClipRectOrAuto<ClipRect<LengthPercentageOrAuto<Length>>>>) -> Self {
        Self {
            value: CachedValue::new(*specified_value, make_clip_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YClip {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Clip::Auto")]
pub struct YClipAuto {}

impl YClipAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Clip::Rect", mark)]
pub struct YClipRect {
    top: CachedValue<ClipSide>,
    right: CachedValue<ClipSide>,
    bottom: CachedValue<ClipSide>,
    left: CachedValue<ClipSide>,
}

impl YClipRect {
    pub fn new(rect: ClipRectValue) -> Self {
        Self {
            top: CachedValue::new(rect.top, make_clip_side),
            right: CachedValue::new(rect.right, make_clip_side),
            bottom: CachedValue::new(rect.bottom, make_clip_side),
            left: CachedValue::new(rect.left, make_clip_side),
        }
    }

    pub fn top(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.top.get(ruby)
    }

    pub fn right(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.right.get(ruby)
    }

    pub fn bottom(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.bottom.get(ruby)
    }

    pub fn left(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.left.get(ruby)
    }
}

impl DataTypeFunctions for YClipRect {
    fn mark(&self, marker: &gc::Marker) {
        self.top.mark(marker);
        self.right.mark(marker);
        self.bottom.mark(marker);
        self.left.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Clip::Length", mark)]
pub struct YClipLength {
    length: CachedValue<Length>,
}

impl YClipLength {
    pub fn new(length: Length) -> Self {
        Self {
            length: CachedValue::new(length, |length, ruby| length_to_value(length, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }
}

impl DataTypeFunctions for YClipLength {
    fn mark(&self, marker: &gc::Marker) {
        self.length.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Clip::LengthAuto")]
pub struct YClipLengthAuto {}

impl YClipLengthAuto {
    pub fn new() -> Self {
        Self {}
    }
}
