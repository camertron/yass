use magnus::{Ruby, typed_data};
use style::values::specified::Integer;

#[magnus::wrap(class = "Yass::Declarations::Order")]
pub struct YOrder {
    integer: Integer
}

impl YOrder {
    pub fn new(integer: Integer) -> Self {
        Self { integer }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> i32 {
        rb_self.integer.value()
    }
}
