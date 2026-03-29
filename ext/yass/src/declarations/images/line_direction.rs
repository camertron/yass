use magnus::{IntoValue, Ruby, Value, typed_data, value::Id};
use style::values::specified::{Angle, image::LineDirection, position::{HorizontalPositionKeyword, VerticalPositionKeyword}};

use crate::{cached_value::CachedValue, declarations::{angle::YAngle, images::{horizontal_keyword_to_id, vertical_keyword_to_id}}};

pub fn make_line_direction(direction: LineDirection, ruby: &Ruby) -> Value {
    match direction {
        LineDirection::Angle(angle) => {
            YAngleLineDirection::new(angle).into_value_with(ruby)
        },

        LineDirection::Horizontal(keyword) => {
            YHorizontalLineDirection::new(keyword).into_value_with(ruby)
        },

        LineDirection::Vertical(keyword) => {
            YVerticalLineDirection::new(keyword).into_value_with(ruby)
        },

        LineDirection::Corner(h_keyword, v_keyword) => {
            YCornerLineDirection::new(h_keyword, v_keyword).into_value_with(ruby)
        },
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::AngleLineDirection")]
pub struct YAngleLineDirection {
    angle: CachedValue<Angle>,
}

impl YAngleLineDirection {
    pub fn new(angle: Angle) -> Self {
        Self {
            angle: CachedValue::new(angle, |angle, ruby| {
                YAngle::new(angle.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn angle(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.angle.get(ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::HorizontalLineDirection")]
pub struct YHorizontalLineDirection {
    keyword: HorizontalPositionKeyword,
}

impl YHorizontalLineDirection {
    pub fn new(keyword: HorizontalPositionKeyword) -> Self {
        Self { keyword }
    }

    pub fn direction(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        horizontal_keyword_to_id(rb_self.keyword, ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::VerticalLineDirection")]
pub struct YVerticalLineDirection {
    keyword: VerticalPositionKeyword,
}

impl YVerticalLineDirection {
    pub fn new(keyword: VerticalPositionKeyword) -> Self {
        Self { keyword }
    }

    pub fn direction(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        vertical_keyword_to_id(rb_self.keyword, ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::CornerLineDirection")]
pub struct YCornerLineDirection {
    horizontal: HorizontalPositionKeyword,
    vertical: VerticalPositionKeyword,
}

impl YCornerLineDirection {
    pub fn new(horizontal: HorizontalPositionKeyword, vertical: VerticalPositionKeyword) -> Self {
        Self { horizontal, vertical }
    }

    pub fn horizontal_direction(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        horizontal_keyword_to_id(rb_self.horizontal, ruby)
    }

    pub fn vertical_direction(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        vertical_keyword_to_id(rb_self.vertical, ruby)
    }
}
