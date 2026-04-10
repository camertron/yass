use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::{generics::position::Position, specified::{PositionComponent, position::{HorizontalPositionKeyword, VerticalPositionKeyword}}};

use crate::{cached_value::CachedValue, declarations::{horizontal_position_component::make_horizontal_position_component, vertical_position_component::make_vertical_position_component}};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::PerspectiveOrigin", mark)]
pub struct YPerspectiveOrigin {
    horizontal: CachedValue<PositionComponent<HorizontalPositionKeyword>>,
    vertical: CachedValue<PositionComponent<VerticalPositionKeyword>>,
}

impl YPerspectiveOrigin {
    pub fn new(position: Box<Position<PositionComponent<HorizontalPositionKeyword>, PositionComponent<VerticalPositionKeyword>>>) -> Self {
        Self {
            horizontal: CachedValue::new(position.horizontal.clone(), |component, ruby| {
                make_horizontal_position_component(component.clone(), ruby)
            }),

            vertical: CachedValue::new(position.vertical.clone(), |component, ruby| {
                make_vertical_position_component(component.clone(), ruby)
            }),
        }
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.horizontal.get(ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.vertical.get(ruby)
    }
}

impl DataTypeFunctions for YPerspectiveOrigin {
    fn mark(&self, marker: &gc::Marker) {
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
    }
}
