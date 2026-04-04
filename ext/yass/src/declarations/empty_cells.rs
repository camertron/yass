use magnus::{Ruby, typed_data, value::Id};
use style::computed_values::empty_cells::T;

#[magnus::wrap(class = "Yass::Declarations::EmptyCells")]
pub struct YEmptyCells {
    specified_value: T
}

impl YEmptyCells {
    pub fn new(specified_value: T) -> Self {
        Self { specified_value }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.specified_value {
            T::Show => ruby.intern("show"),
            T::Hide => ruby.intern("hide"),
        }
    }
}
