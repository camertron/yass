use magnus::{Ruby, typed_data, value::Id};
use style::values::computed::ImageRendering;

#[magnus::wrap(class = "Yass::Declarations::ImageRendering")]
pub struct YImageRendering {
    image_rendering: ImageRendering
}

impl YImageRendering {
    pub fn new(image_rendering: ImageRendering) -> Self {
        Self { image_rendering }
    }

    pub fn value(ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> Id {
        match rb_self.image_rendering {
            ImageRendering::Auto => ruby.intern("auto"),
            ImageRendering::CrispEdges => ruby.intern("crisp_edges"),
            ImageRendering::Pixelated => ruby.intern("pixelated"),
        }
    }
}
