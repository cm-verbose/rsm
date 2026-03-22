use crate::lib::img::png::{chunk::png_chunk::Chunk, img::png_data::PNGData};

/// Represents a PNG image
#[allow(unused)]
#[derive(Default)]
pub struct PNGImage<'i> {
  pub(in super::super) chunks: Vec<Chunk<'i>>,
  data: Option<PNGData>,
}

impl<'i> PNGImage<'i> {
  pub fn new() -> Self {
    PNGImage::default()
  }
}
