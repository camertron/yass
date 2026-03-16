use magnus::{Error, RArray, Ruby, typed_data};
use style::{properties::longhands::animation_composition::SpecifiedValue, values::computed::AnimationComposition};

#[magnus::wrap(class = "Yass::Declarations::AnimationComposition")]
pub struct YAnimationComposition {
  specified_value: SpecifiedValue
}

impl YAnimationComposition {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }

  pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
    let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

    for value in rb_self.specified_value.0.as_ref() {
      let id = match value {
        AnimationComposition::Replace => ruby.intern("replace"),
        AnimationComposition::Add => ruby.intern("add"),
        AnimationComposition::Accumulate => ruby.intern("accumulate"),
      };

      result.push(id)?;
    }

    Ok(result)
  }
}
