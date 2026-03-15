use style::values::specified::Margin;

#[magnus::wrap(class = "Yass::Declarations::MarginInlineStart")]
pub struct YMarginInlineStart {
  margin: Margin
}

impl YMarginInlineStart {
  pub fn new(margin: Margin) -> Self {
    Self { margin }
  }
}
