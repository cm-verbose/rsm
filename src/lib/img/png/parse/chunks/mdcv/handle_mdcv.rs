use crate::lib::{
  img::png::{
    chunk::png_chunk::Chunk,
    parse::{chunks::mdcv::png_color_volume::ColorVolume, png_parser::PNGParser},
  },
  util::err::rsm_error::RSMError,
};

impl PNGParser {
  /// Handle mDCV (Mastering display color volume) chunk
  pub(in super::super::super) fn handle_mdcv(
    &self,
    chunk: &Chunk,
  ) -> Result<ColorVolume, RSMError> {
    let Ok::<&[u8; 24], _>(data) = chunk.data.try_into() else {
      return Err(RSMError::InvalidContent);
    };
    let mut chromacities: [(u16, u16); 3] = [(0, 0); 3];

    for (i, v) in chromacities.iter_mut().enumerate() {
      let s_o: usize = i * 4;
      let c_x: &[u8] = &data[s_o..(s_o + 2)];
      let c_y: &[u8] = &data[(s_o + 2)..(s_o + 4)];

      let x: u16 = u16::from_be_bytes(c_x.try_into().unwrap());
      let y: u16 = u16::from_be_bytes(c_y.try_into().unwrap());

      *v = (x, y);
    }

    let white_point: u32 = u32::from_be_bytes(data[12..16].try_into().unwrap()) / 20_000;
    let max_luminance: u32 = u32::from_be_bytes(data[16..20].try_into().unwrap()) / 10_000;
    let min_luminance: u32 = u32::from_be_bytes(data[20..24].try_into().unwrap()) / 10_000;

    Ok(ColorVolume {
      chromacities,
      white_point,
      max_luminance,
      min_luminance,
    })
  }
}
