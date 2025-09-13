use crate::rsm_lib::img::png::image::png_image::PNGImage;

/// Handles reading PNG datastreams
pub(in super::super) struct PNGReader<'a> {
  pub bytes: &'a [u8],
  pub idat_bytes: Vec<u8>,
  pub is_used: bool,
  pub image: PNGImage,
  pub ptr: usize,
}

impl<'a> Default for PNGReader<'a> {
  fn default() -> Self {
    Self {
      bytes: &[],
      idat_bytes: Vec::new(),
      is_used: false,
      image: PNGImage::default(),
      ptr: 0,
    }
  }
}

impl<'a> PNGReader<'a> {
  /// Creates a new instance of a reader
  pub(in super::super) fn new() -> Self {
    Self::default()
  }
}
