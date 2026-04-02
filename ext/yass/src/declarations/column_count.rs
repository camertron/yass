use magnus::{IntoValue, Ruby, Value, typed_data};
use style::values::{generics::{GreaterThanOrEqualToOne, column::ColumnCount}, specified::Integer};

pub fn make_column_count(column_count: ColumnCount<GreaterThanOrEqualToOne<Integer>>, ruby: &Ruby) -> Value {
    match column_count {
        ColumnCount::Auto => YColumnCountAuto::new().into_value_with(ruby),
        ColumnCount::Integer(greater_than_or_equal_to_one) => {
            YColumnCountInteger::new(greater_than_or_equal_to_one).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::ColumnCount::Auto")]
pub struct YColumnCountAuto {}

impl YColumnCountAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::ColumnCount::Integer")]
pub struct YColumnCountInteger {
    value: i32,
}

impl YColumnCountInteger {
    pub fn new(greater_than_or_equal_to_one: GreaterThanOrEqualToOne<Integer>) -> Self {
        Self {
            value: greater_than_or_equal_to_one.0.value(),
        }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> i32 {
        rb_self.value
    }
}
