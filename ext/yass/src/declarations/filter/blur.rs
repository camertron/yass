use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::NonNegative, specified::Length};

use crate::{cached_value::CachedValue, declarations::length::length_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Filter::Blur", mark)]
pub struct YFilterBlur {
    length: CachedValue<NonNegative<Length>>,
}

impl YFilterBlur {
    pub fn new(length: NonNegative<Length>) -> Self {
        Self {
            length: CachedValue::new(length, |length, ruby| length_to_value(&length.0, ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }
}

impl DataTypeFunctions for YFilterBlur {
    fn mark(&self, marker: &gc::Marker) {
        self.length.mark(marker);
    }
}
