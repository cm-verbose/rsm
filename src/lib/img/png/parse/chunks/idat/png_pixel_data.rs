use std::fmt::{Debug, Formatter, Result};

/// Represents pixel data from the `IDAT` chunk.
pub struct PixelData {
  pub data: Vec<u8>,
  pub width: u32,
  pub height: u32,
}

impl Debug for PixelData {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
    formatter
      .debug_struct("PixelData")
      .field("size", &self.data.len())
      .finish()
  }
}

impl PixelData {
  pub fn at(&self, x: usize, y: usize) -> (u8, u8, u8, u8) {
    let i = (x + y * self.height as usize) * 4;
    (
      self.data[i],
      self.data[i + 1],
      self.data[i + 2],
      self.data[3],
    )
  }

  pub fn display_terminal(&self) {
    for y in (0..self.height as usize).step_by(2) {
      for x in 0..self.width as usize {
        let (r1, g1, b1, _) = self.at(x, y);
        let (r2, g2, b2, _) = if y + 1 < self.height as usize {
          self.at(x, y)
        } else {
          (0, 0, 0, 255)
        };
        print!("\x1b[38;2;{r1};{g1};{b1}m\x1b[48;2;{r2};{g2};{b2}m▀");
      }
      println!("\x1b[0m");
    }
  }
}
