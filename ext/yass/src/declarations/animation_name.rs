use magnus::{Error, RArray, Ruby, typed_data};
use style::properties::longhands::animation_name::SpecifiedValue;

#[magnus::wrap(class = "Yass::Declarations::AnimationName")]
pub struct YAnimationName {
    specified_value: SpecifiedValue
}

impl YAnimationName {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for name in rb_self.specified_value.0.as_ref() {
            result.push(ruby.str_from_slice(name.0.as_atom().as_bytes()))?;
        }

        Ok(result)
    }
}
