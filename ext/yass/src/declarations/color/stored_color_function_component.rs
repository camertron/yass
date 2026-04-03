use magnus::{IntoValue, Ruby, Value, value::Id};
use style::{color::{component::ColorComponent, parsing::{ChannelKeyword, NumberOrAngleComponent, NumberOrPercentageComponent}}, values::specified::{Angle, calc::CalcNode}};

use crate::declarations::{angle::YAngle, number::YNumber, percentage::YPercentage};

#[derive(Clone)]
pub enum StoredColorFunctionComponent {
    NumberOrPercentage(ColorComponent<NumberOrPercentageComponent>),
    NumberOrAngle(ColorComponent<NumberOrAngleComponent>),
}

impl StoredColorFunctionComponent {
    pub fn value(&self) -> Option<StoredColorComponentValue> {
        match self {
            Self::NumberOrPercentage(component) => match component {
                ColorComponent::Value(NumberOrPercentageComponent::Number(value)) => Some(StoredColorComponentValue::Number(*value)),
                ColorComponent::Value(NumberOrPercentageComponent::Percentage(value)) => Some(StoredColorComponentValue::Percentage(*value)),
                _ => None,
            },
            Self::NumberOrAngle(component) => match component {
                ColorComponent::Value(NumberOrAngleComponent::Number(value)) => Some(StoredColorComponentValue::Number(*value)),
                ColorComponent::Value(NumberOrAngleComponent::Angle(value)) => Some(StoredColorComponentValue::Angle(*value)),
                _ => None,
            },
        }
    }

    pub fn channel_keyword(&self) -> Option<ChannelKeyword> {
        match self {
            Self::NumberOrPercentage(component) => match component {
                ColorComponent::ChannelKeyword(channel_keyword) => Some(*channel_keyword),
                _ => None,
            },
            Self::NumberOrAngle(component) => match component {
                ColorComponent::ChannelKeyword(channel_keyword) => Some(*channel_keyword),
                _ => None,
            },
        }
    }

    pub fn calc_node(&self) -> Option<CalcNode> {
        match self {
            Self::NumberOrPercentage(component) => match component {
                ColorComponent::Calc(calc_node) => Some((**calc_node).clone()),
                _ => None,
            },
            Self::NumberOrAngle(component) => match component {
                ColorComponent::Calc(calc_node) => Some((**calc_node).clone()),
                _ => None,
            },
        }
    }
}

#[derive(Clone)]
pub enum StoredColorComponentValue {
    Number(f32),
    Percentage(f32),
    Angle(f32),
}

impl StoredColorComponentValue {
    pub fn to_value(&self, ruby: &Ruby) -> Value {
        match self {
            Self::Number(value) => YNumber::new(*value).into_value_with(ruby),
            Self::Percentage(value) => YPercentage::new(*value).into_value_with(ruby),
            Self::Angle(value) => YAngle::new(Angle::from_degrees(*value, false)).into_value_with(ruby),
        }
    }
}

fn color_component_kind_to_id<T>(component: &ColorComponent<T>, ruby: &Ruby) -> Id {
    match component {
        ColorComponent::None => ruby.intern("none"),
        ColorComponent::Value(_) => ruby.intern("value"),
        ColorComponent::ChannelKeyword(_) => ruby.intern("channel_keyword"),
        ColorComponent::Calc(_) => ruby.intern("calc"),
        ColorComponent::AlphaOmitted => ruby.intern("alpha_omitted"),
    }
}
