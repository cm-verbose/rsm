use std::ops::Range;

use crate::lib::util::err::rsm_error::RSMError;

pub(crate) fn get_bytes(range: Range<usize>, data: &[u8]) -> Result<&[u8], RSMError> {
  let Some(bytes) = data.get(range) else {
    return Err(RSMError::NotEnoughContent);
  };
  Ok(bytes)
}

/// Read text from bytes (Latin-1)
pub(crate) fn read_text(bytes: &[u8]) -> Result<String, RSMError> {
  Ok(bytes.iter().map(|&b| b as char).collect())
}
