use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc::Marker, typed_data, value::Id};
use style::values::specified::{LengthPercentage, position::{HorizontalPositionKeyword, PositionComponent}};

use crate::{cached_value::CachedValue, declarations::{images::horizontal_keyword_to_id, size::YLengthPercentage}};

pub fn make_horizontal_position_component(component: &PositionComponent<HorizontalPositionKeyword>, ruby: &Ruby) -> Value {
    match component {
        PositionComponent::Center => {
            YCenterHorizontalPositionComponent::new().into_value_with(ruby)
        },

        PositionComponent::Length(length) => {
            YLengthHorizontalPositionComponent::new(length.clone()).into_value_with(ruby)
        },

        PositionComponent::Side(keyword, offset) => {
            YSideHorizontalPositionComponent::new(keyword.clone(), offset.clone()).into_value_with(ruby)
        },
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::CenterHorizontalPositionComponent")]
pub struct YCenterHorizontalPositionComponent {}

impl YCenterHorizontalPositionComponent {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(magnus::TypedData)]
#[magnus(class = "Yass::Declarations::Image::LengthHorizontalPositionComponent", mark)]
pub struct YLengthHorizontalPositionComponent {
    length: CachedValue<LengthPercentage>,
}

impl YLengthHorizontalPositionComponent {
    pub fn new(length: LengthPercentage) -> Self {
        Self {
            length: CachedValue::new(length, |length, ruby| {
                YLengthPercentage::new(length.clone()).into_value_with(ruby)
            })
        }
    }

    pub fn length(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.length.get(ruby)
    }
}

impl DataTypeFunctions for YLengthHorizontalPositionComponent {
    fn mark(&self, marker: &Marker) {
        self.length.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::SideHorizontalPositionComponent", mark)]
pub struct YSideHorizontalPositionComponent {
    keyword: HorizontalPositionKeyword,
    offset: CachedValue<Option<LengthPercentage>>,
}

impl YSideHorizontalPositionComponent {
    pub fn new(keyword: HorizontalPositionKeyword, offset: Option<LengthPercentage>) -> Self {
        Self {
            keyword,
            offset: CachedValue::new(offset, |offset, ruby| {
                match offset {
                    Some(offset) => {
                        YLengthPercentage::new(offset.clone()).into_value_with(ruby)
                    },

                    None => ruby.qnil().into_value_with(ruby),
                }
            })
        }
    }

    pub fn side(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        horizontal_keyword_to_id(rb_self.keyword, ruby)
    }

    pub fn offset(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.offset.get(ruby)
    }
}

impl DataTypeFunctions for YSideHorizontalPositionComponent {
    fn mark(&self, marker: &Marker) {
        self.offset.mark(marker);
    }
}
