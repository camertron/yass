use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::specified::BorderSideWidth;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::BorderRightWidth")]
pub struct YBorderRightWidth {
    border_side_width: BorderSideWidth
}

impl YBorderRightWidth {
    pub fn new(border_side_width: BorderSideWidth) -> Self {
        Self { border_side_width }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        // BorderSideWidth wraps LineWidth with a private field, so we have to use
        // to_css_string instead.
        let css = rb_self.border_side_width.to_css_string();
        match css.as_str() {
            keyword @ ("thin" | "medium" | "thick") => ruby.intern(keyword).into_value_with(ruby),
            _ => ruby.str_new(&css).into_value_with(ruby),
        }
    }
}
