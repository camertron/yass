use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, border::BorderCornerRadius}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::YLengthPercentage};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BorderStartEndRadius", mark)]
pub struct YBorderStartEndRadius {
    width: CachedValue<NonNegative<LengthPercentage>>,
    height: CachedValue<NonNegative<LengthPercentage>>
}

impl YBorderStartEndRadius {
    pub fn new(specified_value: Box<BorderCornerRadius<NonNegative<LengthPercentage>>>) -> Self {
        Self {
            width: CachedValue::new(specified_value.0.width, |width, ruby| {
                YLengthPercentage::new(width.0.clone()).into_value_with(ruby)
            }),

            height: CachedValue::new(specified_value.0.height, |height, ruby| {
                YLengthPercentage::new(height.0.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn width(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.width.get(ruby)
    }

    pub fn height(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.height.get(ruby)
    }
}

impl DataTypeFunctions for YBorderStartEndRadius {
    fn mark(&self, marker: &Marker) {
        self.width.mark(marker);
        self.height.mark(marker);
    }
}
