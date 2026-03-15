use style::values::{generics::{NonNegative, position::AspectRatio}, specified::Number};

#[magnus::wrap(class = "Yass::Declarations::AspectRatio")]
pub struct YAspectRatio {
  aspect_ratio: AspectRatio<NonNegative<Number>>
}

impl YAspectRatio {
  pub fn new(aspect_ratio: AspectRatio<NonNegative<Number>>) -> Self {
    Self { aspect_ratio }
  }
}
