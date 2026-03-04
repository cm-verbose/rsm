use crate::lib::img::png::chunk::png_chunk::Chunk;

/// Represents a PNG image
#[derive(Default)]
pub struct PNGImage<'i> {
  chunks: Vec<Chunk<'i>>,
}

impl<'i> PNGImage<'i> {
  pub fn new() -> Self {
    PNGImage::default()
  }
}
