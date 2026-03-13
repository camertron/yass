use magnus::{Integer, Ruby, typed_data};

#[magnus::wrap(class = "Yass::Selector::AnPlusB")]
pub struct YAnPlusB {
    a: i32,
    b: i32
}

impl YAnPlusB {
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    pub fn a(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Integer {
        ruby.integer_from_i128(rb_self.a as i128)
    }

    pub fn b(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Integer {
        ruby.integer_from_i128(rb_self.b as i128)
    }
}
