use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::properties::longhands::background_position_x::{SingleSpecifiedValue, SpecifiedValue};

use crate::{cached_value_list::CachedValueList, declarations::horizontal_position_component::make_horizontal_position_component};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackgroundPositionX", mark)]
pub struct YBackgroundPositionX {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YBackgroundPositionX {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |component, _ctx, ruby| {
                make_horizontal_position_component(component.clone(), ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBackgroundPositionX {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
