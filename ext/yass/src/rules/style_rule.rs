use std::cell::RefCell;

use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, Value, gc, typed_data, value::{Opaque, ReprValue}};
use style::{shared_lock::{Locked, SharedRwLock}, stylesheets::StyleRule};

use crate::{declarations::YDeclaration, selectors::YSelector};

#[derive(TypedData)]
#[magnus(class = "Yass::StyleRule", mark)]
pub struct YStyleRule {
    pub rule: style::servo_arc::Arc<Locked<StyleRule>>,
    pub lock: SharedRwLock,
    pub cached_selectors: RefCell<Option<Vec<Opaque<typed_data::Obj<YSelector>>>>>,
    pub cached_declarations: RefCell<Option<Vec<Opaque<Value>>>>
}

impl DataTypeFunctions for YStyleRule {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(selectors) = self.cached_selectors.borrow().as_ref() {
            marker.mark_slice(selectors.as_slice());
        }

        if let Some(declarations) = self.cached_declarations.borrow().as_ref() {
            marker.mark_slice(declarations.as_slice());
        }
    }
}

impl YStyleRule {
    pub fn new(rule: style::servo_arc::Arc<Locked<StyleRule>>, lock: SharedRwLock) -> Self {
        YStyleRule {
            rule,
            lock,
            cached_selectors: RefCell::new(None),
            cached_declarations: RefCell::new(None)
        }
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

    pub fn declarations(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_declarations.borrow().is_none() {
            let guard = self.lock.read();
            let style_rule = self.rule.read_with(&guard);
            let block = style_rule.block.read_with(&guard);
            let mut new_declarations: Vec<Opaque<Value>> = Vec::with_capacity(block.declarations().len());

            for declaration in block.declarations() {
                let ydeclaration = YDeclaration::from(declaration, ruby)?;
                new_declarations.push(Opaque::from(ydeclaration));
            }
        }

        let cached_declarations = self.cached_declarations.borrow();
        let result = ruby.ary_new_capa(cached_declarations.as_ref().unwrap().len());

        for rs in cached_declarations.as_ref().unwrap() {
            result.push(ruby.get_inner(*rs))?;
        }

        Ok(result)
    }
}
