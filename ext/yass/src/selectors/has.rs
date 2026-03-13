use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc};
use selectors::parser::RelativeSelector;
use style::selector_parser::SelectorImpl;

use crate::selectors::YRelativeSelectorList;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Has", mark)]
pub struct YHas {
    relative_selector_list: YRelativeSelectorList
}

impl YHas {
    pub fn new(relative_selectors: Box<[RelativeSelector<SelectorImpl>]>, ruby: &Ruby) -> Self {
        let relative_selector_list = YRelativeSelectorList::new(&relative_selectors, ruby);
        Self { relative_selector_list }
    }

    pub fn relative_selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.relative_selector_list.to_a(ruby)
    }
}

impl DataTypeFunctions for YHas {
    fn mark(&self, marker: &gc::Marker) {
        self.relative_selector_list.mark(marker);
    }
}
