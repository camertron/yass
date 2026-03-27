use std::cell::RefCell;

use magnus::{Error, RArray, Ruby, Value, gc, value::Opaque};

pub struct CachedValueList<T, C = (), F = fn(&T, &Option<C>, &Ruby) -> Value> {
    values: Vec<T>,
    context: Option<C>,
    transform: F,
    cached_values: RefCell<Option<Vec<Opaque<Value>>>>
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

    pub fn len(&self) -> usize {
        match self.cached_values.borrow().as_ref() {
            Some(values) => values.len(),
            None => 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&self, value: T, ruby: &Ruby) {
        if self.cached_values.borrow().is_none() {
            let cached_values: Vec<Opaque<Value>> = Vec::with_capacity(1);
            *self.cached_values.borrow_mut() = Some(cached_values);
        }

        let mut cached_values = self.cached_values.borrow_mut();
        let new_value = (self.transform)(&value, &self.context, ruby);

        cached_values.as_mut().unwrap().push(Opaque::from(new_value));
    }

    pub fn add_all(&self, values: Vec<T>, ruby: &Ruby) {
        for value in values {
            self.add(value, ruby);
        }
    }

    pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_values.borrow().is_none() {
            let mut cached_values: Vec<Opaque<Value>> = Vec::with_capacity(self.values.len());

            for value in &self.values {
                let new_value = (self.transform)(value, &self.context, ruby);
                cached_values.push(Opaque::from(new_value));
            }

            *self.cached_values.borrow_mut() = Some(cached_values);
        }

        let cached_values = self.cached_values.borrow();
        let result = ruby.ary_new_capa(cached_values.as_ref().unwrap().len());

        for value in cached_values.as_ref().unwrap() {
            result.push(ruby.get_inner(*value))?;
        }

        Ok(result)
    }

    pub fn mark(&self, marker: &gc::Marker) {
        if let Some(cached_values) = self.cached_values.borrow().as_ref() {
            marker.mark_slice(cached_values);
        }
    }
}
