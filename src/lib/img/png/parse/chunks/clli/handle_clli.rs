use crate::lib::{
  img::png::parse::chunks::clli::png_light_level::ContentLightLevel, util::err::rsm_error::RSMError,
};

/// Handle cLLI (Content Light Level Information) chunk
pub(crate) fn handle_clli(data: [u8; 8]) -> Result<Option<ContentLightLevel>, RSMError> {
  let max_cll: Option<f32> = get_light_level(&data[0..4]);
  let max_fall: Option<f32> = get_light_level(&data[4..8]);

  if max_cll.is_none() || max_fall.is_none() {
    return Ok(None);
  }

  Ok(Some(ContentLightLevel {
    max_cll: max_cll.unwrap(),
    max_fall: max_fall.unwrap(),
  }))
}

/// Handle getting a light value
fn get_light_level(data: &[u8]) -> Option<f32> {
  let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
    return None;
  };
  let value: f32 = u32::from_be_bytes(bytes) as f32;
  let level: f32 = value / 10_000.0;

  Some(level)
}
