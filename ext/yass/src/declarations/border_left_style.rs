use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderLeftStyle")]
pub struct YBorderLeftStyle {
  border_style: BorderStyle
}

impl YBorderLeftStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
