use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{Optional, length::{AnchorSizeKeyword, GenericAnchorSizeFunction, GenericMargin}}, specified::{LengthPercentage, Margin}};

use crate::{cached_value::CachedValue, declarations::size::{YLengthPercentage, anchor_size_keyword::YAnchorSizeKeyword, length_percentage_to_value}};

pub fn make_margin(margin: Margin, ruby: &Ruby) -> Value {
    match margin {
        Margin::Auto => YMarginAuto::new().into_value_with(ruby),
        Margin::LengthPercentage(length_percentage) => {
            YLengthPercentage::new(length_percentage).into_value_with(ruby)
        }
        Margin::AnchorSizeFunction(anchor_size_function) => {
            YMarginAnchorSizeFunction::new(*anchor_size_function).into_value_with(ruby)
        }
        Margin::AnchorContainingCalcFunction(length_percentage) => {
            YMarginAnchorContainingCalcFunction::new(length_percentage).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Margin::Auto")]
pub struct YMarginAuto {}

impl YMarginAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Margin::AnchorContainingCalcFunction", mark)]
pub struct YMarginAnchorContainingCalcFunction {
    length_percentage: CachedValue<LengthPercentage>,
}

impl YMarginAnchorContainingCalcFunction {
    pub fn new(length_percentage: LengthPercentage) -> Self {
        Self {
            length_percentage: CachedValue::new(length_percentage, |value, ruby| {
                length_percentage_to_value(value, ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }
}

impl DataTypeFunctions for YMarginAnchorContainingCalcFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Margin::AnchorSizeFunction", mark)]
pub struct YMarginAnchorSizeFunction {
    target_element: CachedValue<String>,
    size: CachedValue<AnchorSizeKeyword>,
    fallback: CachedValue<Optional<GenericMargin<LengthPercentage>>>
}

impl YMarginAnchorSizeFunction {
    pub fn new(anchor_size: GenericAnchorSizeFunction<GenericMargin<LengthPercentage>>) -> Self {
        Self {
            target_element: CachedValue::new(anchor_size.target_element.value.0.to_string(), |el, ruby| {
                ruby.str_new(el).into_value_with(ruby)
            }),

            size: CachedValue::new(anchor_size.size, |size, ruby| {
                YAnchorSizeKeyword::new(*size).into_value_with(ruby)
            }),

            fallback: CachedValue::new(anchor_size.fallback, |fallback, ruby| {
                match fallback {
                    Optional::Some(fb) => make_margin(fb.clone(), ruby),
                    Optional::None => ruby.qnil().into_value_with(ruby)
                }
            })
        }
    }

    pub fn target_element(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.target_element.get(ruby)
    }

    pub fn size(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.size.get(ruby)
    }

    pub fn fallback(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.fallback.get(ruby)
    }
}

impl DataTypeFunctions for YMarginAnchorSizeFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.target_element.mark(marker);
        self.size.mark(marker);
        self.fallback.mark(marker);
    }
}
