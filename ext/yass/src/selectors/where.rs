use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc};
use selectors::SelectorList;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelectorList;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Where", mark)]
pub struct YWhere {
    selectors: YSelectorList
}

impl YWhere {
    pub fn new(selectors: SelectorList<SelectorImpl>) -> Self {
        Self { selectors: YSelectorList::new(selectors.slice()) }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.selectors.to_a(ruby)
    }
}

impl DataTypeFunctions for YWhere {
    fn mark(&self, marker: &gc::Marker) {
        self.selectors.mark(marker);
    }
}
