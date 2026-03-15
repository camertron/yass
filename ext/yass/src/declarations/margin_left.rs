use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginLeft")]
pub struct YMarginLeft {
  margin: Margin
}

impl YMarginLeft {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
