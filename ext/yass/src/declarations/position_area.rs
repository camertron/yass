use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::PositionArea;
use style::values::specified::position::PositionAreaKeyword;

#[magnus::wrap(class = "Yass::Declarations::PositionArea")]
pub struct YPositionArea {
    position_area: PositionArea
}

impl YPositionArea {
    pub fn new(position_area: PositionArea) -> Self {
        Self { position_area }
    }

    pub fn first(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        keyword_to_sym(rb_self.position_area.first, ruby)
    }

    pub fn second(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        keyword_to_sym(rb_self.position_area.second, ruby)
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.position_area.is_none()
    }
}

fn keyword_to_sym(keyword: PositionAreaKeyword, ruby: &Ruby) -> Id {
    match keyword {
        PositionAreaKeyword::None => ruby.intern("none"),
        PositionAreaKeyword::Center => ruby.intern("center"),
        PositionAreaKeyword::SpanAll => ruby.intern("span_all"),
        PositionAreaKeyword::Start => ruby.intern("start"),
        PositionAreaKeyword::End => ruby.intern("end"),
        PositionAreaKeyword::SpanStart => ruby.intern("span_start"),
        PositionAreaKeyword::SpanEnd => ruby.intern("span_end"),
        PositionAreaKeyword::Left => ruby.intern("left"),
        PositionAreaKeyword::Right => ruby.intern("right"),
        PositionAreaKeyword::Top => ruby.intern("top"),
        PositionAreaKeyword::Bottom => ruby.intern("bottom"),
        PositionAreaKeyword::XStart => ruby.intern("x_start"),
        PositionAreaKeyword::XEnd => ruby.intern("x_end"),
        PositionAreaKeyword::YStart => ruby.intern("y_start"),
        PositionAreaKeyword::YEnd => ruby.intern("y_end"),
        PositionAreaKeyword::BlockStart => ruby.intern("block_start"),
        PositionAreaKeyword::BlockEnd => ruby.intern("block_end"),
        PositionAreaKeyword::InlineStart => ruby.intern("inline_start"),
        PositionAreaKeyword::InlineEnd => ruby.intern("inline_end"),
        PositionAreaKeyword::SpanLeft => ruby.intern("span_left"),
        PositionAreaKeyword::SpanRight => ruby.intern("span_right"),
        PositionAreaKeyword::SpanTop => ruby.intern("span_top"),
        PositionAreaKeyword::SpanBottom => ruby.intern("span_bottom"),
        PositionAreaKeyword::SpanXStart => ruby.intern("span_x_start"),
        PositionAreaKeyword::SpanXEnd => ruby.intern("span_x_end"),
        PositionAreaKeyword::SpanYStart => ruby.intern("span_y_start"),
        PositionAreaKeyword::SpanYEnd => ruby.intern("span_y_end"),
        PositionAreaKeyword::SpanBlockStart => ruby.intern("span_block_start"),
        PositionAreaKeyword::SpanBlockEnd => ruby.intern("span_block_end"),
        PositionAreaKeyword::SpanInlineStart => ruby.intern("span_inline_start"),
        PositionAreaKeyword::SpanInlineEnd => ruby.intern("span_inline_end"),
        PositionAreaKeyword::SelfStart => ruby.intern("self_start"),
        PositionAreaKeyword::SelfEnd => ruby.intern("self_end"),
        PositionAreaKeyword::SpanSelfStart => ruby.intern("span_self_start"),
        PositionAreaKeyword::SpanSelfEnd => ruby.intern("span_self_end"),
        PositionAreaKeyword::SelfXStart => ruby.intern("self_x_start"),
        PositionAreaKeyword::SelfXEnd => ruby.intern("self_x_end"),
        PositionAreaKeyword::SelfYStart => ruby.intern("self_y_start"),
        PositionAreaKeyword::SelfYEnd => ruby.intern("self_y_end"),
        PositionAreaKeyword::SelfBlockStart => ruby.intern("self_block_start"),
        PositionAreaKeyword::SelfBlockEnd => ruby.intern("self_block_end"),
        PositionAreaKeyword::SelfInlineStart => ruby.intern("self_inline_start"),
        PositionAreaKeyword::SelfInlineEnd => ruby.intern("self_inline_end"),
        PositionAreaKeyword::SpanSelfXStart => ruby.intern("span_self_x_start"),
        PositionAreaKeyword::SpanSelfXEnd => ruby.intern("span_self_x_end"),
        PositionAreaKeyword::SpanSelfYStart => ruby.intern("span_self_y_start"),
        PositionAreaKeyword::SpanSelfYEnd => ruby.intern("span_self_y_end"),
        PositionAreaKeyword::SpanSelfBlockStart => ruby.intern("span_self_block_start"),
        PositionAreaKeyword::SpanSelfBlockEnd => ruby.intern("span_self_block_end"),
        PositionAreaKeyword::SpanSelfInlineStart => ruby.intern("span_self_inline_start"),
        PositionAreaKeyword::SpanSelfInlineEnd => ruby.intern("span_self_inline_end"),
    }
}
