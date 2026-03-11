use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_color_type::ColorType, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle sBIT (Significant bits) Chunk
  pub(in super::super) fn handle_sbit<'a>(
    &self,
    chunk: &Chunk<'a>,
    color_type: ColorType,
  ) -> Result<&'a [u8], RSMError> {
    let range_end: usize = match color_type {
      // Color Type: 0
      ColorType::Greyscale => 1,

      // Color Type: 2, 3
      ColorType::Truecolor | ColorType::IndexedColor => 3,

      // Color Type: 4
      ColorType::GreyscaleAlpha => 2,

      // Color Type: 6
      ColorType::TruecolorAlpha => 6,
    };
    Self::get_bytes(0..range_end, chunk)
  }
}
