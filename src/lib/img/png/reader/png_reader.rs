use crate::lib::util::err::rsm_error::RSMError;

/// Simple byte reader
pub struct PNGReader<'r> {
  pub(crate) bytes: &'r [u8],
  pub(crate) ptr: usize,
}

impl<'r> PNGReader<'r> {
  /// Create a new reader
  pub fn new(bytes: &'r [u8]) -> Self {
    Self { bytes, ptr: 0 }
  }

  /// Take ***n*** bytes from the reader's current position
  pub fn take(&mut self, n: usize) -> Result<&'r [u8], RSMError> {
    let end: usize = self.ptr.checked_add(n).ok_or(RSMError::NotEnoughContent)?;

    if end <= self.bytes.len() {
      let bytes = &self.bytes[self.ptr..end];
      self.ptr = end;
      Ok(bytes)
    } else {
      Err(RSMError::NotEnoughContent)
    }
  }

  /// Take a constant number **N** of bytes from the reader's current position
  pub fn take_sized<const N: usize>(&mut self) -> Result<&'r [u8; N], RSMError> {
    let sized: &'r [u8; N] = self
      .take(N)?
      .try_into()
      .map_err(|_| RSMError::NotEnoughContent)?;
    Ok(sized)
  }
}
