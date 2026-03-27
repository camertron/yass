use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{computed::ScrollAxis, generics::animation::{ViewFunction, ViewTimelineInset}, specified::LengthPercentage};

use crate::{cached_value::CachedValue, declarations::animation::{inset::YInset, scroll_axis::YScrollAxis}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Animation::ViewFunction", mark)]
pub struct YViewFunction {
    scroll_axis: CachedValue<ScrollAxis>,
    inset: CachedValue<ViewTimelineInset<LengthPercentage>>
}

impl YViewFunction {
    pub fn new(view_function: ViewFunction<LengthPercentage>) -> Self {
        Self {
            scroll_axis: CachedValue::new(view_function.axis, |axis, ruby| {
                YScrollAxis::new(*axis).into_value_with(ruby)
            }),

            inset: CachedValue::new(view_function.inset, |inset, ruby| {
                YInset::new(inset.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn scroll_axis(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.scroll_axis.get(ruby)
    }

    pub fn inset(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.inset.get(ruby)
    }
}

impl DataTypeFunctions for YViewFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.scroll_axis.mark(marker);
        self.inset.mark(marker);
    }
}
