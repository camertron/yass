use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, length::LengthPercentageOrNormal}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

type RowGapValue = LengthPercentageOrNormal<NonNegative<LengthPercentage>>;

fn row_gap_to_value(value: &RowGapValue, ruby: &Ruby) -> Value {
    match value {
        LengthPercentageOrNormal::Normal => YRowGapNormal::new().into_value_with(ruby),
        LengthPercentageOrNormal::LengthPercentage(length_percentage) => {
            YRowGapLengthPercentage::new(length_percentage.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::RowGap", mark)]
pub struct YRowGap {
    value: CachedValue<RowGapValue>,
}

impl YRowGap {
    pub fn new(value: RowGapValue) -> Self {
        Self {
            value: CachedValue::new(value, row_gap_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YRowGap {
    fn mark(&self, marker: &Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::RowGap::Normal")]
pub struct YRowGapNormal {}

impl YRowGapNormal {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::RowGap::LengthPercentage", mark)]
pub struct YRowGapLengthPercentage {
    length_percentage: CachedValue<NonNegative<LengthPercentage>>,
}

impl YRowGapLengthPercentage {
    pub fn new(length_percentage: NonNegative<LengthPercentage>) -> Self {
        Self {
            length_percentage: CachedValue::new(length_percentage, |value, ruby| {
                length_percentage_to_value(&value.0, ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }
}

impl DataTypeFunctions for YRowGapLengthPercentage {
    fn mark(&self, marker: &Marker) {
        self.length_percentage.mark(marker);
    }
}
