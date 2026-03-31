use crate::define_png_enum;

define_png_enum! {
  /// Defines the `sRGB` rendering intent
  #[derive(Debug, PartialEq)]
  pub enum RenderingIntent {
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3
  }
}
