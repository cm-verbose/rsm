use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::actl::png_animation_control::AnimationControl, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle acTL (Animation Control Chunk) chunk
  pub(in super::super::super) fn handle_actl(
    &self,
    chunk: &Chunk,
  ) -> Result<Option<AnimationControl>, RSMError> {
    let Ok::<[u8; 8], _>(data) = chunk.data.try_into() else {
      return Err(RSMError::InvalidContent);
    };
    let frames: u32 = self.read_actl_value(&data[0..4])?;
    if frames == 0 {
      return Ok(None);
    }
    let plays: u32 = self.read_actl_value(&data[4..8])?;
    Ok(Some(AnimationControl { frames, plays }))
  }

  /// Read an acTL value
  fn read_actl_value(&self, data: &[u8]) -> Result<u32, RSMError> {
    let Ok::<[u8; 4], _>(data) = data.try_into() else {
      return Err(RSMError::InvalidLength);
    };

    let value = u32::from_be_bytes(data);
    if value > i32::MAX as u32 {
      return Err(RSMError::InvalidLength);
    }
    Ok(value)
  }
}
