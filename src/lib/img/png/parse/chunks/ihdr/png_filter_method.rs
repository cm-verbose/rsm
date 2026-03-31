use crate::define_png_enum;

define_png_enum! {
  /// Represents the set of filters used to enhance compressabilit
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum FilterMethod {
    Method0 = 0
  }
}
