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
    match color_type {
      // Color type: 0
      ColorType::Greyscale => Self::get_bytes(0..1, chunk),

      // Color type: 2, 3
      ColorType::Truecolor | ColorType::IndexedColor => Self::get_bytes(0..3, chunk),

      // Color type: 4
      ColorType::GreyscaleAlpha => Self::get_bytes(0..2, chunk),

      // Color type: 6
      ColorType::TruecolorAlpha => Self::get_bytes(0..6, chunk),
    }
  }
}
