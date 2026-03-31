use magnus::{Error, IntoValue, RArray, Ruby, gc};
use selectors::parser::Selector;
use style::selector_parser::SelectorImpl;

use crate::{selectors::YSelector, cached_value_list::CachedValueList};

pub struct YSelectorList {
    pub selectors: CachedValueList<Selector<SelectorImpl>>
}

impl YSelectorList {
    pub fn new(selector_list: &[Selector<SelectorImpl>]) -> Self {
        Self {
            selectors: CachedValueList::new(selector_list.to_vec(), |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn empty() -> Self {
        Self {
            selectors: CachedValueList::new(vec![], |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn add_all(&self, selectors: Vec<Selector<SelectorImpl>>, ruby: &Ruby) {
        self.selectors.add_all(selectors, ruby);
    }

    pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.selectors.to_a(ruby)
    }

    pub fn is_empty(&self) -> bool {
        self.selectors.is_empty()
    }

    pub fn mark(&self, marker: &gc::Marker) {
        self.selectors.mark(marker);
    }
}
