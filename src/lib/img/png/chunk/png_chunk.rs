use crate::lib::img::png::chunk::png_chunk_type::ChunkType;

/// Representation of a PNG chunk
#[derive(Debug)]
pub struct Chunk<'a> {
  /// Length of the data field
  pub length: u32,

  /// Type of the chunk (see [ChunkType])
  pub r#type: ChunkType,

  /// Chunk data
  pub data: &'a [u8],

  /// CRC value
  pub crc: [u8; 4],
}
