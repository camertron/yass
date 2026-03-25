use magnus::{Ruby, typed_data};
use style::values::specified::Angle;

#[magnus::wrap(class = "Yass::Declarations::Angle")]
pub struct YAngle {
    angle: Angle
}

impl YAngle {
    pub fn new(angle: Angle) -> Self {
        Self { angle }
    }

    pub fn was_calc(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.angle.was_calc()
    }

    pub fn degrees(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.angle.degrees()
    }

    pub fn unit(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> &'static str {
        rb_self.angle.unit()
    }
}
