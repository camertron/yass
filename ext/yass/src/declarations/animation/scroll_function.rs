use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::values::{computed::ScrollAxis, specified::animation::{ScrollFunction, Scroller}};

use crate::{cached_value::CachedValue, declarations::animation::{scroll_axis::YScrollAxis, scroller::YScroller}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Animation::ScrollFunction", mark)]
pub struct YScrollFunction {
    scroller: CachedValue<Scroller>,
    scroll_axis: CachedValue<ScrollAxis>
}

impl YScrollFunction {
    pub fn new(scroll_function: ScrollFunction) -> Self {
        YScrollFunction {
            scroller: CachedValue::new(scroll_function.scroller, |scroller, ruby| {
                YScroller::new(*scroller).into_value_with(ruby)
            }),

            scroll_axis: CachedValue::new(scroll_function.axis, |axis, ruby| {
                YScrollAxis::new(*axis).into_value_with(ruby)
            })
        }
    }

    pub fn scroller(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.scroller.get(ruby)
    }

    pub fn scroll_axis(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.scroll_axis.get(ruby)
    }
}

impl DataTypeFunctions for YScrollFunction {
    fn mark(&self, marker: &gc::Marker) {
        self.scroller.mark(marker);
        self.scroll_axis.mark(marker);
    }
}
