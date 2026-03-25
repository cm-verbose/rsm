/// Physical pixel dimensions (pHYs)
#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct PhysicalDimensions {
  /// Pixels per unit on the X axis
  pub(in super::super) pp_x: u32,

  /// Pixels per unit on the Y axis
  pub(in super::super) pp_y: u32,

  // Determines if the physical dimensions unit is the meter
  pub(in super::super) is_meter: bool,
}
