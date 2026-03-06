use crate::lib::util::err::rsm_error::RSMError;

/// PNG Bit depth
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
  use crate::lib::img::png::parse::chunks::ihdr::png_bit_depth::BitDepth;

  #[test]
  fn test_depth_mapping() {
    let depth_1: BitDepth = 1.try_into().unwrap();
    assert_eq!(depth_1, BitDepth::D1);

    let depth_2: BitDepth = 2.try_into().unwrap();
    assert_eq!(depth_2, BitDepth::D2);

    let depth_3: BitDepth = 4.try_into().unwrap();
    assert_eq!(depth_3, BitDepth::D4);

    let depth_4: BitDepth = 8.try_into().unwrap();
    assert_eq!(depth_4, BitDepth::D8);

    let depth_16: BitDepth = 16.try_into().unwrap();
    assert_eq!(depth_16, BitDepth::D16);
  }
}
