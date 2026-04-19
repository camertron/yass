use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::transition_delay::SpecifiedValue, values::specified::Time};

use crate::{cached_value_list::CachedValueList, declarations::time::YTime};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TransitionDelay", mark)]
pub struct YTransitionDelay {
    times: CachedValueList<Time>
}

impl YTransitionDelay {
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

impl DataTypeFunctions for YTransitionDelay {
    fn mark(&self, marker: &gc::Marker) {
        self.times.mark(marker);
    }
}
