use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::srgb::png_rendering_intent::RenderingIntent, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle sRGB (Standard RGB color space) chunk
  pub(in super::super::super) fn handle_srgb(
    &self,
    chunk: &Chunk<'_>,
  ) -> Result<RenderingIntent, RSMError> {
    if chunk.data.len() != 1 {
      return Err(RSMError::InvalidLength);
    }
    let rendering_intent_value: u8 = chunk.data[0];
    let intent: RenderingIntent = rendering_intent_value.try_into()?;
    Ok(intent)
  }
}

#[cfg(test)]
mod test {
  use crate::lib::img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::{chunks::srgb::png_rendering_intent::RenderingIntent, png_parser::PNGParser},
  };
  use proptest::{bits::u8, collection::vec, prelude::*};

  /// Test all valid rendering intents
  #[test]
  fn tests_valid_intents() {
    type Intent = RenderingIntent;
    let intents: [Intent; 4] = [
      Intent::Perceptual,
      Intent::RelativeColorimetric,
      Intent::Saturation,
      Intent::AbsoluteColorimetric,
    ];
    let parser: PNGParser = PNGParser::new();

    for initial_intent in intents {
      let num: u8 = initial_intent.clone() as u8;
      let data: &[u8] = &[num];
      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType::sRGB,
        length: data.len() as u32,
        data,
        crc: [0, 0, 0, 0],
      };
      let intent = parser.handle_srgb(&chunk).unwrap();
      assert_eq!(initial_intent, intent)
    }
  }

  proptest! {
    /// Invalidate lengths above 1 for the sRGB chunk
    #[test]
    fn invalidate_srgb_length(ref data in vec(any::<u8>(), 2..100)) {
      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType::sRGB,
        length: data.len() as u32,
        data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let result = parser.handle_srgb(&chunk);
      assert!(result.is_err());
    }

    /// Test invalid srgb values
    #[test]
    fn test_invalid_srgb(intent in 4..=u8::MAX) {
      let data: &[u8] = &[intent];

      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType:: sRGB,
        length: data.len() as u32,
        data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let result = parser.handle_srgb(&chunk);
      assert!(result.is_err());
    }
  }
}
