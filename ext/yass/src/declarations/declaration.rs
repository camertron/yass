use magnus::{Error, Module, RModule, Ruby, Value, method, value::ReprValue};
use style::properties::PropertyDeclaration;

use crate::declarations::{
  YAlignContent,
  YAlignItems,
  YAlignSelf,
  YAlignmentBaseline,
  YAnimationComposition,
  YAnimationDelay,
  YAnimationDirection,
  YAnimationDuration,
  YAnimationFillMode,
  YAnimationIterationCount,
  YAnimationName,
  YAnimationPlayState,
  YAnimationTimeline,
  YAnimationTimingFunction,
  YAspectRatio,
  YBackdropFilter,
  YBackfaceVisibility,
  YBackgroundAttachment,
  YBackgroundClip,
  YBackgroundColor,
  YBackgroundImage,
  YBackgroundOrigin,
  YBackgroundPositionX,
  YBackgroundPositionY,
  YBackgroundRepeat,
  YBackgroundSize,
  YBaselineShift,
  YBaselineSource,
  YBlockSize,
  YBorderBlockEndColor,
  YBorderBlockEndStyle,
  YBorderBlockEndWidth,
  YBorderBlockStartColor,
  YBorderBlockStartStyle,
  YBorderBlockStartWidth,
  YBorderBottomColor,
  YBorderBottomStyle,
  YBorderBottomWidth,
  YBorderCollapse,
  YBorderImageRepeat,
  YBorderImageSource,
  YBorderInlineEndColor,
  YBorderInlineEndStyle,
  YBorderInlineEndWidth,
  YBorderInlineStartColor,
  YBorderInlineStartStyle,
  YBorderInlineStartWidth,
  YBorderLeftColor,
  YBorderLeftStyle,
  YBorderLeftWidth,
  YBorderRightColor,
  YBorderRightStyle,
  YBorderRightWidth,
  YBorderTopColor,
  YBorderTopStyle,
  YBorderTopWidth,
  YBottom,
  YBoxShadow,
  YBoxSizing,
  YCSSWideKeyword,
  YCaptionSide,
  YCaretColor,
  YClear,
  YClipPath,
  YColor,
  YColorScheme,
  YColumnCount,
  YColumnGap,
  YColumnSpan,
  YColumnWidth,
  YContain,
  YContainerName,
  YContainerType,
  YContent,
  YCounterIncrement,
  YCounterReset,
  YCursor,
  YCustom,
  YDirection,
  YDisplay,
  YEmptyCells,
  YFilter,
  YFlexDirection,
  YFlexGrow,
  YFlexShrink,
  YFlexWrap,
  YFloat,
  YFontFamily,
  YFontLanguageOverride,
  YFontOpticalSizing,
  YFontSize,
  YFontStretch,
  YFontStyle,
  YFontSynthesisWeight,
  YFontVariantCaps,
  YFontVariationSettings,
  YFontWeight,
  YGridAutoColumns,
  YGridAutoFlow,
  YGridAutoRows,
  YGridColumnEnd,
  YGridColumnStart,
  YGridRowEnd,
  YGridRowStart,
  YGridTemplateAreas,
  YGridTemplateColumns,
  YGridTemplateRows,
  YHeight,
  YImageRendering,
  YInlineSize,
  YInsetBlockEnd,
  YInsetBlockStart,
  YInsetInlineEnd,
  YInsetInlineStart,
  YIsolation,
  YJustifyContent,
  YJustifyItems,
  YJustifySelf,
  YLeft,
  YLetterSpacing,
  YLineBreak,
  YLineHeight,
  YListStyleImage,
  YListStylePosition,
  YListStyleType,
  YMarginBlockEnd,
  YMarginBlockStart,
  YMarginBottom,
  YMarginInlineEnd,
  YMarginInlineStart,
  YMarginLeft,
  YMarginRight,
  YMarginTop,
  YMaskImage,
  YMaxBlockSize,
  YMaxHeight,
  YMaxInlineSize,
  YMaxWidth,
  YMinBlockSize,
  YMinHeight,
  YMinInlineSize,
  YMinWidth,
  YMixBlendMode,
  YObjectFit,
  YOffsetPath,
  YOpacity,
  YOrder,
  YOutlineColor,
  YOutlineOffset,
  YOutlineStyle,
  YOutlineWidth,
  YOverflowBlock,
  YOverflowClipMargin,
  YOverflowInline,
  YOverflowWrap,
  YOverflowX,
  YOverflowY,
  YPaddingBlockEnd,
  YPaddingBlockStart,
  YPaddingBottom,
  YPaddingInlineEnd,
  YPaddingInlineStart,
  YPaddingLeft,
  YPaddingRight,
  YPaddingTop,
  YPerspective,
  YPointerEvents,
  YPosition,
  YPositionArea,
  YPositionTryFallbacks,
  YQuotes,
  YRight,
  YRowGap,
  YServoOverflowClipBox,
  YServoTopLayer,
  YTableLayout,
  YTextAlign,
  YTextAlignLast,
  YTextDecorationColor,
  YTextDecorationLine,
  YTextDecorationStyle,
  YTextIndent,
  YTextJustify,
  YTextRendering,
  YTextShadow,
  YTextTransform,
  YTextWrapMode,
  YTop,
  YTransform,
  YTransformStyle,
  YTransitionBehavior,
  YTransitionDelay,
  YTransitionDuration,
  YTransitionProperty,
  YTransitionTimingFunction,
  YUnicodeBidi,
  YViewTransitionClass,
  YViewTransitionName,
  YVisibility,
  YWebkitTextSecurity,
  YWhiteSpaceCollapse,
  YWidth,
  YWillChange,
  YWithVariables,
  YWordBreak,
  YWordSpacing,
  YWritingMode,
  YXLang,
  YZIndex,
  YZoom,
  border_bottom_left_radius::YBorderBottomLeftRadius,
  border_bottom_right_radius::YBorderBottomRightRadius,
  border_end_end_radius::YBorderEndEndRadius,
  border_end_start_radius::YBorderEndStartRadius,
  border_image_outset::YBorderImageOutset,
  border_image_slice::YBorderImageSlice,
  border_image_width::YBorderImageWidth,
  border_spacing::YBorderSpacing,
  border_start_end_radius::YBorderStartEndRadius,
  border_start_start_radius::YBorderStartStartRadius,
  border_top_left_radius::YBorderTopLeftRadius,
  border_top_right_radius::YBorderTopRightRadius,
  clip::YClip,
  flex_basis::YFlexBasis,
  object_position::YObjectPosition,
  perspective_origin::YPerspectiveOrigin,
  rotate::YRotate,
  scale::YScale,
  text_overflow::YTextOverflow,
  transform_origin::YTransformOrigin,
  translate::YTranslate
};

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
          Ok(ruby.obj_wrap(YColumnCount::new(generic_column_count.clone())).as_value())
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
          Ok(ruby.obj_wrap(YBaselineShift::new(generic_baseline_shift.clone())).as_value())
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
    }
  }

  pub fn init(ruby: &Ruby, yass_module: &RModule) -> Result<(), Error> {
    let declarations_module = yass_module.define_module("Declarations")?;

    let align_items_class = declarations_module.define_class("AlignItems", ruby.class_object())?;
    align_items_class.define_method("value", method!(YAlignItems::value, 0))?;

    let alignment_baseline_class = declarations_module.define_class("AlignmentBaseline", ruby.class_object())?;
    alignment_baseline_class.define_method("value", method!(YAlignmentBaseline::value, 0))?;

    let aspect_ratio_class = declarations_module.define_class("AspectRatio", ruby.class_object())?;

    let backface_visibility_class = declarations_module.define_class("BackfaceVisibility", ruby.class_object())?;

    let baseline_source_class = declarations_module.define_class("BaselineSource", ruby.class_object())?;

    let border_collapse_class = declarations_module.define_class("BorderCollapse", ruby.class_object())?;

    let border_image_repeat_class = declarations_module.define_class("BorderImageRepeat", ruby.class_object())?;

    let box_sizing_class = declarations_module.define_class("BoxSizing", ruby.class_object())?;

    let caption_side_class = declarations_module.define_class("CaptionSide", ruby.class_object())?;

    let clear_class = declarations_module.define_class("Clear", ruby.class_object())?;

    let column_count_class = declarations_module.define_class("ColumnCount", ruby.class_object())?;

    let column_span_class = declarations_module.define_class("ColumnSpan", ruby.class_object())?;

    let contain_class = declarations_module.define_class("Contain", ruby.class_object())?;

    let container_type_class = declarations_module.define_class("ContainerType", ruby.class_object())?;

    let direction_class = declarations_module.define_class("Direction", ruby.class_object())?;

    let display_class = declarations_module.define_class("Display", ruby.class_object())?;

    let empty_cells_class = declarations_module.define_class("EmptyCells", ruby.class_object())?;

    let flex_direction_class = declarations_module.define_class("FlexDirection", ruby.class_object())?;

    let flex_wrap_class = declarations_module.define_class("FlexWrap", ruby.class_object())?;

    let float_class = declarations_module.define_class("Float", ruby.class_object())?;

    let font_language_override_class = declarations_module.define_class("FontLanguageOverride", ruby.class_object())?;

    let font_optical_sizing_class = declarations_module.define_class("FontOpticalSizing", ruby.class_object())?;

    let font_stretch_class = declarations_module.define_class("FontStretch", ruby.class_object())?;

    let font_style_class = declarations_module.define_class("FontStyle", ruby.class_object())?;

    let font_synthesis_weight_class = declarations_module.define_class("FontSynthesisWeight", ruby.class_object())?;

    let font_variant_caps_class = declarations_module.define_class("FontVariantCaps", ruby.class_object())?;

    let font_weight_class = declarations_module.define_class("FontWeight", ruby.class_object())?;

    let grid_auto_flow_class = declarations_module.define_class("GridAutoFlow", ruby.class_object())?;

    let image_rendering_class = declarations_module.define_class("ImageRendering", ruby.class_object())?;

    let isolation_class = declarations_module.define_class("Isolation", ruby.class_object())?;

    let justify_items_class = declarations_module.define_class("JustifyItems", ruby.class_object())?;

    let line_break_class = declarations_module.define_class("LineBreak", ruby.class_object())?;

    let list_style_position_class = declarations_module.define_class("ListStylePosition", ruby.class_object())?;

    let mix_blend_mode_class = declarations_module.define_class("MixBlendMode", ruby.class_object())?;

    let object_fit_class = declarations_module.define_class("ObjectFit", ruby.class_object())?;

    let opacity_class = declarations_module.define_class("Opacity", ruby.class_object())?;

    let order_class = declarations_module.define_class("Order", ruby.class_object())?;

    let outline_style_class = declarations_module.define_class("OutlineStyle", ruby.class_object())?;

    let overflow_wrap_class = declarations_module.define_class("OverflowWrap", ruby.class_object())?;

    let pointer_events_class = declarations_module.define_class("PointerEvents", ruby.class_object())?;

    let position_class = declarations_module.define_class("Position", ruby.class_object())?;

    let position_area_class = declarations_module.define_class("PositionArea", ruby.class_object())?;

    let servo_overflow_clip_box_class = declarations_module.define_class("ServoOverflowClipBox", ruby.class_object())?;

    let servo_top_layer_class = declarations_module.define_class("ServoTopLayer", ruby.class_object())?;

    let table_layout_class = declarations_module.define_class("TableLayout", ruby.class_object())?;

    let text_align_class = declarations_module.define_class("TextAlign", ruby.class_object())?;

    let text_align_last_class = declarations_module.define_class("TextAlignLast", ruby.class_object())?;

    let text_decoration_line_class = declarations_module.define_class("TextDecorationLine", ruby.class_object())?;

    let text_decoration_style_class = declarations_module.define_class("TextDecorationStyle", ruby.class_object())?;

    let text_justify_class = declarations_module.define_class("TextJustify", ruby.class_object())?;

    let text_rendering_class = declarations_module.define_class("TextRendering", ruby.class_object())?;

    let text_transform_class = declarations_module.define_class("TextTransform", ruby.class_object())?;

    let text_wrap_mode_class = declarations_module.define_class("TextWrapMode", ruby.class_object())?;

    let transform_style_class = declarations_module.define_class("TransformStyle", ruby.class_object())?;

    let unicode_bidi_class = declarations_module.define_class("UnicodeBidi", ruby.class_object())?;

    let visibility_class = declarations_module.define_class("Visibility", ruby.class_object())?;

    let webkit_text_security_class = declarations_module.define_class("WebkitTextSecurity", ruby.class_object())?;

    let white_space_collapse_class = declarations_module.define_class("WhiteSpaceCollapse", ruby.class_object())?;

    let word_break_class = declarations_module.define_class("WordBreak", ruby.class_object())?;

    let writing_mode_class = declarations_module.define_class("WritingMode", ruby.class_object())?;

    let zindex_class = declarations_module.define_class("ZIndex", ruby.class_object())?;

    let zoom_class = declarations_module.define_class("Zoom", ruby.class_object())?;

    let align_content_class = declarations_module.define_class("AlignContent", ruby.class_object())?;

    let justify_content_class = declarations_module.define_class("JustifyContent", ruby.class_object())?;

    let flex_grow_class = declarations_module.define_class("FlexGrow", ruby.class_object())?;

    let flex_shrink_class = declarations_module.define_class("FlexShrink", ruby.class_object())?;

    let align_self_class = declarations_module.define_class("AlignSelf", ruby.class_object())?;

    let justify_self_class = declarations_module.define_class("JustifySelf", ruby.class_object())?;

    let overflow_block_class = declarations_module.define_class("OverflowBlock", ruby.class_object())?;

    let overflow_inline_class = declarations_module.define_class("OverflowInline", ruby.class_object())?;

    let overflow_x_class = declarations_module.define_class("OverflowX", ruby.class_object())?;

    let overflow_y_class = declarations_module.define_class("OverflowY", ruby.class_object())?;

    let border_block_end_style_class = declarations_module.define_class("BorderBlockEndStyle", ruby.class_object())?;

    let border_block_start_style_class = declarations_module.define_class("BorderBlockStartStyle", ruby.class_object())?;

    let border_bottom_style_class = declarations_module.define_class("BorderBottomStyle", ruby.class_object())?;

    let border_inline_end_style_class = declarations_module.define_class("BorderInlineEndStyle", ruby.class_object())?;

    let border_inline_start_style_class = declarations_module.define_class("BorderInlineStartStyle", ruby.class_object())?;

    let border_left_style_class = declarations_module.define_class("BorderLeftStyle", ruby.class_object())?;

    let border_right_style_class = declarations_module.define_class("BorderRightStyle", ruby.class_object())?;

    let border_top_style_class = declarations_module.define_class("BorderTopStyle", ruby.class_object())?;

    let animation_composition_class = declarations_module.define_class("AnimationComposition", ruby.class_object())?;

    let animation_delay_class = declarations_module.define_class("AnimationDelay", ruby.class_object())?;

    let animation_direction_class = declarations_module.define_class("AnimationDirection", ruby.class_object())?;

    let animation_duration_class = declarations_module.define_class("AnimationDuration", ruby.class_object())?;

    let animation_fill_mode_class = declarations_module.define_class("AnimationFillMode", ruby.class_object())?;

    let animation_iteration_count_class = declarations_module.define_class("AnimationIterationCount", ruby.class_object())?;

    let animation_name_class = declarations_module.define_class("AnimationName", ruby.class_object())?;

    let animation_play_state_class = declarations_module.define_class("AnimationPlayState", ruby.class_object())?;

    let animation_timeline_class = declarations_module.define_class("AnimationTimeline", ruby.class_object())?;

    let animation_timing_function_class = declarations_module.define_class("AnimationTimingFunction", ruby.class_object())?;

    let backdrop_filter_class = declarations_module.define_class("BackdropFilter", ruby.class_object())?;

    let background_attachment_class = declarations_module.define_class("BackgroundAttachment", ruby.class_object())?;

    let background_clip_class = declarations_module.define_class("BackgroundClip", ruby.class_object())?;

    let background_image_class = declarations_module.define_class("BackgroundImage", ruby.class_object())?;

    let background_origin_class = declarations_module.define_class("BackgroundOrigin", ruby.class_object())?;

    let background_position_x_class = declarations_module.define_class("BackgroundPositionX", ruby.class_object())?;

    let background_position_y_class = declarations_module.define_class("BackgroundPositionY", ruby.class_object())?;

    let background_repeat_class = declarations_module.define_class("BackgroundRepeat", ruby.class_object())?;

    let background_size_class = declarations_module.define_class("BackgroundSize", ruby.class_object())?;

    let baseline_shift_class = declarations_module.define_class("BaselineShift", ruby.class_object())?;

    let box_shadow_class = declarations_module.define_class("BoxShadow", ruby.class_object())?;

    let caret_color_class = declarations_module.define_class("CaretColor", ruby.class_object())?;

    let clip_path_class = declarations_module.define_class("ClipPath", ruby.class_object())?;

    let color_class = declarations_module.define_class("Color", ruby.class_object())?;

    let color_scheme_class = declarations_module.define_class("ColorScheme", ruby.class_object())?;

    let column_width_class = declarations_module.define_class("ColumnWidth", ruby.class_object())?;

    let container_name_class = declarations_module.define_class("ContainerName", ruby.class_object())?;

    let content_class = declarations_module.define_class("Content", ruby.class_object())?;

    let counter_increment_class = declarations_module.define_class("CounterIncrement", ruby.class_object())?;

    let counter_reset_class = declarations_module.define_class("CounterReset", ruby.class_object())?;

    let cursor_class = declarations_module.define_class("Cursor", ruby.class_object())?;

    let filter_class = declarations_module.define_class("Filter", ruby.class_object())?;

    let font_family_class = declarations_module.define_class("FontFamily", ruby.class_object())?;

    let font_size_class = declarations_module.define_class("FontSize", ruby.class_object())?;

    let font_variation_settings_class = declarations_module.define_class("FontVariationSettings", ruby.class_object())?;

    let grid_template_areas_class = declarations_module.define_class("GridTemplateAreas", ruby.class_object())?;

    let letter_spacing_class = declarations_module.define_class("LetterSpacing", ruby.class_object())?;

    let line_height_class = declarations_module.define_class("LineHeight", ruby.class_object())?;

    let list_style_type_class = declarations_module.define_class("ListStyleType", ruby.class_object())?;

    let mask_image_class = declarations_module.define_class("MaskImage", ruby.class_object())?;

    let offset_path_class = declarations_module.define_class("OffsetPath", ruby.class_object())?;

    let outline_offset_class = declarations_module.define_class("OutlineOffset", ruby.class_object())?;

    let overflow_clip_margin_class = declarations_module.define_class("OverflowClipMargin", ruby.class_object())?;

    let perspective_class = declarations_module.define_class("Perspective", ruby.class_object())?;

    let position_try_fallbacks_class = declarations_module.define_class("PositionTryFallbacks", ruby.class_object())?;

    let quotes_class = declarations_module.define_class("Quotes", ruby.class_object())?;

    let text_indent_class = declarations_module.define_class("TextIndent", ruby.class_object())?;

    let text_shadow_class = declarations_module.define_class("TextShadow", ruby.class_object())?;

    let transform_class = declarations_module.define_class("Transform", ruby.class_object())?;

    let transition_behavior_class = declarations_module.define_class("TransitionBehavior", ruby.class_object())?;

    let transition_delay_class = declarations_module.define_class("TransitionDelay", ruby.class_object())?;

    let transition_duration_class = declarations_module.define_class("TransitionDuration", ruby.class_object())?;

    let transition_property_class = declarations_module.define_class("TransitionProperty", ruby.class_object())?;

    let transition_timing_function_class = declarations_module.define_class("TransitionTimingFunction", ruby.class_object())?;

    let view_transition_class_class = declarations_module.define_class("ViewTransitionClass", ruby.class_object())?;

    let view_transition_name_class = declarations_module.define_class("ViewTransitionName", ruby.class_object())?;

    let will_change_class = declarations_module.define_class("WillChange", ruby.class_object())?;

    let word_spacing_class = declarations_module.define_class("WordSpacing", ruby.class_object())?;

    let xlang_class = declarations_module.define_class("XLang", ruby.class_object())?;

    let grid_template_columns_class = declarations_module.define_class("GridTemplateColumns", ruby.class_object())?;

    let grid_template_rows_class = declarations_module.define_class("GridTemplateRows", ruby.class_object())?;

    let border_image_source_class = declarations_module.define_class("BorderImageSource", ruby.class_object())?;

    let list_style_image_class = declarations_module.define_class("ListStyleImage", ruby.class_object())?;

    let grid_auto_columns_class = declarations_module.define_class("GridAutoColumns", ruby.class_object())?;

    let grid_auto_rows_class = declarations_module.define_class("GridAutoRows", ruby.class_object())?;

    let column_gap_class = declarations_module.define_class("ColumnGap", ruby.class_object())?;

    let row_gap_class = declarations_module.define_class("RowGap", ruby.class_object())?;

    let grid_column_end_class = declarations_module.define_class("GridColumnEnd", ruby.class_object())?;

    let grid_column_start_class = declarations_module.define_class("GridColumnStart", ruby.class_object())?;

    let grid_row_end_class = declarations_module.define_class("GridRowEnd", ruby.class_object())?;

    let grid_row_start_class = declarations_module.define_class("GridRowStart", ruby.class_object())?;

    let max_block_size_class = declarations_module.define_class("MaxBlockSize", ruby.class_object())?;

    let max_height_class = declarations_module.define_class("MaxHeight", ruby.class_object())?;

    let max_inline_size_class = declarations_module.define_class("MaxInlineSize", ruby.class_object())?;

    let max_width_class = declarations_module.define_class("MaxWidth", ruby.class_object())?;

    let bottom_class = declarations_module.define_class("Bottom", ruby.class_object())?;

    let inset_block_end_class = declarations_module.define_class("InsetBlockEnd", ruby.class_object())?;

    let inset_block_start_class = declarations_module.define_class("InsetBlockStart", ruby.class_object())?;

    let inset_inline_end_class = declarations_module.define_class("InsetInlineEnd", ruby.class_object())?;

    let inset_inline_start_class = declarations_module.define_class("InsetInlineStart", ruby.class_object())?;

    let left_class = declarations_module.define_class("Left", ruby.class_object())?;

    let right_class = declarations_module.define_class("Right", ruby.class_object())?;

    let top_class = declarations_module.define_class("Top", ruby.class_object())?;

    let margin_block_end_class = declarations_module.define_class("MarginBlockEnd", ruby.class_object())?;

    let margin_block_start_class = declarations_module.define_class("MarginBlockStart", ruby.class_object())?;

    let margin_bottom_class = declarations_module.define_class("MarginBottom", ruby.class_object())?;

    let margin_inline_end_class = declarations_module.define_class("MarginInlineEnd", ruby.class_object())?;

    let margin_inline_start_class = declarations_module.define_class("MarginInlineStart", ruby.class_object())?;

    let margin_left_class = declarations_module.define_class("MarginLeft", ruby.class_object())?;

    let margin_right_class = declarations_module.define_class("MarginRight", ruby.class_object())?;

    let margin_top_class = declarations_module.define_class("MarginTop", ruby.class_object())?;

    let padding_block_end_class = declarations_module.define_class("PaddingBlockEnd", ruby.class_object())?;

    let padding_block_start_class = declarations_module.define_class("PaddingBlockStart", ruby.class_object())?;

    let padding_bottom_class = declarations_module.define_class("PaddingBottom", ruby.class_object())?;

    let padding_inline_end_class = declarations_module.define_class("PaddingInlineEnd", ruby.class_object())?;

    let padding_inline_start_class = declarations_module.define_class("PaddingInlineStart", ruby.class_object())?;

    let padding_left_class = declarations_module.define_class("PaddingLeft", ruby.class_object())?;

    let padding_right_class = declarations_module.define_class("PaddingRight", ruby.class_object())?;

    let padding_top_class = declarations_module.define_class("PaddingTop", ruby.class_object())?;

    let block_size_class = declarations_module.define_class("BlockSize", ruby.class_object())?;

    let height_class = declarations_module.define_class("Height", ruby.class_object())?;

    let inline_size_class = declarations_module.define_class("InlineSize", ruby.class_object())?;

    let min_block_size_class = declarations_module.define_class("MinBlockSize", ruby.class_object())?;

    let min_height_class = declarations_module.define_class("MinHeight", ruby.class_object())?;

    let min_inline_size_class = declarations_module.define_class("MinInlineSize", ruby.class_object())?;

    let min_width_class = declarations_module.define_class("MinWidth", ruby.class_object())?;

    let width_class = declarations_module.define_class("Width", ruby.class_object())?;

    let border_block_end_width_class = declarations_module.define_class("BorderBlockEndWidth", ruby.class_object())?;

    let border_block_start_width_class = declarations_module.define_class("BorderBlockStartWidth", ruby.class_object())?;

    let border_bottom_width_class = declarations_module.define_class("BorderBottomWidth", ruby.class_object())?;

    let border_inline_end_width_class = declarations_module.define_class("BorderInlineEndWidth", ruby.class_object())?;

    let border_inline_start_width_class = declarations_module.define_class("BorderInlineStartWidth", ruby.class_object())?;

    let border_left_width_class = declarations_module.define_class("BorderLeftWidth", ruby.class_object())?;

    let border_right_width_class = declarations_module.define_class("BorderRightWidth", ruby.class_object())?;

    let border_top_width_class = declarations_module.define_class("BorderTopWidth", ruby.class_object())?;

    let outline_width_class = declarations_module.define_class("OutlineWidth", ruby.class_object())?;

    let background_color_class = declarations_module.define_class("BackgroundColor", ruby.class_object())?;

    let border_block_end_color_class = declarations_module.define_class("BorderBlockEndColor", ruby.class_object())?;

    let border_block_start_color_class = declarations_module.define_class("BorderBlockStartColor", ruby.class_object())?;

    let border_bottom_color_class = declarations_module.define_class("BorderBottomColor", ruby.class_object())?;

    let border_inline_end_color_class = declarations_module.define_class("BorderInlineEndColor", ruby.class_object())?;

    let border_inline_start_color_class = declarations_module.define_class("BorderInlineStartColor", ruby.class_object())?;

    let border_left_color_class = declarations_module.define_class("BorderLeftColor", ruby.class_object())?;

    let border_right_color_class = declarations_module.define_class("BorderRightColor", ruby.class_object())?;

    let border_top_color_class = declarations_module.define_class("BorderTopColor", ruby.class_object())?;

    let outline_color_class = declarations_module.define_class("OutlineColor", ruby.class_object())?;

    let text_decoration_color_class = declarations_module.define_class("TextDecorationColor", ruby.class_object())?;

    let csswide_keyword_class = declarations_module.define_class("CSSWideKeyword", ruby.class_object())?;

    let with_variables_class = declarations_module.define_class("WithVariables", ruby.class_object())?;

    let custom_class = declarations_module.define_class("Custom", ruby.class_object())?;

    Ok(())
  }
}
