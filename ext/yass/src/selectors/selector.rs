use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, gc, typed_data, value::Id};
use selectors::{attr::{AttrSelectorOperator, ParsedCaseSensitivity}, parser::{Component, Selector}};
use style::selector_parser::SelectorImpl;

use crate::{selectors::{
    YAttributeInNoNamespace,
    YAttributeInNoNamespaceExists,
    YAttributeOther,
    YClass,
    YCombinator,
    YDefaultNamespace,
    YEmpty,
    YExplicitAnyNamespace,
    YExplicitNoNamespace,
    YExplicitUniversalType,
    YHas,
    YHost,
    YID,
    YImplicitScope,
    YInvalid,
    YIs,
    YLocalName,
    YNamespace,
    YNegation,
    YNonTSPseudoClass,
    YNth,
    YNthOf,
    YParentSelector,
    YPart,
    YPseudoElement,
    YRelativeSelectorAnchor,
    YRoot,
    YScope,
    YSelectorChild,
    YSlotted,
    YWhere
}, cached_value_list::CachedValueList};

#[derive(TypedData)]
#[magnus(class = "Yass::Selector", mark)]
pub struct YSelector {
    selector: Selector<SelectorImpl>,
    cached_children: CachedValueList<Component<SelectorImpl>>
}

impl DataTypeFunctions for YSelector {
    fn mark(&self, marker: &gc::Marker) {
        self.cached_children.mark(marker);
    }
}

impl YSelector {
    pub fn new(selector: Selector<SelectorImpl>) -> Self {
        Self {
            selector,
            cached_children: CachedValueList::new(vec![], |component, _ctx, ruby| {
                YSelector::wrap_child(component, ruby).into_value_with(ruby)
            })
        }
    }

    fn wrap_child(component: &Component<SelectorImpl>, ruby: &Ruby) -> YSelectorChild {
        match component {
            Component::LocalName(local_name) => {
                YSelectorChild::LocalName(YLocalName::new(local_name.clone()))
            },
            Component::ID(id) => {
                YSelectorChild::ID(YID::new(id.to_string()))
            },
            Component::Class(class) => {
                YSelectorChild::Class(YClass::new(class.to_string()))
            },

            Component::AttributeInNoNamespaceExists { local_name, local_name_lower: _ } => {
                YSelectorChild::AttributeInNoNamespaceExists(YAttributeInNoNamespaceExists::new(local_name.to_string()))
            },

            Component::AttributeInNoNamespace { local_name, operator, value, case_sensitivity } => {
                YSelectorChild::AttributeInNoNamespace(
                    YAttributeInNoNamespace::new(
                        local_name.to_string(),
                        *operator,
                        value.to_string(),
                        *case_sensitivity
                    )
                )
            },

            Component::AttributeOther(attr_selector_with_optional_namespace) => {
                YSelectorChild::AttributeOther(
                    YAttributeOther::new(attr_selector_with_optional_namespace.clone())
                )
            },

            Component::ExplicitUniversalType => {
                YSelectorChild::ExplicitUniversalType(YExplicitUniversalType::new())
            },

            Component::ExplicitAnyNamespace => {
                YSelectorChild::ExplicitAnyNamespace(YExplicitAnyNamespace::new())
            },

            Component::ExplicitNoNamespace => {
                YSelectorChild::ExplicitNoNamespace(YExplicitNoNamespace::new())
            },

            Component::DefaultNamespace(ns) => {
                YSelectorChild::DefaultNamespace(YDefaultNamespace::new(ns.to_string()))
            },

            Component::Namespace(prefix, url) => {
                YSelectorChild::Namespace(YNamespace::new(prefix.to_string(), url.to_string()))
            },

            Component::Negation(selector_list) => {
                YSelectorChild::Negation(YNegation::new(selector_list.clone()))
            },

            Component::Root => YSelectorChild::Root(YRoot::new()),
            Component::Empty => YSelectorChild::Empty(YEmpty::new()),
            Component::Scope => YSelectorChild::Scope(YScope::new()),
            Component::ImplicitScope => YSelectorChild::ImplicitScope(YImplicitScope::new()),
            Component::ParentSelector => YSelectorChild::ParentSelector(YParentSelector::new()),

            Component::Nth(nth_selector_data) => {
                YSelectorChild::Nth(YNth::new(nth_selector_data.clone()))
            },

            Component::NthOf(nth_of_selector_data) => {
                YSelectorChild::NthOf(YNthOf::new(nth_of_selector_data.clone()))
            },

            Component::NonTSPseudoClass(non_ts_pseudo_class) => {
                YSelectorChild::NonTSPseudoClass(YNonTSPseudoClass::new(non_ts_pseudo_class.clone()))
            },

            Component::Slotted(selector) => {
                YSelectorChild::Slotted(YSlotted::new(selector.clone(), ruby))
            },

            Component::Part(items) => {
                let mut item_strings: Vec<String> = Vec::with_capacity(items.len());

                for item in items {
                    item_strings.push(item.to_string());
                }

                YSelectorChild::Part(YPart::new(item_strings))
            },

            Component::Host(selector) => {
                YSelectorChild::Host(YHost::new(selector.clone(), ruby))
            },

            Component::Where(selector_list) => {
                YSelectorChild::Where(YWhere::new(selector_list.clone()))
            },

            Component::Is(selector_list) => {
                YSelectorChild::Is(YIs::new(selector_list.clone()))
            },

            Component::Has(relative_selectors) => {
                YSelectorChild::Has(YHas::new(relative_selectors.clone()))
            },

            Component::Invalid(_) => {
                YSelectorChild::Invalid(YInvalid::new())
            },

            Component::PseudoElement(_) => {
                YSelectorChild::PseudoElement(YPseudoElement::new())
            },

            Component::Combinator(combinator) => {
                YSelectorChild::Combinator(YCombinator::new(combinator.clone()))
            },

            Component::RelativeSelectorAnchor => {
                YSelectorChild::RelativeSelectorAnchor(YRelativeSelectorAnchor::new())
            }
        }
    }

