use magnus::{Error, RArray, Ruby, typed_data};
use style::{properties::longhands::animation_play_state::SpecifiedValue, values::computed::AnimationPlayState};

#[magnus::wrap(class = "Yass::Declarations::AnimationPlayState")]
pub struct YAnimationPlayState {
    specified_value: SpecifiedValue
}

impl YAnimationPlayState {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self { specified_value }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.specified_value.0.len());

        for play_state in rb_self.specified_value.0.as_ref() {
            let id = match play_state {
                AnimationPlayState::Running => ruby.intern("running"),
                AnimationPlayState::Paused => ruby.intern("paused"),
            };

            result.push(id)?;
        }

        Ok(result)
    }
}
