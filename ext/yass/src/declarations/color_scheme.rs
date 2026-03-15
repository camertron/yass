use style::values::computed::ColorScheme;

#[magnus::wrap(class = "Yass::Declarations::ColorScheme")]
pub struct YColorScheme {
  color_scheme: ColorScheme
}

impl YColorScheme {
  pub fn new(color_scheme: ColorScheme) -> Self {
    Self { color_scheme }
  }
}
