use magnus::{RString, Ruby};
use selectors::parser::LocalName;
use style::selector_parser::SelectorImpl;

pub struct YLocalName {
    local_name: LocalName<SelectorImpl>
}

impl YLocalName {
    pub fn new(local_name: LocalName<SelectorImpl>) -> Self {
        Self { local_name }
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.local_name.name.to_string())
    }
}
