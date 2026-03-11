use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle PLTE (Palette) chunk
  pub(in super::super) fn handle_plte(&self, chunk: &Chunk<'_>) -> Result<Vec<[u8; 3]>, RSMError> {
    if !chunk.length.is_multiple_of(3) || chunk.length > 768 || chunk.length == 0 {
      return Err(RSMError::InvalidLength);
    }
    let palette: Vec<[u8; 3]> = chunk
      .data
      .chunks_exact(3)
      .map(|triple| [triple[0], triple[1], triple[2]])
      .collect();
    Ok(palette)
  }
}
