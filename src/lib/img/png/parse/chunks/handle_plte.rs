use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle PLTE (Palette) chunk
  pub(in super::super) fn handle_plte(&self, chunk: &Chunk<'_>) -> Result<Vec<[u8; 3]>, RSMError> {
    if !chunk.data.len().is_multiple_of(3) || chunk.data.len() > 768 || chunk.data.is_empty() {
      return Err(RSMError::InvalidLength);
    }
    let palette: Vec<[u8; 3]> = chunk
      .data
      .chunks_exact(3)
      .map(|triple| [triple[0], triple[1], triple[2]])
      .collect();
    Ok(palette)
  }
}

#[cfg(test)]
mod tests {
  use proptest::{
    collection::vec,
    prelude::{Strategy, any},
    proptest,
  };

  use crate::lib::{
    img::png::{
      chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
      parse::png_parser::PNGParser,
    },
    util::err::rsm_error::RSMError,
  };

  fn filter_above_768() -> impl Strategy<Value = Vec<u8>> {
    (257..10_000usize)
      .prop_map(|k| k * 3)
      .prop_flat_map(|size| vec(any::<u8>(), size))
  }

  #[test]
  fn test_plte_len_zero() {
    let chunk: Chunk<'_> = Chunk {
      r#type: ChunkType::PLTE,
      data: &[],
      length: 6,
      crc: [0, 0, 0, 0],
    };

    let parser: PNGParser = PNGParser::new();
    let res: Result<Vec<[u8; 3]>, RSMError> = parser.handle_plte(&chunk);

    assert!(res.is_err());
  }

  proptest! {
    #[test]
    fn test_plte_invalid_lengths(data in filter_above_768()) {
      let chunk: Chunk<'_> = Chunk {
          r#type: ChunkType::PLTE,
          length: data.len() as u32,
          data: &data,
          crc: [0, 0, 0, 0],
      };

      let parser: PNGParser = PNGParser::new();
      let res: Result<Vec<[u8; 3]>, RSMError> = parser.handle_plte(&chunk);

      assert!(res.is_err());
    }
  }
}
