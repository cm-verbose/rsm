use crate::lib::util::err::rsm_error::RSMError;

/// PNG Bit depth
#[derive(Debug, PartialEq, Clone, Copy)]
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
  use proptest::prelude::*;
  use std::iter::successors;

  /// Test bit depth mapping from u8 values
  #[test]
  fn test_depth_mapping() {
    type Depth = BitDepth;
    let depths: [Depth; 5] = [Depth::D1, Depth::D2, Depth::D4, Depth::D8, Depth::D16];
    let powers = successors(Some(1), |&prev| Some(prev * 2)).take(5);

    for (num, real_depth) in powers.zip(depths) {
      let depth: Depth = num.try_into().unwrap();
      assert_eq!(depth, real_depth)
    }
  }

  proptest! {
    /// Test random bit depths
    #[test]
    fn test_random_depths(val in any::<u8>()) {
      let result: Result<BitDepth, _> = val.try_into();
      if val.is_power_of_two() && val <= 16 {
        prop_assert!(result.is_ok());
      } else {
        prop_assert!(result.is_err())
      }
    }
  }
}
