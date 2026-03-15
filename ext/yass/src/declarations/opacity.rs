use style::values::specified::Opacity;

#[magnus::wrap(class = "Yass::Declarations::Opacity")]
pub struct YOpacity {
  opacity: Opacity
}

impl YOpacity {
  pub fn new(opacity: Opacity) -> Self {
    Self { opacity }
  }
}
