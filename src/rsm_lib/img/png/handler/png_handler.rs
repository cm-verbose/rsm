use crate::rsm_lib::img::png::{chunk::png_chunk::Chunk, image::png_image::PNGImage};
use std::borrow::Cow;

/// Handles manipulating `.png` data streams provided from either files or a
/// raw sequence of bytes.
pub struct PNGHandler<'b> {
  pub bytes: Cow<'b, [u8]>,
  pub chunks: Vec<Chunk>,
  pub idat_bytes: Vec<u8>,
  pub image: PNGImage,
  pub ptr: usize,
}

impl<'b> Default for PNGHandler<'b> {
  fn default() -> Self {
    Self {
      bytes: Cow::Borrowed(&[]),
      chunks: Vec::new(),
      idat_bytes: Vec::new(),
      image: PNGImage::default(),
      ptr: 0,
    }
  }
}

impl<'b> PNGHandler<'b> {
  /// Creates a new PNG image handler
  pub fn new() -> Self {
    Self::default()
  }
}
