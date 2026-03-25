use magnus::{Error, RArray, Ruby, typed_data};
use style::{properties::longhands::animation_fill_mode::SpecifiedValue, values::computed::AnimationFillMode};

#[magnus::wrap(class = "Yass::Declarations::AnimationFillMode")]
pub struct YAnimationFillMode {
    specified_value: SpecifiedValue
}

impl YAnimationFillMode {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for fill_mode in rb_self.specified_value.0.as_ref() {
            let id = match fill_mode {
                AnimationFillMode::None => ruby.intern("none"),
                AnimationFillMode::Forwards => ruby.intern("forwards"),
                AnimationFillMode::Backwards => ruby.intern("backwards"),
                AnimationFillMode::Both => ruby.intern("both"),
            };

            result.push(id)?
        }

        Ok(result)
    }
}
