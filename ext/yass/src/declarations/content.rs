use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data};
use style::counter_style::{CounterStyle, Symbol, Symbols, SymbolsType};
use style::values::generics::counters::GenericContentItem;
use style::values::specified::Content;
use style::values::specified::image::Image as SpecifiedImage;

use crate::{
    cached_value::CachedValue,
    cached_value_list::CachedValueList,
    declarations::images::image_to_value,
};

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Content", mark)]
pub struct YContent {
    value: CachedValue<Content>,
}

impl YContent {
    pub fn new(content: Content) -> Self {
        Self {
            value: CachedValue::new(content, |content, ruby| {
                make_content(content.clone(), ruby)
            }),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.value.get(ruby)
    }
}

impl DataTypeFunctions for YContent {
    fn mark(&self, marker: &gc::Marker) {
        self.value.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::Normal")]
pub struct YContentNormal {}

impl YContentNormal {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::None")]
pub struct YContentNone {}

impl YContentNone {
    pub fn new() -> Self {
        Self {}
    }
}

fn make_content(content: Content, ruby: &Ruby) -> Value {
    match content {
        Content::Normal => YContentNormal::new().into_value_with(ruby),
        Content::None => YContentNone::new().into_value_with(ruby),
        Content::Items(items) => YContentItems::new(items).into_value_with(ruby),
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Content::Items", mark)]
pub struct YContentItems {
    items: CachedValueList<GenericContentItem<SpecifiedImage>>,
    alt_start: usize,
}

impl YContentItems {
    pub fn new(content_items: style::values::generics::counters::GenericContentItems<SpecifiedImage>) -> Self {
        Self {
            items: CachedValueList::new(content_items.items.to_vec(), |item, _ctx, ruby| {
                make_content_item_value(item, ruby)
            }),

            alt_start: content_items.alt_start,
        }
    }

    pub fn items(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.items.to_a(ruby)
    }

    pub fn alt_start(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> usize {
        rb_self.alt_start
    }
}

impl DataTypeFunctions for YContentItems {
    fn mark(&self, marker: &gc::Marker) {
        self.items.mark(marker);
    }
}

fn make_content_item_value(item: &GenericContentItem<SpecifiedImage>, ruby: &Ruby) -> Value {
    match item {
        GenericContentItem::String(s) => {
            YContentItemString::new((&**s).to_owned()).into_value_with(ruby)
        }

        GenericContentItem::Counter(name, style) => {
            YContentItemCounter::new((&*name.0).to_owned(), style.clone())
                .into_value_with(ruby)
        }

        GenericContentItem::Counters(name, separator, style) => {
            YContentItemCounters::new(
                (&*name.0).to_owned(),
                (&**separator).to_owned(),
                style.clone(),
            )
            .into_value_with(ruby)
        }

        GenericContentItem::OpenQuote => ruby.to_symbol("open_quote").into_value_with(ruby),
        GenericContentItem::CloseQuote => ruby.to_symbol("close_quote").into_value_with(ruby),
        GenericContentItem::NoOpenQuote => ruby.to_symbol("no_open_quote").into_value_with(ruby),
        GenericContentItem::NoCloseQuote => ruby.to_symbol("no_close_quote").into_value_with(ruby),
        GenericContentItem::Attr(attr) => {
            YContentItemAttr::new(
                (&**attr.namespace_prefix).to_owned(),
                (&**attr.namespace_url).to_owned(),
                (&*attr.attribute).to_owned(),
                (&**attr.fallback).to_owned(),
            ).into_value_with(ruby)
        }

        GenericContentItem::Image(image) => {
            YContentItemImage::new(image.clone()).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::Item::String")]
pub struct YContentItemString {
    value: String,
}

impl YContentItemString {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.value)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::Item::Counter")]
pub struct YContentItemCounter {
    name: String,
    style: CounterStyle,
}

impl YContentItemCounter {
    pub fn new(name: String, style: CounterStyle) -> Self {
        Self { name, style }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.name)
    }

    pub fn style(ruby: &Ruby, rb_self: &Self) -> Value {
        make_counter_style_value(&rb_self.style, ruby)
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::Item::Counters")]
pub struct YContentItemCounters {
    name: String,
    separator: String,
    style: CounterStyle,
}

impl YContentItemCounters {
    pub fn new(name: String, separator: String, style: CounterStyle) -> Self {
        Self { name, separator, style }
    }

    pub fn name(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.name)
    }

    pub fn separator(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.separator)
    }

    pub fn style(ruby: &Ruby, rb_self: &Self) -> Value {
        make_counter_style_value(&rb_self.style, ruby)
    }
}

fn make_counter_style_value(style: &CounterStyle, ruby: &Ruby) -> Value {
    match style {
        CounterStyle::None => ruby.intern("none").into_value_with(ruby),
        CounterStyle::Name(name) => ruby.intern(&name.0.to_string()).into_value_with(ruby),
        CounterStyle::String(value) => ruby.str_new(value).into_value_with(ruby),
        CounterStyle::Symbols { ty, symbols } => {
            YContentCounterStyleSymbols::new(*ty, symbols.clone()).into_value_with(ruby)
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::CounterStyle::Symbols")]
pub struct YContentCounterStyleSymbols {
    kind: SymbolsType,
    symbols: Vec<String>,
}

impl YContentCounterStyleSymbols {
    pub fn new(kind: SymbolsType, symbols: Symbols) -> Self {
        Self {
            kind,
            symbols: symbols
                .0
                .iter()
                .map(|symbol| match symbol {
                    Symbol::String(value) => (&**value).to_owned(),
                    Symbol::Ident(ident) => ident.0.to_string(),
                })
                .collect(),
        }
    }

    pub fn kind(ruby: &Ruby, rb_self: &Self) -> Value {
        ruby.intern(symbols_type_name(rb_self.kind)).into_value_with(ruby)
    }

    pub fn symbols(ruby: &Ruby, rb_self: &Self) -> Result<RArray, Error> {
        let array = ruby.ary_new_capa(rb_self.symbols.len());

        for symbol in rb_self.symbols.iter() {
            array.push(ruby.str_new(symbol))?;
        }

        Ok(array)
    }
}

fn symbols_type_name(symbols_type: SymbolsType) -> &'static str {
    match symbols_type {
        SymbolsType::Cyclic => "cyclic",
        SymbolsType::Numeric => "numeric",
        SymbolsType::Alphabetic => "alphabetic",
        SymbolsType::Symbolic => "symbolic",
        SymbolsType::Fixed => "fixed",
    }
}

#[magnus::wrap(class = "Yass::Declarations::Content::Item::Attr")]
pub struct YContentItemAttr {
    namespace_prefix: String,
    namespace_url: String,
    attribute: String,
    fallback: String,
}

impl YContentItemAttr {
    pub fn new(namespace_prefix: String, namespace_url: String, attribute: String, fallback: String) -> Self {
        Self { namespace_prefix, namespace_url, attribute, fallback }
    }

    pub fn namespace_prefix(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.namespace_prefix)
    }

    pub fn namespace_url(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.namespace_url)
    }

    pub fn attribute(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.attribute)
    }

    pub fn fallback(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.fallback)
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::Content::Item::Image", mark)]
pub struct YContentItemImage {
    image: CachedValue<SpecifiedImage>,
}

impl YContentItemImage {
    pub fn new(image: SpecifiedImage) -> Self {
        Self {
            image: CachedValue::new(image, |img, ruby| image_to_value(img, ruby)),
        }
    }

    pub fn image(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.image.get(ruby)
    }
}

impl DataTypeFunctions for YContentItemImage {
    fn mark(&self, marker: &gc::Marker) {
        self.image.mark(marker);
    }
}

