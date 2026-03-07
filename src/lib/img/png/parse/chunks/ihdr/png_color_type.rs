use crate::lib::util::err::rsm_error::RSMError;

/// Color type used to render the image
#[derive(Debug, PartialEq, Clone, Copy)]
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

#[cfg(test)]
mod tests {
  use crate::lib::img::png::parse::chunks::ihdr::png_color_type::ColorType;

  #[test]
  fn test_color_tupe_mapping() {
    let greyscale: ColorType = 0.try_into().unwrap();
    assert_eq!(greyscale, ColorType::Greyscale);

    let truecolor: ColorType = 2.try_into().unwrap();
    assert_eq!(truecolor, ColorType::Truecolor);

    let indexed_color: ColorType = 3.try_into().unwrap();
    assert_eq!(indexed_color, ColorType::IndexedColor);

    let greyscale_alpha: ColorType = 4.try_into().unwrap();
    assert_eq!(greyscale_alpha, ColorType::GreyscaleAlpha);

    let truecolor_alpha: ColorType = 5.try_into().unwrap();
    assert_eq!(truecolor_alpha, ColorType::TruecolorAlpha);
  }
}
