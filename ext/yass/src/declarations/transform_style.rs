use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::TransformStyle;

#[magnus::wrap(class = "Yass::Declarations::TransformStyle")]
pub struct YTransformStyle {
    transform_style: TransformStyle
}

impl YTransformStyle {
    pub fn new(transform_style: TransformStyle) -> Self {
        Self { transform_style }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.transform_style {
            TransformStyle::Flat => ruby.intern("flat"),
            TransformStyle::Preserve3d => ruby.intern("preserve_3d"),
        }
    }
}
