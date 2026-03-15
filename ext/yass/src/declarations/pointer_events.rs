use style::values::computed::PointerEvents;

#[magnus::wrap(class = "Yass::Declarations::PointerEvents")]
pub struct YPointerEvents {
  pointer_events: PointerEvents
}

impl YPointerEvents {
  pub fn new(pointer_events: PointerEvents) -> Self {
    Self { pointer_events }
  }
}
