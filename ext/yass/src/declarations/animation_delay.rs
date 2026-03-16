use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data, value::Opaque};
use style::{properties::longhands::animation_delay::SpecifiedValue};

use crate::declarations::time::YTime;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::AnimationDelay", mark)]
pub struct YAnimationDelay {
  specified_value: SpecifiedValue,
  cached_times: RefCell<Option<Vec<Opaque<typed_data::Obj<YTime>>>>>
}

impl DataTypeFunctions for YAnimationDelay {
  fn mark(&self, marker: &gc::Marker) {
    if let Some(times) = self.cached_times.borrow().as_ref() {
      marker.mark_slice(times.as_slice());
    }
  }
}

impl YAnimationDelay {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value, cached_times: RefCell::new(None) }
  }

  pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
    if rb_self.cached_times.borrow().is_none() {
      let mut new_rules: Vec<Opaque<typed_data::Obj<YTime>>> = Vec::with_capacity(rb_self.specified_value.0.len());

      for time in rb_self.specified_value.0.as_ref() {
        new_rules.push(ruby.obj_wrap(YTime::new(time.clone())).into());
      }
    }

    let times = rb_self.cached_times.borrow();
    let result = ruby.ary_new_capa(times.as_ref().unwrap().len());

    for time in times.as_ref().unwrap() {
      result.push(ruby.get_inner(*time))?;
    }

    Ok(result)
  }
}
