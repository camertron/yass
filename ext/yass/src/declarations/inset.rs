use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::{Inset, LengthPercentage};
use style_traits::ToCss;

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

pub fn make_inset(inset: Inset, ruby: &Ruby) -> Value {
    match inset {
        Inset::Auto => YInsetAuto::new().into_value_with(ruby),
        Inset::LengthPercentage(length_percentage) => {
            YInsetLengthPercentage::new(length_percentage).into_value_with(ruby)
        }
        Inset::AnchorContainingCalcFunction(length_percentage) => {
            YInsetAnchorContainingCalcFunction::new(length_percentage).into_value_with(ruby)
        }
        Inset::AnchorFunction(anchor_function) => {
            YInsetAnchorFunction::new(anchor_function.to_css_string()).into_value_with(ruby)
        }
        Inset::AnchorSizeFunction(anchor_size_function) => {
            YInsetAnchorSizeFunction::new(anchor_size_function.to_css_string()).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Inset::Auto")]
pub struct YInsetAuto {}

impl YInsetAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Inset::LengthPercentage", mark)]
pub struct YInsetLengthPercentage {
    length_percentage: CachedValue<LengthPercentage>,
}

impl YInsetLengthPercentage {
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

impl DataTypeFunctions for YInsetLengthPercentage {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Inset::AnchorContainingCalcFunction", mark)]
pub struct YInsetAnchorContainingCalcFunction {
    length_percentage: CachedValue<LengthPercentage>,
}

impl YInsetAnchorContainingCalcFunction {
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

impl DataTypeFunctions for YInsetAnchorContainingCalcFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Inset::AnchorFunction")]
pub struct YInsetAnchorFunction {
    css: String,
}

impl YInsetAnchorFunction {
    pub fn new(css: String) -> Self {
        Self { css }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(&rb_self.css).into_value_with(ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Inset::AnchorSizeFunction")]
pub struct YInsetAnchorSizeFunction {
    css: String,
}

impl YInsetAnchorSizeFunction {
    pub fn new(css: String) -> Self {
        Self { css }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(&rb_self.css).into_value_with(ruby)
    }
}
