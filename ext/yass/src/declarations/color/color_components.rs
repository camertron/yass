use magnus::{Ruby, typed_data};
use style::color::ColorComponents;

#[magnus::wrap(class = "Yass::Declarations::Color::ColorComponents")]
pub struct YColorComponents {
    color_components: ColorComponents,
}

impl YColorComponents {
    pub fn new(color_components: ColorComponents) -> Self {
        Self { color_components }
    }

    pub fn c0(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.color_components.0
    }

    pub fn c1(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.color_components.1
    }

    pub fn c2(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.color_components.2
    }
}
