use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, border::BorderImageSideWidth, rect::Rect}, specified::{LengthPercentage, Number}};

use crate::{cached_value::CachedValue, declarations::{number::YNumber, size::YLengthPercentage}};

type BorderImageWidthValue = BorderImageSideWidth<NonNegative<LengthPercentage>, NonNegative<Number>>;

fn border_image_width_value_to_ruby(value: &BorderImageWidthValue, ruby: &Ruby) -> Value {
    match value {
        BorderImageSideWidth::Number(number) => {
            YNumber::new(number.get()).into_value_with(ruby)
        }

        BorderImageSideWidth::LengthPercentage(length_percentage) => {
            YLengthPercentage::new(length_percentage.0.clone()).into_value_with(ruby)
        }

        BorderImageSideWidth::Auto => YBorderImageWidthAuto::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BorderImageWidth", mark)]
pub struct YBorderImageWidth {
    top: CachedValue<BorderImageWidthValue>,
    right: CachedValue<BorderImageWidthValue>,
    bottom: CachedValue<BorderImageWidthValue>,
    left: CachedValue<BorderImageWidthValue>,
}

impl YBorderImageWidth {
    pub fn new(specified_value: Box<Rect<BorderImageWidthValue>>) -> Self {
        Self {
            top: CachedValue::new(specified_value.0, |value, ruby| {
                border_image_width_value_to_ruby(value, ruby)
            }),

            right: CachedValue::new(specified_value.1, |value, ruby| {
                border_image_width_value_to_ruby(value, ruby)
            }),

            bottom: CachedValue::new(specified_value.2, |value, ruby| {
                border_image_width_value_to_ruby(value, ruby)
            }),

            left: CachedValue::new(specified_value.3, |value, ruby| {
                border_image_width_value_to_ruby(value, ruby)
            }),
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

impl DataTypeFunctions for YBorderImageWidth {
    fn mark(&self, marker: &Marker) {
        self.top.mark(marker);
        self.right.mark(marker);
        self.bottom.mark(marker);
        self.left.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::BorderImageWidth::Auto")]
pub struct YBorderImageWidthAuto {}

impl YBorderImageWidthAuto {
    pub fn new() -> Self {
        Self {}
    }
}
