use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderInlineStartStyle")]
pub struct YBorderInlineStartStyle {
  border_style: BorderStyle
}

impl YBorderInlineStartStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
