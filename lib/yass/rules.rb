# frozen_string_literal: true

module Yass
  class UnimplementedRule
    RUBY_METHODS = %i(kind).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_unimplemented_rule(self)
    end

    def kind
      :unimplemented_rule
    end
  end

  class StyleRule
    RUBY_METHODS = %i(declarations kind selectors).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_style_rule(self)
    end

    def kind
      :style_rule
    end
  end

  class StyleQuery
    class Not
      RUBY_METHODS = %i(kind style_query).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_not(self)
      end

      def kind
        :not
      end
    end

    class Operation
      RUBY_METHODS = %i(kind operator style_query).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_operation(self)
      end

      def kind
        :operation
      end
    end

    class InParens
      RUBY_METHODS = %i(kind style_query).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_in_parens(self)
      end

      def kind
        :in_parens
      end
    end

    class StyleFeature
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_style_feature(self)
      end

      def kind
        :style_feature
      end
    end

    class GenerallyEnclosed
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_generally_enclosed(self)
      end

      def kind
        :generally_enclosed
      end
    end
  end

  class MediaRule
    RUBY_METHODS = %i(kind media_queries rules).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_media_rule(self)
    end

    def kind
      :media_rule
    end
  end

  class MediaQuery
    RUBY_METHODS = %i(kind media_type qualifier query_condition).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_media_query(self)
    end

    def kind
      :media_query
    end

    module QueryCondition
      class FeatureExpression
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_feature_expression(self)
        end

        def kind
          :feature_expression
        end
      end

      class Custom
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_custom(self)
        end

        def kind
          :custom
        end
      end

      class Operation
        RUBY_METHODS = %i(kind operator query_conditions).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_operation(self)
        end

        def kind
          :operation
        end
      end

      class InParens
        RUBY_METHODS = %i(kind query_condition).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_in_parens(self)
        end

        def kind
          :in_parens
        end
      end

      class Style
        RUBY_METHODS = %i(kind style_query).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_style(self)
        end

        def kind
          :style
        end
      end

      class GenerallyEnclosed
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_generally_enclosed(self)
        end

        def kind
          :generally_enclosed
        end
      end
    end
  end

  module MediaType
    class All
      RUBY_METHODS = %i(kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_media_type_all(self)
      end

      def kind
        :all
      end
    end

    class Concrete
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_media_type_concrete(self)
      end

      def kind
        :concrete
      end
    end
  end

  class FontFaceRule
    RUBY_METHODS = %i(ascent_override descent_override family font_face font_stretch_range kind language_override line_gap_override size_adjust sources style unicode_range weight).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_font_face_rule(self)
    end

    def kind
      :font_face_rule
    end
  end

  class FontFace
    RUBY_METHODS = %i(family kind sources).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_font_face(self)
    end

    def kind
      :font_face
    end
  end

  module Font
    module Family
      class Values
        RUBY_METHODS = %i(kind values).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_family_values(self)
        end

        def kind
          :values
        end
      end

      class System
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_family_system(self)
        end

        def kind
          :system
        end
      end

      class Name
        RUBY_METHODS = %i(kind syntax).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_family_name(self)
        end

        def kind
          :name
        end
      end

      class Generic
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_family_generic(self)
        end

        def kind
          :generic
        end
      end
    end

    module Metrics
      class Override
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_metrics_override(self)
        end

        def kind
          :override
        end
      end

      class OverrideNormal
        RUBY_METHODS = %i(kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_metrics_override_normal(self)
        end

        def kind
          :override_normal
        end
      end
    end

    module SourceFormat
      class Keyword
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_source_format_keyword(self)
        end

        def kind
          :keyword
        end
      end

      class String
        RUBY_METHODS = %i(kind value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_source_format_string(self)
        end

        def kind
          :string
        end
      end
    end

    module Source
      class Url
        RUBY_METHODS = %i(format_hint kind specified_url).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_source_url(self)
        end

        def kind
          :url
        end
      end

      class Local
        RUBY_METHODS = %i(family_name kind).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_font_source_local(self)
        end

        def kind
          :local
        end
      end
    end
  end

  module FontStretch
    class Range
      RUBY_METHODS = %i(begin end kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_stretch_range(self)
      end

      def kind
        :range
      end
    end

    class Stretch
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_stretch_stretch(self)
      end

      def kind
        :stretch
      end
    end

    class Keyword
      RUBY_METHODS = %i(kind value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_stretch_keyword(self)
      end

      def kind
        :keyword
      end
    end
  end

  module FontStyle
    class Italic
      RUBY_METHODS = %i(kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_style_italic(self)
      end

      def kind
        :italic
      end
    end

    class Oblique
      RUBY_METHODS = %i(angle1 angle2 kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_style_oblique(self)
      end

      def kind
        :oblique
      end
    end
  end

  module FontWeight
    class Normal
      RUBY_METHODS = %i(kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_weight_normal(self)
      end

      def kind
        :normal
      end
    end

    class Range
      RUBY_METHODS = %i(begin end kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_font_weight_range(self)
      end

      def kind
        :range
      end
    end
  end
end
