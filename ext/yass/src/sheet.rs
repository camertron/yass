use std::cell::RefCell;
use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data, value::Opaque};
use style::stylesheets::{CssRule, Stylesheet};

use crate::rules::{YRule, YMediaRule, YStyleRule};

#[derive(TypedData)]
#[magnus(class = "Yass::Stylesheet", mark)]
pub struct YSheet {
    pub stylesheet: Stylesheet,
    pub cached_rules: RefCell<Option<Vec<Opaque<typed_data::Obj<YRule>>>>>
}

impl DataTypeFunctions for YSheet {
    fn mark(&self, marker: &gc::Marker) {
        println!("Marking rules");
        // let ruby = Ruby::get().unwrap();

        if let Some(rules) = self.cached_rules.borrow().as_ref() {
            marker.mark_slice(rules.as_slice());

            // for rule in rules {
            //     let obj = ruby.get_inner(*rule);
            //     obj.mark(marker);
            // }
        }
    }
}

impl YSheet {
    pub fn new(stylesheet: Stylesheet) -> Self {
        YSheet { stylesheet, cached_rules: RefCell::new(None) }
    }

    pub fn rules(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let guard = rb_self.stylesheet.shared_lock.read();
        let contents = rb_self.stylesheet.contents.read_with(&guard);
        let rules = contents.rules.read_with(&guard);

        if rb_self.cached_rules.borrow().is_none() {
            let mut new_rules: Vec<Opaque<typed_data::Obj<YRule>>> = Vec::with_capacity(rules.0.len());

            for rule in &rules.0 {
                match rule {
                    CssRule::Style(locked_rule) => {
                        let yrule = YRule::StyleRule(
                            YStyleRule::new(
                                locked_rule.clone(),
                                rb_self.stylesheet.shared_lock.clone()
                            )
                        );

                        new_rules.push(ruby.obj_wrap(yrule).into());
                    }

                    CssRule::Media(media_rule) => {
                        let yrule = YRule::MediaRule(YMediaRule { rule: media_rule.clone() });
                        new_rules.push(ruby.obj_wrap(yrule).into());
                    }

                    _ => ()
                }
            }

            *rb_self.cached_rules.borrow_mut() = Some(new_rules);
        }

        let cached_rules = rb_self.cached_rules.borrow();
        let result = ruby.ary_new_capa(cached_rules.as_ref().unwrap().len());

        for rs in cached_rules.as_ref().unwrap() {
            result.push(ruby.get_inner(*rs))?;
        }

        Ok(result)
    }
}
