use magnus::{Error, RArray, Ruby, typed_data};
use style::properties::longhands::background_blend_mode::{SpecifiedValue, single_value::SpecifiedValue as SingleSpecifiedValue};

#[magnus::wrap(class = "Yass::Declarations::BackgroundBlendMode")]
pub struct YBackgroundBlendMode {
    specified_value: SpecifiedValue
}

impl YBackgroundBlendMode {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for blend_mode in rb_self.specified_value.0.as_ref() {
            let id = match blend_mode {
                SingleSpecifiedValue::Normal => ruby.intern("normal"),
                SingleSpecifiedValue::Multiply => ruby.intern("multiply"),
                SingleSpecifiedValue::Screen => ruby.intern("screen"),
                SingleSpecifiedValue::Overlay => ruby.intern("overlay"),
                SingleSpecifiedValue::Darken => ruby.intern("darken"),
                SingleSpecifiedValue::Lighten => ruby.intern("lighten"),
                SingleSpecifiedValue::ColorDodge => ruby.intern("color_dodge"),
                SingleSpecifiedValue::ColorBurn => ruby.intern("color_burn"),
                SingleSpecifiedValue::HardLight => ruby.intern("hard_light"),
                SingleSpecifiedValue::SoftLight => ruby.intern("soft_light"),
                SingleSpecifiedValue::Difference => ruby.intern("difference"),
                SingleSpecifiedValue::Exclusion => ruby.intern("exclusion"),
                SingleSpecifiedValue::Hue => ruby.intern("hue"),
                SingleSpecifiedValue::Saturation => ruby.intern("saturation"),
                SingleSpecifiedValue::Color => ruby.intern("color"),
                SingleSpecifiedValue::Luminosity => ruby.intern("luminosity"),
            };

            result.push(id)?;
        }

        Ok(result)
    }
}
