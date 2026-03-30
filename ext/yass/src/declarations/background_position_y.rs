use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::properties::longhands::background_position_y::{SingleSpecifiedValue, SpecifiedValue};

use crate::{cached_value_list::CachedValueList, declarations::vertical_position_component::make_vertical_position_component};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackgroundPositionY", mark)]
pub struct YBackgroundPositionY {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YBackgroundPositionY {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |component, _ctx, ruby| {
                make_vertical_position_component(component.clone(), ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBackgroundPositionY {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}
