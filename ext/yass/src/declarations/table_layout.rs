use style::computed_values::table_layout::T;

#[magnus::wrap(class = "Yass::Declarations::TableLayout")]
pub struct YTableLayout {
  specified_value: T
}

impl YTableLayout {
  pub fn new(specified_value: T) -> Self {
    Self { specified_value }
  }
}
