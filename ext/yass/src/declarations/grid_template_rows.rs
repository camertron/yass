use magnus::{DataTypeFunctions, Ruby, TypedData, Value, gc, typed_data};
use style::values::specified::GridTemplateComponent;

use crate::{
    cached_value::CachedValue,
    declarations::grid_template::make_grid_template_component_value,
};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::GridTemplateRows", mark)]
pub struct YGridTemplateRows {
    value: CachedValue<GridTemplateComponent>,
}

impl YGridTemplateRows {
    pub fn new(grid_template_component: GridTemplateComponent) -> Self {
        Self {
            value: CachedValue::new(grid_template_component, make_grid_template_component_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YGridTemplateRows {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}
