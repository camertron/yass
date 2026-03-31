#[magnus::wrap(class = "Yass::Declarations::Color::Auto")]
pub struct YAutoColor {}

impl YAutoColor {
    pub fn new() -> Self {
        Self { }
    }
}
