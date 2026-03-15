use style::values::computed::ViewTransitionName;

#[magnus::wrap(class = "Yass::Declarations::ViewTransitionName")]
pub struct YViewTransitionName {
  view_transition_name: ViewTransitionName
}

impl YViewTransitionName {
  pub fn new(view_transition_name: ViewTransitionName) -> Self {
    Self { view_transition_name }
  }
}
