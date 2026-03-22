/// Represents PNG text and its different forms
pub enum Text {
  /// Text produced from the tEXt (Textual data) chunk
  Text(String, String),
}
