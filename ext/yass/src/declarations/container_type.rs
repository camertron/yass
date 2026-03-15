use style::values::computed::ContainerType;

#[magnus::wrap(class = "Yass::Declarations::ContainerType")]
pub struct YContainerType {
  container_type: ContainerType
}

impl YContainerType {
  pub fn new(container_type: ContainerType) -> Self {
    Self { container_type }
  }
}
