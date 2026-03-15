use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderBottomStyle")]
pub struct YBorderBottomStyle {
  border_style: BorderStyle
}

impl YBorderBottomStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
