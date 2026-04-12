use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::NonNegativeNumber;

use crate::{cached_value::CachedValue, declarations::number::YNumber};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FlexShrink", mark)]
pub struct YFlexShrink {
    non_negative_number: CachedValue<NonNegativeNumber>
}

impl YFlexShrink {
    pub fn new(non_negative_number: NonNegativeNumber) -> Self {
        Self {
            non_negative_number: CachedValue::new(non_negative_number, |num, ruby| {
                YNumber::new(num.get()).into_value_with(ruby)
            })
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.non_negative_number.get(ruby)
    }
}

impl DataTypeFunctions for YFlexShrink {
    fn mark(&self, marker: &gc::Marker) {
        self.non_negative_number.mark(marker);
    }
}
