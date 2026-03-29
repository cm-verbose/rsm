use crate::lib::util::err::rsm_error::RSMError;

/// Handle `hIST` (Image histogram) chunk
pub(crate) fn handle_hist(data: &[u8]) -> Result<Option<Vec<u16>>, RSMError> {
  if !data.len().is_multiple_of(2) {
    return Err(RSMError::InvalidContent);
  }
  let histogram: Vec<u16> = data
    .chunks_exact(2)
    .map(|x| u16::from_be_bytes(x.try_into().unwrap()))
    .collect();
  Ok(Some(histogram))
}
