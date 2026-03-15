use style::computed_values::unicode_bidi::T;

#[magnus::wrap(class = "Yass::Declarations::UnicodeBidi")]
pub struct YUnicodeBidi {
  specified_value: T
}

impl YUnicodeBidi {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
