use crate::lib::util::err::rsm_error::RSMError;

pub enum FrameAreaDisposal {
  None = 0,
  Background = 1,
  Previous = 2,
}

impl TryFrom<u8> for FrameAreaDisposal {
  type Error = RSMError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::None),
      1 => Ok(Self::Background),
      2 => Ok(Self::Previous),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
