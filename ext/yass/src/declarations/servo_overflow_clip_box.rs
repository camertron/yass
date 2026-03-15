use style::computed_values::_servo_overflow_clip_box::T;

#[magnus::wrap(class = "Yass::Declarations::ServoOverflowClipBox")]
pub struct YServoOverflowClipBox {
  specified_value: T
}

impl YServoOverflowClipBox {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
