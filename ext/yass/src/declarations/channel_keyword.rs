use magnus::{Ruby, typed_data, value::Id};
use style::color::parsing::ChannelKeyword;

#[magnus::wrap(class = "Yass::Declarations::ChannelKeyword")]
pub struct YChannelKeyword {
    channel_keyword: ChannelKeyword
}

impl YChannelKeyword {
    pub fn new(channel_keyword: ChannelKeyword) -> Self {
        Self { channel_keyword }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.channel_keyword {
            ChannelKeyword::Alpha => ruby.intern("alpha"),
            ChannelKeyword::A => ruby.intern("a"),
            ChannelKeyword::B => ruby.intern("b"),
            ChannelKeyword::C => ruby.intern("c"),
            ChannelKeyword::G => ruby.intern("g"),
            ChannelKeyword::H => ruby.intern("h"),
            ChannelKeyword::L => ruby.intern("l"),
            ChannelKeyword::R => ruby.intern("r"),
            ChannelKeyword::S => ruby.intern("s"),
            ChannelKeyword::W => ruby.intern("w"),
            ChannelKeyword::X => ruby.intern("x"),
            ChannelKeyword::Y => ruby.intern("y"),
            ChannelKeyword::Z => ruby.intern("z"),
        }
    }
}
