use crate::lib::util::err::rsm_error::RSMError;

/// PNG Bit depth
#[derive(Debug)]
pub enum BitDepth {
  D1 = 1,
  D2 = 2,
  D4 = 4,
  D8 = 8,
  D16 = 16,
}

impl TryFrom<u8> for BitDepth {
  type Error = RSMError;

  fn try_from(depth: u8) -> Result<Self, Self::Error> {
    match depth {
      1 => Ok(Self::D1),
      2 => Ok(Self::D2),
      4 => Ok(Self::D4),
      8 => Ok(Self::D8),
      16 => Ok(Self::D16),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
