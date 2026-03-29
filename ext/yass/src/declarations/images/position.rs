use magnus::{DataTypeFunctions, Ruby, Value, gc::Marker, typed_data};
use style::values::specified::position::{HorizontalPositionKeyword, Position, PositionComponent, VerticalPositionKeyword};

use crate::{cached_value::CachedValue, declarations::images::{horizontal_position_component::make_horizontal_position_component, vertical_position_component::make_vertical_position_component}};

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::Position", mark)]
pub struct YImagePosition {
    horizontal: CachedValue<PositionComponent<HorizontalPositionKeyword>>,
    vertical: CachedValue<PositionComponent<VerticalPositionKeyword>>,
}

impl YImagePosition {
    pub fn new(position: Position) -> Self {
        Self {
            horizontal: CachedValue::new(position.horizontal, |component, ruby| {
                make_horizontal_position_component(component.clone(), ruby)
            }),

            vertical: CachedValue::new(position.vertical, |component, ruby| {
                make_vertical_position_component(component.clone(), ruby)
            }),
        }
    }

    pub fn horizontal_position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.horizontal.get(ruby)
    }

    pub fn vertical_position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.vertical.get(ruby)
    }
}

impl DataTypeFunctions for YImagePosition {
    fn mark(&self, marker: &Marker) {
        self.horizontal.mark(marker);
        self.vertical.mark(marker);
    }
}
