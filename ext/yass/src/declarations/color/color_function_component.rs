use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{
    color::{parsing::{ChannelKeyword}},
    values::specified::{calc::CalcNode},
};

use crate::{
    cached_value::CachedValue,
    declarations::{calc::YCalc, channel_keyword::YChannelKeyword, color::stored_color_function_component::{StoredColorComponentValue, StoredColorFunctionComponent}},
};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Color::ColorFunctionComponent", mark)]
pub struct YColorFunctionComponent {
    component: StoredColorFunctionComponent,
    value: CachedValue<Option<StoredColorComponentValue>>,
    channel_keyword: CachedValue<Option<ChannelKeyword>>,
    calc: CachedValue<Option<CalcNode>>,
}

impl YColorFunctionComponent {
    pub fn new(component: StoredColorFunctionComponent) -> Self {
        Self {
            value: CachedValue::new(component.value(), |value, ruby| match value {
                Some(value) => value.to_value(ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            channel_keyword: CachedValue::new(component.channel_keyword(), |channel_keyword, ruby| {
                match channel_keyword {
                    Some(channel_keyword) => YChannelKeyword::new(*channel_keyword).into_value_with(ruby),
                    None => ruby.qnil().into_value_with(ruby),
                }
            }),

            calc: CachedValue::new(component.calc_node(), |calc_node, ruby| match calc_node {
                Some(calc_node) => YCalc::make_node(calc_node, ruby),
                None => ruby.qnil().into_value_with(ruby),
            }),

            component,
        }
    }

    pub fn kind(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        rb_self.component.kind(ruby)
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }

    pub fn channel_keyword(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.channel_keyword.get(ruby)
    }

    pub fn calc(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.calc.get(ruby)
    }
}

impl DataTypeFunctions for YColorFunctionComponent {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
        self.channel_keyword.mark(marker);
        self.calc.mark(marker);
    }
}
