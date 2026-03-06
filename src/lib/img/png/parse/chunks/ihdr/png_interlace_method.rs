use crate::lib::util::err::rsm_error::RSMError;

/// Interlace method
#[derive(Debug)]
pub enum InterlaceMethod {
  Null = 0,
  Adam7 = 1,
}

impl TryFrom<u8> for InterlaceMethod {
  type Error = RSMError;

  fn try_from(method: u8) -> Result<Self, Self::Error> {
    match method {
      0 => Ok(Self::Null),
      1 => Ok(Self::Adam7),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
