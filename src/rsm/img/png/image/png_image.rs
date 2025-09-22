use crate::rsm::color::colors::rgba::RGBA;

#[derive(Debug)]
pub struct PNGImage {
  pub height: Option<u32>,
  pub width: Option<u32>,
  pub bit_depth: Option<u8>,
  pub color_type: Option<u8>,
  pub compression_method: Option<u8>,
  pub filter_method: Option<u8>,
  pub interlace_method: Option<u8>,

  pub pixels: Option<Vec<RGBA>>,
}

impl Default for PNGImage {
  fn default() -> Self {
    Self {
      height: None,
      width: None,
      bit_depth: None,
      color_type: None,
      compression_method: None,
      filter_method: None,
      interlace_method: None,
      pixels: None,
    }
  }
}

impl PNGImage {
  pub fn new() -> Self {
    Self::default()
  }
}
