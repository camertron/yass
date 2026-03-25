use magnus::{Ruby, typed_data};
use style::values::specified::Resolution;

#[magnus::wrap(class = "Yass::Declarations::Resolution")]
pub struct YResolution {
    resolution: Resolution
}

impl YResolution {
    pub fn new(resolution: Resolution) -> Self {
        Self { resolution }
    }

    pub fn dpi(_ruby: &Ruby, rb_self: typed_data::Obj<Self>) -> f32 {
        rb_self.resolution.dpi()
    }
}
