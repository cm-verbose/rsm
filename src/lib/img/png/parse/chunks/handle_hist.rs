use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle hIST (Image histogram) chunk
  pub(in super::super) fn handle_hist(&self, chunk: &Chunk) -> Result<Option<Vec<u16>>, RSMError> {
    if !chunk.data.len().is_multiple_of(2) {
      return Err(RSMError::InvalidContent);
    }
    let histogram: Vec<u16> = chunk
      .data
      .chunks_exact(2)
      .map(|x| u16::from_be_bytes(x.try_into().unwrap()))
      .collect();
    Ok(Some(histogram))
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::{
    img::png::{
      chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
      parse::png_parser::PNGParser,
    },
    util::err::rsm_error::RSMError,
  };
  use proptest::{
    collection::vec,
    prelude::{Strategy, any},
    prop_assert, proptest,
  };

  /// Generate a [Vec] of [u8]s which length is not a multiple of 2
  fn vec_not_multiple_of_2() -> impl Strategy<Value = Vec<u8>> {
    (0..100usize)
      .prop_filter("Not a multiple of 2", |&v| !v.is_multiple_of(2))
      .prop_flat_map(|size| vec(any::<u8>(), size))
  }

  proptest! {
    /// Test invalid sizes for the chunk (lengths that are not multiples of 2)
    #[test]
    fn test_hist_invalid_lengths(data in vec_not_multiple_of_2()) {
      let parser = PNGParser::new();
      let chunk: Chunk<'_> = Chunk {
        r#type: ChunkType::hIST,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };

      let result: Result<Option<Vec<u16>>, RSMError> = parser.handle_hist(&chunk);
      prop_assert!(result.is_err());
    }
  }
}
