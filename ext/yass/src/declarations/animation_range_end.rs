use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{properties::longhands::animation_range_end::SpecifiedValue, values::{generics::{animation::AnimationRangeEnd}, specified::{LengthPercentage}}};

use crate::{declarations::animation::range_value::YRangeValue, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationRangeEnd")]
pub struct YAnimationRangeEnd {
    values: CachedValueList<AnimationRangeEnd<LengthPercentage>>
}

impl YAnimationRangeEnd {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                YRangeValue::new(value.0.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YAnimationRangeEnd {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
