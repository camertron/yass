use style::values::computed::SelfAlignment;

#[magnus::wrap(class = "Yass::Declarations::AlignSelf")]
pub struct YAlignSelf {
  self_alignment: SelfAlignment
}

impl YAlignSelf {
  pub fn new(self_alignment: SelfAlignment) -> Self {
    Self { self_alignment }
  }
}
