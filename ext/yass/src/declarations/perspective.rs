use style::values::specified::Perspective;

#[magnus::wrap(class = "Yass::Declarations::Perspective")]
pub struct YPerspective {
  perspective: Perspective
}

impl YPerspective {
  pub fn new(perspective: Perspective) -> Self {
    Self { perspective }
  }
}
