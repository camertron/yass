use magnus::{Error, Module, RClass, Ruby, method};

use crate::declarations::color::{absolute::YAbsolute, absolute_color::YAbsoluteColor, color_components::YColorComponents, color_function::YColorFunction, color_function_component::YColorFunctionComponent, color_interpolation_method::YColorInterpolationMethod, color_mix::YColorMix, color_mix_item::YColorMixItem, light_dark::YLightDark, system_color::YSystemColor};

pub fn init(ruby: &Ruby, color_class: &RClass) -> Result<(), Error> {
    let absolute_class = color_class.define_class("Absolute", ruby.class_object())?;
    absolute_class.define_method("color", method!(YAbsolute::color, 0))?;
    absolute_class.define_method("authored", method!(YAbsolute::authored, 0))?;

    let absolute_color_class = color_class.define_class("AbsoluteColor", ruby.class_object())?;
    absolute_color_class.define_method("components", method!(YAbsoluteColor::components, 0))?;
    absolute_color_class.define_method("alpha", method!(YAbsoluteColor::alpha, 0))?;
    absolute_color_class.define_method("color_space", method!(YAbsoluteColor::color_space, 0))?;
    absolute_color_class.define_method("legacy_syntax?", method!(YAbsoluteColor::legacy_syntax, 0))?;
    absolute_color_class.define_method("transparent?", method!(YAbsoluteColor::transparent, 0))?;

    let color_components_class = color_class.define_class("ColorComponents", ruby.class_object())?;
    color_components_class.define_method("c0", method!(YColorComponents::c0, 0))?;
    color_components_class.define_method("c1", method!(YColorComponents::c1, 0))?;
    color_components_class.define_method("c2", method!(YColorComponents::c2, 0))?;

    let color_function_class = color_class.define_class("ColorFunction", ruby.class_object())?;
    color_function_class.define_method("kind", method!(YColorFunction::kind, 0))?;
    color_function_class.define_method("has_origin_color?", method!(YColorFunction::has_origin_color, 0))?;
    color_function_class.define_method("origin_color", method!(YColorFunction::origin_color, 0))?;
    color_function_class.define_method("components", method!(YColorFunction::components, 0))?;
    color_function_class.define_method("alpha", method!(YColorFunction::alpha, 0))?;
    color_function_class.define_method("color_space", method!(YColorFunction::color_space, 0))?;

    let color_function_component_class = color_class.define_class("ColorFunctionComponent", ruby.class_object())?;
    color_function_component_class.define_method("kind", method!(YColorFunctionComponent::kind, 0))?;
    color_function_component_class.define_method("value", method!(YColorFunctionComponent::value, 0))?;
    color_function_component_class.define_method("channel_keyword", method!(YColorFunctionComponent::channel_keyword, 0))?;
    color_function_component_class.define_method("calc", method!(YColorFunctionComponent::calc, 0))?;

    let color_mix_class = color_class.define_class("ColorMix", ruby.class_object())?;
    color_mix_class.define_method("interpolation", method!(YColorMix::interpolation, 0))?;
    color_mix_class.define_method("items", method!(YColorMix::items, 0))?;
    color_mix_class.define_method("normalize_weights?", method!(YColorMix::normalize_weights, 0))?;
    color_mix_class.define_method("result_in_modern_syntax?", method!(YColorMix::result_in_modern_syntax, 0))?;

    let color_mix_item_class = color_class.define_class("ColorMixItem", ruby.class_object())?;
    color_mix_item_class.define_method("color", method!(YColorMixItem::color, 0))?;
    color_mix_item_class.define_method("percentage", method!(YColorMixItem::percentage, 0))?;

    let light_dark_class = color_class.define_class("LightDark", ruby.class_object())?;
    light_dark_class.define_method("light", method!(YLightDark::light, 0))?;
    light_dark_class.define_method("dark", method!(YLightDark::dark, 0))?;

    let color_interpolation_method_class = color_class.define_class("ColorInterpolationMethod", ruby.class_object())?;
    color_interpolation_method_class.define_method("space", method!(YColorInterpolationMethod::space, 0))?;
    color_interpolation_method_class.define_method("hue", method!(YColorInterpolationMethod::hue, 0))?;

    let system_color_class = color_class.define_class("SystemColor", ruby.class_object())?;
    system_color_class.define_method("value", method!(YSystemColor::value, 0))?;

    let _current_color_class = color_class.define_class("CurrentColor", ruby.class_object())?;

    Ok(())
}
