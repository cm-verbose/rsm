use crate::lib::{img::png::chunk::png_chunk_type::ChunkType, util::err::rsm_error::RSMError};

pub struct Chunk<'c> {
  pub length: u32,
  pub r#type: ChunkType,
  pub data: &'c [u8],
  pub crc: [u8; 4],
}

impl<'c> Chunk<'c> {
  pub(crate) fn parse_data<T, F>(&self, parse: F) -> Result<T, RSMError>
  where
    F: FnOnce(&'c [u8]) -> Result<T, RSMError>,
  {
    parse(self.data)
  }

  /// Parse data expected to have a particular length **N**.
  pub(crate) fn parse_data_sized<const N: usize, T, F>(&self, parse: F) -> Result<T, RSMError>
  where
    F: FnOnce(&'c [u8; N]) -> Result<T, RSMError>,
  {
    let data: &'c [u8; N] = self.data.try_into().map_err(|_| RSMError::InvalidLength)?;
    parse(data)
  }
}
