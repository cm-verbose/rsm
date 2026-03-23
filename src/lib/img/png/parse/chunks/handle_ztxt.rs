use libdeflater::Decompressor;

use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{
      chunks::{ihdr::png_compression_method::CompressionMethod, text::png_text::Text},
      png_parser::PNGParser,
    },
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle zTXt (Compressed textual data) chunk
  pub(in super::super) fn handle_ztxt(&self, chunk: &Chunk) -> Result<Text, RSMError> {
    let data: &[u8] = chunk.data;
    let mut parts = data.splitn(2, |&n| n == 0);

    let keyword: &[u8] = parts.next().unwrap();
    if keyword.len() > 79 {
      return Err(RSMError::InvalidLength);
    }
    let keyword_str: String = Self::read_text(keyword)?;

    if let Some(text) = parts.next() {
      let method: CompressionMethod = text[0].try_into()?;
      if method != CompressionMethod::Deflate {
        return Err(RSMError::InvalidContent);
      }

      let compressed_data = &text[1..];
      let mut buffer: Vec<u8> = vec![0u8; compressed_data.len() * 4]; // guess
      let mut decompressor: Decompressor = Decompressor::new();

      decompressor
        .zlib_decompress(compressed_data, &mut buffer)
        .map_err(|_| RSMError::DecompressionError)?;

      let text: String = Self::read_text(&buffer)?;
      Ok(Text::CompressedText(keyword_str, text))
    } else {
      Err(RSMError::InvalidContent)
    }
  }
}
