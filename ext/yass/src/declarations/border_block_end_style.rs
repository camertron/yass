use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderBlockEndStyle")]
pub struct YBorderBlockEndStyle {
  border_style: BorderStyle
}

impl YBorderBlockEndStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
