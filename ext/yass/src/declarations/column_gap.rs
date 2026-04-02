use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data};
use style::values::{generics::{NonNegative, length::LengthPercentageOrNormal}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

type ColumnGapValue = LengthPercentageOrNormal<NonNegative<LengthPercentage>>;

fn column_gap_value_to_ruby(value: &ColumnGapValue, ruby: &Ruby) -> Value {
    match value {
        LengthPercentageOrNormal::Normal => YColumnGapNormal::new().into_value_with(ruby),
        LengthPercentageOrNormal::LengthPercentage(length_percentage) => {
            YColumnGapLengthPercentage::new(length_percentage.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ColumnGap", mark)]
pub struct YColumnGap {
    value: CachedValue<ColumnGapValue>,
}

impl YColumnGap {
    pub fn new(value: ColumnGapValue) -> Self {
        Self {
            value: CachedValue::new(value, column_gap_value_to_ruby),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YColumnGap {
    fn mark(&self, marker: &Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ColumnGap::Normal")]
pub struct YColumnGapNormal {}

impl YColumnGapNormal {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ColumnGap::LengthPercentage", mark)]
pub struct YColumnGapLengthPercentage {
    length_percentage: CachedValue<NonNegative<LengthPercentage>>,
}

impl YColumnGapLengthPercentage {
    pub fn new(length_percentage: NonNegative<LengthPercentage>) -> Self {
        Self {
            length_percentage: CachedValue::new(length_percentage, |value, ruby| {
                length_percentage_to_value(value.0.clone(), ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }
}

impl DataTypeFunctions for YColumnGapLengthPercentage {
    fn mark(&self, marker: &Marker) {
        self.length_percentage.mark(marker);
    }
}
