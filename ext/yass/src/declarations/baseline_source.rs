use style::values::computed::BaselineSource;

#[magnus::wrap(class = "Yass::Declarations::BaselineSource")]
pub struct YBaselineSource {
  baseline_source: BaselineSource
}

impl YBaselineSource {
  pub fn new(baseline_source: BaselineSource) -> Self {
    Self { baseline_source }
  }
}
