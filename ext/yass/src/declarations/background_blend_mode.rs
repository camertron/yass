use style::properties::longhands::background_blend_mode::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::BackgroundBlendMode")]
pub struct YBackgroundBlendMode {
    specified_value: SpecifiedValue
}

impl YBackgroundBlendMode {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }
}
