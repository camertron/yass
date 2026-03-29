use magnus::{Ruby, typed_data, value::Id};
use style::color::mix::{ColorInterpolationMethod, HueInterpolationMethod};

use super::color_space_to_id;

#[magnus::wrap(class = "Yass::Declarations::Color::ColorInterpolationMethod")]
pub struct YColorInterpolationMethod {
    interpolation: ColorInterpolationMethod,
}

impl YColorInterpolationMethod {
    pub fn new(interpolation: ColorInterpolationMethod) -> Self {
        Self { interpolation }
    }

    pub fn space(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        color_space_to_id(rb_self.interpolation.space, ruby)
    }

    pub fn hue(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.interpolation.hue {
            HueInterpolationMethod::Shorter => ruby.intern("shorter"),
            HueInterpolationMethod::Longer => ruby.intern("longer"),
            HueInterpolationMethod::Increasing => ruby.intern("increasing"),
            HueInterpolationMethod::Decreasing => ruby.intern("decreasing"),
            HueInterpolationMethod::Specified => ruby.intern("specified"),
        }
    }
}
