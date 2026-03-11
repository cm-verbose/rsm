use crate::lib::img::png::{chunk::png_chunk::Chunk, img::png_data::PNGData};

/// Represents a PNG image
pub struct PNGImage<'i> {
  pub(in super::super) chunks: Vec<Chunk<'i>>,
  data: Option<PNGData>,
}

impl<'i> Default for PNGImage<'i> {
  fn default() -> Self {
    Self {
      chunks: Vec::new(),
      data: None,
    }
  }
}

impl<'i> PNGImage<'i> {
  pub fn new() -> Self {
    PNGImage::default()
  }
}
