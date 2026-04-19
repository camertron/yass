use magnus::{RString, Ruby, typed_data};
use style::values::computed::ViewTransitionName;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::ViewTransitionName")]
pub struct YViewTransitionName {
    view_transition_name: ViewTransitionName,
}

impl YViewTransitionName {
    pub fn new(view_transition_name: ViewTransitionName) -> Self {
        Self { view_transition_name }
    }

    pub fn name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> RString {
        ruby.str_new(&rb_self.view_transition_name.value.to_css_string())
    }
}
