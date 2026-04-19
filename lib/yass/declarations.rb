# frozen_string_literal: true

module Yass
  module Declarations
    class AlignContent
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_align_content(self)
      end

      def kind
        :align_content
      end
    end

    class AlignItems
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_align_items(self)
      end

      def kind
        :align_items
      end
    end

    class AlignSelf
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_align_self(self)
      end

      def kind
        :align_self
      end
    end

    class AlignmentBaseline
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_alignment_baseline(self)
      end

      def kind
        :alignment_baseline
      end
    end

    class Angle
      RUBY_METHODS = %i(calc? degrees kind unit).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_angle(self)
      end

      def kind
        :angle
      end
    end

    module Animation
      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_auto(self)
        end

        def kind
          :auto
        end
      end

      class Inset
        RUBY_METHODS = %i(end kind start).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_inset(self)
        end

        def kind
          :inset
        end
      end

      class LengthAuto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_length_auto(self)
        end

        def kind
          :length_auto
        end
      end

      class RangeValue
        RUBY_METHODS = %i(kind length_percentage).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_range_value(self)
        end

        def kind
          :range_value
        end
      end

      class ScrollAxis
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_scroll_axis(self)
        end

        def kind
          :scroll_axis
        end
      end

      class ScrollFunction
        RUBY_METHODS = %i(kind scroll_axis scroller).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_scroll_function(self)
        end

        def kind
          :scroll_function
        end
      end

      class Scroller
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_scroller(self)
        end

        def kind
          :scroller
        end
      end

      class TimelineName
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_timeline_name(self)
        end

        def kind
          :timeline_name
        end
      end

      module TimingFunction
        class CubicBezier
          RUBY_METHODS = %i(kind x1 x2 y1 y2).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_animation_timing_function_cubic_bezier(self)
          end

          def kind
            :cubic_bezier
          end
        end

        class Keyword
          RUBY_METHODS = %i(kind value).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_animation_timing_function_keyword(self)
          end

          def kind
            :keyword
          end
        end

        class PiecewiseLinearFunction
          RUBY_METHODS = %i(kind values).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_animation_timing_function_piecewise_linear_function(self)
          end

          def kind
            :piecewise_linear_function
          end
        end

        class PiecewiseLinearFunctionEntry
          RUBY_METHODS = %i(kind x y).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_animation_timing_function_piecewise_linear_function_entry(self)
          end

          def kind
            :piecewise_linear_function_entry
          end
        end

        class Steps
          RUBY_METHODS = %i(count kind position).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_animation_timing_function_steps(self)
          end

          def kind
            :steps
          end
        end
      end

      class ViewFunction
        RUBY_METHODS = %i(inset kind scroll_axis).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_animation_view_function(self)
        end

        def kind
          :view_function
        end
      end
    end

    class AnimationComposition
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_composition(self)
      end

      def kind
        :animation_composition
      end
    end

    class AnimationDelay
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_delay(self)
      end

      def kind
        :animation_delay
      end
    end

    class AnimationDirection
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_direction(self)
      end

      def kind
        :animation_direction
      end
    end

    class AnimationDuration
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_duration(self)
      end

      def kind
        :animation_duration
      end
    end

    class AnimationDurationValue
      RUBY_METHODS = %i(kind time).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_duration_value(self)
      end

      def kind
        :animation_duration_value
      end
    end

    class AnimationFillMode
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_fill_mode(self)
      end

      def kind
        :animation_fill_mode
      end
    end

    class AnimationIterationCount
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_iteration_count(self)
      end

      def kind
        :animation_iteration_count
      end
    end

    class AnimationName
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_name(self)
      end

      def kind
        :animation_name
      end
    end

    class AnimationPlayState
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_play_state(self)
      end

      def kind
        :animation_play_state
      end
    end

    class AnimationRangeEnd
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_range_end(self)
      end

      def kind
        :animation_range_end
      end
    end

    class AnimationRangeStart
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_range_start(self)
      end

      def kind
        :animation_range_start
      end
    end

    class AnimationTimeline
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_timeline(self)
      end

      def kind
        :animation_timeline
      end
    end

    class AnimationTimingFunction
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_animation_timing_function(self)
      end

      def kind
        :animation_timing_function
      end
    end

    class AspectRatio
      RUBY_METHODS = %i(auto? denominator kind numerator ratio?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_aspect_ratio(self)
      end

      def kind
        :aspect_ratio
      end
    end

    class BackdropFilter
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_backdrop_filter(self)
      end

      def kind
        :backdrop_filter
      end
    end

    class BackfaceVisibility
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_backface_visibility(self)
      end

      def kind
        :backface_visibility
      end
    end

    class BackgroundAttachment
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_attachment(self)
      end

      def kind
        :background_attachment
      end
    end

    class BackgroundBlendMode
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_blend_mode(self)
      end

      def kind
        :background_blend_mode
      end
    end

    class BackgroundClip
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_clip(self)
      end

      def kind
        :background_clip
      end
    end

    class BackgroundColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_color(self)
      end

      def kind
        :background_color
      end
    end

    class BackgroundImage
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_image(self)
      end

      def kind
        :background_image
      end
    end

    class BackgroundOrigin
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_origin(self)
      end

      def kind
        :background_origin
      end
    end

    class BackgroundPositionX
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_position_x(self)
      end

      def kind
        :background_position_x
      end
    end

    class BackgroundPositionY
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_position_y(self)
      end

      def kind
        :background_position_y
      end
    end

    class BackgroundRepeat
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_repeat(self)
      end

      def kind
        :background_repeat
      end
    end

    class BackgroundRepeatValue
      RUBY_METHODS = %i(horizontal kind vertical).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_repeat_value(self)
      end

      def kind
        :background_repeat_value
      end
    end

    class BackgroundSize
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_background_size(self)
      end

      def kind
        :background_size
      end

      class ExplicitSize
        RUBY_METHODS = %i(height kind width).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_background_size_explicit_size(self)
        end

        def kind
          :explicit_size
        end
      end

      class Cover
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_background_size_cover(self)
        end

        def kind
          :cover
        end
      end

      class Contain
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_background_size_contain(self)
        end

        def kind
          :contain
        end
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_background_size_auto(self)
        end

        def kind
          :auto
        end
      end
    end

    class BaselineShift
      class Keyword
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_baseline_shift_keyword(self)
        end

        def kind
          :keyword
        end
      end

      class Length
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_baseline_shift_length(self)
        end

        def kind
          :length
        end
      end
    end

    class BaselineSource
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_baseline_source(self)
      end

      def kind
        :baseline_source
      end
    end

    class BlockSize
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_block_size(self)
      end

      def kind
        :block_size
      end
    end

    class BorderBlockEndColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_block_end_color(self)
      end

      def kind
        :border_block_end_color
      end
    end

    class BorderBlockEndStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_block_end_style(self)
      end

      def kind
        :border_block_end_style
      end
    end

    class BorderBlockEndWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_block_end_width(self)
      end

      def kind
        :border_block_end_width
      end
    end

    class BorderBlockStartColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_block_start_color(self)
      end

      def kind
        :border_block_start_color
      end
    end

    class BorderBlockStartStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_block_start_style(self)
      end

      def kind
        :border_block_start_style
      end
    end

    class BorderBlockStartWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_block_start_width(self)
      end

      def kind
        :border_block_start_width
      end
    end

    class BorderBottomColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_bottom_color(self)
      end

      def kind
        :border_bottom_color
      end
    end

    class BorderBottomLeftRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_bottom_left_radius(self)
      end

      def kind
        :border_bottom_left_radius
      end
    end

    class BorderBottomRightRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_bottom_right_radius(self)
      end

      def kind
        :border_bottom_right_radius
      end
    end

    class BorderBottomStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_bottom_style(self)
      end

      def kind
        :border_bottom_style
      end
    end

    class BorderBottomWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_bottom_width(self)
      end

      def kind
        :border_bottom_width
      end
    end

    class BorderCollapse
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_collapse(self)
      end

      def kind
        :border_collapse
      end
    end

    class BorderEndEndRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_end_end_radius(self)
      end

      def kind
        :border_end_end_radius
      end
    end

    class BorderEndStartRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_end_start_radius(self)
      end

      def kind
        :border_end_start_radius
      end
    end

    class BorderImageOutset
      RUBY_METHODS = %i(bottom kind left right top).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_image_outset(self)
      end

      def kind
        :border_image_outset
      end
    end

    class BorderImageRepeat
      RUBY_METHODS = %i(horizontal kind vertical).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_image_repeat(self)
      end

      def kind
        :border_image_repeat
      end
    end

    class BorderImageSlice
      RUBY_METHODS = %i(bottom fill? kind left right top).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_image_slice(self)
      end

      def kind
        :border_image_slice
      end
    end

    class BorderImageSource
      RUBY_METHODS = %i(image kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_image_source(self)
      end

      def kind
        :border_image_source
      end
    end

    class BorderImageWidth
      RUBY_METHODS = %i(bottom kind left right top).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_image_width(self)
      end

      def kind
        :border_image_width
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_border_image_width_auto(self)
        end

        def kind
          :auto
        end
      end
    end

    class BorderInlineEndColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_inline_end_color(self)
      end

      def kind
        :border_inline_end_color
      end
    end

    class BorderInlineEndStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_inline_end_style(self)
      end

      def kind
        :border_inline_end_style
      end
    end

    class BorderInlineEndWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_inline_end_width(self)
      end

      def kind
        :border_inline_end_width
      end
    end

    class BorderInlineStartColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_inline_start_color(self)
      end

      def kind
        :border_inline_start_color
      end
    end

    class BorderInlineStartStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_inline_start_style(self)
      end

      def kind
        :border_inline_start_style
      end
    end

    class BorderInlineStartWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_inline_start_width(self)
      end

      def kind
        :border_inline_start_width
      end
    end

    class BorderLeftColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_left_color(self)
      end

      def kind
        :border_left_color
      end
    end

    class BorderLeftStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_left_style(self)
      end

      def kind
        :border_left_style
      end
    end

    class BorderLeftWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_left_width(self)
      end

      def kind
        :border_left_width
      end
    end

    class BorderRightColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_right_color(self)
      end

      def kind
        :border_right_color
      end
    end

    class BorderRightStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_right_style(self)
      end

      def kind
        :border_right_style
      end
    end

    class BorderRightWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_right_width(self)
      end

      def kind
        :border_right_width
      end
    end

    class BorderSpacing
      RUBY_METHODS = %i(horizontal kind vertical).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_spacing(self)
      end

      def kind
        :border_spacing
      end
    end

    class BorderStartEndRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_start_end_radius(self)
      end

      def kind
        :border_start_end_radius
      end
    end

    class BorderStartStartRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_start_start_radius(self)
      end

      def kind
        :border_start_start_radius
      end
    end

    class BorderTopColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_top_color(self)
      end

      def kind
        :border_top_color
      end
    end

    class BorderTopLeftRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_top_left_radius(self)
      end

      def kind
        :border_top_left_radius
      end
    end

    class BorderTopRightRadius
      RUBY_METHODS = %i(height kind width).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_top_right_radius(self)
      end

      def kind
        :border_top_right_radius
      end
    end

    class BorderTopStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_top_style(self)
      end

      def kind
        :border_top_style
      end
    end

    class BorderTopWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_border_top_width(self)
      end

      def kind
        :border_top_width
      end
    end

    class Bottom
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_bottom(self)
      end

      def kind
        :bottom
      end
    end

    class BoxShadow
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_box_shadow(self)
      end

      def kind
        :box_shadow
      end

      class Shadow
        RUBY_METHODS = %i(blur color horizontal inset? kind spread vertical).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_box_shadow_shadow(self)
        end

        def kind
          :shadow
        end
      end
    end

    class BoxSizing
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_box_sizing(self)
      end

      def kind
        :box_sizing
      end
    end

    class Calc
      RUBY_METHODS = %i(clamping_mode kind root).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_calc(self)
      end

      def kind
        :calc
      end

      class Abs
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_abs(self)
        end

        def kind
          :abs
        end
      end

      class AnchorFunction
        RUBY_METHODS = %i(children fallback kind side target_element).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_anchor_function(self)
        end

        def kind
          :anchor_function
        end
      end

      class AnchorSideKeyword
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_anchor_side_keyword(self)
        end

        def kind
          :anchor_side_keyword
        end
      end

      class AnchorSizeFunction
        RUBY_METHODS = %i(children fallback kind size target_element).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_anchor_size_function(self)
        end

        def kind
          :anchor_size_function
        end
      end

      class AnchorSizeKeyword
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_anchor_size_keyword(self)
        end

        def kind
          :anchor_size_keyword
        end
      end

      class Clamp
        RUBY_METHODS = %i(center children kind max min).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_clamp(self)
        end

        def kind
          :clamp
        end
      end

      class Hypot
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_hypot(self)
        end

        def kind
          :hypot
        end
      end

      class Invert
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_invert(self)
        end

        def kind
          :invert
        end
      end

      class MinMax
        RUBY_METHODS = %i(children kind op).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_min_max(self)
        end

        def kind
          :min_max
        end
      end

      class ModRem
        RUBY_METHODS = %i(children dividend divisor kind op).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_mod_rem(self)
        end

        def kind
          :mod_rem
        end
      end

      class Negate
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_negate(self)
        end

        def kind
          :negate
        end
      end

      class Product
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_product(self)
        end

        def kind
          :product
        end
      end

      class Round
        RUBY_METHODS = %i(children kind rounding_strategy step value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_round(self)
        end

        def kind
          :round
        end
      end

      class Sign
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_sign(self)
        end

        def kind
          :sign
        end
      end

      class Sum
        RUBY_METHODS = %i(children kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_calc_sum(self)
        end

        def kind
          :sum
        end
      end
    end

    class CaptionSide
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_caption_side(self)
      end

      def kind
        :caption_side
      end
    end

    class CaretColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_caret_color(self)
      end

      def kind
        :caret_color
      end
    end

    class ChannelKeyword
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_channel_keyword(self)
      end

      def kind
        :channel_keyword
      end
    end

    class Clear
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_clear(self)
      end

      def kind
        :clear
      end
    end

    class Clip
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_clip(self)
      end

      def kind
        :clip
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_auto(self)
        end

        def kind
          :auto
        end
      end

      class Rect
        RUBY_METHODS = %i(bottom kind left right top).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_rect(self)
        end

        def kind
          :rect
        end
      end

      class Length
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_length(self)
        end

        def kind
          :length
        end
      end

      class LengthAuto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_length_auto(self)
        end

        def kind
          :length_auto
        end
      end
    end

    class ClipPath
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_clip_path(self)
      end

      def kind
        :clip_path
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_auto(self)
        end

        def kind
          :auto
        end
      end

      class Circle
        RUBY_METHODS = %i(kind position radius).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_circle(self)
        end

        def kind
          :circle
        end
      end

      class Ellipse
        RUBY_METHODS = %i(kind position x_radius y_radius).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_ellipse(self)
        end

        def kind
          :ellipse
        end
      end

      class Position
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_position(self)
        end

        def kind
          :position
        end
      end

      class PositionAuto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_position_auto(self)
        end

        def kind
          :position_auto
        end
      end

      class ShapeRadiusLength
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_radius_length(self)
        end

        def kind
          :shape_radius_length
        end
      end

      class ShapeRadiusClosestSide
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_radius_closest_side(self)
        end

        def kind
          :shape_radius_closest_side
        end
      end

      class ShapeRadiusFarthestSide
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_radius_farthest_side(self)
        end

        def kind
          :shape_radius_farthest_side
        end
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_none(self)
        end

        def kind
          :none
        end
      end

      class Url
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_url(self)
        end

        def kind
          :url
        end
      end

      class Shape
        RUBY_METHODS = %i(kind reference_box shape).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape(self)
        end

        def kind
          :shape
        end
      end

      class Box
        RUBY_METHODS = %i(kind reference_box).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_box(self)
        end

        def kind
          :box
        end
      end

      class Polygon
        RUBY_METHODS = %i(coordinates fill kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_polygon(self)
        end

        def kind
          :polygon
        end
      end

      class PolygonCoord
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_polygon_coord(self)
        end

        def kind
          :polygon_coord
        end
      end

      class PathOrShape
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_or_shape(self)
        end

        def kind
          :path_or_shape
        end
      end

      class PathFunction
        RUBY_METHODS = %i(commands fill kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_function(self)
        end

        def kind
          :path_function
        end
      end

      class PathCommand
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command(self)
        end

        def kind
          :path_command
        end
      end

      class PathCommandMove
        RUBY_METHODS = %i(kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_move(self)
        end

        def kind
          :path_command_move
        end
      end

      class PathCommandLine
        RUBY_METHODS = %i(kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_line(self)
        end

        def kind
          :path_command_line
        end
      end

      class PathCommandHLine
        RUBY_METHODS = %i(kind x).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_h_line(self)
        end

        def kind
          :path_command_h_line
        end
      end

      class PathCommandVLine
        RUBY_METHODS = %i(kind y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_v_line(self)
        end

        def kind
          :path_command_v_line
        end
      end

      class PathCommandCubicCurve
        RUBY_METHODS = %i(control1 control2 kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_cubic_curve(self)
        end

        def kind
          :path_command_cubic_curve
        end
      end

      class PathCommandQuadCurve
        RUBY_METHODS = %i(control1 kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_quad_curve(self)
        end

        def kind
          :path_command_quad_curve
        end
      end

      class PathCommandSmoothCubic
        RUBY_METHODS = %i(control2 kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_smooth_cubic(self)
        end

        def kind
          :path_command_smooth_cubic
        end
      end

      class PathCommandSmoothQuad
        RUBY_METHODS = %i(kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_smooth_quad(self)
        end

        def kind
          :path_command_smooth_quad
        end
      end

      class PathCommandArc
        RUBY_METHODS = %i(arc_size arc_sweep kind point radii rotate).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_arc(self)
        end

        def kind
          :path_command_arc
        end
      end

      class PathCommandClose
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_close(self)
        end

        def kind
          :path_command_close
        end
      end

      class PathCommandEndPointToPosition
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_end_point_to_position(self)
        end

        def kind
          :path_command_end_point_to_position
        end
      end

      class PathCommandEndPointByCoordinate
        RUBY_METHODS = %i(coord kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_command_end_point_by_coordinate(self)
        end

        def kind
          :path_command_end_point_by_coordinate
        end
      end

      class PathCoordinatePair
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_coordinate_pair(self)
        end

        def kind
          :path_coordinate_pair
        end
      end

      class PathAxisEndPointToPosition
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_axis_end_point_to_position(self)
        end

        def kind
          :path_axis_end_point_to_position
        end
      end

      class PathAxisEndPointByCoordinate
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_axis_end_point_by_coordinate(self)
        end

        def kind
          :path_axis_end_point_by_coordinate
        end
      end

      class PathControlPointAbsolute
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_control_point_absolute(self)
        end

        def kind
          :path_control_point_absolute
        end
      end

      class PathControlPointRelative
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_control_point_relative(self)
        end

        def kind
          :path_control_point_relative
        end
      end

      class PathRelativeControlPoint
        RUBY_METHODS = %i(coord kind reference).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_relative_control_point(self)
        end

        def kind
          :path_relative_control_point
        end
      end

      class PathArcRadii
        RUBY_METHODS = %i(kind rx ry).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_path_arc_radii(self)
        end

        def kind
          :path_arc_radii
        end
      end

      class Rect
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_rect(self)
        end

        def kind
          :rect
        end
      end

      class InsetRect
        RUBY_METHODS = %i(bottom kind left right round top).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_inset_rect(self)
        end

        def kind
          :inset_rect
        end
      end

      class XywhRect
        RUBY_METHODS = %i(height kind round width x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_xywh_rect(self)
        end

        def kind
          :xywh_rect
        end
      end

      class RectFunction
        RUBY_METHODS = %i(bottom kind left right round top).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_rect_function(self)
        end

        def kind
          :rect_function
        end
      end

      class BorderRadius
        RUBY_METHODS = %i(bottom_left bottom_right kind top_left top_right).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_border_radius(self)
        end

        def kind
          :border_radius
        end
      end

      class BorderCornerRadius
        RUBY_METHODS = %i(height kind width).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_border_corner_radius(self)
        end

        def kind
          :border_corner_radius
        end
      end

      class ShapeFunction
        RUBY_METHODS = %i(commands fill kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_function(self)
        end

        def kind
          :shape_function
        end
      end

      class ShapeCommand
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command(self)
        end

        def kind
          :shape_command
        end
      end

      class ShapeCommandMove
        RUBY_METHODS = %i(kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_move(self)
        end

        def kind
          :shape_command_move
        end
      end

      class ShapeCommandLine
        RUBY_METHODS = %i(kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_line(self)
        end

        def kind
          :shape_command_line
        end
      end

      class ShapeCommandHLine
        RUBY_METHODS = %i(kind x).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_h_line(self)
        end

        def kind
          :shape_command_h_line
        end
      end

      class ShapeCommandVLine
        RUBY_METHODS = %i(kind y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_v_line(self)
        end

        def kind
          :shape_command_v_line
        end
      end

      class ShapeCommandCubicCurve
        RUBY_METHODS = %i(control1 control2 kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_cubic_curve(self)
        end

        def kind
          :shape_command_cubic_curve
        end
      end

      class ShapeCommandQuadCurve
        RUBY_METHODS = %i(control1 kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_quad_curve(self)
        end

        def kind
          :shape_command_quad_curve
        end
      end

      class ShapeCommandSmoothCubic
        RUBY_METHODS = %i(control2 kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_smooth_cubic(self)
        end

        def kind
          :shape_command_smooth_cubic
        end
      end

      class ShapeCommandSmoothQuad
        RUBY_METHODS = %i(kind point).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_smooth_quad(self)
        end

        def kind
          :shape_command_smooth_quad
        end
      end

      class ShapeCommandArc
        RUBY_METHODS = %i(arc_size arc_sweep kind point radii rotate).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_arc(self)
        end

        def kind
          :shape_command_arc
        end
      end

      class ShapeCommandClose
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_shape_command_close(self)
        end

        def kind
          :shape_command_close
        end
      end

      class CommandEndPointToPosition
        RUBY_METHODS = %i(horizontal kind vertical).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_command_end_point_to_position(self)
        end

        def kind
          :command_end_point_to_position
        end
      end

      class CommandEndPointByCoordinate
        RUBY_METHODS = %i(coord kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_command_end_point_by_coordinate(self)
        end

        def kind
          :command_end_point_by_coordinate
        end
      end

      class CoordinatePair
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_coordinate_pair(self)
        end

        def kind
          :coordinate_pair
        end
      end

      class AxisEndPointToPosition
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_axis_end_point_to_position(self)
        end

        def kind
          :axis_end_point_to_position
        end
      end

      class AxisEndPointByCoordinate
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_axis_end_point_by_coordinate(self)
        end

        def kind
          :axis_end_point_by_coordinate
        end
      end

      class ControlPointAbsolute
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_control_point_absolute(self)
        end

        def kind
          :control_point_absolute
        end
      end

      class ControlPointRelative
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_control_point_relative(self)
        end

        def kind
          :control_point_relative
        end
      end

      class RelativeControlPoint
        RUBY_METHODS = %i(coord kind reference).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_relative_control_point(self)
        end

        def kind
          :relative_control_point
        end
      end

      class ArcRadii
        RUBY_METHODS = %i(kind rx ry).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_arc_radii(self)
        end

        def kind
          :arc_radii
        end
      end

      class ElementDependent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_element_dependent(self)
        end

        def kind
          :element_dependent
        end
      end

      class FillBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_fill_box(self)
        end

        def kind
          :fill_box
        end
      end

      class StrokeBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_stroke_box(self)
        end

        def kind
          :stroke_box
        end
      end

      class ViewBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_view_box(self)
        end

        def kind
          :view_box
        end
      end

      class MarginBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_margin_box(self)
        end

        def kind
          :margin_box
        end
      end

      class BorderBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_border_box(self)
        end

        def kind
          :border_box
        end
      end

      class PaddingBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_padding_box(self)
        end

        def kind
          :padding_box
        end
      end

      class ContentBox
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_clip_path_content_box(self)
        end

        def kind
          :content_box
        end
      end
    end

    class Color
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_color(self)
      end

      def kind
        :color
      end

      class Absolute
        RUBY_METHODS = %i(authored color kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_absolute(self)
        end

        def kind
          :absolute
        end
      end

      class AbsoluteColor
        RUBY_METHODS = %i(alpha color_space components kind legacy_syntax? transparent?).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_absolute_color(self)
        end

        def kind
          :absolute_color
        end
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_auto(self)
        end

        def kind
          :auto
        end
      end

      class ColorComponents
        RUBY_METHODS = %i(c0 c1 c2 kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_color_components(self)
        end

        def kind
          :color_components
        end
      end

      class ColorFunction
        RUBY_METHODS = %i(alpha color_space components has_origin_color? kind origin_color).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_color_function(self)
        end

        def kind
          :color_function
        end
      end

      class ColorFunctionComponent
        RUBY_METHODS = %i(calc channel_keyword kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_color_function_component(self)
        end

        def kind
          :color_function_component
        end
      end

      class ColorInterpolationMethod
        RUBY_METHODS = %i(hue kind space).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_color_interpolation_method(self)
        end

        def kind
          :color_interpolation_method
        end
      end

      class ColorMix
        RUBY_METHODS = %i(interpolation items kind normalize_weights? result_in_modern_syntax?).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_color_mix(self)
        end

        def kind
          :color_mix
        end
      end

      class ColorMixItem
        RUBY_METHODS = %i(color kind percentage).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_color_mix_item(self)
        end

        def kind
          :color_mix_item
        end
      end

      class CurrentColor
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_current_color(self)
        end

        def kind
          :current_color
        end
      end

      class LightDark
        RUBY_METHODS = %i(dark kind light).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_light_dark(self)
        end

        def kind
          :light_dark
        end
      end

      class SystemColor
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_color_system_color(self)
        end

        def kind
          :system_color
        end
      end
    end

    class ColorScheme
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_color_scheme(self)
      end

      def kind
        :color_scheme
      end
    end

    class ColumnCount
      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_column_count_auto(self)
        end

        def kind
          :auto
        end
      end

      class Integer
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_column_count_integer(self)
        end

        def kind
          :integer
        end
      end
    end

    class ColumnGap
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_column_gap(self)
      end

      def kind
        :column_gap
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_column_gap_normal(self)
        end

        def kind
          :normal
        end
      end

      class LengthPercentage
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_column_gap_length_percentage(self)
        end

        def kind
          :length_percentage
        end
      end
    end

    class ColumnSpan
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_column_span(self)
      end

      def kind
        :column_span
      end
    end

    class ColumnWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_column_width(self)
      end

      def kind
        :column_width
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_column_width_auto(self)
        end

        def kind
          :auto
        end
      end

      class Length
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_column_width_length(self)
        end

        def kind
          :length
        end
      end
    end

    class Contain
      RUBY_METHODS = %i(block_size? content? inline_size? kind layout? none? paint? size? strict? style? values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_contain(self)
      end

      def kind
        :contain
      end
    end

    class ContainerName
      RUBY_METHODS = %i(kind none? values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_container_name(self)
      end

      def kind
        :container_name
      end
    end

    class ContainerType
      RUBY_METHODS = %i(inline_size? kind normal? scroll_state? size? values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_container_type(self)
      end

      def kind
        :container_type
      end
    end

    class Content
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_content(self)
      end

      def kind
        :content
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_content_normal(self)
        end

        def kind
          :normal
        end
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_content_none(self)
        end

        def kind
          :none
        end
      end

      class Items
        RUBY_METHODS = %i(alt_start items kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_content_items(self)
        end

        def kind
          :items
        end
      end

      module Item
        class String
          RUBY_METHODS = %i(kind value).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_content_item_string(self)
          end

          def kind
            :string
          end
        end

        class Counter
          RUBY_METHODS = %i(kind style).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_content_item_counter(self)
          end

          def kind
            :counter
          end
        end

        class Counters
          RUBY_METHODS = %i(kind separator style).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_content_item_counters(self)
          end

          def kind
            :counters
          end
        end

        class Attr
          RUBY_METHODS = %i(attribute fallback kind namespace_prefix namespace_url).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_content_item_attr(self)
          end

          def kind
            :attr
          end
        end

        class Image
          RUBY_METHODS = %i(image kind).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_content_item_image(self)
          end

          def kind
            :image
          end
        end
      end

      module CounterStyle
        class Symbols
          RUBY_METHODS = %i(kind symbols).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_content_counter_style_symbols(self)
          end

          def kind
            :symbols
          end
        end
      end
    end

    class CounterIncrement
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_counter_increment(self)
      end

      def kind
        :counter_increment
      end

      class Counter
        RUBY_METHODS = %i(kind reversed? value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_counter_increment_counter(self)
        end

        def kind
          :counter
        end
      end
    end

    class CounterReset
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_counter_reset(self)
      end

      def kind
        :counter_reset
      end

      class Counter
        RUBY_METHODS = %i(kind reversed? value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_counter_reset_counter(self)
        end

        def kind
          :counter
        end
      end
    end

    class CSSWideKeyword
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_css_wide_keyword(self)
      end

      def kind
        :css_wide_keyword
      end
    end

    class Cursor
      RUBY_METHODS = %i(images keyword kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_cursor(self)
      end

      def kind
        :cursor
      end

      class Image
        RUBY_METHODS = %i(hotspot? hotspot_x hotspot_y image kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_cursor_image(self)
        end

        def kind
          :image
        end
      end
    end

    class Custom
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_custom(self)
      end

      def kind
        :custom
      end

      class Unparsed
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_custom_unparsed(self)
        end

        def kind
          :unparsed
        end
      end

      class Parsed
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_custom_parsed(self)
        end

        def kind
          :parsed
        end
      end

      class CSSWideKeyword
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_custom_css_wide_keyword(self)
        end

        def kind
          :css_wide_keyword
        end
      end
    end

    class Direction
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_direction(self)
      end

      def kind
        :direction
      end
    end

    class Display
      RUBY_METHODS = %i(contents? inline_flow? inside item_container? kind line_participant? list_item? none? outside ruby_level_container? ruby_type?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_display(self)
      end

      def kind
        :display
      end
    end

    class EmptyCells
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_empty_cells(self)
      end

      def kind
        :empty_cells
      end
    end

    class Filter
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_filter(self)
      end

      def kind
        :filter
      end

      class Blur
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_blur(self)
        end

        def kind
          :blur
        end
      end

      class Brightness
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_brightness(self)
        end

        def kind
          :brightness
        end
      end

      class Contrast
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_contrast(self)
        end

        def kind
          :contrast
        end
      end

      class DropShadow
        RUBY_METHODS = %i(blur color horizontal kind vertical).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_drop_shadow(self)
        end

        def kind
          :drop_shadow
        end
      end

      class Grayscale
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_grayscale(self)
        end

        def kind
          :grayscale
        end
      end

      class HueRotate
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_hue_rotate(self)
        end

        def kind
          :hue_rotate
        end
      end

      class Invert
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_invert(self)
        end

        def kind
          :invert
        end
      end

      class Opacity
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_opacity(self)
        end

        def kind
          :opacity
        end
      end

      class Saturate
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_saturate(self)
        end

        def kind
          :saturate
        end
      end

      class Sepia
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_filter_sepia(self)
        end

        def kind
          :sepia
        end
      end
    end

    class FlexBasis
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_flex_basis(self)
      end

      def kind
        :flex_basis
      end

      class Content
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_flex_basis_content(self)
        end

        def kind
          :content
        end
      end

      class Size
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_flex_basis_size(self)
        end

        def kind
          :size
        end
      end
    end

    class FlexDirection
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_flex_direction(self)
      end

      def kind
        :flex_direction
      end
    end

    class FlexGrow
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_flex_grow(self)
      end

      def kind
        :flex_grow
      end
    end

    class FlexShrink
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_flex_shrink(self)
      end

      def kind
        :flex_shrink
      end
    end

    class FlexWrap
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_flex_wrap(self)
      end

      def kind
        :flex_wrap
      end
    end

    class Float
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_float(self)
      end

      def kind
        :float
      end
    end

    class FontFamily
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_family(self)
      end

      def kind
        :font_family
      end
    end

    class FontLanguageOverride
      RUBY_METHODS = %i(kind normal? value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_language_override(self)
      end

      def kind
        :font_language_override
      end
    end

    class FontOpticalSizing
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_optical_sizing(self)
      end

      def kind
        :font_optical_sizing
      end
    end

    class FontSize
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_size(self)
      end

      def kind
        :font_size
      end

      class Length
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_size_length(self)
        end

        def kind
          :length
        end
      end

      class Keyword
        RUBY_METHODS = %i(factor keyword kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_size_keyword(self)
        end

        def kind
          :keyword
        end
      end

      class Smaller
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_size_smaller(self)
        end

        def kind
          :smaller
        end
      end

      class Larger
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_size_larger(self)
        end

        def kind
          :larger
        end
      end

      class System
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_size_system(self)
        end

        def kind
          :system
        end
      end
    end

    class FontStretch
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_stretch(self)
      end

      def kind
        :font_stretch
      end

      class Stretch
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_stretch_stretch(self)
        end

        def kind
          :stretch
        end
      end

      class Keyword
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_stretch_keyword(self)
        end

        def kind
          :keyword
        end
      end

      class System
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_stretch_system(self)
        end

        def kind
          :system
        end
      end
    end

    class FontStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_style(self)
      end

      def kind
        :font_style
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_style_normal(self)
        end

        def kind
          :normal
        end
      end

      class Italic
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_style_italic(self)
        end

        def kind
          :italic
        end
      end

      class Oblique
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_style_oblique(self)
        end

        def kind
          :oblique
        end
      end

      class System
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_style_system(self)
        end

        def kind
          :system
        end
      end
    end

    class FontSynthesisWeight
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_synthesis_weight(self)
      end

      def kind
        :font_synthesis_weight
      end
    end

    class FontVariantCaps
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_variant_caps(self)
      end

      def kind
        :font_variant_caps
      end
    end

    class FontVariationSettings
      RUBY_METHODS = %i(kind normal? values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_variation_settings(self)
      end

      def kind
        :font_variation_settings
      end

      class Setting
        RUBY_METHODS = %i(kind tag value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_variation_settings_setting(self)
        end

        def kind
          :setting
        end
      end
    end

    class FontWeight
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_font_weight(self)
      end

      def kind
        :font_weight
      end

      class Absolute
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_weight_absolute(self)
        end

        def kind
          :absolute
        end
      end

      class Bolder
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_weight_bolder(self)
        end

        def kind
          :bolder
        end
      end

      class Lighter
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_weight_lighter(self)
        end

        def kind
          :lighter
        end
      end

      class System
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_font_weight_system(self)
        end

        def kind
          :system
        end
      end
    end

    class GridAutoColumns
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_auto_columns(self)
      end

      def kind
        :grid_auto_columns
      end
    end

    class GridAutoFlow
      RUBY_METHODS = %i(axis dense? kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_auto_flow(self)
      end

      def kind
        :grid_auto_flow
      end
    end

    class GridAutoRows
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_auto_rows(self)
      end

      def kind
        :grid_auto_rows
      end
    end

    class GridColumnEnd
      RUBY_METHODS = %i(auto? ident ident_only? kind line_num span?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_column_end(self)
      end

      def kind
        :grid_column_end
      end
    end

    class GridColumnStart
      RUBY_METHODS = %i(auto? ident ident_only? kind line_num span?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_column_start(self)
      end

      def kind
        :grid_column_start
      end
    end

    class GridRowEnd
      RUBY_METHODS = %i(auto? ident ident_only? kind line_num span?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_row_end(self)
      end

      def kind
        :grid_row_end
      end
    end

    class GridRowStart
      RUBY_METHODS = %i(auto? ident ident_only? kind line_num span?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_row_start(self)
      end

      def kind
        :grid_row_start
      end
    end

    module GridTemplate
      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_none(self)
        end

        def kind
          :none
        end
      end

      class Masonry
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_masonry(self)
        end

        def kind
          :masonry
        end
      end

      class TrackList
        RUBY_METHODS = %i(auto_repeat? auto_repeat_index explicit? kind line_names values).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_track_list(self)
        end

        def kind
          :track_list
        end
      end

      class TrackListValue
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_track_list_value(self)
        end

        def kind
          :track_list_value
        end

        class TrackRepeat
          RUBY_METHODS = %i(count kind line_names track_sizes).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_grid_template_track_list_value_track_repeat(self)
          end

          def kind
            :track_repeat
          end
        end
      end

      module RepeatCount
        class Number
          RUBY_METHODS = %i(kind value).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_grid_template_repeat_count_number(self)
          end

          def kind
            :number
          end
        end

        class AutoFill
          RUBY_METHODS = %i(kind).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_grid_template_repeat_count_auto_fill(self)
          end

          def kind
            :auto_fill
          end
        end

        class AutoFit
          RUBY_METHODS = %i(kind).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_grid_template_repeat_count_auto_fit(self)
          end

          def kind
            :auto_fit
          end
        end
      end

      class Subgrid
        RUBY_METHODS = %i(expanded_line_names_length kind line_names).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_subgrid(self)
        end

        def kind
          :subgrid
        end
      end

      class LineNameListValue
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_line_name_list_value(self)
        end

        def kind
          :line_name_list_value
        end
      end

      class LineNames
        RUBY_METHODS = %i(kind names).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_line_names(self)
        end

        def kind
          :line_names
        end
      end

      class LineNameRepeat
        RUBY_METHODS = %i(count kind line_names).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_line_name_repeat(self)
        end

        def kind
          :line_name_repeat
        end
      end
    end

    class GridTemplateAreas
      RUBY_METHODS = %i(areas kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_template_areas(self)
      end

      def kind
        :grid_template_areas
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_areas_none(self)
        end

        def kind
          :none
        end
      end

      class AreaList
        RUBY_METHODS = %i(areas kind strings width).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_areas_area_list(self)
        end

        def kind
          :area_list
        end
      end

      class Area
        RUBY_METHODS = %i(column_end column_start kind row_end row_start).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_grid_template_areas_area(self)
        end

        def kind
          :area
        end
      end
    end

    class GridTemplateColumns
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_template_columns(self)
      end

      def kind
        :grid_template_columns
      end
    end

    class GridTemplateRows
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_grid_template_rows(self)
      end

      def kind
        :grid_template_rows
      end
    end

    class Height
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_height(self)
      end

      def kind
        :height
      end
    end

    module Image
      class CenterHorizontalPositionComponent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_center_horizontal_position_component(self)
        end

        def kind
          :center_horizontal_position_component
        end
      end

      class LengthHorizontalPositionComponent
        RUBY_METHODS = %i(kind length).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_length_horizontal_position_component(self)
        end

        def kind
          :length_horizontal_position_component
        end
      end

      class SideHorizontalPositionComponent
        RUBY_METHODS = %i(kind offset side).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_side_horizontal_position_component(self)
        end

        def kind
          :side_horizontal_position_component
        end
      end

      class AngleOrPercentage
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_angle_or_percentage(self)
        end

        def kind
          :angle_or_percentage
        end
      end

      class RadiusCircle
        RUBY_METHODS = %i(kind length).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_radius_circle(self)
        end

        def kind
          :radius_circle
        end
      end

      class ExtentCircle
        RUBY_METHODS = %i(extent kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_extent_circle(self)
        end

        def kind
          :extent_circle
        end
      end

      class ConicGradient
        RUBY_METHODS = %i(angle color_interpolation_method items kind position repeating?).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_conic_gradient(self)
        end

        def kind
          :conic_gradient
        end
      end

      class CrossFade
        RUBY_METHODS = %i(elements kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_cross_fade(self)
        end

        def kind
          :cross_fade
        end
      end

      class CrossFadeElement
        RUBY_METHODS = %i(image kind percent).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_cross_fade_element(self)
        end

        def kind
          :cross_fade_element
        end
      end

      class CrossFadeImage
        RUBY_METHODS = %i(image kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_cross_fade_image(self)
        end

        def kind
          :cross_fade_image
        end
      end

      class CrossFadeColor
        RUBY_METHODS = %i(image kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_cross_fade_color(self)
        end

        def kind
          :cross_fade_color
        end
      end

      class RadiiEllipse
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_radii_ellipse(self)
        end

        def kind
          :radii_ellipse
        end
      end

      class ExtentEllipse
        RUBY_METHODS = %i(extent kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_extent_ellipse(self)
        end

        def kind
          :extent_ellipse
        end
      end

      class EndingShape
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_ending_shape(self)
        end

        def kind
          :ending_shape
        end
      end

      module Gradient
        class SimpleColorStopAngle
          RUBY_METHODS = %i(color kind position).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_image_gradient_simple_color_stop_angle(self)
          end

          def kind
            :simple_color_stop_angle
          end
        end

        class ComplexColorStopAngle
          RUBY_METHODS = %i(color kind position).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_image_gradient_complex_color_stop_angle(self)
          end

          def kind
            :complex_color_stop_angle
          end
        end

        class InterpolationHintAngle
          RUBY_METHODS = %i(color kind position).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_image_gradient_interpolation_hint_angle(self)
          end

          def kind
            :interpolation_hint_angle
          end
        end

        class SimpleColorStopLength
          RUBY_METHODS = %i(color kind).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_image_gradient_simple_color_stop_length(self)
          end

          def kind
            :simple_color_stop_length
          end
        end

        class ComplexColorStopLength
          RUBY_METHODS = %i(color kind position).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_image_gradient_complex_color_stop_length(self)
          end

          def kind
            :complex_color_stop_length
          end
        end

        class InterpolationHintLength
          RUBY_METHODS = %i(kind position).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_image_gradient_interpolation_hint_length(self)
          end

          def kind
            :interpolation_hint_length
          end
        end
      end

      class ImageSet
        RUBY_METHODS = %i(items kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_image_set(self)
        end

        def kind
          :image_set
        end
      end

      class ImageSetItem
        RUBY_METHODS = %i(has_mime_type? image kind mime_type resolution).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_image_set_item(self)
        end

        def kind
          :image_set_item
        end
      end

      class LightDark
        RUBY_METHODS = %i(dark kind light).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_light_dark(self)
        end

        def kind
          :light_dark
        end
      end

      class AngleLineDirection
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_angle_line_direction(self)
        end

        def kind
          :angle_line_direction
        end
      end

      class HorizontalLineDirection
        RUBY_METHODS = %i(direction kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_horizontal_line_direction(self)
        end

        def kind
          :horizontal_line_direction
        end
      end

      class VerticalLineDirection
        RUBY_METHODS = %i(direction kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_vertical_line_direction(self)
        end

        def kind
          :vertical_line_direction
        end
      end

      class CornerLineDirection
        RUBY_METHODS = %i(horizontal_direction kind vertical_direction).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_corner_line_direction(self)
        end

        def kind
          :corner_line_direction
        end
      end

      class LinearGradient
        RUBY_METHODS = %i(color_interpolation_method compat_mode direction items kind repeating?).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_linear_gradient(self)
        end

        def kind
          :linear_gradient
        end
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_none(self)
        end

        def kind
          :none
        end
      end

      class Position
        RUBY_METHODS = %i(horizontal_position kind vertical_position).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_position(self)
        end

        def kind
          :position
        end
      end

      class RadialGradient
        RUBY_METHODS = %i(color_interpolation_method compat_mode items kind position repeating? shape).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_radial_gradient(self)
        end

        def kind
          :radial_gradient
        end
      end

      class Url
        RUBY_METHODS = %i(invalid? kind original resolved).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_url(self)
        end

        def kind
          :url
        end
      end

      class CenterVerticalPositionComponent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_center_vertical_position_component(self)
        end

        def kind
          :center_vertical_position_component
        end
      end

      class LengthVerticalPositionComponent
        RUBY_METHODS = %i(kind length).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_length_vertical_position_component(self)
        end

        def kind
          :length_vertical_position_component
        end
      end

      class SideVerticalPositionComponent
        RUBY_METHODS = %i(kind offset position).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_image_side_vertical_position_component(self)
        end

        def kind
          :side_vertical_position_component
        end
      end
    end

    class ImageRendering
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_image_rendering(self)
      end

      def kind
        :image_rendering
      end
    end

    class InlineSize
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_inline_size(self)
      end

      def kind
        :inline_size
      end
    end

    class Inset
      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_inset_auto(self)
        end

        def kind
          :auto
        end
      end

      class LengthPercentage
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_inset_length_percentage(self)
        end

        def kind
          :length_percentage
        end
      end

      class AnchorContainingCalcFunction
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_inset_anchor_containing_calc_function(self)
        end

        def kind
          :anchor_containing_calc_function
        end
      end

      class AnchorFunction
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_inset_anchor_function(self)
        end

        def kind
          :anchor_function
        end
      end

      class AnchorSizeFunction
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_inset_anchor_size_function(self)
        end

        def kind
          :anchor_size_function
        end
      end
    end

    class InsetBlockEnd
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_inset_block_end(self)
      end

      def kind
        :inset_block_end
      end
    end

    class InsetBlockStart
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_inset_block_start(self)
      end

      def kind
        :inset_block_start
      end
    end

    class InsetInlineEnd
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_inset_inline_end(self)
      end

      def kind
        :inset_inline_end
      end
    end

    class InsetInlineStart
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_inset_inline_start(self)
      end

      def kind
        :inset_inline_start
      end
    end

    class Isolation
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_isolation(self)
      end

      def kind
        :isolation
      end
    end

    class JustifyContent
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_justify_content(self)
      end

      def kind
        :justify_content
      end
    end

    class JustifyItems
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_justify_items(self)
      end

      def kind
        :justify_items
      end
    end

    class JustifySelf
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_justify_self(self)
      end

      def kind
        :justify_self
      end
    end

    class Left
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_left(self)
      end

      def kind
        :left
      end
    end

    module Length
      class Absolute
        RUBY_METHODS = %i(kind unit value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_length_absolute(self)
        end

        def kind
          :absolute
        end
      end

      class FontRelative
        RUBY_METHODS = %i(kind unit value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_length_font_relative(self)
        end

        def kind
          :font_relative
        end
      end

      class ViewportPercentage
        RUBY_METHODS = %i(kind unit value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_length_viewport_percentage(self)
        end

        def kind
          :viewport_percentage
        end
      end

      class ContainerRelative
        RUBY_METHODS = %i(kind unit value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_length_container_relative(self)
        end

        def kind
          :container_relative
        end
      end

      class CharacterWidth
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_length_character_width(self)
        end

        def kind
          :character_width
        end
      end
    end

    class LetterSpacing
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_letter_spacing(self)
      end

      def kind
        :letter_spacing
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_letter_spacing_normal(self)
        end

        def kind
          :normal
        end
      end
    end

    class LineBreak
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_line_break(self)
      end

      def kind
        :line_break
      end
    end

    class LineHeight
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_line_height(self)
      end

      def kind
        :line_height
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_line_height_normal(self)
        end

        def kind
          :normal
        end
      end
    end

    class ListStyleImage
      RUBY_METHODS = %i(image kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_list_style_image(self)
      end

      def kind
        :list_style_image
      end
    end

    class ListStylePosition
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_list_style_position(self)
      end

      def kind
        :list_style_position
      end
    end

    class ListStyleType
      RUBY_METHODS = %i(bullet? kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_list_style_type(self)
      end

      def kind
        :list_style_type
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_list_style_type_none(self)
        end

        def kind
          :none
        end
      end

      class Name
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_list_style_type_name(self)
        end

        def kind
          :name
        end
      end

      class String
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_list_style_type_string(self)
        end

        def kind
          :string
        end
      end

      class Symbols
        RUBY_METHODS = %i(kind symbols type).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_list_style_type_symbols(self)
        end

        def kind
          :symbols
        end
      end
    end

    module Margin
      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_margin_auto(self)
        end

        def kind
          :auto
        end
      end

      class AnchorContainingCalcFunction
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_margin_anchor_containing_calc_function(self)
        end

        def kind
          :anchor_containing_calc_function
        end
      end

      class AnchorSizeFunction
        RUBY_METHODS = %i(fallback kind size target_element).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_margin_anchor_size_function(self)
        end

        def kind
          :anchor_size_function
        end
      end
    end

    class MarginBlockEnd
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_block_end(self)
      end

      def kind
        :margin_block_end
      end
    end

    class MarginBlockStart
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_block_start(self)
      end

      def kind
        :margin_block_start
      end
    end

    class MarginBottom
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_bottom(self)
      end

      def kind
        :margin_bottom
      end
    end

    class MarginInlineEnd
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_inline_end(self)
      end

      def kind
        :margin_inline_end
      end
    end

    class MarginInlineStart
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_inline_start(self)
      end

      def kind
        :margin_inline_start
      end
    end

    class MarginLeft
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_left(self)
      end

      def kind
        :margin_left
      end
    end

    class MarginRight
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_right(self)
      end

      def kind
        :margin_right
      end
    end

    class MarginTop
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_margin_top(self)
      end

      def kind
        :margin_top
      end
    end

    class MaskImage
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_mask_image(self)
      end

      def kind
        :mask_image
      end
    end

    class MaxBlockSize
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_max_block_size(self)
      end

      def kind
        :max_block_size
      end
    end

    class MaxHeight
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_max_height(self)
      end

      def kind
        :max_height
      end
    end

    class MaxInlineSize
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_max_inline_size(self)
      end

      def kind
        :max_inline_size
      end
    end

    class MaxWidth
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_max_width(self)
      end

      def kind
        :max_width
      end
    end

    class MinBlockSize
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_min_block_size(self)
      end

      def kind
        :min_block_size
      end
    end

    class MinHeight
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_min_height(self)
      end

      def kind
        :min_height
      end
    end

    class MinInlineSize
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_min_inline_size(self)
      end

      def kind
        :min_inline_size
      end
    end

    class MinWidth
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_min_width(self)
      end

      def kind
        :min_width
      end
    end

    class MixBlendMode
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_mix_blend_mode(self)
      end

      def kind
        :mix_blend_mode
      end
    end

    class Number
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_number(self)
      end

      def kind
        :number
      end
    end

    class ObjectFit
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_object_fit(self)
      end

      def kind
        :object_fit
      end
    end

    class ObjectPosition
      RUBY_METHODS = %i(horizontal kind vertical).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_object_position(self)
      end

      def kind
        :object_position
      end
    end

    class OffsetPath
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_offset_path(self)
      end

      def kind
        :offset_path
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_none(self)
        end

        def kind
          :none
        end
      end

      class CoordBox
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_coord_box(self)
        end

        def kind
          :coord_box
        end
      end

      class Function
        RUBY_METHODS = %i(coord_box kind path).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_function(self)
        end

        def kind
          :function
        end
      end

      class Ray
        RUBY_METHODS = %i(angle contain? kind position size).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_ray(self)
        end

        def kind
          :ray
        end
      end

      class Url
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_url(self)
        end

        def kind
          :url
        end
      end

      class PositionAuto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_position_auto(self)
        end

        def kind
          :position_auto
        end
      end

      class Position
        RUBY_METHODS = %i(horizontal kind vertical).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_offset_path_position(self)
        end

        def kind
          :position
        end
      end
    end

    class Opacity
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_opacity(self)
      end

      def kind
        :opacity
      end
    end

    class Order
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_order(self)
      end

      def kind
        :order
      end
    end

    class OutlineColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_outline_color(self)
      end

      def kind
        :outline_color
      end
    end

    class OutlineOffset
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_outline_offset(self)
      end

      def kind
        :outline_offset
      end
    end

    class OutlineStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_outline_style(self)
      end

      def kind
        :outline_style
      end
    end

    class OutlineWidth
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_outline_width(self)
      end

      def kind
        :outline_width
      end
    end

    class OverflowBlock
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_overflow_block(self)
      end

      def kind
        :overflow_block
      end
    end

    class OverflowClipMargin
      RUBY_METHODS = %i(kind offset visual_box).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_overflow_clip_margin(self)
      end

      def kind
        :overflow_clip_margin
      end
    end

    class OverflowInline
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_overflow_inline(self)
      end

      def kind
        :overflow_inline
      end
    end

    class OverflowWrap
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_overflow_wrap(self)
      end

      def kind
        :overflow_wrap
      end
    end

    class OverflowX
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_overflow_x(self)
      end

      def kind
        :overflow_x
      end
    end

    class OverflowY
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_overflow_y(self)
      end

      def kind
        :overflow_y
      end
    end

    class PaddingBlockEnd
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_block_end(self)
      end

      def kind
        :padding_block_end
      end
    end

    class PaddingBlockStart
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_block_start(self)
      end

      def kind
        :padding_block_start
      end
    end

    class PaddingBottom
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_bottom(self)
      end

      def kind
        :padding_bottom
      end
    end

    class PaddingInlineEnd
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_inline_end(self)
      end

      def kind
        :padding_inline_end
      end
    end

    class PaddingInlineStart
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_inline_start(self)
      end

      def kind
        :padding_inline_start
      end
    end

    class PaddingLeft
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_left(self)
      end

      def kind
        :padding_left
      end
    end

    class PaddingRight
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_right(self)
      end

      def kind
        :padding_right
      end
    end

    class PaddingTop
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_padding_top(self)
      end

      def kind
        :padding_top
      end
    end

    class Percentage
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_percentage(self)
      end

      def kind
        :percentage
      end
    end

    class Perspective
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_perspective(self)
      end

      def kind
        :perspective
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_perspective_none(self)
        end

        def kind
          :none
        end
      end

      class Length
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_perspective_length(self)
        end

        def kind
          :length
        end
      end
    end

    class PerspectiveOrigin
      RUBY_METHODS = %i(horizontal kind vertical).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_perspective_origin(self)
      end

      def kind
        :perspective_origin
      end
    end

    class PointerEvents
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_pointer_events(self)
      end

      def kind
        :pointer_events
      end
    end

    class Position
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_position(self)
      end

      def kind
        :position
      end
    end

    class PositionArea
      RUBY_METHODS = %i(first is_none kind second).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_position_area(self)
      end

      def kind
        :position_area
      end
    end

    class PositionTryFallbacks
      RUBY_METHODS = %i(is_none kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_position_try_fallbacks(self)
      end

      def kind
        :position_try_fallbacks
      end

      class IdentAndOrTactic
        RUBY_METHODS = %i(has_ident? has_try_tactics? ident kind try_tactics).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_position_try_fallbacks_ident_and_or_tactic(self)
        end

        def kind
          :ident_and_or_tactic
        end
      end

      class PositionArea
        RUBY_METHODS = %i(first kind second).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_position_try_fallbacks_position_area(self)
        end

        def kind
          :position_area
        end
      end
    end

    class Quotes
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_quotes(self)
      end

      def kind
        :quotes
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_quotes_auto(self)
        end

        def kind
          :auto
        end
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_quotes_none(self)
        end

        def kind
          :none
        end
      end

      class QuoteList
        RUBY_METHODS = %i(kind values).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_quotes_quote_list(self)
        end

        def kind
          :quote_list
        end
      end

      class QuotePair
        RUBY_METHODS = %i(closing kind opening).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_quotes_quote_pair(self)
        end

        def kind
          :quote_pair
        end
      end
    end

    class Resolution
      RUBY_METHODS = %i(dpi kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_resolution(self)
      end

      def kind
        :resolution
      end
    end

    class Right
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_right(self)
      end

      def kind
        :right
      end
    end

    class Rotate
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_rotate(self)
      end

      def kind
        :rotate
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_rotate_none(self)
        end

        def kind
          :none
        end
      end

      class Rotate3D
        RUBY_METHODS = %i(angle kind x y z).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_rotate_rotate3_d(self)
        end

        def kind
          :rotate3_d
        end
      end
    end

    class RowGap
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_row_gap(self)
      end

      def kind
        :row_gap
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_row_gap_normal(self)
        end

        def kind
          :normal
        end
      end

      class LengthPercentage
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_row_gap_length_percentage(self)
        end

        def kind
          :length_percentage
        end
      end
    end

    class Scale
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_scale(self)
      end

      def kind
        :scale
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_scale_none(self)
        end

        def kind
          :none
        end
      end

      class Coords
        RUBY_METHODS = %i(kind x y z).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_scale_coords(self)
        end

        def kind
          :coords
        end
      end
    end

    class ServoOverflowClipBox
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_servo_overflow_clip_box(self)
      end

      def kind
        :servo_overflow_clip_box
      end
    end

    class ServoTopLayer
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_servo_top_layer(self)
      end

      def kind
        :servo_top_layer
      end
    end

    module Size
      class AnchorMaxSizeFunction
        RUBY_METHODS = %i(fallback kind size target_element).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_anchor_max_size_function(self)
        end

        def kind
          :anchor_max_size_function
        end
      end

      class AnchorSizeFunction
        RUBY_METHODS = %i(fallback kind size target_element).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_anchor_size_function(self)
        end

        def kind
          :anchor_size_function
        end
      end

      class LengthPercentage
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_length_percentage(self)
        end

        def kind
          :length_percentage
        end
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_none(self)
        end

        def kind
          :none
        end
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_auto(self)
        end

        def kind
          :auto
        end
      end

      class MaxContent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_max_content(self)
        end

        def kind
          :max_content
        end
      end

      class MinContent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_min_content(self)
        end

        def kind
          :min_content
        end
      end

      class FitContent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_fit_content(self)
        end

        def kind
          :fit_content
        end
      end

      class WebkitFillAvailable
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_webkit_fill_available(self)
        end

        def kind
          :webkit_fill_available
        end
      end

      class Stretch
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_stretch(self)
        end

        def kind
          :stretch
        end
      end

      class FitContentFunction
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_fit_content_function(self)
        end

        def kind
          :fit_content_function
        end
      end

      class AnchorContainingCalcFunction
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_size_anchor_containing_calc_function(self)
        end

        def kind
          :anchor_containing_calc_function
        end
      end
    end

    class TableLayout
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_table_layout(self)
      end

      def kind
        :table_layout
      end
    end

    class TextAlign
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_align(self)
      end

      def kind
        :text_align
      end
    end

    class TextAlignLast
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_align_last(self)
      end

      def kind
        :text_align_last
      end
    end

    class TextDecorationColor
      RUBY_METHODS = %i(color kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_decoration_color(self)
      end

      def kind
        :text_decoration_color
      end
    end

    class TextDecorationLine
      RUBY_METHODS = %i(blink? grammar_error? kind line_through? none? overline? spelling_error? underline? values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_decoration_line(self)
      end

      def kind
        :text_decoration_line
      end
    end

    class TextDecorationStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_decoration_style(self)
      end

      def kind
        :text_decoration_style
      end
    end

    class TextIndent
      RUBY_METHODS = %i(each_line? hanging? kind length).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_indent(self)
      end

      def kind
        :text_indent
      end
    end

    class TextJustify
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_justify(self)
      end

      def kind
        :text_justify
      end
    end

    class TextOverflow
      RUBY_METHODS = %i(first kind second sides_are_logical?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_overflow(self)
      end

      def kind
        :text_overflow
      end

      class Clip
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_text_overflow_clip(self)
        end

        def kind
          :clip
        end
      end

      class Ellipsis
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_text_overflow_ellipsis(self)
        end

        def kind
          :ellipsis
        end
      end

      class String
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_text_overflow_string(self)
        end

        def kind
          :string
        end
      end
    end

    class TextRendering
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_rendering(self)
      end

      def kind
        :text_rendering
      end
    end

    class TextShadow
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_shadow(self)
      end

      def kind
        :text_shadow
      end

      class Shadow
        RUBY_METHODS = %i(blur color horizontal kind vertical).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_text_shadow_shadow(self)
        end

        def kind
          :shadow
        end
      end
    end

    class TextTransform
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_transform(self)
      end

      def kind
        :text_transform
      end
    end

    class TextWrapMode
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_text_wrap_mode(self)
      end

      def kind
        :text_wrap_mode
      end
    end

    class Time
      RUBY_METHODS = %i(kind seconds unit).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_time(self)
      end

      def kind
        :time
      end
    end

    class Top
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_top(self)
      end

      def kind
        :top
      end
    end

    class TrackBreadth
      class LengthPercentage
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_breadth_length_percentage(self)
        end

        def kind
          :length_percentage
        end
      end

      class Fr
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_breadth_fr(self)
        end

        def kind
          :fr
        end
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_breadth_auto(self)
        end

        def kind
          :auto
        end
      end

      class MinContent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_breadth_min_content(self)
        end

        def kind
          :min_content
        end
      end

      class MaxContent
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_breadth_max_content(self)
        end

        def kind
          :max_content
        end
      end
    end

    class TrackSize
      class Minmax
        RUBY_METHODS = %i(kind max min).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_size_minmax(self)
        end

        def kind
          :minmax
        end
      end

      class FitContent
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_track_size_fit_content(self)
        end

        def kind
          :fit_content
        end
      end
    end

    class Transform
      RUBY_METHODS = %i(kind operations).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transform(self)
      end

      def kind
        :transform
      end

      class Matrix
        RUBY_METHODS = %i(a b c d e f kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_matrix(self)
        end

        def kind
          :matrix
        end
      end

      class Matrix3D
        RUBY_METHODS = %i(kind m11 m12 m13 m14 m21 m22 m23 m24 m31 m32 m33 m34 m41 m42 m43 m44).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_matrix3_d(self)
        end

        def kind
          :matrix3_d
        end
      end

      class Skew
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_skew(self)
        end

        def kind
          :skew
        end
      end

      class SkewX
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_skew_x(self)
        end

        def kind
          :skew_x
        end
      end

      class SkewY
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_skew_y(self)
        end

        def kind
          :skew_y
        end
      end

      class Translate
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_translate(self)
        end

        def kind
          :translate
        end
      end

      class TranslateX
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_translate_x(self)
        end

        def kind
          :translate_x
        end
      end

      class TranslateY
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_translate_y(self)
        end

        def kind
          :translate_y
        end
      end

      class TranslateZ
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_translate_z(self)
        end

        def kind
          :translate_z
        end
      end

      class Translate3D
        RUBY_METHODS = %i(kind x y z).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_translate3_d(self)
        end

        def kind
          :translate3_d
        end
      end

      class Scale
        RUBY_METHODS = %i(kind x y).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_scale(self)
        end

        def kind
          :scale
        end
      end

      class ScaleX
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_scale_x(self)
        end

        def kind
          :scale_x
        end
      end

      class ScaleY
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_scale_y(self)
        end

        def kind
          :scale_y
        end
      end

      class ScaleZ
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_scale_z(self)
        end

        def kind
          :scale_z
        end
      end

      class Scale3D
        RUBY_METHODS = %i(kind x y z).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_scale3_d(self)
        end

        def kind
          :scale3_d
        end
      end

      class Rotate
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_rotate(self)
        end

        def kind
          :rotate
        end
      end

      class RotateX
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_rotate_x(self)
        end

        def kind
          :rotate_x
        end
      end

      class RotateY
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_rotate_y(self)
        end

        def kind
          :rotate_y
        end
      end

      class RotateZ
        RUBY_METHODS = %i(angle kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_rotate_z(self)
        end

        def kind
          :rotate_z
        end
      end

      class Rotate3D
        RUBY_METHODS = %i(angle kind x y z).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_rotate3_d(self)
        end

        def kind
          :rotate3_d
        end
      end

      class Perspective
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_perspective(self)
        end

        def kind
          :perspective
        end

        class None
          RUBY_METHODS = %i(kind).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_transform_perspective_none(self)
          end

          def kind
            :none
          end
        end

        class Length
          RUBY_METHODS = %i(kind value).freeze

          include ::Yass::Node

          def accept(visitor)
            visitor.visit_declaration_transform_perspective_length(self)
          end

          def kind
            :length
          end
        end
      end

      class InterpolateMatrix
        RUBY_METHODS = %i(from_list kind progress to_list).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_interpolate_matrix(self)
        end

        def kind
          :interpolate_matrix
        end
      end

      class AccumulateMatrix
        RUBY_METHODS = %i(count from_list kind to_list).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_accumulate_matrix(self)
        end

        def kind
          :accumulate_matrix
        end
      end
    end

    class TransformOrigin
      RUBY_METHODS = %i(depth horizontal kind vertical).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transform_origin(self)
      end

      def kind
        :transform_origin
      end

      class Center
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_origin_center(self)
        end

        def kind
          :center
        end
      end

      class SideHorizontalComponent
        RUBY_METHODS = %i(kind side).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_origin_side_horizontal_component(self)
        end

        def kind
          :side_horizontal_component
        end
      end

      class SideVerticalComponent
        RUBY_METHODS = %i(kind side).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transform_origin_side_vertical_component(self)
        end

        def kind
          :side_vertical_component
        end
      end
    end

    class TransformStyle
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transform_style(self)
      end

      def kind
        :transform_style
      end
    end

    class TransitionBehavior
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transition_behavior(self)
      end

      def kind
        :transition_behavior
      end
    end

    class TransitionDelay
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transition_delay(self)
      end

      def kind
        :transition_delay
      end
    end

    class TransitionDuration
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transition_duration(self)
      end

      def kind
        :transition_duration
      end
    end

    class TransitionProperty
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transition_property(self)
      end

      def kind
        :transition_property
      end

      class NonCustom
        RUBY_METHODS = %i(all? kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transition_property_non_custom(self)
        end

        def kind
          :non_custom
        end
      end

      class Custom
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transition_property_custom(self)
        end

        def kind
          :custom
        end
      end

      class Unsupported
        RUBY_METHODS = %i(kind none?).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_transition_property_unsupported(self)
        end

        def kind
          :unsupported
        end
      end
    end

    class TransitionTimingFunction
      RUBY_METHODS = %i(kind values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_transition_timing_function(self)
      end

      def kind
        :transition_timing_function
      end
    end

    class Translate
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_translate(self)
      end

      def kind
        :translate
      end

      class None
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_translate_none(self)
        end

        def kind
          :none
        end
      end

      class Coords
        RUBY_METHODS = %i(kind x y z).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_translate_coords(self)
        end

        def kind
          :coords
        end
      end
    end

    class UnicodeBidi
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_unicode_bidi(self)
      end

      def kind
        :unicode_bidi
      end
    end

    class ViewTransitionClass
      RUBY_METHODS = %i(kind none? values).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_view_transition_class(self)
      end

      def kind
        :view_transition_class
      end
    end

    class ViewTransitionName
      RUBY_METHODS = %i(kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_view_transition_name(self)
      end

      def kind
        :view_transition_name
      end
    end

    class Visibility
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_visibility(self)
      end

      def kind
        :visibility
      end
    end

    class WebkitTextSecurity
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_webkit_text_security(self)
      end

      def kind
        :webkit_text_security
      end
    end

    class WhiteSpaceCollapse
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_white_space_collapse(self)
      end

      def kind
        :white_space_collapse
      end
    end

    class Width
      RUBY_METHODS = %i(kind size).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_width(self)
      end

      def kind
        :width
      end
    end

    class WillChange
      RUBY_METHODS = %i(auto? backdrop_root? contain? fixpos_cb_non_svg? kind opacity? perspective? position? scroll? stacking_context_unconditional? transform? values view_transition_name? z_index?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_will_change(self)
      end

      def kind
        :will_change
      end
    end

    class WithVariables
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_with_variables(self)
      end

      def kind
        :with_variables
      end
    end

    class WordBreak
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_word_break(self)
      end

      def kind
        :word_break
      end
    end

    class WordSpacing
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_word_spacing(self)
      end

      def kind
        :word_spacing
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_word_spacing_normal(self)
        end

        def kind
          :normal
        end
      end
    end

    class WritingMode
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_writing_mode(self)
      end

      def kind
        :writing_mode
      end
    end

    class XLang
      RUBY_METHODS = %i(empty? kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_x_lang(self)
      end

      def kind
        :x_lang
      end
    end

    class ZIndex
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_z_index(self)
      end

      def kind
        :z_index
      end

      class Auto
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_z_index_auto(self)
        end

        def kind
          :auto
        end
      end

      class Integer
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_z_index_integer(self)
        end

        def kind
          :integer
        end
      end
    end

    class Zoom
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_declaration_zoom(self)
      end

      def kind
        :zoom
      end

      class Normal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_zoom_normal(self)
        end

        def kind
          :normal
        end
      end

      class Document
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_zoom_document(self)
        end

        def kind
          :document
        end
      end

      class Value
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_declaration_zoom_value(self)
        end

        def kind
          :value
        end
      end
    end
  end
end
