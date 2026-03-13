use magnus::{Error, Ruby, Value};
use selectors::attr::{AttrSelectorWithOptionalNamespace, NamespaceConstraint};
use style::selector_parser::SelectorImpl;

use crate::{selectors::YSpecificNamespaceConstraint, utils::singleton_instance};

pub struct YAttributeOther {
    attr_selector_with_optional_namespace: Box<AttrSelectorWithOptionalNamespace<SelectorImpl>>
}

impl YAttributeOther {
    pub fn new(attr_selector_with_optional_namespace: Box<AttrSelectorWithOptionalNamespace<SelectorImpl>>) -> Self {
        Self { attr_selector_with_optional_namespace }
    }

    pub fn namespace(&self, ruby: &Ruby) -> Result<Option<Value>, Error> {
        Ok(
            if let Some(constraint) = &self.attr_selector_with_optional_namespace.namespace {
                match constraint {
                    NamespaceConstraint::Any => {
                        Some(singleton_instance("Yass::Selector::AnyNamespace", ruby)?)
                    },

                    NamespaceConstraint::Specific(constraint) => {
                        Some(
                            ruby.into_value(
                                YSpecificNamespaceConstraint::new(
                                    constraint.0.to_string(),
                                    constraint.1.to_string()
                                )
                            )
                        )
                    }
                }
            } else {
                None
            }
        )
    }
}
