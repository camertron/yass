use style::values::computed::Clear;

#[magnus::wrap(class = "Yass::Declarations::Clear")]
pub struct YClear {
  clear: Clear
}

impl YClear {
  pub fn new(clear: Clear) -> Self {
    Self { clear }
  }
}
