use crate::lib::{
  img::png::parse::{chunks::clli::png_light_level::ContentLightLevel, values::png_int::PNGInt},
  util::err::rsm_error::RSMError,
};

/// Handle `cLLI` (Content Light Level Information) chunk
pub(crate) fn handle_clli(data: [u8; 8]) -> Result<Option<ContentLightLevel>, RSMError> {
  let max_cll: PNGInt = data[0..4].try_into()?;
  let max_fall: PNGInt = data[4..8].try_into()?;

  Ok(Some(ContentLightLevel {
    max_cll: (*max_cll as f32) / 10_000.0,
    max_fall: (*max_fall as f32) / 10_000.0,
  }))
}
