use magnus::{RString, Ruby};

pub struct YAttributeInNoNamespaceExists {
    local_name: String
}

impl YAttributeInNoNamespaceExists {
    pub fn new(local_name: String) -> Self {
        Self { local_name }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.local_name.to_string())
    }
}
