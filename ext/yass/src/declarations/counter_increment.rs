use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, gc, typed_data};
use style::values::generics::counters::CounterPair;
use style::values::specified::{CounterIncrement, Integer};

use crate::cached_value_list::CachedValueList;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::CounterIncrement", mark)]
pub struct YCounterIncrement {
    values: CachedValueList<CounterPair<Integer>>,
}

impl YCounterIncrement {
    pub fn new(counter_increment: CounterIncrement) -> Self {
        Self {
            values: CachedValueList::new(counter_increment.to_vec(), |counter, _ctx, ruby| {
                YCounterIncrementCounter::new(counter.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YCounterIncrement {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::CounterIncrement::Counter")]
pub struct YCounterIncrementCounter {
    name: String,
    value: i32,
    is_reversed: bool,
}

impl YCounterIncrementCounter {
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
