use magnus::{Error, RArray, Ruby, typed_data};

use crate::rules::{YMediaRule, YStyleRule};

#[magnus::wrap(class = "Yass::Rule")]
pub enum YRule {
    #[magnus(class = "Yass::StyleRule")]
    StyleRule(YStyleRule),

    #[magnus(class = "Yass::MediaRule")]
    MediaRule(YMediaRule)
}

impl YRule {
    pub fn selectors(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        match &*rb_self {
            YRule::StyleRule(rule) => rule.selectors(ruby),
            _ => Err(Error::new(ruby.exception_no_method_error(), "unreachable"))
        }
    }
}