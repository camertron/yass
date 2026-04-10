use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::transform::Scale, specified::Number};

use crate::{cached_value::CachedValue, declarations::number::YNumber};

fn make_scale_value(value: &Scale<Number>, ruby: &Ruby) -> Value {
    match value {
        Scale::None => YScaleNone::new().into_value_with(ruby),
        Scale::Scale(x, y, z) => YScaleCoords::new(*x, *y, *z).into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Scale", mark)]
pub struct YScale {
    value: CachedValue<Scale<Number>>,
}

impl YScale {
    pub fn new(specified_value: Box<Scale<Number>>) -> Self {
        Self {
            value: CachedValue::new(*specified_value, make_scale_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YScale {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Scale::None")]
pub struct YScaleNone {}

impl YScaleNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Scale::Coords", mark)]
pub struct YScaleCoords {
    x: CachedValue<Number>,
    y: CachedValue<Number>,
    z: CachedValue<Number>,
}

impl YScaleCoords {
    pub fn new(x: Number, y: Number, z: Number) -> Self {
        Self {
            x: CachedValue::new(x, |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),

            y: CachedValue::new(y, |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),

            z: CachedValue::new(z, |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),
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

impl DataTypeFunctions for YScaleCoords {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.z.mark(marker);
    }
}
