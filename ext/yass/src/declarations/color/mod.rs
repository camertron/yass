use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{
    color::ColorSpace,
    values::specified::{Color, ColorPropertyValue},
};

use crate::{cached_value::CachedValue, declarations::color::color::make_color};

pub mod absolute_color;
pub mod absolute;
pub mod color_components;
pub mod color_function_component;
pub mod color_function;
pub mod color_interpolation_method;
pub mod color_mix_item;
pub mod color_mix;
pub mod current_color;
pub mod init;
pub mod light_dark;
pub mod color;
pub mod stored_color_function_component;
pub mod system_color;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color", mark)]
pub struct YColor {
    color: CachedValue<Color>,
}

impl YColor {
    pub fn new(color_property_value: ColorPropertyValue) -> Self {
        Self {
            color: CachedValue::new(color_property_value.0, |color, ruby| {
                make_color(&color, ruby)
            }),
        }
    }

    pub fn color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.color.get(ruby)
    }
}

impl DataTypeFunctions for YColor {
    fn mark(&self, marker: &gc::Marker) {
        self.color.mark(marker);
    }
}

pub fn color_space_to_id(color_space: ColorSpace, ruby: &Ruby) -> Id {
    match color_space {
        ColorSpace::Srgb => ruby.intern("srgb"),
        ColorSpace::Hsl => ruby.intern("hsl"),
        ColorSpace::Hwb => ruby.intern("hwb"),
        ColorSpace::Lab => ruby.intern("lab"),
        ColorSpace::Lch => ruby.intern("lch"),
        ColorSpace::Oklab => ruby.intern("oklab"),
        ColorSpace::Oklch => ruby.intern("oklch"),
        ColorSpace::SrgbLinear => ruby.intern("srgb_linear"),
        ColorSpace::DisplayP3 => ruby.intern("display_p3"),
        ColorSpace::DisplayP3Linear => ruby.intern("display_p3_linear"),
        ColorSpace::A98Rgb => ruby.intern("a98_rgb"),
        ColorSpace::ProphotoRgb => ruby.intern("prophoto_rgb"),
        ColorSpace::Rec2020 => ruby.intern("rec2020"),
        ColorSpace::XyzD50 => ruby.intern("xyz_d50"),
        ColorSpace::XyzD65 => ruby.intern("xyz_d65"),
    }
}
