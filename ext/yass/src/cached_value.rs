use std::cell::RefCell;

use magnus::{Ruby, Value, gc, value::Opaque};

pub struct CachedValue<T, F = fn(&T, &Ruby) -> Value> {
    pub value: T,
    transform: F,
    cached_value: RefCell<Option<Opaque<Value>>>
}

impl<T, F> CachedValue<T, F> where F: Fn(&T, &Ruby) -> Value {
    pub fn new(value: T, cb: F) -> Self {
        Self { value, transform: cb, cached_value: RefCell::new(None) }
    }

    pub fn get(&self, ruby: &Ruby) -> Value {
        if self.cached_value.borrow().is_none() {
            let new_value = (self.transform)(&self.value, ruby);
            *self.cached_value.borrow_mut() = Some(Opaque::from(new_value));
        }

        let opaque= self.cached_value.borrow().unwrap();
        ruby.get_inner(opaque)
    }

    pub fn mark(&self, marker: &gc::Marker) {
        if let Some(cached_value) = self.cached_value.borrow().as_ref() {
            marker.mark(*cached_value);
        }
    }
}
