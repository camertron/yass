use magnus::{IntoValue, Ruby, Value};
use style::values::computed::GridAutoFlow;

#[magnus::wrap(class = "Yass::Declarations::GridAutoFlow")]
pub struct YGridAutoFlow {
    grid_auto_flow: GridAutoFlow
}

impl YGridAutoFlow {
    pub fn new(grid_auto_flow: GridAutoFlow) -> Self {
        Self { grid_auto_flow }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.intern(if rb_self.grid_auto_flow.intersects(GridAutoFlow::COLUMN) {
            "column"
        } else {
            "row"
        })
        .into_value_with(ruby)
    }

    pub fn axis(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.intern(if rb_self.grid_auto_flow.intersects(GridAutoFlow::COLUMN) {
            "column"
        } else {
            "row"
        })
        .into_value_with(ruby)
    }

    pub fn dense(_ruby: &Ruby, rb_self: &Self) -> bool {
        rb_self.grid_auto_flow.intersects(GridAutoFlow::DENSE)
    }
}
