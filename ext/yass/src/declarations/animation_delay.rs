use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::animation_delay::SpecifiedValue, values::specified::Time};

use crate::{declarations::time::YTime, value_list::ValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationDelay", mark)]
pub struct YAnimationDelay {
    cached_times: ValueList<Time>
}

impl YAnimationDelay {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            cached_times: ValueList::new(specified_value.0.to_vec(), |time, _ctx, ruby| {
                YTime::new(time.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.cached_times.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationDelay {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_times.mark(marker);
    }
}
