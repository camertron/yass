use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::generics::font::GenericLineHeight;
use style::values::specified::{LineHeight};

use crate::declarations::size::length_percentage_to_value;
use crate::{cached_value::CachedValue, declarations::{number::YNumber}};

fn make_line_height(line_height: &LineHeight, ruby: &Ruby) -> Value {
    match line_height {
        GenericLineHeight::Normal => YLineHeightNormal::new().into_value_with(ruby),
        GenericLineHeight::Number(non_negative_number) => {
            YNumber::new(non_negative_number.get()).into_value_with(ruby).into_value_with(ruby)
        },
        GenericLineHeight::Length(non_negative_lp) => {
            length_percentage_to_value(&non_negative_lp.0, ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::LineHeight", mark)]
pub struct YLineHeight {
    value: CachedValue<LineHeight>,
}

impl YLineHeight {
    pub fn new(line_height: LineHeight) -> Self {
        Self {
            value: CachedValue::new(line_height, make_line_height),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YLineHeight {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::LineHeight::Normal")]
pub struct YLineHeightNormal {}

impl YLineHeightNormal {
    pub fn new() -> Self {
        Self {}
    }
}
