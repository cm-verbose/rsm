#![allow(unused)]
use crate::{
  impl_color_conversions,
  rsm_lib::color::{color::Color, colors::rgb::RGB},
};

/// RGB color represented as a struct
#[derive(Clone, Copy, Debug)]
pub struct RGBA {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl RGBA {
  pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }
}

impl_color_conversions!(
  RGBA,
  RGB => |base: RGBA| {
    RGB { r: base.r, g: base.g, b: base.b }
  }
);

#[macro_export]
macro_rules! rgba {
  ($r: expr, $g: expr, $b: expr, $a: expr) => {
    $crate::rsm_lib::color::colors::rgba::RGBA::new($r, $g, $b, $a)
  };
}
