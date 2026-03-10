use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data, value::Opaque};
use style::{shared_lock::{Locked, SharedRwLock}, stylesheets::StyleRule};

use crate::selector::{YSelector};

#[derive(TypedData)]
#[magnus(class = "Yass::StyleRule", mark)]
pub struct YStyleRule {
    pub rule: style::servo_arc::Arc<Locked<StyleRule>>,
    pub lock: SharedRwLock,
    pub cached_selectors: RefCell<Option<Vec<Opaque<typed_data::Obj<YSelector>>>>>
}

impl DataTypeFunctions for YStyleRule {
    fn mark(&self, marker: &gc::Marker) {
        println!("Marking selectors");
        // let ruby = Ruby::get().unwrap();

        if let Some(selectors) = self.cached_selectors.borrow().as_ref() {
            marker.mark_slice(selectors.as_slice());

            // for selector in selectors {
            //     ruby.get_inner(*selector).mark(marker);
            // }
        }
    }
}

impl YStyleRule {
    pub fn new(rule: style::servo_arc::Arc<Locked<StyleRule>>, lock: SharedRwLock) -> Self {
        YStyleRule { rule, lock, cached_selectors: RefCell::new(None) }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_selectors.borrow().is_none() {
            let guard = self.lock.read();
            let style_rule = self.rule.read_with(&guard);
            let mut new_selectors: Vec<Opaque<typed_data::Obj<YSelector>>> = Vec::with_capacity(style_rule.selectors.len());

            for selector in style_rule.selectors.slice() {
                let yselector = YSelector::new(selector.clone());
                new_selectors.push(ruby.obj_wrap(yselector).into());
            }

            *self.cached_selectors.borrow_mut() = Some(new_selectors);
        }

        let cached_selectors = self.cached_selectors.borrow();
        let result = ruby.ary_new_capa(cached_selectors.as_ref().unwrap().len());

        for rs in cached_selectors.as_ref().unwrap() {
            result.push(ruby.get_inner(*rs))?;
        }

        Ok(result)
    }
}
