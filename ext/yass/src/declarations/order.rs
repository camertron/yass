use style::values::specified::Integer;

#[magnus::wrap(class = "Yass::Declarations::Order")]
pub struct YOrder {
  integer: Integer
}

impl YOrder {
  pub fn new(integer: Integer) -> Self {
    Self { integer }
  }
}
