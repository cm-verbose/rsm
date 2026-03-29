use crate::lib::{
  img::png::parse::chunks::{ihdr::png_color_type::ColorType, utils::get_bytes},
  util::err::rsm_error::RSMError,
};

/// Handle `sBIT` (Significant bits) Chunk
pub(crate) fn handle_sbit(data: &[u8], color_type: ColorType) -> Result<&[u8], RSMError> {
  let range_end: usize = match color_type {
    // Color type: 0
    ColorType::Greyscale => 1,

    // Color type: 2, 3
    ColorType::Truecolor | ColorType::IndexedColor => 3,

    // Color type: 4
    ColorType::GreyscaleAlpha => 2,

    // Color type: 6
    ColorType::TruecolorAlpha => 6,
  };
  get_bytes(0..range_end, data)
}
