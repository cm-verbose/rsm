use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::phys::png_physical_dimensions::PhysicalDimensions, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle pHYs (Physical pixel dimensions) chunk
  pub(in super::super::super) fn handle_phys(
    &self,
    chunk: &Chunk,
  ) -> Result<Option<PhysicalDimensions>, RSMError> {
    if let Ok::<[u8; 9], _>(bytes) = chunk.data.try_into() {
      let x = self.get_phys_size(&bytes[0..4]);
      let y = self.get_phys_size(&bytes[4..8]);

      if let (Some(pp_x), Some(pp_y)) = (x, y) {
        let unit: u8 = bytes[8];
        if unit > 1 {
          return Ok(None);
        }
        Ok(Some(PhysicalDimensions {
          pp_x,
          pp_y,
          is_meter: unit == 1,
        }))
      } else {
        Ok(None)
      }
    } else {
      Err(RSMError::InvalidLength)
    }
  }

  /// Get a physical size (width / height)
  fn get_phys_size(&self, data: &[u8]) -> Option<u32> {
    let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
      return None;
    };
    let size: u32 = u32::from_be_bytes(bytes);
    if size == 0 || size > (i32::MAX as u32) {
      None
    } else {
      Some(size)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::lib::img::png::{
    chunk::png_chunk_type::ChunkType,
    parse::{chunks::phys::handle_phys::Chunk, png_parser::PNGParser},
  };
  use proptest::{
    collection::vec,
    prelude::{Strategy, any},
    prop_assert, prop_assert_eq, prop_oneof, proptest,
  };

  fn filter_vec_not_size_4() -> impl Strategy<Value = Vec<u8>> {
    prop_oneof![vec(any::<u8>(), 0..4), vec(any::<u8>(), 5..10)]
  }

  fn filter_vec_not_size_9() -> impl Strategy<Value = Vec<u8>> {
    prop_oneof![vec(any::<u8>(), 0..9), vec(any::<u8>(), 10..15)]
  }

  fn filter_vec_4() -> impl Strategy<Value = [u8; 4]> {
    (0..(i32::MAX as u32)).prop_map(|v| v.to_be_bytes())
  }

  fn filter_above_i32() -> impl Strategy<Value = u32> {
    ((i32::MAX as u32) + 1)..=u32::MAX
  }

  #[test]
  fn test_phys_size_zero() {
    let parser = PNGParser::new();
    let res: Option<u32> = parser.get_phys_size(&[0, 0, 0, 0]);
    assert!(res.is_none());
  }

  proptest! {
    #[test]
    fn test_phys_invalid_data_length(data in filter_vec_not_size_9()) {
      let chunk = Chunk {
        r#type: ChunkType::pHYs,
        length: data.len() as u32,
        data: &data,
        crc: [0, 0, 0, 0]
      };
      let parser = PNGParser::new();
      let res = parser.handle_phys(&chunk);
      prop_assert!(res.is_err());
    }

    #[test]
    fn test_phys_invalid_sizes(data in filter_vec_not_size_4()) {
      let parser = PNGParser::new();
      let res: Option<u32> = parser.get_phys_size(&data);
      prop_assert!(res.is_none());
    }

    #[test]
    fn test_phys_above_i32_sizes(data in filter_above_i32()) {
      let parser = PNGParser::new();
      let bytes: [u8; 4] = data.to_be_bytes();
      let res: Option<u32> = parser.get_phys_size(&bytes);

      prop_assert!(res.is_none());
    }

    #[test]
    fn test_phys_valid_sizes(data in filter_vec_4()) {
      let parser = PNGParser::new();
      let res: u32 = parser.get_phys_size(&data).unwrap();
      let expected = u32::from_be_bytes(data);

      prop_assert_eq!(res, expected)
    }
  }
}
