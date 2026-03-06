use crate::lib::util::err::rsm_error::RSMError;

/// Method in which the image was filtered
#[derive(Debug)]
pub enum FilterMethod {
  Method0 = 0,
}

impl TryFrom<u8> for FilterMethod {
  type Error = RSMError;

  fn try_from(method: u8) -> Result<Self, Self::Error> {
    match method {
      0 => Ok(Self::Method0),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
