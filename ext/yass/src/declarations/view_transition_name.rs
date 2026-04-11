use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::computed::ViewTransitionName;

#[magnus::wrap(class = "Yass::Declarations::ViewTransitionName")]
pub struct YViewTransitionName {
    view_transition_name: ViewTransitionName,
}

impl YViewTransitionName {
    pub fn new(view_transition_name: ViewTransitionName) -> Self {
        Self { view_transition_name }
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        matches!(rb_self.view_transition_name, ViewTransitionName::None)
    }

    pub fn is_match_element(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        matches!(rb_self.view_transition_name, ViewTransitionName::MatchElement)
    }

    pub fn name(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        match &rb_self.view_transition_name {
            ViewTransitionName::Ident(atom) => ruby.str_from_slice(atom.as_bytes()).into_value_with(ruby),
            _ => ruby.qnil().into_value_with(ruby),
        }
    }
}
