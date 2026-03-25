use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc};
use selectors::SelectorList;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelectorList;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Negation", mark)]
pub struct YNegation {
    cached_selectors: YSelectorList
}

impl DataTypeFunctions for YNegation {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_selectors.mark(marker);
    }
}

impl YNegation {
    pub fn new(selector_list: SelectorList<SelectorImpl>) -> Self {
        Self { cached_selectors: YSelectorList::new(selector_list.slice()) }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.cached_selectors.to_a(ruby)
    }
}
