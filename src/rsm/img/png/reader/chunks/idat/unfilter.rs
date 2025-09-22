use crate::rsm::img::png::reader::png_reader::PNGReader;

impl<'a> PNGReader<'a> {
  /// Undo the sub filter on a scanline's pixels
  pub(super) fn unfilter_sub<'b>(&self, scanline: &'b mut [u8], bpp: usize) -> &'b mut [u8] {
    for i in bpp..scanline.len() {
      scanline[i] = scanline[i].wrapping_add(scanline[i - bpp]);
    }
    scanline
  }

  /// Undo the up filter on a scanline's pixels
  pub(super) fn unfilter_up<'b>(
    &self,
    scanline: &'b mut [u8],
    previous: Option<&[u8]>,
  ) -> &'b mut [u8] {
    if let Some(row) = previous {
      for i in 0..scanline.len() {
        scanline[i] = scanline[i].wrapping_add(row[i]);
      }
    }
    scanline
  }

  /// Undo the average filter on a scanline's pixels
  pub(super) fn unfilter_average<'b>(
    &self,
    scanline: &'b mut [u8],
    previous: Option<&[u8]>,
    bpp: usize,
  ) -> &'b mut [u8] {
    for i in 0..scanline.len() {
      let left = if i >= bpp { scanline[i - bpp] } else { 0 };
      let above = previous.map_or(0, |prev_row| prev_row[i]);
      let avg = ((left as u16 + above as u16) / 2) as u8;
      scanline[i] = scanline[i].wrapping_add(avg);
    }
    scanline
  }

  /// Undo the paeth filter on a scanline's pixels
  pub(super) fn unfilter_paeth<'b>(
    &self,
    scanline: &'b mut [u8],
    previous: Option<&[u8]>,
    bpp: usize,
  ) -> &'b mut [u8] {
    for i in 0..scanline.len() {
      let left = if i >= bpp { scanline[i - bpp] } else { 0 };
      let above = previous.map_or(0, |prev_row| prev_row[i]);
      let upper_left = if i >= bpp {
        previous.map_or(0, |prev_row| prev_row[i - bpp])
      } else {
        0
      };
      scanline[i] = scanline[i].wrapping_add(self.paeth_predictor(left, above, upper_left));
    }
    scanline
  }

  /// paeth predictor function
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
