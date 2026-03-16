use std::cell::RefCell;
use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};
use style::{shared_lock::SharedRwLock, stylesheets::{CssRule, Stylesheet}};

use crate::{ruby_obj_list::RubyObjList, rules::{YMediaRule, YRule, YStyleRule, rule::YUnimplementedRule}};

#[derive(TypedData)]
#[magnus(class = "Yass::Stylesheet", mark)]
pub struct YSheet {
    pub stylesheet: Stylesheet,
    pub cached_rules: RefCell<Option<RubyObjList<YRule, CssRule, Box<dyn Fn(&CssRule) -> YRule + Send>>>>
}

impl DataTypeFunctions for YSheet {
    fn mark(&self, marker: &gc::Marker) {
        if let Some(cached_rules) = self.cached_rules.borrow().as_ref() {
            cached_rules.mark(marker);
        }
    }
}

impl YSheet {
    pub fn new(stylesheet: Stylesheet) -> Self {
        YSheet { stylesheet, cached_rules: RefCell::new(None) }
    }

    fn make_rule(rule: &CssRule, lock: &SharedRwLock) -> YRule {
        match rule {
            CssRule::Style(locked_rule) => {
                YRule::StyleRule(
                    YStyleRule::new(
                        locked_rule.clone(),
                        lock.clone()
                    )
                )
            }

            CssRule::Media(media_rule) => {
                YRule::MediaRule(YMediaRule { rule: media_rule.clone() })
            }

            _ => {
                YRule::UnimplementedRule(YUnimplementedRule {})
            }
        }
    }

    pub fn rules(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.cached_rules.borrow().is_none() {
            let guard = rb_self.stylesheet.shared_lock.read();
            let contents = rb_self.stylesheet.contents.read_with(&guard);
            let rules = contents.rules.read_with(&guard);
            let shared_lock = rb_self.stylesheet.shared_lock.clone();
            let transform: Box<dyn Fn(&CssRule) -> YRule + Send> = Box::new(move |rule| {
                Self::make_rule(rule, &shared_lock)
            });

            let obj_list = RubyObjList::new(
                rules.0.to_vec(),
                transform
            );

            *rb_self.cached_rules.borrow_mut() = Some(obj_list);
        }

        rb_self.cached_rules.borrow().as_ref().unwrap().to_a(ruby)
    }
}
