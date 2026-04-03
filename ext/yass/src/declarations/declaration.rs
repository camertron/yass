use magnus::{Error, Ruby, Value, value::ReprValue};
use style::properties::PropertyDeclaration;

use crate::declarations::{align_content::YAlignContent, align_items::YAlignItems, align_self::YAlignSelf, alignment_baseline::YAlignmentBaseline, animation_composition::YAnimationComposition, animation_delay::YAnimationDelay, animation_direction::YAnimationDirection, animation_duration::YAnimationDuration, animation_fill_mode::YAnimationFillMode, animation_iteration_count::YAnimationIterationCount, animation_name::YAnimationName, animation_play_state::YAnimationPlayState, animation_range_end::YAnimationRangeEnd, animation_range_start::YAnimationRangeStart, animation_timeline::YAnimationTimeline, animation_timing_function::YAnimationTimingFunction, aspect_ratio::YAspectRatio, backdrop_filter::YBackdropFilter, backface_visibility::YBackfaceVisibility, background_attachment::YBackgroundAttachment, background_blend_mode::YBackgroundBlendMode, background_clip::YBackgroundClip, background_color::YBackgroundColor, background_image::YBackgroundImage, background_origin::YBackgroundOrigin, background_position_x::YBackgroundPositionX, background_position_y::YBackgroundPositionY, background_repeat::YBackgroundRepeat, background_size::YBackgroundSize, baseline_shift::make_baseline_shift, baseline_source::YBaselineSource, block_size::YBlockSize, border_block_end_color::YBorderBlockEndColor, border_block_end_style::YBorderBlockEndStyle, border_block_end_width::YBorderBlockEndWidth, border_block_start_color::YBorderBlockStartColor, border_block_start_style::YBorderBlockStartStyle, border_block_start_width::YBorderBlockStartWidth, border_bottom_color::YBorderBottomColor, border_bottom_left_radius::YBorderBottomLeftRadius, border_bottom_right_radius::YBorderBottomRightRadius, border_bottom_style::YBorderBottomStyle, border_bottom_width::YBorderBottomWidth, border_collapse::YBorderCollapse, border_end_end_radius::YBorderEndEndRadius, border_end_start_radius::YBorderEndStartRadius, border_image_outset::YBorderImageOutset, border_image_repeat::YBorderImageRepeat, border_image_slice::YBorderImageSlice, border_image_source::YBorderImageSource, border_image_width::YBorderImageWidth, border_inline_end_color::YBorderInlineEndColor, border_inline_end_style::YBorderInlineEndStyle, border_inline_end_width::YBorderInlineEndWidth, border_inline_start_color::YBorderInlineStartColor, border_inline_start_style::YBorderInlineStartStyle, border_inline_start_width::YBorderInlineStartWidth, border_left_color::YBorderLeftColor, border_left_style::YBorderLeftStyle, border_left_width::YBorderLeftWidth, border_right_color::YBorderRightColor, border_right_style::YBorderRightStyle, border_right_width::YBorderRightWidth, border_spacing::YBorderSpacing, border_start_end_radius::YBorderStartEndRadius, border_start_start_radius::YBorderStartStartRadius, border_top_color::YBorderTopColor, border_top_left_radius::YBorderTopLeftRadius, border_top_right_radius::YBorderTopRightRadius, border_top_style::YBorderTopStyle, border_top_width::YBorderTopWidth, bottom::YBottom, box_shadow::YBoxShadow, box_sizing::YBoxSizing, caption_side::YCaptionSide, caret_color::YCaretColor, clear::YClear, clip::YClip, clip_path::YClipPath, color::YColor, color_scheme::YColorScheme, column_count::make_column_count, column_gap::YColumnGap, column_span::YColumnSpan, column_width::YColumnWidth, contain::YContain, container_name::YContainerName, container_type::YContainerType, content::YContent, counter_increment::YCounterIncrement, counter_reset::YCounterReset, csswide_keyword::YCSSWideKeyword, cursor::YCursor, custom::YCustom, direction::YDirection, display::YDisplay, empty_cells::YEmptyCells, filter::YFilter, flex_basis::YFlexBasis, flex_direction::YFlexDirection, flex_grow::YFlexGrow, flex_shrink::YFlexShrink, flex_wrap::YFlexWrap, float::YFloat, font_family::YFontFamily, font_language_override::YFontLanguageOverride, font_optical_sizing::YFontOpticalSizing, font_size::YFontSize, font_stretch::YFontStretch, font_style::YFontStyle, font_synthesis_weight::YFontSynthesisWeight, font_variant_caps::YFontVariantCaps, font_variation_settings::YFontVariationSettings, font_weight::YFontWeight, grid_auto_columns::YGridAutoColumns, grid_auto_flow::YGridAutoFlow, grid_auto_rows::YGridAutoRows, grid_column_end::YGridColumnEnd, grid_column_start::YGridColumnStart, grid_row_end::YGridRowEnd, grid_row_start::YGridRowStart, grid_template_areas::YGridTemplateAreas, grid_template_columns::YGridTemplateColumns, grid_template_rows::YGridTemplateRows, height::YHeight, image_rendering::YImageRendering, inline_size::YInlineSize, inset_block_end::YInsetBlockEnd, inset_block_start::YInsetBlockStart, inset_inline_end::YInsetInlineEnd, inset_inline_start::YInsetInlineStart, isolation::YIsolation, justify_content::YJustifyContent, justify_items::YJustifyItems, justify_self::YJustifySelf, left::YLeft, letter_spacing::YLetterSpacing, line_break::YLineBreak, line_height::YLineHeight, list_style_image::YListStyleImage, list_style_position::YListStylePosition, list_style_type::YListStyleType, margin_block_end::YMarginBlockEnd, margin_block_start::YMarginBlockStart, margin_bottom::YMarginBottom, margin_inline_end::YMarginInlineEnd, margin_inline_start::YMarginInlineStart, margin_left::YMarginLeft, margin_right::YMarginRight, margin_top::YMarginTop, mask_image::YMaskImage, max_block_size::YMaxBlockSize, max_height::YMaxHeight, max_inline_size::YMaxInlineSize, max_width::YMaxWidth, min_block_size::YMinBlockSize, min_height::YMinHeight, min_inline_size::YMinInlineSize, min_width::YMinWidth, mix_blend_mode::YMixBlendMode, object_fit::YObjectFit, object_position::YObjectPosition, offset_path::YOffsetPath, opacity::YOpacity, order::YOrder, outline_color::YOutlineColor, outline_offset::YOutlineOffset, outline_style::YOutlineStyle, outline_width::YOutlineWidth, overflow_block::YOverflowBlock, overflow_clip_margin::YOverflowClipMargin, overflow_inline::YOverflowInline, overflow_wrap::YOverflowWrap, overflow_x::YOverflowX, overflow_y::YOverflowY, padding_block_end::YPaddingBlockEnd, padding_block_start::YPaddingBlockStart, padding_bottom::YPaddingBottom, padding_inline_end::YPaddingInlineEnd, padding_inline_start::YPaddingInlineStart, padding_left::YPaddingLeft, padding_right::YPaddingRight, padding_top::YPaddingTop, perspective::YPerspective, perspective_origin::YPerspectiveOrigin, pointer_events::YPointerEvents, position::YPosition, position_area::YPositionArea, position_try_fallbacks::YPositionTryFallbacks, quotes::YQuotes, right::YRight, rotate::YRotate, row_gap::YRowGap, scale::YScale, servo_overflow_clip_box::YServoOverflowClipBox, servo_top_layer::YServoTopLayer, table_layout::YTableLayout, text_align::YTextAlign, text_align_last::YTextAlignLast, text_decoration_color::YTextDecorationColor, text_decoration_line::YTextDecorationLine, text_decoration_style::YTextDecorationStyle, text_indent::YTextIndent, text_justify::YTextJustify, text_overflow::YTextOverflow, text_rendering::YTextRendering, text_shadow::YTextShadow, text_transform::YTextTransform, text_wrap_mode::YTextWrapMode, top::YTop, transform::YTransform, transform_origin::YTransformOrigin, transform_style::YTransformStyle, transition_behavior::YTransitionBehavior, transition_delay::YTransitionDelay, transition_duration::YTransitionDuration, transition_property::YTransitionProperty, transition_timing_function::YTransitionTimingFunction, translate::YTranslate, unicode_bidi::YUnicodeBidi, view_transition_class::YViewTransitionClass, view_transition_name::YViewTransitionName, visibility::YVisibility, webkit_text_security::YWebkitTextSecurity, white_space_collapse::YWhiteSpaceCollapse, width::YWidth, will_change::YWillChange, with_variables::YWithVariables, word_break::YWordBreak, word_spacing::YWordSpacing, writing_mode::YWritingMode, xlang::YXLang, zindex::YZIndex, zoom::YZoom};

