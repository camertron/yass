use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, border::BorderSpacing}, specified::Length};

use crate::{cached_value::CachedValue, declarations::{calc::YCalc, length::no_calc_length_to_value}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BorderSpacing", mark)]
pub struct YBorderSpacing {
    horizontal: CachedValue<NonNegative<Length>>,
    vertical: CachedValue<NonNegative<Length>>,
}

impl YBorderSpacing {
    pub fn new(specified_value: Box<BorderSpacing<NonNegative<Length>>>) -> Self {
        let specified_value = *specified_value;
        let horizontal = specified_value.0.width;
        let vertical = specified_value.0.height;

        Self {
            horizontal: CachedValue::new(horizontal, |h, ruby| {
                non_negative_length_to_value(h, ruby)
            }),

            vertical: CachedValue::new(vertical, |v, ruby| {
                non_negative_length_to_value(v, ruby)
            }),
        }
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.horizontal.get(ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.vertical.get(ruby)
    }
}

impl DataTypeFunctions for YBorderSpacing {
    fn mark(&self, marker: &Marker) {
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
    }
}

fn non_negative_length_to_value(length: &NonNegative<Length>, ruby: &Ruby) -> Value {
    match &length.0 {
        Length::NoCalc(no_calc_length) => no_calc_length_to_value(no_calc_length, ruby),
        Length::Calc(calc_length_percentage) => YCalc::new(calc_length_percentage.clone()).into_value_with(ruby),
    }
}
