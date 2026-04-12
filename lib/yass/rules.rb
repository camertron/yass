# frozen_string_literal: true

module Yass
  class StyleRule
    RUBY_METHODS = %i(declarations selectors).freeze

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
      RUBY_METHODS = %i(style_query).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_not(self)
      end

      def kind
        :not
      end
    end

    class Operation
      RUBY_METHODS = %i(operator style_query).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_operation(self)
      end

      def kind
        :operation
      end
    end

    class InParens
      RUBY_METHODS = %i(style_query).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_in_parens(self)
      end

      def kind
        :in_parens
      end
    end

    class StyleFeature
      RUBY_METHODS = %i(value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_style_query_style_feature(self)
      end

      def kind
        :style_feature
      end
    end

    class GenerallyEnclosed
      RUBY_METHODS = %i(value).freeze

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
    RUBY_METHODS = %i(media_queries rules).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_media_rule(self)
    end

    def kind
      :media_rule
    end
  end

  class MediaQuery
    RUBY_METHODS = %i(media_type qualifier query_condition).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_media_query(self)
    end

    def kind
      :media_query
    end

    module QueryCondition
      class FeatureExpression
        RUBY_METHODS = %i(value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_feature_expression(self)
        end

        def kind
          :feature_expression
        end
      end

      class Custom
        RUBY_METHODS = %i(value).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_custom(self)
        end

        def kind
          :custom
        end
      end

      class Operation
        RUBY_METHODS = %i(operator query_conditions).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_operation(self)
        end

        def kind
          :operation
        end
      end

      class InParens
        RUBY_METHODS = %i(query_condition).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_in_parens(self)
        end

        def kind
          :in_parens
        end
      end

      class Style
        RUBY_METHODS = %i(style_query).freeze

        include ::Yass::Node

        def accept(visitor)
          visitor.visit_media_query_query_condition_style(self)
        end

        def kind
          :style
        end
      end

      class GenerallyEnclosed
        RUBY_METHODS = %i(value).freeze

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
      include ::Yass::Node

      def accept(visitor)
        visitor.visit_media_type_all(self)
      end

      def kind
        :all
      end
    end

    class Concrete
      RUBY_METHODS = %i(value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_media_type_concrete(self)
      end

      def kind
        :concrete
      end
    end
  end
end
