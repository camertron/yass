use magnus::{Error, Module, RModule, Ruby, method};

use crate::declarations::animation::{inset::YInset, range_value::YRangeValue, scroll_axis::YScrollAxis, scroll_function::YScrollFunction, scroller::YScroller, timeline_name::YTimelineName, timing_function, view_function::YViewFunction};

pub fn init(ruby: &Ruby, yass_module: &RModule, declarations_module: &RModule) -> Result<(), Error> {
    let animation_module = declarations_module.define_module("Animation")?;

    let _auto_class = animation_module.define_class("Auto", ruby.class_object())?;

    let inset_class = animation_module.define_class("Inset", ruby.class_object())?;
    inset_class.define_method("start", method!(YInset::start, 0))?;
    inset_class.define_method("end", method!(YInset::end, 0))?;

    let _length_auto_class = animation_module.define_class("LengthAuto", ruby.class_object())?;

    let range_value_class = animation_module.define_class("RangeValue", ruby.class_object())?;
    range_value_class.define_method("length_percentage", method!(YRangeValue::length_percentage, 0))?;
    range_value_class.define_method("name", method!(YRangeValue::name, 0))?;

    let scroll_axis_class = animation_module.define_class("ScrollAxis", ruby.class_object())?;
    scroll_axis_class.define_method("value", method!(YScrollAxis::value, 0))?;

    let scroll_function_class = animation_module.define_class("ScrollFunction", ruby.class_object())?;
    scroll_function_class.define_method("scroller", method!(YScrollFunction::scroller, 0))?;
    scroll_function_class.define_method("scroll_axis", method!(YScrollFunction::scroll_axis, 0))?;

    let scroller_class = animation_module.define_class("Scroller", ruby.class_object())?;
    scroller_class.define_method("value", method!(YScroller::value, 0))?;

    let timeline_name_class = animation_module.define_class("TimelineName", ruby.class_object())?;
    timeline_name_class.define_method("value", method!(YTimelineName::value, 0))?;

    let view_function_class = animation_module.define_class("ViewFunction", ruby.class_object())?;
    view_function_class.define_method("scroll_axis", method!(YViewFunction::scroll_axis, 0))?;
    view_function_class.define_method("inset", method!(YViewFunction::inset, 0))?;

    timing_function::init::init(ruby, yass_module, declarations_module, &animation_module);

    Ok(())
}
