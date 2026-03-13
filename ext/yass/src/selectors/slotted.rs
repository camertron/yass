use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data, value::{Opaque, ReprValue}};
use selectors::parser::Selector;
use style::selector_parser::SelectorImpl;

use crate::selectors::YSelector;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Slotted", mark)]
pub struct YSlotted {
    selector: Opaque<typed_data::Obj<YSelector>>
}

impl YSlotted {
    pub fn new(selector: Selector<SelectorImpl>, ruby: &Ruby) -> Self {
        let yselector = Opaque::from(ruby.obj_wrap(YSelector::new(selector)));
        Self { selector: yselector }
    }

    pub fn selector(&self, ruby: &Ruby) -> Value {
        ruby.get_inner(self.selector).as_value()
    }
}

impl DataTypeFunctions for YSlotted {
    fn mark(&self, marker: &gc::Marker) {
        marker.mark(self.selector);
    }
}
