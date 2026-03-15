use style::computed_values::mix_blend_mode::T;

#[magnus::wrap(class = "Yass::Declarations::MixBlendMode")]
pub struct YMixBlendMode {
  specified_value: T
}

impl YMixBlendMode {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
