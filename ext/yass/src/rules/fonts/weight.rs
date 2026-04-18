use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::{font_face::FontWeightRange, values::specified::font::AbsoluteFontWeight};

use crate::{cached_value::CachedValue, declarations::number::YNumber};

pub fn font_weight_to_value(weight: &AbsoluteFontWeight, ruby: &Ruby) -> Value {
    match weight {
        AbsoluteFontWeight::Weight(number) => {
            YNumber::new(number.get()).into_value_with(ruby)
        },

        AbsoluteFontWeight::Normal => {
            YAbsoluteFontWeightNormal::new().into_value_with(ruby)
        },

        AbsoluteFontWeight::Bold => {
            YAbsoluteFontWeightAuto::new().into_value_with(ruby)
        },
    }
}

#[magnus::wrap(class = "Yass::FontWeight::Normal", mark)]
pub struct YAbsoluteFontWeightNormal {}

impl YAbsoluteFontWeightNormal {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::FontWeight::Normal", mark)]
pub struct YAbsoluteFontWeightAuto {}

impl YAbsoluteFontWeightAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::FontWeight::Range", mark)]
pub struct YFontWeightRange {
    begin: CachedValue<AbsoluteFontWeight>,
    end: CachedValue<AbsoluteFontWeight>,
}

impl YFontWeightRange {
    pub fn new(range: FontWeightRange) -> Self {
        Self {
            begin: CachedValue::new(range.0, |weight, ruby| {
                font_weight_to_value(weight, ruby)
            }),

            end: CachedValue::new(range.1, |weight, ruby| {
                font_weight_to_value(weight, ruby)
            }),
        }
    }

    pub fn begin(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.begin.get(ruby)
    }

    pub fn end(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.end.get(ruby)
    }
}

impl DataTypeFunctions for YFontWeightRange {
    fn mark(&self, marker: &gc::Marker) {
        self.begin.mark(marker);
        self.end.mark(marker);
    }
}