pub struct YDeclaration {
}

impl YDeclaration {
    pub fn from(declaration: &PropertyDeclaration, ruby: &Ruby) -> Result<Value, Error> {
        match declaration {
            PropertyDeclaration::AlignItems(item_placement) => {
                Ok(ruby.obj_wrap(YAlignItems::new(item_placement.clone())).as_value())
            },
            PropertyDeclaration::AlignmentBaseline(alignment_baseline) => {
                Ok(ruby.obj_wrap(YAlignmentBaseline::new(alignment_baseline.clone())).as_value())
            },
            PropertyDeclaration::AspectRatio(generic_aspect_ratio) => {
                Ok(ruby.obj_wrap(YAspectRatio::new(generic_aspect_ratio.clone())).as_value())
            },
            PropertyDeclaration::BackfaceVisibility(t) => {
                Ok(ruby.obj_wrap(YBackfaceVisibility::new(t.clone())).as_value())
            },
            PropertyDeclaration::BaselineSource(baseline_source) => {
                Ok(ruby.obj_wrap(YBaselineSource::new(baseline_source.clone())).as_value())
            }
            PropertyDeclaration::BorderCollapse(t) => {
                Ok(ruby.obj_wrap(YBorderCollapse::new(t.clone())).as_value())
            },
            PropertyDeclaration::BorderImageRepeat(border_image_repeat) => {
                Ok(ruby.obj_wrap(YBorderImageRepeat::new(border_image_repeat.clone())).as_value())
            }
            PropertyDeclaration::BoxSizing(t) => {
                Ok(ruby.obj_wrap(YBoxSizing::new(t.clone())).as_value())
            },
            PropertyDeclaration::CaptionSide(caption_side) => {
                Ok(ruby.obj_wrap(YCaptionSide::new(caption_side.clone())).as_value())
            },
            PropertyDeclaration::Clear(clear) => {
                Ok(ruby.obj_wrap(YClear::new(clear.clone())).as_value())
            }
            PropertyDeclaration::ColumnCount(generic_column_count) => {
                Ok(make_column_count(generic_column_count.clone(), ruby))
            }
            PropertyDeclaration::ColumnSpan(t) => {
                Ok(ruby.obj_wrap(YColumnSpan::new(t.clone())).as_value())
            },
            PropertyDeclaration::Contain(contain) => {
                Ok(ruby.obj_wrap(YContain::new(contain.clone())).as_value())
            }
            PropertyDeclaration::ContainerType(container_type) => {
                Ok(ruby.obj_wrap(YContainerType::new(container_type.clone())).as_value())
            }
            PropertyDeclaration::Direction(t) => {
                Ok(ruby.obj_wrap(YDirection::new(t.clone())).as_value())
            },
            PropertyDeclaration::Display(display) => {
                Ok(ruby.obj_wrap(YDisplay::new(display.clone())).as_value())
            }
            PropertyDeclaration::EmptyCells(t) => {
                Ok(ruby.obj_wrap(YEmptyCells::new(t.clone())).as_value())
            },
            PropertyDeclaration::FlexDirection(t) => {
                Ok(ruby.obj_wrap(YFlexDirection::new(t.clone())).as_value())
            },
            PropertyDeclaration::FlexWrap(t) => {
                Ok(ruby.obj_wrap(YFlexWrap::new(t.clone())).as_value())
            },
            PropertyDeclaration::Float(float) => {
                Ok(ruby.obj_wrap(YFloat::new(float.clone())).as_value())
            }
            PropertyDeclaration::FontLanguageOverride(font_language_override) => {
                Ok(ruby.obj_wrap(YFontLanguageOverride::new(font_language_override.clone())).as_value())
            }
            PropertyDeclaration::FontOpticalSizing(t) => {
                Ok(ruby.obj_wrap(YFontOpticalSizing::new(t.clone())).as_value())
            },
            PropertyDeclaration::FontStretch(font_stretch) => {
                Ok(ruby.obj_wrap(YFontStretch::new(font_stretch.clone())).as_value())
            }
            PropertyDeclaration::FontStyle(font_style) => {
                Ok(ruby.obj_wrap(YFontStyle::new(font_style.clone())).as_value())
            }
            PropertyDeclaration::FontSynthesisWeight(font_synthesis) => {
                Ok(ruby.obj_wrap(YFontSynthesisWeight::new(font_synthesis.clone())).as_value())
            }
            PropertyDeclaration::FontVariantCaps(t) => {
                Ok(ruby.obj_wrap(YFontVariantCaps::new(t.clone())).as_value())
            },
            PropertyDeclaration::FontWeight(font_weight) => {
                Ok(ruby.obj_wrap(YFontWeight::new(font_weight.clone())).as_value())
            }
            PropertyDeclaration::GridAutoFlow(grid_auto_flow) => {
                Ok(ruby.obj_wrap(YGridAutoFlow::new(grid_auto_flow.clone())).as_value())
            }
            PropertyDeclaration::ImageRendering(image_rendering) => {
                Ok(ruby.obj_wrap(YImageRendering::new(image_rendering.clone())).as_value())
            }
            PropertyDeclaration::Isolation(t) => {
                Ok(ruby.obj_wrap(YIsolation::new(t.clone())).as_value())
            },
            PropertyDeclaration::JustifyItems(justify_items) => {
                Ok(ruby.obj_wrap(YJustifyItems::new(justify_items.clone())).as_value())
            }
            PropertyDeclaration::LineBreak(line_break) => {
                Ok(ruby.obj_wrap(YLineBreak::new(line_break.clone())).as_value())
            }
            PropertyDeclaration::ListStylePosition(t) => {
                Ok(ruby.obj_wrap(YListStylePosition::new(t.clone())).as_value())
            },
            PropertyDeclaration::MixBlendMode(t) => {
                Ok(ruby.obj_wrap(YMixBlendMode::new(t.clone())).as_value())
            },
            PropertyDeclaration::ObjectFit(t) => {
                Ok(ruby.obj_wrap(YObjectFit::new(t.clone())).as_value())
            },
            PropertyDeclaration::Opacity(opacity) => {
                Ok(ruby.obj_wrap(YOpacity::new(opacity.clone())).as_value())
            }
            PropertyDeclaration::Order(integer) => {
                Ok(ruby.obj_wrap(YOrder::new(integer.clone())).as_value())
            }
            PropertyDeclaration::OutlineStyle(outline_style) => {
                Ok(ruby.obj_wrap(YOutlineStyle::new(outline_style.clone())).as_value())
            }
            PropertyDeclaration::OverflowWrap(overflow_wrap) => {
                Ok(ruby.obj_wrap(YOverflowWrap::new(overflow_wrap.clone())).as_value())
            }
            PropertyDeclaration::PointerEvents(pointer_events) => {
                Ok(ruby.obj_wrap(YPointerEvents::new(pointer_events.clone())).as_value())
            }
            PropertyDeclaration::Position(position_property) => {
                Ok(ruby.obj_wrap(YPosition::new(position_property.clone())).as_value())
            }
            PropertyDeclaration::PositionArea(position_area) => {
                Ok(ruby.obj_wrap(YPositionArea::new(position_area.clone())).as_value())
            }
            PropertyDeclaration::ServoOverflowClipBox(t) => {
                Ok(ruby.obj_wrap(YServoOverflowClipBox::new(t.clone())).as_value())
            },
            PropertyDeclaration::ServoTopLayer(t) => {
                Ok(ruby.obj_wrap(YServoTopLayer::new(t.clone())).as_value())
            },
            PropertyDeclaration::TableLayout(t) => {
                Ok(ruby.obj_wrap(YTableLayout::new(t.clone())).as_value())
            },
            PropertyDeclaration::TextAlign(text_align) => {
                Ok(ruby.obj_wrap(YTextAlign::new(text_align.clone())).as_value())
            }
            PropertyDeclaration::TextAlignLast(text_align_last) => {
                Ok(ruby.obj_wrap(YTextAlignLast::new(text_align_last.clone())).as_value())
            }
            PropertyDeclaration::TextDecorationLine(text_decoration_line) => {
                Ok(ruby.obj_wrap(YTextDecorationLine::new(text_decoration_line.clone())).as_value())
            }
            PropertyDeclaration::TextDecorationStyle(t) => {
                Ok(ruby.obj_wrap(YTextDecorationStyle::new(t.clone())).as_value())
            },
            PropertyDeclaration::TextJustify(text_justify) => {
                Ok(ruby.obj_wrap(YTextJustify::new(text_justify.clone())).as_value())
            }
            PropertyDeclaration::TextRendering(t) => {
                Ok(ruby.obj_wrap(YTextRendering::new(t.clone())).as_value())
            },
            PropertyDeclaration::TextTransform(text_transform) => {
                Ok(ruby.obj_wrap(YTextTransform::new(text_transform.clone())).as_value())
            }
            PropertyDeclaration::TextWrapMode(t) => {
                Ok(ruby.obj_wrap(YTextWrapMode::new(t.clone())).as_value())
            },
            PropertyDeclaration::TransformStyle(transform_style) => {
                Ok(ruby.obj_wrap(YTransformStyle::new(transform_style.clone())).as_value())
            }
            PropertyDeclaration::UnicodeBidi(t) => {
                Ok(ruby.obj_wrap(YUnicodeBidi::new(t.clone())).as_value())
            },
            PropertyDeclaration::Visibility(t) => {
                Ok(ruby.obj_wrap(YVisibility::new(t.clone())).as_value())
            },
            PropertyDeclaration::WebkitTextSecurity(t) => {
                Ok(ruby.obj_wrap(YWebkitTextSecurity::new(t.clone())).as_value())
            },
            PropertyDeclaration::WhiteSpaceCollapse(t) => {
                Ok(ruby.obj_wrap(YWhiteSpaceCollapse::new(t.clone())).as_value())
            },
            PropertyDeclaration::WordBreak(word_break) => {
                Ok(ruby.obj_wrap(YWordBreak::new(word_break.clone())).as_value())
            }
            PropertyDeclaration::WritingMode(writing_mode_property) => {
                Ok(ruby.obj_wrap(YWritingMode::new(writing_mode_property.clone())).as_value())
            }
            PropertyDeclaration::ZIndex(generic_zindex) => {
                Ok(ruby.obj_wrap(YZIndex::new(generic_zindex.clone())).as_value())
            }
            PropertyDeclaration::Zoom(zoom) => {
                Ok(ruby.obj_wrap(YZoom::new(zoom.clone())).as_value())
            }
            PropertyDeclaration::AlignContent(content_distribution) => {
                Ok(ruby.obj_wrap(YAlignContent::new(content_distribution.clone())).as_value())
            }
            PropertyDeclaration::JustifyContent(content_distribution) => {
                Ok(ruby.obj_wrap(YJustifyContent::new(content_distribution.clone())).as_value())
            }
            PropertyDeclaration::FlexGrow(non_negative) => {
                Ok(ruby.obj_wrap(YFlexGrow::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::FlexShrink(non_negative) => {
                Ok(ruby.obj_wrap(YFlexShrink::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::AlignSelf(self_alignment) => {
                Ok(ruby.obj_wrap(YAlignSelf::new(self_alignment.clone())).as_value())
            }
            PropertyDeclaration::JustifySelf(self_alignment) => {
                Ok(ruby.obj_wrap(YJustifySelf::new(self_alignment.clone())).as_value())
            }
            PropertyDeclaration::OverflowBlock(overflow) => {
                Ok(ruby.obj_wrap(YOverflowBlock::new(overflow.clone())).as_value())
            }
            PropertyDeclaration::OverflowInline(overflow) => {
                Ok(ruby.obj_wrap(YOverflowInline::new(overflow.clone())).as_value())
            }
            PropertyDeclaration::OverflowX(overflow) => {
                Ok(ruby.obj_wrap(YOverflowX::new(overflow.clone())).as_value())
            }
            PropertyDeclaration::OverflowY(overflow) => {
                Ok(ruby.obj_wrap(YOverflowY::new(overflow.clone())).as_value())
            }
            PropertyDeclaration::BorderBlockEndStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderBlockEndStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderBlockStartStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderBlockStartStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderBottomStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderBottomStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderInlineEndStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderInlineEndStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderInlineStartStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderInlineStartStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderLeftStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderLeftStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderRightStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderRightStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::BorderTopStyle(border_style) => {
                Ok(ruby.obj_wrap(YBorderTopStyle::new(border_style.clone())).as_value())
            }
            PropertyDeclaration::AnimationComposition(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationComposition::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationDelay(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationDelay::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationDirection(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationDirection::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationDuration(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationDuration::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationFillMode(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationFillMode::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationIterationCount(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationIterationCount::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationName(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationName::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationPlayState(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationPlayState::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationRangeEnd(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationRangeEnd::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::AnimationRangeStart(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationRangeStart::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::AnimationTimeline(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationTimeline::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::AnimationTimingFunction(specified_value) => {
                Ok(ruby.obj_wrap(YAnimationTimingFunction::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::BackdropFilter(specified_value) => {
                Ok(ruby.obj_wrap(YBackdropFilter::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::BackgroundAttachment(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundAttachment::new(specified_value.clone())).as_value())
            }
            PropertyDeclaration::BackgroundClip(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundClip::new(specified_value.0.clone())).as_value())
            },
            PropertyDeclaration::BackgroundImage(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundImage::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::BackgroundOrigin(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundOrigin::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::BackgroundPositionX(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundPositionX::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::BackgroundPositionY(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundPositionY::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::BackgroundRepeat(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundRepeat::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::BackgroundSize(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundSize::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::BaselineShift(generic_baseline_shift) => {
                Ok(make_baseline_shift(generic_baseline_shift.clone(), ruby))
            }
            PropertyDeclaration::BorderImageOutset(rect) => {
                Ok(ruby.obj_wrap(YBorderImageOutset::new(rect.clone())).as_value())
            }
            PropertyDeclaration::BorderImageSlice(generic_border_image_slice) => {
                Ok(ruby.obj_wrap(YBorderImageSlice::new(generic_border_image_slice.clone())).as_value())
            }
            PropertyDeclaration::BorderImageWidth(rect) => {
                Ok(ruby.obj_wrap(YBorderImageWidth::new(rect.clone())).as_value())
            }
            PropertyDeclaration::BorderSpacing(generic_border_spacing) => {
                Ok(ruby.obj_wrap(YBorderSpacing::new(generic_border_spacing.clone())).as_value())
            }
            PropertyDeclaration::BoxShadow(specified_value) => {
                Ok(ruby.obj_wrap(YBoxShadow::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::CaretColor(generic_caret_color) => {
                Ok(ruby.obj_wrap(YCaretColor::new(generic_caret_color.clone())).as_value())
            },
            PropertyDeclaration::Clip(generic_clip_rect_or_auto) => {
                Ok(ruby.obj_wrap(YClip::new(generic_clip_rect_or_auto.clone())).as_value())
            }
            PropertyDeclaration::ClipPath(generic_clip_path) => {
                Ok(ruby.obj_wrap(YClipPath::new(generic_clip_path.clone())).as_value())
            },
            PropertyDeclaration::Color(color_property_value) => {
                Ok(ruby.obj_wrap(YColor::new(color_property_value.clone())).as_value())
            }
            PropertyDeclaration::ColorScheme(color_scheme) => {
                Ok(ruby.obj_wrap(YColorScheme::new(color_scheme.clone())).as_value())
            }
            PropertyDeclaration::ColumnWidth(generic_length_percentage_or_auto) => {
                Ok(ruby.obj_wrap(YColumnWidth::new(generic_length_percentage_or_auto.clone())).as_value())
            },
            PropertyDeclaration::ContainerName(container_name) => {
                Ok(ruby.obj_wrap(YContainerName::new(container_name.clone())).as_value())
            }
            PropertyDeclaration::Content(generic_content) => {
                Ok(ruby.obj_wrap(YContent::new(generic_content.clone())).as_value())
            }
            PropertyDeclaration::CounterIncrement(generic_counter_increment) => {
                Ok(ruby.obj_wrap(YCounterIncrement::new(generic_counter_increment.clone())).as_value())
            }
            PropertyDeclaration::CounterReset(generic_counter_reset) => {
                Ok(ruby.obj_wrap(YCounterReset::new(generic_counter_reset.clone())).as_value())
            }
            PropertyDeclaration::Cursor(generic_cursor) => {
                Ok(ruby.obj_wrap(YCursor::new(generic_cursor.clone())).as_value())
            }
            PropertyDeclaration::Filter(specified_value) => {
                Ok(ruby.obj_wrap(YFilter::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::FlexBasis(generic_flex_basis) => {
                Ok(ruby.obj_wrap(YFlexBasis::new(generic_flex_basis.clone())).as_value())
            }
            PropertyDeclaration::FontFamily(font_family) => {
                Ok(ruby.obj_wrap(YFontFamily::new(font_family.clone())).as_value())
            }
            PropertyDeclaration::FontSize(font_size) => {
                Ok(ruby.obj_wrap(YFontSize::new(font_size.clone())).as_value())
            }
            PropertyDeclaration::FontVariationSettings(font_settings) => {
                Ok(ruby.obj_wrap(YFontVariationSettings::new(font_settings.clone())).as_value())
            }
            PropertyDeclaration::GridTemplateAreas(grid_template_areas) => {
                Ok(ruby.obj_wrap(YGridTemplateAreas::new(grid_template_areas.clone())).as_value())
            }
            PropertyDeclaration::LetterSpacing(letter_spacing) => {
                Ok(ruby.obj_wrap(YLetterSpacing::new(letter_spacing.clone())).as_value())
            }
            PropertyDeclaration::LineHeight(generic_line_height) => {
                Ok(ruby.obj_wrap(YLineHeight::new(generic_line_height.clone())).as_value())
            }
            PropertyDeclaration::ListStyleType(list_style_type) => {
                Ok(ruby.obj_wrap(YListStyleType::new(list_style_type.clone())).as_value())
            }
            PropertyDeclaration::MaskImage(specified_value) => {
                Ok(ruby.obj_wrap(YMaskImage::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::OffsetPath(generic_offset_path) => {
                Ok(ruby.obj_wrap(YOffsetPath::new(generic_offset_path.clone())).as_value())
            }
            PropertyDeclaration::OutlineOffset(border_side_offset) => {
                Ok(ruby.obj_wrap(YOutlineOffset::new(border_side_offset.clone())).as_value())
            }
            PropertyDeclaration::OverflowClipMargin(generic_overflow_clip_margin) => {
                Ok(ruby.obj_wrap(YOverflowClipMargin::new(generic_overflow_clip_margin.clone())).as_value())
            }
            PropertyDeclaration::Perspective(generic_perspective) => {
                Ok(ruby.obj_wrap(YPerspective::new(generic_perspective.clone())).as_value())
            }
            PropertyDeclaration::PositionTryFallbacks(position_try_fallbacks) => {
                Ok(ruby.obj_wrap(YPositionTryFallbacks::new(position_try_fallbacks.clone())).as_value())
            }
            PropertyDeclaration::Quotes(quotes) => {
                Ok(ruby.obj_wrap(YQuotes::new(quotes.clone())).as_value())
            }
            PropertyDeclaration::Rotate(generic_rotate) => {
                Ok(ruby.obj_wrap(YRotate::new(generic_rotate.clone())).as_value())
            }
            PropertyDeclaration::Scale(generic_scale) => {
                Ok(ruby.obj_wrap(YScale::new(generic_scale.clone())).as_value())
            }
            PropertyDeclaration::TextIndent(generic_text_indent) => {
                Ok(ruby.obj_wrap(YTextIndent::new(generic_text_indent.clone())).as_value())
            }
            PropertyDeclaration::TextOverflow(text_overflow) => {
                Ok(ruby.obj_wrap(YTextOverflow::new(text_overflow.clone())).as_value())
            }
            PropertyDeclaration::TextShadow(specified_value) => {
                Ok(ruby.obj_wrap(YTextShadow::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::Transform(generic_transform) => {
                Ok(ruby.obj_wrap(YTransform::new(generic_transform.clone())).as_value())
            }
            PropertyDeclaration::TransformOrigin(generic_transform_origin) => {
                Ok(ruby.obj_wrap(YTransformOrigin::new(generic_transform_origin.clone())).as_value())
            }
            PropertyDeclaration::TransitionBehavior(specified_value) => {
                Ok(ruby.obj_wrap(YTransitionBehavior::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::TransitionDelay(specified_value) => {
                Ok(ruby.obj_wrap(YTransitionDelay::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::TransitionDuration(specified_value) => {
                Ok(ruby.obj_wrap(YTransitionDuration::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::TransitionProperty(specified_value) => {
                Ok(ruby.obj_wrap(YTransitionProperty::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::TransitionTimingFunction(specified_value) => {
                Ok(ruby.obj_wrap(YTransitionTimingFunction::new(specified_value.clone())).as_value())
            },
            PropertyDeclaration::Translate(generic_translate) => {
                Ok(ruby.obj_wrap(YTranslate::new(generic_translate.clone())).as_value())
            }
            PropertyDeclaration::ViewTransitionClass(view_transition_class) => {
                Ok(ruby.obj_wrap(YViewTransitionClass::new(view_transition_class.clone())).as_value())
            }
            PropertyDeclaration::ViewTransitionName(view_transition_name) => {
                Ok(ruby.obj_wrap(YViewTransitionName::new(view_transition_name.clone())).as_value())
            }
            PropertyDeclaration::WillChange(will_change) => {
                Ok(ruby.obj_wrap(YWillChange::new(will_change.clone())).as_value())
            }
            PropertyDeclaration::WordSpacing(word_spacing) => {
                Ok(ruby.obj_wrap(YWordSpacing::new(word_spacing.clone())).as_value())
            }
            PropertyDeclaration::XLang(xlang) => {
                Ok(ruby.obj_wrap(YXLang::new(xlang.clone())).as_value())
            }
            PropertyDeclaration::ObjectPosition(generic_position) => {
                Ok(ruby.obj_wrap(YObjectPosition::new(generic_position.clone())).as_value())
            }
            PropertyDeclaration::PerspectiveOrigin(generic_position) => {
                Ok(ruby.obj_wrap(YPerspectiveOrigin::new(generic_position.clone())).as_value())
            }
            PropertyDeclaration::GridTemplateColumns(generic_grid_template_component) => {
                Ok(ruby.obj_wrap(YGridTemplateColumns::new(generic_grid_template_component.clone())).as_value())
            }
            PropertyDeclaration::GridTemplateRows(generic_grid_template_component) => {
                Ok(ruby.obj_wrap(YGridTemplateRows::new(generic_grid_template_component.clone())).as_value())
            }
            PropertyDeclaration::BorderImageSource(generic_image) => {
                Ok(ruby.obj_wrap(YBorderImageSource::new(generic_image.clone())).as_value())
            }
            PropertyDeclaration::ListStyleImage(generic_image) => {
                Ok(ruby.obj_wrap(YListStyleImage::new(generic_image.clone())).as_value())
            }
            PropertyDeclaration::GridAutoColumns(generic_implicit_grid_tracks) => {
                Ok(ruby.obj_wrap(YGridAutoColumns::new(generic_implicit_grid_tracks.clone())).as_value())
            }
            PropertyDeclaration::GridAutoRows(generic_implicit_grid_tracks) => {
                Ok(ruby.obj_wrap(YGridAutoRows::new(generic_implicit_grid_tracks.clone())).as_value())
            }
            PropertyDeclaration::ColumnGap(generic_length_percentage_or_normal) => {
                Ok(ruby.obj_wrap(YColumnGap::new(generic_length_percentage_or_normal.clone())).as_value())
            },
            PropertyDeclaration::RowGap(generic_length_percentage_or_normal) => {
                Ok(ruby.obj_wrap(YRowGap::new(generic_length_percentage_or_normal.clone())).as_value())
            },
            PropertyDeclaration::GridColumnEnd(generic_grid_line) => {
                Ok(ruby.obj_wrap(YGridColumnEnd::new(generic_grid_line.clone())).as_value())
            }
            PropertyDeclaration::GridColumnStart(generic_grid_line) => {
                Ok(ruby.obj_wrap(YGridColumnStart::new(generic_grid_line.clone())).as_value())
            }
            PropertyDeclaration::GridRowEnd(generic_grid_line) => {
                Ok(ruby.obj_wrap(YGridRowEnd::new(generic_grid_line.clone())).as_value())
            }
            PropertyDeclaration::GridRowStart(generic_grid_line) => {
                Ok(ruby.obj_wrap(YGridRowStart::new(generic_grid_line.clone())).as_value())
            }
            PropertyDeclaration::MaxBlockSize(generic_max_size) => {
                Ok(ruby.obj_wrap(YMaxBlockSize::new(generic_max_size.clone())).as_value())
            }
            PropertyDeclaration::MaxHeight(generic_max_size) => {
                Ok(ruby.obj_wrap(YMaxHeight::new(generic_max_size.clone())).as_value())
            }
            PropertyDeclaration::MaxInlineSize(generic_max_size) => {
                Ok(ruby.obj_wrap(YMaxInlineSize::new(generic_max_size.clone())).as_value())
            }
            PropertyDeclaration::MaxWidth(generic_max_size) => {
                Ok(ruby.obj_wrap(YMaxWidth::new(generic_max_size.clone())).as_value())
            }
            PropertyDeclaration::BorderBottomLeftRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderBottomLeftRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderBottomRightRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderBottomRightRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderEndEndRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderEndEndRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderEndStartRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderEndStartRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderStartEndRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderStartEndRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderStartStartRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderStartStartRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderTopLeftRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderTopLeftRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::BorderTopRightRadius(generic_border_corner_radius) => {
                Ok(ruby.obj_wrap(YBorderTopRightRadius::new(generic_border_corner_radius.clone())).as_value())
            }
            PropertyDeclaration::Bottom(generic_inset) => {
                Ok(ruby.obj_wrap(YBottom::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::InsetBlockEnd(generic_inset) => {
                Ok(ruby.obj_wrap(YInsetBlockEnd::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::InsetBlockStart(generic_inset) => {
                Ok(ruby.obj_wrap(YInsetBlockStart::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::InsetInlineEnd(generic_inset) => {
                Ok(ruby.obj_wrap(YInsetInlineEnd::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::InsetInlineStart(generic_inset) => {
                Ok(ruby.obj_wrap(YInsetInlineStart::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::Left(generic_inset) => {
                Ok(ruby.obj_wrap(YLeft::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::Right(generic_inset) => {
                Ok(ruby.obj_wrap(YRight::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::Top(generic_inset) => {
                Ok(ruby.obj_wrap(YTop::new(generic_inset.clone())).as_value())
            }
            PropertyDeclaration::MarginBlockEnd(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginBlockEnd::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginBlockStart(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginBlockStart::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginBottom(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginBottom::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginInlineEnd(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginInlineEnd::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginInlineStart(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginInlineStart::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginLeft(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginLeft::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginRight(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginRight::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::MarginTop(generic_margin) => {
                Ok(ruby.obj_wrap(YMarginTop::new(generic_margin.clone())).as_value())
            }
            PropertyDeclaration::PaddingBlockEnd(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingBlockEnd::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingBlockStart(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingBlockStart::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingBottom(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingBottom::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingInlineEnd(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingInlineEnd::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingInlineStart(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingInlineStart::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingLeft(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingLeft::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingRight(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingRight::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::PaddingTop(non_negative) => {
                Ok(ruby.obj_wrap(YPaddingTop::new(non_negative.clone())).as_value())
            }
            PropertyDeclaration::BlockSize(generic_size) => {
                Ok(ruby.obj_wrap(YBlockSize::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::Height(generic_size) => {
                Ok(ruby.obj_wrap(YHeight::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::InlineSize(generic_size) => {
                Ok(ruby.obj_wrap(YInlineSize::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::MinBlockSize(generic_size) => {
                Ok(ruby.obj_wrap(YMinBlockSize::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::MinHeight(generic_size) => {
                Ok(ruby.obj_wrap(YMinHeight::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::MinInlineSize(generic_size) => {
                Ok(ruby.obj_wrap(YMinInlineSize::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::MinWidth(generic_size) => {
                Ok(ruby.obj_wrap(YMinWidth::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::Width(generic_size) => {
                Ok(ruby.obj_wrap(YWidth::new(generic_size.clone())).as_value())
            }
            PropertyDeclaration::BorderBlockEndWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderBlockEndWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderBlockStartWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderBlockStartWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderBottomWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderBottomWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderInlineEndWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderInlineEndWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderInlineStartWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderInlineStartWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderLeftWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderLeftWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderRightWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderRightWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BorderTopWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YBorderTopWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::OutlineWidth(border_side_width) => {
                Ok(ruby.obj_wrap(YOutlineWidth::new(border_side_width.clone())).as_value())
            }
            PropertyDeclaration::BackgroundColor(color) => {
                Ok(ruby.obj_wrap(YBackgroundColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderBlockEndColor(color) => {
                Ok(ruby.obj_wrap(YBorderBlockEndColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderBlockStartColor(color) => {
                Ok(ruby.obj_wrap(YBorderBlockStartColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderBottomColor(color) => {
                Ok(ruby.obj_wrap(YBorderBottomColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderInlineEndColor(color) => {
                Ok(ruby.obj_wrap(YBorderInlineEndColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderInlineStartColor(color) => {
                Ok(ruby.obj_wrap(YBorderInlineStartColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderLeftColor(color) => {
                Ok(ruby.obj_wrap(YBorderLeftColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderRightColor(color) => {
                Ok(ruby.obj_wrap(YBorderRightColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::BorderTopColor(color) => {
                Ok(ruby.obj_wrap(YBorderTopColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::OutlineColor(color) => {
                Ok(ruby.obj_wrap(YOutlineColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::TextDecorationColor(color) => {
                Ok(ruby.obj_wrap(YTextDecorationColor::new(color.clone())).as_value())
            }
            PropertyDeclaration::CSSWideKeyword(wide_keyword_declaration) => {
                Ok(ruby.obj_wrap(YCSSWideKeyword::new(wide_keyword_declaration.clone())).as_value())
            },
            PropertyDeclaration::WithVariables(variable_declaration) => {
                Ok(ruby.obj_wrap(YWithVariables::new(variable_declaration.clone())).as_value())
            },
            PropertyDeclaration::Custom(custom_declaration) => {
                Ok(ruby.obj_wrap(YCustom::new(custom_declaration.clone())).as_value())
            },
            PropertyDeclaration::BackgroundBlendMode(specified_value) => {
                Ok(ruby.obj_wrap(YBackgroundBlendMode::new(specified_value.clone())).as_value())
            },
        }
    }
}
