use style::values::computed::ContainerName;

#[magnus::wrap(class = "Yass::Declarations::ContainerName")]
pub struct YContainerName {
  container_name: ContainerName
}

impl YContainerName {
  pub fn new(container_name: ContainerName) -> Self {
    Self { container_name }
  }
}
