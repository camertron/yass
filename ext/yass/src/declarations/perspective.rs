use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::NonNegative, specified::{Length, Perspective}};

use crate::{cached_value::CachedValue, declarations::length::length_to_value};

fn make_perspective_value(value: &Perspective, ruby: &Ruby) -> Value {
    match value {
        Perspective::Length(non_negative) => {
            YPerspectiveLength::new(non_negative.clone()).into_value_with(ruby)
        }
        Perspective::None => YPerspectiveNone::new().into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Perspective", mark)]
pub struct YPerspective {
    value: CachedValue<Perspective>,
}

impl YPerspective {
    pub fn new(perspective: Perspective) -> Self {
        Self {
            value: CachedValue::new(perspective, make_perspective_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YPerspective {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Perspective::None")]
pub struct YPerspectiveNone {}

impl YPerspectiveNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Perspective::Length", mark)]
pub struct YPerspectiveLength {
    length: CachedValue<NonNegative<Length>>,
}

impl YPerspectiveLength {
    pub fn new(non_negative_length: NonNegative<Length>) -> Self {
        Self {
            length: CachedValue::new(non_negative_length, |value, ruby| {
                length_to_value(&value.0, ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }
}

impl DataTypeFunctions for YPerspectiveLength {
    fn mark(&self, marker: &gc::Marker) {
        self.length.mark(marker);
    }
}
