use crate::lib::{
  img::png::parse::chunks::{text::png_text::Text, utils::read_text},
  util::err::rsm_error::RSMError,
};

/// Handle `tEXt` (Textual data) chunk
pub(in super::super::super) fn handle_text(data: &[u8]) -> Result<Text, RSMError> {
  let mut parts = data.splitn(2, |&n| n == 0);

  let keyword: &[u8] = parts.next().unwrap();
  if keyword.len() > 79 {
    return Err(RSMError::InvalidLength);
  }
  let keyword_str: String = read_text(keyword)?;

  if let Some(text) = parts.next() {
    if text.contains(&0) {
      return Err(RSMError::InvalidContent);
    }
    let text_content: String = read_text(text)?;
    Ok(Text::Text(keyword_str, text_content))
  } else {
    Err(RSMError::InvalidContent)
  }
}
