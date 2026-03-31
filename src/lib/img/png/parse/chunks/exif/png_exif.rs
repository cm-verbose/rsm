use crate::lib::util::err::rsm_error::RSMError;
use exif::Exif;
use std::collections::HashMap;

/// Represents data obtained from an `eXIf` chunk
#[derive(Debug)]
pub struct PNGExifData {
  pub data: HashMap<String, String>,
}

impl TryFrom<Exif> for PNGExifData {
  type Error = RSMError;

  fn try_from(value: Exif) -> std::result::Result<Self, Self::Error> {
    let mut data: HashMap<String, String> = HashMap::new();

    for field in value.fields() {
      let tag: String = field.tag.to_string();
      let value: String = field.display_value().to_string();

      data.insert(tag, value);
    }
    Ok(Self { data })
  }
}
