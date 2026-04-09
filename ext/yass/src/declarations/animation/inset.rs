use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::{animation::ViewTimelineInset, length::LengthPercentageOrAuto}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::{animation::length_auto::YLengthAuto, size::length_percentage_to_value}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Animation::Inset", mark)]
pub struct YInset {
    start: CachedValue<LengthPercentageOrAuto<LengthPercentage>>,
    end: CachedValue<LengthPercentageOrAuto<LengthPercentage>>
}

impl YInset {
    pub fn new(inset: ViewTimelineInset<LengthPercentage>) -> Self {
        Self {
            start: CachedValue::new(inset.start, |start, ruby| {
                match start {
                    LengthPercentageOrAuto::Auto => YLengthAuto::new().into_value_with(ruby),
                    LengthPercentageOrAuto::LengthPercentage(length) => {
                        length_percentage_to_value(length, ruby)
                    }
                }
            }),

            end: CachedValue::new(inset.end, |end, ruby| {
                match end {
                    LengthPercentageOrAuto::Auto => YLengthAuto::new().into_value_with(ruby),
                    LengthPercentageOrAuto::LengthPercentage(length) => {
                        length_percentage_to_value(length, ruby)
                    }
                }
            })
        }
    }

    pub fn start(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.start.get(ruby)
    }

    pub fn end(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.end.get(ruby)
    }
}

impl DataTypeFunctions for YInset {
    fn mark(&self, marker: &gc::Marker) {
        self.start.mark(marker);
        self.end.mark(marker);
    }
}
