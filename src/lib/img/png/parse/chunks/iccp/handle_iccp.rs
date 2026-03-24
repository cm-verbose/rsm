use libdeflater::Decompressor;

use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::iccp::png_iccp_profile::ICCPProfile, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle iCCP (Embedded ICC profile) chunk
  pub(in super::super::super) fn handle_iccp(
    &self,
    chunk: &Chunk,
  ) -> Result<ICCPProfile, RSMError> {
    let data = chunk.data;

    let mut parts = data.splitn(2, |&v| v == 0);

    let keyword: &[u8] = parts.next().unwrap();
    if keyword.len() > 79 {
      return Err(RSMError::InvalidLength);
    };
    let keyword_str: String = Self::read_text(keyword)?;

    if let Some(text) = parts.next() {
      if text.contains(&0) {
        return Err(RSMError::InvalidContent);
      }

      let compressed_data = &text[1..];
      let mut buffer: Vec<u8> = vec![0u8; compressed_data.len() * 4]; // guess
      let mut decompressor: Decompressor = Decompressor::new();

      decompressor
        .zlib_decompress(compressed_data, &mut buffer)
        .map_err(|_| RSMError::DecompressionError)?;

      Ok(ICCPProfile {
        name: keyword_str,
        code: buffer,
      })
    } else {
      Err(RSMError::InvalidContent)
    }
  }
}
