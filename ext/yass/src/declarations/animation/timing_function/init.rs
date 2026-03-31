use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::animation::timing_function::{cubic_bezier::YCubicBezier, keyword::YKeyword, piecewise_linear_function::{YPiecewiseLinearFunction, YPiecewiseLinearFunctionEntry}, steps::YSteps};

pub fn init(ruby: &Ruby, _yass_module: &RModule, _declarations_module: &RModule, animation_module: &RModule) -> Result<(), Error> {
    let timing_function_module = animation_module.define_module("TimingFunction")?;

    let cubic_bezier_class = timing_function_module.define_class("CubicBezier", ruby.class_object())?;
    cubic_bezier_class.define_method("x1", method!(YCubicBezier::x1, 0))?;
    cubic_bezier_class.define_method("y1", method!(YCubicBezier::y1, 0))?;
    cubic_bezier_class.define_method("x2", method!(YCubicBezier::x2, 0))?;
    cubic_bezier_class.define_method("y2", method!(YCubicBezier::y2, 0))?;

    let keyword_class = timing_function_module.define_class("Keyword", ruby.class_object())?;
    keyword_class.define_method("value", method!(YKeyword::value, 0))?;

    let piecewise_linear_function_class = timing_function_module.define_class("PiecewiseLinearFunction", ruby.class_object())?;
    piecewise_linear_function_class.define_method("values", method!(YPiecewiseLinearFunction::values, 0))?;

    let piecewise_linear_function_entry_class = timing_function_module.define_class("PiecewiseLinearFunctionEntry", ruby.class_object())?;
    piecewise_linear_function_entry_class.define_method("x", method!(YPiecewiseLinearFunctionEntry::x, 0))?;
    piecewise_linear_function_entry_class.define_method("y", method!(YPiecewiseLinearFunctionEntry::y, 0))?;

    let steps_class = timing_function_module.define_class("Steps", ruby.class_object())?;
    steps_class.define_method("count", method!(YSteps::count, 0))?;
    steps_class.define_method("position", method!(YSteps::position, 0))?;

    Ok(())
}
