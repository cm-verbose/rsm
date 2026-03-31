use crate::define_png_enum;

define_png_enum! {
  /// Color type of a given chunk
  #[derive(Debug, PartialEq, Clone, Copy)]
  pub enum ColorType {
    Greyscale = 0,
    Truecolor = 2,
    IndexedColor = 3,
    GreyscaleAlpha = 4,
    TruecolorAlpha = 6,
  }
}
