use crate::lib::util::err::rsm_error::RSMError;

/// sRGB rendering intent
#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
  use crate::lib::img::png::parse::chunks::srgb::png_rendering_intent::RenderingIntent;
  use proptest::prelude::*;

  /// Test all rendering intents
  #[test]
  fn test_rendering_intents_mappings() {
    type Intent = RenderingIntent;

    let intents: [Intent; 4] = [
      Intent::Perceptual,
      Intent::RelativeColorimetric,
      Intent::Saturation,
      Intent::AbsoluteColorimetric,
    ];

    for i in 0..4 {
      let intent: Intent = (i as u8).try_into().unwrap();
      assert_eq!(intent, intents[i])
    }
  }

  proptest! {
    /// Test all other invalid intents 4 to u8::MAX (255)
    #[test]
    fn test_invalid_intents(method in 4..=u8::MAX) {
      let intent: Result<RenderingIntent, _> = method.try_into();
      assert!(intent.is_err())
    }
  }
}
