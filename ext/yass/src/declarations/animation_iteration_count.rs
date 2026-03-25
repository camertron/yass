use magnus::{Error, Module, RArray, Ruby, typed_data};
use style::{properties::longhands::animation_iteration_count::SpecifiedValue, values::specified::AnimationIterationCount};

#[magnus::wrap(class = "Yass::Declarations::AnimationIterationCount")]
pub struct YAnimationIterationCount {
    specified_value: SpecifiedValue
}

impl YAnimationIterationCount {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for count in rb_self.specified_value.0.as_ref() {
            let float = match count {
                AnimationIterationCount::Number(num) => {
                    ruby.float_from_f64(num.0.get() as f64)
                },

                AnimationIterationCount::Infinite => {
                    ruby.class_float().const_get("INFINITY")?
                }
            };

            result.push(float)?;
        }

        Ok(result)
    }
}
