use crate::rsm_lib::img::png::reader::png_reader::PNGReader;

impl<'a> PNGReader<'a> {
  /// Undo the sub filter
  pub(super) fn unfilter_sub<'b>(
    &self,
    scanline: &'b [u8],
    out: &'b mut [u8],
    bpp: usize,
  ) -> &'b mut [u8] {
    for i in 0..scanline.len() {
      let left: u8 = if i >= bpp { out[i - bpp] } else { 0 };
      out[i] = scanline[i].wrapping_add(left);
    }
    out
  }

  /// Unfilter the up filter
  pub(super) fn unfilter_up<'b>(
    &self,
    scanline: &'b [u8],
    out: &'b mut [u8],
    prev: Option<&'b [u8]>,
  ) -> &'b mut [u8] {
    for i in 0..scanline.len() {
      let b = prev.map_or(0, |p| p[i]);
      out[i] = scanline[i].wrapping_add(b);
    }
    out
  }

  /// Unfilter average filter
  pub(super) fn unfilter_average<'b>(
    &self,
    scanline: &'b [u8],
    out: &'b mut [u8],
    prev: Option<&'b [u8]>,
    bpp: usize,
  ) -> &'b mut [u8] {
    for i in 0..scanline.len() {
      let a = if i >= bpp { out[i - bpp] } else { 0 };
      let b = prev.map_or(0, |p| p[i]);
      out[i] = scanline[i].wrapping_add(((a as u16 + b as u16) / 2) as u8);
    }
    out
  }

  /// Unfilter paeth filter
  pub(super) fn unfilter_paeth<'b>(
    &self,
    scanline: &'b [u8],
    out: &'b mut [u8],
    prev: Option<&'b [u8]>,
    bpp: usize,
  ) -> &'b mut [u8] {
    for i in 0..scanline.len() {
      let a = if i >= bpp { out[i - bpp] } else { 0 };
      let b = prev.map_or(0, |p| p[i]);
      let c = if i >= bpp {
        prev.map_or(0, |p| p[i - bpp])
      } else {
        0
      };
      out[i] = scanline[i].wrapping_add(self.paeth_predictor(a, b, c));
    }
    out
  }

  /// Paeth predictor function
  fn paeth_predictor(&self, a: u8, b: u8, c: u8) -> u8 {
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
}
