/// Representation of a PNG chunk
#[derive(Debug)]
pub struct Chunk<'a> {
  pub length: u32,
  pub data: &'a [u8],
  pub crc: [u8; 4],
}
