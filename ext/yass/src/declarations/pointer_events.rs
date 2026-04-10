use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::PointerEvents;

#[magnus::wrap(class = "Yass::Declarations::PointerEvents")]
pub struct YPointerEvents {
    pointer_events: PointerEvents
}

impl YPointerEvents {
    pub fn new(pointer_events: PointerEvents) -> Self {
        Self { pointer_events }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.pointer_events {
            PointerEvents::Auto => ruby.intern("auto"),
            PointerEvents::None => ruby.intern("none"),
        }
    }
}
