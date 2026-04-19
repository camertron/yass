use std::cell::RefCell;

use magnus::{IntoValue, Ruby, Value, gc, value::Opaque};

pub struct CachedValue<T, F = fn(&T, &Ruby) -> Value> {
    pub value: Option<T>,
    transform: F,
    cached_value: RefCell<Option<Opaque<Value>>>
}

impl<T, F> CachedValue<T, F> where F: Fn(&T, &Ruby) -> Value {
    pub fn new(value: T, cb: F) -> Self {
        Self { value: Some(value), transform: cb, cached_value: RefCell::new(None) }
    }

    pub fn empty(cb: F) -> Self {
        Self { value: None, transform: cb, cached_value: RefCell::new(None) }
    }

    pub fn is_empty(&self) -> bool {
        self.cached_value.borrow().is_none()
    }

    pub fn get(&self, ruby: &Ruby) -> Value {
        if self.cached_value.borrow().is_none() {
            if self.value.is_none() {
                return ruby.qnil().into_value_with(ruby)
            }

            let new_value = (self.transform)(&self.value.as_ref().unwrap(), ruby);
            *self.cached_value.borrow_mut() = Some(Opaque::from(new_value));
        }

        let opaque= self.cached_value.borrow().unwrap();
        ruby.get_inner(opaque)
    }

    pub fn set(&self, value: T, ruby: &Ruby) {
        let new_value = (self.transform)(&value, ruby);
        *self.cached_value.borrow_mut() = Some(Opaque::from(new_value));
    }

    pub fn mark(&self, marker: &gc::Marker) {
        if let Some(cached_value) = self.cached_value.borrow().as_ref() {
            marker.mark(*cached_value);
        }
    }
}
