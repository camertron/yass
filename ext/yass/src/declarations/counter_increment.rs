use style::values::specified::CounterIncrement;

#[magnus::wrap(class = "Yass::Declarations::CounterIncrement")]
pub struct YCounterIncrement {
  counter_increment: CounterIncrement
}

impl YCounterIncrement {
  pub fn new(counter_increment: CounterIncrement) -> Self {
    Self { counter_increment }
  }
}
