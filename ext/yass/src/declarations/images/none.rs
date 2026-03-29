#[magnus::wrap(class = "Yass::Declarations::Image::None")]
pub struct YImageNone {}

impl YImageNone {
    pub fn new() -> Self {
        Self { }
    }
}
