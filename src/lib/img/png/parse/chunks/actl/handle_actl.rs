use crate::lib::{
  img::png::parse::{
    chunks::actl::png_animation_control::AnimationControl, values::png_int::PNGInt,
  },
  util::err::rsm_error::RSMError,
};

/// Handle `acTL` (Animation Control Chunk) chunk
pub(crate) fn handle_actl(data: [u8; 8]) -> Result<Option<AnimationControl>, RSMError> {
  let frames: PNGInt = data[0..4].try_into()?;
  if *frames == 0 {
    return Ok(None);
  }

  let plays: PNGInt = data[4..8].try_into()?;
  Ok(Some(AnimationControl { frames, plays }))
}
