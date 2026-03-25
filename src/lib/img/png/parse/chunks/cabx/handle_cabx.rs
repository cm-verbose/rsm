use std::io::Cursor;

use c2pa::Reader;

use crate::lib::img::png::{
  chunk::png_chunk::Chunk,
  parse::{chunks::cabx::png_attribution_manifest::AttributionManifest, png_parser::PNGParser},
};

impl PNGParser {
  /// Handle cabx (Content Credentials) chunk
  pub(in super::super::super) fn handle_cabx(
    &self,
    chunk: &Chunk,
  ) -> Option<Vec<AttributionManifest>> {
    let dummy: Cursor<&[u8]> = Cursor::new(chunk.data);
    let mut manifests: Vec<AttributionManifest> = Vec::new();

    if let Ok(reader) = Reader::from_manifest_data_and_stream(chunk.data, "image/png", dummy) {
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
      None
    } else {
      Some(manifests)
    }
  }
}
