use crate::rsm::img::png::reader::png_reader::PNGReader;
use crc32fast::Hasher;

impl<'a> PNGReader<'a> {
  /// The signature of the image, as a contiguous sequence of bytes
  const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0xd, 0xa, 0x1a, 0xa];

  /// Validate the image signature
  pub(super) fn validate_signature(&self) -> Result<(), String> {
    if self.bytes[0..8] != Self::SIGNATURE {
      return Err(format!("Invalid signature"));
    }
    Ok(())
  }

  /// Validate the CRC bytes of a chunk
  pub(super) fn validate_crc(
    &self,
    chunk_type: &[u8; 4],
    data: &[u8],
    crc: [u8; 4],
  ) -> Result<(), String> {
    let expected: u32 = u32::from_be_bytes(crc);
    let computed: u32 = self.compute_crc(chunk_type, data);

    if expected != computed {
      Err(format!("Invalid CRC for the image data"))
    } else {
      Ok(())
    }
  }

  /// Compute the CRC from the chunk type and the chunk data
  fn compute_crc(&self, chunk_type: &[u8; 4], data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();

    hasher.update(chunk_type);
    hasher.update(data);
    hasher.finalize()
  }
}
