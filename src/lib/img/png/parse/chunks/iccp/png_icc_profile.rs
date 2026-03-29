/// ICC profile from the `iCCP` chunk
#[derive(Debug, PartialEq)]
pub struct ICCProfile {
  pub name: String,
  pub code: Vec<u8>,
}