    pub fn children(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        if rb_self.cached_children.is_empty() {
            let mut iter = rb_self.selector.iter();

            loop {
                for raw_component in iter.by_ref() {
                    rb_self.cached_children.add(raw_component.clone(), ruby)?;
                }

                if let Some(combinator) = iter.next_sequence() {
                    rb_self.cached_children.add(Component::Combinator(combinator), ruby)?;
                } else {
                    break;
                }
            }
        }

        rb_self.cached_children.to_a(ruby)
    }

    pub fn intern_operator(operator: &AttrSelectorOperator, ruby: &Ruby) -> Id {
        match operator {
            AttrSelectorOperator::Equal => ruby.intern("equal"),
            AttrSelectorOperator::Includes => ruby.intern("includes"),
            AttrSelectorOperator::DashMatch => ruby.intern("dash_match"),
            AttrSelectorOperator::Prefix => ruby.intern("prefix"),
            AttrSelectorOperator::Substring => ruby.intern("substring"),
            AttrSelectorOperator::Suffix => ruby.intern("suffix")
        }
    }

    pub fn intern_case_sensitivity(cs: &ParsedCaseSensitivity, ruby: &Ruby) -> Id {
        match cs {
            // 's' was specified.
            ParsedCaseSensitivity::ExplicitCaseSensitive => ruby.intern("explicit_case_sensitive"),
            // 'i' was specified.
            ParsedCaseSensitivity::AsciiCaseInsensitive => ruby.intern("ascii_case_insensitive"),
            // No flags were specified and HTML says this is a case-sensitive attribute.
            ParsedCaseSensitivity::CaseSensitive => ruby.intern("case_sensitive"),
            // No flags were specified and HTML says this is a case-insensitive attribute.
            ParsedCaseSensitivity::AsciiCaseInsensitiveIfInHtmlElementInHtmlDocument => {
                ruby.intern("ascii_case_insensitive_if_in_html_element_in_html_document")
            }
        }
    }
}
