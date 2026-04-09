use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::box_::BaselineShiftKeyword, specified::{BaselineShift, LengthPercentage}};

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

pub fn make_baseline_shift(baseline_shift: BaselineShift, ruby: &Ruby) -> Value {
    match baseline_shift {
        BaselineShift::Keyword(keyword) => {
            YBaselineShiftKeyword::new(keyword).into_value_with(ruby)
        },

        BaselineShift::Length(length_percentage) => {
            YBaselineShiftLength::new(length_percentage).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::BaselineShift::Keyword")]
pub struct YBaselineShiftKeyword {
    keyword: BaselineShiftKeyword,
}

impl YBaselineShiftKeyword {
    pub fn new(keyword: BaselineShiftKeyword) -> Self {
        Self { keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match &rb_self.keyword {
            BaselineShiftKeyword::Sub => ruby.intern("sub"),
            BaselineShiftKeyword::Super => ruby.intern("super"),
            BaselineShiftKeyword::Top => ruby.intern("top"),
            BaselineShiftKeyword::Center => ruby.intern("center"),
            BaselineShiftKeyword::Bottom => ruby.intern("bottom"),
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BaselineShift::Length", mark)]
pub struct YBaselineShiftLength {
    value: CachedValue<LengthPercentage>,
}

impl YBaselineShiftLength {
    pub fn new(length_percentage: LengthPercentage) -> Self {
        Self {
            value: CachedValue::new(length_percentage, |length_percentage, ruby| {
                length_percentage_to_value(length_percentage, ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YBaselineShiftLength {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
