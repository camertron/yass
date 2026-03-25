use style::{properties::longhands::animation_range_start::SpecifiedValue};

#[magnus::wrap(class = "Yass::Declarations::AnimationRangeStart")]
pub struct YAnimationRangeStart {
    specified_value: SpecifiedValue
}

impl YAnimationRangeStart {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }
}
