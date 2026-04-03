use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::calc::{YCalc, abs::YAbs, anchor_function::{YAnchorFunction, YAnchorSideKeyword}, anchor_size_function::{YAnchorSizeFunction, YAnchorSizeKeyword}, clamp::YClamp, hypot::YHypot, invert::YInvert, min_max::YMinMax, mod_rem::YModRem, negate::YNegate, product::YProduct, round::YRound, sign::YSign, sum::YSum};

pub fn init(ruby: &Ruby, _yass_module: &RModule, declarations_module: &RModule) -> Result<(), Error> {
    let calc_class = declarations_module.define_class("Calc", ruby.class_object())?;
    calc_class.define_method("clamping_mode", method!(YCalc::clamping_mode, 0))?;
    calc_class.define_method("root", method!(YCalc::root, 0))?;

    let abs_class = calc_class.define_class("Abs", ruby.class_object())?;
    abs_class.define_method("children", method!(YAbs::children, 0))?;

    let anchor_function_class = calc_class.define_class("AnchorFunction", ruby.class_object())?;
    anchor_function_class.define_method("children", method!(YAnchorFunction::children, 0))?;
    anchor_function_class.define_method("target_element", method!(YAnchorFunction::target_element, 0))?;
    anchor_function_class.define_method("side", method!(YAnchorFunction::side, 0))?;
    anchor_function_class.define_method("fallback", method!(YAnchorFunction::fallback, 0))?;

    let anchor_side_keyword_class = calc_class.define_class("AnchorSideKeyword", ruby.class_object())?;
    anchor_side_keyword_class.define_method("value", method!(YAnchorSideKeyword::value, 0))?;

    let anchor_size_function_class = calc_class.define_class("AnchorSizeFunction", ruby.class_object())?;
    anchor_size_function_class.define_method("children", method!(YAnchorSizeFunction::children, 0))?;
    anchor_size_function_class.define_method("target_element", method!(YAnchorSizeFunction::target_element, 0))?;
    anchor_size_function_class.define_method("size", method!(YAnchorSizeFunction::size, 0))?;
    anchor_size_function_class.define_method("fallback", method!(YAnchorSizeFunction::fallback, 0))?;

    let anchor_size_keyword_class = calc_class.define_class("AnchorSizeKeyword", ruby.class_object())?;
    anchor_size_keyword_class.define_method("value", method!(YAnchorSizeKeyword::value, 0))?;

    let clamp_class = calc_class.define_class("Clamp", ruby.class_object())?;
    clamp_class.define_method("children", method!(YClamp::children, 0))?;
    clamp_class.define_method("min", method!(YClamp::min, 0))?;
    clamp_class.define_method("center", method!(YClamp::center, 0))?;
    clamp_class.define_method("max", method!(YClamp::max, 0))?;

    let hypot_class = calc_class.define_class("Hypot", ruby.class_object())?;
    hypot_class.define_method("children", method!(YHypot::children, 0))?;

    let invert_class = calc_class.define_class("Invert", ruby.class_object())?;
    invert_class.define_method("children", method!(YInvert::children, 0))?;

    let min_max_class = calc_class.define_class("MinMax", ruby.class_object())?;
    min_max_class.define_method("children", method!(YMinMax::children, 0))?;
    min_max_class.define_method("op", method!(YMinMax::op, 0))?;

    let mod_rem_class = calc_class.define_class("ModRem", ruby.class_object())?;
    mod_rem_class.define_method("children", method!(YModRem::children, 0))?;
    mod_rem_class.define_method("dividend", method!(YModRem::dividend, 0))?;
    mod_rem_class.define_method("divisor", method!(YModRem::divisor, 0))?;
    mod_rem_class.define_method("op", method!(YModRem::op, 0))?;

    let negate_class = calc_class.define_class("Negate", ruby.class_object())?;
    negate_class.define_method("children", method!(YNegate::children, 0))?;

    let product_class = calc_class.define_class("Product", ruby.class_object())?;
    product_class.define_method("children", method!(YProduct::children, 0))?;

    let round_class = calc_class.define_class("Round", ruby.class_object())?;
    round_class.define_method("children", method!(YRound::children, 0))?;
    round_class.define_method("value", method!(YRound::value, 0))?;
    round_class.define_method("step", method!(YRound::step, 0))?;
    round_class.define_method("rounding_strategy", method!(YRound::rounding_strategy, 0))?;

    let sign_class = calc_class.define_class("Sign", ruby.class_object())?;
    sign_class.define_method("children", method!(YSign::children, 0))?;

    let sum_class = calc_class.define_class("Sum", ruby.class_object())?;
    sum_class.define_method("children", method!(YSum::children, 0))?;

    Ok(())
}
