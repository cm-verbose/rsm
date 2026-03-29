use crate::lib::{
  img::png::parse::chunks::ihdr::png_header::PNGHeader, util::err::rsm_error::RSMError,
};

/// Handle the `IHDR` (Image header) chunk.
pub(crate) fn handle_ihdr(data: [u8; 13]) -> Result<PNGHeader, RSMError> {
  let width: u32 = get_ihdr_size(&data[0..4])?;
  let height: u32 = get_ihdr_size(&data[4..8])?;

  Ok(PNGHeader {
    width,
    height,
    bit_depth: data[8].try_into()?,
    color_type: data[9].try_into()?,
    compression_method: data[10].try_into()?,
    filter_method: data[11].try_into()?,
    interlace_method: data[12].try_into()?,
  })
}

/// Get a specific `IHDR` size (width or height).
fn get_ihdr_size(data: &[u8]) -> Result<u32, RSMError> {
  let Ok::<[u8; 4], _>(bytes) = data.try_into() else {
    return Err(RSMError::InvalidContent);
  };
  let size: u32 = u32::from_be_bytes(bytes);
  if size == 0 || size > i32::MAX as u32 {
    return Err(RSMError::OutOfBounds);
  }
  Ok(size)
}
