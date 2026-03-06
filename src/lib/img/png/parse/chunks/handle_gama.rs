use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// handle gAMA (Image gamma) chunk
  pub(in super::super) fn handle_gama(&mut self, chunk: &Chunk<'_>) -> Result<f32, RSMError> {
    if let Ok::<&[u8; 4], _>(bytes) = chunk.data.try_into() {
      let value: u32 = u32::from_be_bytes(*bytes);
      if value > (i32::MAX as u32) || value == 0 {
        return Err(RSMError::InvalidLength);
      }

      let gamma_value: f32 = value as f32 / 100_000.0;
      Ok(gamma_value)
    } else {
      Err(RSMError::InvalidContent)
    }
  }
}
