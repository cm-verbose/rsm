/// Code points obtained from the cICP chunk (structs not implemented for the)
/// sake of simplicity.
#[derive(Debug, PartialEq)]
pub struct CodePoints {
  pub color_primaries: u8,
  pub transfer_function: u8,
  pub matrix_coefficient: u8,
  pub full_video_range: u8,
}
