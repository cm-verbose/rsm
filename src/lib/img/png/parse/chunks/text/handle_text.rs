use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::text::png_text::Text, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle tEXt (Textual data) chunk
  pub(in super::super::super) fn handle_text(&self, chunk: &Chunk) -> Result<Text, RSMError> {
    let data: &[u8] = chunk.data;
    let mut parts = data.splitn(2, |&n| n == 0);

    let keyword: &[u8] = parts.next().unwrap();
    if keyword.len() > 79 {
      return Err(RSMError::InvalidLength);
    }
    let keyword_str: String = Self::read_text(keyword)?;

    if let Some(text) = parts.next() {
      if text.contains(&0) {
        return Err(RSMError::InvalidContent);
      }
      let text_content: String = Self::read_text(text)?;
      Ok(Text::Text(keyword_str, text_content))
    } else {
      Err(RSMError::InvalidContent)
    }
  }
}
