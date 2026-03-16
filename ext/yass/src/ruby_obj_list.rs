use std::cell::RefCell;

use magnus::{Error, RArray, Ruby, TypedData, gc, typed_data, value::Opaque};

pub struct RubyObjList<T: TypedData, E: Clone, F = fn(&E) -> T> {
  values: Vec<E>,
  transform: F,
  cached_values: RefCell<Option<Vec<Opaque<typed_data::Obj<T>>>>>
}

impl<T, E, F> RubyObjList<T, E, F> where T: TypedData, E: Clone, F: Fn(&E) -> T {
  pub fn new(values: Vec<E>, cb: F) -> Self {
    Self { values, transform: cb, cached_values: RefCell::new(None) }
  }

  pub fn to_a(&self, ruby: &Ruby) -> Result<RArray, Error> {
    if self.cached_values.borrow().is_none() {
      let mut cached_values: Vec<Opaque<typed_data::Obj<T>>> = Vec::with_capacity(self.values.len());

      for value in &self.values {
        let new_value = (self.transform)(value);
        cached_values.push(Opaque::from(ruby.obj_wrap(new_value)));
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
