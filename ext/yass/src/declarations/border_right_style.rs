use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderRightStyle")]
pub struct YBorderRightStyle {
  border_style: BorderStyle
}

impl YBorderRightStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
