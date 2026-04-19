use magnus::{DataTypeFunctions, Error, IntoValue, RArray, RString, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::{counter_style::{CounterStyle, Symbol, Symbols, SymbolsType}, values::computed::ListStyleType};

use crate::cached_value::CachedValue;

fn make_list_style_type_value(list_style_type: &ListStyleType, ruby: &Ruby) -> Value {
    match &list_style_type.0 {
        CounterStyle::None => YListStyleTypeNone::new().into_value_with(ruby),
        CounterStyle::Name(name) => YListStyleTypeName::new(name.0.to_string()).into_value_with(ruby),
        CounterStyle::String(value) => YListStyleTypeString::new(value.to_string()).into_value_with(ruby),
        CounterStyle::Symbols { ty, symbols } => {
            YListStyleTypeSymbols::new(*ty, symbols.clone()).into_value_with(ruby)
        }
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

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::ListStyleType", mark)]
pub struct YListStyleType {
    list_style_type: CachedValue<ListStyleType>,
}

impl YListStyleType {
    pub fn new(list_style_type: ListStyleType) -> Self {
        Self {
            list_style_type: CachedValue::new(list_style_type, make_list_style_type_value),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.list_style_type.get(ruby)
    }

    pub fn bullet(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> bool {
        if let Some(value) = &rb_self.list_style_type.value {
            value.is_bullet()
        } else {
            false
        }
    }
}

impl DataTypeFunctions for YListStyleType {
    fn mark(&self, marker: &gc::Marker) {
        self.list_style_type.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::ListStyleType::None")]
pub struct YListStyleTypeNone {}

impl YListStyleTypeNone {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::ListStyleType::Name")]
pub struct YListStyleTypeName {
    name: String,
}

impl YListStyleTypeName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> Id {
        ruby.intern(&rb_self.name)
    }
}

#[magnus::wrap(class = "Yass::Declarations::ListStyleType::String")]
pub struct YListStyleTypeString {
    value: String,
}

impl YListStyleTypeString {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(ruby: &Ruby, rb_self: &Self) -> RString {
        ruby.str_new(&rb_self.value)
    }
}

#[magnus::wrap(class = "Yass::Declarations::ListStyleType::Symbols")]
pub struct YListStyleTypeSymbols {
    kind: SymbolsType,
    symbols: Vec<String>,
}

impl YListStyleTypeSymbols {
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

    pub fn ty(ruby: &Ruby, rb_self: &Self) -> Value {
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
