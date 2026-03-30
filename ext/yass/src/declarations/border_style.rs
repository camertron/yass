use magnus::{Ruby, value::Id};
use style::values::computed::BorderStyle;

pub fn border_style_to_id(border_style: BorderStyle, ruby: &Ruby) -> Id {
    match border_style {
        BorderStyle::None => ruby.intern("none"),
        BorderStyle::Hidden => ruby.intern("hidden"),
        BorderStyle::Dotted => ruby.intern("dotted"),
        BorderStyle::Dashed => ruby.intern("dashed"),
        BorderStyle::Solid => ruby.intern("solid"),
        BorderStyle::Double => ruby.intern("double"),
        BorderStyle::Groove => ruby.intern("groove"),
        BorderStyle::Ridge => ruby.intern("ridge"),
        BorderStyle::Inset => ruby.intern("inset"),
        BorderStyle::Outset => ruby.intern("outset"),
    }
}
