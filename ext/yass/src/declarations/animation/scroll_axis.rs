use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::ScrollAxis;

#[magnus::wrap(class = "Yass::Declarations::Animation::ScrollAxis")]
pub struct YScrollAxis {
    scroll_axis: ScrollAxis
}

impl YScrollAxis {
    pub fn new(scroll_axis: ScrollAxis) -> Self {
        Self { scroll_axis }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.scroll_axis {
            ScrollAxis::Block => ruby.intern("block"),
            ScrollAxis::Inline => ruby.intern("inline"),
            ScrollAxis::X => ruby.intern("x"),
            ScrollAxis::Y => ruby.intern("y"),
        }
    }
}
