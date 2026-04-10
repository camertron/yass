use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::transform::Rotate, specified::{Angle, Number}};

use crate::{cached_value::CachedValue, declarations::{angle::YAngle, number::YNumber}};

fn make_rotate_value(value: &Rotate<Number, Angle>, ruby: &Ruby) -> Value {
    match value {
        Rotate::None => YRotateNone::new().into_value_with(ruby),
        Rotate::Rotate(angle) => YAngle::new(angle.clone()).into_value_with(ruby),
        Rotate::Rotate3D(x, y, z, angle) => {
            YRotate3D::new(*x, *y, *z, *angle).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Rotate", mark)]
pub struct YRotate {
    value: CachedValue<Rotate<Number, Angle>>,
}

impl YRotate {
    pub fn new(specified_value: Box<Rotate<Number, Angle>>) -> Self {
        Self {
            value: CachedValue::new(*specified_value, make_rotate_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YRotate {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Rotate::None")]
pub struct YRotateNone {}

impl YRotateNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Rotate::Rotate3D", mark)]
pub struct YRotate3D {
    x: CachedValue<Number>,
    y: CachedValue<Number>,
    z: CachedValue<Number>,
    angle: CachedValue<Angle>,
}

impl YRotate3D {
    pub fn new(x: Number, y: Number, z: Number, angle: Angle) -> Self {
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

            angle: CachedValue::new(angle, |angle, ruby| {
                YAngle::new(*angle).into_value_with(ruby)
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

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle.get(ruby)
    }
}

impl DataTypeFunctions for YRotate3D {
    fn mark(&self, marker: &gc::Marker) {
        self.x.mark(marker);
        self.y.mark(marker);
        self.z.mark(marker);
        self.angle.mark(marker);
    }
}
