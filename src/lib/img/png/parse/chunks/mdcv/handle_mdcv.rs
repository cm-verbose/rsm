use crate::lib::{
  img::png::parse::{chunks::mdcv::png_color_volume::ColorVolume, values::png_int::PNGInt},
  util::err::rsm_error::RSMError,
};

/// Handle `mDCV` (Mastering display color volume) chunk.
pub(crate) fn handle_mdcv(data: [u8; 24]) -> Result<ColorVolume, RSMError> {
  let mut chromacities: [(u16, u16); 3] = [(0, 0); 3];

  for (i, v) in chromacities.iter_mut().enumerate() {
    let s_o: usize = i * 4;
    let c_x: &[u8] = &data[s_o..(s_o + 2)];
    let c_y: &[u8] = &data[(s_o + 2)..(s_o + 4)];

    let x: u16 = u16::from_be_bytes(c_x.try_into().unwrap());
    let y: u16 = u16::from_be_bytes(c_y.try_into().unwrap());

    *v = (x, y);
  }

  let white_point: u32 = *PNGInt::try_from(&data[12..16])? / 20_000;
  let max_luminance: u32 = *PNGInt::try_from(&data[16..20])? / 10_000;
  let min_luminance: u32 = *PNGInt::try_from(&data[20..24])? / 10_000;

  Ok(ColorVolume {
    chromacities,
    white_point,
    max_luminance,
    min_luminance,
  })
}
