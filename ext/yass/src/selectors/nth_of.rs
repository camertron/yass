use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, Value, gc, typed_data, value::{InnerValue, Opaque, ReprValue}};
use selectors::parser::NthOfSelectorData;
use style::selector_parser::SelectorImpl;

use crate::selectors::{YNth, YSelectorList};

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::NthOf", mark)]
pub struct YNthOf {
    nth_of_selector_data: NthOfSelectorData<SelectorImpl>,
    cached_nth: RefCell<Option<Opaque<typed_data::Obj<YNth>>>>,
    cached_selectors: RefCell<Option<YSelectorList>>
}

impl YNthOf {
    pub fn new(nth_of_selector_data: NthOfSelectorData<SelectorImpl>) -> Self {
        YNthOf {
            nth_of_selector_data,
            cached_nth: RefCell::new(None),
            cached_selectors: RefCell::new(None)
        }
    }

    pub fn nth(&self, ruby: &Ruby) -> Value {
        if self.cached_nth.borrow().is_none() {
            let nth = YNth::new(*self.nth_of_selector_data.nth_data());
            *self.cached_nth.borrow_mut() = Some(Opaque::from(ruby.obj_wrap(nth)));
        }

        let cached_nth = self.cached_nth.borrow().unwrap();
        ruby.get_inner(cached_nth).as_value()
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_selectors.borrow().is_none() {
            *self.cached_selectors.borrow_mut() = Some(
                YSelectorList::new(
                    self.nth_of_selector_data.selectors(),
                    ruby
                )
            );
        }

        self.cached_selectors.borrow().as_ref().unwrap().to_a(ruby)
    }
}

impl DataTypeFunctions for YNthOf {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(nth) = self.cached_nth.borrow().as_ref() {
            let ruby = Ruby::get().unwrap();
            marker.mark(nth.get_inner_with(&ruby));
        }

        if let Some(selectors) = self.cached_selectors.borrow().as_ref() {
            selectors.mark(marker);
        }
    }
}
