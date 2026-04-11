use magnus::{Error, RArray, Ruby, typed_data};
use style::values::computed::ViewTransitionClass;
use style_traits::ToCss;

#[magnus::wrap(class = "Yass::Declarations::ViewTransitionClass")]
pub struct YViewTransitionClass {
    view_transition_class: ViewTransitionClass,
}

impl YViewTransitionClass {
    pub fn new(view_transition_class: ViewTransitionClass) -> Self {
        Self { view_transition_class }
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.view_transition_class.is_none()
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.view_transition_class.is_none() {
            return Ok(ruby.ary_new());
        }

        let css = rb_self.view_transition_class.to_css_string();
        let idents: Vec<&str> = css.split_whitespace().collect();
        let result = ruby.ary_new_capa(idents.len());

        for ident in idents {
            result.push(ruby.str_new(ident))?;
        }

        Ok(result)
    }
}
