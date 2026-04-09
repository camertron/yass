use magnus::{Ruby, typed_data, value::Id};
use style::values::generics::length::AnchorSizeKeyword;

#[magnus::wrap(class = "Yass::Declarations::Calc::AnchorSizeKeyword")]
pub struct YAnchorSizeKeyword {
    anchor_size_keyword: AnchorSizeKeyword
}

impl YAnchorSizeKeyword {
    pub fn new(anchor_size_keyword: AnchorSizeKeyword) -> Self {
        Self { anchor_size_keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.anchor_size_keyword {
            AnchorSizeKeyword::None => unreachable!(),
            AnchorSizeKeyword::Width => ruby.intern("width"),
            AnchorSizeKeyword::Height => ruby.intern("height"),
            AnchorSizeKeyword::Block => ruby.intern("block"),
            AnchorSizeKeyword::Inline => ruby.intern("inline"),
            AnchorSizeKeyword::SelfBlock => ruby.intern("self_block"),
            AnchorSizeKeyword::SelfInline => ruby.intern("self_inline"),
        }
    }
}
