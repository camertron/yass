use style::values::specified::ImplicitGridTracks;

#[magnus::wrap(class = "Yass::Declarations::GridAutoColumns")]
pub struct YGridAutoColumns {
  implicit_grid_tracks: ImplicitGridTracks
}

impl YGridAutoColumns {
  pub fn new(implicit_grid_tracks: ImplicitGridTracks) -> Self {
    Self { implicit_grid_tracks }
  }
}
