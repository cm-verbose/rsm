use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::srgb::png_rendering_intent::RenderingIntent, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle sRGB (Standard RGB color space) chunk
  pub(in super::super::super) fn handle_srgb(
    &self,
    chunk: &Chunk<'_>,
  ) -> Result<RenderingIntent, RSMError> {
    if chunk.data.len() != 1 {
      return Err(RSMError::InvalidLength);
    }
    let rendering_intent_value: u8 = chunk.data[0];
    let intent: RenderingIntent = rendering_intent_value.try_into()?;
    Ok(intent)
  }
}
