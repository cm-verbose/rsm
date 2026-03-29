use crate::lib::{
  img::png::parse::chunks::{ihdr::png_color_type::ColorType, utils::get_bytes},
  util::err::rsm_error::RSMError,
};

/// Handle `tRNS` (Transparency) Chunk
pub(crate) fn handle_trns(data: &[u8], color_type: ColorType) -> Result<&[u8], RSMError> {
  let range_end: usize = match color_type {
    // Color type: 0
    ColorType::Greyscale => Ok(2),

    // Color type: 2
    ColorType::Truecolor => Ok(6),

    // Color type: 3
    ColorType::IndexedColor => Ok(data.len()),
    _ => Err(RSMError::InvalidContent),
  }?;
  get_bytes(0..range_end, data)
}
