use style::values::specified::ImplicitGridTracks;

#[magnus::wrap(class = "Yass::Declarations::GridAutoRows")]
pub struct YGridAutoRows {
  implicit_grid_tracks: ImplicitGridTracks
}

impl YGridAutoRows {
  pub fn new(implicit_grid_tracks: ImplicitGridTracks) -> Self {
    Self { implicit_grid_tracks }
  }
}
