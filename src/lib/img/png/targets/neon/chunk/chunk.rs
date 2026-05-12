/// Representation of a PNG chunk.
pub(crate) struct Chunk<'d> {
  pub r#type: u32,
  pub data: &'d [u8],
  pub crc: [u8; 4],
}
