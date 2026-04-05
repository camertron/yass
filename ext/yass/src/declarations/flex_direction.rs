use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::flex_direction::T;

#[magnus::wrap(class = "Yass::Declarations::FlexDirection")]
pub struct YFlexDirection {
    specified_value: T
}

impl YFlexDirection {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Row => ruby.intern("row"),
            T::RowReverse => ruby.intern("row_reverse"),
            T::Column => ruby.intern("column"),
            T::ColumnReverse => ruby.intern("column_reverse"),
        }
    }
}
