use magnus::{IntoValue, Ruby, Value, gc};

use crate::cached_value::CachedValue;

type OptionalTransform<T> = fn(&(Option<T>, fn(&T, &Ruby) -> Value), &Ruby) -> Value;

pub struct OptionalCachedValue<T> {
    cached_value: CachedValue<(Option<T>, fn(&T, &Ruby) -> Value), OptionalTransform<T>>
}

impl<T> OptionalCachedValue<T> {
    pub fn new(value: Option<T>, cb: fn(&T, &Ruby) -> Value) -> Self {
        let transform: OptionalTransform<T> = |(value, cb), ruby| {
            if let Some(v) = value {
                cb(v, ruby)
            } else {
                ruby.qnil().into_value_with(ruby)
            }
        };

        Self {
            cached_value: CachedValue::new((value, cb), transform)
        }
    }

    pub fn get(&self, ruby: &Ruby) -> Value {
        self.cached_value.get(ruby)
    }

    pub fn mark(&self, marker: &gc::Marker) {
        self.cached_value.mark(marker);
    }
}
