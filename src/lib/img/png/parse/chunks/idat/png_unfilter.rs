/// Undo the **sub** filter
pub(crate) fn unfilter_sub(current: &mut [u8], bpp: usize) {
  for i in bpp..current.len() {
    current[i] = current[i].wrapping_add(current[i - bpp]);
  }
}

/// Undo the **up** filter
pub(crate) fn unfilter_up(current: &mut [u8], previous: &[u8]) {
  for i in 0..current.len() {
    current[i] = current[i].wrapping_add(previous[i])
  }
}

/// Undo the **average** filter
pub(crate) fn unfilter_average(current: &mut [u8], previous: &[u8], bpp: usize) {
  for i in 0..current.len() {
    let left = if i >= bpp { current[i - bpp] as u16 } else { 0 };
    let above = previous[i] as u16;

    let average = ((left + above) / 2) as u8;
    current[i] = current[i].wrapping_add(average)
  }
}

/// Undo the **paeth** filter
pub(crate) fn unfilter_paeth(current: &mut [u8], previous: &[u8], bpp: usize) {
  for i in 0..current.len() {
    let left = if i >= bpp { current[i - bpp] } else { 0 };
    let above = previous[i];
    let upper_left = if i >= bpp { previous[i - bpp] } else { 0 };
    current[i] = current[i].wrapping_add(paeth_predictor(left, above, upper_left))
  }
}

/// Paeth predictor function
fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
  let p = a as i32 + b as i32 - c as i32;
  let pa = (p - a as i32).abs();
  let pb = (p - b as i32).abs();
  let pc = (p - c as i32).abs();

  if pa <= pb && pa <= pc {
    a
  } else if pb <= pc {
    b
  } else {
    c
  }
}
