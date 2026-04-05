use magnus::{IntoValue, Ruby, Value};
use style::values::specified::GridLine;

#[magnus::wrap(class = "Yass::Declarations::GridRowEnd")]
pub struct YGridRowEnd {
    grid_line: GridLine
}

impl YGridRowEnd {
    pub fn new(grid_line: GridLine) -> Self {
        Self { grid_line }
    }

    pub fn ident(ruby: &Ruby, rb_self: &Self) -> Value {
        let ident = rb_self.grid_line.ident.0.to_string();

        if ident.is_empty() {
            ruby.qnil().into_value_with(ruby)
        } else {
            ruby.str_new(&ident).into_value_with(ruby)
        }
    }

    pub fn line_num(_ruby: &Ruby, rb_self: &Self) -> i32 {
        rb_self.grid_line.line_num.value()
    }

    pub fn span(_ruby: &Ruby, rb_self: &Self) -> bool {
        rb_self.grid_line.is_span
    }

    pub fn auto(_ruby: &Ruby, rb_self: &Self) -> bool {
        rb_self.grid_line.is_auto()
    }

    pub fn ident_only(_ruby: &Ruby, rb_self: &Self) -> bool {
        rb_self.grid_line.is_ident_only()
    }
}
