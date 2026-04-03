use magnus::{RString, Ruby};

#[magnus::wrap(class = "Yass::Selector::DefaultNamespace")]
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
