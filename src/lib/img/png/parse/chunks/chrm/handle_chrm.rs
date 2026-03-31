use crate::lib::{
  img::png::parse::{chunks::chrm::png_chromaticities::Chromaticities, values::png_int::PNGInt},
  util::err::rsm_error::RSMError,
};

/// Handle `cHRM` (primary chromaticities and white point) chunk
pub(crate) fn handle_chrm(data: [u8; 32]) -> Result<Option<Chromaticities>, RSMError> {
  let mut chromacities_values: [(f32, f32); 4] = [(0.0, 0.0); 4];

  for (i, chromacity) in chromacities_values.iter_mut().enumerate() {
    // Start offset
    let s_o: usize = i * 8;
    let cx: &[u8] = &data[s_o..(s_o + 4)];
    let cy: &[u8] = &data[(s_o + 4)..(s_o + 8)];

    let x: PNGInt = cx.try_into()?;
    let y: PNGInt = cy.try_into()?;

    *chromacity = (*x as f32 / 100000.0, *y as f32 / 100000.0)
  }

  let chromacities: Chromaticities = Chromaticities {
    white_point: chromacities_values[0],
    red: chromacities_values[1],
    green: chromacities_values[2],
    blue: chromacities_values[3],
  };
  Ok(Some(chromacities))
}
