use crate::lib::{img::png::chunk::png_chunk_type::ChunkType, util::err::rsm_error::RSMError};

/// Representation of a chunk in the PNG datastream
pub struct Chunk<'c> {
  /// Indicates in the datastream the amount of bytes that the parser should
  /// read in order to obtain the information for a chunk.
  pub length: u32,

  /// The [type](`ChunkType`) of the chunk as described in the official
  /// documentation. This type determines the overall behavior to use to
  /// interpret the data within this chunk. [Private](`ChunkType::Private`)
  /// chunks should be handled accordingly.
  pub r#type: ChunkType,

  /// The data for a specific chunk
  pub data: &'c [u8],
  pub crc: [u8; 4],
}

impl<'c> Chunk<'c> {
  /// Parse data of any size by delegating the handling to a closure
  pub(crate) fn parse_data<T, F>(&self, parse: F) -> Result<T, RSMError>
  where
    F: FnOnce(&'c [u8]) -> Result<T, RSMError>,
  {
    parse(self.data)
  }

  /// Parse data expected to have a particular length **N** by delegating the
  /// handling to a closure.
  pub(crate) fn parse_data_sized<const N: usize, T, F>(&self, parse: F) -> Result<T, RSMError>
  where
    F: FnOnce(&'c [u8; N]) -> Result<T, RSMError>,
  {
    let data: &'c [u8; N] = self.data.try_into().map_err(|_| RSMError::InvalidLength)?;
    parse(data)
  }
}
