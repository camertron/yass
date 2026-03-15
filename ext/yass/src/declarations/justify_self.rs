use style::values::computed::SelfAlignment;

#[magnus::wrap(class = "Yass::Declarations::JustifySelf")]
pub struct YJustifySelf {
  self_alignment: SelfAlignment
}

impl YJustifySelf {
  pub fn new(self_alignment: SelfAlignment) -> Self {
    Self { self_alignment }
  }
}
