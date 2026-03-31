use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, length::LengthOrNumber, rect::Rect}, specified::{Length, Number}};

use crate::{cached_value::CachedValue, declarations::{calc::YCalc, length::no_calc_length_to_value, number::YNumber}};

type BorderImageOutsetValue = LengthOrNumber<NonNegative<Length>, NonNegative<Number>>;

fn border_image_outset_value_to_ruby(value: &BorderImageOutsetValue, ruby: &Ruby) -> Value {
    match value {
        LengthOrNumber::Number(number) => YNumber::new(number.0.get()).into_value_with(ruby),

        LengthOrNumber::Length(length) => match &length.0 {
            Length::NoCalc(no_calc_length) => no_calc_length_to_value(no_calc_length, ruby),
            Length::Calc(calc_length) => YCalc::new(calc_length.clone()).into_value_with(ruby),
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BorderImageOutset", mark)]
pub struct YBorderImageOutset {
    top: CachedValue<BorderImageOutsetValue>,
    right: CachedValue<BorderImageOutsetValue>,
    bottom: CachedValue<BorderImageOutsetValue>,
    left: CachedValue<BorderImageOutsetValue>,
}

impl YBorderImageOutset {
    pub fn new(specified_value: Box<Rect<BorderImageOutsetValue>>) -> Self {
        Self {
            top: CachedValue::new(specified_value.0, |value, ruby| {
                border_image_outset_value_to_ruby(value, ruby)
            }),

            right: CachedValue::new(specified_value.1, |value, ruby| {
                border_image_outset_value_to_ruby(value, ruby)
            }),

            bottom: CachedValue::new(specified_value.2, |value, ruby| {
                border_image_outset_value_to_ruby(value, ruby)
            }),

            left: CachedValue::new(specified_value.3, |value, ruby| {
                border_image_outset_value_to_ruby(value, ruby)
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

impl DataTypeFunctions for YBorderImageOutset {
    fn mark(&self, marker: &Marker) {
        self.top.mark(marker);
        self.right.mark(marker);
        self.bottom.mark(marker);
        self.left.mark(marker);
    }
}
