use style::values::specified::Cursor;

#[magnus::wrap(class = "Yass::Declarations::Cursor")]
pub struct YCursor {
  cursor: Cursor
}

impl YCursor {
  pub fn new(cursor: Cursor) -> Self {
    Self { cursor }
  }
}
