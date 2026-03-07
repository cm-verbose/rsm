use std::ops::Range;

use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_color_type::ColorType, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle bKGD (Background color) chunk
  pub(in super::super) fn handle_bkgd<'a>(
    &self,
    chunk: &Chunk<'a>,
    color_type: ColorType,
  ) -> Result<&'a [u8], RSMError> {
    match color_type {
      // Color types: 0, 4
      ColorType::Greyscale | ColorType::GreyscaleAlpha => Self::get_bytes(0..2, chunk),

      // Color types: 2, 6
      ColorType::Truecolor | ColorType::TruecolorAlpha => Self::get_bytes(0..6, chunk),

      // Color type: 3
      ColorType::IndexedColor => Self::get_bytes(0..1, chunk),
    }
  }

  /// Gets bytes from a range
  fn get_bytes<'a>(range: Range<usize>, chunk: &Chunk<'a>) -> Result<&'a [u8], RSMError> {
    let Some(bytes) = chunk.data.get(range) else {
      return Err(RSMError::NotEnoughContent);
    };
    Ok(bytes)
  }
}
