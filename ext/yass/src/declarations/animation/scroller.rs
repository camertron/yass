use magnus::{Ruby, typed_data, value::Id};
use style::values::specified::animation::Scroller;

#[magnus::wrap(class = "Yass::Declarations::Animation::Scroller")]
pub struct YScroller {
    scroller: Scroller
}

impl YScroller {
    pub fn new(scroller: Scroller) -> Self {
        Self { scroller }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.scroller {
            Scroller::Nearest => ruby.intern("nearest"),
            Scroller::Root => ruby.intern("root"),
            Scroller::SelfElement => ruby.intern("self_element"),
        }
    }
}
