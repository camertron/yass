use magnus::{RString, Ruby};

#[magnus::wrap(class = "Yass::Selector::Klass")]
pub struct YClass {
    pub class: String
}

impl YClass {
    pub fn new(class: String) -> Self {
        Self { class }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(self.class.as_str())
    }
}
