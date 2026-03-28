use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::ZeroToOne, specified::effects::FilterFactor};

use crate::cached_value::CachedValue;

use super::filter_factor_to_value;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Filter::Sepia", mark)]
pub struct YFilterSepia {
    factor: CachedValue<ZeroToOne<FilterFactor>>,
}

impl YFilterSepia {
    pub fn new(factor: ZeroToOne<FilterFactor>) -> Self {
        Self {
            factor: CachedValue::new(factor, |factor, ruby| filter_factor_to_value(&factor.0, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.factor.get(ruby)
    }
}

impl DataTypeFunctions for YFilterSepia {
    fn mark(&self, marker: &gc::Marker) {
        self.factor.mark(marker);
    }
}
