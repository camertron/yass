use magnus::{Error, RArray, Ruby, typed_data};
use style::{properties::longhands::animation_direction::SpecifiedValue, values::computed::AnimationDirection};

#[magnus::wrap(class = "Yass::Declarations::AnimationDirection")]
pub struct YAnimationDirection {
  specified_value: SpecifiedValue
}

impl YAnimationDirection {
  pub fn new(specified_value: SpecifiedValue) -> Self {
    Self { specified_value }
  }

  pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
    let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

    for direction in rb_self.specified_value.0.as_ref() {
      let id = match direction {
        AnimationDirection::Normal => ruby.intern("normal"),
        AnimationDirection::Reverse => ruby.intern("reverse"),
        AnimationDirection::Alternate => ruby.intern("alternate"),
        AnimationDirection::AlternateReverse => ruby.intern("alternate_reverse"),
      };

      result.push(id)?;
    }

    Ok(result)
  }
}
