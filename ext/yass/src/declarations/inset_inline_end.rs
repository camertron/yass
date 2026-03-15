use style::values::specified::Inset;

#[magnus::wrap(class = "Yass::Declarations::InsetInlineEnd")]
pub struct YInsetInlineEnd {
  inset: Inset
}

impl YInsetInlineEnd {
  pub fn new(inset: Inset) -> Self {
    Self { inset }
  }
}
