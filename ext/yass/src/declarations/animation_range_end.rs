use style::{properties::longhands::animation_range_end::SpecifiedValue};

#[magnus::wrap(class = "Yass::Declarations::AnimationRangeEnd")]
pub struct YAnimationRangeEnd {
    specified_value: SpecifiedValue
}

impl YAnimationRangeEnd {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }
}
