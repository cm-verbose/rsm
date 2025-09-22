use crate::rsm::img::png::image::png_image::PNGImage;
use std::ptr;

/// Handles reading PNG images
pub(in super::super) struct PNGReader<'a> {
  pub bytes: &'a [u8],
  pub idat_bytes: Vec<u8>,
  pub image: PNGImage,
  pub ptr: *const u8,
  pub ptr_end: *const u8,
  pub used: bool,
}

impl<'a> Default for PNGReader<'a> {
  fn default() -> Self {
    Self {
      bytes: &[],
      idat_bytes: Vec::new(),
      image: PNGImage::default(),
      ptr: ptr::null(),
      ptr_end: ptr::null(),
      used: false,
    }
  }
}

impl<'a> PNGReader<'a> {
  pub fn new() -> Self {
    Self::default()
  }
}
