use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, border::BorderImageSlice}, specified::NumberOrPercentage};

use crate::{cached_value::CachedValue, declarations::{number::YNumber, percentage::YPercentage}};

type BorderImageSliceValue = NonNegative<NumberOrPercentage>;

fn border_image_slice_value_to_ruby(value: &BorderImageSliceValue, ruby: &Ruby) -> Value {
    match value.0 {
        NumberOrPercentage::Number(number) => YNumber::new(number.get()).into_value_with(ruby),
        NumberOrPercentage::Percentage(percentage) => {
            YPercentage::new(percentage.get()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BorderImageSlice", mark)]
pub struct YBorderImageSlice {
    top: CachedValue<BorderImageSliceValue>,
    right: CachedValue<BorderImageSliceValue>,
    bottom: CachedValue<BorderImageSliceValue>,
    left: CachedValue<BorderImageSliceValue>,
    fill: bool,
}

impl YBorderImageSlice {
    pub fn new(specified_value: Box<BorderImageSlice<BorderImageSliceValue>>) -> Self {
        Self {
            top: CachedValue::new(specified_value.offsets.0, |value, ruby| {
                border_image_slice_value_to_ruby(value, ruby)
            }),

            right: CachedValue::new(specified_value.offsets.1, |value, ruby| {
                border_image_slice_value_to_ruby(value, ruby)
            }),

            bottom: CachedValue::new(specified_value.offsets.2, |value, ruby| {
                border_image_slice_value_to_ruby(value, ruby)
            }),

            left: CachedValue::new(specified_value.offsets.3, |value, ruby| {
                border_image_slice_value_to_ruby(value, ruby)
            }),

            fill: specified_value.fill,
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

    pub fn fill(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.fill
    }
}

impl DataTypeFunctions for YBorderImageSlice {
    fn mark(&self, marker: &Marker) {
        self.top.mark(marker);
        self.right.mark(marker);
        self.bottom.mark(marker);
        self.left.mark(marker);
    }
}
