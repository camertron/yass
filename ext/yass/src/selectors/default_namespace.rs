use magnus::{RString, Ruby};

pub struct YDefaultNamespace {
    url: String
}

impl YDefaultNamespace {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn url(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.url)
    }
}
