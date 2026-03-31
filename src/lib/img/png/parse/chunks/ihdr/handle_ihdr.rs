use crate::lib::{
  img::png::parse::{chunks::ihdr::png_header::PNGHeader, values::png_int::PNGInt},
  util::err::rsm_error::RSMError,
};

/// Handle the `IHDR` (Image header) chunk.
pub(crate) fn handle_ihdr(data: [u8; 13]) -> Result<PNGHeader, RSMError> {
  let width: PNGInt = data[0..4].try_into()?;
  let height: PNGInt = data[4..8].try_into()?;

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
