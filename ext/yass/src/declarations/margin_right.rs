use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginRight")]
pub struct YMarginRight {
  margin: Margin
}

impl YMarginRight {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
