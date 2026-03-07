/// Physical pixel dimensions (pHYs)
pub struct PhysicalDimensions {
  /// Pixels per unit on the X axis
  pub(in super::super) pp_x: u32,

  /// Pixels per unit on the Y axis
  pub(in super::super) pp_y: u32,
  pub(in super::super) is_meter: bool,
}
