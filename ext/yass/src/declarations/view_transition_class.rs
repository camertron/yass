use style::values::computed::ViewTransitionClass;

#[magnus::wrap(class = "Yass::Declarations::ViewTransitionClass")]
pub struct YViewTransitionClass {
  view_transition_class: ViewTransitionClass
}

impl YViewTransitionClass {
  pub fn new(view_transition_class: ViewTransitionClass) -> Self {
    Self { view_transition_class }
  }
}
