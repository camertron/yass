use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::transition_duration::SpecifiedValue, values::specified::Time};

use crate::{cached_value_list::CachedValueList, declarations::time::YTime};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TransitionDuration", mark)]
pub struct YTransitionDuration {
    cached_times: CachedValueList<Time>
}

impl YTransitionDuration {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            cached_times: CachedValueList::new(specified_value.0.to_vec(), |time, _ctx, ruby| {
                YTime::new(time.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.cached_times.to_a(ruby)
    }
}

impl DataTypeFunctions for YTransitionDuration {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_times.mark(marker);
    }
}
