use std::cell::RefCell;

use magnus::{Error, RArray, Ruby, Value, gc, value::Opaque};

pub struct CachedValueList<T, C = (), F = fn(&T, &Option<C>, &Ruby) -> Value> {
    values: Vec<T>,
    context: Option<C>,
    transform: F,
    cached_values: RefCell<Option<Opaque<RArray>>>
}

impl<T, C, F> CachedValueList<T, C, F> where F: Fn(&T, &Option<C>, &Ruby) -> Value {
    pub fn new(values: Vec<T>, cb: F) -> Self {
        Self { values, context: None, transform: cb, cached_values: RefCell::new(None) }
    }

    pub fn empty(context: Option<C>, cb: F) -> Self {
        Self {
            values: vec![],
            context,
            transform: cb,
            cached_values: RefCell::new(None)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cached_values.borrow().is_none()
    }

    pub fn add(&self, value: T, ruby: &Ruby) -> Result<(), Error> {
        if self.cached_values.borrow().is_none() {
            let cached_values = ruby.ary_new_capa(1);
            *self.cached_values.borrow_mut() = Some(Opaque::from(cached_values));
        }

        let new_value = (self.transform)(&value, &self.context, ruby);
        let cached_values = ruby.get_inner(*self.cached_values.borrow().as_ref().unwrap());
        cached_values.push(new_value)?;

        Ok(())
    }

    pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_values.borrow().is_none() {
            let cached_values = ruby.ary_new_capa(self.values.len());

            for value in &self.values {
                let new_value = (self.transform)(value, &self.context, ruby);
                cached_values.push(new_value)?;
            }

            *self.cached_values.borrow_mut() = Some(Opaque::from(cached_values));
        }

        Ok(ruby.get_inner(*self.cached_values.borrow().as_ref().unwrap()).dup())
    }

    pub fn mark(&self, marker: &gc::Marker) {
        if let Some(cached_values) = self.cached_values.borrow().as_ref() {
            marker.mark(*cached_values);
        }
    }
}
