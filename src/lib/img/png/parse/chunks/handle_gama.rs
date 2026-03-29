use crate::lib::util::err::rsm_error::RSMError;

/// handle `gAMA` (Image gamma) chunk
pub(crate) fn handle_gama(data: [u8; 4]) -> Result<f32, RSMError> {
  let value: u32 = u32::from_be_bytes(data);
  if value > (i32::MAX as u32) || value == 0 {
    return Err(RSMError::InvalidLength);
  }

  let gamma_value: f32 = value as f32 / 100_000.0;
  Ok(gamma_value)
}
