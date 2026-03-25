use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::size::{YAnchorContainingCalcFunction, YFitContentFunction, YLengthPercentage, anchor_size_function::{YAnchorSizeFunction, YAnchorSizeKeyword}};

pub fn init(ruby: &Ruby, _yass_module: &RModule, declarations_module: &RModule) -> Result<(), Error> {
    let size_module = declarations_module.define_module("Size")?;

    let length_percentage_class = size_module.define_class("LengthPercentage", ruby.class_object())?;
    length_percentage_class.define_method("value", method!(YLengthPercentage::value, 0))?;

    let _auto_class = size_module.define_class("Auto", ruby.class_object())?;

    let _max_content_class = size_module.define_class("MaxContent", ruby.class_object())?;

    let _min_content_class = size_module.define_class("MinContent", ruby.class_object())?;

    let _fit_content_class = size_module.define_class("FitContent", ruby.class_object())?;

    let _webkit_fill_available_class = size_module.define_class("WebkitFillAvailable", ruby.class_object())?;

    let _stretch_class = size_module.define_class("Stretch", ruby.class_object())?;

    let fit_content_function_class = size_module.define_class("FitContentFunction", ruby.class_object())?;
    fit_content_function_class.define_method("value", method!(YFitContentFunction::value, 0))?;

    let anchor_containing_calc_function_class = size_module.define_class("AnchorContainingCalcFunction", ruby.class_object())?;
    anchor_containing_calc_function_class.define_method("value", method!(YAnchorContainingCalcFunction::value, 0))?;

    let anchor_size_function_class = size_module.define_class("AnchorSizeFunction", ruby.class_object())?;
    anchor_size_function_class.define_method("target_element", method!(YAnchorSizeFunction::target_element, 0))?;
    anchor_size_function_class.define_method("size", method!(YAnchorSizeFunction::size, 0))?;
    anchor_size_function_class.define_method("fallback", method!(YAnchorSizeFunction::fallback, 0))?;

    let anchor_size_keyword_class = size_module.define_class("AnchorSizeKeyword", ruby.class_object())?;
    anchor_size_keyword_class.define_method("value", method!(YAnchorSizeKeyword::value, 0))?;

    Ok(())
}
