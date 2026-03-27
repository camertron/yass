use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Number;

use crate::{cached_value::CachedValue, declarations::number::YNumber};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Animation::TimingFunction::CubicBezier", mark)]
pub struct YCubicBezier {
    x1: CachedValue<Number>,
    y1: CachedValue<Number>,
    x2: CachedValue<Number>,
    y2: CachedValue<Number>
}

impl YCubicBezier {
    pub fn new(x1: Number, y1: Number, x2: Number, y2: Number) -> Self {
        Self {
            x1: CachedValue::new(x1.clone(), |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),

            y1: CachedValue::new(y1.clone(), |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),

            x2: CachedValue::new(x2.clone(), |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),

            y2: CachedValue::new(y2.clone(), |number, ruby| {
                YNumber::new(number.get()).into_value_with(ruby)
            }),
        }
    }

    pub fn x1(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x1.get(ruby)
    }

    pub fn y1(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y1.get(ruby)
    }

    pub fn x2(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.x2.get(ruby)
    }

    pub fn y2(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.y2.get(ruby)
    }
}

impl DataTypeFunctions for YCubicBezier {
    fn mark(&self, marker: &gc::Marker) {
        self.x1.mark(marker);
        self.y1.mark(marker);
        self.x2.mark(marker);
        self.y2.mark(marker);
    }
}
