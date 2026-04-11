use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::white_space_collapse::T;

#[magnus::wrap(class = "Yass::Declarations::WhiteSpaceCollapse")]
pub struct YWhiteSpaceCollapse {
    specified_value: T
}

impl YWhiteSpaceCollapse {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Collapse => ruby.intern("collapse"),
            T::Preserve => ruby.intern("preserve"),
            T::PreserveBreaks => ruby.intern("preserve_breaks"),
            T::BreakSpaces => ruby.intern("break_spaces"),
        }
    }
}
