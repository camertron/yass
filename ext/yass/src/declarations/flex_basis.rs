use style::values::{generics::{NonNegative, flex::FlexBasis, length::Size}, specified::LengthPercentage};

#[magnus::wrap(class = "Yass::Declarations::FlexBasis")]
pub struct YFlexBasis {
  specified_value: Box<FlexBasis<Size<NonNegative<LengthPercentage>>>>
}

impl YFlexBasis {
  pub fn new(specified_value: Box<FlexBasis<Size<NonNegative<LengthPercentage>>>>) -> Self {
    Self { specified_value }
  }
}
