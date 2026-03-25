use crate::lib::img::png::chunk::png_chunk_type::ChunkType;

/// Representation of a PNG chunk
#[derive(Debug)]
pub struct Chunk<'a> {
  /// Length of the data field.
  pub length: u32,

  /// The [type](ChunkType) of this given chunk, as per the PNG specification.
  /// Unknown chunks will be defined as [private](ChunkType::Private) chunks,
  /// and should be handled appropriately.
  pub r#type: ChunkType,

  /// Chunk data, spanning *n* bytes.
  pub data: &'a [u8],

  /// CRC (Cyclic redundancy check) value used for error correction.
  pub crc: [u8; 4],
}
