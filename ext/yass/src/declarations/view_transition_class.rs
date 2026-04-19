use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::ViewTransitionClass;

#[magnus::wrap(class = "Yass::Declarations::ViewTransitionClass")]
pub struct YViewTransitionClass {
    view_transition_class: ViewTransitionClass,
}

impl YViewTransitionClass {
    pub fn new(view_transition_class: ViewTransitionClass) -> Self {
        Self { view_transition_class }
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.view_transition_class.value.is_none()
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.view_transition_class.value.is_none() {
            return Ok(ruby.ary_new());
        }

        let result = ruby.ary_new();

        for value in rb_self.view_transition_class.value.iter() {
            result.push(ruby.str_new(&value.0.to_string()))?;
        }

        Ok(result)
    }
}
