use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::filter::{YFilter, blur::YFilterBlur, brightness::YFilterBrightness, contrast::YFilterContrast, drop_shadow::YFilterDropShadow, grayscale::YFilterGrayscale, hue_rotate::YFilterHueRotate, invert::YFilterInvert, opacity::YFilterOpacity, saturate::YFilterSaturate, sepia::YFilterSepia};

pub fn init(ruby: &Ruby, _yass_module: &RModule, declarations_module: &RModule) -> Result<(), Error> {
    let filter_class = declarations_module.define_class("Filter", ruby.class_object())?;
    filter_class.define_method("values", method!(YFilter::values, 0))?;

    let blur_class = filter_class.define_class("Blur", ruby.class_object())?;
    blur_class.define_method("value", method!(YFilterBlur::value, 0))?;

    let brightness_class = filter_class.define_class("Brightness", ruby.class_object())?;
    brightness_class.define_method("value", method!(YFilterBrightness::value, 0))?;

    let contrast_class = filter_class.define_class("Contrast", ruby.class_object())?;
    contrast_class.define_method("value", method!(YFilterContrast::value, 0))?;

    let drop_shadow_class = filter_class.define_class("DropShadow", ruby.class_object())?;
    drop_shadow_class.define_method("color", method!(YFilterDropShadow::color, 0))?;
    drop_shadow_class.define_method("horizontal", method!(YFilterDropShadow::horizontal, 0))?;
    drop_shadow_class.define_method("vertical", method!(YFilterDropShadow::vertical, 0))?;
    drop_shadow_class.define_method("blur", method!(YFilterDropShadow::blur, 0))?;

    let grayscale_class = filter_class.define_class("Grayscale", ruby.class_object())?;
    grayscale_class.define_method("value", method!(YFilterGrayscale::value, 0))?;

    let hue_rotate_class = filter_class.define_class("HueRotate", ruby.class_object())?;
    hue_rotate_class.define_method("value", method!(YFilterHueRotate::value, 0))?;

    let invert_class = filter_class.define_class("Invert", ruby.class_object())?;
    invert_class.define_method("value", method!(YFilterInvert::value, 0))?;

    let opacity_class = filter_class.define_class("Opacity", ruby.class_object())?;
    opacity_class.define_method("value", method!(YFilterOpacity::value, 0))?;

    let saturate_class = filter_class.define_class("Saturate", ruby.class_object())?;
    saturate_class.define_method("value", method!(YFilterSaturate::value, 0))?;

    let sepia_class = filter_class.define_class("Sepia", ruby.class_object())?;
    sepia_class.define_method("value", method!(YFilterSepia::value, 0))?;

    Ok(())
}
