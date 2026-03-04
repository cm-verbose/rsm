use crate::lib::img::png::chunk::png_chunk_type::ChunkType;

/// Representation of a PNG chunk
#[derive(Debug)]
pub struct Chunk<'a> {
  pub length: u32,
  pub r#type: ChunkType,
  pub data: &'a [u8],
  pub crc: [u8; 4],
}
