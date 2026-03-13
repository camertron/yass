use magnus::{RString, Ruby};

pub struct YID {
    id: String
}

impl YID {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(self.id.as_str())
    }
}
