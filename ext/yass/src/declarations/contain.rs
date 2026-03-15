use style::values::computed::Contain;

#[magnus::wrap(class = "Yass::Declarations::Contain")]
pub struct YContain {
  contain: Contain
}

impl YContain {
  pub fn new(contain: Contain) -> Self {
    Self { contain }
  }
}
