use style::values::computed::BorderStyle;

#[magnus::wrap(class = "Yass::Declarations::BorderTopStyle")]
pub struct YBorderTopStyle {
  border_style: BorderStyle
}

impl YBorderTopStyle {
  pub fn new(border_style: BorderStyle) -> Self {
    Self { border_style }
  }
}
