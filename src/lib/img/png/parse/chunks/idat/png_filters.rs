use crate::define_png_enum;

define_png_enum! {
  /// Filter types for filter method 0
  #[derive(Debug, PartialEq)]
  pub enum FilterType {
    None = 0,
    Sub = 1,
    Up = 2,
    Average = 3,
    Paeth = 4,
  }
}
