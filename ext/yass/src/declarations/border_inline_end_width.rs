use style::values::specified::BorderSideWidth;

#[magnus::wrap(class = "Yass::Declarations::BorderInlineEndWidth")]
pub struct YBorderInlineEndWidth {
  border_side_width: BorderSideWidth
}

impl YBorderInlineEndWidth {
  pub fn new(border_side_width: BorderSideWidth) -> Self {
    Self { border_side_width }
  }
}
