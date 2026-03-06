use crate::lib::util::err::rsm_error::RSMError;

/// Color type used to render the image
#[derive(Debug)]
pub enum ColorType {
  Greyscale = 0,
  Truecolor = 2,
  IndexedColor = 3,
  GreyscaleAlpha = 4,
  TruecolorAlpha = 5,
}

impl TryFrom<u8> for ColorType {
  type Error = RSMError;

  fn try_from(color_type: u8) -> Result<Self, Self::Error> {
    match color_type {
      0 => Ok(Self::Greyscale),
      2 => Ok(Self::Truecolor),
      3 => Ok(Self::IndexedColor),
      4 => Ok(Self::GreyscaleAlpha),
      5 => Ok(Self::TruecolorAlpha),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
