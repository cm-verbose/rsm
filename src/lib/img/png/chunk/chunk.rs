/// Represents a PNG chunk.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Chunk<'c> {
  pub r#type: u32,
  pub data: &'c [u8],
  pub _crc: [u8; 4],
}
