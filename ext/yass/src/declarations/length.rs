use magnus::{IntoValue, Ruby, Value, typed_data, value::Id};
use style::values::specified::{AbsoluteLength, CharacterWidth, FontRelativeLength, NoCalcLength, ViewportPercentageLength, length::ContainerRelativeLength};

pub struct YLength { }

impl YLength {
    pub fn make(length: NoCalcLength, ruby: &Ruby) -> Value {
        match length {
            NoCalcLength::Absolute(absolute_length) => {
                YAbsoluteLength::new(absolute_length).into_value_with(ruby)
            },
            NoCalcLength::FontRelative(font_relative_length) => {
                YFontRelativeLength::new(font_relative_length).into_value_with(ruby)
            },
            NoCalcLength::ViewportPercentage(viewport_percentage_length) => {
                YViewportPercentageLength::new(viewport_percentage_length).into_value_with(ruby)
            },
            NoCalcLength::ContainerRelative(container_relative_length) => {
                YContainerRelativeLength::new(container_relative_length).into_value_with(ruby)
            },
            NoCalcLength::ServoCharacterWidth(character_width) => {
                YCharacterWidthLength::new(character_width).into_value_with(ruby)
            },
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Length::Absolute")]
pub struct YAbsoluteLength {
    absolute_length: AbsoluteLength
}

impl YAbsoluteLength {
    pub fn new(absolute_length: AbsoluteLength) -> Self {
        Self { absolute_length }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        match rb_self.absolute_length {
            AbsoluteLength::Px(value) => value,
            AbsoluteLength::In(value) => value,
            AbsoluteLength::Cm(value) => value,
            AbsoluteLength::Mm(value) => value,
            AbsoluteLength::Q(value) => value,
            AbsoluteLength::Pt(value) => value,
            AbsoluteLength::Pc(value) => value,
        }
    }

    pub fn unit(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.absolute_length {
            AbsoluteLength::Px(_) => ruby.intern("px"),
            AbsoluteLength::In(_) => ruby.intern("in"),
            AbsoluteLength::Cm(_) => ruby.intern("cm"),
            AbsoluteLength::Mm(_) => ruby.intern("mm"),
            AbsoluteLength::Q(_) => ruby.intern("q"),
            AbsoluteLength::Pt(_) => ruby.intern("pt"),
            AbsoluteLength::Pc(_) => ruby.intern("pc"),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Length::FontRelative")]
pub struct YFontRelativeLength {
    font_relative_length: FontRelativeLength
}

impl YFontRelativeLength {
    pub fn new(font_relative_length: FontRelativeLength) -> Self {
        Self { font_relative_length }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        match rb_self.font_relative_length {
            FontRelativeLength::Em(value) => value,
            FontRelativeLength::Ex(value) => value,
            FontRelativeLength::Rex(value) => value,
            FontRelativeLength::Ch(value) => value,
            FontRelativeLength::Rch(value) => value,
            FontRelativeLength::Cap(value) => value,
            FontRelativeLength::Rcap(value) => value,
            FontRelativeLength::Ic(value) => value,
            FontRelativeLength::Ric(value) => value,
            FontRelativeLength::Rem(value) => value,
            FontRelativeLength::Lh(value) => value,
            FontRelativeLength::Rlh(value) => value,
        }
    }

    pub fn unit(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.font_relative_length {
            FontRelativeLength::Em(_) => ruby.intern("em"),
            FontRelativeLength::Ex(_) => ruby.intern("ex"),
            FontRelativeLength::Rex(_) => ruby.intern("rex"),
            FontRelativeLength::Ch(_) => ruby.intern("ch"),
            FontRelativeLength::Rch(_) => ruby.intern("rch"),
            FontRelativeLength::Cap(_) => ruby.intern("cap"),
            FontRelativeLength::Rcap(_) => ruby.intern("rcap"),
            FontRelativeLength::Ic(_) => ruby.intern("ic"),
            FontRelativeLength::Ric(_) => ruby.intern("ric"),
            FontRelativeLength::Rem(_) => ruby.intern("rem"),
            FontRelativeLength::Lh(_) => ruby.intern("lh"),
            FontRelativeLength::Rlh(_) => ruby.intern("rlh"),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Length::ViewportPercentage")]
pub struct YViewportPercentageLength {
    viewport_percentage_length: ViewportPercentageLength
}

impl YViewportPercentageLength {
    pub fn new(viewport_percentage_length: ViewportPercentageLength) -> Self {
        Self { viewport_percentage_length }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        match rb_self.viewport_percentage_length {
            ViewportPercentageLength::Vw(value) => value,
            ViewportPercentageLength::Svw(value) => value,
            ViewportPercentageLength::Lvw(value) => value,
            ViewportPercentageLength::Dvw(value) => value,
            ViewportPercentageLength::Vh(value) => value,
            ViewportPercentageLength::Svh(value) => value,
            ViewportPercentageLength::Lvh(value) => value,
            ViewportPercentageLength::Dvh(value) => value,
            ViewportPercentageLength::Vmin(value) => value,
            ViewportPercentageLength::Svmin(value) => value,
            ViewportPercentageLength::Lvmin(value) => value,
            ViewportPercentageLength::Dvmin(value) => value,
            ViewportPercentageLength::Vmax(value) => value,
            ViewportPercentageLength::Svmax(value) => value,
            ViewportPercentageLength::Lvmax(value) => value,
            ViewportPercentageLength::Dvmax(value) => value,
            ViewportPercentageLength::Vb(value) => value,
            ViewportPercentageLength::Svb(value) => value,
            ViewportPercentageLength::Lvb(value) => value,
            ViewportPercentageLength::Dvb(value) => value,
            ViewportPercentageLength::Vi(value) => value,
            ViewportPercentageLength::Svi(value) => value,
            ViewportPercentageLength::Lvi(value) => value,
            ViewportPercentageLength::Dvi(value) => value,
        }
    }

    pub fn unit(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.viewport_percentage_length {
            ViewportPercentageLength::Vw(_) => ruby.intern("vw"),
            ViewportPercentageLength::Svw(_) => ruby.intern("svw"),
            ViewportPercentageLength::Lvw(_) => ruby.intern("lvw"),
            ViewportPercentageLength::Dvw(_) => ruby.intern("dvw"),
            ViewportPercentageLength::Vh(_) => ruby.intern("vh"),
            ViewportPercentageLength::Svh(_) => ruby.intern("svh"),
            ViewportPercentageLength::Lvh(_) => ruby.intern("lvh"),
            ViewportPercentageLength::Dvh(_) => ruby.intern("dvh"),
            ViewportPercentageLength::Vmin(_) => ruby.intern("vmin"),
            ViewportPercentageLength::Svmin(_) => ruby.intern("svmin"),
            ViewportPercentageLength::Lvmin(_) => ruby.intern("lvmin"),
            ViewportPercentageLength::Dvmin(_) => ruby.intern("dvmin"),
            ViewportPercentageLength::Vmax(_) => ruby.intern("vmax"),
            ViewportPercentageLength::Svmax(_) => ruby.intern("svmax"),
            ViewportPercentageLength::Lvmax(_) => ruby.intern("lvmax"),
            ViewportPercentageLength::Dvmax(_) => ruby.intern("dvmax"),
            ViewportPercentageLength::Vb(_) => ruby.intern("vb"),
            ViewportPercentageLength::Svb(_) => ruby.intern("svb"),
            ViewportPercentageLength::Lvb(_) => ruby.intern("lvb"),
            ViewportPercentageLength::Dvb(_) => ruby.intern("dvb"),
            ViewportPercentageLength::Vi(_) => ruby.intern("vi"),
            ViewportPercentageLength::Svi(_) => ruby.intern("svi"),
            ViewportPercentageLength::Lvi(_) => ruby.intern("lvi"),
            ViewportPercentageLength::Dvi(_) => ruby.intern("dvi"),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Length::ContainerRelative")]
pub struct YContainerRelativeLength {
    container_relative_length: ContainerRelativeLength
}

impl YContainerRelativeLength {
    pub fn new(container_relative_length: ContainerRelativeLength) -> Self {
        Self { container_relative_length }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        match rb_self.container_relative_length {
            ContainerRelativeLength::Cqw(value) => value,
            ContainerRelativeLength::Cqh(value) => value,
            ContainerRelativeLength::Cqi(value) => value,
            ContainerRelativeLength::Cqb(value) => value,
            ContainerRelativeLength::Cqmin(value) => value,
            ContainerRelativeLength::Cqmax(value) => value,
        }
    }

    pub fn unit(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.container_relative_length {
            ContainerRelativeLength::Cqw(_) => ruby.intern("cqw"),
            ContainerRelativeLength::Cqh(_) => ruby.intern("cqh"),
            ContainerRelativeLength::Cqi(_) => ruby.intern("cqi"),
            ContainerRelativeLength::Cqb(_) => ruby.intern("cqb"),
            ContainerRelativeLength::Cqmin(_) => ruby.intern("cqmin"),
            ContainerRelativeLength::Cqmax(_) => ruby.intern("cqmax"),
        }
    }
}

#[magnus::wrap(class = "Yass::Declarations::Length::CharacterWidth")]
pub struct YCharacterWidthLength {
    character_width_length: CharacterWidth
}

impl YCharacterWidthLength {
    pub fn new(character_width_length: CharacterWidth) -> Self {
        Self { character_width_length }
    }

    pub fn value(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> i32 {
        rb_self.character_width_length.0
    }
}
