use magnus::{RString, Ruby};

#[magnus::wrap(class = "Yass::Selector::Namespace")]
pub struct YNamespace {
    prefix: String,
    url: String
}

impl YNamespace {
    pub fn new(prefix: String, url: String) -> Self {
        Self { prefix, url }
    }

    pub fn prefix(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.prefix)
    }

    pub fn url(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.url)
    }
}
