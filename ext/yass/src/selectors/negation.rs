use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc};
use selectors::SelectorList;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelectorList;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Negation", mark)]
pub struct YNegation {
    selector_list: SelectorList<SelectorImpl>,
    cached_selectors: RefCell<Option<YSelectorList>>
}

impl DataTypeFunctions for YNegation {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(selectors) = self.cached_selectors.borrow().as_ref() {
            selectors.mark(marker);
        }
    }
}

impl YNegation {
    pub fn new(selector_list: SelectorList<SelectorImpl>) -> Self {
        Self { selector_list, cached_selectors: RefCell::new(None) }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_selectors.borrow().is_none() {
            *self.cached_selectors.borrow_mut() = Some(
                YSelectorList::new(&self.selector_list.slice(), ruby)
            );
        }

        self.cached_selectors.borrow().as_ref().unwrap().to_a(ruby)
    }
}
