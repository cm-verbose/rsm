/// Manifests obtained from the `caBX` chunk
#[derive(Debug)]
pub struct AttributionManifest {
  pub issuer: Option<String>,
  pub label: Option<String>,
  pub name: Option<String>,
  pub time: Option<String>,
  pub title: Option<String>,
}
