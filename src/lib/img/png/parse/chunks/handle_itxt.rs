use crate::lib::{
  img::png::{chunk::png_chunk::Chunk, parse::png_parser::PNGParser},
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle iTXt (International textual data) chunk
  pub(in super::super) fn handle_itxt(&self, chunk: &Chunk) -> Result<(), RSMError> {
    let _data: &[u8] = chunk.data;
    Ok(())
  }
}
