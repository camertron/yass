use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use style::{
    color::ColorFunction,
    values::specified::Color as SpecifiedColor,
};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::color::{color::make_color, color_function_component::YColorFunctionComponent, stored_color_function_component::StoredColorFunctionComponent}};

use super::{color_space_to_id};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::ColorFunction", mark)]
pub struct YColorFunction {
    color_function: ColorFunction<SpecifiedColor>,
    origin_color: CachedValue<Option<SpecifiedColor>>,
    components: CachedValueList<StoredColorFunctionComponent>,
    alpha: CachedValue<StoredColorFunctionComponent>,
}

impl YColorFunction {
    pub fn new(color_function: ColorFunction<SpecifiedColor>) -> Self {
        let origin_color = color_function_origin(&color_function);
        let components = color_function_components(&color_function);
        let alpha = color_function_alpha(&color_function);

        Self {
            origin_color: CachedValue::new(origin_color, |color, ruby| {
                maybe_specified_color_to_value(color.clone(), ruby)
            }),

            components: CachedValueList::new(components, |component, _ctx, ruby| {
                YColorFunctionComponent::new(component.clone()).into_value_with(ruby)
            }),

            alpha: CachedValue::new(alpha, |component, ruby| {
                YColorFunctionComponent::new(component.clone()).into_value_with(ruby)
            }),

            color_function,
        }
    }

    pub fn has_origin_color(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.color_function.has_origin_color()
    }

    pub fn origin_color(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.origin_color.get(ruby)
    }

    pub fn components(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.components.to_a(ruby)
    }

    pub fn alpha(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.alpha.get(ruby)
    }

    pub fn color_space(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        match rb_self.color_function {
            ColorFunction::Color(.., color_space) => color_space_to_id(color_space, ruby).into_value_with(ruby),
            _ => ruby.qnil().into_value_with(ruby),
        }
    }
}

impl DataTypeFunctions for YColorFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.origin_color.mark(marker);
        self.components.mark(marker);
        self.alpha.mark(marker);
    }
}

fn maybe_specified_color_to_value(color: Option<SpecifiedColor>, ruby: &Ruby) -> Value {
    match color {
        Some(color) => make_color(&color, ruby),
        None => ruby.qnil().into_value_with(ruby),
    }
}

fn color_function_origin(color_function: &ColorFunction<SpecifiedColor>) -> Option<SpecifiedColor> {
    match color_function {
        ColorFunction::Rgb(origin_color, ..)
        | ColorFunction::Hsl(origin_color, ..)
        | ColorFunction::Hwb(origin_color, ..)
        | ColorFunction::Lab(origin_color, ..)
        | ColorFunction::Lch(origin_color, ..)
        | ColorFunction::Oklab(origin_color, ..)
        | ColorFunction::Oklch(origin_color, ..)
        | ColorFunction::Color(origin_color, ..) => origin_color.as_ref().cloned(),
    }
}

fn color_function_components(color_function: &ColorFunction<SpecifiedColor>) -> Vec<StoredColorFunctionComponent> {
    match color_function {
        ColorFunction::Rgb(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrPercentage(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c2.clone()),
        ],
        ColorFunction::Lab(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrPercentage(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c2.clone()),
        ],
        ColorFunction::Oklab(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrPercentage(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c2.clone()),
        ],
        ColorFunction::Color(_, c0, c1, c2, _, _) => vec![
            StoredColorFunctionComponent::NumberOrPercentage(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c2.clone()),
        ],
        ColorFunction::Hsl(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrAngle(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c2.clone()),
        ],
        ColorFunction::Hwb(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrAngle(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c2.clone()),
        ],
        ColorFunction::Lch(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrPercentage(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrAngle(c2.clone()),
        ],
        ColorFunction::Oklch(_, c0, c1, c2, _) => vec![
            StoredColorFunctionComponent::NumberOrPercentage(c0.clone()),
            StoredColorFunctionComponent::NumberOrPercentage(c1.clone()),
            StoredColorFunctionComponent::NumberOrAngle(c2.clone()),
        ],
    }
}

fn color_function_alpha(color_function: &ColorFunction<SpecifiedColor>) -> StoredColorFunctionComponent {
    match color_function {
        ColorFunction::Rgb(_, _, _, _, alpha)
        | ColorFunction::Hsl(_, _, _, _, alpha)
        | ColorFunction::Hwb(_, _, _, _, alpha)
        | ColorFunction::Lab(_, _, _, _, alpha)
        | ColorFunction::Lch(_, _, _, _, alpha)
        | ColorFunction::Oklab(_, _, _, _, alpha)
        | ColorFunction::Oklch(_, _, _, _, alpha)
        | ColorFunction::Color(_, _, _, _, alpha, _) => StoredColorFunctionComponent::NumberOrPercentage(alpha.clone()),
    }
}
