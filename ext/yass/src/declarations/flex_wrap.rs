use style::computed_values::flex_wrap::T;

#[magnus::wrap(class = "Yass::Declarations::FlexWrap")]
pub struct YFlexWrap {
  specified_value: T
}

impl YFlexWrap {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
