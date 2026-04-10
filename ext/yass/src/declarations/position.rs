use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::PositionProperty;

#[magnus::wrap(class = "Yass::Declarations::Position")]
pub struct YPosition {
    position_property: PositionProperty
}

impl YPosition {
    pub fn new(position_property: PositionProperty) -> Self {
        Self { position_property }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.position_property {
            PositionProperty::Static => ruby.intern("static"),
            PositionProperty::Relative => ruby.intern("relative"),
            PositionProperty::Absolute => ruby.intern("absolute"),
            PositionProperty::Fixed => ruby.intern("fixed"),
            PositionProperty::Sticky => ruby.intern("sticky"),
        }
    }
}
