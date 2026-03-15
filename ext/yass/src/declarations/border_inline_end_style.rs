use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderInlineEndStyle")]
pub struct YBorderInlineEndStyle {
  border_style: BorderStyle
}

impl YBorderInlineEndStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
