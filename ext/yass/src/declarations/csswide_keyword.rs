use magnus::{Ruby, typed_data, value::Id};
use style::properties::{CSSWideKeyword, WideKeywordDeclaration};

pub fn css_wide_keyword_to_id(ruby: &Ruby, keyword: CSSWideKeyword) -> Id {
    match keyword {
        CSSWideKeyword::Initial => ruby.intern("initial"),
        CSSWideKeyword::Inherit => ruby.intern("inherit"),
        CSSWideKeyword::Unset => ruby.intern("unset"),
        CSSWideKeyword::Revert => ruby.intern("revert"),
        CSSWideKeyword::RevertLayer => ruby.intern("revert_layer"),
        CSSWideKeyword::RevertRule => ruby.intern("revert_rule"),
    }
}

#[magnus::wrap(class = "Yass::Declarations::CSSWideKeyword")]
pub struct YCSSWideKeyword {
    wide_keyword_declaration: WideKeywordDeclaration
}

impl YCSSWideKeyword {
    pub fn new(wide_keyword_declaration: WideKeywordDeclaration) -> Self {
        Self { wide_keyword_declaration }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        css_wide_keyword_to_id(ruby, rb_self.wide_keyword_declaration.keyword)
    }
}
