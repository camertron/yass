use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::animation_delay::SpecifiedValue, values::specified::Time};

use crate::{declarations::time::YTime, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationDelay", mark)]
pub struct YAnimationDelay {
    times: CachedValueList<Time>
}

impl YAnimationDelay {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            times: CachedValueList::new(specified_value.0.to_vec(), |time, _ctx, ruby| {
                YTime::new(time.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.times.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationDelay {
    fn mark(&self, marker: &gc::Marker) {
        self.times.mark(marker);
    }
}
