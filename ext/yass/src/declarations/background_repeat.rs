use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data, value::Id};
use style::{properties::longhands::background_repeat::{SingleSpecifiedValue, SpecifiedValue}, values::specified::background::BackgroundRepeatKeyword};

use crate::cached_value_list::CachedValueList;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::BackgroundRepeat", mark)]
pub struct YBackgroundRepeat {
    values: CachedValueList<SingleSpecifiedValue>,
}

impl YBackgroundRepeat {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |value, _ctx, ruby| {
                YBackgroundRepeatValue::new(value.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YBackgroundRepeat {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::BackgroundRepeatValue")]
pub struct YBackgroundRepeatValue {
    background_repeat: SingleSpecifiedValue,
}

impl YBackgroundRepeatValue {
    pub fn new(background_repeat: SingleSpecifiedValue) -> Self {
        Self { background_repeat }
    }

    pub fn horizontal(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        background_repeat_keyword_to_id(&rb_self.background_repeat.0, ruby)
    }

    pub fn vertical(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        background_repeat_keyword_to_id(&rb_self.background_repeat.1, ruby)
    }
}

fn background_repeat_keyword_to_id(keyword: &BackgroundRepeatKeyword, ruby: &Ruby) -> Id {
    match keyword {
        BackgroundRepeatKeyword::Repeat => ruby.intern("repeat"),
        BackgroundRepeatKeyword::Space => ruby.intern("space"),
        BackgroundRepeatKeyword::Round => ruby.intern("round"),
        BackgroundRepeatKeyword::NoRepeat => ruby.intern("no_repeat"),
    }
}
