use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{NonNegative, length::LengthPercentageOrAuto}, specified::Length};

use crate::{cached_value::CachedValue, declarations::length::length_to_value};

fn make_column_width_value(value: &LengthPercentageOrAuto<NonNegative<Length>>, ruby: &Ruby) -> Value {
    match value {
        LengthPercentageOrAuto::Auto => YColumnWidthAuto::new().into_value_with(ruby),
        LengthPercentageOrAuto::LengthPercentage(non_negative) => {
            YColumnWidthLength::new(non_negative.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ColumnWidth", mark)]
pub struct YColumnWidth {
    value: CachedValue<LengthPercentageOrAuto<NonNegative<Length>>>,
}

impl YColumnWidth {
    pub fn new(non_negative_length_or_auto: LengthPercentageOrAuto<NonNegative<Length>>) -> Self {
        Self {
            value: CachedValue::new(non_negative_length_or_auto, make_column_width_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YColumnWidth {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ColumnWidth::Auto")]
pub struct YColumnWidthAuto {}

impl YColumnWidthAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ColumnWidth::Length", mark)]
pub struct YColumnWidthLength {
    length: CachedValue<NonNegative<Length>>,
}

impl YColumnWidthLength {
    pub fn new(non_negative: NonNegative<Length>) -> Self {
        Self {
            length: CachedValue::new(non_negative, |v, ruby| length_to_value(&v.0, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }
}

impl DataTypeFunctions for YColumnWidthLength {
    fn mark(&self, marker: &gc::Marker) {
        self.length.mark(marker);
    }
}
