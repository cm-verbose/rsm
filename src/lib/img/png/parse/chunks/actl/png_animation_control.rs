use crate::lib::img::png::parse::values::png_int::PNGInt;

///  Animation control values from the acTL chunk
#[derive(Debug, PartialEq)]
pub struct AnimationControl {
  pub frames: PNGInt,
  pub plays: PNGInt,
}
