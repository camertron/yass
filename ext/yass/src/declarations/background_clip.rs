use style::{OwnedSlice, computed_values::background_clip::SingleComputedValue};

#[magnus::wrap(class = "Yass::Declarations::BackgroundClip")]
pub struct YBackgroundClip {
  specified_value: OwnedSlice<SingleComputedValue>
}

impl YBackgroundClip {
  pub fn new(specified_value: OwnedSlice<SingleComputedValue>) -> Self {
    Self { specified_value }
  }
}
