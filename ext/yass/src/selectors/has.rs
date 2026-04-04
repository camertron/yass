use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc};
use selectors::parser::RelativeSelector;
use style::selector_parser::SelectorImpl;

use crate::{cached_value_list::CachedValueList, selectors::{YRelativeSelector}};

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Has", mark)]
pub struct YHas {
    relative_selectors: CachedValueList<RelativeSelector<SelectorImpl>>
}

impl YHas {
    pub fn new(relative_selectors: Box<[RelativeSelector<SelectorImpl>]>) -> Self {
        Self {
            relative_selectors: CachedValueList::new(relative_selectors.to_vec(), |relative_selector, _ctx, ruby| {
                YRelativeSelector::new(relative_selector.clone(), ruby).into_value_with(ruby)
            })
        }
    }

    pub fn relative_selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.relative_selectors.to_a(ruby)
    }
}

impl DataTypeFunctions for YHas {
    fn mark(&self, marker: &gc::Marker) {
        self.relative_selectors.mark(marker);
    }
}
