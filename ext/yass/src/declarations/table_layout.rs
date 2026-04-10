use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::table_layout::T;

#[magnus::wrap(class = "Yass::Declarations::TableLayout")]
pub struct YTableLayout {
    specified_value: T,
}

impl YTableLayout {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Auto => ruby.intern("auto"),
            T::Fixed => ruby.intern("fixed"),
        }
    }
}
