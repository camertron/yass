use magnus::{IntoValue, Ruby, Value};
use style::values::specified::calc::Leaf;

use crate::declarations::{angle::YAngle, channel_keyword::YChannelKeyword, length::no_calc_length_to_value, number::YNumber, percentage::YPercentage, resolution::YResolution, time::YTime};

pub fn make_leaf(leaf: &Leaf, ruby: &Ruby) -> Value {
    match leaf {
        Leaf::Length(no_calc_length) => {
            no_calc_length_to_value(&no_calc_length, ruby)
        },

        Leaf::Angle(angle) => {
            YAngle::new(angle.clone()).into_value_with(ruby)
        },

        Leaf::Time(time) => {
            YTime::new(time.clone()).into_value_with(ruby)
        },

        Leaf::Resolution(resolution) => {
            YResolution::new(resolution.clone()).into_value_with(ruby)
        },

        Leaf::ColorComponent(channel_keyword) => {
            YChannelKeyword::new(channel_keyword.clone()).into_value_with(ruby)
        },

        Leaf::Percentage(percentage) => {
            YPercentage::new(percentage.clone()).into_value_with(ruby)
        },

        Leaf::Number(number) => {
            YNumber::new(number.clone()).into_value_with(ruby)
        },
    }
}
