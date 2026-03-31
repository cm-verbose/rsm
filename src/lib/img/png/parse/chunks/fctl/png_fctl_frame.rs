use crate::lib::img::png::parse::{
  chunks::fctl::{png_alpha_blend::AlphaBlend, png_frame_area_disposal::FrameAreaDisposal},
  values::png_int::PNGInt,
};

#[derive(Debug)]
pub struct FrameControl {
  pub sequence_number: PNGInt,
  pub width: PNGInt,
  pub height: PNGInt,
  pub x_offset: PNGInt,
  pub y_offset: PNGInt,
  pub delay_num: u16,
  pub delay_den: u16,
  pub dispose_op: FrameAreaDisposal,
  pub blend_op: AlphaBlend,
}
