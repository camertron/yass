use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data, value::{Opaque, ReprValue}};
use selectors::parser::Selector;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelector;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Host", mark)]
pub struct YHost {
    selector: Option<Opaque<typed_data::Obj<YSelector>>>
}

impl YHost {
    pub fn new(selector: Option<Selector<SelectorImpl>>, ruby: &Ruby) -> Self {
        Self {
            selector: selector.and_then(|f|
                Some(Opaque::from(ruby.obj_wrap(YSelector::new(f))))
            )
        }
    }

    pub fn selector(&self, ruby: &Ruby) -> Option<Value> {
        self.selector.and_then(|f|
            Some(ruby.get_inner(f).as_value())
        )
    }
}

impl DataTypeFunctions for YHost {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(selector) = self.selector {
            marker.mark(selector);
        }
    }
}
