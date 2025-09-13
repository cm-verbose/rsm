use crate::rsm_lib::img::png::reader::png_reader::PNGReader;

impl<'a> PNGReader<'a> {
  /// Minimum size of the file for the `.png` file to be valid, as it must contain at least :
  ///
  /// - The PNG image signature:                                                   =  8 bytes
  /// - IHDR: 4 bytes (type) + 4 bytes (length = 13) + 13 bytes (fields) + 4 (crc) = 25 bytes
  /// - IDAT: 4 bytes (type) + 4 bytes (length)      + 10 bytes (data)   + 4 (crc) = 22 bytes
  /// - IEND: 4 bytes (type) + 4 bytes (length = 0)                      + 4 (crc) = 12 bytes
  ///  Total:                                                                        67 bytes
  ///
  const MIN_SIZE: usize = 67;
  const SIGNATURE: u64 = 0x89504E470D0A1A0A;

  /// Validate the image signature
  pub(super) fn validate_signature(&mut self) -> Result<(), String> {
    if self.bytes.len() < Self::MIN_SIZE {
      return Err(format!("fail : minimum length"));
    }

    let signature: [u8; 8] = self.bytes[0..8].try_into().unwrap();
    if u64::from_be_bytes(signature) != Self::SIGNATURE {
      return Err(format!("fail: invalid signature"));
    }
    self.ptr = 8;
    Ok(())
  }
}
