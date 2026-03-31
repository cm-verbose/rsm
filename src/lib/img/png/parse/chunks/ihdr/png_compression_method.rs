use crate::define_png_enum;

define_png_enum! {
  /// Method used to compress the image
  #[derive(Debug, PartialEq, Clone, Copy)]
  pub enum CompressionMethod {
    Deflate = 0
  }
}
