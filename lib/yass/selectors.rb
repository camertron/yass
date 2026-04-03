# frozen_string_literal: true

module Yass
  class Selector
    RUBY_METHODS = %i(children kind).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_selector(self)
    end

    def kind
      :selector
    end

    class AnPlusB
      RUBY_METHODS = %i(b a kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_an_plus_b(self)
      end

      def kind
        :an_plus_b
      end
    end

    class AttributeInNoNamespace
      RUBY_METHODS = %i(kind operator value case_sensitivity).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_attribute_in_no_namespace(self)
      end

      def kind
        :attribute_in_no_namespace
      end
    end

    class AttributeInNoNamespaceExists
      RUBY_METHODS = %i(kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_attribute_in_no_namespace_exists(self)
      end

      def kind
        :attribute_in_no_namespace_exists
      end
    end

    class AttributeOther
      RUBY_METHODS = %i(namespace kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_attribute_other(self)
      end

      def kind
        :attribute_other
      end
    end

    class Klass
      RUBY_METHODS = %i(value kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_klass(self)
      end

      def kind
        :klass
      end
    end

    class Combinator
      RUBY_METHODS = %i(kind ancestor? pseudo_element? sibling?).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_combinator(self)
      end

      def kind
        :combinator
      end
    end

    class DefaultNamespace
      RUBY_METHODS = %i(url kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_default_namespace(self)
      end

      def kind
        :default_namespace
      end
    end

    class Has
      RUBY_METHODS = %i(relative_selectors kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_has(self)
      end

      def kind
        :has
      end
    end

    class Host
      RUBY_METHODS = %i(selector kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_host(self)
      end

      def kind
        :host
      end
    end

    class Id
      RUBY_METHODS = %i(value kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_id(self)
      end

      def kind
        :id
      end
    end

    class Is
      RUBY_METHODS = %i(selectors kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_is(self)
      end

      def kind
        :is
      end
    end

    class LocalName
      RUBY_METHODS = %i(value kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_local_name(self)
      end

      def kind
        :local_name
      end
    end

    class Namespace
      RUBY_METHODS = %i(kind url prefix).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_namespace(self)
      end

      def kind
        :namespace
      end
    end

    class Negation
      RUBY_METHODS = %i(selectors kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_negation(self)
      end

      def kind
        :negation
      end
    end

    class NonTsPseudoClass
      RUBY_METHODS = %i(kind type value).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_non_ts_pseudo_class(self)
      end

      def kind
        :non_ts_pseudo_class
      end
    end

    class Nth
      RUBY_METHODS = %i(kind function? type an_plus_b).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_nth(self)
      end

      def kind
        :nth
      end
    end

    class NthOf
      RUBY_METHODS = %i(nth selectors kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_nth_of(self)
      end

      def kind
        :nth_of
      end
    end

    class Part
      RUBY_METHODS = %i(items kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_part(self)
      end

      def kind
        :part
      end
    end

    class Slotted
      RUBY_METHODS = %i(selector kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_slotted(self)
      end

      def kind
        :slotted
      end
    end

    class SpecificNamespaceConstraint
      RUBY_METHODS = %i(kind url prefix).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_specific_namespace_constraint(self)
      end

      def kind
        :specific_namespace_constraint
      end
    end

    class Where
      RUBY_METHODS = %i(selectors kind).freeze

      include ::Yass::Node

      def accept(visitor)
        visitor.visit_selector_where(self)
      end

      def kind
        :where
      end
    end
  end

  class RelativeSelector
    RUBY_METHODS = %i(match_hint kind selector).freeze

    include ::Yass::Node

    def accept(visitor)
      visitor.visit_relative_selector(self)
    end

    def kind
      :relative_selector
    end
  end
end
