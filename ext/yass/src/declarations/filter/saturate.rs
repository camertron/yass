use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::NonNegative, specified::effects::FilterFactor};

use crate::cached_value::CachedValue;

use super::filter_factor_to_value;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Filter::Saturate", mark)]
pub struct YFilterSaturate {
    factor: CachedValue<NonNegative<FilterFactor>>,
}

impl YFilterSaturate {
    pub fn new(factor: NonNegative<FilterFactor>) -> Self {
        Self {
            factor: CachedValue::new(factor, |factor, ruby| filter_factor_to_value(&factor.0, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.factor.get(ruby)
    }
}

impl DataTypeFunctions for YFilterSaturate {
    fn mark(&self, marker: &gc::Marker) {
        self.factor.mark(marker);
    }
}
