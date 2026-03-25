use magnus::{Ruby, typed_data};

#[magnus::wrap(class = "Yass::Declarations::Number")]
pub struct YNumber {
    value: f32
}

impl YNumber {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.value
    }
}
