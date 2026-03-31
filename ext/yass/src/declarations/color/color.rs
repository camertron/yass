use magnus::{IntoValue, Ruby, Value};
use style::values::{generics::color::ColorOrAuto, specified::Color};

use crate::declarations::color::{absolute::YAbsolute, auto::YAutoColor, color_function::YColorFunction, color_mix::YColorMix, current_color::YCurrentColor, light_dark::YLightDark, system_color::YSystemColor};

pub fn make_color(color: &Color, ruby: &Ruby) -> Value {
    match &color {
        Color::Absolute(absolute) => {
            YAbsolute::new((**absolute).clone()).into_value_with(ruby)
        },
        Color::ColorFunction(color_function) => {
            YColorFunction::new((**color_function).clone()).into_value_with(ruby)
        },
        Color::ColorMix(color_mix) => {
            YColorMix::new((**color_mix).clone()).into_value_with(ruby)
        },
        Color::LightDark(light_dark) => {
            YLightDark::new((**light_dark).clone()).into_value_with(ruby)
        },
        Color::ContrastColor(contrast_color) => {
            make_color(contrast_color, ruby)
        },
        Color::CurrentColor => {
            YCurrentColor::new().into_value_with(ruby)
        },
        Color::System(system_color) => {
            YSystemColor::new(*system_color).into_value_with(ruby)
        },
    }
}

pub fn make_color_or_auto(color_or_auto: &ColorOrAuto<Color>, ruby: &Ruby) -> Value {
    match color_or_auto {
        ColorOrAuto::Color(c) => make_color(c, ruby),
        ColorOrAuto::Auto => YAutoColor::new().into_value_with(ruby),
    }
}