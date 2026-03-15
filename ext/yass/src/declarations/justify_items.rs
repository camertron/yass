use style::values::specified::JustifyItems;

#[magnus::wrap(class = "Yass::Declarations::JustifyItems")]
pub struct YJustifyItems {
  justify_items: JustifyItems
}

impl YJustifyItems {
  pub fn new(justify_items: JustifyItems) -> Self {
    Self { justify_items }
  }
}
