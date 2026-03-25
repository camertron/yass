use magnus::{Error, IntoValue, RArray, Ruby, gc};
use selectors::parser::Selector;
use style::selector_parser::SelectorImpl;

use crate::{selectors::YSelector, value_list::ValueList};

pub struct YSelectorList {
    pub selectors: ValueList<Selector<SelectorImpl>>
}

impl YSelectorList {
    pub fn new(selector_list: &[Selector<SelectorImpl>]) -> Self {
        Self {
            selectors: ValueList::new(selector_list.to_vec(), |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn empty() -> Self {
        Self {
            selectors: ValueList::new(vec![], |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn add(&self, selector: Selector<SelectorImpl>, ruby: &Ruby) {
        self.selectors.add(selector, ruby);
    }

    pub fn add_all(&self, selectors: Vec<Selector<SelectorImpl>>, ruby: &Ruby) {
        self.selectors.add_all(selectors, ruby);
    }

    pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.selectors.to_a(ruby)
    }

    pub fn len(&self) -> usize {
        self.selectors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.selectors.is_empty()
    }

    pub fn mark(&self, marker: &gc::Marker) {
        self.selectors.mark(marker);
    }
}
