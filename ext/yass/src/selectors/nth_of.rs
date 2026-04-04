use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc};
use selectors::parser::{NthOfSelectorData, NthSelectorData, Selector};
use style::selector_parser::SelectorImpl;

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, selectors::{YNth, YSelector}};

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::NthOf", mark)]
pub struct YNthOf {
    // nth_of_selector_data: NthOfSelectorData<SelectorImpl>,
    cached_nth: CachedValue<NthSelectorData>,
    cached_selectors: CachedValueList<Selector<SelectorImpl>>
}

impl YNthOf {
    pub fn new(nth_of_selector_data: NthOfSelectorData<SelectorImpl>) -> Self {
        YNthOf {
            // nth_of_selector_data,
            cached_nth: CachedValue::new(nth_of_selector_data.nth_data().clone(), |nth_data, ruby| {
                YNth::new(*nth_data).into_value_with(ruby)
            }),

            cached_selectors: CachedValueList::new(nth_of_selector_data.selectors().to_vec(), |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn nth(&self, ruby: &Ruby) -> Value {
        self.cached_nth.get(ruby)
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        self.cached_selectors.to_a(ruby)
    }
}

impl DataTypeFunctions for YNthOf {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_nth.mark(marker);
        self.cached_selectors.mark(marker);
    }
}
