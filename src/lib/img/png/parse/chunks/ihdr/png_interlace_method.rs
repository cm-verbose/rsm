use crate::define_png_enum;

define_png_enum! {
  /// Defines the way the image is interlaced, or not
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum InterlaceMethod {
    Null = 0,
    Adam7 = 1,
  }
}
