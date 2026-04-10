use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::_servo_top_layer::T;

#[magnus::wrap(class = "Yass::Declarations::ServoTopLayer")]
pub struct YServoTopLayer {
    specified_value: T
}

impl YServoTopLayer {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::None => ruby.intern("none"),
            T::Top => ruby.intern("top"),
        }
    }
}
