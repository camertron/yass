use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::Angle;

use crate::{cached_value::CachedValue, declarations::angle::YAngle};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Filter::HueRotate", mark)]
pub struct YFilterHueRotate {
    angle: CachedValue<Angle>,
}

impl YFilterHueRotate {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, |angle, ruby| YAngle::new(angle.clone()).into_value_with(ruby)),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle.get(ruby)
    }
}

impl DataTypeFunctions for YFilterHueRotate {
    fn mark(&self, marker: &gc::Marker) {
        self.angle.mark(marker);
    }
}
