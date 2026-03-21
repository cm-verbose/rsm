/// Represents PNG data
pub struct PNGData {
  pub gamma: Option<f32>,
}

impl PNGData {
  pub fn new() -> Self {
    Self { gamma: None }
  }
}

#[cfg(test)]
mod test {
  use crate::lib::img::png::img::png_data::PNGData;

  #[test]
  fn test_png_data_instantiation() {
    PNGData::new();
  }
}