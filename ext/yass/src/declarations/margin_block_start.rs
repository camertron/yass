use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginBlockStart")]
pub struct YMarginBlockStart {
  margin: Margin
}

impl YMarginBlockStart {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
