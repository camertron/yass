use style::values::specified::Image;

#[magnus::wrap(class = "Yass::Declarations::ListStyleImage")]
pub struct YListStyleImage {
  image: Image
}

impl YListStyleImage {
  pub fn new(image: Image) -> Self {
    Self { image }
  }
}
