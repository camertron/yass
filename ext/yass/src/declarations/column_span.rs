use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::column_span::T;

#[magnus::wrap(class = "Yass::Declarations::ColumnSpan")]
pub struct YColumnSpan {
    specified_value: T
}

impl YColumnSpan {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::None => ruby.intern("none"),
            T::All => ruby.intern("all"),
        }
    }
}
