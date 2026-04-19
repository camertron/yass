use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, value::{Id, ReprValue}};
use selectors::parser::{NthSelectorData, NthType};

use crate::{cached_value::CachedValue, selectors::YAnPlusB};

#[derive(TypedData)]
#[magnus(class = "Yass::Selector::Nth", mark)]
pub struct YNth {
    nth_selector_data: NthSelectorData,
    an_plus_b: CachedValue<(i32, i32)>
}

impl YNth {
    pub fn new(nth_selector_data: NthSelectorData) -> Self {
        Self {
            nth_selector_data,
            an_plus_b: CachedValue::new((nth_selector_data.an_plus_b.0, nth_selector_data.an_plus_b.1), |values, ruby| {
                YAnPlusB::new(values.0, values.1).into_value_with(ruby)
            })
        }
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
        self.an_plus_b.get(ruby)
    }
}

impl DataTypeFunctions for YNth {
    fn mark(&self, marker: &gc::Marker) {
        self.an_plus_b.mark(marker);
    }
}
