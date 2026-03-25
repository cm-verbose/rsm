/// Represents PNG text and its different forms
#[derive(Debug, PartialEq)]
pub enum Text {
  /// Text obtained from the `tEXt` (Textual data) chunk
  Text(String, String),

  /// Text obtained from the `zTXt` (Compressed textual data) chunk
  CompressedText(String, String),
}
