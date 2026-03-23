use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::clli::png_light_level::ContentLightLevel, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle cLLI (Content Light Level Information) chunk
  pub(in super::super::super) fn handle_clli(
    &self,
    chunk: &Chunk,
  ) -> Result<Option<ContentLightLevel>, RSMError> {
    let Ok::<[u8; 8], _>(data) = chunk.data.try_into() else {
      return Err(RSMError::InvalidContent);
    };

    let max_cll: Option<f32> = self.get_light_level(&data[0..4]);
    let max_fall: Option<f32> = self.get_light_level(&data[4..8]);

    if max_cll.is_none() || max_fall.is_none() {
      return Ok(None);
    }

    Ok(Some(ContentLightLevel {
      max_cll: max_cll.unwrap(),
      max_fall: max_fall.unwrap(),
    }))
  }

  /// Handle getting a light value
  fn get_light_level(&self, data: &[u8]) -> Option<f32> {
    let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
      return None;
    };
    let value: f32 = u32::from_be_bytes(bytes) as f32;
    let level: f32 = value / 10_000.0;

    Some(level)
  }
}
