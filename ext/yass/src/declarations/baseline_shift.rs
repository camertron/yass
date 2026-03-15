use style::values::specified::BaselineShift;

#[magnus::wrap(class = "Yass::Declarations::BaselineShift")]
pub struct YBaselineShift {
  baseline_shift: BaselineShift
}

impl YBaselineShift {
  pub fn new(baseline_shift: BaselineShift) -> Self {
    Self { baseline_shift }
  }
}
