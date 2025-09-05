use crate::rsm_lib::{img::png::handler::png_handler::PNGHandler, util::reporter::Reporter};

impl<'b> PNGHandler<'b> {
  /// Minimum size of the file for the `.png` file to be valid, as it must contain at least :
  ///
  /// - The PNG image signature:                                                   =  8 bytes
  /// - IHDR: 4 bytes (type) + 4 bytes (length = 13) + 13 bytes (fields) + 4 (crc) = 25 bytes
  /// - IDAT: 4 bytes (type) + 4 bytes (length)      + 10 bytes (data)   + 4 (crc) = 22 bytes
  /// - IEND: 4 bytes (type) + 4 bytes (length = 0)                      + 4 (crc) = 12 bytes
  ///  Total:                                                                        67 bytes
  ///
  pub(super) const MIN_FILE_LENGTH: usize = 67;

  /// The PNG image signature a big-endian to avoid storing a `[u8; 8]`, which would be
  /// equivalent to \[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
  const SIGNATURE: u64 = 0x89504E470D0A1A0A;

  /// Validate the signature and the length of the image
  pub(super) fn validate_signature(&mut self) -> Result<(), String> {
    if self.bytes.len() < Self::MIN_FILE_LENGTH {
      return Reporter::report_under_minimum_bytes(Self::MIN_FILE_LENGTH);
    }

    let signature: &[u8; 8] = self.bytes[0..8].try_into().unwrap();
    if u64::from_be_bytes(*signature) != Self::SIGNATURE {
      return Err(format!("Image signature does not match"));
    }
    self.ptr = 8;
    Ok(())
  }
}
