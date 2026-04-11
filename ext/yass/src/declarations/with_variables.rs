use magnus::{DataTypeFunctions, IntoValue, Ruby, TypedData, Value, gc, typed_data};
use style::properties::VariableDeclaration;
use style_traits::ToCss;

use crate::cached_value::CachedValue;

fn variable_declaration_to_value(variable_declaration: &VariableDeclaration, ruby: &Ruby) -> Value {
    ruby
        .str_new(&variable_declaration.to_css_string())
        .into_value_with(ruby)
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::WithVariables", mark)]
pub struct YWithVariables {
    value: CachedValue<VariableDeclaration>,
}

impl YWithVariables {
    pub fn new(variable_declaration: VariableDeclaration) -> Self {
        Self {
            value: CachedValue::new(variable_declaration, variable_declaration_to_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YWithVariables {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
