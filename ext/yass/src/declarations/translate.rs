use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::transform::Translate, specified::{Length, LengthPercentage}};

use crate::{cached_value::CachedValue, declarations::{length::length_to_value, size::length_percentage_to_value}};

fn make_translate_value(value: &Translate<LengthPercentage, Length>, ruby: &Ruby) -> Value {
    match value {
        Translate::None => YTranslateNone::new().into_value_with(ruby),
        Translate::Translate(x, y, z) => {
            YTranslateCoords::new(x.clone(), y.clone(), z.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Translate", mark)]
pub struct YTranslate {
    value: CachedValue<Translate<LengthPercentage, Length>>,
}

impl YTranslate {
    pub fn new(specified_value: Box<Translate<LengthPercentage, Length>>) -> Self {
        Self {
            value: CachedValue::new(*specified_value, make_translate_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YTranslate {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Translate::None")]
pub struct YTranslateNone {}

impl YTranslateNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Translate::Coords", mark)]
pub struct YTranslateCoords {
    x: CachedValue<LengthPercentage>,
    y: CachedValue<LengthPercentage>,
    z: CachedValue<Length>,
}

impl YTranslateCoords {
    pub fn new(x: LengthPercentage, y: LengthPercentage, z: Length) -> Self {
        Self {
            x: CachedValue::new(x, length_percentage_to_value),
            y: CachedValue::new(y, length_percentage_to_value),
            z: CachedValue::new(z, length_to_value),
        }
    }

    pub fn x(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x.get(ruby)
    }

    pub fn y(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y.get(ruby)
    }

    pub fn z(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.z.get(ruby)
    }
}

impl DataTypeFunctions for YTranslateCoords {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.z.mark(marker);
    }
}
