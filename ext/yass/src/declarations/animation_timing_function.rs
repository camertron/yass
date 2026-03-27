use style::properties::longhands::animation_timing_function::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationTimingFunction")]
pub struct YAnimationTimingFunction {
    specified_value: SpecifiedValue
}

impl YAnimationTimingFunction {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }
}
