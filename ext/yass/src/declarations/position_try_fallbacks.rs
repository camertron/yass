use style::values::computed::PositionTryFallbacks;

#[magnus::wrap(class = "Yass::Declarations::PositionTryFallbacks")]
pub struct YPositionTryFallbacks {
  position_try_fallbacks: PositionTryFallbacks
}

impl YPositionTryFallbacks {
  pub fn new(position_try_fallbacks: PositionTryFallbacks) -> Self {
    Self { position_try_fallbacks }
  }
}
