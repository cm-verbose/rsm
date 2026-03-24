use std::ops::Range;

use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    img::png_data::PNGData,
    parse::chunks::{
      actl::png_animation_control::AnimationControl, chrm::png_chromacities::Chromacities,
      cicp::png_code_points::CodePoints, clli::png_light_level::ContentLightLevel,
      iccp::png_iccp_profile::ICCPProfile, ihdr::png_header::PNGHeader,
      mdcv::png_color_volume::ColorVolume, phys::png_physical_dimensions::PhysicalDimensions,
      srgb::png_rendering_intent::RenderingIntent, text::png_text::Text,
      time::png_time::ModifiedTime,
    },
  },
  util::err::rsm_error::RSMError,
};

/// Parse chunks into meaningful data
pub struct PNGParser {
  pub(super) animation_control: Option<AnimationControl>,
  pub(super) background_bytes: Option<Vec<u8>>,
  pub(super) chromacities: Option<Chromacities>,
  pub(super) code_points: Option<CodePoints>,
  pub(super) color_volume: Option<ColorVolume>,
  pub(super) compressed_text_entries: Option<Vec<Text>>,
  pub(super) iccp_profile: Option<ICCPProfile>,
  pub(super) idat_bytes: Vec<u8>,
  pub(super) image_header: Option<PNGHeader>,
  pub(super) gamma: Option<f32>,
  pub(super) light_level: Option<ContentLightLevel>,
  pub(super) modified_time: Option<ModifiedTime>,
  pub(super) palette: Option<Vec<[u8; 3]>>,
  pub(super) physical_dimensions: Option<PhysicalDimensions>,
  pub(super) rendering_intent: Option<RenderingIntent>,
  pub(super) significant_bits: Option<Vec<u8>>,
  pub(super) text_entries: Option<Vec<Text>>,
  pub(super) transparency_bytes: Option<Vec<u8>>,

  pub(super) parsed_idat: bool,
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
      code_points: None,
      color_volume: None,
      compressed_text_entries: None,
      iccp_profile: None,
      image_header: None,
      idat_bytes: Vec::new(),
      gamma: None,
      light_level: None,
      modified_time: None,
      palette: None,
      physical_dimensions: None,
      rendering_intent: None,
      significant_bits: None,
      text_entries: None,
      transparency_bytes: None,

      parsed_idat: false,
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

  /// Read text from bytes (Latin-1)
  pub(super) fn read_text(bytes: &[u8]) -> Result<String, RSMError> {
    Ok(bytes.iter().map(|&b| b as char).collect())
  }
}
