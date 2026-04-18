use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::specified::{LengthPercentage, position::{PositionComponent, VerticalPositionKeyword}};

use crate::{cached_value::CachedValue, declarations::{images::vertical_keyword_to_id, size::YLengthPercentage}};

pub fn make_vertical_position_component(component: &PositionComponent<VerticalPositionKeyword>, ruby: &Ruby) -> Value {
    match component {
        PositionComponent::Center => {
            YCenterVerticalPositionComponent::new().into_value_with(ruby)
        },

        PositionComponent::Length(length) => {
            YLengthVerticalPositionComponent::new(length.clone()).into_value_with(ruby)
        },

        PositionComponent::Side(keyword, offset) => {
            YSideVerticalPositionComponent::new(keyword.clone(), offset.clone()).into_value_with(ruby)
        },
    }
}

#[magnus::wrap(class = "Yass::Declarations::Image::CenterVerticalPositionComponent")]
pub struct YCenterVerticalPositionComponent {}

impl YCenterVerticalPositionComponent {
    pub fn new() -> Self {
        Self { }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::LengthVerticalPositionComponent", mark)]
pub struct YLengthVerticalPositionComponent {
    length: CachedValue<LengthPercentage>,
}

impl YLengthVerticalPositionComponent {
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

impl DataTypeFunctions for YLengthVerticalPositionComponent {
    fn mark(&self, marker: &gc::Marker) {
        self.length.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Image::SideVerticalPositionComponent", mark)]
pub struct YSideVerticalPositionComponent {
    keyword: VerticalPositionKeyword,
    offset: CachedValue<Option<LengthPercentage>>,
}

impl YSideVerticalPositionComponent {
    pub fn new(keyword: VerticalPositionKeyword, offset: Option<LengthPercentage>) -> Self {
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

    pub fn position(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        vertical_keyword_to_id(rb_self.keyword, ruby)
    }

    pub fn offset(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.offset.get(ruby)
    }
}

impl DataTypeFunctions for YSideVerticalPositionComponent {
    fn mark(&self, marker: &gc::Marker) {
        self.offset.mark(marker);
    }
}
