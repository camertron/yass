use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::visibility::T;

#[magnus::wrap(class = "Yass::Declarations::Visibility")]
pub struct YVisibility {
    specified_value: T
}

impl YVisibility {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Visible => ruby.intern("visible"),
            T::Hidden => ruby.intern("hidden"),
            T::Collapse => ruby.intern("collapse"),
        }
    }
}
