use style::values::computed::WillChange;

#[magnus::wrap(class = "Yass::Declarations::WillChange")]
pub struct YWillChange {
  will_change: WillChange
}

impl YWillChange {
  pub fn new(will_change: WillChange) -> Self {
    Self { will_change }
  }
}
