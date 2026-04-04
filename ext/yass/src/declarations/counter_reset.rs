use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, gc, typed_data};
use style::values::generics::counters::CounterPair;
use style::values::specified::{CounterReset, Integer};

use crate::cached_value_list::CachedValueList;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::CounterReset", mark)]
pub struct YCounterReset {
    values: CachedValueList<CounterPair<Integer>>,
}

impl YCounterReset {
    pub fn new(counter_reset: CounterReset) -> Self {
        Self {
            values: CachedValueList::new(counter_reset.to_vec(), |counter, _ctx, ruby| {
                YCounterResetCounter::new(counter.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YCounterReset {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::CounterReset::Counter")]
pub struct YCounterResetCounter {
    name: String,
    value: i32,
    is_reversed: bool,
}

impl YCounterResetCounter {
    pub fn new(counter: CounterPair<Integer>) -> Self {
        Self {
            name: counter.name.0.to_string(),
            value: counter.value.value(),
            is_reversed: counter.is_reversed,
        }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.name)
    }

    pub fn value(_ruby: &Ruby, rb_self: &Self) -> i32 {
        rb_self.value
    }

    pub fn reversed(_ruby: &Ruby, rb_self: &Self) -> bool {
        rb_self.is_reversed
    }
}
