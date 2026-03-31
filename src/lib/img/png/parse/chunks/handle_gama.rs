use crate::lib::{img::png::parse::values::png_int::PNGInt, util::err::rsm_error::RSMError};

/// handle `gAMA` (Image gamma) chunk
pub(crate) fn handle_gama(data: [u8; 4]) -> Result<f32, RSMError> {
  let value: PNGInt = data.try_into()?;
  if *value == 0 {
    return Err(RSMError::InvalidLength);
  }

  let gamma_value: f32 = *value as f32 / 100_000.0;
  Ok(gamma_value)
}
