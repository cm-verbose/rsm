use std::ops::Range;

use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    img::png_data::PNGData,
    parse::chunks::{
      actl::png_animation_control::AnimationControl, chrm::png_chromacities::Chromacities,
      ihdr::png_header::PNGHeader, phys::png_physical_dimensions::PhysicalDimensions,
      srgb::png_rendering_intent::RenderingIntent, text::png_text::Text,
    },
  },
  util::err::rsm_error::RSMError,
};

/// Parse chunks into meaningful data
pub struct PNGParser {
  pub(super) animation_control: Option<AnimationControl>,
  pub(super) background_bytes: Option<Vec<u8>>,
  pub(super) chromacities: Option<Chromacities>,
  pub(super) idat_bytes: Vec<u8>,
  pub(super) image_header: Option<PNGHeader>,
  pub(super) gamma: Option<f32>,
  pub(super) palette: Option<Vec<[u8; 3]>>,
  pub(super) parsed_idat: bool,
  pub(super) physical_dimensions: Option<PhysicalDimensions>,
  pub(super) rendering_intent: Option<RenderingIntent>,
  pub(super) significant_bits: Option<Vec<u8>>,
  pub(super) text_entries: Option<Vec<Text>>,
  pub(super) transparency_bytes: Option<Vec<u8>>,
}

impl Default for PNGParser {
  fn default() -> Self {
    Self::new()
  }
}

impl PNGParser {
  pub fn new() -> Self {
    Self {
      animation_control: None,
      background_bytes: None,
      chromacities: None,
      image_header: None,
      idat_bytes: Vec::new(),
      gamma: None,
      palette: None,
      parsed_idat: false,
      physical_dimensions: None,
      rendering_intent: None,
      significant_bits: None,
      text_entries: None,
      transparency_bytes: None,
    }
  }

  /// Gets bytes from a range
  pub(super) fn get_bytes<'a>(
    range: Range<usize>,
    chunk: &Chunk<'a>,
  ) -> Result<&'a [u8], RSMError> {
    let Some(bytes) = chunk.data.get(range) else {
      return Err(RSMError::NotEnoughContent);
    };
    Ok(bytes)
  }

  /// Map parsed data to for usage
  pub(super) fn map_png_data(&self) -> PNGData {
    let data: PNGData = PNGData {
      gamma: self.gamma,
      rendering_intent: self.rendering_intent.clone(),
    };
    data
  }
}
