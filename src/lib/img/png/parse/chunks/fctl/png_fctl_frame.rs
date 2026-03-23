use crate::lib::img::png::parse::chunks::fctl::{
  png_alpha_blend::AlphaBlend, png_frame_area_disposal::FrameAreaDisposal,
};

pub struct FCTLFrame {
  pub sequence_number: u32,
  pub width: u32,
  pub height: u32,
  pub x_offset: u32,
  pub y_offset: u32,
  pub delay_num: u16,
  pub delay_den: u16,
  pub dispose_op: FrameAreaDisposal,
  pub blend_op: AlphaBlend,
}
