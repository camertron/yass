use magnus::{DataTypeFunctions, Error, IntoValue, RArray, Ruby, TypedData, Value, gc, typed_data, value::Id};
use style::values::{computed::font::{FamilyName, FontFamilyNameSyntax, FontFamilyList, GenericFontFamily, SingleFontFamily}, specified::FontFamily};

use crate::{cached_value::CachedValue, cached_value_list::CachedValueList};

fn make_font_family(font_family: &FontFamily, ruby: &Ruby) -> Value {
    match font_family {
        FontFamily::Values(values) => YFontFamilyValues::new(values.clone()).into_value_with(ruby),
        FontFamily::System(_) => YFontFamilySystem::new().into_value_with(ruby),
    }
}

fn make_single_font_family(value: &SingleFontFamily, _ctx: &Option<()>, ruby: &Ruby) -> Value {
    match value {
        SingleFontFamily::FamilyName(name) => YFontFamilyName::new(name.clone()).into_value_with(ruby),
        SingleFontFamily::Generic(generic) => {
            YFontFamilyGeneric::new(*generic).into_value_with(ruby)
        },
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontFamily", mark)]
pub struct YFontFamily {
    font_family: CachedValue<FontFamily>
}

impl YFontFamily {
    pub fn new(font_family: FontFamily) -> Self {
        Self {
            font_family: CachedValue::new(font_family, make_font_family),
        }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Value {
        rb_self.font_family.get(ruby)
    }
}

impl DataTypeFunctions for YFontFamily {
    fn mark(&self, marker: &gc::Marker) {
        self.font_family.mark(marker);
    }
}

#[derive(TypedData)]
#[magnus(class = "Yass::Declarations::FontFamily::Values", mark)]
pub struct YFontFamilyValues {
    values: CachedValueList<SingleFontFamily>
}

impl YFontFamilyValues {
    pub fn new(values: FontFamilyList) -> Self {
        Self {
            values: CachedValueList::new(values.iter().cloned().collect(), make_single_font_family),
        }
    }

    pub fn values(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Result<RArray, Error> {
        rb_self.values.to_a(ruby)
    }
}

impl DataTypeFunctions for YFontFamilyValues {
    fn mark(&self, marker: &gc::Marker) {
        self.values.mark(marker);
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontFamily::System")]
pub struct YFontFamilySystem {}

impl YFontFamilySystem {
    pub fn new() -> Self {
        Self {}
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontFamily::Name")]
pub struct YFontFamilyName {
    family_name: FamilyName,
}

impl YFontFamilyName {
    pub fn new(family_name: FamilyName) -> Self {
        Self { family_name }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> String {
        rb_self.family_name.name.to_string()
    }

    pub fn syntax(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.family_name.syntax {
            FontFamilyNameSyntax::Quoted => ruby.intern("quoted"),
            FontFamilyNameSyntax::Identifiers => ruby.intern("identifiers"),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::FontFamily::Generic")]
pub struct YFontFamilyGeneric {
    generic: GenericFontFamily,
}

impl YFontFamilyGeneric {
    pub fn new(generic: GenericFontFamily) -> Self {
        Self { generic }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.generic {
            GenericFontFamily::None => ruby.intern("none"),
            GenericFontFamily::Serif => ruby.intern("serif"),
            GenericFontFamily::SansSerif => ruby.intern("sans_serif"),
            GenericFontFamily::Monospace => ruby.intern("monospace"),
            GenericFontFamily::Cursive => ruby.intern("cursive"),
            GenericFontFamily::Fantasy => ruby.intern("fantasy"),
            GenericFontFamily::SystemUi => ruby.intern("system_ui"),
        }
    }
}
