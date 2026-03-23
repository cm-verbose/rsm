use crate::lib::util::err::rsm_error::RSMError;

pub enum AlphaBlend {
  Source = 0,
  Over = 1,
}

impl TryFrom<u8> for AlphaBlend {
  type Error = RSMError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Self::Source),
      1 => Ok(Self::Over),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
