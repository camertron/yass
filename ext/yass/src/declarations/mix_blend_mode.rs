use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::mix_blend_mode::T;

#[magnus::wrap(class = "Yass::Declarations::MixBlendMode")]
pub struct YMixBlendMode {
    specified_value: T
}

impl YMixBlendMode {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Normal => ruby.intern("normal"),
            T::Multiply => ruby.intern("multiply"),
            T::Screen => ruby.intern("screen"),
            T::Overlay => ruby.intern("overlay"),
            T::Darken => ruby.intern("darken"),
            T::Lighten => ruby.intern("lighten"),
            T::ColorDodge => ruby.intern("color_dodge"),
            T::ColorBurn => ruby.intern("color_burn"),
            T::HardLight => ruby.intern("hard_light"),
            T::SoftLight => ruby.intern("soft_light"),
            T::Difference => ruby.intern("difference"),
            T::Exclusion => ruby.intern("exclusion"),
            T::Hue => ruby.intern("hue"),
            T::Saturation => ruby.intern("saturation"),
            T::Color => ruby.intern("color"),
            T::Luminosity => ruby.intern("luminosity"),
            T::PlusLighter => ruby.intern("plus_lighter"),
        }
    }
}
