use style::values::computed::Quotes;

#[magnus::wrap(class = "Yass::Declarations::Quotes")]
pub struct YQuotes {
  quotes: Quotes
}

impl YQuotes {
  pub fn new(quotes: Quotes) -> Self {
    Self { quotes }
  }
}
