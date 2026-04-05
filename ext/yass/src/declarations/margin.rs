use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::{LengthPercentage, Margin};
use style_traits::ToCss;

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

pub fn make_margin(margin: Margin, ruby: &Ruby) -> Value {
    match margin {
        Margin::Auto => YMarginAuto::new().into_value_with(ruby),
        Margin::LengthPercentage(length_percentage) => {
            YMarginLengthPercentage::new(length_percentage).into_value_with(ruby)
        }
        Margin::AnchorSizeFunction(anchor_size_function) => {
            YMarginAnchorSizeFunction::new(anchor_size_function.to_css_string()).into_value_with(ruby)
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
#[magnus(class = "Yass::Declarations::Margin::LengthPercentage", mark)]
pub struct YMarginLengthPercentage {
    length_percentage: CachedValue<LengthPercentage>,
}

impl YMarginLengthPercentage {
    pub fn new(length_percentage: LengthPercentage) -> Self {
        Self {
            length_percentage: CachedValue::new(length_percentage, |value, ruby| {
                length_percentage_to_value(value.clone(), ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }
}

impl DataTypeFunctions for YMarginLengthPercentage {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
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
                length_percentage_to_value(value.clone(), ruby)
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

#[magnus::wrap(class = "Yass::Declarations::Margin::AnchorSizeFunction")]
pub struct YMarginAnchorSizeFunction {
    css: String,
}

impl YMarginAnchorSizeFunction {
    pub fn new(css: String) -> Self {
        Self { css }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(&rb_self.css).into_value_with(ruby)
    }
}