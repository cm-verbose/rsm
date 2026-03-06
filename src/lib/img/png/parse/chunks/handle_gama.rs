use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// handle gAMA (Image gamma) chunk
  pub(in super::super) fn handle_gama(&mut self, chunk: &Chunk<'_>) -> Result<f32, RSMError> {
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

  /// Test handling an array with an incorrect length
  #[test]
  fn test_invalid_gamma_length() {
    let mut parser = PNGParser::new();
    let gama_result = parser.handle_gama(&Chunk {
      r#type: ChunkType::gAMA,
      length: 3,
      data: &[2, 3, 1],
      crc: [0, 0, 0, 0],
    });
    assert!(gama_result.is_err())
  }

  /// Test handling the value 0
  #[test]
  fn test_null_gamma_value() {
    let mut parser = PNGParser::new();
    let gama_result = parser.handle_gama(&Chunk {
      r#type: ChunkType::gAMA,
      length: 4,
      data: &[0, 0, 0, 0],
      crc: [0, 0, 0, 0],
    });

    assert!(gama_result.is_err())
  }

  /// Test handling values superior to the max value
  #[test]
  fn test_superior_gamma_values() {
    let mut parser = PNGParser::new();
    let gama_result = parser.handle_gama(&Chunk {
      r#type: ChunkType::gAMA,
      length: 4,
      data: &[0xFF, 0xFF, 0xFF, 0xFF],
      crc: [0, 0, 0, 0],
    });

    assert!(gama_result.is_err())
  }
}
