use crate::lib::{
  img::png::parse::chunks::{
    fctl::{
      png_alpha_blend::AlphaBlend, png_fctl_frame::FCTLFrame,
      png_frame_area_disposal::FrameAreaDisposal,
    },
    ihdr::png_header::PNGHeader,
  },
  util::err::rsm_error::RSMError,
};

/// Handle fcTL (Frame Control) chunk
pub(in super::super::super) fn handle_fctl(
  data: [u8; 26],
  header: &PNGHeader,
) -> Result<Option<FCTLFrame>, RSMError> {
  let sequence_number: u32 = u32::from_be_bytes(data[0..4].try_into().unwrap());
  let width: u32 = u32::from_be_bytes(data[4..8].try_into().unwrap());
  let height: u32 = u32::from_be_bytes(data[8..12].try_into().unwrap());

  if width == 0 || height == 0 {
    return Err(RSMError::InvalidContent);
  }

  let x_offset: u32 = u32::from_be_bytes(data[12..16].try_into().unwrap());
  let y_offset: u32 = u32::from_be_bytes(data[16..20].try_into().unwrap());

  if x_offset + width > header.width || y_offset + height > header.height {
    return Err(RSMError::InvalidContent);
  }

  let delay_num: u16 = u16::from_be_bytes(data[20..22].try_into().unwrap());
  let mut delay_den: u16 = u16::from_be_bytes(data[22..24].try_into().unwrap());

  if delay_den == 0 {
    delay_den = 100
  }

  let dispose_op: FrameAreaDisposal = data[24].try_into()?;
  let blend_op: AlphaBlend = data[25].try_into()?;

  Ok(Some(FCTLFrame {
    sequence_number,
    width,
    height,
    x_offset,
    y_offset,
    delay_num,
    delay_den,
    dispose_op,
    blend_op,
  }))
}
