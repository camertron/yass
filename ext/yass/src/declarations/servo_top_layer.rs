use style::computed_values::_servo_top_layer::T;

#[magnus::wrap(class = "Yass::Declarations::ServoTopLayer")]
pub struct YServoTopLayer {
  specified_value: T
}

impl YServoTopLayer {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
