use crate::lib::util::err::rsm_error::RSMError;

/// sRGB rendering intent
pub enum RenderingIntent {
  Perceptual = 0,
  RelativeColorimetric = 1,
  Saturation = 2,
  AbsoluteColorimetric = 3,
}

impl TryFrom<u8> for RenderingIntent {
  type Error = RSMError;

  fn try_from(intent: u8) -> Result<Self, Self::Error> {
    match intent {
      0 => Ok(Self::Perceptual),
      1 => Ok(Self::RelativeColorimetric),
      2 => Ok(Self::Saturation),
      3 => Ok(Self::AbsoluteColorimetric),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
