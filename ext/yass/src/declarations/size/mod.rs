use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{NonNegative, length::Size}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::{calc::YCalc, length::YLength, percentage::YPercentage, size::anchor_size_function::YAnchorSizeFunction}};

pub mod anchor_size_function;
pub mod init;

pub use init::*;

pub fn make_size(size: Size<NonNegative<LengthPercentage>>, ruby: &Ruby) -> Value {
    match size {
        Size::LengthPercentage(percentage) => {
            YLengthPercentage::new(percentage).into_value_with(ruby)
        },

        Size::Auto => YAuto::new().into_value_with(ruby),
        Size::MaxContent => YMaxContent::new().into_value_with(ruby),
        Size::MinContent => YMinContent::new().into_value_with(ruby),
        Size::FitContent => YFitContent::new().into_value_with(ruby),
        Size::WebkitFillAvailable => YWebkitFillAvailable::new().into_value_with(ruby),
        Size::Stretch => YStretch::new().into_value_with(ruby),

        Size::FitContentFunction(fit_content_function) => {
            YFitContentFunction::new(fit_content_function).into_value_with(ruby)
        },

        Size::AnchorSizeFunction(generic_anchor_size_function) => {
            YAnchorSizeFunction::new((*generic_anchor_size_function).clone()).into_value_with(ruby)
        },

        Size::AnchorContainingCalcFunction(anchor_with_calc) => {
            YAnchorContainingCalcFunction::new(anchor_with_calc).into_value_with(ruby)
        },
    }
}

pub fn length_percentage_to_value(length_percentage: LengthPercentage, ruby: &Ruby) -> Value {
    match length_percentage {
        LengthPercentage::Length(no_calc_length) => {
            YLength::make(no_calc_length, ruby)
        },

        LengthPercentage::Percentage(percentage) => {
            YPercentage::new(percentage.0).into_value_with(ruby)
        },

        LengthPercentage::Calc(calc_length_percentage) => {
            YCalc::new(calc_length_percentage.clone()).into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Size::LengthPercentage", mark)]
pub struct YLengthPercentage {
    length_percentage: CachedValue<NonNegative<LengthPercentage>>
}

impl YLengthPercentage {
    pub fn new(length_percentage: NonNegative<LengthPercentage>) -> Self {
        Self {
            length_percentage: CachedValue::new(length_percentage, |lp, ruby| {
                length_percentage_to_value(lp.0.clone(), ruby)
            })
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }
}

impl DataTypeFunctions for YLengthPercentage {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Size::Auto")]
pub struct YAuto {
}

impl YAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Size::MaxContent")]
pub struct YMaxContent {
}

impl YMaxContent {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Size::MinContent")]
pub struct YMinContent {
}

impl YMinContent {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Size::FitContent")]
pub struct YFitContent {
}

impl YFitContent {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Size::WebkitFillAvailable")]
pub struct YWebkitFillAvailable {
}

impl YWebkitFillAvailable {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Size::Stretch")]
pub struct YStretch {
}

impl YStretch {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Size::FitContentFunction", mark)]
pub struct YFitContentFunction {
    fit_content_function: CachedValue<NonNegative<LengthPercentage>>
}

impl YFitContentFunction {
    pub fn new(fit_content_function: NonNegative<LengthPercentage>) -> Self {
        Self {
            fit_content_function: CachedValue::new(fit_content_function, |fcf, ruby| {
                length_percentage_to_value(fcf.0.clone(), ruby)
            })
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.fit_content_function.get(ruby)
    }
}

impl DataTypeFunctions for YFitContentFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.fit_content_function.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Size::AnchorContainingCalcFunction", mark)]
pub struct YAnchorContainingCalcFunction {
    anchor_containing_calc_function: CachedValue<NonNegative<LengthPercentage>>
}

impl YAnchorContainingCalcFunction {
    pub fn new(anchor_containing_calc_function: NonNegative<LengthPercentage>) -> Self {
        Self {
            anchor_containing_calc_function: CachedValue::new(anchor_containing_calc_function, |accf, ruby| {
                length_percentage_to_value(accf.0.clone(), ruby)
            })
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.anchor_containing_calc_function.get(ruby)
    }
}

impl DataTypeFunctions for YAnchorContainingCalcFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.anchor_containing_calc_function.mark(marker);
    }
}
