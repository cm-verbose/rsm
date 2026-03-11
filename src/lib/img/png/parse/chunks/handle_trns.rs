use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::ihdr::png_color_type::ColorType, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle tRNS (Transparency) Chunk
  pub(in super::super) fn handle_trns<'a>(
    &self,
    chunk: &Chunk<'a>,
    color_type: ColorType,
  ) -> Result<&'a [u8], RSMError> {
    match color_type {
      // Color type: 0
      ColorType::Greyscale => Self::get_bytes(0..2, chunk),

      // Color type: 2
      ColorType::Truecolor => Self::get_bytes(0..6, chunk),

      // Color type: 3
      ColorType::IndexedColor => Self::get_bytes(0..chunk.data.len(), chunk),
      _ => Err(RSMError::InvalidContent),
    }
  }
}
