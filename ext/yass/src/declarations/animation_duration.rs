use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data, value::{Id, Opaque, ReprValue}};
use style::{properties::longhands::animation_duration::SpecifiedValue, values::{generics::animation::AnimationDuration, specified::Time}};

use crate::{declarations::time::YTime, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationDuration", mark)]
pub struct YAnimationDuration {
    cached_values: CachedValueList<AnimationDuration<Time>>
}

impl YAnimationDuration {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            cached_values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                YAnimationDurationValue::new(value.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.cached_values.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationDuration {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_values.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationDurationValue", mark)]
pub struct YAnimationDurationValue {
    animation_duration: AnimationDuration<Time>,
    cached_time: RefCell<Option<Opaque<typed_data::Obj<YTime>>>>
}

impl YAnimationDurationValue {
    pub fn new(animation_duration: AnimationDuration<Time>) -> Self {
        Self { animation_duration, cached_time: RefCell::new(None) }
    }

    pub fn time(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Option<Value> {
        match rb_self.animation_duration {
            AnimationDuration::Time(time) => {
                if rb_self.cached_time.borrow().is_none() {
                    *rb_self.cached_time.borrow_mut() = Some(Opaque::from(ruby.obj_wrap(YTime::new(time))));
                }

                Some(ruby.get_inner(rb_self.cached_time.borrow().unwrap()).as_value())
            }

            AnimationDuration::Auto => None
        }
    }
}

impl DataTypeFunctions for YAnimationDurationValue {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(time) = self.cached_time.borrow().as_ref() {
            marker.mark(*time);
        }
    }
}
