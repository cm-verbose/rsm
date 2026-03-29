use libdeflater::Decompressor;

use crate::lib::{
  img::png::parse::chunks::{iccp::png_icc_profile::ICCProfile, utils::read_text},
  util::err::rsm_error::RSMError,
};

/// Handle `iCCP` (Embedded ICC profile) chunk
pub(crate) fn handle_iccp(data: &[u8]) -> Result<ICCProfile, RSMError> {
  let mut parts = data.splitn(2, |&v| v == 0);

  let keyword: &[u8] = parts.next().unwrap();
  if keyword.len() > 79 {
    return Err(RSMError::InvalidLength);
  };
  let keyword_str: String = read_text(keyword)?;

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

    Ok(ICCProfile {
      name: keyword_str,
      code: buffer,
    })
  } else {
    Err(RSMError::InvalidContent)
  }
}
