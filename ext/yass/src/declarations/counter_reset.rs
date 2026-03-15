use style::values::specified::CounterReset;

#[magnus::wrap(class = "Yass::Declarations::CounterReset")]
pub struct YCounterReset {
  counter_reset: CounterReset
}

impl YCounterReset {
  pub fn new(counter_reset: CounterReset) -> Self {
    Self { counter_reset }
  }
}
