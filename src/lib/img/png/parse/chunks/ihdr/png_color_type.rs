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
  use proptest::prelude::*;
  use std::{array::IntoIter, iter::Chain, ops::RangeInclusive};

  /// Test if u8 values map correctly to color types ypes
  #[test]
  fn test_color_type_mapping() {
    let color_types: [ColorType; 5] = [
      ColorType::Greyscale,
      ColorType::Truecolor,
      ColorType::IndexedColor,
      ColorType::GreyscaleAlpha,
      ColorType::TruecolorAlpha,
    ];

    // This range is not continuous, which is a combination of 0 and the range 2 to 5
    let range: Chain<IntoIter<usize, 1>, RangeInclusive<usize>> = [0].into_iter().chain(2..=5);
    for (num, real_type) in range.zip(color_types) {
      let r#type: ColorType = (num as u8).try_into().unwrap();
      assert_eq!(r#type, real_type);
    }
  }

  proptest! {
    /// Test random u8 values
    #[test]
    fn test_random_color_type_values(val in any::<u8>()) {
      let r#type: Result<ColorType, _> = val.try_into();
      if val == 0 || (2..=5).contains(&val) {
        assert!(r#type.is_ok());
      } else {
        assert!(r#type.is_err())
      }
    }
  }
}
