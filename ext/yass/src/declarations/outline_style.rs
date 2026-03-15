use style::values::computed::OutlineStyle;

#[magnus::wrap(class = "Yass::Declarations::OutlineStyle")]
pub struct YOutlineStyle {
  outline_style: OutlineStyle
}

impl YOutlineStyle {
  pub fn new(outline_style: OutlineStyle) -> Self {
    Self { outline_style }
  }
}
