use crate::{
  impl_color_conversions,
  rsm_lib::color::{color::Color, colors::rgba::RGBA},
};

/// RGB color represented as a struct
#[derive(Clone, Copy, Debug)]
pub struct RGB {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl RGB {
  pub const fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}

impl_color_conversions!(
  RGB,
  RGBA => |base: RGB| {
    RGBA { r: base.r, g: base.g, b: base.b, a: 255 }
  }
);

#[macro_export]
macro_rules! rgb {
  ($r: expr, $g: expr, $b: expr) => {
    $crate::rsm_lib::color::colors::rgb::RGB::new($r, $g, $b)
  };
}
