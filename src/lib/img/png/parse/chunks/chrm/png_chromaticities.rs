/// Chromacity values parsed from the cHRM chunk
#[derive(Debug, PartialEq)]
pub struct Chromaticities {
  // White point (x, y) position representation
  pub white_point: (f32, f32),

  // Red (x, y) position representation
  pub red: (f32, f32),

  // Green (x, y) position representation
  pub green: (f32, f32),

  // Blue (x, y) position representation
  pub blue: (f32, f32),
}
