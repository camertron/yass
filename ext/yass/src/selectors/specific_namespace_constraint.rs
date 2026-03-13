use magnus::{RString, Ruby, typed_data};

#[magnus::wrap(class = "Yass::Selector::SpecificNamespaceConstraint")]
pub struct YSpecificNamespaceConstraint {
    prefix: String,
    url: String
}

impl YSpecificNamespaceConstraint {
    pub fn new(prefix: String, url: String) -> Self {
        Self { prefix, url }
    }

    pub fn prefix(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.prefix)
    }

    pub fn url(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.url)
    }
}
