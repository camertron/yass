use style::values::computed::Display;

#[magnus::wrap(class = "Yass::Declarations::Display")]
pub struct YDisplay {
  display: Display
}

impl YDisplay {
  pub fn new(display: Display) -> Self {
    Self { display }
  }
}
