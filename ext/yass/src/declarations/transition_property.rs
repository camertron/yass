use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data};
use style::{
    properties::longhands::transition_property::SpecifiedValue,
    values::specified::TransitionProperty,
};

use crate::cached_value_list::CachedValueList;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::TransitionProperty", mark)]
pub struct YTransitionProperty {
    values: CachedValueList<TransitionProperty>,
}

impl YTransitionProperty {
    pub fn new(specified_value: SpecifiedValue) -> Self {
        Self {
            values: CachedValueList::new(specified_value.0.to_vec(), |transition_property, _ctx, ruby| {
                make_transition_property(transition_property, ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YTransitionProperty {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::TransitionProperty::NonCustom")]
pub struct YTransitionPropertyNonCustom {
    name: String,
    is_all: bool,
}

impl YTransitionPropertyNonCustom {
    pub fn new(transition_property: &TransitionProperty) -> Self {
        let TransitionProperty::NonCustom(id) = transition_property else {
            unreachable!()
        };

        Self {
            name: id.name().to_owned(),
            is_all: transition_property.is_all(),
        }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.name)
    }

    pub fn is_all(rb_self: &Self) -> bool {
        rb_self.is_all
    }
}

#[magnus::wrap(class = "Yass::Declarations::TransitionProperty::Custom")]
pub struct YTransitionPropertyCustom {
    name: String,
}

impl YTransitionPropertyCustom {
    pub fn new(transition_property: &TransitionProperty) -> Self {
        let TransitionProperty::Custom(name) = transition_property else {
            unreachable!()
        };

        Self {
            name: format!("--{}", name),
        }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.name)
    }
}

#[magnus::wrap(class = "Yass::Declarations::TransitionProperty::Unsupported")]
pub struct YTransitionPropertyUnsupported {
    name: String,
    is_none: bool,
}

impl YTransitionPropertyUnsupported {
    pub fn new(transition_property: &TransitionProperty) -> Self {
        let TransitionProperty::Unsupported(custom_ident) = transition_property else {
            unreachable!()
        };

        Self {
            name: custom_ident.0.to_string(),
            is_none: transition_property.is_none(),
        }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.name)
    }

    pub fn is_none(rb_self: &Self) -> bool {
        rb_self.is_none
    }
}

fn make_transition_property(transition_property: &TransitionProperty, ruby: &Ruby) -> Value {
    match transition_property {
        TransitionProperty::NonCustom(_) => {
            YTransitionPropertyNonCustom::new(transition_property).into_value_with(ruby)
        }
        TransitionProperty::Custom(_) => {
            YTransitionPropertyCustom::new(transition_property).into_value_with(ruby)
        }
        TransitionProperty::Unsupported(_) => {
            YTransitionPropertyUnsupported::new(transition_property).into_value_with(ruby)
        }
    }
}
