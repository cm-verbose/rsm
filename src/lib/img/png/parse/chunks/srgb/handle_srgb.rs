use crate::lib::{
  img::png::parse::chunks::srgb::png_rendering_intent::RenderingIntent,
  util::err::rsm_error::RSMError,
};

/// Handle sRGB (Standard RGB color space) chunk
pub(crate) fn handle_srgb(data: [u8; 1]) -> Result<RenderingIntent, RSMError> {
  let rendering_intent_value: u8 = data[0];
  let intent: RenderingIntent = rendering_intent_value.try_into()?;
  Ok(intent)
}
