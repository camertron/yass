use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data};
use style::{shared_lock::SharedRwLock, stylesheets::{CssRule, Stylesheet}};

use crate::{cached_value_list::CachedValueList, rules::rule::make_rule};

#[derive(TypedData)]
#[magnus(class = "Yass::Stylesheet", mark)]
pub struct YSheet {
    pub stylesheet: Stylesheet,
    pub cached_rules: CachedValueList<CssRule, SharedRwLock>
}

impl YSheet {
    pub fn new(stylesheet: Stylesheet) -> Self {
        YSheet {
            stylesheet: stylesheet.clone(),
            cached_rules: CachedValueList::empty(Some(stylesheet.shared_lock), |rule, shared_lock, ruby| {
                make_rule(rule, &shared_lock.as_ref().unwrap(), ruby).into_value_with(ruby)
            })
        }
    }

    pub fn rules(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.cached_rules.is_empty() {
            let guard = rb_self.stylesheet.shared_lock.read();
            let contents = rb_self.stylesheet.contents.read_with(&guard);
            let rules = contents.rules.read_with(&guard);

            for rule in &rules.0 {
                rb_self.cached_rules.add(rule.clone(), ruby)?;
            }
        }

        rb_self.cached_rules.to_a(ruby)
    }
}

impl DataTypeFunctions for YSheet {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_rules.mark(marker);
    }
}
