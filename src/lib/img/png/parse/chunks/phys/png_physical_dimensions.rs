/// Physical pixel dimensions (`pHYs`)
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct PhysicalDimensions {
  /// Pixels per unit on the X axis
  pub pp_x: u32,

  /// Pixels per unit on the Y axis
  pub pp_y: u32,

  // Determines if the physical dimensions unit is the meter
  pub is_meter: bool,
}
