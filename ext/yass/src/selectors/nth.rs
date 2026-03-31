use std::cell::RefCell;

use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data, value::{Id, Opaque, ReprValue}};
use selectors::parser::{NthSelectorData, NthType};

use crate::selectors::YAnPlusB;

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Nth", mark)]
pub struct YNth {
    nth_selector_data: NthSelectorData,
    cached_an_plus_b: RefCell<Option<Opaque<typed_data::Obj<YAnPlusB>>>>
}

impl YNth {
    pub fn new(nth_selector_data: NthSelectorData) -> Self {
        Self { nth_selector_data, cached_an_plus_b: RefCell::new(None) }
    }

    pub fn ty(&self, ruby: &Ruby) -> Id {
        match self.nth_selector_data.ty {
            NthType::Child => ruby.intern("child"),
            NthType::LastChild => ruby.intern("last_child"),
            NthType::OnlyChild => ruby.intern("only_child"),
            NthType::OfType => ruby.intern("of_type"),
            NthType::LastOfType => ruby.intern("last_of_type"),
            NthType::OnlyOfType => ruby.intern("only_of_type"),
        }
    }

    pub fn is_function(&self, ruby: &Ruby) -> Value {
        if self.nth_selector_data.is_function {
            ruby.qtrue().as_value()
        } else {
            ruby.qfalse().as_value()
        }
    }

    pub fn an_plus_b(&self, ruby: &Ruby) -> Value {
        if self.cached_an_plus_b.borrow().is_none() {
            let an_plus_b = YAnPlusB::new(
                self.nth_selector_data.an_plus_b.0,
                self.nth_selector_data.an_plus_b.1
            );

            *self.cached_an_plus_b.borrow_mut() = Some(Opaque::from(ruby.obj_wrap(an_plus_b)));
        }

        let cached_an_plus_b = self.cached_an_plus_b.borrow().unwrap();
        ruby.get_inner(cached_an_plus_b).as_value()
    }
}

impl DataTypeFunctions for YNth {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(an_plus_b) = self.cached_an_plus_b.borrow().as_ref() {
            marker.mark(*an_plus_b);
        }
    }
}
