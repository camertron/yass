use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data};
use style::values::{computed::Quotes, specified::list::{QuoteList, QuotePair}};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList};

fn make_quotes_value(quotes: &Quotes, ruby: &Ruby) -> Value {
    match quotes {
        Quotes::Auto => YQuotesAuto::new().into_value_with(ruby),
        Quotes::QuoteList(quote_list) if quote_list.0.is_empty() => {
            YQuotesNone::new().into_value_with(ruby)
        }
        Quotes::QuoteList(quote_list) => {
            YQuotesQuoteList::new(quote_list.clone()).into_value_with(ruby)
        }
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Quotes", mark)]
pub struct YQuotes {
    quotes: CachedValue<Quotes>,
}

impl YQuotes {
    pub fn new(quotes: Quotes) -> Self {
        Self {
            quotes: CachedValue::new(quotes, make_quotes_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.quotes.get(ruby)
    }
}

impl DataTypeFunctions for YQuotes {
    fn mark(&self, marker: &gc::Marker) {
        self.quotes.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Quotes::Auto")]
pub struct YQuotesAuto {}

impl YQuotesAuto {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Quotes::None")]
pub struct YQuotesNone {}

impl YQuotesNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Quotes::QuoteList", mark)]
pub struct YQuotesQuoteList {
    values: CachedValueList<QuotePair>,
}

impl YQuotesQuoteList {
    pub fn new(quote_list: QuoteList) -> Self {
        Self {
            values: CachedValueList::new(quote_list.0.iter().cloned().collect(), |quote_pair, _ctx, ruby| {
                YQuotesQuotePair::new(quote_pair.clone()).into_value_with(ruby)
            }),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YQuotesQuoteList {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Quotes::QuotePair")]
pub struct YQuotesQuotePair {
    opening: String,
    closing: String,
}

impl YQuotesQuotePair {
    pub fn new(quote_pair: QuotePair) -> Self {
        Self {
            opening: quote_pair.opening.to_string(),
            closing: quote_pair.closing.to_string(),
        }
    }

    pub fn opening(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.opening)
    }

    pub fn closing(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.closing)
    }
}
