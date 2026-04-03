use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{generics::{Optional, animation::AnimationRangeValue}, specified::{LengthPercentage, animation::TimelineRangeName}};

use crate::{cached_value::CachedValue, declarations::size::length_percentage_to_value};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Animation::RangeValue", mark)]
pub struct YRangeValue {
    length_percentage: CachedValue<Optional<LengthPercentage>>,
    name: TimelineRangeName
}

impl YRangeValue {
    pub fn new(value: AnimationRangeValue<LengthPercentage>) -> Self {
        Self {
            length_percentage: CachedValue::new(value.lp, |lp, ruby| {
                match lp {
                    Optional::Some(lp) => length_percentage_to_value(lp.clone(), ruby),
                    Optional::None => ruby.qnil().into_value_with(ruby)
                }
            }),

            name: value.name
        }
    }

    pub fn length_percentage(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length_percentage.get(ruby)
    }

    pub fn name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.name {
            TimelineRangeName::None => ruby.intern("none"),
            TimelineRangeName::Cover => ruby.intern("cover"),
            TimelineRangeName::Contain => ruby.intern("contain"),
            TimelineRangeName::Entry => ruby.intern("entry"),
            TimelineRangeName::Exit => ruby.intern("exit"),
            TimelineRangeName::EntryCrossing => ruby.intern("entry_crossing"),
            TimelineRangeName::ExitCrossing => ruby.intern("exit_crossing"),
            TimelineRangeName::Scroll => ruby.intern("scroll"),
        }
    }
}

impl DataTypeFunctions for YRangeValue {
    fn mark(&self, marker: &gc::Marker) {
        self.length_percentage.mark(marker);
    }
}
