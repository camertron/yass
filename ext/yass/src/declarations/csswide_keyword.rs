use magnus::{Ruby, typed_data, value::Id};
use style::properties::{CSSWideKeyword, WideKeywordDeclaration};

#[magnus::wrap(class = "Yass::Declarations::CSSWideKeyword")]
pub struct YCSSWideKeyword {
    wide_keyword_declaration: WideKeywordDeclaration
}

impl YCSSWideKeyword {
    pub fn new(wide_keyword_declaration: WideKeywordDeclaration) -> Self {
        Self { wide_keyword_declaration }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.wide_keyword_declaration.keyword {
            CSSWideKeyword::Initial => ruby.intern("initial"),
            CSSWideKeyword::Inherit => ruby.intern("inherit"),
            CSSWideKeyword::Unset => ruby.intern("unset"),
            CSSWideKeyword::Revert => ruby.intern("revert"),
            CSSWideKeyword::RevertLayer => ruby.intern("revert_layer"),
        }
    }
}
