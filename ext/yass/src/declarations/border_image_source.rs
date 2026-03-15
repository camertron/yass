use style::values::specified::Image;

#[magnus::wrap(class = "Yass::Declarations::BorderImageSource")]
pub struct YBorderImageSource {
  image: Image
}

impl YBorderImageSource {
  pub fn new(image: Image) -> Self {
    Self { image }
  }
}
