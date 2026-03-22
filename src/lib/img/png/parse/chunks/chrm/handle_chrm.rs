use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::chrm::png_chromacities::Chromacities, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle cHRM (Primary chromaticities and white point)
  pub(in super::super::super) fn handle_chrm(
    &self,
    chunk: &Chunk<'_>,
  ) -> Result<Option<Chromacities>, RSMError> {
    if let Ok::<[u8; 32], _>(data) = chunk.data.try_into() {
      let mut chromacities_values: [(f32, f32); 4] = [(0.0, 0.0); 4];

      for (i, chromacity) in chromacities_values.iter_mut().enumerate() {
        // Start offset
        let s_o: usize = i * 8;
        let cx: &[u8] = &data[s_o..(s_o + 4)];
        let cy: &[u8] = &data[(s_o + 4)..(s_o + 8)];

        let x: u32 = u32::from_be_bytes(cx.try_into().unwrap());
        let y: u32 = u32::from_be_bytes(cy.try_into().unwrap());

        if x > (i32::MAX as u32) || y > (i32::MAX as u32) {
          return Ok(None);
        }
        *chromacity = ((x as f32) / 100000.0, (y as f32) / 100000.0)
      }

      let chromacities: Chromacities = Chromacities {
        white_point: chromacities_values[0],
        red: chromacities_values[1],
        green: chromacities_values[2],
        blue: chromacities_values[3],
      };
      Ok(Some(chromacities))
    } else {
      Err(RSMError::InvalidLength)
    }
  }
}

#[cfg(test)]
mod tests {
  use proptest::{
    collection::vec,
    prelude::{Strategy, any},
    prop_assert, prop_oneof, proptest,
  };

  use crate::lib::img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::png_parser::PNGParser,
  };

  fn filter_vec_not_size_32() -> impl Strategy<Value = Vec<u8>> {
    prop_oneof![vec(any::<u8>(), 0..32), vec(any::<u8>(), 33..40)]
  }

  fn invalid_chrm_stragegy() -> impl Strategy<Value = [u8; 32]> {
    any::<[u32; 8]>()
      .prop_filter("Invalid chromacity", |values| {
        values.iter().any(|&v| v > i32::MAX as u32)
      })
      .prop_map(|values| {
        let mut bytes: [u8; 32] = [0; 32];

        for (i, &val) in values.iter().enumerate() {
          bytes[i * 4..(i * 4) + 4].copy_from_slice(&val.to_be_bytes());
        }
        bytes
      })
  }

  fn valid_chrm_strategy() -> impl Strategy<Value = [u8; 32]> {
    vec(0..=i32::MAX as u32, 8).prop_map(|values| {
      let mut bytes = [0u8; 32];

      for (i, &val) in values.iter().enumerate() {
        bytes[i * 4..(i * 4) + 4].copy_from_slice(&val.to_be_bytes());
      }
      bytes
    })
  }

  proptest! {
    #[test]
    fn test_chrm_invalid_data_length(data in filter_vec_not_size_32()) {
      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType::cHRM,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let res = parser.handle_chrm(&chunk);
      prop_assert!(res.is_err())
    }

    #[test]
    fn test_chrm_invalid_data(data in invalid_chrm_stragegy()) {
      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType::cHRM,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let res = parser.handle_chrm(&chunk).unwrap();
      prop_assert!(res.is_none())
    }

    #[test]
    fn test_chrm_valid_data(data in valid_chrm_strategy()) {
      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType::cHRM,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let res = parser.handle_chrm(&chunk).unwrap();
      prop_assert!(res.is_some())
    }
  }
}
