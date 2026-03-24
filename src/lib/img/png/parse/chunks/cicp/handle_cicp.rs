use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::cicp::png_code_points::CodePoints, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle cICP (Coding-independent code points for video signal type
  /// identification) chunk
  pub(in super::super::super) fn handle_cicp(&self, chunk: &Chunk) -> Result<CodePoints, RSMError> {
    let Ok::<&[u8; 4], _>(data) = chunk.data.try_into() else {
      return Err(RSMError::InvalidLength);
    };

    Ok(CodePoints {
      color_primaries: data[0],
      transfer_function: data[1],
      matrix_coefficient: data[2],
      full_video_range: data[3],
    })
  }
}
