/// Represents PNG data
pub struct PNGData {
  gamma: Option<f32>,
}

impl PNGData {
  pub fn new() -> Self {
    Self { gamma: None }
  }
}
