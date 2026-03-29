#[magnus::wrap(class = "Yass::Declarations::Color::CurrentColor")]
pub struct YCurrentColor {}

impl YCurrentColor {
    pub fn new() -> Self {
        Self { }
    }
}
