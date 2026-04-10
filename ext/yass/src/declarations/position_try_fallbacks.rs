use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{computed::PositionTryFallbacks, specified::position::{DashedIdentAndOrTryTactic, PositionArea, PositionAreaKeyword, PositionTryFallbacksItem, PositionTryFallbacksTryTacticKeyword}};

use crate::cached_value_list::CachedValueList;

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::PositionTryFallbacks", mark)]
pub struct YPositionTryFallbacks {
    values: CachedValueList<PositionTryFallbacksItem>,
    is_none: bool,
}

impl YPositionTryFallbacks {
    pub fn new(position_try_fallbacks: PositionTryFallbacks) -> Self {
        Self {
            values: CachedValueList::new(position_try_fallbacks.0.to_vec(), |item, _ctx, ruby| {
                make_position_try_fallbacks_item(item, ruby)
            }),

            is_none: position_try_fallbacks.is_none(),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }

    pub fn is_none(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        rb_self.is_none
    }
}

impl DataTypeFunctions for YPositionTryFallbacks {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::PositionTryFallbacks::IdentAndOrTactic")]
pub struct YPositionTryFallbacksIdentAndOrTactic {
    ident: Option<String>,
    try_tactics: Vec<PositionTryFallbacksTryTacticKeyword>,
}

impl YPositionTryFallbacksIdentAndOrTactic {
    pub fn new(dashed_ident_and_or_try_tactic: DashedIdentAndOrTryTactic) -> Self {
        Self {
            ident: if dashed_ident_and_or_try_tactic.ident.is_empty() {
                None
            } else {
                Some(dashed_ident_and_or_try_tactic.ident.0.to_string())
            },

            try_tactics: dashed_ident_and_or_try_tactic.try_tactic.iter().cloned().collect(),
        }
    }

    pub fn ident(ruby: &Ruby, rb_self: &Self) -> Option<RString> {
        rb_self.ident.as_ref().map(|ident| ruby.str_new(ident))
    }

    pub fn has_ident(_ruby: &Ruby, rb_self: &Self) -> bool {
        rb_self.ident.is_some()
    }

    pub fn try_tactics(ruby: &Ruby, rb_self: &Self) -> Result<RArray, Error> {
        let result = ruby.ary_new_capa(rb_self.try_tactics.len());

        for keyword in rb_self.try_tactics.iter() {
            result.push(try_tactic_keyword_to_sym(*keyword, ruby))?;
        }

        Ok(result)
    }

    pub fn has_try_tactics(_ruby: &Ruby, rb_self: &Self) -> bool {
        !rb_self.try_tactics.is_empty()
    }
}

#[magnus::wrap(class = "Yass::Declarations::PositionTryFallbacks::PositionArea")]
pub struct YPositionTryFallbacksPositionArea {
    position_area: PositionArea,
}

impl YPositionTryFallbacksPositionArea {
    pub fn new(position_area: PositionArea) -> Self {
        Self { position_area }
    }

    pub fn first(ruby: &Ruby, rb_self: &Self) -> Id {
        position_area_keyword_to_sym(rb_self.position_area.first, ruby)
    }

    pub fn second(ruby: &Ruby, rb_self: &Self) -> Id {
        position_area_keyword_to_sym(rb_self.position_area.second, ruby)
    }
}

fn make_position_try_fallbacks_item(position_try_fallbacks_item: &PositionTryFallbacksItem, ruby: &Ruby) -> Value {
    match position_try_fallbacks_item {
        PositionTryFallbacksItem::IdentAndOrTactic(dashed_ident_and_or_try_tactic) => {
            YPositionTryFallbacksIdentAndOrTactic::new(dashed_ident_and_or_try_tactic.clone()).into_value_with(ruby)
        }
        PositionTryFallbacksItem::PositionArea(position_area) => {
            YPositionTryFallbacksPositionArea::new(position_area.clone()).into_value_with(ruby)
        }
    }
}

fn try_tactic_keyword_to_sym(try_tactic_keyword: PositionTryFallbacksTryTacticKeyword, ruby: &Ruby) -> Id {
    match try_tactic_keyword {
        PositionTryFallbacksTryTacticKeyword::FlipBlock => ruby.intern("flip_block"),
        PositionTryFallbacksTryTacticKeyword::FlipInline => ruby.intern("flip_inline"),
        PositionTryFallbacksTryTacticKeyword::FlipStart => ruby.intern("flip_start"),
        PositionTryFallbacksTryTacticKeyword::FlipX => ruby.intern("flip_x"),
        PositionTryFallbacksTryTacticKeyword::FlipY => ruby.intern("flip_y"),
    }
}

fn position_area_keyword_to_sym(position_area_keyword: PositionAreaKeyword, ruby: &Ruby) -> Id {
    match position_area_keyword {
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
