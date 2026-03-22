use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// handle gAMA (Image gamma) chunk
  pub(in super::super) fn handle_gama(&self, chunk: &Chunk<'_>) -> Result<f32, RSMError> {
    if let Ok::<&[u8; 4], _>(bytes) = chunk.data.try_into() {
      let value: u32 = u32::from_be_bytes(*bytes);
      if value > (i32::MAX as u32) || value == 0 {
        return Err(RSMError::InvalidLength);
      }

      let gamma_value: f32 = value as f32 / 100_000.0;
      Ok(gamma_value)
    } else {
      Err(RSMError::InvalidContent)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::png_parser::PNGParser,
  };
  use proptest::{
    collection::vec, prelude::any, prop_assert, prop_oneof, proptest, strategy::Strategy,
  };

  fn filter_vec_not_size_4() -> impl Strategy<Value = Vec<u8>> {
    prop_oneof![vec(any::<u8>(), 0..4), vec(any::<u8>(), 5..10)]
  }

  fn filter_above_i32() -> impl Strategy<Value = u32> {
    ((i32::MAX as u32) + 1)..=u32::MAX
  }

  #[test]
  fn test_gama_value_zero() {
    let chunk = Chunk {
      r#type: ChunkType::gAMA,
      length: 4,
      data: &[0, 0, 0, 0],
      crc: [0, 0, 0, 0],
    };
    let parser: PNGParser = PNGParser::new();
    let res = parser.handle_gama(&chunk);

    assert!(res.is_err());
  }

  proptest! {
    #[test]
    fn test_gama_invalid_data_length(data in filter_vec_not_size_4()){
      let chunk = Chunk {
        r#type: ChunkType::gAMA,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let res = parser.handle_gama(&chunk);

      prop_assert!(res.is_err());
    }

    #[test]
    fn test_gama_invalid_i32_values(data in filter_above_i32()) {
      let data: [u8; 4] = data.to_be_bytes();
      let chunk = Chunk {
        r#type: ChunkType::gAMA,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };
      let parser: PNGParser = PNGParser::new();
      let res = parser.handle_gama(&chunk);
      prop_assert!(res.is_err());
    }
  }
}
