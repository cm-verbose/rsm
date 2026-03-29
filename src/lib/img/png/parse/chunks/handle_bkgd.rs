use crate::lib::{
  img::png::parse::chunks::{ihdr::png_color_type::ColorType, utils::get_bytes},
  util::err::rsm_error::RSMError,
};

/// Handle `bKGD` (Background color) chunk
pub(crate) fn handle_bkgd(data: &[u8], color_type: ColorType) -> Result<&[u8], RSMError> {
  let range_end: usize = match color_type {
    // Color types: 0, 4
    ColorType::Greyscale | ColorType::GreyscaleAlpha => 2,

    // Color types: 2, 6
    ColorType::Truecolor | ColorType::TruecolorAlpha => 6,

    // Color type: 3
    ColorType::IndexedColor => 1,
  };
  get_bytes(0..range_end, data)
}
