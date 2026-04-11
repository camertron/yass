use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::computed::XLang;

#[magnus::wrap(class = "Yass::Declarations::XLang")]
pub struct YXLang {
    xlang: XLang
}

impl YXLang {
    pub fn new(xlang: XLang) -> Self {
        Self { xlang }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_from_slice(rb_self.xlang.0.as_bytes()).into_value_with(ruby)
    }

    pub fn is_empty(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.xlang.0.as_bytes().is_empty()
    }
}
