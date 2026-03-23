use crate::lib::img::png::parse::chunks::srgb::png_rendering_intent::RenderingIntent;

/// Represents PNG data
#[derive(Default)]
pub struct PNGData {
  pub gamma: Option<f32>,
  pub rendering_intent: Option<RenderingIntent>,
}

impl PNGData {
  pub fn new() -> Self {
    Self {
      gamma: None,
      rendering_intent: None,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::lib::img::png::img::png_data::PNGData;

  /// Check if PNG data can be instantiated
  #[test]
  fn test_png_data_instantiation() {
    PNGData::new();
  }
}
