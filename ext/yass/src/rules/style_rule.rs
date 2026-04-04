use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc};
use selectors::parser::Selector;
use style::{properties::PropertyDeclaration, selector_parser::SelectorImpl, shared_lock::{Locked, SharedRwLock}, stylesheets::StyleRule};

use crate::{cached_value_list::CachedValueList, declarations::declaration::YDeclaration, selectors::YSelector};

#[derive(TypedData)]
#[magnus(class = "Yass::StyleRule", mark)]
pub struct YStyleRule {
    pub rule: style::servo_arc::Arc<Locked<StyleRule>>,
    pub lock: SharedRwLock,
    // pub cached_selectors: RefCell<Option<Vec<Opaque<typed_data::Obj<YSelector>>>>>,
    // pub cached_declarations: RefCell<Option<Vec<Opaque<Value>>>>
    pub cached_selectors: CachedValueList<Selector<SelectorImpl>>,
    pub cached_declarations: CachedValueList<PropertyDeclaration>
}

impl DataTypeFunctions for YStyleRule {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_selectors.mark(marker);
        self.cached_declarations.mark(marker);
    }
}

impl YStyleRule {
    pub fn new(rule: style::servo_arc::Arc<Locked<StyleRule>>, lock: SharedRwLock) -> Self {
        YStyleRule {
            rule,
            lock,

            cached_selectors: CachedValueList::empty(None, |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            }),

            cached_declarations: CachedValueList::empty(None, |declaration, _ctx, ruby| {
                YDeclaration::from(declaration, ruby).unwrap().into_value_with(ruby)
            })
        }
    }

    pub fn selectors(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_selectors.is_empty() {
            let guard = self.lock.read();
            let style_rule = self.rule.read_with(&guard);

            for selector in style_rule.selectors.slice() {
                self.cached_selectors.add(selector.clone(), ruby)?;
            }
        }

        self.cached_selectors.to_a(ruby)
    }

    pub fn declarations(&self, ruby: &Ruby) -> Result<RArray, Error> {
        if self.cached_declarations.is_empty() {
            let guard = self.lock.read();
            let style_rule = self.rule.read_with(&guard);
            let block = style_rule.block.read_with(&guard);

            for declaration in block.declarations() {
                self.cached_declarations.add(declaration.clone(), ruby)?;
            }
        }

        self.cached_declarations.to_a(ruby)
    }
}
