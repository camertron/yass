use magnus::{Error, RArray, Ruby, typed_data};
use style::{properties::longhands::transition_behavior::SpecifiedValue, values::specified::TransitionBehavior};

#[magnus::wrap(class = "Yass::Declarations::TransitionBehavior")]
pub struct YTransitionBehavior {
    specified_value: SpecifiedValue
}

impl YTransitionBehavior {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for behavior in rb_self.specified_value.0.as_ref() {
            let id = match behavior {
                TransitionBehavior::Normal => ruby.intern("normal"),
                TransitionBehavior::AllowDiscrete => ruby.intern("allow_discrete"),
            };

            result.push(id)?;
        }

        Ok(result)
    }
}
