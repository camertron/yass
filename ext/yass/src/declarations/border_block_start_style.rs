use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderBlockStartStyle")]
pub struct YBorderBlockStartStyle {
  border_style: BorderStyle
}

impl YBorderBlockStartStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
