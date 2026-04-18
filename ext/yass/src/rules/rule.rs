use magnus::{IntoValue, Ruby, Value};
use style::{shared_lock::SharedRwLock, stylesheets::CssRule};

use crate::rules::{font_face_rule::YFontFaceRule, media_rule::YMediaRule, style_rule::YStyleRule};

pub fn make_rule(rule: &CssRule, lock: &SharedRwLock, ruby: &Ruby) -> Value {
    match rule {
        CssRule::Style(locked_rule) => {
            YStyleRule::new(locked_rule.clone(), lock.clone()).into_value_with(ruby)
        }

        CssRule::Media(media_rule) => {
            YMediaRule::new(media_rule.clone(), lock.clone()).into_value_with(ruby)
        }

        CssRule::FontFace(font_face_rule) => {
            YFontFaceRule::new(font_face_rule.clone(), lock.clone()).into_value_with(ruby)
        }

        _ => {
            YUnimplementedRule::new().into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::UnimplementedRule")]
pub struct YUnimplementedRule { }

impl YUnimplementedRule {
    pub fn new() -> Self {
        Self {}
    }
}
