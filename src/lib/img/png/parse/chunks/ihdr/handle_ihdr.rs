use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_header::PNGHeader, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle IHDR (Image header) chunk
  pub(in super::super::super) fn handle_ihdr(
    &self,
    chunk: &Chunk<'_>,
  ) -> Result<PNGHeader, RSMError> {
    if let Ok::<[u8; 13], _>(data) = chunk.data.try_into() {
      let width: u32 = self.get_ihdr_size(&data[0..4])?;
      let height: u32 = self.get_ihdr_size(&data[4..8])?;

      Ok(PNGHeader {
        width,
        height,
        bit_depth: data[8].try_into()?,
        color_type: data[9].try_into()?,
        compression_method: data[10].try_into()?,
        filter_method: data[11].try_into()?,
        interlace_method: data[12].try_into()?,
      })
    } else {
      Err(RSMError::InvalidContent)
    }
  }

  /// Get IHDR size (width or height)
  fn get_ihdr_size(&self, data: &[u8]) -> Result<u32, RSMError> {
    let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
      return Err(RSMError::InvalidContent);
    };
    let size: u32 = u32::from_be_bytes(bytes);
    if size == 0 || size > i32::MAX as u32 {
      return Err(RSMError::InvalidLength);
    }
    Ok(size)
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::img::png::{
    chunk::{png_chunk::Chunk, png_chunk_type::ChunkType},
    parse::png_parser::PNGParser,
  };
  use proptest::{collection::vec, prelude::*};

  proptest! {
    /// Test invalid IDHR data lengths
    #[test]
    fn test_invalid_ihdr_data_lengths(vec in prop_oneof![0..12usize, 14..19usize].prop_flat_map(|length|
      vec(any::<u8>(), length)
    )) {
      let data: Chunk<'_> = Chunk {
        r#type: ChunkType::IHDR,
        data: &vec,
        length: vec.len() as u32,
        crc: [0, 0, 0, 0]
      };

      let parser: PNGParser = PNGParser::new();
      let result = parser.handle_ihdr(&data);
      prop_assert!(result.is_err());
    }

    /// Test invalid size array lengths
    #[test]
    fn test_invalid_size_array_lengths(vec in prop_oneof![0..3usize, 5..10usize].prop_flat_map(|length|
      vec(any::<u8>(), length)
    )) {
      let parser: PNGParser = PNGParser::new();
      let result = parser.get_ihdr_size(&vec);
      prop_assert!(result.is_err());
    }


    /// Test sizes (values below 0 and above i32::MAX are deemed invalid)
    #[test]
    fn test_invalid_lengths(arr in any::<[u8; 4]>()) {
      let slice: &[u8] = &arr;
      let value: u32 = u32::from_be_bytes(arr);
      let parser: PNGParser = PNGParser::new();

      let result = parser.get_ihdr_size(slice);
      if value == 0 || value > i32::MAX as u32 {
        prop_assert!(result.is_err());
      } else {
        prop_assert!(result.is_ok());
      }
    }
  }
}
