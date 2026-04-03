use magnus::{DataTypeFunctions, Error, RArray, Ruby, TypedData, gc, typed_data};

use crate::rules::{YMediaRule, YStyleRule};

#[derive(TypedData)]
#[magnus(class = "Yass::Rule", mark)]
pub enum YRule {
    #[magnus(class = "Yass::StyleRule")]
    StyleRule(YStyleRule),

    #[magnus(class = "Yass::MediaRule")]
    MediaRule(YMediaRule),

    #[magnus(class = "Yass::UnimplementedRule")]
    UnimplementedRule(YUnimplementedRule)
}

impl DataTypeFunctions for YRule {
    fn mark(&self, marker: &gc::Marker) {
        match self {
            YRule::StyleRule(style_rule) => style_rule.mark(marker),
            _ => ()
        }
    }
}

impl YRule {
    pub fn selectors(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        match &*rb_self {
            YRule::StyleRule(rule) => rule.selectors(ruby),
            _ => Err(Error::new(ruby.exception_no_method_error(), "unreachable"))
        }
    }

    pub fn declarations(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        match &*rb_self {
            YRule::StyleRule(rule) => rule.declarations(ruby),
            _ => Err(Error::new(ruby.exception_no_method_error(), "unreachable"))
        }
    }
}

pub struct YUnimplementedRule { }
