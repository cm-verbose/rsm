use crate::lib::{
  img::png::parse::chunks::actl::png_animation_control::AnimationControl,
  util::err::rsm_error::RSMError,
};

/// Handle acTL (Animation Control Chunk) chunk
pub(crate) fn handle_actl(data: [u8; 8]) -> Result<Option<AnimationControl>, RSMError> {
  let frames: u32 = read_actl_value(&data[0..4])?;
  if frames == 0 {
    return Ok(None);
  }
  let plays: u32 = read_actl_value(&data[4..8])?;
  Ok(Some(AnimationControl { frames, plays }))
}

/// Read an `acTL` value
fn read_actl_value(data: &[u8]) -> Result<u32, RSMError> {
  let Ok::<[u8; 4], _>(data) = data.try_into() else {
    return Err(RSMError::InvalidLength);
  };

  let value = u32::from_be_bytes(data);
  if value > i32::MAX as u32 {
    return Err(RSMError::OutOfBounds);
  }
  Ok(value)
}
