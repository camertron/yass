use cssparser::SourceLocation;
use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data};
use selectors::parser::Selector;
use style::{properties::PropertyDeclaration, selector_parser::SelectorImpl, servo_arc::Arc, shared_lock::{Locked, SharedRwLock}, stylesheets::StyleRule};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList, declarations::declaration::YDeclaration, general::YSourceLocation, selectors::YSelector};

#[derive(TypedData)]
#[magnus(class = "Yass::StyleRule", mark)]
pub struct YStyleRule {
    rule: Arc<Locked<StyleRule>>,
    lock: SharedRwLock,
    selectors: CachedValueList<Selector<SelectorImpl>>,
    declarations: CachedValueList<PropertyDeclaration>,
    source_location: CachedValue<SourceLocation>
}

impl YStyleRule {
    pub fn new(rule: Arc<Locked<StyleRule>>, lock: SharedRwLock) -> Self {
        YStyleRule {
            rule,
            lock,

            selectors: CachedValueList::empty(None, |selector, _ctx, ruby| {
                YSelector::new(selector.clone()).into_value_with(ruby)
            }),

            declarations: CachedValueList::empty(None, |declaration, _ctx, ruby| {
                YDeclaration::from(declaration, ruby).unwrap().into_value_with(ruby)
            }),

            source_location: CachedValue::empty(|source_location, ruby| {
                YSourceLocation::new(source_location.line, source_location.column).into_value_with(ruby)
            })
        }
    }

    pub fn selectors(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.selectors.is_empty() {
            let guard = rb_self.lock.read();
            let style_rule = rb_self.rule.read_with(&guard);

            for selector in style_rule.selectors.slice() {
                rb_self.selectors.add(selector.clone(), ruby)?;
            }
        }

        rb_self.selectors.to_a(ruby)
    }

    pub fn declarations(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.declarations.is_empty() {
            let guard = rb_self.lock.read();
            let style_rule = rb_self.rule.read_with(&guard);
            let block = style_rule.block.read_with(&guard);

            for declaration in block.declarations() {
                rb_self.declarations.add(declaration.clone(), ruby)?;
            }
        }

        rb_self.declarations.to_a(ruby)
    }

    pub fn source_location(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        if rb_self.source_location.is_empty() {
            let guard = rb_self.lock.read();
            let style_rule = rb_self.rule.read_with(&guard);
            rb_self.source_location.set(style_rule.source_location, ruby);
        }

        rb_self.source_location.get(ruby)
    }
}

impl DataTypeFunctions for YStyleRule {
    fn mark(&self, marker: &gc::Marker) {
        self.selectors.mark(marker);
        self.declarations.mark(marker);
        self.source_location.mark(marker);
    }
}
