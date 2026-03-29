use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::specified::url::SpecifiedUrl;

#[magnus::wrap(class = "Yass::Declarations::Image::Url")]
pub struct YImageUrl {
    url: SpecifiedUrl,
}

impl YImageUrl {
    pub fn new(url: SpecifiedUrl) -> Self {
        Self { url }
    }

    pub fn resolved(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        match rb_self.url.url() {
            Some(url) => ruby.str_new(url.as_str()).into_value_with(ruby),
            None => ruby.qnil().into_value_with(ruby),
        }
    }

    pub fn original(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        ruby.str_new(rb_self.url.as_str()).into_value_with(ruby)
    }

    pub fn invalid(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.url.is_invalid()
    }
}
