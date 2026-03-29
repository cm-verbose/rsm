use crate::lib::{
  img::png::parse::chunks::cabx::png_attribution_manifest::AttributionManifest,
  util::err::rsm_error::RSMError,
};
use c2pa::Reader;
use std::io::Cursor;

/// Handle caBX (Content Credentials) chunk
pub(in super::super::super) fn handle_cabx(
  data: &[u8],
) -> Result<Option<Vec<AttributionManifest>>, RSMError> {
  let dummy: Cursor<&[u8]> = Cursor::new(data);
  let mut manifests: Vec<AttributionManifest> = Vec::new();

  if let Ok(reader) = Reader::from_manifest_data_and_stream(data, "image/png", dummy) {
    for manifest in reader.iter_manifests() {
      let name = manifest.common_name();
      let title = manifest.title().map(String::from);
      let time = manifest.time();
      let issuer = manifest.issuer();
      let label = manifest.label().map(String::from);

      let manifest: AttributionManifest = AttributionManifest {
        name,
        time,
        title,
        issuer,
        label,
      };
      manifests.push(manifest);
    }
  }

  if manifests.is_empty() {
    Ok(None)
  } else {
    Ok(Some(manifests))
  }
}
