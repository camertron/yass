use magnus::{RString, Ruby, value::Id};
use selectors::attr::{AttrSelectorOperator, ParsedCaseSensitivity};

use crate::selectors::YSelector;

#[magnus::wrap(class = "Yass::Selector::AttributeInNoNamespace")]
pub struct YAttributeInNoNamespace {
    local_name: String,
    operator: AttrSelectorOperator,
    value: String,
    case_sensitivity: ParsedCaseSensitivity
}

impl YAttributeInNoNamespace {
    pub fn new(local_name: String, operator: AttrSelectorOperator, value: String, case_sensitivity: ParsedCaseSensitivity) -> Self {
        Self { local_name, operator, value, case_sensitivity }
    }

    pub fn name(&self, ruby: &Ruby) -> RString {
        return ruby.str_new(&self.local_name)
    }

    pub fn operator(&self, ruby: &Ruby) -> Id {
        YSelector::intern_operator(&self.operator, ruby)
    }

    pub fn value(&self, ruby: &Ruby) -> RString {
        ruby.str_new(&self.value)
    }

    pub fn case_sensitivity(&self, ruby: &Ruby) -> Id {
        return YSelector::intern_case_sensitivity(&self.case_sensitivity, ruby)
    }
}
