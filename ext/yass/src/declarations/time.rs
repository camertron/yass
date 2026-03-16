use magnus::{Ruby, typed_data};
use style::values::specified::Time;

#[magnus::wrap(class = "Yass::Declarations::Time")]
pub struct YTime {
  time: Time
}

impl YTime {
  pub fn new(time: Time) -> Self {
    Self { time }
  }

  pub fn seconds(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
    rb_self.time.seconds()
  }

  pub fn unit(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> &'static str {
    rb_self.time.unit()
  }
}
