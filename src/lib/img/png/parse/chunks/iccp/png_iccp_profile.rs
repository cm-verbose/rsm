#[derive(Debug, PartialEq)]
pub struct ICCPProfile {
  pub name: String,
  pub code: Vec<u8>,
}
