use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle hIST (Image histogram) chunk
  pub(in super::super) fn handle_hist(&self, chunk: &Chunk) -> Result<Option<Vec<u16>>, RSMError> {
    if !chunk.data.len().is_multiple_of(2) {
      return Err(RSMError::InvalidContent);
    }
    let s: Vec<u16> = chunk
      .data
      .chunks_exact(2)
      .map(|x| u16::from_be_bytes(x.try_into().unwrap()))
      .collect();
    Ok(Some(s))
  }
}
