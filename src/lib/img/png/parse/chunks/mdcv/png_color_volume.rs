#[derive(Debug, PartialEq)]
pub struct ColorVolume {
  pub chromacities: [(u16, u16); 3],
  pub white_point: u32,
  pub max_luminance: u32,
  pub min_luminance: u32,
}
