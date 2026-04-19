use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc};
use selectors::SelectorList;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelectorList;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Negation", mark)]
pub struct YNegation {
    selectors: YSelectorList
}

impl DataTypeFunctions for YNegation {
    fn mark(&self, marker: &gc::Marker) {
        self.selectors.mark(marker);
    }
}

impl YNegation {
    pub fn new(selector_list: SelectorList<SelectorImpl>) -> Self {
        Self { selectors: YSelectorList::new(selector_list.slice()) }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.selectors.to_a(ruby)
    }
}
